#![allow(deprecated)]

use core::hash::CityHash64;



#[test]
#[cfg(target_pointer_width = "64")]
fn test_hash() {
    let mut hash = CityHash64::default();
    let val: &[u8] = "deadbeafdeadbeaf".as_bytes();
    assert_eq!(hash.write(&val), 7688115097038849050);
}