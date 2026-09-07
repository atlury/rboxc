// SPDX-License-Identifier: GPL-3.0-or-later
// The directory cache lasts until program exit. Input ownership moves to
// a DIR stream during traversal and otherwise ends at the original close.
static mut RBOXC_GZIP_INPUT: ::core::ffi::c_int = -1;
unsafe fn rboxc_gzip_close_input(descriptor: ::core::ffi::c_int) -> ::core::ffi::c_int {
    if RBOXC_GZIP_INPUT == descriptor { RBOXC_GZIP_INPUT = -1; }
    close(descriptor)
}
extern "C" fn rboxc_gzip_release_owned() {
    unsafe {
        let saved = *libc::__errno_location();
        if RBOXC_GZIP_INPUT >= 0 && RBOXC_GZIP_INPUT != STDIN_FILENO {
            rboxc_gzip_close_input(RBOXC_GZIP_INPUT);
        }
        if dfd >= 0 { close(dfd); dfd = -1; }
        *libc::__errno_location() = saved;
    }
}
