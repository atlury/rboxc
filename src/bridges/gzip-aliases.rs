// SPDX-License-Identifier: GPL-3.0-or-later
// Adapt the pinned GNU shell aliases' first-argument help/version checks,
// then insert their options before internal dispatch to the Rust gzip entry.
static mut RBOXC_GZIP_ALIAS_ARGV: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut();
extern "C" fn rboxc_gzip_release_alias() {
    unsafe {
    let saved = *libc::__errno_location();
    libc::free(RBOXC_GZIP_ALIAS_ARGV.cast());
    RBOXC_GZIP_ALIAS_ARGV = ::core::ptr::null_mut();
    *libc::__errno_location() = saved;
    }
}
unsafe fn rboxc_gzip_alias(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
    zcat: bool) -> ::core::ffi::c_int {
    if argc > 1 {
        let first = *argv.add(1);
        let version = libc::strcmp(first, b"--version\0".as_ptr().cast()) == 0;
        let help = libc::strcmp(first, b"--help\0".as_ptr().cast()) == 0;
        if version || help {
            let text = if version {
                if zcat { RBOXC_ZCAT_VERSION } else { RBOXC_GUNZIP_VERSION }
            } else {
                if zcat { RBOXC_ZCAT_USAGE } else { RBOXC_GUNZIP_USAGE }
            };
            let output = stdout.cast::<libc::FILE>();
            let status = if version {
                libc::fprintf(output, b"%s\n\0".as_ptr().cast(), text.as_ptr())
            } else {
                libc::fprintf(output, b"Usage: %s%s\n\0".as_ptr().cast(), *argv, text.as_ptr())
            };
            let write_errno = *libc::__errno_location();
            let flushed = libc::fflush(output);
            if status < 0 || flushed != 0 {
                let error = if flushed != 0 { *libc::__errno_location() } else { write_errno };
                let line = if zcat {
                    if version { RBOXC_ZCAT_VERSION_LINE } else { RBOXC_ZCAT_HELP_LINE }
                } else {
                    if version { RBOXC_GUNZIP_VERSION_LINE } else { RBOXC_GUNZIP_HELP_LINE }
                };
                // GNU configures these aliases with Bash on this recorded
                // host. Preserve its two printf diagnostics and source line.
                libc::fprintf(stderr.cast::<libc::FILE>(),
                    b"%s: line %u: printf: %s\n%s: line %u: printf: write error: %s\n\0".as_ptr().cast(),
                    *argv, line, libc::strerror(error), *argv, line, libc::strerror(error));
                return 1;
            }
            return 0;
        }
    }
    let Some(count) = argc.checked_add(1) else { return 1; };
    let Some(bytes) = (count as usize + 1).checked_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()) else { return 1; };
    let adjusted = libc::malloc(bytes).cast::<*mut ::core::ffi::c_char>();
    if adjusted.is_null() { return 1; }
    RBOXC_GZIP_ALIAS_ARGV = adjusted;
    libc::atexit(rboxc_gzip_release_alias);
    *adjusted = b"gzip\0".as_ptr().cast_mut().cast();
    *adjusted.add(1) = if zcat { b"-cd\0".as_ptr() } else { b"-d\0".as_ptr() }.cast_mut().cast();
    for index in 1..argc as usize { *adjusted.add(index+1) = *argv.add(index); }
    *adjusted.add(count as usize) = ::core::ptr::null_mut();
    let status = single_binary_main_gzip(count, adjusted);
    rboxc_gzip_release_alias();
    status
}
pub unsafe extern "C" fn single_binary_main_gunzip(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_gzip_alias(argc, argv, false)
}
pub unsafe extern "C" fn single_binary_main_uncompress(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_gzip_alias(argc, argv, false)
}
pub unsafe extern "C" fn single_binary_main_zcat(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_gzip_alias(argc, argv, true)
}
