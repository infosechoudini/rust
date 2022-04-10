use crate::convert::TryInto;

struct U128{
    lo: u64,
    hi: u64,
}


// Some primes between 2^63 and 2^64 for various uses.
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const K0: u64 = 0xc3a5c85c97cb3127;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const K1: u64 = 0xb492b66fbe98f273;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const K2: u64 = 0x9ae16a3b2f90404f;

fn bswap64(x: u64) -> u64 {
	return ((x & 0xff00000000000000) >> 56) |
		((x & 0x00ff000000000000) >> 40) |
		((x & 0x0000ff0000000000) >> 24) |
		((x & 0x000000ff00000000) >> 8) |
		((x & 0x00000000ff000000) << 8) |
		((x & 0x0000000000ff0000) << 24) |
		((x & 0x000000000000ff00) << 40) |
		((x & 0x00000000000000ff) << 56)
}

fn fetch32(p: &[u8]) -> u32 {
	return u32::from_le_bytes(p[0..4].try_into().unwrap());
}

fn fetch64(p: &[u8]) -> u64 {
    return u64::from_le_bytes(p[0..8].try_into().unwrap());
}

fn rotate(val: u64, shift: u8) -> u64 {
    if shift == 0 {
        return val;
    }
    return (val << shift) | (val >> ((64 as u8 - shift) as u64));
}

fn shiftmix(val: u64) -> u64 {
    return val ^ (val >> 47);
}

fn hash128to64(x: U128) -> u64 {
	let mul = 0x9ddfea08eb382d69 as u64;
    let (lo, hi) = (x.lo, x.hi);
	let mut a = (lo ^ hi) * mul;
	a ^= a >> (47 as u64);
	let mut b = (hi ^ a) * mul;
	b ^= b >> (47 as u64);
	b *= mul;
	return b;
}



fn hashlen16(u: u64, v: u64) -> u64 {
    return hash128to64(U128 {
        lo: u,
        hi: v,
    });
}

fn hashlen16mul(u: u64, v: u64, mul: u64) -> u64 {
    let mut a = ( u ^ v) * mul;
    a ^= a >> 47;
    let mut b = (v ^ a) * mul;
    b ^= b >> 47;
    b *= mul;
    return b;
}

fn hashlen0to16(s: &[u8]) -> u64 {
    let len = s.len();
    if len > 8 {
        let mul = K2 + ((len * 2) as u64);
        let a = fetch64(s) + K2;
        let b = fetch64(&s[len-8..]);
        let c = rotate(b, 37) * mul + a;
        let d = (rotate(a, 25) + b) * mul;
        return hashlen16mul(c, d, mul);
    }
    if len >= 4 {
        let mul = K2 + ((len * 2) as u64);
        let a = fetch32(s) as u64;
        let first = (len as u64) + (a << 3 as u64);
        let second = fetch32(&s[len-4..]) as u64;
        return hashlen16mul(first, second, mul);
    }
    if len > 0 {
        let a = s[0] as u8;
        let b = s[len>>1 as usize] as u8;
        let c = s[len-1 as usize] as u8;
        let y = ((a as u32) + ((b as u32) << (8 as u32))) as u64;
        let z = ((len as u32) + (( c as u32) << (2 as u32))) as u64;
        return shiftmix((y * K2 ^ z * K0).into()) * K2;
    }

    return K2;
  
}

fn hashlen17to32(s: &[u8]) -> u64 {
    let len = s.len();
    let mul = K2 + ((len * 2) as u64);
    let a = fetch64(&s) * K1;
    let b = fetch64(&s[8..]) * K2;
    let c = fetch64(&s[len-8..]) * mul;
    let d = fetch64(&s[len-16..]) * K2;
    return hashlen16mul(rotate(a + b, 43) + rotate(c, 30) + d,
                        a + rotate(b + K2, 18) + c, mul);
}

fn weakhashlen32withseeds(w: u64, x: u64, y: u64, z: u64, mut a: u64, mut b: u64) -> U128 {
    a += w;
    b = rotate(b + a + z, 21);
    let c = a; 
    a += x;
    a += y;
    b += rotate(a, 44);
    return U128 {
        lo: a + z,
        hi: b + c,
    };
}

fn weakhashlen32withseedsbyte(s: &[u8], a: u64, b: u64) -> U128 {
    let w = fetch64(&s);
    let x = fetch64(&s[8..]);
    let y = fetch64(&s[16..]);
    let z = fetch64(&s[24..]);
    return weakhashlen32withseeds(w, x, y, z, a, b);
}

