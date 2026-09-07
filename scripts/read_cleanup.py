"""Track owned inputs before GNU read-error paths can terminate the process."""
# SPDX-License-Identifier: GPL-3.0-or-later
import re


def cleanup_reads(name, text, replace_once):
    marker = '// Rbox read-error ownership tracking.'
    if name not in ('cat', 'csplit', 'date', 'join', 'shuf', 'sort', 'tail', 'uniq') or marker in text:
        return text
    declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_'+name+'('
    registration = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
    helper = marker+'\n'
    if name == 'cat':
        local = '''    static mut pipefd: [::core::ffi::c_int; 2] =
        [-1 as ::core::ffi::c_int, -1 as ::core::ffi::c_int];'''
        assert text.count(local) == 1
        text = text.replace(local, '')
        text = re.sub(r'\bpipefd\b', 'RBOXC_CAT_PIPE', text)
        helper += '''static mut RBOXC_CAT_PIPE: [::core::ffi::c_int; 2] = [-1, -1];
unsafe extern "C" fn rboxc_close_read_inputs() {
    let saved_errno = *::libc::__errno_location();
    for i in 0..2 {
        let fd = RBOXC_CAT_PIPE[i];
        RBOXC_CAT_PIPE[i] = -1;
        if fd >= 0 { close(fd); }
    }
    *::libc::__errno_location() = saved_errno;
}
'''
    elif name in ('shuf', 'uniq'):
        # Only successful freopen on stdin transfers ownership. GNU still
        # closes ordinary borrowed stdin itself on its normal path.
        helper += '''static mut RBOXC_READ_INPUT_OWNED: bool = false;
unsafe fn rboxc_reopen_input(path: *const ::core::ffi::c_char, mode: *const ::core::ffi::c_char, stream: *mut FILE) -> *mut FILE {
    let result = freopen_safer(path, mode, stream);
    if stream == stdin { RBOXC_READ_INPUT_OWNED = !result.is_null(); }
    result
}
unsafe fn rboxc_finish_read_input() -> ::core::ffi::c_int {
    RBOXC_READ_INPUT_OWNED = false;
    fclose(stdin)
}
unsafe extern "C" fn rboxc_close_read_inputs() {
    if RBOXC_READ_INPUT_OWNED {
        let saved_errno = *::libc::__errno_location();
        rboxc_finish_read_input();
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        text = re.sub(r'(?<!fn )\bfreopen_safer\(', 'rboxc_reopen_input(', text)
        text = text.replace('fclose(stdin)', 'rboxc_finish_read_input()')
    elif name == 'csplit':
        helper += '''static mut RBOXC_READ_INPUT_OWNED: bool = false;
unsafe fn rboxc_reopen_input(fd: ::core::ffi::c_int, path: *const ::core::ffi::c_char, flags: ::core::ffi::c_int, mode: mode_t) -> ::core::ffi::c_int {
    let result = fd_reopen(fd, path, flags, mode);
    if fd == STDIN_FILENO { RBOXC_READ_INPUT_OWNED = result >= 0; }
    result
}
unsafe fn rboxc_finish_read_input() -> ::core::ffi::c_int {
    RBOXC_READ_INPUT_OWNED = false;
    close(STDIN_FILENO)
}
unsafe extern "C" fn rboxc_close_read_inputs() {
    if RBOXC_READ_INPUT_OWNED {
        let saved_errno = *::libc::__errno_location();
        rboxc_finish_read_input();
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        text = replace_once(text, 'fd_reopen(STDIN_FILENO, name, O_RDONLY, 0 as mode_t)',
                            'rboxc_reopen_input(STDIN_FILENO, name, O_RDONLY, 0 as mode_t)')
        text = text.replace('close(STDIN_FILENO)', 'rboxc_finish_read_input()')
    elif name == 'join':
        helper += '''static mut RBOXC_READ_INPUTS: [*mut FILE; 2] = [::core::ptr::null_mut(); 2];
unsafe fn rboxc_finish_join_input(stream: *mut FILE) -> ::core::ffi::c_int {
    for i in 0..2 {
        if RBOXC_READ_INPUTS[i] == stream { RBOXC_READ_INPUTS[i] = ::core::ptr::null_mut(); }
    }
    fclose(stream)
}
unsafe extern "C" fn rboxc_close_read_inputs() {
    let saved_errno = *::libc::__errno_location();
    for i in 0..2 {
        let stream = RBOXC_READ_INPUTS[i];
        if !stream.is_null() { rboxc_finish_join_input(stream); }
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        for index in (1, 2):
            anchor = f'    if fp{index}.is_null() {{'
            text = replace_once(text, anchor, f'    if fp{index} != stdin {{ RBOXC_READ_INPUTS[{index-1}] = fp{index}; }}\n'+anchor)
            text = text.replace(f'fclose(fp{index})', f'rboxc_finish_join_input(fp{index})')
    elif name == 'date':
        helper += '''static mut RBOXC_DATE_INPUT: *mut FILE = ::core::ptr::null_mut();
static mut RBOXC_DATE_LINE: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_close_read_inputs() {
    let saved_errno = *::libc::__errno_location();
    let stream = RBOXC_DATE_INPUT;
    RBOXC_DATE_INPUT = ::core::ptr::null_mut();
    if !stream.is_null() { fclose(stream); }
    let line = RBOXC_DATE_LINE;
    RBOXC_DATE_LINE = ::core::ptr::null_mut();
    free(line.cast());
    *::libc::__errno_location() = saved_errno;
}
'''
        anchor = '    let mut line: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();'
        text = replace_once(text, anchor, '    if in_stream != stdin { RBOXC_DATE_INPUT = in_stream; }\n'+anchor)
        anchor = '            getline(&raw mut line, &raw mut buflen, in_stream) as ssize_t;'
        text = replace_once(text, anchor, anchor+'\n        RBOXC_DATE_LINE = line;')
        anchor = '    if fclose(in_stream) == EOF {'
        text = replace_once(text, anchor, '    RBOXC_DATE_INPUT = ::core::ptr::null_mut();\n'+anchor)
        anchor = '    free(line as *mut ::core::ffi::c_void);'
        text = replace_once(text, anchor, '    RBOXC_DATE_LINE = ::core::ptr::null_mut();\n'+anchor)
    elif name == 'sort':
        # The normal sort loop owns one input at a time; merge streams have
        # a separate lifetime. The filename-list stream is finalized earlier.
        helper += '''static mut RBOXC_SORT_READ_INPUT: *mut FILE = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_close_read_inputs() {
    let stream = RBOXC_SORT_READ_INPUT;
    RBOXC_SORT_READ_INPUT = ::core::ptr::null_mut();
    if !stream.is_null() {
        let saved_errno = *::libc::__errno_location();
        fclose(stream);
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        anchor = '        let mut fp: *mut FILE = xfopen(file, b"r\\0".as_ptr() as *const ::core::ffi::c_char);'
        text = replace_once(text, anchor, anchor+'\n        if fp != stdin { RBOXC_SORT_READ_INPUT = fp; }')
        assert text.count('xfclose(fp, file);') == 2
        text = text.replace('xfclose(fp, file);', 'RBOXC_SORT_READ_INPUT = ::core::ptr::null_mut();\n        xfclose(fp, file);')
        registration = '    atexit(Some(exit_cleanup as unsafe extern "C" fn() -> ()));'
    elif name == 'tail':
        # Retain ownership while the initial read runs, before follow-mode
        # metadata is recorded. A fatal read must not lose the named fd.
        anchor = '    (*f).tailable = r#false != 0;'
        text = replace_once(text, anchor, '    if !is_stdin { (*f).fd = fd; }\n'+anchor)
        anchor = '} else if !is_stdin && close(fd) < 0 as ::core::ffi::c_int {'
        text = replace_once(text, anchor, '} else if !is_stdin && { (*f).fd = -1; close(fd) } < 0 as ::core::ffi::c_int {')
        return marker+'\n'+text
    text = replace_once(text, declaration, helper+declaration)
    text = replace_once(text, registration, registration+'\n    atexit(Some(rboxc_close_read_inputs));')
    return text
