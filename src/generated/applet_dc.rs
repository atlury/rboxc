// Generated from pinned GNU BC 1.08.2 by scripts/translate-bc.py.
// Source SHA-256: 3e8b10f89231095d3349310b33ea20e7b3b6cd6c4c3329cee7fa92d5208ddca1
/*
 * implement the "dc" Desk Calculator language.
 *
 * Copyright (C) 1994, 1997, 1998, 2000, 2003, 2006, 2008, 2013, 2017, 2022, 2024
 * Free Software Foundation, Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_bc_dc_dc_array_init"]
    fn dc_array_init();
    #[link_name = "rboxc_bc_dc_dc_math_init"]
    fn dc_math_init();
    #[link_name = "rboxc_bc_dc_dc_register_init"]
    fn dc_register_init();
    #[link_name = "rboxc_bc_dc_dc_string_init"]
    fn dc_string_init();
    #[link_name = "rboxc_bc_dc_input_cleanup"]
    fn input_cleanup();
    #[link_name = "rboxc_bc_dc_dc_flush_stdout"]
    fn dc_flush_stdout(_: ::core::ffi::c_int);
    #[link_name = "rboxc_bc_dc_dc_set_interactive_flag"]
    fn dc_set_interactive_flag(_: ::core::ffi::c_int);
    #[link_name = "rboxc_bc_dc_dc_evalfile"]
    fn dc_evalfile(_: *mut FILE) -> ::core::ffi::c_int;
    #[link_name = "rboxc_bc_dc_dc_evalstr"]
    fn dc_evalstr(_: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    #[bitfield(name = "_flags2", ty = "::core::ffi::c_int", bits = "0..=23")]
    pub _flags2: [u8; 3],
    pub _short_backupbuf: [::core::ffi::c_char; 1],
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: ::core::ffi::c_int,
    pub _unused3: ::core::ffi::c_int,
    pub _total_written: __uint64_t,
    pub _unused2: [::core::ffi::c_char; 8],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub const DC_COPYRIGHT: [::core::ffi::c_char; 116] = unsafe {
    ::core::mem::transmute::<
        [u8; 116],
        [::core::ffi::c_char; 116],
    >(
        *b"Copyright 1994, 1997, 1998, 2000, 2001, 2003-2006, 2008, 2010, 2012-2018, 2022, 2024 Free Software Foundation, Inc.\0",
    )
};
pub const DC_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"1.5.2\0") };
pub const PACKAGE: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"bc\0") };
pub const VERSION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.08.2\0") };
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const DC_SUCCESS: ::core::ffi::c_int = 0;
pub const DC_EXIT_REQUEST: ::core::ffi::c_int = 3;
#[export_name = "rboxc_bc_dc_progname"]
pub static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_bc_dc_max_recursion_depth"]
pub static mut max_recursion_depth: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
unsafe extern "C" fn bug_report_info() {
    printf(b"Email bug reports to:  bug-dc@gnu.org .\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn show_version() {
    printf(
        b"dc %s (GNU %s %s)\n\0".as_ptr() as *const ::core::ffi::c_char,
        DC_VERSION.as_ptr(),
        PACKAGE.as_ptr(),
        VERSION.as_ptr(),
    );
    printf(
        b"\n%s\nThis is free software; see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE,\nto the extent permitted by law.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        DC_COPYRIGHT.as_ptr(),
    );
}
unsafe extern "C" fn usage(mut f: *mut FILE) {
    fprintf(
        f,
        b"Usage: %s [OPTION] [file ...]\n  -e, --expression=EXPR    evaluate expression\n  -f, --file=FILE          evaluate contents of file\n  -i, --interactive        force interactive mode\n  --max-recursion=depth    limit recursion depth\n  -h, --help               display this help and exit\n  -V, --version            output version information and exit\n\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        progname,
    );
    bug_report_info();
}
unsafe extern "C" fn r1bindex(
    mut s: *mut ::core::ffi::c_char,
    mut c: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = strrchr(s, c);
    return if !p.is_null() {
        p.offset(1 as ::core::ffi::c_int as isize)
    } else {
        s
    };
}
unsafe extern "C" fn try_file(mut filename: *const ::core::ffi::c_char) {
    let mut input: *mut FILE = ::core::ptr::null_mut::<FILE>();
    if filename.is_null()
        || strcmp(filename, b"-\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        input = stdin;
        filename = b"(stdin)\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        input = fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
        if input.is_null() {
            fprintf(
                stderr,
                b"%s: Could not open file %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                progname,
                filename,
            );
            return;
        }
    }
    let mut s: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if fstat(fileno(input), &raw mut s) == -1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"%s: Could not fstat file \0".as_ptr() as *const ::core::ffi::c_char,
            progname,
        );
        perror(filename);
    } else if s.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
        fprintf(
            stderr,
            b"%s: Will not attempt to process directory %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            progname,
            filename,
        );
    } else if s.st_mode & __S_IFMT as __mode_t == 0o60000 as __mode_t {
        fprintf(
            stderr,
            b"%s: Will not attempt to process block-special file %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            progname,
            filename,
        );
    } else if !(s.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t)
        && !(s.st_mode & __S_IFMT as __mode_t == 0o20000 as __mode_t)
        && !(s.st_mode & __S_IFMT as __mode_t == 0o10000 as __mode_t)
        && !(s.st_mode & __S_IFMT as __mode_t == 0o140000 as __mode_t)
    {
        fprintf(
            stderr,
            b"%s: Will not attempt to process file of unusual type: %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            progname,
            filename,
        );
    } else {
        match dc_evalfile(input) {
            DC_SUCCESS => {}
            DC_EXIT_REQUEST => {
                exit(EXIT_SUCCESS);
            }
            _ => {
                exit(EXIT_FAILURE);
            }
        }
    }
    if input == stdin {
        input_cleanup();
    } else {
        fclose(input);
    };
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_dc(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut long_opts: [option; 6] = [
        option {
            name: b"expression\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'e' as ::core::ffi::c_int,
        },
        option {
            name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'f' as ::core::ffi::c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'V' as ::core::ffi::c_int,
        },
        option {
            name: b"max-recursion\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 256 as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut interactive: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut did_eval: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    progname = r1bindex(*argv, '/' as ::core::ffi::c_int);
    dc_math_init();
    dc_string_init();
    dc_register_init();
    dc_array_init();
    loop {
        c = getopt_long(
            argc,
            argv,
            b"hVe:f:i\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const long_opts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if c == EOF {
            break;
        }
        match c {
            101 => {
                c = dc_evalstr(optarg);
                if c != DC_SUCCESS {
                    dc_flush_stdout(0 as ::core::ffi::c_int);
                    return if c == DC_EXIT_REQUEST {
                        EXIT_SUCCESS
                    } else {
                        EXIT_FAILURE
                    };
                }
                did_eval = 1 as ::core::ffi::c_int;
            }
            102 => {
                try_file(optarg);
                did_eval = 1 as ::core::ffi::c_int;
            }
            104 => {
                usage(stdout);
                dc_flush_stdout(1 as ::core::ffi::c_int);
                return EXIT_SUCCESS;
            }
            105 => {
                interactive = 1 as ::core::ffi::c_int;
            }
            86 => {
                show_version();
                dc_flush_stdout(1 as ::core::ffi::c_int);
                return EXIT_SUCCESS;
            }
            256 => {
                max_recursion_depth = strtol(
                    optarg,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                ) as ::core::ffi::c_uint;
            }
            _ => {
                usage(stderr);
                return EXIT_FAILURE;
            }
        }
    }
    dc_set_interactive_flag(interactive);
    if optind < argc {
        did_eval = 0 as ::core::ffi::c_int;
    }
    while optind < argc {
        try_file(*argv.offset(optind as isize));
        optind += 1;
    }
    if did_eval == 0 as ::core::ffi::c_int {
        try_file(::core::ptr::null::<::core::ffi::c_char>());
    }
    dc_flush_stdout(1 as ::core::ffi::c_int);
    if ferror(stderr) != 0 || fclose(stderr) != 0 {
        return EXIT_FAILURE;
    }
    return EXIT_SUCCESS;
}
