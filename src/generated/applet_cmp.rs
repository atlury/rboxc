// Generated from pinned GNU Diffutils 3.12 by scripts/translate-diffutils.py.
// Source SHA-256: 9fc2b64cecfaa008fde27ec0ee177b8a17ac3b05d851e06e40f6db62e276d42c
/* GNU cmp - compare two files byte by byte

   Copyright (C) 1990-1996, 1998, 2001-2002, 2004, 2006-2007, 2009-2013,
   2015-2025 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct incomplete { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn rawmemchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
    #[link_name = "rboxc_diffutils_proper_name_lite"]
    fn proper_name_lite(
        name_ascii: *const ::core::ffi::c_char,
        name_utf8: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_Version"]
    static mut Version: *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_same_file"]
    fn same_file(_: *const stat, _: *const stat) -> bool;
    #[link_name = "rboxc_diffutils_stat_size"]
    fn stat_size(_: *const stat) -> off_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fputs_unlocked(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_c_stack_action"]
    fn c_stack_action(
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_block_read"]
    fn block_read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char, _: idx_t) -> ptrdiff_t;
    #[link_name = "rboxc_diffutils_buffer_lcm"]
    fn buffer_lcm(_: idx_t, _: idx_t, _: idx_t) -> idx_t;
    #[link_name = "rboxc_diffutils_squote"]
    fn squote(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_try_help"]
    fn try_help(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_diffutils_exit_failure"]
    static mut exit_failure: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_hard_locale"]
    fn hard_locale(category: ::core::ffi::c_int) -> bool;
    #[link_name = "rboxc_diffutils_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_quote"]
    fn quote(arg: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_version_etc"]
    fn version_etc(
        stream: *mut FILE,
        command_name: *const ::core::ffi::c_char,
        package: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_diffutils_emit_bug_reporting_address"]
    fn emit_bug_reporting_address();
    #[link_name = "rboxc_diffutils_xinmalloc"]
    fn xinmalloc(n: idx_t, s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xstdopen"]
    fn xstdopen();
    #[link_name = "rboxc_diffutils_xstrtoimax"]
    fn xstrtoimax(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut intmax_t,
        _: *const ::core::ffi::c_char,
    ) -> strtol_error;
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
pub type off_t = __off_t;
pub type ptrdiff_t = isize;
pub type size_t = usize;
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
pub type intmax_t = ::libc::intmax_t;
pub type idx_t = ptrdiff_t;
pub type word = *mut incomplete;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const EXIT_TROUBLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct strtol_error(pub ::core::ffi::c_uint);
impl strtol_error {
    pub const LONGINT_OK: Self = Self(0);
    pub const LONGINT_OVERFLOW: Self = Self(1);
    pub const LONGINT_INVALID_SUFFIX_CHAR: Self = Self(2);
    pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: Self = Self(3);
    pub const LONGINT_INVALID: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct comparison_type(pub ::core::ffi::c_uint);
impl comparison_type {
    pub const type_first_diff: Self = Self(0);
    pub const type_all_diffs: Self = Self(1);
    pub const type_no_stdout: Self = Self(2);
    pub const type_status: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const HELP_OPTION: Self = Self(128);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const DEV_BSIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INTMAX_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const PTRDIFF_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const IDX_MAX: ::core::ffi::c_long = PTRDIFF_MAX;
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const NULL_DEVICE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"/dev/null\0") };
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return O_BINARY;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return __gl_setmode(fd, mode);
}
#[inline]
unsafe extern "C" fn c_isprint(mut c: ::core::ffi::c_int) -> bool {
    match c {
        32 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 | 97 | 98 | 99 | 100 | 101 | 102
        | 103 | 104 | 105 | 106 | 107 | 108 | 109 | 110 | 111 | 112 | 113 | 114 | 115 | 116
        | 117 | 118 | 119 | 120 | 121 | 122 | 33 | 34 | 35 | 36 | 37 | 38 | 39 | 40 | 41 | 42
        | 43 | 44 | 45 | 46 | 47 | 58 | 59 | 60 | 61 | 62 | 63 | 64 | 91 | 92 | 93 | 94 | 95
        | 96 | 123 | 124 | 125 | 126 | 65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75
        | 76 | 77 | 78 | 79 | 80 | 81 | 82 | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return r#true != 0
        }
        _ => return r#false != 0,
    };
}
static mut PROGRAM_NAME: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"cmp\0") };
unsafe extern "C" fn hard_locale_LC_MESSAGES() -> bool {
    return hard_locale(LC_MESSAGES);
}
static mut file: [*const ::core::ffi::c_char; 2] = [::core::ptr::null::<::core::ffi::c_char>(); 2];
static mut file_desc: [::core::ffi::c_int; 2] = [0; 2];
static mut stat_buf: [stat; 2] = [stat {
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
}; 2];
static mut buffer: [*mut word; 2] = [::core::ptr::null_mut::<word>(); 2];
static mut buf_size: idx_t = 0;
static mut ignore_initial: [intmax_t; 2] = [0; 2];
static mut bytes: intmax_t = INTMAX_MAX as intmax_t;
static mut comparison_type_0: comparison_type = comparison_type::type_first_diff;
static mut opt_print_bytes: bool = false;
static mut shortopts: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"bci:ln:sv\0") };
static mut longopts: [option; 10] = [
    option {
        name: b"print-bytes\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"print-chars\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-initial\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'i' as ::core::ffi::c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_0::HELP_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
static mut valid_suffixes: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"kKMGTPEZY0\0") };
unsafe extern "C" fn specify_ignore_initial(
    mut f: ::core::ffi::c_int,
    mut argptr: *mut *mut ::core::ffi::c_char,
    mut delimiter: ::core::ffi::c_char,
) {
    let mut val: intmax_t = 0;
    let mut arg: *const ::core::ffi::c_char = *argptr;
    let mut d: strtol_error = xstrtoimax(
        arg,
        argptr,
        0 as ::core::ffi::c_int,
        &raw mut val,
        &raw const valid_suffixes as *const ::core::ffi::c_char,
    );
    let mut e: strtol_error = strtol_error(
        d.0 & !(strtol_error::LONGINT_OVERFLOW.0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    if !((e.0 == strtol_error::LONGINT_OK.0
        || e.0 == strtol_error::LONGINT_INVALID_SUFFIX_CHAR.0
            && **argptr as ::core::ffi::c_int == delimiter as ::core::ffi::c_int)
        && 0 as intmax_t <= val)
    {
        try_help(
            b"invalid --ignore-initial value %s\0".as_ptr() as *const ::core::ffi::c_char,
            quote(arg),
        );
    }
    if 0 as intmax_t <= ignore_initial[f as usize] && ignore_initial[f as usize] < val {
        ignore_initial[f as usize] = if d.0 == e.0 { val } else { -1 as intmax_t };
    }
}
unsafe extern "C" fn specify_comparison_type(mut t: comparison_type) {
    if comparison_type_0.0 != 0 && comparison_type_0.0 != t.0 {
        try_help(
            b"options -l and -s are incompatible\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    comparison_type_0 = t as comparison_type;
}
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        if 0 != 0 {
            error(
                C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"write failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
            );
            if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int =
                    C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"write failed\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else if fclose(stdout) != 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                *__errno_location(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"standard output\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
            );
            if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int =
                    C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"standard output\0".as_ptr() as *const ::core::ffi::c_char,
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
}
static mut option_help_msgid: [*const ::core::ffi::c_char; 9] = [
    b"-b, --print-bytes          print differing bytes\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-i, --ignore-initial=SKIP         skip first SKIP bytes of both inputs\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-i, --ignore-initial=SKIP1:SKIP2  skip first SKIP1 bytes of FILE1 and\n                                      first SKIP2 bytes of FILE2\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-l, --verbose              output byte numbers and differing byte values\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-n, --bytes=LIMIT          compare at most LIMIT bytes\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-s, --quiet, --silent      suppress all normal output\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --help                 display this help and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-v, --version              output version information and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... FILE1 [FILE2 [SKIP1 [SKIP2]]]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        squote(0 as ::core::ffi::c_int, program_name),
    );
    puts(dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Compare two files byte by byte.\0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    ));
    printf(
        b"\n%s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"The optional SKIP1 and SKIP2 specify the number of bytes to skip\nat the beginning of each file (zero by default).\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fputs_unlocked(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Mandatory arguments to long options are mandatory for short options too.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    let mut p: *const *const ::core::ffi::c_char =
        &raw const option_help_msgid as *const *const ::core::ffi::c_char;
    while !(*p).is_null() {
        printf(
            b"  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            dcgettext(::core::ptr::null::<::core::ffi::c_char>(), *p, LC_MESSAGES),
        );
        p = p.offset(1);
    }
    printf(
        b"\n%s\n\n%s\n%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"SKIP values may be followed by the following multiplicative suffixes:\nkB 1000, K 1024, MB 1,000,000, M 1,048,576,\nGB 1,000,000,000, G 1,073,741,824, and so on for T, P, E, Z, Y.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"If a FILE is '-' or missing, read standard input.\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    emit_bug_reporting_address();
}
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut rboxc_diffutils_stderr: *mut libc::FILE;
}
unsafe extern "C" fn rboxc_diffutils_error_prefix() {
    libc::fprintf(rboxc_diffutils_stderr, b"%s: \0".as_ptr().cast(), program_name);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_cmp(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ::core::ptr::write_volatile(
        &raw mut exit_failure,
        C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
    );
    set_program_name(*argv.offset(0isize));
    let prior_error_prefix = error_print_progname;
    if prior_error_prefix.is_none() {
        error_print_progname = Some(rboxc_diffutils_error_prefix);
    }
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    bindtextdomain(
        b"gnulib\0".as_ptr() as *const ::core::ffi::c_char,
        GNULIB_LOCALEDIR.as_ptr(),
    );
    textdomain(PACKAGE.as_ptr());
    c_stack_action(None);
    xstdopen();
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv,
            &raw const shortopts as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if 0 as ::core::ffi::c_int > c {
            break;
        }
        match c {
            98 | 99 => {
                opt_print_bytes = r#true != 0;
            }
            105 => {
                specify_ignore_initial(
                    0 as ::core::ffi::c_int,
                    &raw mut optarg,
                    ':' as ::core::ffi::c_char,
                );
                let c2rust_fresh5 = optarg;
                optarg = optarg.offset(1);
                if *c2rust_fresh5 as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                    specify_ignore_initial(
                        1 as ::core::ffi::c_int,
                        &raw mut optarg,
                        0 as ::core::ffi::c_char,
                    );
                } else if ignore_initial[1usize] < ignore_initial[0usize]
                    || ignore_initial[0usize] < 0 as intmax_t
                {
                    ignore_initial[1usize] = ignore_initial[0usize];
                }
            }
            108 => {
                specify_comparison_type(comparison_type::type_all_diffs);
            }
            110 => {
                let mut n: intmax_t = 0;
                let mut e: strtol_error = xstrtoimax(
                    optarg,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                    0 as ::core::ffi::c_int,
                    &raw mut n,
                    &raw const valid_suffixes as *const ::core::ffi::c_char,
                );
                if e.0
                    & !(strtol_error::LONGINT_OVERFLOW.0 as ::core::ffi::c_int)
                        as ::core::ffi::c_uint
                    != strtol_error::LONGINT_OK.0
                    || n < 0 as intmax_t
                {
                    try_help(
                        b"invalid --bytes value %s\0".as_ptr() as *const ::core::ffi::c_char,
                        quote(optarg),
                    );
                }
                bytes = if bytes < n { bytes } else { n };
            }
            115 => {
                specify_comparison_type(comparison_type::type_status);
            }
            118 => {
                version_etc(
                    stdout,
                    &raw const PROGRAM_NAME as *const ::core::ffi::c_char,
                    PACKAGE_NAME.as_ptr(),
                    Version,
                    proper_name_lite(
                        b"Torbjorn Granlund\0".as_ptr() as *const ::core::ffi::c_char,
                        b"Torbj\xC3\xB6rn Granlund\0".as_ptr() as *const ::core::ffi::c_char,
                    ),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"David MacKenzie\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    nullptr,
                );
                check_stdout();
                return EXIT_SUCCESS;
            }
            128 => {
                usage();
                check_stdout();
                return EXIT_SUCCESS;
            }
            _ => {
                try_help(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
    if optind == argc {
        try_help(
            b"missing operand after %s\0".as_ptr() as *const ::core::ffi::c_char,
            quote(*argv.offset((argc - 1 as ::core::ffi::c_int) as isize)),
        );
    }
    let c2rust_fresh6 = optind;
    optind += 1;
    file[0usize] = *argv.offset(c2rust_fresh6 as isize);
    file[1usize] = if optind < argc {
        let c2rust_fresh7 = optind;
        optind += 1;
        *argv.offset(c2rust_fresh7 as isize) as *const ::core::ffi::c_char
    } else {
        b"-\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f < 2 as ::core::ffi::c_int && optind < argc {
        let c2rust_fresh8 = optind;
        optind += 1;
        let mut arg: *mut ::core::ffi::c_char = *argv.offset(c2rust_fresh8 as isize);
        specify_ignore_initial(f, &raw mut arg, 0 as ::core::ffi::c_char);
        f += 1;
    }
    if optind < argc {
        try_help(
            b"extra operand %s\0".as_ptr() as *const ::core::ffi::c_char,
            quote(*argv.offset(optind as isize)),
        );
    }
    let mut f_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_0 < 2 as ::core::ffi::c_int {
        if f_0 != 0
            && 0 as intmax_t <= ignore_initial[0usize]
            && ignore_initial[0usize] == ignore_initial[1usize]
            && strcmp(file[0usize], file[1usize]) == 0 as ::core::ffi::c_int
        {
            return EXIT_SUCCESS;
        }
        if strcmp(
            file[f_0 as usize],
            b"-\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            file_desc[f_0 as usize] = STDIN_FILENO;
            if O_BINARY != 0 && isatty(STDIN_FILENO) == 0 {
                set_binary_mode(STDIN_FILENO, O_BINARY);
            }
        } else {
            file_desc[f_0 as usize] = open(file[f_0 as usize], O_RDONLY | O_BINARY | O_CLOEXEC);
            if file_desc[f_0 as usize] < 0 as ::core::ffi::c_int {
                if comparison_type_0.0 != comparison_type::type_status.0 {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            *__errno_location(),
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            squote(0 as ::core::ffi::c_int, file[f_0 as usize]),
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
                                *__errno_location(),
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                squote(0 as ::core::ffi::c_int, file[f_0 as usize]),
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
                exit(C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
            }
        }
        if fstat(
            file_desc[f_0 as usize],
            (&raw mut stat_buf as *mut stat).offset(f_0 as isize),
        ) < 0 as ::core::ffi::c_int
        {
            stat_buf[f_0 as usize].st_size = -2 as __off_t;
        } else {
            stat_buf[f_0 as usize].st_size =
                stat_size((&raw mut stat_buf as *mut stat).offset(f_0 as isize)) as __off_t;
        }
        f_0 += 1;
    }
    if -1 as __off_t <= stat_buf[0usize].st_size
        && -1 as __off_t <= stat_buf[1usize].st_size
        && same_file(
            (&raw mut stat_buf as *mut stat).offset(0isize),
            (&raw mut stat_buf as *mut stat).offset(1isize),
        ) as ::core::ffi::c_int
            != 0
        && file_position(0 as ::core::ffi::c_int) == file_position(1 as ::core::ffi::c_int)
    {
        return EXIT_SUCCESS;
    }
    if comparison_type_0.0 != comparison_type::type_status.0 {
        let mut outstat: stat = stat {
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
        let mut nullstat: stat = stat {
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
        if fstat(STDOUT_FILENO, &raw mut outstat) == 0 as ::core::ffi::c_int
            && outstat.st_mode & __S_IFMT as __mode_t == 0o20000 as __mode_t
            && stat(NULL_DEVICE.as_ptr(), &raw mut nullstat) == 0 as ::core::ffi::c_int
            && same_file(&raw mut outstat, &raw mut nullstat) as ::core::ffi::c_int != 0
        {
            comparison_type_0 = comparison_type::type_no_stdout;
        }
    }
    if comparison_type::type_no_stdout.0 <= comparison_type_0.0
        && 0 as __off_t <= stat_buf[0usize].st_size
        && stat_buf[0usize].st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        && 0 as __off_t <= stat_buf[1usize].st_size
        && stat_buf[1usize].st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
    {
        let mut pos0: off_t = file_position(0 as ::core::ffi::c_int);
        if 0 as off_t <= pos0 {
            let mut pos1: off_t = file_position(1 as ::core::ffi::c_int);
            if 0 as off_t <= pos1 {
                let mut s0: off_t = stat_buf[0usize].st_size - pos0;
                let mut s1: off_t = stat_buf[1usize].st_size - pos1;
                if s0 < 0 as off_t {
                    s0 = 0 as off_t;
                }
                if s1 < 0 as off_t {
                    s1 = 0 as off_t;
                }
                if s0 != s1
                    && if s0 < s1 {
                        s0 as intmax_t
                    } else {
                        s1 as intmax_t
                    } < bytes
                {
                    exit(EXIT_FAILURE);
                }
            }
        }
    }
    let mut blksize: [idx_t; 2] = [0; 2];
    let mut f_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_1 < 2 as ::core::ffi::c_int {
        if if (0 as __blksize_t) < stat_buf[0usize].st_blksize
            && stat_buf[0usize].st_blksize as size_t
                <= (-1 as ::core::ffi::c_int as size_t)
                    .wrapping_div(8 as size_t)
                    .wrapping_add(1 as size_t)
        {
            stat_buf[0usize].st_blksize
        } else {
            DEV_BSIZE as __blksize_t
        } < 0 as __blksize_t
            || {
                let (c2rust_result, c2rust_overflowed) = ((if (0 as __blksize_t)
                    < stat_buf[0usize].st_blksize
                    && stat_buf[0usize].st_blksize as size_t
                        <= (-1 as ::core::ffi::c_int as size_t)
                            .wrapping_div(8 as size_t)
                            .wrapping_add(1 as size_t)
                {
                    stat_buf[0usize].st_blksize
                } else {
                    512 as __blksize_t
                }) as i128)
                    .overflowing_add(0 as ::core::ffi::c_int as i128);
                let c2rust_result_narrow = c2rust_result as idx_t;
                *(&raw mut blksize as *mut idx_t).offset(f_1 as isize) = c2rust_result_narrow;
                c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result
            }
        {
            blksize[f_1 as usize] = 0 as idx_t;
        }
        f_1 += 1;
    }
    buf_size = buffer_lcm(
        blksize[0usize],
        blksize[1usize],
        (IDX_MAX as usize).wrapping_sub(::core::mem::size_of::<word>()) as idx_t,
    );
    let mut words_per_buffer: idx_t = (buf_size as usize)
        .wrapping_add(2usize.wrapping_mul(::core::mem::size_of::<word>()))
        .wrapping_sub(1usize)
        .wrapping_div(::core::mem::size_of::<word>())
        as idx_t;
    buffer[0usize] = xinmalloc(
        words_per_buffer,
        2usize.wrapping_mul(::core::mem::size_of::<word>()) as idx_t,
    ) as *mut word;
    buffer[1usize] = buffer[0usize].offset(words_per_buffer as isize);
    let mut exit_status: ::core::ffi::c_int = cmp();
    let mut f_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_2 < 2 as ::core::ffi::c_int {
        if close(file_desc[f_2 as usize]) != 0 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    squote(0 as ::core::ffi::c_int, file[f_2 as usize]),
                );
                if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        squote(0 as ::core::ffi::c_int, file[f_2 as usize]),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        f_2 += 1;
    }
    if exit_status != EXIT_SUCCESS && comparison_type_0.0 < comparison_type::type_no_stdout.0 {
        check_stdout();
    }
    exit(exit_status);
}
unsafe extern "C" fn cmp() -> ::core::ffi::c_int {
    let mut buffer0: *mut word = buffer[0usize];
    let mut buffer1: *mut word = buffer[1usize];
    let mut buf0: *mut ::core::ffi::c_char = buffer0 as *mut ::core::ffi::c_char;
    let mut buf1: *mut ::core::ffi::c_char = buffer1 as *mut ::core::ffi::c_char;
    let mut offset_width: ::core::ffi::c_int = 0;
    if comparison_type_0.0 == comparison_type::type_all_diffs.0 {
        let mut byte_number_max: intmax_t = bytes;
        let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while f < 2 as ::core::ffi::c_int {
            if 0 as __off_t <= stat_buf[f as usize].st_size
                && stat_buf[f as usize].st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
            {
                let mut pos: off_t = file_position(f);
                if 0 as off_t <= pos {
                    byte_number_max = if byte_number_max
                        < if 0 as __off_t > stat_buf[f as usize].st_size - pos {
                            0 as intmax_t
                        } else {
                            stat_buf[f as usize].st_size as intmax_t - pos as intmax_t
                        } {
                        byte_number_max
                    } else if 0 as __off_t > stat_buf[f as usize].st_size - pos {
                        0 as intmax_t
                    } else {
                        stat_buf[f as usize].st_size as intmax_t - pos as intmax_t
                    };
                }
            }
            f += 1;
        }
        offset_width = 1 as ::core::ffi::c_int;
        loop {
            byte_number_max /= 10 as intmax_t;
            if byte_number_max == 0 as intmax_t {
                break;
            }
            offset_width += 1;
        }
    } else {
        offset_width = comparison_type_0.0.wrapping_neg() as ::core::ffi::c_int;
    }
    let mut eof: [bool; 2] = [r#false != 0, r#false != 0];
    let mut f_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_0 < 2 as ::core::ffi::c_int {
        let mut ig: intmax_t = ignore_initial[f_0 as usize];
        if ig != 0 as intmax_t {
            if 0 as off_t > file_position(f_0) {
                if !(0 as intmax_t <= ig
                    && ig
                        < (if (0 as ::core::ffi::c_int as off_t) < -1 as ::core::ffi::c_int as off_t
                        {
                            -1 as ::core::ffi::c_int as off_t
                        } else {
                            (((1 as ::core::ffi::c_int as off_t)
                                << ::core::mem::size_of::<off_t>()
                                    .wrapping_mul(CHAR_BIT as usize)
                                    .wrapping_sub(2usize))
                                - 1 as off_t)
                                * 2 as off_t
                                + 1 as off_t
                        }) as intmax_t)
                    && -1 as __off_t <= stat_buf[f_0 as usize].st_size
                    && stat_buf[f_0 as usize].st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                {
                    eof[f_0 as usize] = r#true != 0;
                } else if ig < 0 as intmax_t {
                    if 0 != 0 {
                        error(
                            C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                            75 as ::core::ffi::c_int,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            squote(0 as ::core::ffi::c_int, file[f_0 as usize]),
                        );
                        if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int =
                                C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                75 as ::core::ffi::c_int,
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                squote(0 as ::core::ffi::c_int, file[f_0 as usize]),
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                } else {
                    loop {
                        let mut bytes_to_read: idx_t = if ig < buf_size as intmax_t {
                            ig as idx_t
                        } else {
                            buf_size
                        };
                        let mut r: ptrdiff_t =
                            block_read(file_desc[f_0 as usize], buf0, bytes_to_read);
                        if r != bytes_to_read {
                            if r < 0 as ptrdiff_t {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        *__errno_location(),
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        squote(0 as ::core::ffi::c_int, file[f_0 as usize]),
                                    );
                                    if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            *__errno_location(),
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            squote(0 as ::core::ffi::c_int, file[f_0 as usize]),
                                        );
                                        if __errstatus != 0 as ::core::ffi::c_int {
                                            unreachable!();
                                        } else {
                                        };
                                    });
                                };
                            }
                            break;
                        } else {
                            ig = (ig as ::core::ffi::c_long - r as ::core::ffi::c_long) as intmax_t;
                            if (0 as intmax_t) >= ig {
                                break;
                            }
                        }
                    }
                }
            }
        }
        f_0 += 1;
    }
    let mut at_line_start: bool = r#true != 0;
    let mut line_number: intmax_t = 1 as intmax_t;
    let mut byte_number: intmax_t = 1 as intmax_t;
    let mut remaining: intmax_t = bytes;
    loop {
        let mut bytes_to_read_0: idx_t = if buf_size < remaining as idx_t {
            buf_size
        } else {
            remaining as idx_t
        };
        remaining =
            (remaining as ::core::ffi::c_long - bytes_to_read_0 as ::core::ffi::c_long) as intmax_t;
        let mut read0: ptrdiff_t = if eof[0usize] as ::core::ffi::c_int != 0 {
            0 as ptrdiff_t
        } else {
            block_read(file_desc[0usize], buf0, bytes_to_read_0)
        };
        if read0 < 0 as ptrdiff_t {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    squote(0 as ::core::ffi::c_int, file[0usize]),
                );
                if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        squote(0 as ::core::ffi::c_int, file[0usize]),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        let mut read1: ptrdiff_t = if eof[1usize] as ::core::ffi::c_int != 0 {
            0 as ptrdiff_t
        } else {
            block_read(file_desc[1usize], buf1, bytes_to_read_0)
        };
        if read1 < 0 as ptrdiff_t {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    squote(0 as ::core::ffi::c_int, file[1usize]),
                );
                if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        squote(0 as ::core::ffi::c_int, file[1usize]),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        let mut smaller: idx_t = if read0 < read1 { read0 } else { read1 };
        let mut first_diff: idx_t = 0;
        if memcmp(
            buf0 as *const ::core::ffi::c_void,
            buf1 as *const ::core::ffi::c_void,
            smaller as size_t,
        ) == 0 as ::core::ffi::c_int
        {
            first_diff = smaller;
        } else {
            if read0 >= read1 {
                *buf1.offset(read0 as isize) = 0x55 as ::core::ffi::c_char;
            }
            if read1 >= read0 {
                *buf0.offset(read1 as isize) = 0x79 as ::core::ffi::c_char;
            }
            *buf0.offset(read0 as isize) =
                !(*buf1.offset(read0 as isize) as ::core::ffi::c_int) as ::core::ffi::c_char;
            *buf1.offset(read1 as isize) =
                !(*buf0.offset(read1 as isize) as ::core::ffi::c_int) as ::core::ffi::c_char;
            memset(
                buf0.offset(read0 as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<word>()
                    .wrapping_sub((read0 as size_t).wrapping_rem(::core::mem::size_of::<word>()))
                    .wrapping_sub(1 as size_t),
            );
            memset(
                buf1.offset(read1 as isize)
                    .offset(1 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<word>()
                    .wrapping_sub((read1 as size_t).wrapping_rem(::core::mem::size_of::<word>()))
                    .wrapping_sub(1 as size_t),
            );
            first_diff = block_compare(buffer0, buffer1);
        }
        byte_number =
            (byte_number as ::core::ffi::c_long + first_diff as ::core::ffi::c_long) as intmax_t;
        if offset_width == -(comparison_type::type_first_diff.0 as ::core::ffi::c_int)
            && first_diff != 0 as idx_t
        {
            line_number = (line_number as ::core::ffi::c_long
                + count_newlines(buf0, first_diff) as ::core::ffi::c_long)
                as intmax_t;
            at_line_start = *buf0.offset((first_diff - 1 as idx_t) as isize) as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int;
        }
        let mut differing: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if first_diff < smaller {
            's_397: {
                match offset_width {
                    0 => {
                        if !opt_print_bytes {
                            static mut char_message: [::core::ffi::c_char; 34] = unsafe {
                                ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
                                    *b"%s %s differ: char %ld, line %ld\n\0",
                                )
                            };
                            static mut byte_msgid: [::core::ffi::c_char; 34] = unsafe {
                                ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
                                    *b"%s %s differ: byte %ld, line %ld\n\0",
                                )
                            };
                            let mut byte_message: *const ::core::ffi::c_char = dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                &raw const byte_msgid as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            );
                            let mut use_byte_message: bool = byte_message
                                != &raw const byte_msgid as *const ::core::ffi::c_char
                                || hard_locale_LC_MESSAGES() as ::core::ffi::c_int != 0;
                            printf(
                                if use_byte_message as ::core::ffi::c_int != 0 {
                                    byte_message
                                } else {
                                    &raw const char_message as *const ::core::ffi::c_char
                                },
                                file[0usize],
                                file[1usize],
                                byte_number,
                                line_number,
                            );
                        } else {
                            let mut c0: ::core::ffi::c_uchar =
                                *buf0.offset(first_diff as isize) as ::core::ffi::c_uchar;
                            let mut c1: ::core::ffi::c_uchar =
                                *buf1.offset(first_diff as isize) as ::core::ffi::c_uchar;
                            let mut s0: [::core::ffi::c_char; 5] = [0; 5];
                            let mut s1: [::core::ffi::c_char; 5] = [0; 5];
                            sprintc(&raw mut s0 as *mut ::core::ffi::c_char, c0);
                            sprintc(&raw mut s1 as *mut ::core::ffi::c_char, c1);
                            printf(
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"%s %s differ: byte %ld, line %ld is %3o %s %3o %s\n\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    LC_MESSAGES,
                                ),
                                file[0usize],
                                file[1usize],
                                byte_number,
                                line_number,
                                c0 as ::core::ffi::c_int,
                                &raw mut s0 as *mut ::core::ffi::c_char,
                                c1 as ::core::ffi::c_int,
                                &raw mut s1 as *mut ::core::ffi::c_char,
                            );
                        }
                    }
                    -3 => {}
                    -2 => {
                        differing = 1 as ::core::ffi::c_int;
                        break 's_397;
                    }
                    _ => {
                        if comparison_type_0.0 == comparison_type::type_all_diffs.0 {
                        } else {
                            unreachable!();
                        };
                        loop {
                            let mut c0_0: ::core::ffi::c_uchar =
                                *buf0.offset(first_diff as isize) as ::core::ffi::c_uchar;
                            let mut c1_0: ::core::ffi::c_uchar =
                                *buf1.offset(first_diff as isize) as ::core::ffi::c_uchar;
                            if c0_0 as ::core::ffi::c_int != c1_0 as ::core::ffi::c_int {
                                if !opt_print_bytes {
                                    printf(
                                        b"%*ld %3o %3o\n\0".as_ptr() as *const ::core::ffi::c_char,
                                        offset_width,
                                        byte_number,
                                        c0_0 as ::core::ffi::c_int,
                                        c1_0 as ::core::ffi::c_int,
                                    );
                                } else {
                                    let mut s0_0: [::core::ffi::c_char; 5] = [0; 5];
                                    let mut s1_0: [::core::ffi::c_char; 5] = [0; 5];
                                    sprintc(&raw mut s0_0 as *mut ::core::ffi::c_char, c0_0);
                                    sprintc(&raw mut s1_0 as *mut ::core::ffi::c_char, c1_0);
                                    printf(
                                        b"%*ld %3o %-4s %3o %s\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        offset_width,
                                        byte_number,
                                        c0_0 as ::core::ffi::c_int,
                                        &raw mut s0_0 as *mut ::core::ffi::c_char,
                                        c1_0 as ::core::ffi::c_int,
                                        &raw mut s1_0 as *mut ::core::ffi::c_char,
                                    );
                                }
                            }
                            byte_number += 1;
                            first_diff += 1;
                            if first_diff >= smaller {
                                break;
                            }
                        }
                        differing = -1 as ::core::ffi::c_int;
                        break 's_397;
                    }
                }
                return EXIT_FAILURE;
            }
        }
        if read0 != read1 {
            if differing <= 0 as ::core::ffi::c_int
                && offset_width != -(comparison_type::type_status.0 as ::core::ffi::c_int)
            {
                fprintf(
                    stderr,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        if byte_number == 1 as intmax_t {
                            b"cmp: EOF on %s which is empty\n\0".as_ptr()
                                as *const ::core::ffi::c_char
                        } else if offset_width
                            != -(comparison_type::type_first_diff.0 as ::core::ffi::c_int)
                        {
                            b"cmp: EOF on %s after byte %ld\n\0".as_ptr()
                                as *const ::core::ffi::c_char
                        } else if at_line_start as ::core::ffi::c_int != 0 {
                            b"cmp: EOF on %s after byte %ld, line %ld\n\0".as_ptr()
                                as *const ::core::ffi::c_char
                        } else {
                            b"cmp: EOF on %s after byte %ld, in line %ld\n\0".as_ptr()
                                as *const ::core::ffi::c_char
                        },
                        LC_MESSAGES,
                    ),
                    quote(file[(read1 < read0) as ::core::ffi::c_int as usize]),
                    byte_number - 1 as intmax_t,
                    line_number - at_line_start as intmax_t,
                );
            }
            return EXIT_FAILURE;
        }
        if (0 as ::core::ffi::c_int) < differing || read0 != buf_size {
            return if differing == 0 as ::core::ffi::c_int {
                EXIT_SUCCESS
            } else {
                EXIT_FAILURE
            };
        }
    }
}
unsafe extern "C" fn block_compare(mut p0: *const word, mut p1: *const word) -> idx_t {
    let mut l0: *const word = ::core::ptr::null::<word>();
    let mut l1: *const word = ::core::ptr::null::<word>();
    let mut c0: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut c1: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    l0 = p0;
    l1 = p1;
    while *l0 == *l1 {
        l0 = l0.offset(1);
        l1 = l1.offset(1);
    }
    c0 = l0 as *const ::core::ffi::c_char;
    c1 = l1 as *const ::core::ffi::c_char;
    while *c0 as ::core::ffi::c_int == *c1 as ::core::ffi::c_int {
        c0 = c0.offset(1);
        c1 = c1.offset(1);
    }
    return c0.offset_from(p0 as *const ::core::ffi::c_char);
}
unsafe extern "C" fn count_newlines(
    mut buf: *mut ::core::ffi::c_char,
    mut bufsize: idx_t,
) -> idx_t {
    let mut count: idx_t = 0 as idx_t;
    let mut lim: *mut ::core::ffi::c_char = buf.offset(bufsize as isize);
    let mut ch: ::core::ffi::c_char = *lim;
    *lim = '\n' as ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = buf;
    loop {
        p = rawmemchr(p as *const ::core::ffi::c_void, '\n' as ::core::ffi::c_int)
            as *mut ::core::ffi::c_char;
        if p == lim {
            break;
        }
        count += 1;
        p = p.offset(1);
    }
    *lim = ch;
    return count;
}
unsafe extern "C" fn sprintc(mut buf: *mut ::core::ffi::c_char, mut c: ::core::ffi::c_uchar) {
    if !c_isprint(c as ::core::ffi::c_int) {
        if c as ::core::ffi::c_int >= 128 as ::core::ffi::c_int {
            let c2rust_fresh0 = buf;
            buf = buf.offset(1);
            *c2rust_fresh0 = 'M' as ::core::ffi::c_char;
            let c2rust_fresh1 = buf;
            buf = buf.offset(1);
            *c2rust_fresh1 = '-' as ::core::ffi::c_char;
            c = (c as ::core::ffi::c_int - 128 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        }
        if (c as ::core::ffi::c_int) < 32 as ::core::ffi::c_int {
            let c2rust_fresh2 = buf;
            buf = buf.offset(1);
            *c2rust_fresh2 = '^' as ::core::ffi::c_char;
            c = (c as ::core::ffi::c_int + 64 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        } else if c as ::core::ffi::c_int == 127 as ::core::ffi::c_int {
            let c2rust_fresh3 = buf;
            buf = buf.offset(1);
            *c2rust_fresh3 = '^' as ::core::ffi::c_char;
            c = '?' as ::core::ffi::c_uchar;
        }
    }
    let c2rust_fresh4 = buf;
    buf = buf.offset(1);
    *c2rust_fresh4 = c as ::core::ffi::c_char;
    *buf = 0 as ::core::ffi::c_char;
}
unsafe extern "C" fn file_position(mut f: ::core::ffi::c_int) -> off_t {
    static mut positioned: [bool; 2] = [false; 2];
    static mut position: [off_t; 2] = [0; 2];
    if !positioned[f as usize] {
        positioned[f as usize] = r#true != 0;
        let mut pos: off_t = ignore_initial[f as usize] as off_t;
        position[f as usize] = (if 0 as off_t <= pos
            && pos
                <= if (0 as ::core::ffi::c_int as off_t) < -1 as ::core::ffi::c_int as off_t {
                    -1 as ::core::ffi::c_int as off_t
                } else {
                    (((1 as ::core::ffi::c_int as off_t)
                        << ::core::mem::size_of::<off_t>()
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2usize))
                        - 1 as off_t)
                        * 2 as off_t
                        + 1 as off_t
                } {
            lseek(file_desc[f as usize], pos, SEEK_CUR)
        } else {
            -1 as __off_t
        }) as off_t;
    }
    return position[f as usize];
}
pub const nullptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GNULIB_LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"diffutils\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"GNU diffutils\0") };
