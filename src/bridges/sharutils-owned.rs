// SPDX-License-Identifier: GPL-3.0-or-later
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut rboxc_sharutils_stderr: *mut libc::FILE;
}
static mut RBOXC_SHARUTILS_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
static mut RBOXC_SHARUTILS_INPUT: *mut FILE = ::core::ptr::null_mut();
static mut RBOXC_SHARUTILS_OUTPUT: *mut FILE = ::core::ptr::null_mut();

unsafe extern "C" fn rboxc_sharutils_error_prefix() {
    libc::fprintf(rboxc_sharutils_stderr, b"%s: \0".as_ptr().cast(), RBOXC_SHARUTILS_INVOCATION);
}

extern "C" fn rboxc_sharutils_release_streams() {
    unsafe {
    let saved_errno = *libc::__errno_location();
    if !RBOXC_SHARUTILS_OUTPUT.is_null() {
        libc::fclose(RBOXC_SHARUTILS_OUTPUT.cast());
        RBOXC_SHARUTILS_OUTPUT = ::core::ptr::null_mut();
    }
    if !RBOXC_SHARUTILS_INPUT.is_null() {
        libc::fclose(RBOXC_SHARUTILS_INPUT.cast());
        RBOXC_SHARUTILS_INPUT = ::core::ptr::null_mut();
    }
    *libc::__errno_location() = saved_errno;
    }
}

unsafe fn rboxc_sharutils_freopen(
    path: *const ::core::ffi::c_char, mode: *const ::core::ffi::c_char, stream: *mut FILE,
) -> *mut FILE {
    // A failed freopen has already closed its old stream. Clear ownership
    // before calling it, and acquire only the returned live stream.
    let input = stream == stdin;
    if input {
        RBOXC_SHARUTILS_INPUT = ::core::ptr::null_mut();
    } else {
        RBOXC_SHARUTILS_OUTPUT = ::core::ptr::null_mut();
    }
    let result = freopen(path, mode, stream);
    if input {
        RBOXC_SHARUTILS_INPUT = result;
    } else {
        RBOXC_SHARUTILS_OUTPUT = result;
    }
    result
}

unsafe fn rboxc_sharutils_setup(argv: *mut *mut ::core::ffi::c_char) {
    RBOXC_SHARUTILS_INVOCATION = *argv;
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_sharutils_error_prefix);
    }
    if libc::atexit(rboxc_sharutils_release_streams) != 0 {
        libc::_exit(1);
    }
}
