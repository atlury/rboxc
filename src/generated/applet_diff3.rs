// Generated from pinned GNU Diffutils 3.12 by scripts/translate-diffutils.py.
// Source SHA-256: b893b782f84193ab4eb6756b350f3c89ef4b89e3b54df761505721e75cba52b9
/* GNU diff3 - compare three files line by line

   Copyright (C) 1988-1989, 1992-1996, 1998, 2001-2002, 2004, 2006, 2009-2013,
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
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
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
    #[link_name = "rboxc_diffutils_Version"]
    static mut Version: *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_c_stack_action"]
    fn c_stack_action(
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_block_read"]
    fn block_read(_: ::core::ffi::c_int, _: *mut ::core::ffi::c_char, _: idx_t) -> ptrdiff_t;
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputs_unlocked(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite_unlocked(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __uflow(_: *mut FILE) -> ::core::ffi::c_int;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_exit_failure"]
    static mut exit_failure: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    #[link_name = "rboxc_diffutils_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_ximalloc"]
    fn ximalloc(s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xinmalloc"]
    fn xinmalloc(n: idx_t, s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xicalloc"]
    fn xicalloc(n: idx_t, s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xpalloc"]
    fn xpalloc(
        pa: *mut ::core::ffi::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xfreopen"]
    fn xfreopen(
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
        fp: *mut FILE,
    );
    #[link_name = "rboxc_diffutils_xstdopen"]
    fn xstdopen();
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
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type idx_t = ptrdiff_t;
pub type lin = ptrdiff_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const EXIT_TROUBLE: Self = Self(2);
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const FILE0: Self = Self(0);
    pub const FILE1: Self = Self(1);
    pub const FILE2: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const FILEC: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const FO: Self = Self(0);
    pub const FC: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const RANGE_START: Self = Self(0);
    pub const RANGE_END: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct diff_type(pub ::core::ffi::c_uint);
impl diff_type {
    pub const DIFF_ERROR: Self = Self(0);
    pub const DIFF_ADD: Self = Self(1);
    pub const DIFF_CHANGE: Self = Self(2);
    pub const DIFF_DELETE: Self = Self(3);
    pub const DIFF_ALL: Self = Self(4);
    pub const DIFF_1ST: Self = Self(5);
    pub const DIFF_2ND: Self = Self(6);
    pub const DIFF_3RD: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct diff_block {
    pub ranges: [[lin; 2]; 2],
    pub lines: [*mut *mut ::core::ffi::c_char; 2],
    pub lengths: [*mut idx_t; 2],
    pub next: *mut diff_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct diff3_block {
    pub correspond: diff_type,
    pub ranges: [[lin; 2]; 3],
    pub lines: [*mut *mut ::core::ffi::c_char; 3],
    pub lengths: [*mut idx_t; 3],
    pub next: *mut diff3_block,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const DIFF_PROGRAM_OPTION: Self = Self(128);
    pub const HELP_OPTION: Self = Self(129);
    pub const STRIP_TRAILING_CR_OPTION: Self = Self(130);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const OPTION_3: Self = Self(0);
    pub const OPTION_A: Self = Self(1);
    pub const OPTION_E: Self = Self(2);
    pub const OPTION_X: Self = Self(3);
    pub const OPTION_e: Self = Self(4);
    pub const OPTION_x: Self = Self(5);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const DEV_BSIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const DEFAULT_DIFF_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"diff\0") };
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[inline]
unsafe extern "C" fn c_isdigit(mut c: ::core::ffi::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return r#true != 0,
        _ => return r#false != 0,
    };
}
pub const _IO_EOF_SEEN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> ::core::ffi::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __uflow(__fp)
    } else {
        let c2rust_fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(c2rust_fresh0 as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn fputc_unlocked(
    mut __c: ::core::ffi::c_int,
    mut __stream: *mut FILE,
) -> ::core::ffi::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(__stream, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh1;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: ::core::ffi::c_int,
    mut __stream: *mut FILE,
) -> ::core::ffi::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(__stream, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh2 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh2;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(stdout, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh3 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh3;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_EOF_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
static mut PROGRAM_NAME: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"diff3\0") };
static mut text: bool = false;
static mut strip_trailing_cr: bool = false;
static mut edscript: bool = false;
static mut flagging: bool = false;
static mut initial_tab: bool = false;
static mut simple_only: bool = false;
static mut overlap_only: bool = false;
static mut show_2nd: bool = false;
static mut finalwrite: bool = false;
static mut merge: bool = false;
static mut diff_program: *const ::core::ffi::c_char = DEFAULT_DIFF_PROGRAM.as_ptr();
static mut shortopts: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"aeimvx3AEL:TX\0") };
static mut longopts: [option; 14] = [
    option {
        name: b"diff-program\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_4::DIFF_PROGRAM_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"easy-only\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: '3' as ::core::ffi::c_int,
    },
    option {
        name: b"ed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'e' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_4::HELP_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"initial-tab\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'T' as ::core::ffi::c_int,
    },
    option {
        name: b"label\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'L' as ::core::ffi::c_int,
    },
    option {
        name: b"merge\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'm' as ::core::ffi::c_int,
    },
    option {
        name: b"overlap-only\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'x' as ::core::ffi::c_int,
    },
    option {
        name: b"show-all\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'A' as ::core::ffi::c_int,
    },
    option {
        name: b"show-overlap\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'E' as ::core::ffi::c_int,
    },
    option {
        name: b"strip-trailing-cr\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_4::STRIP_TRAILING_CR_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"text\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'a' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut rboxc_diffutils_stderr: *mut libc::FILE;
}
unsafe extern "C" fn rboxc_diffutils_error_prefix() {
    libc::fprintf(rboxc_diffutils_stderr, b"%s: \0".as_ptr().cast(), program_name);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_diff3(
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
    let mut incompat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tag_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tag_strings: [*mut ::core::ffi::c_char; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 3];
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
        's_192: {
            match c {
                97 => {
                    text = r#true != 0;
                    break 's_192;
                }
                65 => {
                    show_2nd = r#true != 0;
                    flagging = r#true != 0;
                    incompat |= (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_5::OPTION_A.0 as ::core::ffi::c_int;
                    break 's_192;
                }
                120 => {
                    overlap_only = r#true != 0;
                    incompat |= (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_5::OPTION_x.0 as ::core::ffi::c_int;
                    break 's_192;
                }
                51 => {
                    simple_only = r#true != 0;
                    incompat |= (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_5::OPTION_3.0 as ::core::ffi::c_int;
                    break 's_192;
                }
                105 => {
                    finalwrite = r#true != 0;
                    break 's_192;
                }
                109 => {
                    merge = r#true != 0;
                    break 's_192;
                }
                88 => {
                    overlap_only = r#true != 0;
                    incompat |= (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_5::OPTION_X.0 as ::core::ffi::c_int;
                    break 's_192;
                }
                69 => {
                    flagging = r#true != 0;
                    incompat |= (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_5::OPTION_E.0 as ::core::ffi::c_int;
                    break 's_192;
                }
                101 => {
                    incompat |= (1 as ::core::ffi::c_int)
                        << C2Rust_Unnamed_5::OPTION_e.0 as ::core::ffi::c_int;
                    break 's_192;
                }
                84 => {
                    initial_tab = r#true != 0;
                    break 's_192;
                }
                130 => {
                    strip_trailing_cr = r#true != 0;
                    break 's_192;
                }
                118 => {
                    version_etc(
                        stdout,
                        &raw const PROGRAM_NAME as *const ::core::ffi::c_char,
                        PACKAGE_NAME.as_ptr(),
                        Version,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Randy Smith\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        nullptr,
                    );
                    check_stdout();
                    return EXIT_SUCCESS;
                }
                128 => {
                    diff_program = optarg;
                    break 's_192;
                }
                129 => {
                    usage();
                    check_stdout();
                    return EXIT_SUCCESS;
                }
                76 => {
                    if tag_count < 3 as ::core::ffi::c_int {
                        let c2rust_fresh26 = tag_count;
                        tag_count += 1;
                        tag_strings[c2rust_fresh26 as usize] = optarg;
                        break 's_192;
                    } else {
                        try_help(
                            b"too many file label options\0".as_ptr() as *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                        );
                    }
                }
                _ => {}
            }
            try_help(
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
    }
    edscript = (incompat != 0) as ::core::ffi::c_int & !merge as ::core::ffi::c_int != 0;
    show_2nd = show_2nd as ::core::ffi::c_int
        | (incompat == 0) as ::core::ffi::c_int & merge as ::core::ffi::c_int
        != 0;
    flagging = flagging as ::core::ffi::c_int
        | (incompat == 0) as ::core::ffi::c_int & merge as ::core::ffi::c_int
        != 0;
    if incompat & incompat - 1 as ::core::ffi::c_int != 0
        || finalwrite as ::core::ffi::c_int & merge as ::core::ffi::c_int != 0
        || tag_count != 0 && !flagging
    {
        try_help(
            b"incompatible options\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    if argc - optind != 3 as ::core::ffi::c_int {
        if argc - optind < 3 as ::core::ffi::c_int {
            try_help(
                b"missing operand after %s\0".as_ptr() as *const ::core::ffi::c_char,
                quote(*argv.offset((argc - 1 as ::core::ffi::c_int) as isize)),
            );
        } else {
            try_help(
                b"extra operand %s\0".as_ptr() as *const ::core::ffi::c_char,
                quote(*argv.offset((optind + 3 as ::core::ffi::c_int) as isize)),
            );
        }
    }
    let mut file: *mut *mut ::core::ffi::c_char = argv.offset(optind as isize);
    let mut i: ::core::ffi::c_int = tag_count;
    while i < 3 as ::core::ffi::c_int {
        tag_strings[i as usize] = *file.offset(i as isize);
        i += 1;
    }
    let mut common: ::core::ffi::c_int =
        2 as ::core::ffi::c_int - (edscript as ::core::ffi::c_int | merge as ::core::ffi::c_int);
    if strcmp(
        *file.offset(common as isize),
        b"-\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        common = 3 as ::core::ffi::c_int - common;
        if strcmp(
            *file.offset(0isize),
            b"-\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
            || strcmp(
                *file.offset(common as isize),
                b"-\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
        {
            fatal(b"'-' specified for more than one input file\0".as_ptr()
                as *const ::core::ffi::c_char);
        }
    }
    let mut mapping: [::core::ffi::c_int; 3] = [
        0 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int - common,
        common,
    ];
    let mut rev_mapping: [::core::ffi::c_int; 3] = [0; 3];
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 3 as ::core::ffi::c_int {
        rev_mapping[mapping[i_0 as usize] as usize] = i_0;
        i_0 += 1;
    }
    signal(SIGCHLD, SIG_DFL);
    let mut commonname: *mut ::core::ffi::c_char = *file
        .offset(rev_mapping[C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize] as isize);
    let mut thread1: *mut diff_block = process_diff(
        *file
            .offset(rev_mapping[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize] as isize),
        commonname,
    );
    let mut thread0: *mut diff_block = process_diff(
        *file
            .offset(rev_mapping[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize] as isize),
        commonname,
    );
    let mut diff3: *mut diff3_block = make_3way_diff(thread0, thread1);
    let mut conflicts_found: bool = false;
    if edscript {
        conflicts_found = output_diff3_edscript(
            stdout,
            diff3,
            &raw mut mapping as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            &raw mut rev_mapping as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            tag_strings[0usize],
            tag_strings[1usize],
            tag_strings[2usize],
        );
    } else if merge {
        xfreopen(
            *file.offset(
                rev_mapping[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize] as isize,
            ),
            b"re\0".as_ptr() as *const ::core::ffi::c_char,
            stdin,
        );
        conflicts_found = output_diff3_merge(
            stdin,
            stdout,
            diff3,
            &raw mut mapping as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            &raw mut rev_mapping as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            tag_strings[0usize],
            tag_strings[1usize],
            tag_strings[2usize],
        );
        if ferror_unlocked(stdin) != 0 {
            fatal(b"read failed\0".as_ptr() as *const ::core::ffi::c_char);
        }
    } else {
        output_diff3(
            stdout,
            diff3,
            &raw mut mapping as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
            &raw mut rev_mapping as *mut ::core::ffi::c_int as *const ::core::ffi::c_int,
        );
        conflicts_found = r#false != 0;
    }
    check_stdout();
    exit(conflicts_found as ::core::ffi::c_int);
}
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        fatal(b"write failed\0".as_ptr() as *const ::core::ffi::c_char);
    } else if fclose(stdout) != 0 as ::core::ffi::c_int {
        perror_with_exit(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"standard output\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
}
static mut option_help_msgid: [*const ::core::ffi::c_char; 20] = [
    b"-A, --show-all              output all changes, bracketing conflicts\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-e, --ed                    output ed script incorporating changes\n                                from OLDFILE to YOURFILE into MYFILE\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-E, --show-overlap          like -e, but bracket conflicts\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-3, --easy-only             like -e, but incorporate only nonoverlapping changes\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-x, --overlap-only          like -e, but incorporate only overlapping changes\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-X                          like -x, but bracket conflicts\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-i                          append 'w' and 'q' commands to ed scripts\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-m, --merge                 output actual merged file, according to\n                                -A if no other options are given\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-a, --text                  treat all files as text\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --strip-trailing-cr     strip trailing carriage return on input\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-T, --initial-tab           make tabs line up by prepending a tab\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --diff-program=PROGRAM  use PROGRAM to compare files\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-L, --label=LABEL           use LABEL instead of file name\n                                (can be repeated up to three times)\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"    --help                  display this help and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-v, --version               output version information and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... MYFILE OLDFILE YOURFILE\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        squote(0 as ::core::ffi::c_int, program_name),
    );
    printf(
        b"%s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Compare three files line by line.\0".as_ptr() as *const ::core::ffi::c_char,
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
        if **p != 0 {
            printf(
                b"  %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                dcgettext(::core::ptr::null::<::core::ffi::c_char>(), *p, LC_MESSAGES),
            );
        } else {
            putchar_unlocked('\n' as ::core::ffi::c_int);
        }
        p = p.offset(1);
    }
    fputs_unlocked(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\nThe default output format is a somewhat human-readable representation of\nthe changes.\n\nThe -e, -E, -x, -X (and corresponding long) options cause an ed script\nto be output instead of the default.\n\nFinally, the -m (--merge) option causes diff3 to do the merge internally\nand output the actual merged file.  For unusual input, this is more\nrobust than using ed.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    printf(
        b"\n%s\n%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"If a FILE is '-', read standard input.\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Exit status is 0 if successful, 1 if conflicts, 2 if trouble.\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    emit_bug_reporting_address();
}
unsafe extern "C" fn make_3way_diff(
    mut thread0: *mut diff_block,
    mut thread1: *mut diff_block,
) -> *mut diff3_block {
    let mut current: [*mut diff_block; 2] = [thread0, thread1];
    let mut result: *mut diff3_block = ::core::ptr::null_mut::<diff3_block>();
    let mut result_end: *mut *mut diff3_block = &raw mut result;
    static mut zero_diff3: diff3_block = diff3_block {
        correspond: diff_type::DIFF_ERROR,
        ranges: [[0; 2]; 3],
        lines: [::core::ptr::null_mut::<*mut ::core::ffi::c_char>(); 3],
        lengths: [::core::ptr::null_mut::<idx_t>(); 3],
        next: ::core::ptr::null_mut::<diff3_block>(),
    };
    let mut last_diff3: *const diff3_block = &raw const zero_diff3;
    while !current[0usize].is_null() || !current[1usize].is_null() {
        let mut using: [*mut diff_block; 2] = [
            ::core::ptr::null_mut::<diff_block>(),
            ::core::ptr::null_mut::<diff_block>(),
        ];
        let mut last_using: [*mut diff_block; 2] = [
            ::core::ptr::null_mut::<diff_block>(),
            ::core::ptr::null_mut::<diff_block>(),
        ];
        let mut base_water_thread: ::core::ffi::c_int = if current[0usize].is_null() {
            1 as ::core::ffi::c_int
        } else if current[1usize].is_null() {
            0 as ::core::ffi::c_int
        } else {
            ((*current[0usize]).ranges[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                > (*current[1usize]).ranges[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize])
                as ::core::ffi::c_int
        };
        let mut high_water_thread: ::core::ffi::c_int = base_water_thread;
        let mut high_water_diff: *mut diff_block = current[high_water_thread as usize];
        let mut high_water_mark: lin = (*high_water_diff).ranges
            [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
        last_using[high_water_thread as usize] = high_water_diff;
        using[high_water_thread as usize] = last_using[high_water_thread as usize];
        current[high_water_thread as usize] = (*high_water_diff).next;
        (*last_using[high_water_thread as usize]).next = ::core::ptr::null_mut::<diff_block>();
        let mut other_thread: ::core::ffi::c_int = high_water_thread ^ 0x1 as ::core::ffi::c_int;
        let mut other_diff: *mut diff_block = current[other_thread as usize];
        while !other_diff.is_null()
            && (*other_diff).ranges[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                <= high_water_mark + 1 as lin
        {
            if !using[other_thread as usize].is_null() {
                (*last_using[other_thread as usize]).next = other_diff;
            } else {
                using[other_thread as usize] = other_diff;
            }
            last_using[other_thread as usize] = other_diff;
            current[other_thread as usize] = (*current[other_thread as usize]).next;
            (*other_diff).next = ::core::ptr::null_mut::<diff_block>();
            if high_water_mark
                < (*other_diff).ranges[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
            {
                high_water_thread ^= 1 as ::core::ffi::c_int;
                high_water_mark = (*other_diff).ranges
                    [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
            }
            other_thread = high_water_thread ^ 0x1 as ::core::ffi::c_int;
            other_diff = current[other_thread as usize];
        }
        let mut tmpblock: *mut diff3_block = using_to_diff3_block(
            &raw mut using as *mut *mut diff_block,
            &raw mut last_using as *mut *mut diff_block,
            base_water_thread,
            high_water_thread,
            last_diff3,
        );
        if tmpblock.is_null() {
            fatal(
                b"internal error: screwup in format of diff blocks\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        *result_end = tmpblock;
        result_end = &raw mut (*tmpblock).next;
        last_diff3 = tmpblock;
    }
    return result;
}
unsafe extern "C" fn using_to_diff3_block(
    mut using: *mut *mut diff_block,
    mut last_using: *mut *mut diff_block,
    mut low_thread: ::core::ffi::c_int,
    mut high_thread: ::core::ffi::c_int,
    mut last_diff3: *const diff3_block,
) -> *mut diff3_block {
    let mut lowc: lin = (**using.offset(low_thread as isize)).ranges
        [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize];
    let mut highc: lin = (**last_using.offset(high_thread as isize)).ranges
        [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
    let mut low: [lin; 2] = [0; 2];
    let mut high: [lin; 2] = [0; 2];
    let mut d: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while d < 2 as ::core::ffi::c_int {
        if !(*using.offset(d as isize)).is_null() {
            low[d as usize] = lowc
                - (**using.offset(d as isize)).ranges
                    [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                + (**using.offset(d as isize)).ranges
                    [C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize];
            high[d as usize] = highc
                - (**last_using.offset(d as isize)).ranges
                    [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                + (**last_using.offset(d as isize)).ranges
                    [C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
        } else {
            low[d as usize] = lowc
                - (*last_diff3).ranges[C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                + (*last_diff3).ranges
                    [(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d) as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
            high[d as usize] = highc
                - (*last_diff3).ranges[C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                + (*last_diff3).ranges
                    [(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d) as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
        }
        d += 1;
    }
    let mut result: *mut diff3_block = create_diff3_block(
        low[0usize],
        high[0usize],
        low[1usize],
        high[1usize],
        lowc,
        highc,
    );
    let mut d_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while d_0 < 2 as ::core::ffi::c_int {
        let mut ptr: *mut diff_block = *using.offset(d_0 as isize);
        while !ptr.is_null() {
            let mut result_offset: lin = (*ptr).ranges
                [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                - lowc;
            if !copy_stringlist(
                (*ptr).lines[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    as *const *mut ::core::ffi::c_char,
                (*ptr).lengths[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    as *const idx_t,
                (*result).lines[C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                    .offset(result_offset as isize),
                (*result).lengths[C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                    .offset(result_offset as isize),
                (*ptr).ranges[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                    - (*ptr).ranges[C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                    + 1 as lin,
            ) {
                return ::core::ptr::null_mut::<diff3_block>();
            }
            ptr = (*ptr).next;
        }
        d_0 += 1;
    }
    let mut d_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while d_1 < 2 as ::core::ffi::c_int {
        let mut u: *mut diff_block = *using.offset(d_1 as isize);
        let mut lo: lin = low[d_1 as usize];
        let mut hi: lin = high[d_1 as usize];
        let mut i: lin = 0 as lin;
        while i + lo
            < if !u.is_null() {
                (*u).ranges[C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
            } else {
                hi + 1 as lin
            }
        {
            *(*result).lines[(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d_1) as usize]
                .offset(i as isize) = *(*result).lines
                [C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                .offset(i as isize);
            *(*result).lengths[(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d_1) as usize]
                .offset(i as isize) = *(*result).lengths
                [C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                .offset(i as isize);
            i += 1;
        }
        let mut ptr_0: *mut diff_block = u;
        while !ptr_0.is_null() {
            let mut result_offset_0: lin = (*ptr_0).ranges
                [C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                - lo;
            if !copy_stringlist(
                (*ptr_0).lines[C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                    as *const *mut ::core::ffi::c_char,
                (*ptr_0).lengths[C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                    as *const idx_t,
                (*result).lines[(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d_1) as usize]
                    .offset(result_offset_0 as isize),
                (*result).lengths[(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d_1) as usize]
                    .offset(result_offset_0 as isize),
                (*ptr_0).ranges[C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                    - (*ptr_0).ranges[C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                    + 1 as lin,
            ) {
                return ::core::ptr::null_mut::<diff3_block>();
            }
            let mut linec: lin = (*ptr_0).ranges
                [C2Rust_Unnamed_2::FC.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                + 1 as lin
                - lowc;
            let mut i_0: lin = (*ptr_0).ranges
                [C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                + 1 as lin
                - lo;
            while i_0
                < if !(*ptr_0).next.is_null() {
                    (*(*ptr_0).next).ranges[C2Rust_Unnamed_2::FO.0 as ::core::ffi::c_int as usize]
                        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                } else {
                    hi + 1 as lin
                } - lo
            {
                *(*result).lines
                    [(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d_1) as usize]
                    .offset(i_0 as isize) = *(*result).lines
                    [C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                    .offset(linec as isize);
                *(*result).lengths
                    [(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int + d_1) as usize]
                    .offset(i_0 as isize) = *(*result).lengths
                    [C2Rust_Unnamed_1::FILEC.0 as ::core::ffi::c_int as usize]
                    .offset(linec as isize);
                linec += 1;
                i_0 += 1;
            }
            ptr_0 = (*ptr_0).next;
        }
        d_1 += 1;
    }
    if (*using.offset(0isize)).is_null() {
        (*result).correspond = diff_type::DIFF_2ND;
    } else if (*using.offset(1isize)).is_null() {
        (*result).correspond = diff_type::DIFF_1ST;
    } else {
        let mut nl0: lin = (*result).ranges
            [C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
            - (*result).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
            + 1 as lin;
        let mut nl1: lin = (*result).ranges
            [C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
            - (*result).ranges[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
            + 1 as lin;
        if nl0 != nl1
            || !compare_line_list(
                (*result).lines[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
                    as *const *mut ::core::ffi::c_char,
                (*result).lengths[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
                    as *const idx_t,
                (*result).lines[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
                    as *const *mut ::core::ffi::c_char,
                (*result).lengths[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
                    as *const idx_t,
                nl0,
            )
        {
            (*result).correspond = diff_type::DIFF_ALL;
        } else {
            (*result).correspond = diff_type::DIFF_3RD;
        }
    }
    return result;
}
unsafe extern "C" fn copy_stringlist(
    mut fromptrs: *const *mut ::core::ffi::c_char,
    mut fromlengths: *const idx_t,
    mut toptrs: *mut *mut ::core::ffi::c_char,
    mut tolengths: *mut idx_t,
    mut copynum: lin,
) -> bool {
    let mut f: *const *mut ::core::ffi::c_char = fromptrs;
    let mut t: *mut *mut ::core::ffi::c_char = toptrs;
    let mut fl: *const idx_t = fromlengths;
    let mut tl: *mut idx_t = tolengths;
    loop {
        let c2rust_fresh19 = copynum;
        copynum -= 1;
        if c2rust_fresh19 == 0 {
            break;
        }
        if !(*t).is_null() {
            if *fl != *tl
                || memcmp(
                    *f as *const ::core::ffi::c_void,
                    *t as *const ::core::ffi::c_void,
                    *fl as size_t,
                ) != 0 as ::core::ffi::c_int
            {
                return r#false != 0;
            }
        } else {
            *t = *f;
            *tl = *fl;
        }
        t = t.offset(1);
        f = f.offset(1);
        tl = tl.offset(1);
        fl = fl.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn create_diff3_block(
    mut low0: lin,
    mut high0: lin,
    mut low1: lin,
    mut high1: lin,
    mut low2: lin,
    mut high2: lin,
) -> *mut diff3_block {
    let mut result: *mut diff3_block =
        xmalloc(::core::mem::size_of::<diff3_block>()) as *mut diff3_block;
    (*result).correspond = diff_type::DIFF_ERROR;
    (*result).next = ::core::ptr::null_mut::<diff3_block>();
    (*result).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize] = low0;
    (*result).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize] = high0;
    (*result).ranges[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize] = low1;
    (*result).ranges[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize] = high1;
    (*result).ranges[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize] = low2;
    (*result).ranges[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize] = high2;
    let mut numlines: lin = (*result).ranges
        [C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
        - (*result).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
            [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
        + 1 as lin;
    if numlines != 0 {
        (*result).lines[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize] = xicalloc(
            numlines,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as idx_t,
        )
            as *mut *mut ::core::ffi::c_char;
        (*result).lengths[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize] =
            xicalloc(numlines, ::core::mem::size_of::<idx_t>() as idx_t) as *mut idx_t;
    } else {
        (*result).lines[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*result).lengths[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<idx_t>();
    }
    numlines = (*result).ranges[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
        - (*result).ranges[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize]
            [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
        + 1 as lin;
    if numlines != 0 {
        (*result).lines[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize] = xicalloc(
            numlines,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as idx_t,
        )
            as *mut *mut ::core::ffi::c_char;
        (*result).lengths[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize] =
            xicalloc(numlines, ::core::mem::size_of::<idx_t>() as idx_t) as *mut idx_t;
    } else {
        (*result).lines[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*result).lengths[C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<idx_t>();
    }
    numlines = (*result).ranges[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize]
        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
        - (*result).ranges[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize]
            [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
        + 1 as lin;
    if numlines != 0 {
        (*result).lines[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize] = xicalloc(
            numlines,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>() as idx_t,
        )
            as *mut *mut ::core::ffi::c_char;
        (*result).lengths[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize] =
            xicalloc(numlines, ::core::mem::size_of::<idx_t>() as idx_t) as *mut idx_t;
    } else {
        (*result).lines[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*result).lengths[C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as usize] =
            ::core::ptr::null_mut::<idx_t>();
    }
    return result;
}
unsafe extern "C" fn compare_line_list(
    mut list1: *const *mut ::core::ffi::c_char,
    mut lengths1: *const idx_t,
    mut list2: *const *mut ::core::ffi::c_char,
    mut lengths2: *const idx_t,
    mut nl: lin,
) -> bool {
    let mut l1: *const *mut ::core::ffi::c_char = list1;
    let mut l2: *const *mut ::core::ffi::c_char = list2;
    let mut lgths1: *const idx_t = lengths1;
    let mut lgths2: *const idx_t = lengths2;
    loop {
        let c2rust_fresh14 = nl;
        nl -= 1;
        if c2rust_fresh14 == 0 {
            break;
        }
        if (*l1).is_null()
            || (*l2).is_null()
            || {
                let c2rust_fresh15 = lgths2;
                lgths2 = lgths2.offset(1);
                *lgths1 != *c2rust_fresh15
            }
            || {
                let c2rust_fresh16 = l1;
                l1 = l1.offset(1);
                let c2rust_fresh17 = l2;
                l2 = l2.offset(1);
                let c2rust_fresh18 = lgths1;
                lgths1 = lgths1.offset(1);
                memcmp(
                    *c2rust_fresh16 as *const ::core::ffi::c_void,
                    *c2rust_fresh17 as *const ::core::ffi::c_void,
                    *c2rust_fresh18 as size_t,
                ) != 0 as ::core::ffi::c_int
            }
        {
            return r#false != 0;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn process_diff(
    mut filea: *const ::core::ffi::c_char,
    mut fileb: *const ::core::ffi::c_char,
) -> *mut diff_block {
    let mut block_list: *mut diff_block = ::core::ptr::null_mut::<diff_block>();
    let mut block_list_end: *mut *mut diff_block = &raw mut block_list;
    let mut scan_diff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut diff_limit: *mut ::core::ffi::c_char = read_diff(filea, fileb, &raw mut scan_diff);
    while scan_diff < diff_limit {
        let mut bptr: *mut diff_block =
            xmalloc(::core::mem::size_of::<diff_block>()) as *mut diff_block;
        (*bptr).lines[1usize] = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*bptr).lines[0usize] = (*bptr).lines[1usize];
        (*bptr).lengths[1usize] = ::core::ptr::null_mut::<idx_t>();
        (*bptr).lengths[0usize] = (*bptr).lengths[1usize];
        let mut dt: diff_type = process_diff_control(&raw mut scan_diff, bptr);
        if dt.0 == diff_type::DIFF_ERROR.0
            || *scan_diff as ::core::ffi::c_int != '\n' as ::core::ffi::c_int
        {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s: diff failed: \0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                squote(0 as ::core::ffi::c_int, program_name),
            );
            loop {
                putc_unlocked(*scan_diff as ::core::ffi::c_int, stderr);
                let c2rust_fresh24 = scan_diff;
                scan_diff = scan_diff.offset(1);
                if *c2rust_fresh24 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
                    break;
                }
            }
            exit(C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
        }
        scan_diff = scan_diff.offset(1);
        match dt {
            diff_type::DIFF_ADD => {
                (*bptr).ranges[0usize][0usize] += 1;
            }
            diff_type::DIFF_DELETE => {
                (*bptr).ranges[1usize][0usize] += 1;
            }
            diff_type::DIFF_CHANGE => {}
            _ => {
                fatal(
                    b"internal error: invalid diff type in process_diff\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        if dt.0 != diff_type::DIFF_ADD.0 {
            let mut numlines: lin = (*bptr).ranges[0usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                - (*bptr).ranges[0usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                + 1 as lin;
            (*bptr).lines[0usize] = xinmalloc(
                numlines,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as idx_t,
            ) as *mut *mut ::core::ffi::c_char;
            (*bptr).lengths[0usize] =
                xinmalloc(numlines, ::core::mem::size_of::<idx_t>() as idx_t) as *mut idx_t;
            let mut i: lin = 0 as lin;
            while i < numlines {
                scan_diff = scan_diff_line(
                    scan_diff,
                    (*bptr).lines[0usize].offset(i as isize),
                    (*bptr).lengths[0usize].offset(i as isize),
                    diff_limit,
                    '<' as ::core::ffi::c_char,
                );
                i += 1;
            }
        }
        if dt.0 == diff_type::DIFF_CHANGE.0 {
            if strncmp(
                scan_diff,
                b"---\n\0".as_ptr() as *const ::core::ffi::c_char,
                4 as size_t,
            ) != 0
            {
                fatal(b"invalid diff format; invalid change separator\0".as_ptr()
                    as *const ::core::ffi::c_char);
            }
            scan_diff = scan_diff.offset(4 as ::core::ffi::c_int as isize);
        }
        if dt.0 != diff_type::DIFF_DELETE.0 {
            let mut numlines_0: lin = (*bptr).ranges[1usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                - (*bptr).ranges[1usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                + 1 as lin;
            (*bptr).lines[1usize] = xinmalloc(
                numlines_0,
                ::core::mem::size_of::<*mut ::core::ffi::c_char>() as idx_t,
            ) as *mut *mut ::core::ffi::c_char;
            (*bptr).lengths[1usize] =
                xinmalloc(numlines_0, ::core::mem::size_of::<idx_t>() as idx_t) as *mut idx_t;
            let mut i_0: lin = 0 as lin;
            while i_0 < numlines_0 {
                scan_diff = scan_diff_line(
                    scan_diff,
                    (*bptr).lines[1usize].offset(i_0 as isize),
                    (*bptr).lengths[1usize].offset(i_0 as isize),
                    diff_limit,
                    '>' as ::core::ffi::c_char,
                );
                i_0 += 1;
            }
        }
        *block_list_end = bptr;
        block_list_end = &raw mut (*bptr).next;
    }
    *block_list_end = ::core::ptr::null_mut::<diff_block>();
    return block_list;
}
unsafe extern "C" fn skipwhite(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    while *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *s as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        s = s.offset(1);
    }
    return s;
}
unsafe extern "C" fn readnum(
    mut s: *mut ::core::ffi::c_char,
    mut pnum: *mut lin,
) -> *mut ::core::ffi::c_char {
    let mut c: ::core::ffi::c_uchar = *s as ::core::ffi::c_uchar;
    let mut num: lin = 0 as lin;
    if !c_isdigit(c as ::core::ffi::c_int) {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    loop {
        num = (c as ::core::ffi::c_int - '0' as ::core::ffi::c_int) as lin + num * 10 as lin;
        s = s.offset(1);
        c = *s as ::core::ffi::c_uchar;
        if !c_isdigit(c as ::core::ffi::c_int) {
            break;
        }
    }
    *pnum = num;
    return s;
}
unsafe extern "C" fn process_diff_control(
    mut string: *mut *mut ::core::ffi::c_char,
    mut db: *mut diff_block,
) -> diff_type {
    let mut s: *mut ::core::ffi::c_char = *string;
    s = readnum(
        skipwhite(s),
        (&raw mut *(&raw mut (*db).ranges as *mut [lin; 2]).offset(0isize) as *mut lin)
            .offset(C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as isize),
    );
    if s.is_null() {
        return diff_type::DIFF_ERROR;
    }
    s = skipwhite(s);
    if *s as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
        s = readnum(
            s.offset(1 as ::core::ffi::c_int as isize),
            (&raw mut *(&raw mut (*db).ranges as *mut [lin; 2]).offset(0isize) as *mut lin)
                .offset(C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as isize),
        );
        if s.is_null() {
            return diff_type::DIFF_ERROR;
        }
    } else {
        (*db).ranges[0usize][C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize] =
            (*db).ranges[0usize][C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize];
    }
    s = skipwhite(s);
    let mut r#type: diff_type = diff_type::DIFF_ERROR;
    match *s as ::core::ffi::c_int {
        97 => {
            r#type = diff_type::DIFF_ADD;
        }
        99 => {
            r#type = diff_type::DIFF_CHANGE;
        }
        100 => {
            r#type = diff_type::DIFF_DELETE;
        }
        _ => return diff_type::DIFF_ERROR,
    }
    s = s.offset(1);
    s = readnum(
        skipwhite(s),
        (&raw mut *(&raw mut (*db).ranges as *mut [lin; 2]).offset(1isize) as *mut lin)
            .offset(C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as isize),
    );
    if s.is_null() {
        return diff_type::DIFF_ERROR;
    }
    s = skipwhite(s);
    if *s as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
        s = readnum(
            s.offset(1 as ::core::ffi::c_int as isize),
            (&raw mut *(&raw mut (*db).ranges as *mut [lin; 2]).offset(1isize) as *mut lin)
                .offset(C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as isize),
        );
        if s.is_null() {
            return diff_type::DIFF_ERROR;
        }
        s = skipwhite(s);
    } else {
        (*db).ranges[1usize][C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize] =
            (*db).ranges[1usize][C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize];
    }
    *string = s;
    return r#type;
}
unsafe extern "C" fn read_diff(
    mut filea: *const ::core::ffi::c_char,
    mut fileb: *const ::core::ffi::c_char,
    mut output_placement: *mut *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut argv: [*const ::core::ffi::c_char; 10] =
        [::core::ptr::null::<::core::ffi::c_char>(); 10];
    let mut ap: *mut *const ::core::ffi::c_char = &raw mut argv as *mut *const ::core::ffi::c_char;
    let c2rust_fresh4 = ap;
    ap = ap.offset(1);
    *c2rust_fresh4 = diff_program;
    if text {
        let c2rust_fresh5 = ap;
        ap = ap.offset(1);
        *c2rust_fresh5 = b"-a\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if strip_trailing_cr {
        let c2rust_fresh6 = ap;
        ap = ap.offset(1);
        *c2rust_fresh6 = b"--strip-trailing-cr\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let c2rust_fresh7 = ap;
    ap = ap.offset(1);
    *c2rust_fresh7 = b"--horizon-lines=100\0".as_ptr() as *const ::core::ffi::c_char;
    let c2rust_fresh8 = ap;
    ap = ap.offset(1);
    *c2rust_fresh8 = b"---no-directory\0".as_ptr() as *const ::core::ffi::c_char;
    let c2rust_fresh9 = ap;
    ap = ap.offset(1);
    *c2rust_fresh9 = b"--\0".as_ptr() as *const ::core::ffi::c_char;
    let c2rust_fresh10 = ap;
    ap = ap.offset(1);
    *c2rust_fresh10 = filea;
    let c2rust_fresh11 = ap;
    ap = ap.offset(1);
    *c2rust_fresh11 = fileb;
    *ap = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fds: [::core::ffi::c_int; 2] = [0; 2];
    if pipe(&raw mut fds as *mut ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
        perror_with_exit(b"pipe\0".as_ptr() as *const ::core::ffi::c_char);
    }
    let mut pid: pid_t = fork();
    if pid == 0 as ::core::ffi::c_int {
        close(fds[0usize]);
        if fds[1usize] != STDOUT_FILENO {
            dup2(fds[1usize], STDOUT_FILENO);
            close(fds[1usize]);
        }
        execvp(
            diff_program,
            &raw mut argv as *mut *const ::core::ffi::c_char as *mut *mut ::core::ffi::c_char
                as *const *mut ::core::ffi::c_char,
        );
        _exit(if *__errno_location() == ENOENT {
            127 as ::core::ffi::c_int
        } else {
            126 as ::core::ffi::c_int
        });
    }
    if pid == -1 as ::core::ffi::c_int {
        perror_with_exit(b"fork\0".as_ptr() as *const ::core::ffi::c_char);
    }
    close(fds[1usize]);
    let mut fd: ::core::ffi::c_int = fds[0usize];
    let mut pipestat: stat = stat {
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
    let mut current_chunk_size: idx_t = 0;
    if fstat(fd, &raw mut pipestat) < 0 as ::core::ffi::c_int
        || if (0 as __blksize_t) < pipestat.st_blksize
            && pipestat.st_blksize as size_t
                <= (-1 as ::core::ffi::c_int as size_t)
                    .wrapping_div(8 as size_t)
                    .wrapping_add(1 as size_t)
        {
            pipestat.st_blksize
        } else {
            DEV_BSIZE as __blksize_t
        } <= 0 as __blksize_t
        || {
            let (c2rust_result, c2rust_overflowed) = ((if (0 as __blksize_t) < pipestat.st_blksize
                && pipestat.st_blksize as size_t
                    <= (-1 as ::core::ffi::c_int as size_t)
                        .wrapping_div(8 as size_t)
                        .wrapping_add(1 as size_t)
            {
                pipestat.st_blksize
            } else {
                512 as __blksize_t
            }) as i128)
                .overflowing_add(0 as ::core::ffi::c_int as i128);
            let c2rust_result_narrow = c2rust_result as idx_t;
            *&raw mut current_chunk_size = c2rust_result_narrow;
            c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result
        }
    {
        current_chunk_size = (8 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as idx_t;
    }
    let mut diff_result: *mut ::core::ffi::c_char =
        ximalloc(current_chunk_size) as *mut ::core::ffi::c_char;
    let mut total: idx_t = 0 as idx_t;
    loop {
        let mut bytes_to_read: idx_t = current_chunk_size - total;
        let mut bytes: ptrdiff_t =
            block_read(fd, diff_result.offset(total as isize), bytes_to_read);
        total += bytes;
        if bytes != bytes_to_read {
            if bytes < 0 as ptrdiff_t {
                perror_with_exit(dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"read failed\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ));
            }
            break;
        } else {
            diff_result = xpalloc(
                diff_result as *mut ::core::ffi::c_void,
                &raw mut current_chunk_size,
                1 as idx_t,
                -1 as ptrdiff_t,
                1 as idx_t,
            ) as *mut ::core::ffi::c_char;
        }
    }
    if total != 0 as idx_t
        && *diff_result.offset((total - 1 as idx_t) as isize) as ::core::ffi::c_int
            != '\n' as ::core::ffi::c_int
    {
        fatal(b"invalid diff format; incomplete last line\0".as_ptr() as *const ::core::ffi::c_char);
    }
    *output_placement = diff_result;
    let mut werrno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut wstatus: ::core::ffi::c_int = 0;
    if close(fd) != 0 as ::core::ffi::c_int {
        perror_with_exit(b"close\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if waitpid(pid, &raw mut wstatus, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        perror_with_exit(b"waitpid\0".as_ptr() as *const ::core::ffi::c_char);
    }
    let mut status: ::core::ffi::c_int =
        if werrno == 0 && wstatus & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (wstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
        } else {
            INT_MAX
        };
    if C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int <= status {
        if 0 != 0 {
            error(
                C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                werrno,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    if status == 126 as ::core::ffi::c_int {
                        b"subsidiary program %s could not be invoked\0".as_ptr()
                            as *const ::core::ffi::c_char
                    } else if status == 127 as ::core::ffi::c_int {
                        b"subsidiary program %s not found\0".as_ptr() as *const ::core::ffi::c_char
                    } else if status == 2147483647 as ::core::ffi::c_int {
                        b"subsidiary program %s failed\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"subsidiary program %s failed (exit status %d)\0".as_ptr()
                            as *const ::core::ffi::c_char
                    },
                    5 as ::core::ffi::c_int,
                ),
                quote(diff_program),
                status,
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
                    werrno,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        if status == 126 as ::core::ffi::c_int {
                            b"subsidiary program %s could not be invoked\0".as_ptr()
                                as *const ::core::ffi::c_char
                        } else if status == 127 as ::core::ffi::c_int {
                            b"subsidiary program %s not found\0".as_ptr()
                                as *const ::core::ffi::c_char
                        } else if status == 2147483647 as ::core::ffi::c_int {
                            b"subsidiary program %s failed\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"subsidiary program %s failed (exit status %d)\0".as_ptr()
                                as *const ::core::ffi::c_char
                        },
                        5 as ::core::ffi::c_int,
                    ),
                    quote(diff_program),
                    status,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return diff_result.offset(total as isize);
}
unsafe extern "C" fn scan_diff_line(
    mut scan_ptr: *mut ::core::ffi::c_char,
    mut set_start: *mut *mut ::core::ffi::c_char,
    mut set_length: *mut idx_t,
    mut limit: *mut ::core::ffi::c_char,
    mut leadingchar: ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !(*scan_ptr.offset(0isize) as ::core::ffi::c_int == leadingchar as ::core::ffi::c_int
        && *scan_ptr.offset(1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int)
    {
        fatal(
            b"invalid diff format; incorrect leading line chars\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    }
    *set_start = scan_ptr.offset(2 as ::core::ffi::c_int as isize);
    let mut line_ptr: *mut ::core::ffi::c_char = *set_start;
    loop {
        let c2rust_fresh12 = line_ptr;
        line_ptr = line_ptr.offset(1);
        if *c2rust_fresh12 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
            break;
        }
    }
    *set_length = line_ptr.offset_from(*set_start) as idx_t;
    if line_ptr < limit && *line_ptr as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
        if edscript {
            fprintf(
                stderr,
                b"%s:\0".as_ptr() as *const ::core::ffi::c_char,
                squote(0 as ::core::ffi::c_int, program_name),
            );
        } else {
            *set_length -= 1;
        }
        line_ptr = line_ptr.offset(1);
        loop {
            if edscript {
                putc_unlocked(*line_ptr as ::core::ffi::c_int, stderr);
            }
            let c2rust_fresh13 = line_ptr;
            line_ptr = line_ptr.offset(1);
            if *c2rust_fresh13 as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
                break;
            }
        }
    }
    return line_ptr;
}
unsafe extern "C" fn output_diff3(
    mut outputfile: *mut FILE,
    mut diff: *mut diff3_block,
    mut mapping: *const ::core::ffi::c_int,
    mut rev_mapping: *const ::core::ffi::c_int,
) {
    let mut dontprint: ::core::ffi::c_int = 0;
    static mut skew_increment: [::core::ffi::c_int; 3] = [
        2 as ::core::ffi::c_int,
        3 as ::core::ffi::c_int,
        1 as ::core::ffi::c_int,
    ];
    let mut line_prefix: *const ::core::ffi::c_char = if initial_tab as ::core::ffi::c_int != 0 {
        b"\t\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"  \0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut ptr: *mut diff3_block = diff;
    while !ptr.is_null() {
        let mut x: [::core::ffi::c_char; 2] = [0; 2];
        let mut oddoneout: ::core::ffi::c_int = 0;
        match (*ptr).correspond {
            diff_type::DIFF_ALL => {
                x[0usize] = 0 as ::core::ffi::c_char;
                dontprint = 3 as ::core::ffi::c_int;
                oddoneout = 3 as ::core::ffi::c_int;
            }
            diff_type::DIFF_1ST | diff_type::DIFF_2ND | diff_type::DIFF_3RD => {
                oddoneout = *rev_mapping
                    .offset((*ptr).correspond.0.wrapping_sub(diff_type::DIFF_1ST.0) as isize);
                x[0usize] = (oddoneout + '1' as ::core::ffi::c_int) as ::core::ffi::c_char;
                x[1usize] = 0 as ::core::ffi::c_char;
                dontprint = (oddoneout == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            }
            _ => {
                fatal(
                    b"internal error: invalid diff type passed to output\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        }
        fprintf(
            outputfile,
            b"====%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut x as *mut ::core::ffi::c_char,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int {
            let mut realfile: ::core::ffi::c_int = *mapping.offset(i as isize);
            let mut low_t: lin = (*ptr).ranges[realfile as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize];
            let mut high_t: lin = (*ptr).ranges[realfile as usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
            fprintf(
                outputfile,
                b"%d:\0".as_ptr() as *const ::core::ffi::c_char,
                i + 1 as ::core::ffi::c_int,
            );
            match low_t - high_t {
                1 => {
                    fprintf(
                        outputfile,
                        b"%tda\n\0".as_ptr() as *const ::core::ffi::c_char,
                        low_t - 1 as lin,
                    );
                }
                0 => {
                    fprintf(
                        outputfile,
                        b"%tdc\n\0".as_ptr() as *const ::core::ffi::c_char,
                        low_t,
                    );
                }
                _ => {
                    fprintf(
                        outputfile,
                        b"%td,%tdc\n\0".as_ptr() as *const ::core::ffi::c_char,
                        low_t,
                        high_t,
                    );
                }
            }
            if i != dontprint {
                if low_t <= high_t {
                    let mut line: lin = 0 as lin;
                    loop {
                        fputs_unlocked(line_prefix, outputfile);
                        let mut cp: *mut ::core::ffi::c_char =
                            *(*ptr).lines[realfile as usize].offset(line as isize);
                        let mut length: idx_t =
                            *(*ptr).lengths[realfile as usize].offset(line as isize);
                        if 0 != 0
                            && 0 != 0
                            && ::core::mem::size_of::<::core::ffi::c_char>()
                                .wrapping_mul(length as size_t)
                                <= 8 as size_t
                            && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
                        {
                            ({
                                let mut __ptr: *const ::core::ffi::c_char =
                                    cp as *const ::core::ffi::c_char;
                                let mut __stream: *mut FILE = outputfile;
                                let mut __cnt: size_t = 0;
                                __cnt = ::core::mem::size_of::<::core::ffi::c_char>()
                                    .wrapping_mul(length as size_t);
                                while __cnt > 0 as size_t {
                                    let c2rust_fresh25 = __ptr;
                                    __ptr = __ptr.offset(1);
                                    if putc_unlocked(
                                        *c2rust_fresh25 as ::core::ffi::c_int,
                                        __stream,
                                    ) == EOF
                                    {
                                        break;
                                    }
                                    __cnt = __cnt.wrapping_sub(1);
                                }
                            });
                        } else {
                            if 0 != 0
                                && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
                                || 0 != 0 && length as size_t == 0 as size_t
                            {
                            } else {
                                fwrite_unlocked(
                                    cp as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<::core::ffi::c_char>(),
                                    length as size_t,
                                    outputfile,
                                );
                            };
                        };
                        if high_t - low_t <= line {
                            if *cp.offset((length - 1 as idx_t) as isize) as ::core::ffi::c_int
                                != '\n' as ::core::ffi::c_int
                            {
                                fprintf(
                                    outputfile,
                                    b"\n\\ %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"No newline at end of file\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        LC_MESSAGES,
                                    ),
                                );
                            }
                            break;
                        } else {
                            line += 1;
                        }
                    }
                }
            }
            i = if oddoneout == 1 as ::core::ffi::c_int {
                skew_increment[i as usize]
            } else {
                i + 1 as ::core::ffi::c_int
            };
        }
        ptr = (*ptr).next;
    }
}
unsafe extern "C" fn dotlines(
    mut outputfile: *mut FILE,
    mut b: *mut diff3_block,
    mut filenum: ::core::ffi::c_int,
) -> bool {
    let mut leading_dot: bool = r#false != 0;
    let mut i: lin = 0 as lin;
    while i
        < (*b).ranges[filenum as usize]
            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
            - (*b).ranges[filenum as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
            + 1 as lin
    {
        let mut line: *mut ::core::ffi::c_char = *(*b).lines[filenum as usize].offset(i as isize);
        if *line.offset(0isize) as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
            leading_dot = r#true != 0;
            fputc_unlocked('.' as ::core::ffi::c_int, outputfile);
        }
        if 0 != 0
            && 0 != 0
            && ::core::mem::size_of::<::core::ffi::c_char>()
                .wrapping_mul(*(*b).lengths[filenum as usize].offset(i as isize) as size_t)
                <= 8 as size_t
            && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
        {
            ({
                let mut __ptr: *const ::core::ffi::c_char = line as *const ::core::ffi::c_char;
                let mut __stream: *mut FILE = outputfile;
                let mut __cnt: size_t = 0;
                __cnt = ::core::mem::size_of::<::core::ffi::c_char>()
                    .wrapping_mul(*(*b).lengths[filenum as usize].offset(i as isize) as size_t);
                while __cnt > 0 as size_t {
                    let c2rust_fresh20 = __ptr;
                    __ptr = __ptr.offset(1);
                    if putc_unlocked(*c2rust_fresh20 as ::core::ffi::c_int, __stream) == EOF {
                        break;
                    }
                    __cnt = __cnt.wrapping_sub(1);
                }
            });
        } else {
            if 0 != 0 && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
                || 0 != 0
                    && *(*b).lengths[filenum as usize].offset(i as isize) as size_t == 0 as size_t
            {
            } else {
                fwrite_unlocked(
                    line as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_char>(),
                    *(*b).lengths[filenum as usize].offset(i as isize) as size_t,
                    outputfile,
                );
            };
        };
        i += 1;
    }
    return leading_dot;
}
unsafe extern "C" fn undotlines(
    mut outputfile: *mut FILE,
    mut leading_dot: bool,
    mut start: lin,
    mut num: lin,
) {
    fputs_unlocked(b".\n\0".as_ptr() as *const ::core::ffi::c_char, outputfile);
    if leading_dot {
        if num == 1 as lin {
            fprintf(
                outputfile,
                b"%tds/^\\.//\n\0".as_ptr() as *const ::core::ffi::c_char,
                start,
            );
        } else {
            fprintf(
                outputfile,
                b"%td,%tds/^\\.//\n\0".as_ptr() as *const ::core::ffi::c_char,
                start,
                start + num - 1 as lin,
            );
        }
    }
}
unsafe extern "C" fn output_diff3_edscript(
    mut outputfile: *mut FILE,
    mut diff: *mut diff3_block,
    mut mapping: *const ::core::ffi::c_int,
    mut rev_mapping: *const ::core::ffi::c_int,
    mut file0: *const ::core::ffi::c_char,
    mut file1: *const ::core::ffi::c_char,
    mut file2: *const ::core::ffi::c_char,
) -> bool {
    let mut conflicts_found: bool = r#false != 0;
    let mut b: *mut diff3_block = reverse_diff3_blocklist(diff);
    while !b.is_null() {
        let mut r#type: diff_type = diff_type(
            (if (*b).correspond.0 == diff_type::DIFF_ALL.0 {
                diff_type::DIFF_ALL.0 as ::core::ffi::c_int
            } else {
                diff_type::DIFF_1ST.0 as ::core::ffi::c_int
                    + *rev_mapping
                        .offset((*b).correspond.0.wrapping_sub(diff_type::DIFF_1ST.0) as isize)
            }) as ::core::ffi::c_uint,
        );
        let mut conflict: bool = false;
        's_6: {
            match r#type {
                diff_type::DIFF_2ND => {
                    if !show_2nd {
                        break 's_6;
                    } else {
                        conflict = r#true != 0;
                    }
                }
                diff_type::DIFF_3RD => {
                    if overlap_only {
                        break 's_6;
                    } else {
                        conflict = r#false != 0;
                    }
                }
                diff_type::DIFF_ALL => {
                    if simple_only {
                        break 's_6;
                    } else {
                        conflict = flagging;
                    }
                }
                _ => {
                    break 's_6;
                }
            }
            let mut low0: lin = (*b).ranges[*mapping
                .offset(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize)
                as usize][C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize];
            let mut high0: lin = (*b).ranges[*mapping
                .offset(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize)
                as usize][C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize];
            if conflict {
                conflicts_found = r#true != 0;
                fprintf(
                    outputfile,
                    b"%tda\n\0".as_ptr() as *const ::core::ffi::c_char,
                    high0,
                );
                let mut leading_dot: bool = r#false != 0;
                if r#type.0 == diff_type::DIFF_ALL.0 {
                    if show_2nd {
                        fprintf(
                            outputfile,
                            b"||||||| %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            file1,
                        );
                        leading_dot = dotlines(
                            outputfile,
                            b,
                            *mapping
                                .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize),
                        );
                    }
                    fputs_unlocked(
                        b"=======\n\0".as_ptr() as *const ::core::ffi::c_char,
                        outputfile,
                    );
                    leading_dot = leading_dot as ::core::ffi::c_int
                        | dotlines(
                            outputfile,
                            b,
                            *mapping
                                .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize),
                        ) as ::core::ffi::c_int
                        != 0;
                }
                fprintf(
                    outputfile,
                    b">>>>>>> %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    file2,
                );
                undotlines(
                    outputfile,
                    leading_dot,
                    high0 + 2 as lin,
                    (*b).ranges[*mapping
                        .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize)
                        as usize]
                        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                        - (*b).ranges[*mapping
                            .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize)
                            as usize]
                            [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                        + 1 as lin
                        + ((*b).ranges[*mapping
                            .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                            as usize]
                            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                            - (*b).ranges[*mapping
                                .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                                as usize]
                                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                            + 1 as lin)
                        + 1 as lin,
                );
                fprintf(
                    outputfile,
                    b"%tda\n<<<<<<< %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    low0 - 1 as lin,
                    if r#type.0 == diff_type::DIFF_ALL.0 {
                        file0
                    } else {
                        file1
                    },
                );
                leading_dot = r#false != 0;
                if r#type.0 == diff_type::DIFF_2ND.0 {
                    leading_dot = dotlines(
                        outputfile,
                        b,
                        *mapping.offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize),
                    );
                    fputs_unlocked(
                        b"=======\n\0".as_ptr() as *const ::core::ffi::c_char,
                        outputfile,
                    );
                }
                undotlines(
                    outputfile,
                    leading_dot,
                    low0 + 1 as lin,
                    (*b).ranges[*mapping
                        .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize)
                        as usize]
                        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                        - (*b).ranges[*mapping
                            .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize)
                            as usize]
                            [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                        + 1 as lin,
                );
            } else if (*b).ranges
                [*mapping.offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize) as usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                - (*b).ranges[*mapping
                    .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                    as usize][C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                + 1 as lin
                == 0 as lin
            {
                if low0 == high0 {
                    fprintf(
                        outputfile,
                        b"%tdd\n\0".as_ptr() as *const ::core::ffi::c_char,
                        low0,
                    );
                } else {
                    fprintf(
                        outputfile,
                        b"%td,%tdd\n\0".as_ptr() as *const ::core::ffi::c_char,
                        low0,
                        high0,
                    );
                }
            } else {
                match high0 - low0 {
                    -1 => {
                        fprintf(
                            outputfile,
                            b"%tda\n\0".as_ptr() as *const ::core::ffi::c_char,
                            high0,
                        );
                    }
                    0 => {
                        fprintf(
                            outputfile,
                            b"%tdc\n\0".as_ptr() as *const ::core::ffi::c_char,
                            high0,
                        );
                    }
                    _ => {
                        fprintf(
                            outputfile,
                            b"%td,%tdc\n\0".as_ptr() as *const ::core::ffi::c_char,
                            low0,
                            high0,
                        );
                    }
                }
                undotlines(
                    outputfile,
                    dotlines(
                        outputfile,
                        b,
                        *mapping.offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize),
                    ),
                    low0,
                    (*b).ranges[*mapping
                        .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                        as usize]
                        [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                        - (*b).ranges[*mapping
                            .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                            as usize]
                            [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                        + 1 as lin,
                );
            }
        }
        b = (*b).next;
    }
    if finalwrite {
        fputs_unlocked(
            b"w\nq\n\0".as_ptr() as *const ::core::ffi::c_char,
            outputfile,
        );
    }
    return conflicts_found;
}
unsafe extern "C" fn output_diff3_merge(
    mut infile: *mut FILE,
    mut outputfile: *mut FILE,
    mut diff: *mut diff3_block,
    mut mapping: *const ::core::ffi::c_int,
    mut rev_mapping: *const ::core::ffi::c_int,
    mut file0: *const ::core::ffi::c_char,
    mut file1: *const ::core::ffi::c_char,
    mut file2: *const ::core::ffi::c_char,
) -> bool {
    let mut conflicts_found: bool = r#false != 0;
    let mut linesread: lin = 0 as lin;
    let mut b: *mut diff3_block = diff;
    while !b.is_null() {
        let mut r#type: diff_type = diff_type(
            (if (*b).correspond.0 == diff_type::DIFF_ALL.0 {
                diff_type::DIFF_ALL.0 as ::core::ffi::c_int
            } else {
                diff_type::DIFF_1ST.0 as ::core::ffi::c_int
                    + *rev_mapping
                        .offset((*b).correspond.0.wrapping_sub(diff_type::DIFF_1ST.0) as isize)
            }) as ::core::ffi::c_uint,
        );
        let mut format_2nd: *const ::core::ffi::c_char =
            b"<<<<<<< %s\n\0".as_ptr() as *const ::core::ffi::c_char;
        let mut conflict: bool = false;
        's_8: {
            match r#type {
                diff_type::DIFF_2ND => {
                    if !show_2nd {
                        break 's_8;
                    } else {
                        conflict = r#true != 0;
                    }
                }
                diff_type::DIFF_3RD => {
                    if overlap_only {
                        break 's_8;
                    } else {
                        conflict = r#false != 0;
                    }
                }
                diff_type::DIFF_ALL => {
                    if simple_only {
                        break 's_8;
                    } else {
                        conflict = flagging;
                        format_2nd = b"||||||| %s\n\0".as_ptr() as *const ::core::ffi::c_char;
                    }
                }
                _ => {
                    break 's_8;
                }
            }
            let mut i0: lin = (*b).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                - linesread
                - 1 as lin;
            linesread += i0;
            loop {
                i0 -= 1;
                if 0 as lin > i0 {
                    break;
                }
                loop {
                    let mut c: ::core::ffi::c_int = getc_unlocked(infile);
                    if c == EOF {
                        if ferror_unlocked(infile) != 0 {
                            perror_with_exit(dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"read failed\0".as_ptr() as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ));
                        } else if feof_unlocked(infile) != 0 {
                            fatal(b"input file shrank\0".as_ptr() as *const ::core::ffi::c_char);
                        }
                    }
                    putc_unlocked(c, outputfile);
                    if c == '\n' as ::core::ffi::c_int {
                        break;
                    }
                }
            }
            if conflict {
                conflicts_found = r#true != 0;
                if r#type.0 == diff_type::DIFF_ALL.0 {
                    fprintf(
                        outputfile,
                        b"<<<<<<< %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        file0,
                    );
                    let mut i: lin = 0 as lin;
                    while i
                        < (*b).ranges[*mapping
                            .offset(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize)
                            as usize]
                            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                            - (*b).ranges[*mapping
                                .offset(C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize)
                                as usize]
                                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                            + 1 as lin
                    {
                        if 0 != 0
                            && 0 != 0
                            && ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(
                                *(*b).lengths[*mapping.offset(
                                    C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize,
                                ) as usize]
                                    .offset(i as isize) as size_t,
                            ) <= 8 as size_t
                            && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
                        {
                            ({
                                let mut __ptr: *const ::core::ffi::c_char =
                                    *(*b).lines[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i as isize)
                                        as *const ::core::ffi::c_char;
                                let mut __stream: *mut FILE = outputfile;
                                let mut __cnt: size_t = 0;
                                __cnt = ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(
                                    *(*b).lengths[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i as isize)
                                        as size_t,
                                );
                                while __cnt > 0 as size_t {
                                    let c2rust_fresh21 = __ptr;
                                    __ptr = __ptr.offset(1);
                                    if putc_unlocked(
                                        *c2rust_fresh21 as ::core::ffi::c_int,
                                        __stream,
                                    ) == EOF
                                    {
                                        break;
                                    }
                                    __cnt = __cnt.wrapping_sub(1);
                                }
                            });
                        } else {
                            if 0 != 0
                                && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
                                || 0 != 0
                                    && *(*b).lengths[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i as isize)
                                        as size_t
                                        == 0 as size_t
                            {
                            } else {
                                fwrite_unlocked(
                                    *(*b).lines[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i as isize)
                                        as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<::core::ffi::c_char>(),
                                    *(*b).lengths[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i as isize)
                                        as size_t,
                                    outputfile,
                                );
                            };
                        };
                        i += 1;
                    }
                }
                if show_2nd {
                    fprintf(outputfile, format_2nd, file1);
                    let mut i_0: lin = 0 as lin;
                    while i_0
                        < (*b).ranges[*mapping
                            .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize)
                            as usize]
                            [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                            - (*b).ranges[*mapping
                                .offset(C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize)
                                as usize]
                                [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                            + 1 as lin
                    {
                        if 0 != 0
                            && 0 != 0
                            && ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(
                                *(*b).lengths[*mapping.offset(
                                    C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize,
                                ) as usize]
                                    .offset(i_0 as isize) as size_t,
                            ) <= 8 as size_t
                            && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
                        {
                            ({
                                let mut __ptr: *const ::core::ffi::c_char =
                                    *(*b).lines[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i_0 as isize)
                                        as *const ::core::ffi::c_char;
                                let mut __stream: *mut FILE = outputfile;
                                let mut __cnt: size_t = 0;
                                __cnt = ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(
                                    *(*b).lengths[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i_0 as isize)
                                        as size_t,
                                );
                                while __cnt > 0 as size_t {
                                    let c2rust_fresh22 = __ptr;
                                    __ptr = __ptr.offset(1);
                                    if putc_unlocked(
                                        *c2rust_fresh22 as ::core::ffi::c_int,
                                        __stream,
                                    ) == EOF
                                    {
                                        break;
                                    }
                                    __cnt = __cnt.wrapping_sub(1);
                                }
                            });
                        } else {
                            if 0 != 0
                                && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
                                || 0 != 0
                                    && *(*b).lengths[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i_0 as isize)
                                        as size_t
                                        == 0 as size_t
                            {
                            } else {
                                fwrite_unlocked(
                                    *(*b).lines[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i_0 as isize)
                                        as *const ::core::ffi::c_void,
                                    ::core::mem::size_of::<::core::ffi::c_char>(),
                                    *(*b).lengths[*mapping.offset(
                                        C2Rust_Unnamed_0::FILE1.0 as ::core::ffi::c_int as isize,
                                    ) as usize]
                                        .offset(i_0 as isize)
                                        as size_t,
                                    outputfile,
                                );
                            };
                        };
                        i_0 += 1;
                    }
                }
                fputs_unlocked(
                    b"=======\n\0".as_ptr() as *const ::core::ffi::c_char,
                    outputfile,
                );
            }
            let mut i_1: lin = 0 as lin;
            while i_1
                < (*b).ranges[*mapping
                    .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                    as usize][C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                    - (*b).ranges[*mapping
                        .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                        as usize]
                        [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                    + 1 as lin
            {
                if 0 != 0
                    && 0 != 0
                    && ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(
                        *(*b).lengths[*mapping
                            .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                            as usize]
                            .offset(i_1 as isize) as size_t,
                    ) <= 8 as size_t
                    && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
                {
                    ({
                        let mut __ptr: *const ::core::ffi::c_char = *(*b).lines[*mapping
                            .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                            as usize]
                            .offset(i_1 as isize)
                            as *const ::core::ffi::c_char;
                        let mut __stream: *mut FILE = outputfile;
                        let mut __cnt: size_t = 0;
                        __cnt = ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(
                            *(*b).lengths[*mapping
                                .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                                as usize]
                                .offset(i_1 as isize) as size_t,
                        );
                        while __cnt > 0 as size_t {
                            let c2rust_fresh23 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*c2rust_fresh23 as ::core::ffi::c_int, __stream) == EOF
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                        }
                    });
                } else {
                    if 0 != 0 && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
                        || 0 != 0
                            && *(*b).lengths[*mapping
                                .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                                as usize]
                                .offset(i_1 as isize) as size_t
                                == 0 as size_t
                    {
                    } else {
                        fwrite_unlocked(
                            *(*b).lines[*mapping
                                .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                                as usize]
                                .offset(i_1 as isize)
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_char>(),
                            *(*b).lengths[*mapping
                                .offset(C2Rust_Unnamed_0::FILE2.0 as ::core::ffi::c_int as isize)
                                as usize]
                                .offset(i_1 as isize) as size_t,
                            outputfile,
                        );
                    };
                };
                i_1 += 1;
            }
            if conflict {
                fprintf(
                    outputfile,
                    b">>>>>>> %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    file2,
                );
            }
            let mut i1: lin = (*b).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
                [C2Rust_Unnamed_3::RANGE_END.0 as ::core::ffi::c_int as usize]
                - (*b).ranges[C2Rust_Unnamed_0::FILE0.0 as ::core::ffi::c_int as usize]
                    [C2Rust_Unnamed_3::RANGE_START.0 as ::core::ffi::c_int as usize]
                + 1 as lin;
            linesread += i1;
            loop {
                i1 -= 1;
                if 0 as lin > i1 {
                    break;
                }
                let mut c_0: ::core::ffi::c_int = 0;
                loop {
                    c_0 = getc_unlocked(infile);
                    if c_0 == '\n' as ::core::ffi::c_int {
                        break;
                    }
                    if c_0 == EOF {
                        if ferror_unlocked(infile) != 0 {
                            perror_with_exit(dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"read failed\0".as_ptr() as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ));
                        } else if feof_unlocked(infile) != 0 {
                            if i1 != 0 || !(*b).next.is_null() {
                                fatal(b"input file shrank\0".as_ptr() as *const ::core::ffi::c_char);
                            }
                            return conflicts_found;
                        }
                    }
                }
            }
        }
        b = (*b).next;
    }
    let mut c_1: ::core::ffi::c_int = 0;
    loop {
        c_1 = getc_unlocked(infile);
        if !(c_1 != EOF || ferror_unlocked(infile) | feof_unlocked(infile) == 0) {
            break;
        }
        putc_unlocked(c_1, outputfile);
    }
    return conflicts_found;
}
unsafe extern "C" fn reverse_diff3_blocklist(mut diff: *mut diff3_block) -> *mut diff3_block {
    let mut prev: *mut diff3_block = ::core::ptr::null_mut::<diff3_block>();
    let mut tmp: *mut diff3_block = diff;
    while !tmp.is_null() {
        let mut next: *mut diff3_block = (*tmp).next;
        (*tmp).next = prev;
        prev = tmp;
        tmp = next;
    }
    return prev;
}
unsafe extern "C" fn fatal(mut msgid: *const ::core::ffi::c_char) {
    if 0 != 0 {
        error(
            C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                msgid,
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
                    msgid,
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
unsafe extern "C" fn perror_with_exit(mut string: *const ::core::ffi::c_char) {
    if 0 != 0 {
        error(
            C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
            *__errno_location(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            string,
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
                string,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
}
pub const nullptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const GNULIB_LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"diffutils\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"GNU diffutils\0") };
