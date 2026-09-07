// SPDX-License-Identifier: GPL-3.0-or-later
// Translate the pinned egrep.sh warning and option insertion into internal
// Rust dispatch. The GNU shell aliases themselves do not implement matching.
static mut RBOXC_GREP_ALIAS_ARGV: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_grep_release_alias() {
    let saved_errno = *libc::__errno_location();
    libc::free(RBOXC_GREP_ALIAS_ARGV.cast());
    RBOXC_GREP_ALIAS_ARGV = ::core::ptr::null_mut();
    *libc::__errno_location() = saved_errno;
}
unsafe fn rboxc_grep_alias(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
    option: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let name = *argv;
    let slash = libc::strrchr(name, b'/' as ::core::ffi::c_int);
    let command = if slash.is_null() { name } else { slash.add(1) };
    libc::fprintf(rboxc_grep_stderr,
        b"%s: warning: %s is obsolescent; using grep %s\n\0".as_ptr().cast(),
        command, command, option);
    let Some(count) = argc.checked_add(1) else { return 2; };
    let adjusted = xnmalloc((count as usize + 1) as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
        .cast::<*mut ::core::ffi::c_char>();
    RBOXC_GREP_ALIAS_ARGV = adjusted;
    atexit(Some(rboxc_grep_release_alias));
    *adjusted = b"grep\0".as_ptr().cast_mut().cast();
    *adjusted.add(1) = option;
    for index in 1..argc as usize {
        *adjusted.add(index + 1) = *argv.add(index);
    }
    *adjusted.add(count as usize) = ::core::ptr::null_mut();
    let status = single_binary_main_grep(count, adjusted);
    rboxc_grep_release_alias();
    status
}
pub unsafe extern "C" fn single_binary_main_egrep(argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_grep_alias(argc, argv, b"-E\0".as_ptr().cast_mut().cast())
}
pub unsafe extern "C" fn single_binary_main_fgrep(argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_grep_alias(argc, argv, b"-F\0".as_ptr().cast_mut().cast())
}
