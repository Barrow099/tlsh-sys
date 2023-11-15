use std::ffi::{c_char, c_void};

pub type tlsh_t = *mut c_void;

extern "C" {
    pub fn tlsh_new() -> tlsh_t;
    pub fn tlsh_destroy(tlsh: tlsh_t);
    pub fn tlsh_update(tlsh: tlsh_t, data: *const u8, len: usize);
    pub fn tlsh_final(tlsh: tlsh_t);
    pub fn tlsh_reset(tlsh: tlsh_t);
    pub fn tlsh_getHash(tlsh: tlsh_t) -> *const c_char;
    pub fn tlsh_getHashBuf(tlsh: tlsh_t, output: *mut c_char, len: usize);
    pub fn tlsh_totalDiff(tlsh: tlsh_t, other: tlsh_t) -> i32;
    pub fn tlsh_fromTlshStr(tlsh: tlsh_t, str: *const c_char) -> i32;
}
