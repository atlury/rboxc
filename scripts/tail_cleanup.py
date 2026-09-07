"""Release tail's owned follow state on normal and fatal process exits."""
# SPDX-License-Identifier: GPL-3.0-or-later


def cleanup_tail(text, replace_once):
    if 'unsafe extern "C" fn rboxc_free_tail_resources()' in text:
        return text
    text = text.replace('    free(F.cast());\n    return if ok', '    return if ok')
    declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_tail('
    helper = '''static mut RBOXC_TAIL_FILES: *mut File_spec = ::core::ptr::null_mut();
static mut RBOXC_TAIL_FILE_COUNT: ::core::ffi::c_int = 0;
static mut RBOXC_TAIL_WATCH_FD: ::core::ffi::c_int = -1;
static mut RBOXC_TAIL_WATCH_TABLE: *mut Hash_table = ::core::ptr::null_mut();
static mut RBOXC_TAIL_EVENTS: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
unsafe fn rboxc_free_tail_watches() {
    let events = RBOXC_TAIL_EVENTS;
    RBOXC_TAIL_EVENTS = ::core::ptr::null_mut();
    free(events.cast());
    let table = RBOXC_TAIL_WATCH_TABLE;
    RBOXC_TAIL_WATCH_TABLE = ::core::ptr::null_mut();
    if !table.is_null() { hash_free(table); }
    let fd = RBOXC_TAIL_WATCH_FD;
    RBOXC_TAIL_WATCH_FD = -1;
    if fd >= 0 { close(fd); }
}
unsafe extern "C" fn rboxc_free_tail_resources() {
    let saved_errno = *::libc::__errno_location();
    rboxc_free_tail_watches();
    let files = RBOXC_TAIL_FILES;
    let count = RBOXC_TAIL_FILE_COUNT;
    RBOXC_TAIL_FILES = ::core::ptr::null_mut();
    RBOXC_TAIL_FILE_COUNT = 0;
    for i in 0..count {
        let file = files.offset(i as isize);
        let fd = (*file).fd;
        (*file).fd = -1;
        // GNU's main owns stdin's close; these entries own only named files.
        if fd > STDIN_FILENO { close(fd); }
    }
    free(files.cast());
    let writers = pids;
    pids = ::core::ptr::null_mut();
    nbpids = 0;
    free(writers.cast());
    *::libc::__errno_location() = saved_errno;
}
'''
    text = replace_once(text, declaration, helper+declaration)
    anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
    text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_tail_resources));')
    anchor = '        (*F.offset(i_0 as isize)).name = *file.offset(i_0 as isize);'
    text = replace_once(text, anchor, '        (*F.offset(i_0 as isize)).fd = -1;\n'+anchor)
    anchor = '    if header_mode_0.0 == header_mode::always.0'
    text = replace_once(text, anchor, '    RBOXC_TAIL_FILES = F;\n    RBOXC_TAIL_FILE_COUNT = n_files;\n'+anchor)
    anchor = '            let mut wd: ::core::ffi::c_int = inotify_init();'
    text = replace_once(text, anchor, anchor+'\n            RBOXC_TAIL_WATCH_FD = wd;')
    anchor = '    *wd_to_namep = wd_to_name;'
    text = replace_once(text, anchor, anchor+'\n    RBOXC_TAIL_WATCH_TABLE = wd_to_name;')
    anchor = '    evbuf = ximalloc(evlen) as *mut ::core::ffi::c_char;'
    text = replace_once(text, anchor, anchor+'\n    RBOXC_TAIL_EVENTS = evbuf;')
    anchor = '''                evbuf =
                    xirealloc(evbuf as *mut ::core::ffi::c_void, evlen) as *mut ::core::ffi::c_char;'''
    text = replace_once(text, anchor, anchor+'\n                RBOXC_TAIL_EVENTS = evbuf;')
    anchor = '                hash_free(ht);\n                close(wd);'
    text = replace_once(text, anchor, '                rboxc_free_tail_watches();')
    anchor = '    return if ok as ::core::ffi::c_int != 0 {\n        0 as ::core::ffi::c_int\n    } else {\n        1 as ::core::ffi::c_int\n    };\n}\npub const __CHAR_BIT__'
    text = replace_once(text, anchor, '    rboxc_free_tail_resources();\n'+anchor)
    return text
