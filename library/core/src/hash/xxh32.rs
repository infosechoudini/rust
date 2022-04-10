

//!32 bit version of xxhash algorithm
//!
//!Written using C implementation as reference.

use crate::ptr::{copy_nonoverlapping, read_unaligned, read};
use crate::slice::from_raw_parts;
use crate::hash::xxh32_common::{CHUNK_SIZE, PRIME_1, PRIME_2, PRIME_3, PRIME_4, PRIME_5, round, avalanche};



#[inline(always)]
fn read_le_unaligned(data: *const u8) -> u32 {
    debug_assert!(!data.is_null());

    unsafe {
       read_unaligned(data as *const u32).to_le()
    }
}

#[inline(always)]
fn read_le_aligned(data: *const u8) -> u32 {
    debug_assert!(!data.is_null());

    unsafe {
       read(data as *const u32).to_le()
    }
}

#[inline(always)]
fn read_le_is_align(data: *const u8, is_aligned: bool) -> u32 {
    match is_aligned {
        true => read_le_aligned(data),
        false => read_le_unaligned(data)
    }
}

fn finalize(mut input: u32, mut data: &[u8], is_aligned: bool) -> u32 {
    while data.len() >= 4 {
        input = input.wrapping_add(
            read_le_is_align(data.as_ptr(), is_aligned).wrapping_mul(PRIME_3)
        );
        data = &data[4..];
        input = input.rotate_left(17).wrapping_mul(PRIME_4);
    }

    for byte in data.iter() {
        input = input.wrapping_add((*byte as u32).wrapping_mul(PRIME_5));
        input = input.rotate_left(11).wrapping_mul(PRIME_1);
    }

    avalanche(input)
}

///XXH32 Streaming algorithm
#[derive(Debug, Clone, Copy)]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub struct Xxh32 {
    total_len: u32,
    is_large_len: bool,
    v1: u32,
    v2: u32,
    v3: u32,
    v4: u32,
    mem: [u32; 4],
    mem_size: u32,
}


///Creates new hasher with null seed
#[unstable(feature = "hashmap_xxhash", issue = "none")]
impl Default for Xxh32 {
    #[inline]
    fn default() -> Self {
        let seed = 0 as u32;
        Self {
            total_len: 0,
            is_large_len: false,
            v1: seed.wrapping_add(PRIME_1).wrapping_add(PRIME_2),
            v2: seed.wrapping_add(PRIME_2),
            v3: seed,
            v4: seed.wrapping_sub(PRIME_1),
            mem: [0, 0, 0, 0],
            mem_size: 0,
        }
    }
}

#[unstable(feature = "hashmap_xxhash", issue = "none")]
impl super::Hasher for Xxh32 {

    ///Hashes provided input.
    fn write(&mut self, mut input: &[u8]) {
        self.total_len = self.total_len.wrapping_add(input.len() as u32);
        self.is_large_len |= (input.len() as u32 >= CHUNK_SIZE as u32) | (self.total_len >= CHUNK_SIZE as u32);

        if (self.mem_size + input.len() as u32) < CHUNK_SIZE as u32 {
            unsafe {
               copy_nonoverlapping(input.as_ptr(), (self.mem.as_mut_ptr() as *mut u8).offset(self.mem_size as isize), input.len())
            }
            self.mem_size += input.len() as u32;
            return
        }

        if self.mem_size > 0 {
            //previous if can fail only when we do not have enough space in buffer for input.
            //hence fill_len >= input.len()
            let fill_len = CHUNK_SIZE - self.mem_size as usize;

            unsafe {
               copy_nonoverlapping(input.as_ptr(), (self.mem.as_mut_ptr() as *mut u8).offset(self.mem_size as isize), fill_len)
            }

            self.v1 = round(self.v1, self.mem[0].to_le());
            self.v2 = round(self.v2, self.mem[1].to_le());
            self.v3 = round(self.v3, self.mem[2].to_le());
            self.v4 = round(self.v4, self.mem[3].to_le());

            input = &input[fill_len..];
            self.mem_size = 0;
        }

        if input.len() >= CHUNK_SIZE {
            //In general this loop is not that long running on small input
            //So it is questionable whether we want to allocate local vars here.
            //Streaming version is likely to be used with relatively small chunks anyway.
            loop {
                self.v1 = round(self.v1, read_le_unaligned(input.as_ptr()));
                input = &input[4..];
                self.v2 = round(self.v2, read_le_unaligned(input.as_ptr()));
                input = &input[4..];
                self.v3 = round(self.v3, read_le_unaligned(input.as_ptr()));
                input = &input[4..];
                self.v4 = round(self.v4, read_le_unaligned(input.as_ptr()));
                input = &input[4..];

                if input.len() < CHUNK_SIZE {
                    break;
                }
            }
        }

        if input.len() > 0 {
            unsafe {
               copy_nonoverlapping(input.as_ptr(), self.mem.as_mut_ptr() as *mut u8, input.len())
            }
            self.mem_size = input.len() as u32;
        }
    }

    ///Finalize hashing.
    fn finish(&self) -> u64 {
        let mut result = self.total_len;

        if self.is_large_len {
            result = result.wrapping_add(
                self.v1.rotate_left(1).wrapping_add(
                    self.v2.rotate_left(7).wrapping_add(
                        self.v3.rotate_left(12).wrapping_add(
                            self.v4.rotate_left(18)
                        )
                    )
                )
            );
        } else {
            result = result.wrapping_add(self.v3.wrapping_add(PRIME_5));
        }

        let input = unsafe {
           from_raw_parts(self.mem.as_ptr() as *const u8, self.mem_size as usize)
        };

        return finalize(result, input, true) as u64;
    }
}