fn hashlen33to64(s: &[u8]) -> u64 {
    let len = s.len();
    let mul = K2 + ((len * 2) as u64);
    let a = fetch64(&s) * K2;
    let b = fetch64(&s[8..]);
    let c = fetch64(&s[len-24..]);
    let d = fetch64(&s[len-32..]);
    let e = fetch64(&s[16..]) * K2;
    let f = fetch64(&s[24..]) * 9;
    let g = fetch64(&s[len-8..]);
    let h = fetch64(&s[len-16..]) * mul;
    let u = rotate(a + g, 43) + (rotate(b, 30) + c) * 9;
    let v = ((a + g) ^ d) + f + 1;
    let w = (bswap64((u+v)*mul) + g) * mul;
    let x = rotate(e + f, 42) + c;
    let y = (bswap64((v+w)*mul) + g) * mul;
    let z = e + f + c;
    let a = bswap64((x+z)*mul + y) + b;
    let b = shiftmix((z + a) * mul + d + h) * mul;

    return b + x;
}

fn cityhash64(s: &[u8]) -> u64 {
    let len = s.len();
    if len <= 32{
        if len <=16{
            return hashlen0to16(&s);
        }
        return hashlen17to32(&s);
    } else if len <= 64 {
        return hashlen33to64(&s);
    }

    //Strings over 64 bytes we hash the end first
    let mut x = fetch64(&s[len-40..]);
    let mut y = fetch64(&s[len-16..]) + fetch64(&s[len-56..]);
    let mut z = hashlen16(fetch64(&s[len-48..]) + (len as u64), fetch64(&s[len-24..]));
    let mut v = weakhashlen32withseedsbyte(&s[len-64..], len as u64, z);
    let mut w = weakhashlen32withseedsbyte(&s[len-32..], y + K1, x);
    x = x*K1 + fetch64(&s);


    // Decrease len to nearest multiple of 64, and operate on 640-byte chunks
    let mut tmp_len = len as u32;
    tmp_len = ((tmp_len - 1) as u32) & (63 as u32);
    while tmp_len > 0{
        x = rotate(x + y + v.lo + fetch64(&s[8..]), 37) * K1;
        y = rotate(y + v.hi + fetch64(&s[48..]), 42) * K1;
        x ^= w.hi;
        y += v.lo + fetch64(&s[40..]);
        z = rotate(z + w.lo, 33) * K1;
        v = weakhashlen32withseedsbyte(&s, v.hi * K1, x + w.lo);
        w = weakhashlen32withseedsbyte(&s[32..], z + w.hi, y + fetch64(&s[16..]));
        tmp_len -= 64;
    }

    return hashlen16(hashlen16(v.lo, w.lo) + shiftmix(y) * K1 + z,
                     hashlen16(v.hi, w.hi) + x);
    
}


// Hash Struct
#[unstable(feature = "hashmap_xxhash", issue = "none")]
#[derive(Debug)]
#[doc(hidden)]
pub struct CityHash64{
    _seed0: u64,
    _seed1: u64,
    hash: u64,
}



#[unstable(feature = "hashmap_xxhash", issue = "none")]
impl Default for CityHash64{
    fn default() -> CityHash64{
        CityHash64{
            _seed0: 0,
            _seed1: 0,
            hash: 0,
        }
    }
}


///Hash builder for `Xxh64`
#[derive(Clone, Copy, Debug)]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub struct CityHash64Builder {
    _seed: u64
}


#[unstable(feature = "hashmap_xxhash", issue = "none")]
impl CityHash64Builder {
    #[inline(always)]
    ///Creates builder with provided `seed`
    pub fn new(seed: u64) -> Self {
        Self {
            _seed: seed
        }
    }

    #[inline(always)]
    ///Creates hasher.
    pub fn build(self) -> CityHash64 {
        CityHash64::default()
    }
}


#[unstable(feature = "hashmap_xxhash", issue = "none")]
impl super::BuildHasher for CityHash64Builder {
    type Hasher = CityHash64;

    #[inline(always)]
    fn build_hasher(&self) -> Self::Hasher {
        self.build()
    }
}

#[unstable(feature = "hashmap_xxhash", issue = "none")]
impl super::Hasher for CityHash64 {
    #[inline(always)]
    fn finish(&self) -> u64 {
        self.hash
    }

    #[inline(always)]
    fn write(&mut self, input: &[u8]) {
        self.hash = cityhash64(input);
    }
}