"""Ownership of sort's argument arrays and NUL-separated filename storage."""
# SPDX-License-Identifier: GPL-3.0-or-later


def cleanup_sort(text, replace_once):
    if 'unsafe extern "C" fn rboxc_free_sort_resources()' in text:
        return text
    declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_sort('
    helper = '''static mut RBOXC_SORT_FILES: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut();
static mut RBOXC_SORT_MERGE_FILES: *mut sortfile = ::core::ptr::null_mut();
static mut RBOXC_SORT_LIST_STREAM: *mut FILE = ::core::ptr::null_mut();
static mut RBOXC_SORT_TOKENS: ::core::mem::MaybeUninit<Tokens> = ::core::mem::MaybeUninit::uninit();
static mut RBOXC_SORT_TOKENS_READY: bool = false;
unsafe extern "C" { fn readtokens0_free(t: *mut Tokens); }
unsafe fn rboxc_free_sort_files() {
    let files = RBOXC_SORT_FILES;
    RBOXC_SORT_FILES = ::core::ptr::null_mut();
    free(files.cast());
}
unsafe extern "C" fn rboxc_free_sort_resources() {
    let saved_errno = *::libc::__errno_location();
    let stream = RBOXC_SORT_LIST_STREAM;
    RBOXC_SORT_LIST_STREAM = ::core::ptr::null_mut();
    if !stream.is_null() { fclose(stream); }
    let merge_files = RBOXC_SORT_MERGE_FILES;
    RBOXC_SORT_MERGE_FILES = ::core::ptr::null_mut();
    free(merge_files.cast());
    rboxc_free_sort_files();
    if RBOXC_SORT_TOKENS_READY {
        RBOXC_SORT_TOKENS_READY = false;
        readtokens0_free((&raw mut RBOXC_SORT_TOKENS).cast());
    }
    *::libc::__errno_location() = saved_errno;
}
'''
    text = replace_once(text, declaration, helper+declaration)
    # Tokens must outlive main's stack for GNU's exit callbacks. The pointers
    # returned by readtokens0 borrow its obstacks, not the argv pointer array.
    start = text.index('    let mut tok: Tokens = Tokens {')
    end = text.index('    let mut outfile:', start)
    text = text[:start] + '    let tok = (&raw mut RBOXC_SORT_TOKENS).cast::<Tokens>();\n' + text[end:]
    text = text.replace('&raw mut tok', 'tok')
    text = text.replace('tok.n_tok', '(*tok).n_tok').replace('tok.tok', '(*tok).tok')
    anchor = '    atexit(Some(exit_cleanup as unsafe extern "C" fn() -> ()));'
    text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_sort_resources));')
    anchor = '''    files = xnmalloc(
        argc as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
    ) as *mut *mut ::core::ffi::c_char;'''
    text = replace_once(text, anchor, anchor+'\n    RBOXC_SORT_FILES = files;')
    anchor = '        readtokens0_init(tok);'
    text = replace_once(text, anchor, anchor+'\n        RBOXC_SORT_TOKENS_READY = true;')
    anchor = '        let mut stream: *mut FILE =\n            xfopen(files_from, b"r\\0".as_ptr() as *const ::core::ffi::c_char);'
    text = replace_once(text, anchor, anchor+'\n        if stream != stdin { RBOXC_SORT_LIST_STREAM = stream; }')
    anchor = '        xfclose(stream, files_from);'
    text = replace_once(text, anchor, '        RBOXC_SORT_LIST_STREAM = ::core::ptr::null_mut();\n'+anchor)
    # Remove the old normal-return-only cleanup supplied by the numeric adapter.
    anchor = '''    if files_from.is_null() {
        free(files as *mut ::core::ffi::c_void);
    }
    return 0 as ::core::ffi::c_int;'''
    text = replace_once(text, anchor, '    rboxc_free_sort_resources();\n    return 0 as ::core::ffi::c_int;')
    assert text.count('free(files as *mut ::core::ffi::c_void);') == 2
    text = text.replace('free(files as *mut ::core::ffi::c_void);', 'rboxc_free_sort_files();')
    anchor = '''        files = xmalloc(::core::mem::size_of::<*mut ::core::ffi::c_char>())
            as *mut *mut ::core::ffi::c_char;'''
    text = replace_once(text, anchor, anchor+'\n        RBOXC_SORT_FILES = files;')
    anchor = '''        let mut sortfiles: *mut sortfile =
            xcalloc(nfiles, ::core::mem::size_of::<sortfile>()) as *mut sortfile;'''
    text = replace_once(text, anchor, anchor+'\n        RBOXC_SORT_MERGE_FILES = sortfiles;')
    # A failed temporary-file node never joined GNU's cleanup list. Release
    # it before error() exits, preserving the original mkostemp errno.
    start = text.index('unsafe extern "C" fn create_temp_file(')
    end = text.index('unsafe extern "C" fn get_outstatus()', start)
    body = text[start:end]
    release = '        free(node as *mut ::core::ffi::c_void);\n        node = ::core::ptr::null_mut::<tempnode>();\n'
    assert body.count(release) == 1
    body = body.replace(release, '')
    anchor = '    if fd < 0 as ::core::ffi::c_int {\n'
    body = replace_once(body, anchor, anchor+release+'        *__errno_location() = saved_errno;\n')
    text = text[:start]+body+text[end:]
    return text
