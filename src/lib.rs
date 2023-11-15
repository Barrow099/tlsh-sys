use std::ffi::c_void;

pub type tlsh_t = *mut c_void;

extern "C" {
    fn tlsh_new() -> tlsh_t;
    fn tlsh_destroy(tlsh: tlsh_t);
    fn tlsh_update(tlsh: tlsh_t, data: *const u8, len: usize);
    fn tlsh_final(tlsh: tlsh_t);
    fn tlsh_reset(tlsh: tlsh_t);
    fn tlsh_getHash(tlsh: tlsh_t) -> *const u8;
    fn tlsh_getHashBuf(tlsh: tlsh_t, output: *mut u8, len: usize);
    fn tlsh_totalDiff(tlsh: tlsh_t, other: tlsh_t) -> i32;
    fn tlsh_fromTlshStr(tlsh: tlsh_t, str: *const u8) -> i32;
}
