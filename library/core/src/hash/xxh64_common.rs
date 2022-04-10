#![allow(unused)]

use core::mem;


#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const CHUNK_SIZE: usize = mem::size_of::<u64>() * 4;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_1: u64 = 0x9E3779B185EBCA87;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_2: u64 = 0xC2B2AE3D27D4EB4F;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_3: u64 = 0x165667B19E3779F9;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_4: u64 = 0x85EBCA77C2B2AE63;
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub const PRIME_5: u64 = 0x27D4EB2F165667C5;

#[inline]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub fn round(acc: u64, input: u64) -> u64 {
    acc.wrapping_add(input.wrapping_mul(PRIME_2))
       .rotate_left(31)
       .wrapping_mul(PRIME_1)
}

#[inline]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub fn merge_round(mut acc: u64, val: u64) -> u64 {
    acc ^= round(0, val);
    acc.wrapping_mul(PRIME_1).wrapping_add(PRIME_4)
}

#[inline]
#[unstable(feature = "hashmap_xxhash", issue = "none")]
pub fn avalanche(mut input: u64) -> u64 {
    input ^= input >> 33;
    input = input.wrapping_mul(PRIME_2);
    input ^= input >> 29;
    input = input.wrapping_mul(PRIME_3);
    input ^= input >> 32;
    input
}