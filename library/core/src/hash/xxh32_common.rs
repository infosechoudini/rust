#![allow(unused)]

use crate::mem;


#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const CHUNK_SIZE: usize = mem::size_of::<u32>() * 4;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_1: u32 = 0x9E3779B1;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_2: u32 = 0x85EBCA77;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_3: u32 = 0xC2B2AE3D;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_4: u32 = 0x27D4EB2F;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_5: u32 = 0x165667B1;


#[inline]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub fn round(acc: u32, input: u32) -> u32 {
    acc.wrapping_add(input.wrapping_mul(PRIME_2))
       .rotate_left(13)
       .wrapping_mul(PRIME_1)
}

#[inline]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub fn avalanche(mut input: u32) -> u32 {
    input ^= input >> 15;
    input = input.wrapping_mul(PRIME_2);
    input ^= input >> 13;
    input = input.wrapping_mul(PRIME_3);
    input ^= input >> 16;
    input
}