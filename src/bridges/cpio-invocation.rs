// SPDX-License-Identifier: GPL-3.0-or-later
// Each generated command includes its own invocation state.
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut RBOXC_CPIO_STDERR: *mut libc::FILE;
}
static mut RBOXC_CPIO_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_cpio_error_prefix() {
    libc::fprintf(RBOXC_CPIO_STDERR, b"%s: \0".as_ptr().cast(), RBOXC_CPIO_INVOCATION);
}
unsafe fn rboxc_cpio_setup(argv: *mut *mut ::core::ffi::c_char) {
    RBOXC_CPIO_INVOCATION = *argv;
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_cpio_error_prefix);
    }
}
