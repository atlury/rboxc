"""Pinned GNU exit-time ownership repairs, applied after C2Rust translation."""


def replace_once(text, before, after):
    if after in text:
        return text
    assert text.count(before) == 1, 'GNU cleanup anchor changed: ' + before[:90]
    return text.replace(before, after, 1)


def cleanup(name, text):
    if name == 'pr':
        text = replace_once(text, '    cleanup();\n', '    cleanup();\n    free(file_names.cast());\n')
    if name == 'tac':
        declaration = 'unsafe extern "C" fn copy_to_temp('
        text = replace_once(text, declaration,
                            'static mut RBOXC_TEMP_STREAM: *mut FILE = ::core::ptr::null_mut();\n'
                            + declaration)
        opened = '    if !temp_stream(&raw mut fp, &raw mut file_name) {\n        return -1 as off_t;\n    }'
        text = replace_once(text, opened, opened+'\n    RBOXC_TEMP_STREAM = fp;')
        anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        0 as ::core::ffi::c_int\n    } else {\n        1 as ::core::ffi::c_int\n    };\n}\npub const MANUAL_URL'
        # GNU retains an interior pointer, including after buffer growth.
        # Its reallocation path recovers the base using this same offset.
        # Keep the cached stream through every operand, then close it.
        text = replace_once(text, anchor,
                            '    let offset = if sentinel_length != 0 { sentinel_length } else { 1 };\n'
                            '    ::libc::free(G_buffer.sub(offset as usize).cast());\n'
                            '    G_buffer = ::core::ptr::null_mut();\n'
                            '    if !RBOXC_TEMP_STREAM.is_null() {\n'
                            '        let saved_errno = *::libc::__errno_location();\n'
                            '        ::libc::fclose(RBOXC_TEMP_STREAM.cast());\n'
                            '        RBOXC_TEMP_STREAM = ::core::ptr::null_mut();\n'
                            '        *::libc::__errno_location() = saved_errno;\n'
                            '    }\n' + anchor)
    if name == 'cat':
        anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        EXIT_SUCCESS\n    } else {\n        EXIT_FAILURE\n    };\n}\npub const __LONG_LONG_MAX__'
        # Same two owned buffers released by GNU's #ifdef lint cleanup.
        text = replace_once(text, anchor,
                            '    ::libc::free(outbuf.cast());\n    ::libc::free(inbuf.cast());\n' + anchor)
    if name == 'split':
        anchor = '    closeout(\n        ::core::ptr::null_mut::<FILE>(),\n        output_desc,\n        filter_pid,\n        outfile,\n    );'
        text = replace_once(text, anchor, anchor + '\n    ::libc::free(buf.cast());')
    if name == 'expr':
        text = replace_once(text, '    printv(v);\n    return null(v) as ::core::ffi::c_int;',
                            '    printv(v);\n    let status = null(v) as ::core::ffi::c_int;\n'
                            '    freev(v);\n    return status;')
    if name == 'date':
        text = replace_once(text,
                            '    fn tzalloc(__name: *const ::core::ffi::c_char) -> timezone_t;',
                            '    fn tzalloc(__name: *const ::core::ffi::c_char) -> timezone_t;\n'
                            '    fn tzfree(tz: timezone_t);')
        anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        0 as ::core::ffi::c_int\n    } else {\n        1 as ::core::ffi::c_int\n    };\n}\npub const MANUAL_URL'
        text = replace_once(text, anchor,
                            '    free(format_copy.cast());\n    tzfree(tz);\n' + anchor)
    if name == 'tail':
        anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        0 as ::core::ffi::c_int\n    } else {\n        1 as ::core::ffi::c_int\n    };\n}\npub const __CHAR_BIT__'
        # File names in F borrow argv; tail_file closes non-followed files.
        # No member owns an allocation in this non-following path.
        text = replace_once(text, anchor, '    if !forever { free(F.cast()); }\n' + anchor)
    if name == 'tr':
        declaration = 'unsafe extern "C" fn spec_init(mut spec_list: *mut Spec_list) {'
        helper = '''// Each list owns its dummy head and every appended element.
unsafe fn rboxc_free_spec(spec: *mut Spec_list) {
    if spec.is_null() { return; }
    let mut element = (*spec).head;
    while !element.is_null() {
        let next = (*element).next;
        free(element.cast());
        element = next;
    }
    (*spec).head = ::core::ptr::null_mut();
}
'''
        text = replace_once(text, declaration, helper + declaration)
        first = '    if !parse_str(*argv.offset(optind as isize), s1) {\n        return 1 as ::core::ffi::c_int;'
        text = replace_once(text, first, first.replace('        return', '        rboxc_free_spec(s1);\n        return'))
        second = '            s2,\n        ) {\n            return 1 as ::core::ffi::c_int;'
        text = replace_once(text, second, second.replace('            return',
                            '            rboxc_free_spec(s1);\n            rboxc_free_spec(s2);\n            return'))
        anchor = '    return 0 as ::core::ffi::c_int;\n}\npub const __INT_MAX__'
        text = replace_once(text, anchor, '    rboxc_free_spec(s1);\n    rboxc_free_spec(s2);\n' + anchor)
    return text
