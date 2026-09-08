// SPDX-License-Identifier: GPL-3.0-or-later
// GNU's early diagnostic exits can bypass the local descriptor's normal close.
static mut RBOXC_MT_OWNED_LOCAL: ::core::ffi::c_int = -1;
extern "C" fn rboxc_mt_release() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        let descriptor = RBOXC_MT_OWNED_LOCAL;
        RBOXC_MT_OWNED_LOCAL = -1;
        if descriptor >= 0 {
            libc::close(descriptor);
        }
        *libc::__errno_location() = saved_errno;
    }
}
