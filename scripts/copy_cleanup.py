"""Release the command-owned copy/move bookkeeping after GNU finishes using it."""


def cleanup_copy(name, text, replace_once):
    declaration = f'#[no_mangle]\npub unsafe extern "C" fn single_binary_main_{name}('
    helper = '''extern "C" { fn hash_free(table: *mut Hash_table); }
static mut RBOXC_COPY_DIR_FD: ::core::ffi::c_int = -1;
static mut RBOXC_COPY_TABLES: [*mut Hash_table; 2] = [::core::ptr::null_mut(); 2];
unsafe extern "C" fn rboxc_free_copy_resources() {
    let saved_errno = *::libc::__errno_location();
    let fd = RBOXC_COPY_DIR_FD;
    RBOXC_COPY_DIR_FD = -1;
    if fd >= 0 { ::libc::close(fd); }
    for index in 0..2 {
        let table = RBOXC_COPY_TABLES[index];
        RBOXC_COPY_TABLES[index] = ::core::ptr::null_mut();
        if !table.is_null() { hash_free(table); }
    }
    *::libc::__errno_location() = saved_errno;
}
'''
    text = replace_once(text, declaration, helper+declaration)
    anchor = '    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));'
    text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_copy_resources));')
    anchor = '        target_dirfd = target_directory_operand(target_directory, &raw mut sb);'
    text = replace_once(text, anchor, anchor+'\n        RBOXC_COPY_DIR_FD = target_dirfd;')
    indent = '            ' if name == 'cp' else '                '
    anchor = indent+'target_dirfd = fd;'
    text = replace_once(text, anchor, anchor+'\n'+indent+'RBOXC_COPY_DIR_FD = fd;')
    if name == 'cp':
        for index, kind in enumerate(('dest', 'src')):
            anchor = f'            {kind}_info_init(x);'
            text = replace_once(text, anchor, anchor+f'\n            RBOXC_COPY_TABLES[{index}] = (*x).{kind}_info;')
        # The existing normal-return repair closes this descriptor. Consume
        # ownership first, including close failures, so atexit cannot retry it.
        anchor = '    if target_dirfd >= 0 {\n        let saved_errno = *::libc::__errno_location();'
        text = replace_once(text, anchor, '    RBOXC_COPY_DIR_FD = -1;\n'+anchor)
    else:
        anchor = '            dest_info_init(&raw mut x);'
        text = replace_once(text, anchor, anchor+'\n            RBOXC_COPY_TABLES[0] = x.dest_info;')
    return text
