// SPDX-License-Identifier: GPL-3.0-or-later
extern "C" {
    static mut program_invocation_name: *mut ::core::ffi::c_char;
    static mut program_invocation_short_name: *mut ::core::ffi::c_char;
}
unsafe fn rboxc_gzip_set_invocation(name: *mut ::core::ffi::c_char) {
    program_invocation_name = name;
    let slash = libc::strrchr(name, b'/' as ::core::ffi::c_int);
    program_invocation_short_name = if slash.is_null() { name } else { slash.add(1) };
}
