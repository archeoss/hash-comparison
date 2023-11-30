use ahash::*;
use core::slice;
use hash_test::final3::{make_secret, wyhash, WyHash};
use std::hash::Hasher;

#[no_mangle]
pub extern "C" fn wyhash_3_64(buf: *const (), len: usize, seed: u64) -> u64 {
    let buf: &[u8] = unsafe { slice::from_raw_parts(buf as *const u8, len) };
    let mut hasher = WyHash::new(
        seed,
        [
            10796424174082827157, // make_secret(0) results
            13018060901348910741,
            6212095988037150041,
            16276446910676016553,
        ],
    );
    // wyhash(&buf, seed, [0, 0, 0, 0])
    // wyhash::wyhash(&buf, seed)
    hasher.write(buf);
    hasher.finish()
}

#[no_mangle]
pub extern "C" fn ahash64(buf: *const (), len: usize, seed: u64) -> u64 {
    let buf: &[u8] = unsafe { slice::from_raw_parts(buf as *const u8, len) };
    let mut build_hasher = hash_test::ahash::AHasher::new_with_keys(
        seed as u128,        // * seed as u128,
        (seed as u128) << 1, // * seed as u128,
    ); //::RandomState::with_seeds(seed, seed, seed, seed);
    build_hasher.write(&buf);
    build_hasher.finish()
}

#[no_mangle]
pub extern "C" fn wyhash_64(buf: *const (), len: usize, seed: u64) -> u64 {
    let buf: &[u8] = unsafe { slice::from_raw_parts(buf as *const u8, len) };
    // let mut hasher = wyhash::WyHash::(seed, [seed, seed, seed, seed]);
    wyhash::wyhash(&buf, seed)
    // hasher.write(buf);
    // hasher.finish()
}
