// Generated from pinned GNU Hello 2.12.3 by scripts/translate-hello.py.
// Source SHA-256: c1cc8989d3fc7b3cc97fca71278b1097b035b755607ea7629753d7a57f107252
/* hello.c -- print a greeting message and exit.

   Copyright 1992-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn wprintf(__format: *const wchar_t, ...) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_hello_rpl_mbsrtowcs"]
    fn rpl_mbsrtowcs(
        dest: *mut wchar_t,
        srcp: *mut *const ::core::ffi::c_char,
        len: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bindtextdomain(
        __domainname: *const ::core::ffi::c_char,
        __dirname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_hello_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_hello_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_hello_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_hello_proper_name"]
    fn proper_name(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_hello_version_etc"]
    fn version_etc(
        stream: *mut FILE,
        command_name: *const ::core::ffi::c_char,
        package: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_hello_emit_bug_reporting_address"]
    fn emit_bug_reporting_address();
    #[link_name = "rboxc_hello_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type wchar_t = ::libc::wchar_t;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: ::core::ffi::c_int,
    pub __value: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __wch: ::core::ffi::c_uint,
    pub __wchb: [::core::ffi::c_char; 4],
}
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
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type mbstate_t = __mbstate_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const OPT_HELP: Self = Self(128);
    pub const OPT_VERSION: Self = Self(129);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _GL_MBSTATE_ZERO_SIZE: usize = ::core::mem::size_of::<mbstate_t>();
#[inline]
unsafe extern "C" fn mbszero(mut ps: *mut mbstate_t) {
    memset(
        ps as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        _GL_MBSTATE_ZERO_SIZE,
    );
}
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOCALEDIR: [::core::ffi::c_char; 44] = unsafe {
    ::core::mem::transmute::<[u8; 44], [::core::ffi::c_char; 44]>(
        *b"/root/rboxc/build/oracle/hello/share/locale\0",
    )
};
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PROGRAM_NAME: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"hello\0") };
unsafe extern "C" fn print_help(mut out: *mut FILE) {
    let mut lc_messages: *const ::core::ffi::c_char =
        setlocale(LC_MESSAGES, ::core::ptr::null::<::core::ffi::c_char>());
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]...\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Print a friendly, customizable greeting.\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        out,
    );
    fputs(b"\n\0".as_ptr() as *const ::core::ffi::c_char, out);
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -t, --traditional       use traditional greeting\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        out,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -g, --greeting=TEXT     use TEXT as the greeting message\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        out,
    );
    fputs(b"\n\0".as_ptr() as *const ::core::ffi::c_char, out);
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --help     display this help and exit\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        out,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --version  output version information and exit\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        out,
    );
    emit_bug_reporting_address();
    if !lc_messages.is_null()
        && strncmp(
            lc_messages,
            b"en_\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 4]>().wrapping_sub(1 as size_t),
        ) != 0
    {
        fprintf(
            out,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Report %s translation bugs to <https://translationproject.org/team/>\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            PACKAGE_NAME.as_ptr(),
        );
    }
    exit(if out == stderr {
        EXIT_FAILURE
    } else {
        EXIT_SUCCESS
    });
}
unsafe extern "C" fn parse_options(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut greeting_msg: *mut *const ::core::ffi::c_char,
) {
    let mut optc: ::core::ffi::c_int = 0;
    let mut lose: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut longopts: [option; 5] = [
        option {
            name: b"greeting\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'g' as ::core::ffi::c_int,
        },
        option {
            name: b"traditional\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 't' as ::core::ffi::c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_0::OPT_HELP.0 as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_0::OPT_VERSION.0 as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    loop {
        optc = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"g:t\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if optc == -1 as ::core::ffi::c_int {
            break;
        }
        's_49: {
            match optc {
                129 => {
                    version_etc(
                        stdout,
                        PROGRAM_NAME.as_ptr(),
                        PACKAGE_NAME.as_ptr(),
                        PACKAGE_VERSION.as_ptr(),
                        proper_name(b"Karl Berry\0".as_ptr() as *const ::core::ffi::c_char),
                        proper_name(b"Sami Kerola\0".as_ptr() as *const ::core::ffi::c_char),
                        proper_name(b"Jim Meyering\0".as_ptr() as *const ::core::ffi::c_char),
                        proper_name(b"Reuben Thomas\0".as_ptr() as *const ::core::ffi::c_char),
                        NULL as *mut ::core::ffi::c_char,
                    );
                    exit(EXIT_SUCCESS);
                }
                103 => {
                    *greeting_msg = optarg;
                    break 's_49;
                }
                128 => {
                    print_help(stdout);
                }
                116 => {}
                _ => {
                    lose = 1 as ::core::ffi::c_int;
                    break 's_49;
                }
            }
            *greeting_msg = dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"hello, world\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            );
        }
    }
    if lose != 0 || optind < argc {
        if !(*argv.offset(optind as isize)).is_null() {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"extra operand\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    *argv.offset(optind as isize),
                );
                if 0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"extra operand\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        *argv.offset(optind as isize),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
        );
        exit(EXIT_FAILURE);
    }
}
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
unsafe extern "C" fn rboxc_hello_error_prefix() {
    fprintf(stderr, b"%s: \0".as_ptr().cast(), program_name);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_hello(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut greeting_msg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut mb_greeting: *mut wchar_t = ::core::ptr::null_mut::<wchar_t>();
    let mut mbstate: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2Rust_Unnamed { __wch: 0 },
    };
    mbszero(&raw mut mbstate);
    let mut len: size_t = 0;
    set_program_name(*argv.offset(0isize));
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_hello_error_prefix);
    }
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    greeting_msg = dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Hello, world!\0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    );
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    parse_options(argc, argv, &raw mut greeting_msg);
    len = strlen(greeting_msg).wrapping_add(1 as size_t);
    mb_greeting = xmalloc(len.wrapping_mul(::core::mem::size_of::<wchar_t>())) as *mut wchar_t;
    len = rpl_mbsrtowcs(mb_greeting, &raw mut greeting_msg, len, &raw mut mbstate);
    if len == -1 as ::core::ffi::c_int as size_t {
        let saved_errno = *__errno_location();
        free(mb_greeting.cast());
        mb_greeting = ::core::ptr::null_mut();
        *__errno_location() = saved_errno;
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"conversion to a multibyte string failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
            );
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"conversion to a multibyte string failed\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    wprintf(
        const {
            unsafe {
                ::core::mem::transmute::<[u8; 20], [::core::ffi::c_int; 5]>(
                    *b"%\0\0\0l\0\0\0s\0\0\0\n\0\0\0\0\0\0\0",
                )
            }
        }
        .as_ptr() as *const wchar_t,
        mb_greeting,
    );
    free(mb_greeting as *mut ::core::ffi::c_void);
    exit(EXIT_SUCCESS);
}
pub const PACKAGE: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"hello\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"GNU Hello\0") };
pub const PACKAGE_VERSION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"2.12.3\0") };
