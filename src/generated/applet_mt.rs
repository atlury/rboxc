// Generated from pinned GNU Cpio 2.15 by scripts/translate-cpio.py.
// Source SHA-256: 43959ab7a6601a4d55a96f94a30da3847b866e5a53af87e9ad355f2363b5e663
/* mt -- control magnetic tape drive operation
   Copyright (C) 1991-2024 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
   02110-1301 USA
*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut stderr: *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bindtextdomain(
        __domainname: *const ::core::ffi::c_char,
        __dirname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_mt_stat_error"]
    fn stat_error(_: *const ::core::ffi::c_char);
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_mt_argp_parse"]
    fn argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_cpio_mt_argp_err_exit_status"]
    static mut argp_err_exit_status: error_t;
    #[link_name = "rboxc_cpio_mt_argp_state_help"]
    fn argp_state_help(
        __state: *const argp_state,
        __stream: *mut FILE,
        __flags: ::core::ffi::c_uint,
    );
    #[link_name = "rboxc_cpio_mt_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_cpio_mt_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_cpio_mt_rmt_dev_name__"]
    static mut rmt_dev_name__: *const ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_mt_rmt_open__"]
    fn rmt_open__(
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_mt_rmt_close__"]
    fn rmt_close__(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_mt_rmt_ioctl__"]
    fn rmt_ioctl__(
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_ulong,
        _: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_mt_force_local_option"]
    static mut force_local_option: bool;
    #[link_name = "rboxc_cpio_mt_argmatch_die"]
    static mut argmatch_die: argmatch_exit_fn;
    #[link_name = "rboxc_cpio_mt___xargmatch_internal"]
    fn __xargmatch_internal(
        context: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
        arglist: *const *const ::core::ffi::c_char,
        vallist: *const ::core::ffi::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
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
pub type __daddr_t = ::core::ffi::c_int;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = isize;
pub type error_t = ::core::ffi::c_int;
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
pub struct mtop {
    pub mt_op: ::core::ffi::c_short,
    pub mt_count: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mtget {
    pub mt_type: ::core::ffi::c_long,
    pub mt_resid: ::core::ffi::c_long,
    pub mt_dsreg: ::core::ffi::c_long,
    pub mt_gstat: ::core::ffi::c_long,
    pub mt_erreg: ::core::ffi::c_long,
    pub mt_fileno: __daddr_t,
    pub mt_blkno: __daddr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_option {
    pub name: *const ::core::ffi::c_char,
    pub key: ::core::ffi::c_int,
    pub arg: *const ::core::ffi::c_char,
    pub flags: ::core::ffi::c_int,
    pub doc: *const ::core::ffi::c_char,
    pub group: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp {
    pub options: *const argp_option,
    pub parser: argp_parser_t,
    pub args_doc: *const ::core::ffi::c_char,
    pub doc: *const ::core::ffi::c_char,
    pub children: *const argp_child,
    pub help_filter: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> *mut ::core::ffi::c_char,
    >,
    pub argp_domain: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_child {
    pub argp: *const argp,
    pub flags: ::core::ffi::c_int,
    pub header: *const ::core::ffi::c_char,
    pub group: ::core::ffi::c_int,
}
pub type argp_parser_t = Option<
    unsafe extern "C" fn(::core::ffi::c_int, *mut ::core::ffi::c_char, *mut argp_state) -> error_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct argp_state {
    pub root_argp: *const argp,
    pub argc: ::core::ffi::c_int,
    pub argv: *mut *mut ::core::ffi::c_char,
    pub next: ::core::ffi::c_int,
    pub flags: ::core::ffi::c_uint,
    pub arg_num: ::core::ffi::c_uint,
    pub quoted: ::core::ffi::c_int,
    pub input: *mut ::core::ffi::c_void,
    pub child_inputs: *mut *mut ::core::ffi::c_void,
    pub hook: *mut ::core::ffi::c_void,
    pub name: *mut ::core::ffi::c_char,
    pub err_stream: *mut FILE,
    pub out_stream: *mut FILE,
    pub pstate: *mut ::core::ffi::c_void,
}
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const RSH_COMMAND_OPTION: Self = Self(256);
}
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFCHR: ::core::ffi::c_int = 0o20000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFCHR: ::core::ffi::c_int = __S_IFCHR;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOCALEDIR: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"/root/rboxc/build/oracle/cpio/share/locale\0",
    )
};
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const PACKAGE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"cpio\0") };
pub const MTFSF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MTBSF: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MTFSR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MTBSR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MTWEOF: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MTREW: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MTOFFL: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const MTNOP: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MTRETEN: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const MTBSFM: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MTFSFM: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const MTEOM: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const MTERASE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const MTSEEK: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const DEFTAPE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"/dev/tape\0") };
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_KEY_ARG: ::core::ffi::c_int = 0;
pub const ARGP_KEY_FINI: ::core::ffi::c_int = 16777223;
pub const ARGP_IN_ORDER: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const ARGP_HELP_SHORT_USAGE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const ARGP_HELP_SEE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const ARGP_HELP_EXIT_ERR: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const ARGP_HELP_STD_USAGE: ::core::ffi::c_int =
    ARGP_HELP_SHORT_USAGE | ARGP_HELP_SEE | ARGP_HELP_EXIT_ERR;
#[inline]
unsafe extern "C" fn argp_usage(mut __state: *const argp_state) {
    argp_state_help(__state, stderr, ARGP_HELP_STD_USAGE as ::core::ffi::c_uint);
}
pub const __REM_BIAS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int;
pub const MT_EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MT_EXIT_INVOP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MT_EXIT_FAILURE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[export_name = "rboxc_cpio_mt_opnames"]
pub static mut opnames: [*const ::core::ffi::c_char; 19] = [
    b"eof\0".as_ptr() as *const ::core::ffi::c_char,
    b"weof\0".as_ptr() as *const ::core::ffi::c_char,
    b"fsf\0".as_ptr() as *const ::core::ffi::c_char,
    b"bsf\0".as_ptr() as *const ::core::ffi::c_char,
    b"fsr\0".as_ptr() as *const ::core::ffi::c_char,
    b"bsr\0".as_ptr() as *const ::core::ffi::c_char,
    b"rewind\0".as_ptr() as *const ::core::ffi::c_char,
    b"offline\0".as_ptr() as *const ::core::ffi::c_char,
    b"rewoffl\0".as_ptr() as *const ::core::ffi::c_char,
    b"eject\0".as_ptr() as *const ::core::ffi::c_char,
    b"status\0".as_ptr() as *const ::core::ffi::c_char,
    b"bsfm\0".as_ptr() as *const ::core::ffi::c_char,
    b"eom\0".as_ptr() as *const ::core::ffi::c_char,
    b"retension\0".as_ptr() as *const ::core::ffi::c_char,
    b"erase\0".as_ptr() as *const ::core::ffi::c_char,
    b"asf\0".as_ptr() as *const ::core::ffi::c_char,
    b"fsfm\0".as_ptr() as *const ::core::ffi::c_char,
    b"seek\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const MTASF: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
#[export_name = "rboxc_cpio_mt_operations"]
pub static mut operations: [::core::ffi::c_short; 18] = [
    MTWEOF as ::core::ffi::c_short,
    MTWEOF as ::core::ffi::c_short,
    MTFSF as ::core::ffi::c_short,
    MTBSF as ::core::ffi::c_short,
    MTFSR as ::core::ffi::c_short,
    MTBSR as ::core::ffi::c_short,
    MTREW as ::core::ffi::c_short,
    MTOFFL as ::core::ffi::c_short,
    MTOFFL as ::core::ffi::c_short,
    MTOFFL as ::core::ffi::c_short,
    MTNOP as ::core::ffi::c_short,
    MTBSFM as ::core::ffi::c_short,
    MTEOM as ::core::ffi::c_short,
    MTRETEN as ::core::ffi::c_short,
    MTERASE as ::core::ffi::c_short,
    MTASF as ::core::ffi::c_short,
    MTFSFM as ::core::ffi::c_short,
    MTSEEK as ::core::ffi::c_short,
];
#[export_name = "rboxc_cpio_mt_argp_program_bug_address"]
pub static mut argp_program_bug_address: *const ::core::ffi::c_char =
    b"<bug-cpio@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
static mut doc: [::core::ffi::c_char; 38] = unsafe {
    ::core::mem::transmute::<[u8; 38], [::core::ffi::c_char; 38]>(
        *b"control magnetic tape drive operation\0",
    )
};
#[export_name = "rboxc_cpio_mt_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 3] = [
    b"David MacKenzie\0".as_ptr() as *const ::core::ffi::c_char,
    b"Sergey Poznyakoff\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut options: [argp_option; 3] = [
    argp_option {
        name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: b"DEVICE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use device as the file name of the tape drive to operate on\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"rsh-command\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed::RSH_COMMAND_OPTION.0 as ::core::ffi::c_int,
        arg: b"COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use remote COMMAND instead of rsh\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: 0,
    },
];
#[export_name = "rboxc_cpio_mt_tapedev"]
pub static mut tapedev: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_cpio_mt_rsh_command_option"]
pub static mut rsh_command_option: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_cpio_mt_operation"]
pub static mut operation: ::core::ffi::c_short = 0;
#[export_name = "rboxc_cpio_mt_count"]
pub static mut count: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_cpio_mt_argcnt"]
pub static mut argcnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        ARGP_KEY_ARG => {
            let c2rust_fresh0 = argcnt;
            argcnt += 1;
            match c2rust_fresh0 {
                0 => {
                    operation = operations[__xargmatch_internal(
                        b"operation\0".as_ptr() as *const ::core::ffi::c_char,
                        arg,
                        &raw const opnames as *const *const ::core::ffi::c_char,
                        &raw mut operations as *mut ::core::ffi::c_short
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_short>(),
                        argmatch_die,
                        r#true != 0,
                    ) as usize];
                }
                1 => {
                    let mut p: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    let mut val: ::core::ffi::c_long =
                        strtol(arg, &raw mut p, 0 as ::core::ffi::c_int);
                    if *p as ::core::ffi::c_int != 0 || {
                        count = val as ::core::ffi::c_int;
                        count != count
                    } {
                        if 0 != 0 {
                            error(
                                1 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"invalid count value\0".as_ptr() as *const ::core::ffi::c_char,
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
                                        b"invalid count value\0".as_ptr()
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
                }
                _ => {
                    argp_usage(state);
                }
            }
        }
        ARGP_KEY_FINI => {
            if argcnt == 0 as ::core::ffi::c_int {
                argp_usage(state);
            }
            if tapedev.is_null() {
                tapedev = getenv(b"TAPE\0".as_ptr() as *const ::core::ffi::c_char);
                if tapedev.is_null() {
                    tapedev = DEFTAPE.as_ptr() as *mut ::core::ffi::c_char;
                }
            }
        }
        102 | 116 => {
            tapedev = arg;
        }
        256 => {
            rsh_command_option = arg;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    return 0 as error_t;
}
static mut argp: argp = unsafe {
    argp {
        options: &raw const options as *mut argp_option,
        parser: Some(
            parse_opt
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut argp_state,
                ) -> error_t,
        ),
        args_doc: b"operation [count]\0".as_ptr() as *const ::core::ffi::c_char,
        doc: &raw const doc as *mut ::core::ffi::c_char,
        children: ::core::ptr::null::<argp_child>(),
        help_filter: None,
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
#[export_name = "rboxc_cpio_mt_check_type"]
pub unsafe extern "C" fn check_type(
    mut dev: *mut ::core::ffi::c_char,
    mut desc: ::core::ffi::c_int,
) {
    let mut stats: stat = stat {
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
    if desc >= __REM_BIAS {
        return;
    }
    if fstat(desc, &raw mut stats) == -1 as ::core::ffi::c_int {
        stat_error(dev);
    }
    if stats.st_mode & S_IFMT as __mode_t != S_IFCHR as __mode_t {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s is not a character special file\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                dev,
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
                        b"%s is not a character special file\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    dev,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
#[export_name = "rboxc_cpio_mt_perform_operation"]
pub unsafe extern "C" fn perform_operation(
    mut dev: *mut ::core::ffi::c_char,
    mut desc: ::core::ffi::c_int,
    mut op: ::core::ffi::c_short,
    mut count_0: ::core::ffi::c_int,
) {
    let mut control: mtop = mtop {
        mt_op: 0,
        mt_count: 0,
    };
    control.mt_op = op;
    control.mt_count = count_0;
    if if desc >= __REM_BIAS {
        rmt_ioctl__(
            desc - __REM_BIAS,
            ((1 as ::core::ffi::c_uint)
                << 0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int
                | (('m' as ::core::ffi::c_int) << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as ::core::ffi::c_ulong
                | (::core::mem::size_of::<mtop>() as ::core::ffi::c_ulong)
                    << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
            &raw mut control as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        )
    } else {
        ioctl(
            desc,
            ((1 as ::core::ffi::c_uint)
                << 0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int
                | (('m' as ::core::ffi::c_int) << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                | ((1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as ::core::ffi::c_ulong
                | (::core::mem::size_of::<mtop>() as ::core::ffi::c_ulong)
                    << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
            &raw mut control as *mut ::core::ffi::c_char,
        )
    } == -1 as ::core::ffi::c_int
    {
        if 0 != 0 {
            error(
                2 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s: rmtioctl failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                dev,
            );
            if 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s: rmtioctl failed\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    dev,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
#[export_name = "rboxc_cpio_mt_print_status"]
pub unsafe extern "C" fn print_status(
    mut dev: *mut ::core::ffi::c_char,
    mut desc: ::core::ffi::c_int,
) {
    let mut status: mtget = mtget {
        mt_type: 0,
        mt_resid: 0,
        mt_dsreg: 0,
        mt_gstat: 0,
        mt_erreg: 0,
        mt_fileno: 0,
        mt_blkno: 0,
    };
    if if desc >= __REM_BIAS {
        rmt_ioctl__(
            desc - __REM_BIAS,
            ((2 as ::core::ffi::c_uint)
                << 0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int
                | (('m' as ::core::ffi::c_int) << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                | ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as ::core::ffi::c_ulong
                | (::core::mem::size_of::<mtget>() as ::core::ffi::c_ulong)
                    << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
            &raw mut status as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        )
    } else {
        ioctl(
            desc,
            ((2 as ::core::ffi::c_uint)
                << 0 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 8 as ::core::ffi::c_int
                    + 14 as ::core::ffi::c_int
                | (('m' as ::core::ffi::c_int) << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int)
                    as ::core::ffi::c_uint
                | ((2 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int) as ::core::ffi::c_uint)
                as ::core::ffi::c_ulong
                | (::core::mem::size_of::<mtget>() as ::core::ffi::c_ulong)
                    << 0 as ::core::ffi::c_int + 8 as ::core::ffi::c_int + 8 as ::core::ffi::c_int,
            &raw mut status as *mut ::core::ffi::c_char,
        )
    } == -1 as ::core::ffi::c_int
    {
        if 0 != 0 {
            error(
                2 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s: rmtioctl failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                dev,
            );
            if 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s: rmtioctl failed\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    dev,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    printf(
        b"drive type = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        status.mt_type as ::core::ffi::c_int,
    );
    printf(
        b"drive status = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        status.mt_dsreg as ::core::ffi::c_int,
    );
    printf(
        b"sense key error = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        status.mt_erreg as ::core::ffi::c_int,
    );
    printf(
        b"residue count = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        status.mt_resid as ::core::ffi::c_int,
    );
    printf(
        b"file number = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        status.mt_fileno,
    );
    printf(
        b"block number = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        status.mt_blkno,
    );
}
#[export_name = "rboxc_cpio_mt_fatal_exit"]
pub unsafe extern "C" fn fatal_exit() {
    exit(MT_EXIT_FAILURE);
}
unsafe extern "C" fn rboxc_cpio_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut tapedesc: ::core::ffi::c_int = 0;
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    set_program_name(*argv.offset(0isize));
    argp_version_setup(
        b"mt\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut program_authors as *mut *const ::core::ffi::c_char,
    );
    argmatch_die = Some(fatal_exit as unsafe extern "C" fn() -> ()) as argmatch_exit_fn;
    argp_err_exit_status = MT_EXIT_INVOP as error_t;
    if argp_parse(
        &raw mut argp,
        argc,
        argv,
        ARGP_IN_ORDER as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        NULL,
    ) != 0
    {
        exit(MT_EXIT_INVOP);
    }
    match operation as ::core::ffi::c_int {
        MTWEOF | MTERASE => {
            tapedesc = if !force_local_option
                && {
                    rmt_dev_name__ = strchr(tapedev, ':' as ::core::ffi::c_int);
                    !rmt_dev_name__.is_null()
                }
                && rmt_dev_name__ > tapedev as *const ::core::ffi::c_char
                && memchr(
                    tapedev as *const ::core::ffi::c_void,
                    '/' as ::core::ffi::c_int,
                    rmt_dev_name__.offset_from(tapedev) as size_t,
                )
                .is_null()
            {
                rmt_open__(
                    tapedev,
                    0o1 as ::core::ffi::c_int,
                    __REM_BIAS,
                    rsh_command_option,
                )
            } else {
                open(tapedev, 0o1 as ::core::ffi::c_int, 0 as ::core::ffi::c_int)
            };
        }
        _ => {
            tapedesc = if !force_local_option
                && {
                    rmt_dev_name__ = strchr(tapedev, ':' as ::core::ffi::c_int);
                    !rmt_dev_name__.is_null()
                }
                && rmt_dev_name__ > tapedev as *const ::core::ffi::c_char
                && memchr(
                    tapedev as *const ::core::ffi::c_void,
                    '/' as ::core::ffi::c_int,
                    rmt_dev_name__.offset_from(tapedev) as size_t,
                )
                .is_null()
            {
                rmt_open__(
                    tapedev,
                    0 as ::core::ffi::c_int,
                    __REM_BIAS,
                    rsh_command_option,
                )
            } else {
                open(tapedev, 0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int)
            };
        }
    }
    if tapedesc == -1 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s: rmtopen failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                tapedev,
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
                        b"%s: rmtopen failed\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    tapedev,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    if tapedesc >= 0 && tapedesc < __REM_BIAS {
        RBOXC_MT_OWNED_LOCAL = tapedesc;
    }
    check_type(tapedev, tapedesc);
    if operation as ::core::ffi::c_int == MTASF {
        perform_operation(
            tapedev,
            tapedesc,
            MTREW as ::core::ffi::c_short,
            1 as ::core::ffi::c_int,
        );
        operation = MTFSF as ::core::ffi::c_short;
    }
    perform_operation(tapedev, tapedesc, operation, count);
    if operation as ::core::ffi::c_int == MTNOP {
        print_status(tapedev, tapedesc);
    }
    RBOXC_MT_OWNED_LOCAL = -1;
    if if tapedesc >= __REM_BIAS {
        rmt_close__(tapedesc - __REM_BIAS)
    } else {
        close(tapedesc)
    } == -1 as ::core::ffi::c_int
    {
        if 0 != 0 {
            error(
                2 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s: rmtclose failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                tapedev,
            );
            if 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s: rmtclose failed\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    tapedev,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    exit(MT_EXIT_SUCCESS);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

include!("../bridges/mt-owned.rs");

include!("../bridges/cpio-invocation.rs");
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_mt(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rboxc_cpio_setup(argv);
    if libc::atexit(rboxc_mt_release) != 0 { libc::_exit(2); }
    rboxc_cpio_main_inner(argc, argv)
}
