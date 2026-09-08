"""Release command-owned iconv resources after GNU's final use."""
# SPDX-License-Identifier: GPL-3.0-or-later
import re

def iconv_cleanup(text):
    anchor='''unsafe extern "C" fn close_output_file(mut cd: __gconv_t, mut status: ::core::ffi::c_int) {
    if status != EXIT_SUCCESS
        && omit_invalid == 0
        && (output_using_temporary_file as ::core::ffi::c_int != 0
            || output_fd < 0 as ::core::ffi::c_int)
    {
        return;
    }'''
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor.replace('        return;', '''        // GNU intentionally skips the flush to preserve overlapping input.
        // The anonymous spool has no remaining consumer on this return path.
        if output_fd >= 0 {
            let saved = *__errno_location();
            close(output_fd);
            output_fd = -1;
            *__errno_location() = saved;
        }
        return;'''))
    anchor='        __gconv_destroy_spec(&raw mut conv_spec);'
    assert text.count(anchor)==1
    text=text.replace(anchor,anchor+'''
        if res == C2Rust_Unnamed_1::__GCONV_OK.0 as ::core::ffi::c_int {
            RBOXC_ICONV_OWNED = cd;
        }''')
    pattern=r'''iconv_open\((b"UTF-8\\0"\.as_ptr\(\) as \*const ::core::ffi::c_char, from_code|to_code, b"UTF-8\\0"\.as_ptr\(\) as \*const ::core::ffi::c_char)\)
                        == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>\(
                            -1 as ::core::ffi::c_int as usize,
                        \)
                        && \*__errno_location\(\) == EINVAL'''
    text,count=re.subn(pattern,lambda m:'rboxc_iconv_encoding_unavailable('+m[1]+')',text)
    assert count==2
    anchor='    rboxc_iconv_main_inner(argc, argv)'
    assert text.count(anchor)==1
    text=text.replace(anchor,'''    if rboxc_iconv_at_exit(rboxc_iconv_release) != 0 { return 1; }
'''+anchor)
    return text+'''
static mut RBOXC_ICONV_OWNED: __gconv_t = ::core::ptr::null_mut();
extern "C" {
    #[link_name = "atexit"]
    fn rboxc_iconv_at_exit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;
    #[link_name = "tdestroy"]
    fn rboxc_iconv_destroy_tree(root: *mut ::core::ffi::c_void,
        free_key: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>);
}
// Name strings belong to the libc conversion database, not this print tree.
unsafe extern "C" fn rboxc_iconv_borrowed_key(_: *mut ::core::ffi::c_void) {}
unsafe extern "C" fn rboxc_iconv_release() {
    let saved = *__errno_location();
    if !RBOXC_ICONV_OWNED.is_null() {
        iconv_close(RBOXC_ICONV_OWNED.cast());
        RBOXC_ICONV_OWNED = ::core::ptr::null_mut();
    }
    libc::free(output_buffer_start.cast());
    output_buffer_start = ::core::ptr::null_mut();
    if !printlist.is_null() {
        rboxc_iconv_destroy_tree(printlist, Some(rboxc_iconv_borrowed_key));
        printlist = ::core::ptr::null_mut();
    }
    *__errno_location() = saved;
}
unsafe fn rboxc_iconv_encoding_unavailable(to: *const ::core::ffi::c_char,
    from: *const ::core::ffi::c_char) -> bool {
    let handle = iconv_open(to, from);
    let saved = *__errno_location();
    let failed = handle == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(usize::MAX);
    if !failed { iconv_close(handle); }
    *__errno_location() = saved;
    failed && saved == EINVAL
}
'''
