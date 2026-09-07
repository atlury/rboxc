"""Pinned GNU exit-time ownership repairs, applied after C2Rust translation."""


def replace_once(text, before, after):
    if after in text:
        return text
    assert text.count(before) == 1, 'GNU cleanup anchor changed: ' + before[:90]
    return text.replace(before, after, 1)


def cleanup(name, text):
    if name == 'wc':
        declaration = 'unsafe extern "C" fn get_input_fstatus('
        helper = '''static mut RBOXC_FILE_LIST: *mut FILE = ::core::ptr::null_mut();
static mut RBOXC_FILE_LIST_CLOSED_STDIN: bool = false;
unsafe extern "C" fn rboxc_close_owned_file_list() {
    let stream = RBOXC_FILE_LIST;
    RBOXC_FILE_LIST = ::core::ptr::null_mut();
    if !stream.is_null() {
        let saved_errno = *::libc::__errno_location();
        fclose(stream);
        *::libc::__errno_location() = saved_errno;
    }
}
unsafe fn rboxc_finish_file_list(stream: *mut FILE) -> ::core::ffi::c_int {
    if stream == RBOXC_FILE_LIST { RBOXC_FILE_LIST = ::core::ptr::null_mut(); }
    if stream == stdin { RBOXC_FILE_LIST_CLOSED_STDIN = true; }
    fclose(stream)
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_close_owned_file_list));')
        anchor = '            stream = fopen(files_from, b"r\\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;'
        text = replace_once(text, anchor, anchor+'\n            RBOXC_FILE_LIST = stream;')
        anchor = '            if !readtokens0(stream, &raw mut tok) || fclose(stream) != 0 as ::core::ffi::c_int {'
        text = replace_once(text, anchor, anchor.replace('fclose(stream)', 'rboxc_finish_file_list(stream)'))
        # A '-' token from a closed stdin list is rejected by the original
        # operand loop. Preserve the failed metadata result without issuing
        # fstat on the descriptor that this entry has just closed.
        anchor = '                fstat(STDIN_FILENO, &raw mut (*fstatus.offset(i as isize)).st)'
        text = replace_once(text, anchor,
                            '                if RBOXC_FILE_LIST_CLOSED_STDIN {\n'
                            '                    *::libc::__errno_location() = ::libc::EBADF;\n'
                            '                    -1\n                } else {\n'+anchor+'\n                }')
    if name == 'ginstall':
        declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_ginstall('
        helper = '''extern "C" { fn hash_free(table: *mut Hash_table); }
