// SPDX-License-Identifier: GPL-3.0-or-later
// GNU grep uses glibc invocation names instead of Gnulib set_program_name.
extern "C" {
    static mut program_invocation_name: *mut ::core::ffi::c_char;
    static mut program_invocation_short_name: *mut ::core::ffi::c_char;
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut rboxc_grep_stderr: *mut libc::FILE;
}
unsafe extern "C" fn rboxc_grep_error_prefix() {
    libc::fprintf(rboxc_grep_stderr, b"%s: \0".as_ptr().cast(), program_invocation_name);
}
unsafe fn rboxc_grep_set_invocation(name: *mut ::core::ffi::c_char) {
    program_invocation_name = name;
    let slash = libc::strrchr(name, b'/' as ::core::ffi::c_int);
    program_invocation_short_name = if slash.is_null() { name } else { slash.add(1) };
    let prior = error_print_progname;
    if prior.is_none() {
        error_print_progname = Some(rboxc_grep_error_prefix);
    }
}
