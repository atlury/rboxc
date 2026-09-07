// SPDX-License-Identifier: GPL-3.0-or-later
// Ownership cleanup inserted into the translated GNU diff module.
extern "C" {
    fn atexit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;
    fn regfree(buffer: *mut re_pattern_buffer);
    #[link_name = "rboxc_diffutils_free_exclude"]
    fn rboxc_diff_free_exclude(value: *mut exclude);
}

unsafe extern "C" fn rboxc_diff_reset_regex(value: *mut re_pattern_buffer) {
    // The disjunction path creates its fastmap before recompiling. Keep that
    // allocation while releasing the previous compiled expression.
    let fastmap = (*value).fastmap;
    (*value).fastmap = ::core::ptr::null_mut();
    if !(*value).buffer.is_null() {
        regfree(value);
    }
    ::core::ptr::write_bytes(value, 0, 1);
    (*value).fastmap = fastmap;
}

unsafe extern "C" fn rboxc_diff_compile_regex(
    pattern: *const ::core::ffi::c_char,
    length: size_t,
    value: *mut re_pattern_buffer,
) -> *const ::core::ffi::c_char {
    rboxc_diff_reset_regex(value);
    re_compile_pattern(pattern, length, value)
}

unsafe extern "C" fn rboxc_diff_release_owned() {
    let saved_errno = *__errno_location();
    for value in [&raw mut function_regexp, &raw mut ignore_regexp] {
        if !(*value).buffer.is_null() {
            regfree(value);
        } else {
            libc::free((*value).fastmap.cast());
        }
        ::core::ptr::write_bytes(value, 0, 1);
    }
    libc::free(function_regexp_list.regexps.cast());
    function_regexp_list.regexps = ::core::ptr::null_mut();
    libc::free(ignore_regexp_list.regexps.cast());
    ignore_regexp_list.regexps = ::core::ptr::null_mut();
    libc::free(switch_string.cast());
    switch_string = ::core::ptr::null_mut();
    if !excluded.is_null() {
        rboxc_diff_free_exclude(excluded);
        excluded = ::core::ptr::null_mut();
    }
    *__errno_location() = saved_errno;
}

unsafe extern "C" fn rboxc_diff_close_input(value: *mut file_data) -> ::core::ffi::c_int {
    let stream = (*value).dirstream;
    let descriptor = (*value).desc;
    if stream.is_null() {
        return if descriptor >= 0 { close(descriptor) } else { 0 };
    }
    // A directory/file comparison can retain the directory stream while
    // replacing desc with the selected file. Both resources then need closing.
    let mut close_error = 0;
    if descriptor >= 0 && descriptor != libc::dirfd(stream.cast()) && close(descriptor) < 0 {
        close_error = *__errno_location();
    }
    let result = closedir(stream);
    if close_error != 0 {
        *__errno_location() = close_error;
        return -1;
    }
    result
}