static mut RBOXC_INSTALL_DIR_FD: ::core::ffi::c_int = -1;
static mut RBOXC_INSTALL_DEST_INFO: *mut Hash_table = ::core::ptr::null_mut();
static mut RBOXC_INSTALL_STRIP: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_free_install_resources() {
    let saved_errno = *::libc::__errno_location();
    let fd = RBOXC_INSTALL_DIR_FD;
    RBOXC_INSTALL_DIR_FD = -1;
    if fd >= 0 { ::libc::close(fd); }
    let table = RBOXC_INSTALL_DEST_INFO;
    RBOXC_INSTALL_DEST_INFO = ::core::ptr::null_mut();
    if !table.is_null() { hash_free(table); }
    let strip = RBOXC_INSTALL_STRIP;
    RBOXC_INSTALL_STRIP = ::core::ptr::null_mut();
    ::libc::free(strip.cast());
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    atexit(Some(close_stdin as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_install_resources));')
        # Both operand parsing and -D can open the directory. GNU retains it
        # through every copy, including fatal mode/ownership option errors.
        for anchor, fd in (
            ('        target_dirfd = target_directory_operand(target_directory, &raw mut sb);', 'target_dirfd'),
            ('            target_dirfd = fd;', 'fd'),
            ('            *target_dirfd = fd;', 'fd'),
        ):
            text = replace_once(text, anchor, anchor+f'\n        RBOXC_INSTALL_DIR_FD = {fd};')
        anchor = '            dest_info_init(&raw mut x);'
        text = replace_once(text, anchor, anchor+'\n            RBOXC_INSTALL_DEST_INFO = x.dest_info;')
        anchor = '                strip_program = xstrdup(optarg);'
        text = replace_once(text, anchor,
                            '                let next_strip = xstrdup(optarg);\n'
                            '                ::libc::free(RBOXC_INSTALL_STRIP.cast());\n'
                            '                strip_program = next_strip;\n'
                            '                RBOXC_INSTALL_STRIP = next_strip;')
    if name == 'dd':
        declaration = 'unsafe extern "C" fn iclose(mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {'
        helper = '''// Only named inputs/outputs reopened by dd belong to this finalizer.
static mut RBOXC_REOPENED_FDS: [bool; 2] = [false; 2];
unsafe extern "C" fn rboxc_close_reopened_fds() {
    let saved_errno = *::libc::__errno_location();
    for fd in 0..2 {
        if RBOXC_REOPENED_FDS[fd] {
            RBOXC_REOPENED_FDS[fd] = false;
            ::libc::close(fd as ::core::ffi::c_int);
        }
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        text = replace_once(text, declaration, declaration+'\n    if fd >= 0 && fd < 2 { RBOXC_REOPENED_FDS[fd as usize] = false; }')
        anchor = '    atexit(Some(maybe_close_stdout as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_close_reopened_fds));')
        anchor = '    return ret;\n}\nunsafe extern "C" fn ifstat('
        text = replace_once(text, anchor,
                            '    if ret >= 0 && desired_fd >= 0 && desired_fd < 2 {\n'
                            '        RBOXC_REOPENED_FDS[desired_fd as usize] = true;\n    }\n'+anchor)
    if name == 'head':
        declaration = 'unsafe extern "C" fn head_file('
        helper = '''static mut RBOXC_INPUT_FD: ::core::ffi::c_int = -1;
unsafe extern "C" fn rboxc_close_owned_input() {
    let fd = RBOXC_INPUT_FD;
    RBOXC_INPUT_FD = -1;
    if fd >= 0 {
        let saved_errno = *::libc::__errno_location();
        ::libc::close(fd);
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_close_owned_input));')
        anchor = '    ok = head(filename, fd, n_units, count_lines, elide_from_end);'
        text = replace_once(text, anchor, '    RBOXC_INPUT_FD = if is_stdin { -1 } else { fd };\n'
                            +anchor+'\n    RBOXC_INPUT_FD = -1;')
    if name in ('chown', 'mktemp'):
        declaration = f'#[no_mangle]\npub unsafe extern "C" fn single_binary_main_{name}('
        helper = '''static mut RBOXC_OWNED_NAMES: [*mut ::core::ffi::c_char; 2] = [::core::ptr::null_mut(); 2];
unsafe extern "C" fn rboxc_free_owned_names() {
    let saved_errno = *::libc::__errno_location();
    for index in 0..2 {
        let name = RBOXC_OWNED_NAMES[index];
        RBOXC_OWNED_NAMES[index] = ::core::ptr::null_mut();
        ::libc::free(name.cast());
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        finalizer = 'close_stdout' if name == 'chown' else 'maybe_close_stdout'
        anchor = f'    atexit(Some({finalizer} as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_owned_names));')
        if name == 'chown':
            # chown-core owns both option names, including reference lookups
            # and the empty user name synthesized for group-only diagnostics.
            for anchor, index, value in (
                ('            chopt.user_name = uid_to_name(ref_stats.st_uid);', 0, 'chopt.user_name'),
                ('        chopt.group_name = gid_to_name(ref_stats.st_gid);', 1, 'chopt.group_name'),
                ('            chopt.user_name = xstrdup(b"\\0".as_ptr() as *const ::core::ffi::c_char);', 0, 'chopt.user_name'),
            ):
                text = replace_once(text, anchor, anchor+f'\n        RBOXC_OWNED_NAMES[{index}] = {value};')
            anchor = '            &raw mut warn_0,\n        );'
            text = replace_once(text, anchor, anchor+'\n        RBOXC_OWNED_NAMES = [chopt.user_name, chopt.group_name];')
        else:
            # Track the template separately from its randomized destination.
            # Before the final clone, dest_name can alias the template.
            for anchor, value in (
                ('        dest_name = xcharalloc(len.wrapping_add(suffix_len).wrapping_add(1 as size_t));', 'dest_name'),
                ('        template = xstrdup(template);', 'template'),
                ('        free(template as *mut ::core::ffi::c_void);', 'dest_name'),
            ):
                text = replace_once(text, anchor, anchor+f'\n        RBOXC_OWNED_NAMES[0] = {value};')
            anchor = '    dest_name = xstrdup(template);'
            text = replace_once(text, anchor, anchor+'\n    RBOXC_OWNED_NAMES[1] = dest_name;')
    if name == 'stat':
        declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_stat('
        helper = '''static mut RBOXC_DEFAULT_FORMATS: [*mut ::core::ffi::c_char; 2] = [::core::ptr::null_mut(); 2];
unsafe extern "C" fn rboxc_free_default_formats() {
    let saved_errno = *::libc::__errno_location();
    for index in 0..2 {
        let format = RBOXC_DEFAULT_FORMATS[index];
        RBOXC_DEFAULT_FORMATS[index] = ::core::ptr::null_mut();
        ::libc::free(format.cast());
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_default_formats));')
        # GNU default_format returns separate owned strings. Explicit formats
        # borrow argv and alias format2, so only track these allocation sites.
        for index, variable, device in ((0, 'format', 'false'), (1, 'format2', 'true')):
            anchor = f'        {variable} = default_format(fs, terse, r#{device} != 0);'
            text = replace_once(text, anchor, anchor+f'\n        RBOXC_DEFAULT_FORMATS[{index}] = {variable};')
    if name == 'chmod':
        declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_chmod('
        helper = '''static mut RBOXC_MODE_STORAGE: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_free_mode_storage() {
    let storage = RBOXC_MODE_STORAGE;
    RBOXC_MODE_STORAGE = ::core::ptr::null_mut();
    let saved_errno = *::libc::__errno_location();
    ::libc::free(storage.cast());
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()) );'
        anchor = anchor.replace(') );', '));')
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_free_mode_storage));')
        # Only minus-prefixed mode operands use this allocated concatenation.
        # Ordinary argv modes are borrowed and never enter this branch.
        anchor = "                *mode.offset(mode_len as isize) = ',' as ::core::ffi::c_char;"
        text = replace_once(text, anchor, '                RBOXC_MODE_STORAGE = mode;\n'+anchor)
    if name in ('nl', 'base32', 'base64', 'basenc'):
        declaration = ('unsafe extern "C" fn nl_file(mut file: *const ::core::ffi::c_char) -> bool {'
                       if name == 'nl' else
                       'unsafe extern "C" fn finish_and_exit(mut r#in: *mut FILE, mut infile: *const ::core::ffi::c_char) {')
        helper = '''static mut RBOXC_OWNED_INPUT: *mut FILE = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_close_owned_input() {
    let stream = RBOXC_OWNED_INPUT;
    RBOXC_OWNED_INPUT = ::core::ptr::null_mut();
    if !stream.is_null() {
        let saved_errno = *::libc::__errno_location();
        fclose(stream);
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));'
        text = replace_once(text, anchor, anchor+'\n    atexit(Some(rboxc_close_owned_input));')
        stream = 'stream' if name == 'nl' else 'input_fh'
        anchor = f'    fadvise({stream}, fadvice_t::FADVISE_SEQUENTIAL);'
        text = replace_once(text, anchor,
                            f'    RBOXC_OWNED_INPUT = if {stream} != stdin {{ {stream} }} else {{ ::core::ptr::null_mut() }};\n'+anchor)
        if name == 'nl':
            # process_file can terminate on numbering or output errors.
            anchor = '    process_file(stream);'
            text = replace_once(text, anchor, anchor+'\n    RBOXC_OWNED_INPUT = ::core::ptr::null_mut();')
        else:
            # GNU's normal finalizer consumes the FILE even when fclose fails.
            anchor = '    if fclose(r#in) != 0 as ::core::ffi::c_int {'
            text = replace_once(text, anchor, '    RBOXC_OWNED_INPUT = ::core::ptr::null_mut();\n'+anchor)
    if name == 'tee':
        declaration = 'unsafe extern "C" fn fail_output('
        helper = '''static mut RBOXC_OUTPUTS: *mut ::core::ffi::c_int = ::core::ptr::null_mut();
static mut RBOXC_OUTPUT_COUNT: ::core::ffi::c_int = 0;
unsafe fn rboxc_close_output(descriptors: *mut ::core::ffi::c_int, index: ::core::ffi::c_int) -> bool {
    let fd = *descriptors.offset(index as isize);
    *descriptors.offset(index as isize) = -1;
    // Slot zero is borrowed stdout and belongs to close_stdout.
    index == 0 || fd < 0 || close_wait(fd)
}
unsafe extern "C" fn rboxc_close_outputs() {
    let saved_errno = *::libc::__errno_location();
    if !RBOXC_OUTPUTS.is_null() {
        for index in 1..=RBOXC_OUTPUT_COUNT {
            rboxc_close_output(RBOXC_OUTPUTS, index);
        }
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '    let mut w_errno: ::core::ffi::c_int = *__errno_location();'
        text = replace_once(text, anchor, anchor+'\n    rboxc_close_output(descriptors, i);\n    *__errno_location() = w_errno;')
        anchor = '''    ) as *mut ::core::ffi::c_int;
    if pipe_check {'''
        replacement = '''    ) as *mut ::core::ffi::c_int;
    for index in 0..=nfiles {
        *descriptors.offset(index as isize) = -1;
    }
    RBOXC_OUTPUTS = descriptors;
    RBOXC_OUTPUT_COUNT = nfiles;
    atexit(Some(rboxc_close_outputs));
    if pipe_check {'''
        text = replace_once(text, anchor, replacement)
        text = replace_once(text, '!close_wait(*descriptors.offset(i_1 as isize))',
                            '!rboxc_close_output(descriptors, i_1)')
        anchor = '    free(descriptors as *mut ::core::ffi::c_void);'
        text = replace_once(text, anchor, '    RBOXC_OUTPUTS = ::core::ptr::null_mut();\n    RBOXC_OUTPUT_COUNT = 0;\n'+anchor)
    if name == 'comm':
        declaration = 'unsafe extern "C" fn compare_files(mut infiles: *mut *mut ::core::ffi::c_char) {'
        helper = '''static mut RBOXC_INPUTS: [*mut FILE; 2] = [::core::ptr::null_mut(); 2];
unsafe extern "C" fn rboxc_close_inputs() {
    let saved_errno = *::libc::__errno_location();
    for index in 0..2 {
        let stream = RBOXC_INPUTS[index];
        RBOXC_INPUTS[index] = ::core::ptr::null_mut();
        if !stream.is_null() {
            fclose(stream);
        }
    }
    *::libc::__errno_location() = saved_errno;
}
'''
        text = replace_once(text, declaration,
                            helper+declaration+'\n    atexit(Some(rboxc_close_inputs));')
        # Track only successfully opened files. Stdin remains borrowed.
        anchor = '        if streams[i as usize].is_null() {'
        track = ('        RBOXC_INPUTS[i as usize] = if streams[i as usize] != stdin {\n'
                 '            streams[i as usize]\n'
                 '        } else { ::core::ptr::null_mut() };\n')
        text = replace_once(text, anchor, track+anchor)
        # fclose consumes the FILE even on error; remove ownership first.
        anchor = '        if fclose(streams[i_1 as usize]) != 0 as ::core::ffi::c_int {'
        text = replace_once(text, anchor,
                            '        RBOXC_INPUTS[i_1 as usize] = ::core::ptr::null_mut();\n'+anchor)
    if name == 'tsort':
        # GNU's IF_LINT free is omitted by the normal build. Once this edge
        # has been unlinked, no traversal retains it.
        anchor = '                                *p = (*s).next;'
        text = replace_once(text, anchor, anchor+'\n                                ::libc::free(s.cast());')
        declaration = 'unsafe extern "C" fn tsort(mut file: *const ::core::ffi::c_char) {'
        helper = '''static mut RBOXC_REOPENED_INPUT: bool = false;
unsafe extern "C" fn rboxc_close_reopened_input() {
    if RBOXC_REOPENED_INPUT {
        RBOXC_REOPENED_INPUT = false;
        let saved_errno = *::libc::__errno_location();
        fclose(stdin);
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        text = replace_once(text, declaration, helper+declaration)
        # Reaching fadvise means freopen succeeded. On early errors this
        # callback closes the owned input, without closing borrowed stdin.
        anchor = '    fadvise(stdin, fadvice_t::FADVISE_SEQUENTIAL);'
        register = ('    if !is_stdin {\n'
                    '        RBOXC_REOPENED_INPUT = true;\n'
                    '        atexit(Some(rboxc_close_reopened_input));\n'
                    '    }\n')
        text = replace_once(text, anchor, register+anchor)
        anchor = '    if fclose(stdin) != 0 as ::core::ffi::c_int {'
        text = replace_once(text, anchor, '    RBOXC_REOPENED_INPUT = false;\n'+anchor)
    if name == 'ln':
        # GNU keeps the opened target directory until process exit. Register
        # ownership immediately: backup-option errors can exit before the
        # final link loop returns. AT_FDCWD and failed opens are not owned.
        declaration = '#[no_mangle]\npub unsafe extern "C" fn single_binary_main_ln('
        helper = '''static mut RBOXC_DESTDIR_FD: ::core::ffi::c_int = -1;
unsafe extern "C" fn rboxc_close_destdir() {
    let fd = RBOXC_DESTDIR_FD;
    RBOXC_DESTDIR_FD = -1;
    if fd >= 0 {
        let saved_errno = *::libc::__errno_location();
        ::libc::close(fd);
        *::libc::__errno_location() = saved_errno;
    }
}
'''
        text = replace_once(text, declaration, helper+declaration)
        anchor = '            destdir_fd = openat_safer(AT_FDCWD, d, flags);'
        register = (anchor+'\n            if destdir_fd >= 0 {\n'
                    '                RBOXC_DESTDIR_FD = destdir_fd;\n'
                    '                atexit(Some(rboxc_close_destdir));\n'
                    '            }')
        text = replace_once(text, anchor, register)
    if name == 'stdbuf':
        declaration = 'unsafe extern "C" fn set_LD_PRELOAD() {'
        helper = '''// putenv borrows these four possible strings until exec. If exec
// fails, remove the borrowed environment entries before freeing their storage.
static mut RBOXC_ENV_STORAGE: [*mut ::core::ffi::c_char; 4] = [::core::ptr::null_mut(); 4];
static mut RBOXC_ENV_COUNT: usize = 0;
unsafe fn rboxc_putenv_owned(value: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let result = putenv(value);
    if result == 0 {
        assert!(RBOXC_ENV_COUNT < 4);
        RBOXC_ENV_STORAGE[RBOXC_ENV_COUNT] = value;
        RBOXC_ENV_COUNT += 1;
    }
    result
}
unsafe fn rboxc_free_environment() {
    for index in 0..RBOXC_ENV_COUNT {
        let value = RBOXC_ENV_STORAGE[index];
        let equal = ::libc::strchr(value, b'=' as i32);
        assert!(!equal.is_null());
        let length = equal.offset_from(value) as usize;
        let mut name = [0 as ::core::ffi::c_char; 64];
        assert!(length < name.len());
        ::core::ptr::copy_nonoverlapping(value, name.as_mut_ptr(), length);
        ::libc::unsetenv(name.as_ptr());
        ::libc::free(value.cast());
        RBOXC_ENV_STORAGE[index] = ::core::ptr::null_mut();
    }
    RBOXC_ENV_COUNT = 0;
}
'''
        text = replace_once(text, declaration, helper+declaration)
        text = replace_once(text, 'putenv(LD_PRELOAD)', 'rboxc_putenv_owned(LD_PRELOAD)')
        text = replace_once(text, 'putenv(var)', 'rboxc_putenv_owned(var)')
        anchor = '    return exit_status;\n}\npub const MANUAL_URL'
        text = replace_once(text, anchor, '    rboxc_free_environment();\n'+anchor)
    if name == 'hostname':
        text = replace_once(text, '        puts(hostname);',
                            '        puts(hostname);\n        ::libc::free(hostname.cast());')
    if name == 'df':
        anchor = '    return exit_status;\n}\npub const MANUAL_URL'
        text = replace_once(text, anchor, '    ::libc::free(stats.cast());\n'+anchor)
    if name == 'shuf':
        text = replace_once(text, '    fn xmalloc(s: size_t)',
                            '    fn randint_all_free(source: *mut randint_source) -> ::core::ffi::c_int;\n'
                            '    fn xmalloc(s: size_t)')
        # On an early EOF, the slot beyond the returned count can own a
        # buffer. Release it while the allocation count is still in scope.
        text = replace_once(text, '    *out_rsrv = rsrv;',
                            '    let kept = (k as usize).min(n_lines as usize);\n'
                            '    for index in kept..n_alloc_lines as usize {\n'
                            '        freebuffer(rsrv.add(index));\n    }\n'
                            '    *out_rsrv = rsrv;')
        anchor = '    return 0 as ::core::ffi::c_int;\n}\npub const __CHAR_BIT__'
        text = replace_once(text, anchor,
                            '    ::libc::free(permutation.cast());\n'
                            '    randint_all_free(randint_source);\n'
                            '    if !input_lines.is_null() {\n'
                            '        ::libc::free((*input_lines).cast());\n'
                            '        ::libc::free(input_lines.cast());\n    }\n'
                            '    if echo && !line.is_null() {\n'
                            '        ::libc::free((*line).cast());\n    }\n'
                            '    if !reservoir.is_null() {\n'
                            '        for index in 0..n_lines as usize {\n'
                            '            freebuffer(reservoir.add(index));\n        }\n'
                            '        ::libc::free(reservoir.cast());\n    }\n'+anchor)
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
