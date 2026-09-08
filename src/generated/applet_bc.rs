// Generated from pinned GNU BC 1.08.2 by scripts/translate-bc.py.
// Source SHA-256: 95880c007a0db461b9adbba2e8f9efc25c1aaad7c348661e62c8328fefb8c68a
/*  This file is part of GNU bc.

    Copyright (C) 1991-1994, 1997, 2006, 2008, 2012-2017, 2024, 2025
    Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3 of the License , or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; see the file COPYING.  If not, see
    <http://www.gnu.org/licenses>.

    You may contact the author by:
       e-mail:  philnelson@acm.org
      us-mail:  Philip A. Nelson
                Computer Science Department, 9062
                Western Washington University
                Bellingham, WA 98226-9062
       
*************************************************************************/
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
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bc_bc_interactive"]
    static mut interactive: ::core::ffi::c_char;
    #[link_name = "rboxc_bc_bc_compile_only"]
    static mut compile_only: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_use_math"]
    static mut use_math: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_warn_not_std"]
    static mut warn_not_std: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_std_only"]
    static mut std_only: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_quiet"]
    static mut quiet: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_file_names"]
    static mut file_names: *mut file_node;
    #[link_name = "rboxc_bc_bc_file_name"]
    static mut file_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bc_bc_is_std_in"]
    static mut is_std_in: ::core::ffi::c_char;
    #[link_name = "rboxc_bc_bc_line_size"]
    static mut line_size: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_line_no"]
    static mut line_no: ::core::ffi::c_int;
    static mut optind: ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_yyin"]
    static mut yyin: *mut FILE;
    #[link_name = "rboxc_bc_bc_libmath"]
    static mut libmath: [*const ::core::ffi::c_char; 0];
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn srandom(__seed: ::core::ffi::c_uint);
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bc_bc_init_gen"]
    fn init_gen();
    #[link_name = "rboxc_bc_bc_init_tree"]
    fn init_tree();
    #[link_name = "rboxc_bc_bc_lookup"]
    fn lookup(name: *mut ::core::ffi::c_char, namekind: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_bc_bc_bc_malloc"]
    fn bc_malloc(_: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_bc_bc_show_bc_version"]
    fn show_bc_version();
    #[link_name = "rboxc_bc_bc_bc_exit"]
    fn bc_exit(_: ::core::ffi::c_int) -> !;
    #[link_name = "rboxc_bc_bc_init_load"]
    fn init_load();
    #[link_name = "rboxc_bc_bc_load_code"]
    fn load_code(code: *const ::core::ffi::c_char);
    #[link_name = "rboxc_bc_bc_init_storage"]
    fn init_storage();
    #[link_name = "rboxc_bc_bc_yyparse"]
    fn yyparse() -> ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
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
pub type ssize_t = isize;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_node {
    pub name: *mut ::core::ffi::c_char,
    pub next: *mut file_node,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub const _IOLBF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const FALSE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FUNCT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
static mut first_file: ::core::ffi::c_char = 0;
static mut last: *mut file_node = ::core::ptr::null_mut::<file_node>();
static mut long_options: [option; 9] = unsafe {
    [
        option {
            name: b"compile\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const compile_only as *mut ::core::ffi::c_int,
            val: TRUE,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"interactive\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'i' as ::core::ffi::c_int,
        },
        option {
            name: b"mathlib\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const use_math as *mut ::core::ffi::c_int,
            val: TRUE,
        },
        option {
            name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const quiet as *mut ::core::ffi::c_int,
            val: TRUE,
        },
        option {
            name: b"standard\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const std_only as *mut ::core::ffi::c_int,
            val: TRUE,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as ::core::ffi::c_int,
        },
        option {
            name: b"warn\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw const warn_not_std as *mut ::core::ffi::c_int,
            val: TRUE,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn usage(mut progname: *const ::core::ffi::c_char) {
    printf(
        b"usage: %s [options] [file ...]\n%s%s%s%s%s%s%s%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        progname,
        b"  -h  --help         print this usage and exit\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -i  --interactive  force interactive mode\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -l  --mathlib      use the predefined math routines\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -q  --quiet        don't print initial banner\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -s  --standard     non-standard bc constructs are errors\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -w  --warn         warn about non-standard bc constructs\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -v  --version      print version information and exit\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"\nPlease report bugs to <bug-bc@gnu.org>\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn parse_args(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut optch: ::core::ffi::c_int = 0;
    let mut long_index: ::core::ffi::c_int = 0;
    let mut temp: *mut file_node = ::core::ptr::null_mut::<file_node>();
    srandom(time(::core::ptr::null_mut::<time_t>()) as ::core::ffi::c_uint);
    optind = 0 as ::core::ffi::c_int;
    loop {
        optch = getopt_long(
            argc,
            argv,
            b"chilqswv\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            &raw mut long_index,
        );
        if optch == EOF {
            break;
        }
        match optch {
            0 => {}
            99 => {
                compile_only = TRUE;
            }
            104 => {
                usage(*argv.offset(0isize));
                bc_exit(0 as ::core::ffi::c_int);
            }
            105 => {
                interactive = TRUE as ::core::ffi::c_char;
            }
            108 => {
                use_math = TRUE;
            }
            113 => {
                quiet = TRUE;
            }
            115 => {
                std_only = TRUE;
            }
            118 => {
                show_bc_version();
                bc_exit(0 as ::core::ffi::c_int);
            }
            119 => {
                warn_not_std = TRUE;
            }
            _ => {
                usage(*argv.offset(0isize));
                bc_exit(1 as ::core::ffi::c_int);
            }
        }
    }
    while optind < argc {
        temp = bc_malloc(::core::mem::size_of::<file_node>()) as *mut file_node;
        (*temp).name = *argv.offset(optind as isize);
        (*temp).next = ::core::ptr::null_mut::<file_node>();
        if last.is_null() {
            file_names = temp;
        } else {
            (*last).next = temp as *mut file_node;
        }
        last = temp;
        optind += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_bc(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut env_value: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut env_argv: [*mut ::core::ffi::c_char; 30] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 30];
    let mut env_argc: ::core::ffi::c_int = 0;
    if isatty(0 as ::core::ffi::c_int) != 0 && isatty(1 as ::core::ffi::c_int) != 0 {
        interactive = TRUE as ::core::ffi::c_char;
    }
    setvbuf(
        stdout,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        _IOLBF,
        0 as size_t,
    );
    env_value = getenv(b"BC_ENV_ARGS\0".as_ptr() as *const ::core::ffi::c_char);
    if !env_value.is_null() {
        env_argc = 1 as ::core::ffi::c_int;
        env_argv[0usize] = strdup(b"BC_ENV_ARGS\0".as_ptr() as *const ::core::ffi::c_char);
        while *env_value as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            if *env_value as ::core::ffi::c_int != ' ' as ::core::ffi::c_int {
                let c2rust_fresh0 = env_argc;
                env_argc += 1;
                env_argv[c2rust_fresh0 as usize] = env_value;
                while *env_value as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    && *env_value as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    env_value = env_value.offset(1);
                }
                if *env_value as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    *env_value = 0 as ::core::ffi::c_char;
                    env_value = env_value.offset(1);
                }
            } else {
                env_value = env_value.offset(1);
            }
        }
        parse_args(env_argc, &raw mut env_argv as *mut *mut ::core::ffi::c_char);
    }
    parse_args(argc, argv);
    if !getenv(b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        std_only = TRUE;
    }
    env_value = getenv(b"BC_LINE_LENGTH\0".as_ptr() as *const ::core::ffi::c_char);
    if !env_value.is_null() {
        line_size = atoi(env_value);
        if line_size < 3 as ::core::ffi::c_int && line_size != 0 as ::core::ffi::c_int {
            line_size = 70 as ::core::ffi::c_int;
        }
    } else {
        line_size = 70 as ::core::ffi::c_int;
    }
    init_storage();
    init_load();
    if interactive != 0 {
        signal(
            SIGINT,
            Some(use_quit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
    }
    init_tree();
    init_gen();
    is_std_in = FALSE as ::core::ffi::c_char;
    first_file = TRUE as ::core::ffi::c_char;
    if open_new_file() == 0 {
        bc_exit(1 as ::core::ffi::c_int);
    }
    yyparse();
    if compile_only != 0 {
        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    bc_exit(0 as ::core::ffi::c_int);
}
#[export_name = "rboxc_bc_bc_open_new_file"]
pub unsafe extern "C" fn open_new_file() -> ::core::ffi::c_int {
    let mut new_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut temp: *mut file_node = ::core::ptr::null_mut::<file_node>();
    line_no = 1 as ::core::ffi::c_int;
    if is_std_in != 0 {
        return FALSE;
    }
    if use_math != 0 && first_file as ::core::ffi::c_int != 0 {
        let mut mstr: *mut *const ::core::ffi::c_char =
            ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
        lookup(strdup(b"e\0".as_ptr() as *const ::core::ffi::c_char), FUNCT);
        lookup(strdup(b"l\0".as_ptr() as *const ::core::ffi::c_char), FUNCT);
        lookup(strdup(b"s\0".as_ptr() as *const ::core::ffi::c_char), FUNCT);
        lookup(strdup(b"a\0".as_ptr() as *const ::core::ffi::c_char), FUNCT);
        lookup(strdup(b"c\0".as_ptr() as *const ::core::ffi::c_char), FUNCT);
        lookup(strdup(b"j\0".as_ptr() as *const ::core::ffi::c_char), FUNCT);
        mstr = &raw mut libmath as *mut *const ::core::ffi::c_char;
        while !(*mstr).is_null() {
            load_code(*mstr);
            mstr = mstr.offset(1);
        }
    }
    if !file_names.is_null() {
        new_file = fopen(
            (*file_names).name,
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if !new_file.is_null() {
            new_yy_file(new_file);
            temp = file_names;
            file_name = (*temp).name;
            file_names = (*temp).next as *mut file_node;
            free(temp as *mut ::core::ffi::c_void);
            return TRUE;
        }
        fprintf(
            stderr,
            b"File %s is unavailable.\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*file_names).name,
        );
        bc_exit(1 as ::core::ffi::c_int);
    }
    new_yy_file(stdin);
    is_std_in = TRUE as ::core::ffi::c_char;
    return TRUE;
}
#[export_name = "rboxc_bc_bc_new_yy_file"]
pub unsafe extern "C" fn new_yy_file(mut file: *mut FILE) {
    if first_file == 0 {
        fclose(yyin);
    }
    yyin = file;
    first_file = FALSE as ::core::ffi::c_char;
}
#[export_name = "rboxc_bc_bc_use_quit"]
pub unsafe extern "C" fn use_quit(mut sig: ::core::ffi::c_int) {
    write(
        1 as ::core::ffi::c_int,
        b"\n(interrupt) Exiting bc.\n\0".as_ptr() as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        26 as size_t,
    );
    bc_exit(0 as ::core::ffi::c_int);
}
