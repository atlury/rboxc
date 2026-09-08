// Generated from pinned GNU frcode 4.11.0 by scripts/translate-entry-provider.py.
// Source SHA-256: 3a06418e81686e3602a4730ef02d560ae62ab95f237d144f0aa0bfaeec592144
/* frcode -- front-compress a sorted list
   Copyright (C) 1994-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn free(_: *mut ::core::ffi::c_void);
    #[link_name = "rboxc_frcode_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_frcode_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_frcode_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_frcode_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    fn dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_frcode_explain_how_to_report_bugs"]
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_frcode_display_findutils_version"]
    fn display_findutils_version(official_name: *const ::core::ffi::c_char);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub const SHRT_MAX: ::core::ffi::c_int = __SHRT_MAX__;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const LONG_MAX: ::core::ffi::c_long = __LONG_MAX__;
pub const SHRT_MIN: ::core::ffi::c_int = -__SHRT_MAX__ - 1 as ::core::ffi::c_int;
pub const LONG_MIN: ::core::ffi::c_long = -__LONG_MAX__ - 1 as ::core::ffi::c_long;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 27] = unsafe {
    ::core::mem::transmute::<[u8; 27], [::core::ffi::c_char; 27]>(*b"int put_short(int, FILE *)\0")
};
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LOCATEDB_MAGIC: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\0LOCATE02\0") };
pub const LOCATEDB_ESCAPE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const LOCATEDB_ONEBYTE_MAX: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const LOCATEDB_ONEBYTE_MIN: ::core::ffi::c_int = -127 as ::core::ffi::c_int;
unsafe extern "C" fn put_short(mut c: ::core::ffi::c_int, mut fp: *mut FILE) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if c <= 32767 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"c <= SHRT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/frcode.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                102 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    '_c2rust_label_0: {
        if c >= -32767 as ::core::ffi::c_int - 1 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"c >= SHRT_MIN\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/frcode.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                103 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    return (putc(c >> 8 as ::core::ffi::c_int, fp) != EOF && putc(c, fp) != EOF)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn prefix_length(
    mut s1: *mut ::core::ffi::c_char,
    mut s2: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut start: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut limit: ::core::ffi::c_int = INT_MAX;
    start = s1;
    while *s1 as ::core::ffi::c_int == *s2 as ::core::ffi::c_int
        && *s1 as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
    {
        limit -= 1;
        if 0 as ::core::ffi::c_int == limit {
            break;
        }
        s1 = s1.offset(1);
        s2 = s2.offset(1);
    }
    return s1.offset_from(start) as ::core::ffi::c_int;
}
static mut longopts: [option; 4] = [
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
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"null\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: '0' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) -> ! {
    if status != EXIT_SUCCESS {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [-0 | --null] [--version] [--help]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        program_name,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
unsafe extern "C" fn get_seclevel(mut s: *mut ::core::ffi::c_char) -> ::core::ffi::c_long {
    let mut result: ::core::ffi::c_long = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    result = strtol(s, &raw mut p, 10 as ::core::ffi::c_int);
    if 0 as ::core::ffi::c_long == result && p == optarg {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"You need to specify a security level as a decimal integer.\0".as_ptr()
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
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"You need to specify a security level as a decimal integer.\0".as_ptr()
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
        return -1 as ::core::ffi::c_long;
    } else if (LONG_MIN == result || LONG_MAX == result) && *__errno_location() != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Security level %s is outside the convertible range.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                s,
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
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Security level %s is outside the convertible range.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        return -1 as ::core::ffi::c_long;
    } else if *p != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Security level %s has unexpected suffix %s.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                s,
                p,
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
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Security level %s has unexpected suffix %s.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                    p,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        return -1 as ::core::ffi::c_long;
    } else {
        return result;
    };
}
unsafe extern "C" fn outerr() {
    if 0 != 0 {
        error(
            1 as ::core::ffi::c_int,
            *__errno_location(),
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"write error\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"write error\0".as_ptr() as *const ::core::ffi::c_char,
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
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_frcode(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut oldpath: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut pathsize: size_t = 0;
    let mut oldpathsize: size_t = 0;
    let mut count: ::core::ffi::c_int = 0;
    let mut oldcount: ::core::ffi::c_int = 0;
    let mut diffcount: ::core::ffi::c_int = 0;
    let mut line_len: ::core::ffi::c_int = 0;
    let mut delimiter: ::core::ffi::c_int = '\n' as ::core::ffi::c_int;
    let mut optc: ::core::ffi::c_int = 0;
    let mut slocate_compat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut slocate_seclevel: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    if !(*argv.offset(0isize)).is_null() {
        set_program_name(*argv.offset(0isize));
    } else {
        set_program_name(b"frcode\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"The atexit library function failed\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"The atexit library function failed\0".as_ptr()
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
    oldpathsize = 1026 as size_t;
    pathsize = oldpathsize;
    path = xmalloc(pathsize) as *mut ::core::ffi::c_char;
    oldpath = xmalloc(oldpathsize) as *mut ::core::ffi::c_char;
    *oldpath.offset(0isize) = 0 as ::core::ffi::c_char;
    oldcount = 0 as ::core::ffi::c_int;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"hv0S:\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if optc == -1 as ::core::ffi::c_int {
            break;
        }
        match optc {
            48 => {
                delimiter = 0 as ::core::ffi::c_int;
            }
            83 => {
                slocate_compat = 1 as ::core::ffi::c_int;
                slocate_seclevel = get_seclevel(optarg);
                if slocate_seclevel < 0 as ::core::ffi::c_long
                    || slocate_seclevel > 1 as ::core::ffi::c_long
                {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"slocate security level %ld is unsupported.\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            slocate_seclevel,
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
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"slocate security level %ld is unsupported.\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                slocate_seclevel,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
            }
            104 => {
                usage(EXIT_SUCCESS);
            }
            118 => {
                display_findutils_version(b"frcode\0".as_ptr() as *const ::core::ffi::c_char);
                return 0 as ::core::ffi::c_int;
            }
            _ => {
                usage(EXIT_FAILURE);
            }
        }
    }
    if optind != argc {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"no argument expected.\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"no argument expected.\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        usage(EXIT_FAILURE);
    }
    if slocate_compat != 0 {
        fputc(
            if slocate_seclevel != 0 {
                '1' as ::core::ffi::c_int
            } else {
                '0' as ::core::ffi::c_int
            },
            stdout,
        );
        fputc(0 as ::core::ffi::c_int, stdout);
    } else if fwrite(
        LOCATEDB_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        1 as size_t,
        ::core::mem::size_of::<[::core::ffi::c_char; 10]>(),
        stdout,
    ) as usize
        != ::core::mem::size_of::<[::core::ffi::c_char; 10]>()
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Failed to write to standard output\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"Failed to write to standard output\0".as_ptr()
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
    loop {
        line_len =
            getdelim(&raw mut path, &raw mut pathsize, delimiter, stdin) as ::core::ffi::c_int;
        if line_len <= 0 as ::core::ffi::c_int {
            break;
        }
        if *path.offset((line_len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            != delimiter
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"The input file should end with the delimiter\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"The input file should end with the delimiter\0".as_ptr()
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
        } else {
            *path.offset((line_len - 1 as ::core::ffi::c_int) as isize) =
                '\0' as ::core::ffi::c_char;
        }
        count = prefix_length(oldpath, path);
        diffcount = count - oldcount;
        if diffcount > SHRT_MAX || diffcount < SHRT_MIN {
            count = 0 as ::core::ffi::c_int;
            diffcount = -oldcount;
        }
        oldcount = count;
        if slocate_compat != 0 {
            slocate_compat = 0 as ::core::ffi::c_int;
        } else if diffcount < LOCATEDB_ONEBYTE_MIN || diffcount > LOCATEDB_ONEBYTE_MAX {
            if EOF == putc(LOCATEDB_ESCAPE, stdout) {
                outerr();
            }
            if put_short(diffcount, stdout) == 0 {
                outerr();
            }
        } else if EOF == putc(diffcount, stdout) {
            outerr();
        }
        if EOF == fputs(path.offset(count as isize), stdout)
            || EOF == putc('\0' as ::core::ffi::c_int, stdout)
        {
            outerr();
        }
        let mut tmppath: *mut ::core::ffi::c_char = oldpath;
        let mut tmppathsize: size_t = oldpathsize;
        oldpath = path;
        oldpathsize = pathsize;
        path = tmppath;
        pathsize = tmppathsize;
    }
    free(path as *mut ::core::ffi::c_void);
    free(oldpath as *mut ::core::ffi::c_void);
    return 0 as ::core::ffi::c_int;
}
pub const __SHRT_MAX__: ::core::ffi::c_int = 32767 as ::core::ffi::c_int;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
