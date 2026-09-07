"""Reproducible ownership of GNU split's input, output, and round-robin records."""


def cleanup_split(text, replace_once):
    declaration = 'unsafe extern "C" fn create(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {'
    helper = '''static mut RBOXC_SPLIT_INPUT: bool = false;
static mut RBOXC_SPLIT_PENDING_FD: ::core::ffi::c_int = -1;
static mut RBOXC_SPLIT_FILES: *mut of_t = ::core::ptr::null_mut();
static mut RBOXC_SPLIT_FILE_COUNT: usize = 0;
static mut RBOXC_SPLIT_BUFFER: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
static mut RBOXC_SPLIT_TEMP: *mut FILE = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_free_split_resources() {
    let saved_errno = *::libc::__errno_location();
    let pending = RBOXC_SPLIT_PENDING_FD;
    RBOXC_SPLIT_PENDING_FD = -1;
    if pending >= 0 { ::libc::close(pending); }
    let output = output_desc;
    output_desc = -1;
    if output >= 0 { ::libc::close(output); }
    let files = RBOXC_SPLIT_FILES;
    let count = RBOXC_SPLIT_FILE_COUNT;
    RBOXC_SPLIT_FILES = ::core::ptr::null_mut();
    RBOXC_SPLIT_FILE_COUNT = 0;
    for index in 0..count {
        let entry = &mut *files.add(index);
        if entry.ofd >= 0 {
            if entry.ofile.is_null() { ::libc::close(entry.ofd); }
            else { fclose(entry.ofile); }
        }
        ::libc::free(entry.of_name.cast());
    }
    ::libc::free(files.cast());
    let temporary = RBOXC_SPLIT_TEMP;
    RBOXC_SPLIT_TEMP = ::core::ptr::null_mut();
    if !temporary.is_null() { fclose(temporary); }
    if RBOXC_SPLIT_INPUT {
        RBOXC_SPLIT_INPUT = false;
        ::libc::close(STDIN_FILENO);
    }
    let buffer = RBOXC_SPLIT_BUFFER;
    RBOXC_SPLIT_BUFFER = ::core::ptr::null_mut();
    ::libc::free(buffer.cast());
    *::libc::__errno_location() = saved_errno;
}
unsafe fn rboxc_close_rr_stream(entry: *mut of_t) -> ::core::ffi::c_int {
    let stream = (*entry).ofile;
    (*entry).ofile = ::core::ptr::null_mut();
    (*entry).ofd = C2Rust_Unnamed_5::OFD_APPEND.0;
    fclose(stream)
}
unsafe fn rboxc_finish_rr_output(entry: *mut of_t) {
    let stream = (*entry).ofile;
    let fd = (*entry).ofd;
    (*entry).ofile = ::core::ptr::null_mut();
    (*entry).ofd = C2Rust_Unnamed_5::OFD_APPEND.0;
    closeout(stream, fd, (*entry).opid, (*entry).of_name);
}
'''
    text = replace_once(text, declaration, helper+declaration)
    anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
    text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_split_resources));')
    # create can fail after opening an existing destination, before returning
    # its descriptor to either output_desc or a round-robin record.
    for anchor in (
        '        let mut fd: ::core::ffi::c_int = open_safer(name, oflags | O_EXCL, MODE_RW_UGO);',
        '        fd = open_safer(name, oflags, MODE_RW_UGO);',
    ):
        text = replace_once(text, anchor, anchor+'\n        RBOXC_SPLIT_PENDING_FD = fd;')
    for anchor in ('        output_desc = create(outfile);',
                   '        (*files.offset(i_check as isize)).ofd = fd;'):
        text = replace_once(text, anchor, anchor+'\n        RBOXC_SPLIT_PENDING_FD = -1;')
    anchor = '    if !fp.is_null() && fclose(fp) != 0 as ::core::ffi::c_int && !ignorable(*__errno_location()) {'
    text = replace_once(text, anchor, '    if fd >= 0 && fd == output_desc { output_desc = -1; }\n'+anchor)
    anchor = '        files = *filesp;'
    text = replace_once(text, anchor, anchor+'\n        RBOXC_SPLIT_FILES = files;')
    anchor = '            (*files.offset(i_file as isize)).opid = 0 as ::core::ffi::c_int as pid_t;'
    text = replace_once(text, anchor, anchor+'\n            RBOXC_SPLIT_FILE_COUNT = i_file as usize + 1;')
    # fclose consumes FILE even on error. Remove ownership before it can
    # trigger a fatal diagnostic; normal descriptor rotation stays unchanged.
    for index in ('i_reopen', 'i_file'):
        anchor = f'fclose((*files.offset({index} as isize)).ofile)'
        text = replace_once(text, anchor, f'rboxc_close_rr_stream(files.offset({index} as isize))')
    anchor = '''                closeout(
                    (*files.offset(i_file as isize)).ofile,
                    (*files.offset(i_file as isize)).ofd,
                    (*files.offset(i_file as isize)).opid,
                    (*files.offset(i_file as isize)).of_name,
                );'''
    text = replace_once(text, anchor, '                rboxc_finish_rr_output(files.offset(i_file as isize));')
    anchor = '    xset_binary_mode(STDIN_FILENO, O_BINARY);'
    text = replace_once(text, anchor, '    RBOXC_SPLIT_INPUT = !streq(infile, b"-\\0".as_ptr().cast());\n'+anchor)
    anchor = '    if close(STDIN_FILENO) != 0 as ::core::ffi::c_int {'
    text = replace_once(text, anchor, '    RBOXC_SPLIT_INPUT = false;\n'+anchor)
    anchor = '    let mut initial_read: ssize_t = -1 as ssize_t;'
    text = replace_once(text, anchor, '    RBOXC_SPLIT_BUFFER = buf;\n'+anchor)
    anchor = '    closeout(\n        ::core::ptr::null_mut::<FILE>(),\n        output_desc,\n        filter_pid,\n        outfile,\n    );'
    text = replace_once(text, anchor, anchor+'\n    RBOXC_SPLIT_BUFFER = ::core::ptr::null_mut();\n    ::libc::free(buf.cast());')
    anchor = '    let mut copied: off_t = 0 as off_t;'
    text = replace_once(text, anchor, '    RBOXC_SPLIT_TEMP = tmp;\n'+anchor)
    anchor = '    r = dup2(fileno_unlocked(tmp), fd) as off_t;'
    text = replace_once(text, anchor, anchor+'\n    if r >= 0 && fd == STDIN_FILENO { RBOXC_SPLIT_INPUT = true; }')
    anchor = '    if fclose(tmp) < 0 as ::core::ffi::c_int {'
    text = replace_once(text, anchor, '    RBOXC_SPLIT_TEMP = ::core::ptr::null_mut();\n'+anchor)
    return text
