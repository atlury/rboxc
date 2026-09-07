"""Release cat/tac resources when GNU's fatal write paths call exit."""
# SPDX-License-Identifier: GPL-3.0-or-later
import re


def cleanup_writes(name, text, replace_once):
    declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
    registration = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
    if name == 'cat':
        helper = '''// Keep owned buffers and named input available to fatal-exit cleanup.
static mut RBOXC_CAT_BUFFERS: [*mut ::core::ffi::c_char; 2] = [::core::ptr::null_mut(); 2];
static mut RBOXC_CAT_INPUT: ::core::ffi::c_int = -1;
unsafe fn rboxc_finish_cat_input() -> ::core::ffi::c_int {
    let fd = RBOXC_CAT_INPUT;
    RBOXC_CAT_INPUT = -1;
    close(fd)
}
unsafe extern "C" fn rboxc_free_write_resources() {
    let saved_errno = *::libc::__errno_location();
    if RBOXC_CAT_INPUT >= 0 { rboxc_finish_cat_input(); }
    for index in 0..2 {
        let buffer = RBOXC_CAT_BUFFERS[index];
        RBOXC_CAT_BUFFERS[index] = ::core::ptr::null_mut();
        alignfree(buffer.cast());
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        # Clear the ownership slot before freeing, including when the next
        # allocation exits. Never retain pointers into main's stack at exit.
        anchor = 'unsafe extern "C" fn ensure_buf_size(\n    mut buf: *mut ::core::ffi::c_char,'
        text = replace_once(text, anchor, anchor.replace('mut buf: *mut ::core::ffi::c_char',
                                                        'owner: *mut *mut ::core::ffi::c_char'))
        anchor = ') -> *mut ::core::ffi::c_char {\n    \'_c2rust_label: {'
        text = replace_once(text, anchor, anchor.replace("    '_c2rust_label:", "    let mut buf = *owner;\n    '_c2rust_label:"))
        anchor = '        alignfree(buf as *mut ::core::ffi::c_void);'
        text = replace_once(text, anchor, '        *owner = ::core::ptr::null_mut();\n'+anchor)
        anchor = '        *buf_alloc = size;'
        text = replace_once(text, anchor, '        *owner = buf;\n'+anchor)
        for index, buffer in enumerate(('inbuf', 'outbuf')):
            local = f'    let mut {buffer}: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();'
            assert text.count(local) == 1, buffer
            text = text.replace(local, '', 1)
            text, count = re.subn(r'ensure_buf_size\(\s*'+buffer+r',',
                                 f'ensure_buf_size(&raw mut RBOXC_CAT_BUFFERS[{index}],', text)
            assert count == (2 if index == 0 else 1), (buffer, count)
        anchor = '                input_desc = open(infile, file_open_mode);'
        text = replace_once(text, anchor, anchor+'\n                RBOXC_CAT_INPUT = input_desc;')
        text = replace_once(text, 'close(input_desc)', 'rboxc_finish_cat_input()')
        anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        EXIT_SUCCESS\n    } else {\n        EXIT_FAILURE\n    };\n}\npub const __LONG_LONG_MAX__'
    elif name == 'tac':
        helper = '''// G_buffer retains GNU's sentinel offset, even after growth.
static mut RBOXC_TEMP_STREAM: *mut FILE = ::core::ptr::null_mut();
static mut RBOXC_TAC_INPUT: ::core::ffi::c_int = -1;
unsafe fn rboxc_finish_tac_input() -> ::core::ffi::c_int {
    let fd = RBOXC_TAC_INPUT;
    RBOXC_TAC_INPUT = -1;
    close(fd)
}
unsafe extern "C" fn rboxc_free_write_resources() {
    let saved_errno = *::libc::__errno_location();
    if RBOXC_TAC_INPUT >= 0 { rboxc_finish_tac_input(); }
    let buffer = G_buffer;
    G_buffer = ::core::ptr::null_mut();
    if !buffer.is_null() {
        let offset = if sentinel_length != 0 { sentinel_length } else { 1 };
        ::libc::free(buffer.sub(offset as usize).cast());
    }
    let stream = RBOXC_TEMP_STREAM;
    RBOXC_TEMP_STREAM = ::core::ptr::null_mut();
    if !stream.is_null() { ::libc::fclose(stream.cast()); }
    *::libc::__errno_location() = saved_errno;
}
'''
        opened = '    if !temp_stream(&raw mut fp, &raw mut file_name) {\n        return -1 as off_t;\n    }'
        text = replace_once(text, opened, opened+'\n    RBOXC_TEMP_STREAM = fp;')
        opened = '        fd = open(filename, O_RDONLY | O_BINARY);'
        text = replace_once(text, opened, opened+'\n        RBOXC_TAC_INPUT = fd;')
        text = replace_once(text, 'close(fd)', 'rboxc_finish_tac_input()')
        anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        0 as ::core::ffi::c_int\n    } else {\n        1 as ::core::ffi::c_int\n    };\n}\npub const MANUAL_URL'
    else:
        raise ValueError(name)
    text = replace_once(text, declaration, helper+declaration)
    text = replace_once(text, registration, registration+'\n    atexit(Some(rboxc_free_write_resources));')
    text = replace_once(text, anchor, '    rboxc_free_write_resources();\n'+anchor)
    if name == 'cat':
        # Only main's variables become global ownership slots; cat() keeps
        # its borrowed buffer parameters and GNU's algorithm unchanged.
        start = text.index(declaration)
        main = text[start:]
        for index, buffer in enumerate(('inbuf', 'outbuf')):
            main = re.sub(r'\b'+buffer+r'\b', f'RBOXC_CAT_BUFFERS[{index}]', main)
        text = text[:start]+main
    return text
