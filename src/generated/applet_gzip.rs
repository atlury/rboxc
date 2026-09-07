// Generated from pinned GNU Gzip 1.14 by scripts/translate-gzip.py.
// Source SHA-256: e8ab9e54c5e242bed181c35b73e51c32f05c363b2d3f218a00d2eadd3bda9e9f
/* gzip (GNU zip) -- compress files with zip algorithm and 'compress' interface

   Copyright (C) 1999, 2001-2002, 2006-2007, 2009-2025 Free Software
   Foundation, Inc.
   Copyright (C) 1992-1993 Jean-loup Gailly

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

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
#[repr(C)]
pub struct __dirstream { _opaque: [u8; 0] }
#[repr(C, align(4096))]
pub struct RboxcGzipAligned<const N: usize>([uch; N]);
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn raise(__sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigismember(__set: *const sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigprocmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fchown(__fd: ::core::ffi::c_int, __owner: __uid_t, __group: __gid_t) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unlinkat(
        __fd: ::core::ffi::c_int,
        __name: *const ::core::ffi::c_char,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    static mut opterr: ::core::ffi::c_int;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fdatasync(__fildes: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fchmod(__fd: ::core::ffi::c_int, __mode: __mode_t) -> ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_header_bytes"]
    static mut header_bytes: off_t;
    #[link_name = "rboxc_gzip_zip"]
    fn zip(r#in: ::core::ffi::c_int, out: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_unzip_crc"]
    static mut unzip_crc: ulg;
    #[link_name = "rboxc_gzip_unzip"]
    fn unzip(r#in: ::core::ffi::c_int, out: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_check_zipfile"]
    fn check_zipfile(r#in: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_unpack"]
    fn unpack(r#in: ::core::ffi::c_int, out: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_unlzh"]
    fn unlzh(r#in: ::core::ffi::c_int, out: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_copy"]
    fn copy(r#in: ::core::ffi::c_int, out: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_updcrc"]
    fn updcrc(s: *const uch, n: ::core::ffi::c_uint) -> ulg;
    #[link_name = "rboxc_gzip_clear_bufs"]
    fn clear_bufs();
    #[link_name = "rboxc_gzip_fill_inbuf"]
    fn fill_inbuf(eof_ok: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_write_buf"]
    fn write_buf(fd: ::core::ffi::c_int, buf: voidp, cnt: ::core::ffi::c_uint);
    #[link_name = "rboxc_gzip_strlwr"]
    fn strlwr(s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_gzip_base_name"]
    fn gzip_base_name(fname: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_xunlink"]
    fn xunlink(fname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_add_envopt"]
    fn add_envopt(
        argcp: *mut ::core::ffi::c_int,
        argvp: *mut *mut *mut ::core::ffi::c_char,
        env_0: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_gzip_error"]
    fn gzip_error(m: *const ::core::ffi::c_char);
    #[link_name = "rboxc_gzip_read_error"]
    fn read_error();
    #[link_name = "rboxc_gzip_write_error"]
    fn write_error();
    #[link_name = "rboxc_gzip_display_ratio"]
    fn display_ratio(num: off_t, den: off_t, file: *mut FILE);
    #[link_name = "rboxc_gzip_unlzw"]
    fn unlzw(r#in: ::core::ffi::c_int, out: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    #[link_name = "rboxc_gzip_last_component"]
    fn last_component(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_open_safer"]
    fn open_safer(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_openat_safer"]
    fn openat_safer(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gzip_Version"]
    static mut Version: *const ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_yesno"]
    fn yesno() -> bool;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn fdopendir(__fd: ::core::ffi::c_int) -> *mut DIR;
    #[link_name = "rboxc_gzip_streamsavedir"]
    fn streamsavedir(_: *mut DIR, _: savedir_option) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gzip_fdutimens"]
    fn fdutimens(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: *const timespec,
    ) -> ::core::ffi::c_int;
}
pub type __uint32_t = u32;
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
pub type __clock_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type ptrdiff_t = isize;
pub type size_t = usize;
pub type intmax_t = ::libc::intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2Rust_Unnamed_8,
    pub _timer: C2Rust_Unnamed_7,
    pub _rt: C2Rust_Unnamed_6,
    pub _sigchld: C2Rust_Unnamed_5,
    pub _sigfault: C2Rust_Unnamed_2,
    pub _sigpoll: C2Rust_Unnamed_1,
    pub _sigsys: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub _addr_bnd: C2Rust_Unnamed_4,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2Rust_Unnamed_9,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
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
pub type voidp = *mut ::core::ffi::c_void;
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
pub type uch = ::core::ffi::c_uchar;
pub type ush = ::core::ffi::c_ushort;
pub type ulg = ::core::ffi::c_ulong;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const TIMESPEC_RESOLUTION: Self = Self(1000000000);
}
pub type DIR = __dirstream;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct savedir_option(pub ::core::ffi::c_uint);
impl savedir_option {
    pub const SAVEDIR_SORT_NONE: Self = Self(0);
    pub const SAVEDIR_SORT_NAME: Self = Self(1);
    pub const SAVEDIR_SORT_FASTREAD: Self = Self(0);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const PRESUME_INPUT_TTY_OPTION: Self = Self(128);
    pub const RSYNCABLE_OPTION: Self = Self(129);
    pub const SYNCHRONOUS_OPTION: Self = Self(130);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const try_opening_directories: Self = Self(1);
}
static mut license_msg: [*const ::core::ffi::c_char; 6] = [
    b"Copyright (C) 2025 Free Software Foundation, Inc.\0".as_ptr() as *const ::core::ffi::c_char,
    b"Copyright (C) 1993 Jean-loup Gailly.\0".as_ptr() as *const ::core::ffi::c_char,
    b"This is free software.  You may redistribute copies of it under the terms of\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"the GNU General Public License <https://www.gnu.org/licenses/gpl.html>.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"There is NO WARRANTY, to the extent permitted by law.\0".as_ptr()
        as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_ISUID: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __S_ISGID: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const __S_ISVTX: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const SIGXFSZ: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SIGXCPU: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const S_IRWXUGO: ::core::ffi::c_int = S_IRWXU | S_IRWXG | S_IRWXO;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPTIONS_VAR: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"GZIP\0") };
pub const Z_SUFFIX: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b".gz\0") };
pub const MAX_SUFFIX: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const MIN_PART: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EEXIST: ::core::ffi::c_int = 17;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ERROR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WARNING: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const STORED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const COMPRESSED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PACKED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LZHED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const DEFLATED: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const INBUFSIZ: ::core::ffi::c_int = 0x40000 as ::core::ffi::c_int;
pub const PACK_MAGIC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\x1F\x1E\0") };
pub const GZIP_MAGIC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\x1F\x8B\0") };
pub const OLD_GZIP_MAGIC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\x1F\x9E\0") };
pub const LZH_MAGIC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\x1F\xA0\0") };
pub const PKZIP_MAGIC: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"PK\x03\x04\0") };
pub const HEADER_CRC: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const EXTRA_FIELD: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const ORIG_NAME: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const COMMENT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const ENCRYPTED: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const RESERVED: ::core::ffi::c_int = 0xc0 as ::core::ffi::c_int;
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const BITS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const LZW_MAGIC: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\x1F\x9D\0") };
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_NOCTTY: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const S_ISUID: ::core::ffi::c_int = __S_ISUID;
pub const S_ISGID: ::core::ffi::c_int = __S_ISGID;
pub const S_ISVTX: ::core::ffi::c_int = __S_ISVTX;
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRWXG: ::core::ffi::c_int = S_IRWXU >> 3 as ::core::ffi::c_int;
pub const S_IRWXO: ::core::ffi::c_int = S_IRWXG >> 3 as ::core::ffi::c_int;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const O_SEARCH: ::core::ffi::c_int = O_RDONLY;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn get_stat_atime(mut st: *const stat) -> timespec {
    return (*st).st_atim;
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
pub const MAX_PATH_LEN: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const PART_SEP: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b".\0") };
#[export_name = "rboxc_gzip_inbuf"]
pub static mut inbuf: RboxcGzipAligned<262208> = RboxcGzipAligned([0; 262208]);
#[export_name = "rboxc_gzip_outbuf"]
pub static mut outbuf: RboxcGzipAligned<264192> = RboxcGzipAligned([0; 264192]);
#[export_name = "rboxc_gzip_d_buf"]
pub static mut d_buf: [ush; 32768] = [0; 32768];
#[export_name = "rboxc_gzip_window"]
pub static mut window: RboxcGzipAligned<65536> = RboxcGzipAligned([0; 65536]);
#[export_name = "rboxc_gzip_prev"]
pub static mut prev: [ush; 65536] = [0; 65536];
static mut presume_input_tty: bool = false;
static mut synchronous: bool = false;
static mut ascii: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_gzip_to_stdout"]
pub static mut to_stdout: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut decompress: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut force: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut keep: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut no_name: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut no_time: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut recursive: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut list: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut verbose: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_gzip_quiet"]
pub static mut quiet: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_gzip_test"]
pub static mut test: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut foreground: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_gzip_program_name"]
pub static mut program_name: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_gzip_maxbits"]
pub static mut maxbits: ::core::ffi::c_int = BITS;
#[export_name = "rboxc_gzip_method"]
pub static mut method: ::core::ffi::c_int = DEFLATED;
#[export_name = "rboxc_gzip_level"]
pub static mut level: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[export_name = "rboxc_gzip_exit_code"]
pub static mut exit_code: ::core::ffi::c_int = OK;
#[export_name = "rboxc_gzip_save_orig_name"]
pub static mut save_orig_name: ::core::ffi::c_int = 0;
static mut last_member: ::core::ffi::c_int = 0;
static mut part_nb: ::core::ffi::c_int = 0;
#[export_name = "rboxc_gzip_ifile_size"]
pub static mut ifile_size: off_t = 0;
static mut env: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut z_suffix: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut z_len: size_t = 0;
#[export_name = "rboxc_gzip_time_stamp"]
pub static mut time_stamp: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
static mut caught_signals: sigset_t = sigset_t { __val: [0; 16] };
static mut remove_ofname_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut remove_ofname: [::core::ffi::c_char; 1024] = [0; 1024];
static mut stdin_was_read: bool = false;
#[export_name = "rboxc_gzip_bytes_in"]
pub static mut bytes_in: off_t = 0;
#[export_name = "rboxc_gzip_bytes_out"]
pub static mut bytes_out: off_t = 0;
static mut total_in: off_t = 0;
static mut total_out: off_t = 0;
#[export_name = "rboxc_gzip_ifname"]
pub static mut ifname: [::core::ffi::c_char; 1024] = [0; 1024];
#[export_name = "rboxc_gzip_ofname"]
pub static mut ofname: [::core::ffi::c_char; 1024] = [0; 1024];
static mut dfname: [::core::ffi::c_char; 1024] = [0; 1024];
static mut istat: stat = stat {
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
#[export_name = "rboxc_gzip_ifd"]
pub static mut ifd: ::core::ffi::c_int = 0;
#[export_name = "rboxc_gzip_ofd"]
pub static mut ofd: ::core::ffi::c_int = 0;
static mut dfd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_gzip_insize"]
pub static mut insize: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_gzip_inptr"]
pub static mut inptr: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_gzip_outcnt"]
pub static mut outcnt: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_gzip_rsync"]
pub static mut rsync: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut handled_sig: [::core::ffi::c_int; 6] =
    [SIGINT, SIGHUP, SIGPIPE, SIGTERM, SIGXCPU, SIGXFSZ];
static mut shortopts: [::core::ffi::c_char; 34] = unsafe {
    ::core::mem::transmute::<[u8; 34], [::core::ffi::c_char; 34]>(
        *b"ab:cdfhH?klLmMnNqrS:tvVZ123456789\0",
    )
};
static mut longopts: [option; 27] = [
    option {
        name: b"ascii\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'a' as ::core::ffi::c_int,
    },
    option {
        name: b"to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"stdout\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"decompress\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"uncompress\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"force\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'f' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'h' as ::core::ffi::c_int,
    },
    option {
        name: b"keep\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'k' as ::core::ffi::c_int,
    },
    option {
        name: b"list\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"license\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'L' as ::core::ffi::c_int,
    },
    option {
        name: b"no-name\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"name\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'N' as ::core::ffi::c_int,
    },
    option {
        name: b"-presume-input-tty\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_11::PRESUME_INPUT_TTY_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'q' as ::core::ffi::c_int,
    },
    option {
        name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'q' as ::core::ffi::c_int,
    },
    option {
        name: b"synchronous\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_11::SYNCHRONOUS_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"recursive\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"suffix\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'S' as ::core::ffi::c_int,
    },
    option {
        name: b"test\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 't' as ::core::ffi::c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'V' as ::core::ffi::c_int,
    },
    option {
        name: b"fast\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: '1' as ::core::ffi::c_int,
    },
    option {
        name: b"best\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: '9' as ::core::ffi::c_int,
    },
    option {
        name: b"lzw\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'Z' as ::core::ffi::c_int,
    },
    option {
        name: b"bits\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"rsyncable\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_11::RSYNCABLE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
static mut work: Option<
    unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
> = Some(zip as unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int);
unsafe extern "C" fn try_help() {
    fprintf(
        stderr,
        b"Try `%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
    );
    do_exit(ERROR);
}
unsafe extern "C" fn help() {
    static mut help_msg: [*const ::core::ffi::c_char; 28] = [
        b"Compress or uncompress FILEs (by default, compress FILES in-place).\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"Mandatory arguments to long options are mandatory for short options too.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -c, --stdout      write on standard output, keep original files unchanged\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -d, --decompress  decompress\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -f, --force       force overwrite of output file and compress links\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -h, --help        give this help\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -k, --keep        keep (don't delete) input files\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -l, --list        list compressed file contents\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -L, --license     display software license\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -n, --no-name     do not save or restore the original name and timestamp\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -N, --name        save or restore the original name and timestamp\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -q, --quiet       suppress all warnings\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -r, --recursive   operate recursively on directories\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"      --rsyncable   make rsync-friendly archive\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -S, --suffix=SUF  use suffix SUF on compressed files\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"      --synchronous synchronous output (safer if system crashes, but slower)\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -t, --test        test compressed file integrity\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -v, --verbose     verbose mode\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -V, --version     display version number\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -1, --fast        compress faster\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -9, --best        compress better\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"With no FILE, or when FILE is -, read standard input.\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"Report bugs to <bug-gzip@gnu.org>.\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut p: *const *const ::core::ffi::c_char =
        &raw const help_msg as *const *const ::core::ffi::c_char;
    printf(
        b"Usage: %s [OPTION]... [FILE]...\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
    );
    while !(*p).is_null() {
        let c2rust_fresh2 = p;
        p = p.offset(1);
        printf(
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            *c2rust_fresh2,
        );
    }
}
unsafe extern "C" fn license() {
    let mut p: *const *const ::core::ffi::c_char =
        &raw const license_msg as *const *const ::core::ffi::c_char;
    printf(
        b"%s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
        Version,
    );
    while !(*p).is_null() {
        let c2rust_fresh3 = p;
        p = p.offset(1);
        printf(
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            *c2rust_fresh3,
        );
    }
}
unsafe extern "C" fn version() {
    license();
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(b"Written by Jean-loup Gailly.\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn progerror(mut string: *const ::core::ffi::c_char) {
    fprintf(
        stderr,
        b"%s: %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
        string,
        strerror(*__errno_location()),
    );
    exit_code = ERROR;
}
// SPDX-License-Identifier: GPL-3.0-or-later
extern "C" {
    static mut program_invocation_name: *mut ::core::ffi::c_char;
    static mut program_invocation_short_name: *mut ::core::ffi::c_char;
}
unsafe fn rboxc_gzip_set_invocation(name: *mut ::core::ffi::c_char) {
    program_invocation_name = name;
    let slash = libc::strrchr(name, b'/' as ::core::ffi::c_int);
    program_invocation_short_name = if slash.is_null() { name } else { slash.add(1) };
}

#[no_mangle]
pub unsafe extern "C" fn single_binary_main_gzip(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut file_count: ::core::ffi::c_int = 0;
    let mut proglen: size_t = 0;
    let mut argv_copy: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut env_argc: ::core::ffi::c_int = 0;
    let mut env_argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    rboxc_gzip_set_invocation(*argv);
    program_name = gzip_base_name(*argv.offset(0isize));
    proglen = strlen(program_name);
    if (4 as size_t) < proglen
        && strcmp(
            program_name
                .offset(proglen as isize)
                .offset(-(4 as ::core::ffi::c_int as isize)),
            b".exe\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        *program_name.offset(proglen.wrapping_sub(4 as size_t) as isize) =
            '\0' as ::core::ffi::c_char;
    }
    argv_copy = argv;
    env = add_envopt(&raw mut env_argc, &raw mut argv_copy, OPTIONS_VAR.as_ptr());
    env_argv = if !env.is_null() {
        argv_copy
    } else {
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>()
    };
    opterr = env.is_null() as ::core::ffi::c_int;
    z_suffix = Z_SUFFIX.as_ptr();
    z_len = strlen(z_suffix);
    loop {
        let mut optc: ::core::ffi::c_int = 0;
        if !env_argv.is_null() {
            loop {
                optc = getopt_long(
                    env_argc,
                    env_argv,
                    &raw const shortopts as *const ::core::ffi::c_char,
                    &raw const longopts as *const option,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
                if optc < 0 as ::core::ffi::c_int {
                    free(env_argv as *mut ::core::ffi::c_void);
                    env_argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
                    opterr = 1 as ::core::ffi::c_int;
                    optind = 1 as ::core::ffi::c_int;
                    break;
                } else if '1' as ::core::ffi::c_int <= optc && optc <= '9' as ::core::ffi::c_int
                    || optc == C2Rust_Unnamed_11::RSYNCABLE_OPTION.0 as ::core::ffi::c_int
                    || optc == C2Rust_Unnamed_11::SYNCHRONOUS_OPTION.0 as ::core::ffi::c_int
                {
                    break;
                }
            }
        }
        if env_argv.is_null() {
            optc = getopt_long(
                argc,
                argv,
                &raw const shortopts as *const ::core::ffi::c_char,
                &raw const longopts as *const option,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if optc < 0 as ::core::ffi::c_int {
            break;
        }
        match optc {
            97 => {
                ascii = 1 as ::core::ffi::c_int;
            }
            98 => {
                maxbits = atoi(optarg);
                while *optarg != 0 {
                    if !('0' as ::core::ffi::c_int <= *optarg as ::core::ffi::c_int
                        && *optarg as ::core::ffi::c_int <= '9' as ::core::ffi::c_int)
                    {
                        fprintf(
                            stderr,
                            b"%s: -b operand is not an integer\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            program_name,
                        );
                        try_help();
                    }
                    optarg = optarg.offset(1);
                }
            }
            99 => {
                to_stdout = 1 as ::core::ffi::c_int;
            }
            100 => {
                decompress = 1 as ::core::ffi::c_int;
            }
            102 => {
                force += 1;
            }
            104 | 72 => {
                help();
                finish_out();
            }
            107 => {
                keep = 1 as ::core::ffi::c_int;
            }
            108 => {
                to_stdout = 1 as ::core::ffi::c_int;
                test = to_stdout;
                decompress = test;
                list = decompress;
            }
            76 => {
                license();
                finish_out();
            }
            109 => {
                no_time = 1 as ::core::ffi::c_int;
            }
            77 => {
                no_time = 0 as ::core::ffi::c_int;
            }
            110 => {
                no_time = 1 as ::core::ffi::c_int;
                no_name = no_time;
            }
            78 => {
                no_time = 0 as ::core::ffi::c_int;
                no_name = no_time;
            }
            128 => {
                presume_input_tty = r#true != 0;
            }
            113 => {
                quiet = 1 as ::core::ffi::c_int;
                verbose = 0 as ::core::ffi::c_int;
            }
            114 => {
                recursive = 1 as ::core::ffi::c_int;
            }
            129 => {
                rsync = 1 as ::core::ffi::c_int;
            }
            83 => {
                z_len = 0 as size_t;
                while *optarg.offset(z_len as isize) != 0 {
                    if *optarg.offset(z_len as isize) as ::core::ffi::c_int
                        == '/' as ::core::ffi::c_int
                    {
                        fprintf(
                            stderr,
                            b"%s: suffix contains '/'\n\0".as_ptr() as *const ::core::ffi::c_char,
                            program_name,
                        );
                        do_exit(ERROR);
                    }
                    z_len = z_len.wrapping_add(1);
                }
                z_suffix = optarg;
            }
            130 => {
                synchronous = r#true != 0;
            }
            116 => {
                to_stdout = 1 as ::core::ffi::c_int;
                decompress = to_stdout;
                test = decompress;
            }
            118 => {
                verbose += 1;
                quiet = 0 as ::core::ffi::c_int;
            }
            86 => {
                version();
                finish_out();
            }
            90 => {
                fprintf(
                    stderr,
                    b"%s: -Z not supported in this version\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                );
                try_help();
            }
            49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                level = optc - '0' as ::core::ffi::c_int;
            }
            _ => {
                try_help();
            }
        }
    }
    if no_time < 0 as ::core::ffi::c_int {
        no_time = decompress;
    }
    if no_name < 0 as ::core::ffi::c_int {
        no_name = decompress;
    }
    file_count = argc - optind;
    if ascii != 0 && quiet == 0 {
        fprintf(
            stderr,
            b"%s: option --ascii ignored on this system\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
        );
    }
    if z_len == 0 as size_t || z_len > MAX_SUFFIX as size_t {
        fprintf(
            stderr,
            b"%s: invalid suffix '%s'\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
            z_suffix,
        );
        do_exit(ERROR);
    }
    if file_count != 0 as ::core::ffi::c_int {
        to_stdout != 0 && test == 0 && (decompress == 0 || ascii == 0);
        while optind < argc {
            let c2rust_fresh27 = optind;
            optind += 1;
            treat_file(*argv.offset(c2rust_fresh27 as isize));
        }
    } else {
        treat_stdin();
    }
    if stdin_was_read as ::core::ffi::c_int != 0 && close(STDIN_FILENO) != 0 as ::core::ffi::c_int {
        strcpy(
            &raw mut ifname as *mut ::core::ffi::c_char,
            b"stdin\0".as_ptr() as *const ::core::ffi::c_char,
        );
        read_error();
    }
    if list != 0 {
        if quiet == 0 && (1 as ::core::ffi::c_int) < file_count {
            do_list(-1 as ::core::ffi::c_int);
        }
        if fflush(stdout) != 0 as ::core::ffi::c_int {
            write_error();
        }
    }
    if to_stdout != 0
        && (synchronous as ::core::ffi::c_int != 0
            && fdatasync(STDOUT_FILENO) != 0 as ::core::ffi::c_int
            && *__errno_location() != EINVAL
            || close(STDOUT_FILENO) != 0 as ::core::ffi::c_int)
        && *__errno_location() != EBADF
    {
        write_error();
    }
    do_exit(exit_code);
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn input_eof() -> ::core::ffi::c_int {
    if decompress == 0 || last_member != 0 {
        return 1 as ::core::ffi::c_int;
    }
    if inptr == insize {
        if insize != INBUFSIZ as ::core::ffi::c_uint || fill_inbuf(1 as ::core::ffi::c_int) == EOF {
            return 1 as ::core::ffi::c_int;
        }
        inptr = 0 as ::core::ffi::c_uint;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn get_input_size_and_time() {
    ifile_size = -1 as off_t;
    time_stamp.tv_nsec = -1 as __syscall_slong_t;
    if istat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
        ifile_size = istat.st_size as off_t;
        if no_time == 0 || list != 0 {
            time_stamp = get_stat_mtime(&raw mut istat);
        }
    }
}
unsafe extern "C" fn treat_stdin() {
    if force == 0
        && list == 0
        && (presume_input_tty as ::core::ffi::c_int != 0
            || isatty(if decompress != 0 {
                STDIN_FILENO
            } else {
                STDOUT_FILENO
            }) != 0)
    {
        if quiet == 0 {
            fprintf(
                stderr,
                b"%s: compressed data not %s a terminal. Use -f to force %scompression.\nFor help, type: %s -h\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                program_name,
                if decompress != 0 {
                    b"read from\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"written to\0".as_ptr() as *const ::core::ffi::c_char
                },
                if decompress != 0 {
                    b"de\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"\0".as_ptr() as *const ::core::ffi::c_char
                },
                program_name,
            );
        }
        do_exit(ERROR);
    }
    decompress != 0 || ascii == 0;
    test == 0 && (decompress == 0 || ascii == 0);
    strcpy(
        &raw mut ifname as *mut ::core::ffi::c_char,
        b"stdin\0".as_ptr() as *const ::core::ffi::c_char,
    );
    strcpy(
        &raw mut ofname as *mut ::core::ffi::c_char,
        b"stdout\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if fstat(STDIN_FILENO, &raw mut istat) != 0 as ::core::ffi::c_int {
        progerror(b"standard input\0".as_ptr() as *const ::core::ffi::c_char);
        do_exit(ERROR);
    }
    get_input_size_and_time();
    clear_bufs();
    to_stdout = 1 as ::core::ffi::c_int;
    part_nb = 0 as ::core::ffi::c_int;
    ifd = STDIN_FILENO;
    stdin_was_read = r#true != 0;
    if decompress != 0 {
        method = get_method(ifd);
        if method < 0 as ::core::ffi::c_int {
            do_exit(exit_code);
        }
    }
    loop {
        if work.expect("non-null function pointer")(STDIN_FILENO, STDOUT_FILENO) != OK {
            return;
        }
        if input_eof() != 0 {
            break;
        }
        method = get_method(ifd);
        if method < 0 as ::core::ffi::c_int {
            return;
        }
    }
    if list != 0 {
        do_list(method);
        return;
    }
    if verbose != 0 {
        if test != 0 {
            fprintf(stderr, b" OK\n\0".as_ptr() as *const ::core::ffi::c_char);
        } else if decompress == 0 {
            display_ratio(bytes_in - (bytes_out - header_bytes), bytes_in, stderr);
            fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
static mut dot: ::core::ffi::c_char = '.' as ::core::ffi::c_char;
unsafe extern "C" fn atdir_eq(mut dir: *const ::core::ffi::c_char, mut dirlen: ptrdiff_t) -> bool {
    if dirlen == 0 as ptrdiff_t {
        dir = &raw const dot;
        dirlen = 1 as ptrdiff_t;
    }
    return memcmp(
        &raw mut dfname as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        dir as *const ::core::ffi::c_void,
        dirlen as size_t,
    ) == 0 as ::core::ffi::c_int
        && dfname[dirlen as usize] == 0;
}
unsafe extern "C" fn atdir_set(
    mut dir: *const ::core::ffi::c_char,
    mut dirlen: ptrdiff_t,
) -> ::core::ffi::c_int {
    if C2Rust_Unnamed_12::try_opening_directories.0 as ::core::ffi::c_int != 0
        && !atdir_eq(dir, dirlen)
    {
        if 0 as ::core::ffi::c_int <= dfd {
            close(dfd);
        }
        if dirlen == 0 as ptrdiff_t {
            dir = &raw const dot;
            dirlen = 1 as ptrdiff_t;
        }
        memcpy(
            &raw mut dfname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            dir as *const ::core::ffi::c_void,
            dirlen as size_t,
        );
        dfname[dirlen as usize] = '\0' as ::core::ffi::c_char;
        dfd = open_safer(
            &raw mut dfname as *mut ::core::ffi::c_char,
            O_SEARCH | O_DIRECTORY,
        );
    }
    return dfd;
}
unsafe extern "C" fn treat_file(mut iname: *mut ::core::ffi::c_char) {
    if strcmp(iname, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        let mut cflag: ::core::ffi::c_int = to_stdout;
        treat_stdin();
        to_stdout = cflag;
        return;
    }
    ifd = open_input_file(iname, &raw mut istat);
    if ifd < 0 as ::core::ffi::c_int {
        return;
    }
    if istat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
        if recursive != 0 {
            treat_dir(ifd, iname);
            return;
        }
        close(ifd);
        if quiet == 0 {
            fprintf(
                stderr,
                b"%s: %s is a directory -- ignored\n\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
            );
        }
        if exit_code == OK {
            exit_code = WARNING;
        }
        return;
    }
    if to_stdout == 0 {
        if !(istat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t) {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s: %s is not a directory or a regular file - ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                    &raw mut ifname as *mut ::core::ffi::c_char,
                );
            }
            if exit_code == OK {
                exit_code = WARNING;
            }
            close(ifd);
            return;
        }
        if istat.st_mode & S_ISUID as __mode_t != 0 {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s: %s is set-user-ID on execution - ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                    &raw mut ifname as *mut ::core::ffi::c_char,
                );
            }
            if exit_code == OK {
                exit_code = WARNING;
            }
            close(ifd);
            return;
        }
        if istat.st_mode & S_ISGID as __mode_t != 0 {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s: %s is set-group-ID on execution - ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                    &raw mut ifname as *mut ::core::ffi::c_char,
                );
            }
            if exit_code == OK {
                exit_code = WARNING;
            }
            close(ifd);
            return;
        }
        if force == 0 {
            if istat.st_mode & S_ISVTX as __mode_t != 0 {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"%s: %s has the sticky bit set - file ignored\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        program_name,
                        &raw mut ifname as *mut ::core::ffi::c_char,
                    );
                }
                if exit_code == OK {
                    exit_code = WARNING;
                }
                close(ifd);
                return;
            }
            if 2 as __nlink_t <= istat.st_nlink {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"%s: %s has %lu other link%s -- file ignored\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        program_name,
                        &raw mut ifname as *mut ::core::ffi::c_char,
                        istat.st_nlink.wrapping_sub(1 as ::core::ffi::c_ulong),
                        if istat.st_nlink == 2 as __nlink_t {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"s\0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                }
                if exit_code == OK {
                    exit_code = WARNING;
                }
                close(ifd);
                return;
            }
        }
    }
    get_input_size_and_time();
    if to_stdout != 0 && test == 0 {
        strcpy(
            &raw mut ofname as *mut ::core::ffi::c_char,
            b"stdout\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else if make_ofname() != OK {
        close(ifd);
        return;
    }
    clear_bufs();
    part_nb = 0 as ::core::ffi::c_int;
    if decompress != 0 {
        method = get_method(ifd);
        if method < 0 as ::core::ffi::c_int {
            close(ifd);
            return;
        }
    }
    if to_stdout != 0 {
        ofd = STDOUT_FILENO;
    } else {
        if create_outfile() != OK {
            return;
        }
        if decompress == 0 && save_orig_name != 0 && verbose == 0 && quiet == 0 {
            fprintf(
                stderr,
                b"%s: %s compressed to %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
                &raw mut ofname as *mut ::core::ffi::c_char,
            );
        }
    }
    if save_orig_name == 0 {
        save_orig_name = (no_name == 0) as ::core::ffi::c_int;
    }
    if verbose != 0 && list == 0 {
        fprintf(
            stderr,
            b"%s:\t\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut ifname as *mut ::core::ffi::c_char,
        );
    }
    loop {
        if Some(work.expect("non-null function pointer")).expect("non-null function pointer")(
            ifd, ofd,
        ) != OK
        {
            method = -1 as ::core::ffi::c_int;
            break;
        } else {
            if input_eof() != 0 {
                break;
            }
            method = get_method(ifd);
            if method < 0 as ::core::ffi::c_int {
                break;
            }
        }
    }
    if close(ifd) != 0 as ::core::ffi::c_int {
        read_error();
    }
    if list != 0 {
        do_list(method);
        return;
    }
    if to_stdout == 0 {
        copy_stat(&raw mut istat);
        if synchronous as ::core::ffi::c_int != 0
            && (0 as ::core::ffi::c_int <= dfd
                && fdatasync(dfd) != 0 as ::core::ffi::c_int
                && *__errno_location() != EINVAL
                || fsync(ofd) != 0 as ::core::ffi::c_int && *__errno_location() != EINVAL)
            || close(ofd) != 0 as ::core::ffi::c_int
        {
            write_error();
        }
        if keep == 0 {
            let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
            let mut unlink_errno: ::core::ffi::c_int = 0;
            let mut ifbase: *mut ::core::ffi::c_char =
                last_component(&raw mut ifname as *mut ::core::ffi::c_char);
            let mut ufd: ::core::ffi::c_int = if atdir_eq(
                &raw mut ifname as *mut ::core::ffi::c_char,
                ifbase.offset_from(&raw mut ifname as *mut ::core::ffi::c_char),
            ) as ::core::ffi::c_int
                != 0
            {
                dfd
            } else {
                -1 as ::core::ffi::c_int
            };
            let mut res: ::core::ffi::c_int = 0;
            sigprocmask(SIG_BLOCK, &raw mut caught_signals, &raw mut oldset);
            ::core::ptr::write_volatile(&raw mut remove_ofname_fd, -1 as ::core::ffi::c_int);
            res = if ufd < 0 as ::core::ffi::c_int {
                xunlink(&raw mut ifname as *mut ::core::ffi::c_char)
            } else {
                unlinkat(ufd, ifbase, 0 as ::core::ffi::c_int)
            };
            unlink_errno = if res == 0 as ::core::ffi::c_int {
                0 as ::core::ffi::c_int
            } else {
                *__errno_location()
            };
            sigprocmask(
                SIG_SETMASK,
                &raw mut oldset,
                ::core::ptr::null_mut::<sigset_t>(),
            );
            if unlink_errno != 0 {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"%s: %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        program_name,
                        &raw mut ifname as *mut ::core::ffi::c_char,
                        strerror(unlink_errno),
                    );
                }
                if exit_code == OK {
                    exit_code = WARNING;
                }
            }
        }
    }
    if method == -1 as ::core::ffi::c_int {
        if to_stdout == 0 {
            remove_output_file(r#false != 0);
        }
        return;
    }
    if verbose != 0 {
        if test != 0 {
            fprintf(stderr, b" OK\0".as_ptr() as *const ::core::ffi::c_char);
        } else if decompress != 0 {
            display_ratio(bytes_out - (bytes_in - header_bytes), bytes_out, stderr);
        } else {
            display_ratio(bytes_in - (bytes_out - header_bytes), bytes_in, stderr);
        }
        if test == 0 {
            fprintf(
                stderr,
                b" -- %s %s\0".as_ptr() as *const ::core::ffi::c_char,
                if keep != 0 {
                    b"created\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"replaced with\0".as_ptr() as *const ::core::ffi::c_char
                },
                &raw mut ofname as *mut ::core::ffi::c_char,
            );
        }
        fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn volatile_strcpy(
    mut dst: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
) {
    loop {
        let c2rust_fresh0 = src;
        src = src.offset(1);
        let c2rust_fresh1 = dst;
        dst = dst.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh1;
        ::core::ptr::write_volatile(
            c2rust_lvalue_ptr,
            ::core::ptr::read_volatile::<::core::ffi::c_char>(c2rust_fresh0),
        );
        if ::core::ptr::read_volatile::<::core::ffi::c_char>(c2rust_lvalue_ptr) == 0 {
            break;
        }
    }
}
unsafe extern "C" fn create_outfile() -> ::core::ffi::c_int {
    static mut signal_handlers_installed: bool = false;
    let mut name_shortened: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut flags: ::core::ffi::c_int = O_WRONLY
        | O_CREAT
        | O_EXCL
        | if ascii != 0 && decompress != 0 {
            0 as ::core::ffi::c_int
        } else {
            O_BINARY
        };
    let mut base: *const ::core::ffi::c_char = &raw mut ofname as *mut ::core::ffi::c_char;
    let mut atfd: ::core::ffi::c_int = AT_FDCWD;
    if keep == 0 {
        let mut b: *const ::core::ffi::c_char =
            last_component(&raw mut ofname as *mut ::core::ffi::c_char);
        let mut f: ::core::ffi::c_int = atdir_set(
            &raw mut ofname as *mut ::core::ffi::c_char,
            b.offset_from(&raw mut ofname as *mut ::core::ffi::c_char),
        );
        if 0 as ::core::ffi::c_int <= f {
            base = b;
            atfd = f;
        }
    }
    if !signal_handlers_installed {
        signal_handlers_installed = r#true != 0;
        install_signal_handlers();
    }
    loop {
        let mut open_errno: ::core::ffi::c_int = 0;
        let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
        volatile_strcpy(
            &raw mut remove_ofname as *mut ::core::ffi::c_char,
            &raw mut ofname as *mut ::core::ffi::c_char,
        );
        sigprocmask(SIG_BLOCK, &raw mut caught_signals, &raw mut oldset);
        ofd = openat_safer(atfd, base, flags, S_IRUSR | S_IWUSR);
        ::core::ptr::write_volatile(&raw mut remove_ofname_fd, ofd);
        open_errno = *__errno_location();
        sigprocmask(
            SIG_SETMASK,
            &raw mut oldset,
            ::core::ptr::null_mut::<sigset_t>(),
        );
        if 0 as ::core::ffi::c_int <= ofd {
            break;
        }
        match open_errno {
            ENAMETOOLONG => {
                shorten_name(&raw mut ofname as *mut ::core::ffi::c_char);
                name_shortened = 1 as ::core::ffi::c_int;
            }
            EEXIST => {
                if check_ofname() != OK {
                    close(ifd);
                    return ERROR;
                }
            }
            _ => {
                write_error();
                close(ifd);
                return ERROR;
            }
        }
    }
    if name_shortened != 0 && decompress != 0 {
        if quiet == 0 {
            fprintf(
                stderr,
                b"%s: %s: warning, name truncated\n\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
                &raw mut ofname as *mut ::core::ffi::c_char,
            );
        }
        if exit_code == OK {
            exit_code = WARNING;
        }
    }
    return OK;
}
unsafe extern "C" fn get_suffix(mut name: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut nlen: ::core::ffi::c_int = 0;
    let mut slen: ::core::ffi::c_int = 0;
    let mut suffix: [::core::ffi::c_char; 33] = [0; 33];
    static mut known_suffixes: [*const ::core::ffi::c_char; 10] = [
        ::core::ptr::null::<::core::ffi::c_char>(),
        b".gz\0".as_ptr() as *const ::core::ffi::c_char,
        b".z\0".as_ptr() as *const ::core::ffi::c_char,
        b".taz\0".as_ptr() as *const ::core::ffi::c_char,
        b".tgz\0".as_ptr() as *const ::core::ffi::c_char,
        b"-gz\0".as_ptr() as *const ::core::ffi::c_char,
        b"-z\0".as_ptr() as *const ::core::ffi::c_char,
        b"_z\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut suf: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut suffix_of_builtin: bool = r#false != 0;
    suf = (&raw mut known_suffixes as *mut *const ::core::ffi::c_char)
        .offset(1 as ::core::ffi::c_int as isize);
    while !(*suf).is_null() {
        let mut suflen: size_t = strlen(*suf);
        if z_len < suflen
            && strcmp(
                z_suffix,
                (*suf).offset(suflen as isize).offset(-(z_len as isize)),
            ) == 0 as ::core::ffi::c_int
        {
            suffix_of_builtin = r#true != 0;
            break;
        } else {
            suf = suf.offset(1);
        }
    }
    let mut z_lower: *mut ::core::ffi::c_char = xstrdup(z_suffix);
    strlwr(z_lower);
    known_suffixes[if suffix_of_builtin as ::core::ffi::c_int != 0 {
        ::core::mem::size_of::<[*const ::core::ffi::c_char; 10]>()
            .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
            .wrapping_sub(2usize)
    } else {
        0usize
    }] = z_lower;
    suf = (&raw mut known_suffixes as *mut *const ::core::ffi::c_char)
        .offset(suffix_of_builtin as ::core::ffi::c_int as isize);
    nlen = strlen(name) as ::core::ffi::c_int;
    if nlen <= MAX_SUFFIX + 2 as ::core::ffi::c_int {
        strcpy(&raw mut suffix as *mut ::core::ffi::c_char, name);
    } else {
        strcpy(
            &raw mut suffix as *mut ::core::ffi::c_char,
            name.offset(nlen as isize)
                .offset(-(MAX_SUFFIX as isize))
                .offset(-(2 as ::core::ffi::c_int as isize)),
        );
    }
    strlwr(&raw mut suffix as *mut ::core::ffi::c_char);
    slen = strlen(&raw mut suffix as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
    let mut r#match: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        let mut s: ::core::ffi::c_int = strlen(*suf) as ::core::ffi::c_int;
        if slen > s
            && !(suffix[(slen - s - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                == '/' as ::core::ffi::c_int)
            && strcmp(
                (&raw mut suffix as *mut ::core::ffi::c_char)
                    .offset(slen as isize)
                    .offset(-(s as isize)),
                *suf,
            ) == 0 as ::core::ffi::c_int
        {
            r#match = name.offset(nlen as isize).offset(-(s as isize));
            break;
        } else {
            suf = suf.offset(1);
            if (*suf).is_null() {
                break;
            }
        }
    }
    free(z_lower as *mut ::core::ffi::c_void);
    return r#match;
}
unsafe extern "C" fn open_and_stat(
    mut name: *mut ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut st: *mut stat,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut atfd: ::core::ffi::c_int = AT_FDCWD;
    let mut base: *const ::core::ffi::c_char = name;
    if to_stdout == 0 && force == 0 {
        flags |= O_NOFOLLOW;
    }
    if keep == 0 {
        let mut b: *const ::core::ffi::c_char = last_component(name);
        let mut f: ::core::ffi::c_int = atdir_set(name, b.offset_from(name));
        if 0 as ::core::ffi::c_int <= f {
            base = b;
            atfd = f;
        }
    }
    fd = openat_safer(atfd, base, flags);
    if 0 as ::core::ffi::c_int <= fd && fstat(fd, st) != 0 as ::core::ffi::c_int {
        let mut e: ::core::ffi::c_int = *__errno_location();
        close(fd);
        *__errno_location() = e;
        return -1 as ::core::ffi::c_int;
    }
    return fd;
}
unsafe extern "C" fn open_input_file(
    mut iname: *mut ::core::ffi::c_char,
    mut sbuf: *mut stat,
) -> ::core::ffi::c_int {
    let mut ilen: ::core::ffi::c_int = 0;
    let mut z_suffix_errno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut suffixes: [*const ::core::ffi::c_char; 6] = [
        ::core::ptr::null::<::core::ffi::c_char>(),
        b".gz\0".as_ptr() as *const ::core::ffi::c_char,
        b".z\0".as_ptr() as *const ::core::ffi::c_char,
        b"-z\0".as_ptr() as *const ::core::ffi::c_char,
        b".Z\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    let mut suf: *mut *const ::core::ffi::c_char =
        &raw mut suffixes as *mut *const ::core::ffi::c_char;
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut fd: ::core::ffi::c_int = 0;
    let mut open_flags: ::core::ffi::c_int = O_RDONLY
        | O_NONBLOCK
        | O_NOCTTY
        | if ascii != 0 && decompress == 0 {
            0 as ::core::ffi::c_int
        } else {
            O_BINARY
        };
    *suf = z_suffix;
    '_name_too_long: {
        if ::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(1usize)
            > strlen(iname)
        {
            strcpy(&raw mut ifname as *mut ::core::ffi::c_char, iname);
            fd = open_and_stat(
                &raw mut ifname as *mut ::core::ffi::c_char,
                open_flags,
                sbuf,
            );
            if 0 as ::core::ffi::c_int <= fd {
                return fd;
            }
            if decompress == 0 || *__errno_location() != ENOENT {
                progerror(&raw mut ifname as *mut ::core::ffi::c_char);
                return -1 as ::core::ffi::c_int;
            }
            s = get_suffix(&raw mut ifname as *mut ::core::ffi::c_char);
            if !s.is_null() {
                progerror(&raw mut ifname as *mut ::core::ffi::c_char);
                return -1 as ::core::ffi::c_int;
            }
            ilen = strlen(&raw mut ifname as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
            if strcmp(z_suffix, b".gz\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                suf = suf.offset(1);
            }
            loop {
                s = *suf;
                let mut s0: *const ::core::ffi::c_char = s;
                strcpy(&raw mut ifname as *mut ::core::ffi::c_char, iname);
                if ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
                    <= (ilen as size_t).wrapping_add(strlen(s))
                {
                    break '_name_too_long;
                }
                strcat(&raw mut ifname as *mut ::core::ffi::c_char, s);
                fd = open_and_stat(
                    &raw mut ifname as *mut ::core::ffi::c_char,
                    open_flags,
                    sbuf,
                );
                if 0 as ::core::ffi::c_int <= fd {
                    return fd;
                }
                if *__errno_location() != ENOENT {
                    progerror(&raw mut ifname as *mut ::core::ffi::c_char);
                    return -1 as ::core::ffi::c_int;
                }
                if strcmp(s0, z_suffix) == 0 as ::core::ffi::c_int {
                    z_suffix_errno = *__errno_location();
                }
                suf = suf.offset(1);
                if (*suf).is_null() {
                    break;
                }
            }
            strcpy(&raw mut ifname as *mut ::core::ffi::c_char, iname);
            strcat(&raw mut ifname as *mut ::core::ffi::c_char, z_suffix);
            *__errno_location() = z_suffix_errno;
            progerror(&raw mut ifname as *mut ::core::ffi::c_char);
            return -1 as ::core::ffi::c_int;
        }
    }
    fprintf(
        stderr,
        b"%s: %s: file name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
        iname,
    );
    exit_code = ERROR;
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn make_ofname() -> ::core::ffi::c_int {
    let mut suff: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    strcpy(
        &raw mut ofname as *mut ::core::ffi::c_char,
        &raw mut ifname as *mut ::core::ffi::c_char,
    );
    suff = get_suffix(&raw mut ofname as *mut ::core::ffi::c_char);
    if decompress != 0 {
        if suff.is_null() {
            if recursive == 0 && test != 0 {
                return OK;
            }
            if verbose != 0 || recursive == 0 && quiet == 0 {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"%s: %s: unknown suffix -- ignored\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        program_name,
                        &raw mut ifname as *mut ::core::ffi::c_char,
                    );
                }
                if exit_code == OK {
                    exit_code = WARNING;
                }
            }
            return WARNING;
        }
        strlwr(suff);
        if strcmp(suff, b".tgz\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
            || strcmp(suff, b".taz\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            strcpy(suff, b".tar\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            *suff = '\0' as ::core::ffi::c_char;
        }
    } else if !suff.is_null() && force == 0 {
        if verbose != 0 || recursive == 0 && quiet == 0 {
            fprintf(
                stderr,
                b"%s: %s already has %s suffix -- unchanged\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
                suff,
            );
        }
        return WARNING;
    } else {
        save_orig_name = 0 as ::core::ffi::c_int;
        if ::core::mem::size_of::<[::core::ffi::c_char; 1024]>()
            <= strlen(&raw mut ofname as *mut ::core::ffi::c_char).wrapping_add(z_len)
        {
            if quiet == 0 {
                fprintf(
                    stderr,
                    b"%s: %s: file name too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                    program_name,
                    &raw mut ifname as *mut ::core::ffi::c_char,
                );
            }
            if exit_code == OK {
                exit_code = WARNING;
            }
            return WARNING;
        } else {
            strcat(&raw mut ofname as *mut ::core::ffi::c_char, z_suffix);
        }
    }
    return OK;
}
unsafe extern "C" fn discard_input_bytes(mut nbytes: size_t, mut flags: ::core::ffi::c_uint) {
    while nbytes != 0 as size_t {
        let mut c: uch = (if inptr < insize {
            let c2rust_fresh24 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh24 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as uch;
        if flags & HEADER_CRC as ::core::ffi::c_uint != 0 {
            updcrc(&raw mut c, 1 as ::core::ffi::c_uint);
        }
        if nbytes != -1 as ::core::ffi::c_int as size_t {
            nbytes = nbytes.wrapping_sub(1);
        } else if c == 0 {
            break;
        }
    }
}
unsafe extern "C" fn get_method(mut r#in: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut flags: uch = 0;
    let mut magic: [uch; 10] = [0; 10];
    let mut imagic0: ::core::ffi::c_int = 0;
    let mut imagic1: ::core::ffi::c_int = 0;
    let mut stamp: ulg = 0;
    if force != 0 && to_stdout != 0 {
        imagic0 = if inptr < insize {
            let c2rust_fresh4 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh4 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(1 as ::core::ffi::c_int)
        };
        magic[0usize] = imagic0 as uch;
        imagic1 = if inptr < insize {
            let c2rust_fresh5 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh5 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(1 as ::core::ffi::c_int)
        };
        magic[1usize] = imagic1 as uch;
    } else {
        magic[0usize] = (if inptr < insize {
            let c2rust_fresh6 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh6 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as uch;
        imagic0 = 0 as ::core::ffi::c_int;
        if magic[0usize] != 0 {
            magic[1usize] = (if inptr < insize {
                let c2rust_fresh7 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf.0[c2rust_fresh7 as usize] as ::core::ffi::c_int
            } else {
                fill_inbuf(0 as ::core::ffi::c_int)
            }) as uch;
            imagic1 = 0 as ::core::ffi::c_int;
        } else {
            imagic1 = if inptr < insize {
                let c2rust_fresh8 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf.0[c2rust_fresh8 as usize] as ::core::ffi::c_int
            } else {
                fill_inbuf(1 as ::core::ffi::c_int)
            };
            magic[1usize] = imagic1 as uch;
        }
    }
    method = -1 as ::core::ffi::c_int;
    part_nb += 1;
    header_bytes = 0 as off_t;
    last_member = 0 as ::core::ffi::c_int;
    if memcmp(
        &raw mut magic as *mut uch as *const ::core::ffi::c_void,
        GZIP_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
        || memcmp(
            &raw mut magic as *mut uch as *const ::core::ffi::c_void,
            OLD_GZIP_MAGIC.as_ptr() as *const ::core::ffi::c_void,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        method = if inptr < insize {
            let c2rust_fresh9 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh9 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        };
        if method != DEFLATED {
            fprintf(
                stderr,
                b"%s: %s: unknown method %d -- not supported\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
                method,
            );
            exit_code = ERROR;
            return -1 as ::core::ffi::c_int;
        }
        work = Some(
            unzip
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
            >;
        flags = (if inptr < insize {
            let c2rust_fresh10 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh10 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as uch;
        if flags as ::core::ffi::c_int & ENCRYPTED != 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"%s: %s is encrypted -- not supported\n\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
            );
            exit_code = ERROR;
            return -1 as ::core::ffi::c_int;
        }
        if flags as ::core::ffi::c_int & RESERVED != 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"%s: %s has flags 0x%x -- not supported\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
                flags as ::core::ffi::c_int,
            );
            exit_code = ERROR;
            if force <= 1 as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
        }
        stamp = (if inptr < insize {
            let c2rust_fresh11 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh11 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as ulg;
        stamp |= ((if inptr < insize {
            let c2rust_fresh12 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh12 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as ulg)
            << 8 as ::core::ffi::c_int;
        stamp |= ((if inptr < insize {
            let c2rust_fresh13 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh13 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as ulg)
            << 16 as ::core::ffi::c_int;
        stamp |= ((if inptr < insize {
            let c2rust_fresh14 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh14 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as ulg)
            << 24 as ::core::ffi::c_int;
        if stamp != 0 as ulg && no_time == 0 {
            if stamp
                <= (if (0 as ::core::ffi::c_int as time_t) < -1 as ::core::ffi::c_int as time_t {
                    -1 as ::core::ffi::c_int as time_t
                } else {
                    (((1 as ::core::ffi::c_int as time_t)
                        << ::core::mem::size_of::<time_t>()
                            .wrapping_mul(CHAR_BIT as usize)
                            .wrapping_sub(2usize))
                        - 1 as time_t)
                        * 2 as time_t
                        + 1 as time_t
                }) as ulg
            {
                time_stamp.tv_sec = stamp as __time_t;
                time_stamp.tv_nsec = 0 as __syscall_slong_t;
            } else {
                if quiet == 0 {
                    fprintf(
                        stderr,
                        b"%s: %s: MTIME %lu out of range for this platform\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        program_name,
                        &raw mut ifname as *mut ::core::ffi::c_char,
                        stamp,
                    );
                }
                if exit_code == OK {
                    exit_code = WARNING;
                }
                time_stamp.tv_sec =
                    (if (0 as ::core::ffi::c_int as time_t) < -1 as ::core::ffi::c_int as time_t {
                        -1 as ::core::ffi::c_int as time_t
                    } else {
                        (((1 as ::core::ffi::c_int as time_t)
                            << ::core::mem::size_of::<time_t>()
                                .wrapping_mul(CHAR_BIT as usize)
                                .wrapping_sub(2usize))
                            - 1 as time_t)
                            * 2 as time_t
                            + 1 as time_t
                    }) as __time_t;
                time_stamp.tv_nsec =
                    (C2Rust_Unnamed_10::TIMESPEC_RESOLUTION.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as __syscall_slong_t;
            }
        }
        magic[8usize] = (if inptr < insize {
            let c2rust_fresh15 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh15 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as uch;
        magic[9usize] = (if inptr < insize {
            let c2rust_fresh16 = inptr;
            inptr = inptr.wrapping_add(1);
            inbuf.0[c2rust_fresh16 as usize] as ::core::ffi::c_int
        } else {
            fill_inbuf(0 as ::core::ffi::c_int)
        }) as uch;
        if flags as ::core::ffi::c_int & HEADER_CRC != 0 {
            magic[2usize] = DEFLATED as uch;
            magic[3usize] = flags;
            magic[4usize] = (stamp & 0xff as ulg) as uch;
            magic[5usize] = (stamp >> 8 as ::core::ffi::c_int & 0xff as ulg) as uch;
            magic[6usize] = (stamp >> 16 as ::core::ffi::c_int & 0xff as ulg) as uch;
            magic[7usize] = (stamp >> 24 as ::core::ffi::c_int) as uch;
            updcrc(::core::ptr::null::<uch>(), 0 as ::core::ffi::c_uint);
            updcrc(&raw mut magic as *mut uch, 10 as ::core::ffi::c_uint);
        }
        if flags as ::core::ffi::c_int & EXTRA_FIELD != 0 as ::core::ffi::c_int {
            let mut lenbuf: [uch; 2] = [0; 2];
            lenbuf[0usize] = (if inptr < insize {
                let c2rust_fresh17 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf.0[c2rust_fresh17 as usize] as ::core::ffi::c_int
            } else {
                fill_inbuf(0 as ::core::ffi::c_int)
            }) as uch;
            let mut len: ::core::ffi::c_uint = lenbuf[0usize] as ::core::ffi::c_uint;
            lenbuf[1usize] = (if inptr < insize {
                let c2rust_fresh18 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf.0[c2rust_fresh18 as usize] as ::core::ffi::c_int
            } else {
                fill_inbuf(0 as ::core::ffi::c_int)
            }) as uch;
            len |= ((lenbuf[1usize] as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
                as ::core::ffi::c_uint;
            if verbose != 0 {
                fprintf(
                    stderr,
                    b"%s: %s: extra field of %u bytes ignored\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                    &raw mut ifname as *mut ::core::ffi::c_char,
                    len,
                );
            }
            if flags as ::core::ffi::c_int & HEADER_CRC != 0 {
                updcrc(&raw mut lenbuf as *mut uch, 2 as ::core::ffi::c_uint);
            }
            discard_input_bytes(len as size_t, flags as ::core::ffi::c_uint);
        }
        if flags as ::core::ffi::c_int & ORIG_NAME != 0 as ::core::ffi::c_int {
            if no_name != 0 || to_stdout != 0 && list == 0 || part_nb > 1 as ::core::ffi::c_int {
                discard_input_bytes(
                    -1 as ::core::ffi::c_int as size_t,
                    flags as ::core::ffi::c_uint,
                );
            } else {
                let mut p: *mut ::core::ffi::c_char =
                    gzip_base_name(&raw mut ofname as *mut ::core::ffi::c_char);
                let mut base: *mut ::core::ffi::c_char = p;
                loop {
                    *p = (if inptr < insize {
                        let c2rust_fresh19 = inptr;
                        inptr = inptr.wrapping_add(1);
                        inbuf.0[c2rust_fresh19 as usize] as ::core::ffi::c_int
                    } else {
                        fill_inbuf(0 as ::core::ffi::c_int)
                    }) as ::core::ffi::c_char;
                    let c2rust_fresh20 = p;
                    p = p.offset(1);
                    if *c2rust_fresh20 as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                        break;
                    }
                    if p >= (&raw mut ofname as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as isize)
                    {
                        gzip_error(b"corrupted input -- file name too large\0".as_ptr()
                            as *const ::core::ffi::c_char);
                    }
                }
                if flags as ::core::ffi::c_int & HEADER_CRC != 0 {
                    updcrc(base as *mut uch, p.offset_from(base) as ::core::ffi::c_uint);
                }
                p = gzip_base_name(base);
                memmove(
                    base as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    strlen(p).wrapping_add(1 as size_t),
                );
                if list == 0 {
                    if !base.is_null() {
                        list = 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
        if flags as ::core::ffi::c_int & COMMENT != 0 as ::core::ffi::c_int {
            discard_input_bytes(
                -1 as ::core::ffi::c_int as size_t,
                flags as ::core::ffi::c_uint,
            );
        }
        if flags as ::core::ffi::c_int & HEADER_CRC != 0 {
            let mut crc16: ::core::ffi::c_uint =
                (updcrc(&raw mut magic as *mut uch, 0 as ::core::ffi::c_uint) & 0xffff as ulg)
                    as ::core::ffi::c_uint;
            let mut header16: ::core::ffi::c_uint = (if inptr < insize {
                let c2rust_fresh21 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf.0[c2rust_fresh21 as usize] as ::core::ffi::c_int
            } else {
                fill_inbuf(0 as ::core::ffi::c_int)
            }) as ::core::ffi::c_uint;
            header16 |= ((if inptr < insize {
                let c2rust_fresh22 = inptr;
                inptr = inptr.wrapping_add(1);
                inbuf.0[c2rust_fresh22 as usize] as ::core::ffi::c_int
            } else {
                fill_inbuf(0 as ::core::ffi::c_int)
            }) as ::core::ffi::c_uint)
                << 8 as ::core::ffi::c_int;
            if header16 != crc16 {
                fprintf(
                    stderr,
                    b"%s: %s: header checksum 0x%04x != computed checksum 0x%04x\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                    &raw mut ifname as *mut ::core::ffi::c_char,
                    header16,
                    crc16,
                );
                exit_code = ERROR;
                if force <= 1 as ::core::ffi::c_int {
                    return -1 as ::core::ffi::c_int;
                }
            }
        }
        if part_nb == 1 as ::core::ffi::c_int {
            header_bytes = inptr.wrapping_add(
                (2 as ::core::ffi::c_int * 4 as ::core::ffi::c_int) as ::core::ffi::c_uint,
            ) as off_t;
        }
    } else if memcmp(
        &raw mut magic as *mut uch as *const ::core::ffi::c_void,
        PKZIP_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
        && inptr == 2 as ::core::ffi::c_uint
        && memcmp(
            &raw mut inbuf as *mut uch as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            PKZIP_MAGIC.as_ptr() as *const ::core::ffi::c_void,
            4 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        inptr = 0 as ::core::ffi::c_uint;
        work = Some(
            unzip
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
            >;
        if check_zipfile(r#in) != OK {
            return -1 as ::core::ffi::c_int;
        }
        last_member = 1 as ::core::ffi::c_int;
    } else if memcmp(
        &raw mut magic as *mut uch as *const ::core::ffi::c_void,
        PACK_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        work = Some(
            unpack
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
            >;
        method = PACKED;
    } else if memcmp(
        &raw mut magic as *mut uch as *const ::core::ffi::c_void,
        LZW_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        work = Some(
            unlzw
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
            >;
        method = COMPRESSED;
        last_member = 1 as ::core::ffi::c_int;
    } else if memcmp(
        &raw mut magic as *mut uch as *const ::core::ffi::c_void,
        LZH_MAGIC.as_ptr() as *const ::core::ffi::c_void,
        2 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        work = Some(
            unlzh
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    ::core::ffi::c_int,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
            >;
        method = LZHED;
        last_member = 1 as ::core::ffi::c_int;
    } else if force != 0 && to_stdout != 0 && list == 0 {
        method = STORED;
        work = Some(
            copy as unsafe extern "C" fn(
                ::core::ffi::c_int,
                ::core::ffi::c_int,
            ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int,
            >;
        if imagic1 != EOF {
            inptr = inptr.wrapping_sub(1);
        }
        last_member = 1 as ::core::ffi::c_int;
        if imagic0 != EOF {
            write_buf(
                STDOUT_FILENO,
                &raw mut magic as *mut uch as voidp,
                1 as ::core::ffi::c_uint,
            );
        }
    }
    if method >= 0 as ::core::ffi::c_int {
        return method;
    }
    if part_nb == 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"\n%s: %s: not in gzip format\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
            &raw mut ifname as *mut ::core::ffi::c_char,
        );
        exit_code = ERROR;
        return -1 as ::core::ffi::c_int;
    } else {
        if magic[0usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            let mut inbyte: ::core::ffi::c_int = 0;
            inbyte = imagic1;
            while inbyte == 0 as ::core::ffi::c_int {
                inbyte = if inptr < insize {
                    let c2rust_fresh23 = inptr;
                    inptr = inptr.wrapping_add(1);
                    inbuf.0[c2rust_fresh23 as usize] as ::core::ffi::c_int
                } else {
                    fill_inbuf(1 as ::core::ffi::c_int)
                };
            }
            if inbyte == EOF {
                if verbose != 0 {
                    if quiet == 0 {
                        fprintf(
                            stderr,
                            b"\n%s: %s: decompression OK, trailing zero bytes ignored\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            program_name,
                            &raw mut ifname as *mut ::core::ffi::c_char,
                        );
                    }
                    if exit_code == OK {
                        exit_code = WARNING;
                    }
                }
                return -3 as ::core::ffi::c_int;
            }
        }
        if quiet == 0 {
            fprintf(
                stderr,
                b"\n%s: %s: decompression OK, trailing garbage ignored\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                program_name,
                &raw mut ifname as *mut ::core::ffi::c_char,
            );
        }
        if exit_code == OK {
            exit_code = WARNING;
        }
        return -2 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn do_list(mut method_0: ::core::ffi::c_int) {
    let mut crc: ulg = 0;
    static mut first_time: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    static mut methods: [*const ::core::ffi::c_char; 9] = [
        b"store\0".as_ptr() as *const ::core::ffi::c_char,
        b"compr\0".as_ptr() as *const ::core::ffi::c_char,
        b"pack \0".as_ptr() as *const ::core::ffi::c_char,
        b"lzh  \0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        b"defla\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut positive_off_t_width: ::core::ffi::c_int = ::core::mem::size_of::<off_t>()
        .wrapping_mul(8usize)
        .wrapping_sub(
            !((0 as ::core::ffi::c_int as off_t) < -1 as ::core::ffi::c_int as off_t)
                as ::core::ffi::c_int as usize,
        )
        .wrapping_mul(146usize)
        .wrapping_add(484usize)
        .wrapping_div(485usize)
        .wrapping_add(
            !((0 as ::core::ffi::c_int as off_t) < -1 as ::core::ffi::c_int as off_t)
                as ::core::ffi::c_int as usize,
        )
        .wrapping_sub(1usize)
        as ::core::ffi::c_int;
    if first_time != 0 && method_0 >= 0 as ::core::ffi::c_int {
        first_time = 0 as ::core::ffi::c_int;
        if verbose != 0 {
            printf(b"method  crc     date  time  \0".as_ptr() as *const ::core::ffi::c_char);
        }
        if quiet == 0 {
            printf(
                b"%*.*s %*.*s  ratio uncompressed_name\n\0".as_ptr() as *const ::core::ffi::c_char,
                positive_off_t_width,
                positive_off_t_width,
                b"compressed\0".as_ptr() as *const ::core::ffi::c_char,
                positive_off_t_width,
                positive_off_t_width,
                b"uncompressed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    } else if method_0 < 0 as ::core::ffi::c_int {
        if total_in <= 0 as off_t || total_out <= 0 as off_t {
            return;
        }
        if verbose != 0 {
            printf(b"                            \0".as_ptr() as *const ::core::ffi::c_char);
        }
        if verbose != 0 || quiet == 0 {
            printf(
                b"%*jd %*jd \0".as_ptr() as *const ::core::ffi::c_char,
                positive_off_t_width,
                total_in as intmax_t,
                positive_off_t_width,
                total_out as intmax_t,
            );
        }
        display_ratio(total_out - (total_in - header_bytes), total_out, stdout);
        printf(b" (totals)\n\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
    crc = !(0 as ::core::ffi::c_int) as ulg;
    if method_0 == DEFLATED && last_member == 0 {
        crc = unzip_crc;
    }
    if verbose != 0 {
        static mut month_abbr: [[::core::ffi::c_char; 4]; 12] = unsafe {
            [
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Jan\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Feb\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Mar\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Apr\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"May\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Jun\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Jul\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Aug\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Sep\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Oct\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Nov\0"),
                ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"Dec\0"),
            ]
        };
        let mut tm: *mut tm = localtime(&raw mut time_stamp.tv_sec);
        printf(
            b"%5s %08lx \0".as_ptr() as *const ::core::ffi::c_char,
            methods[method_0 as usize],
            crc,
        );
        if !tm.is_null() {
            printf(
                b"%s%3d %02d:%02d \0".as_ptr() as *const ::core::ffi::c_char,
                &raw const *(&raw const month_abbr as *const [::core::ffi::c_char; 4])
                    .offset((*tm).tm_mon as isize) as *const ::core::ffi::c_char,
                (*tm).tm_mday,
                (*tm).tm_hour,
                (*tm).tm_min,
            );
        } else {
            printf(b"??? ?? ??:?? \0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    printf(
        b"%*jd %*jd \0".as_ptr() as *const ::core::ffi::c_char,
        positive_off_t_width,
        bytes_in as intmax_t,
        positive_off_t_width,
        bytes_out as intmax_t,
    );
    if bytes_in == -1 as ::core::ffi::c_long {
        total_in = -1 as ::core::ffi::c_long as off_t;
        header_bytes = 0 as off_t;
        bytes_out = header_bytes;
        bytes_in = bytes_out;
    } else if total_in >= 0 as off_t {
        total_in += bytes_in;
    }
    if bytes_out == -1 as ::core::ffi::c_long {
        total_out = -1 as ::core::ffi::c_long as off_t;
        header_bytes = 0 as off_t;
        bytes_out = header_bytes;
        bytes_in = bytes_out;
    } else if total_out >= 0 as off_t {
        total_out += bytes_out;
    }
    display_ratio(bytes_out - (bytes_in - header_bytes), bytes_out, stdout);
    printf(
        b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ofname as *mut ::core::ffi::c_char,
    );
}
unsafe extern "C" fn shorten_name(mut name: *mut ::core::ffi::c_char) {
    let mut len: ::core::ffi::c_int = 0;
    let mut trunc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut plen: ::core::ffi::c_int = 0;
    let mut min_part: ::core::ffi::c_int = MIN_PART;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    len = strlen(name) as ::core::ffi::c_int;
    if decompress != 0 {
        if len <= 1 as ::core::ffi::c_int {
            gzip_error(b"name too short\0".as_ptr() as *const ::core::ffi::c_char);
        }
        *name.offset((len - 1 as ::core::ffi::c_int) as isize) = '\0' as ::core::ffi::c_char;
        return;
    }
    p = get_suffix(name);
    if p.is_null() {
        gzip_error(b"can't recover suffix\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    *p = '\0' as ::core::ffi::c_char;
    save_orig_name = 1 as ::core::ffi::c_int;
    if len > 4 as ::core::ffi::c_int
        && strcmp(
            p.offset(-(4 as ::core::ffi::c_int as isize)),
            b".tar\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        strcpy(
            p.offset(-(4 as ::core::ffi::c_int as isize)),
            b".tgz\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return;
    }
    loop {
        p = last_component(name);
        while *p != 0 {
            plen = strcspn(p, PART_SEP.as_ptr()) as ::core::ffi::c_int;
            p = p.offset(plen as isize);
            if plen > min_part {
                trunc = p.offset(-(1 as ::core::ffi::c_int as isize));
            }
            if *p != 0 {
                p = p.offset(1);
            }
        }
        if !(trunc.is_null() && {
            min_part -= 1;
            min_part != 0 as ::core::ffi::c_int
        }) {
            break;
        }
    }
    if !trunc.is_null() {
        loop {
            *trunc.offset(0isize) = *trunc.offset(1isize);
            let c2rust_fresh25 = trunc;
            trunc = trunc.offset(1);
            if *c2rust_fresh25 == 0 {
                break;
            }
        }
        trunc = trunc.offset(-1);
    } else {
        trunc = strrchr(
            name,
            ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b".\0")[0usize]
                as ::core::ffi::c_int,
        );
        if trunc.is_null() {
            gzip_error(b"internal error in shorten_name\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if *trunc.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            trunc = trunc.offset(-1);
        }
    }
    strcpy(trunc, z_suffix);
}
unsafe extern "C" fn check_ofname() -> ::core::ffi::c_int {
    if force == 0 {
        let mut ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        fprintf(
            stderr,
            b"%s: %s already exists;\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
            &raw mut ofname as *mut ::core::ffi::c_char,
        );
        if foreground != 0
            && (presume_input_tty as ::core::ffi::c_int != 0 || isatty(STDIN_FILENO) != 0)
        {
            fprintf(
                stderr,
                b" do you wish to overwrite (y or n)? \0".as_ptr() as *const ::core::ffi::c_char,
            );
            fflush(stderr);
            ok = yesno() as ::core::ffi::c_int;
        }
        if ok == 0 {
            fprintf(
                stderr,
                b"\tnot overwritten\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if exit_code == OK {
                exit_code = WARNING;
            }
            return ERROR;
        }
    }
    if xunlink(&raw mut ofname as *mut ::core::ffi::c_char) != 0 {
        progerror(&raw mut ofname as *mut ::core::ffi::c_char);
        return ERROR;
    }
    return OK;
}
unsafe extern "C" fn do_chown(
    mut fd: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut uid: uid_t,
    mut gid: gid_t,
) {
    fchown(fd, uid, gid);
}
unsafe extern "C" fn copy_stat(mut ifstat: *mut stat) {
    let mut mode: mode_t = (*ifstat).st_mode & S_IRWXUGO as mode_t;
    let mut r: ::core::ffi::c_int = 0;
    let mut restoring: bool = false;
    let mut timespec: [timespec; 2] = [timespec {
        tv_sec: 0,
        tv_nsec: 0,
    }; 2];
    timespec[0usize] = get_stat_atime(ifstat);
    timespec[1usize] = get_stat_mtime(ifstat);
    restoring = decompress != 0
        && 0 as __syscall_slong_t <= time_stamp.tv_nsec
        && !(timespec[1usize].tv_sec == time_stamp.tv_sec
            && timespec[1usize].tv_nsec == time_stamp.tv_nsec);
    if restoring {
        timespec[1usize] = time_stamp;
    }
    if fdutimens(
        ofd,
        &raw mut ofname as *mut ::core::ffi::c_char,
        &raw mut timespec as *mut timespec as *const timespec,
    ) == 0 as ::core::ffi::c_int
    {
        if restoring as ::core::ffi::c_int != 0 && (1 as ::core::ffi::c_int) < verbose {
            fprintf(
                stderr,
                b"%s: timestamp restored\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut ofname as *mut ::core::ffi::c_char,
            );
        }
    } else {
        if quiet == 0 {
            fprintf(
                stderr,
                b"%s: %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
                &raw mut ofname as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if exit_code == OK {
            exit_code = WARNING;
        }
    }
    do_chown(
        ofd,
        &raw mut ofname as *mut ::core::ffi::c_char,
        -1 as ::core::ffi::c_int as uid_t,
        (*ifstat).st_gid,
    );
    r = fchmod(ofd, mode);
    if r != 0 as ::core::ffi::c_int {
        if quiet == 0 {
            fprintf(
                stderr,
                b"%s: %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
                &raw mut ofname as *mut ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
        if exit_code == OK {
            exit_code = WARNING;
        }
    }
    do_chown(
        ofd,
        &raw mut ofname as *mut ::core::ffi::c_char,
        (*ifstat).st_uid,
        -1 as ::core::ffi::c_int as gid_t,
    );
}
unsafe extern "C" fn treat_dir(mut fd: ::core::ffi::c_int, mut dir: *mut ::core::ffi::c_char) {
    let mut dirp: *mut DIR = ::core::ptr::null_mut::<DIR>();
    let mut nbuf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut entries: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut entry: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut entrylen: size_t = 0;
    dirp = fdopendir(fd);
    if dirp.is_null() {
        progerror(dir);
        close(fd);
        return;
    }
    entries = streamsavedir(dirp, savedir_option::SAVEDIR_SORT_NONE);
    if entries.is_null() {
        progerror(dir);
    }
    if closedir(dirp) != 0 as ::core::ffi::c_int {
        progerror(dir);
    }
    if entries.is_null() {
        return;
    }
    entry = entries;
    while *entry != 0 {
        let mut len: size_t = strlen(dir);
        entrylen = strlen(entry);
        if !(strcmp(entry, b".\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
            || strcmp(entry, b"..\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int)
        {
            if len.wrapping_add(entrylen) < (MAX_PATH_LEN - 2 as ::core::ffi::c_int) as size_t {
                strcpy(&raw mut nbuf as *mut ::core::ffi::c_char, dir);
                if *last_component(&raw mut nbuf as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                    != 0
                    && !(nbuf[len.wrapping_sub(1 as size_t)] as ::core::ffi::c_int
                        == '/' as ::core::ffi::c_int)
                {
                    let c2rust_fresh26 = len;
                    len = len.wrapping_add(1);
                    nbuf[c2rust_fresh26] = '/' as ::core::ffi::c_char;
                }
                strcpy(
                    (&raw mut nbuf as *mut ::core::ffi::c_char).offset(len as isize),
                    entry,
                );
                treat_file(&raw mut nbuf as *mut ::core::ffi::c_char);
            } else {
                fprintf(
                    stderr,
                    b"%s: %s/%s: pathname too long\n\0".as_ptr() as *const ::core::ffi::c_char,
                    program_name,
                    dir,
                    entry,
                );
                exit_code = ERROR;
            }
        }
        entry = entry.offset(entrylen.wrapping_add(1 as size_t) as isize);
    }
    free(entries as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn install_signal_handlers() {
    let mut nsigs: ::core::ffi::c_int = ::core::mem::size_of::<[::core::ffi::c_int; 6]>()
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>())
        as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0;
    let mut act: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&raw mut caught_signals);
    i = 0 as ::core::ffi::c_int;
    while i < nsigs {
        sigaction(
            handled_sig[i as usize],
            ::core::ptr::null::<sigaction>(),
            &raw mut act,
        );
        if act.__sigaction_handler.sa_handler
            != ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            )
        {
            sigaddset(&raw mut caught_signals, handled_sig[i as usize]);
        }
        i += 1;
    }
    act.__sigaction_handler.sa_handler =
        Some(abort_gzip_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
    act.sa_mask = caught_signals as __sigset_t;
    act.sa_flags = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < nsigs {
        if sigismember(&raw mut caught_signals, handled_sig[i as usize]) != 0 {
            if i == 0 as ::core::ffi::c_int {
                foreground = 1 as ::core::ffi::c_int;
            }
            sigaction(
                handled_sig[i as usize],
                &raw mut act,
                ::core::ptr::null_mut::<sigaction>(),
            );
        }
        i += 1;
    }
}
unsafe extern "C" fn do_exit(mut exitcode: ::core::ffi::c_int) {
    static mut in_exit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if in_exit != 0 {
        exit(exitcode);
    }
    in_exit = 1 as ::core::ffi::c_int;
    free(env as *mut ::core::ffi::c_void);
    env = ::core::ptr::null_mut::<::core::ffi::c_char>();
    exit(exitcode);
}
unsafe extern "C" fn finish_out() {
    if fclose(stdout) != 0 as ::core::ffi::c_int {
        write_error();
    }
    do_exit(OK);
}
unsafe extern "C" fn remove_output_file(mut signals_already_blocked: bool) {
    let mut fd: ::core::ffi::c_int = 0;
    let mut oldset: sigset_t = sigset_t { __val: [0; 16] };
    if !signals_already_blocked {
        sigprocmask(SIG_BLOCK, &raw mut caught_signals, &raw mut oldset);
    }
    fd = ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const remove_ofname_fd);
    if 0 as ::core::ffi::c_int <= fd {
        let mut fname: [::core::ffi::c_char; 1024] = [0; 1024];
        ::core::ptr::write_volatile(&raw mut remove_ofname_fd, -1 as ::core::ffi::c_int);
        close(fd);
        volatile_strcpy(
            &raw mut fname as *mut ::core::ffi::c_char as *mut ::core::ffi::c_char,
            &raw mut remove_ofname as *mut ::core::ffi::c_char,
        );
        xunlink(&raw mut fname as *mut ::core::ffi::c_char);
    }
    if !signals_already_blocked {
        sigprocmask(
            SIG_SETMASK,
            &raw mut oldset,
            ::core::ptr::null_mut::<sigset_t>(),
        );
    }
}
#[export_name = "rboxc_gzip_finish_up_gzip"]
pub unsafe extern "C" fn finish_up_gzip(mut exitcode: ::core::ffi::c_int) {
    if 0 as ::core::ffi::c_int
        <= ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const remove_ofname_fd)
    {
        remove_output_file(r#false != 0);
    }
    do_exit(exitcode);
}
#[export_name = "rboxc_gzip_abort_gzip"]
pub unsafe extern "C" fn abort_gzip() {
    finish_up_gzip(ERROR);
}
unsafe extern "C" fn abort_gzip_signal(mut sig: ::core::ffi::c_int) {
    remove_output_file(r#true != 0);
    signal(sig, SIG_DFL);
    raise(sig);
}
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const HAVE_WORKING_O_NOFOLLOW: ::core::ffi::c_int = 1 as ::core::ffi::c_int;

const RBOXC_GUNZIP_VERSION: &[u8] = &[103,117,110,122,105,112,32,40,103,122,105,112,41,32,49,46,49,52,10,67,111,112,121,114,105,103,104,116,32,40,67,41,32,50,48,50,53,32,70,114,101,101,32,83,111,102,116,119,97,114,101,32,70,111,117,110,100,97,116,105,111,110,44,32,73,110,99,46,10,84,104,105,115,32,105,115,32,102,114,101,101,32,115,111,102,116,119,97,114,101,46,32,32,89,111,117,32,109,97,121,32,114,101,100,105,115,116,114,105,98,117,116,101,32,99,111,112,105,101,115,32,111,102,32,105,116,32,117,110,100,101,114,32,116,104,101,32,116,101,114,109,115,32,111,102,10,116,104,101,32,71,78,85,32,71,101,110,101,114,97,108,32,80,117,98,108,105,99,32,76,105,99,101,110,115,101,32,60,104,116,116,112,115,58,47,47,119,119,119,46,103,110,117,46,111,114,103,47,108,105,99,101,110,115,101,115,47,103,112,108,46,104,116,109,108,62,46,10,84,104,101,114,101,32,105,115,32,78,79,32,87,65,82,82,65,78,84,89,44,32,116,111,32,116,104,101,32,101,120,116,101,110,116,32,112,101,114,109,105,116,116,101,100,32,98,121,32,108,97,119,46,10,10,87,114,105,116,116,101,110,32,98,121,32,80,97,117,108,32,69,103,103,101,114,116,46,0];

const RBOXC_GUNZIP_USAGE: &[u8] = &[32,91,79,80,84,73,79,78,93,46,46,46,32,91,70,73,76,69,93,46,46,46,10,85,110,99,111,109,112,114,101,115,115,32,70,73,76,69,115,32,40,98,121,32,100,101,102,97,117,108,116,44,32,105,110,45,112,108,97,99,101,41,46,10,10,77,97,110,100,97,116,111,114,121,32,97,114,103,117,109,101,110,116,115,32,116,111,32,108,111,110,103,32,111,112,116,105,111,110,115,32,97,114,101,32,109,97,110,100,97,116,111,114,121,32,102,111,114,32,115,104,111,114,116,32,111,112,116,105,111,110,115,32,116,111,111,46,10,10,32,32,45,99,44,32,45,45,115,116,100,111,117,116,32,32,32,32,32,32,119,114,105,116,101,32,111,110,32,115,116,97,110,100,97,114,100,32,111,117,116,112,117,116,44,32,107,101,101,112,32,111,114,105,103,105,110,97,108,32,102,105,108,101,115,32,117,110,99,104,97,110,103,101,100,10,32,32,45,102,44,32,45,45,102,111,114,99,101,32,32,32,32,32,32,32,102,111,114,99,101,32,111,118,101,114,119,114,105,116,101,32,111,102,32,111,117,116,112,117,116,32,102,105,108,101,32,97,110,100,32,99,111,109,112,114,101,115,115,32,108,105,110,107,115,10,32,32,45,107,44,32,45,45,107,101,101,112,32,32,32,32,32,32,32,32,107,101,101,112,32,40,100,111,110,39,116,32,100,101,108,101,116,101,41,32,105,110,112,117,116,32,102,105,108,101,115,10,32,32,45,108,44,32,45,45,108,105,115,116,32,32,32,32,32,32,32,32,108,105,115,116,32,99,111,109,112,114,101,115,115,101,100,32,102,105,108,101,32,99,111,110,116,101,110,116,115,10,32,32,45,110,44,32,45,45,110,111,45,110,97,109,101,32,32,32,32,32,100,111,32,110,111,116,32,115,97,118,101,32,111,114,32,114,101,115,116,111,114,101,32,116,104,101,32,111,114,105,103,105,110,97,108,32,110,97,109,101,32,97,110,100,32,116,105,109,101,115,116,97,109,112,10,32,32,45,78,44,32,45,45,110,97,109,101,32,32,32,32,32,32,32,32,115,97,118,101,32,111,114,32,114,101,115,116,111,114,101,32,116,104,101,32,111,114,105,103,105,110,97,108,32,110,97,109,101,32,97,110,100,32,116,105,109,101,115,116,97,109,112,10,32,32,45,113,44,32,45,45,113,117,105,101,116,32,32,32,32,32,32,32,115,117,112,112,114,101,115,115,32,97,108,108,32,119,97,114,110,105,110,103,115,10,32,32,45,114,44,32,45,45,114,101,99,117,114,115,105,118,101,32,32,32,111,112,101,114,97,116,101,32,114,101,99,117,114,115,105,118,101,108,121,32,111,110,32,100,105,114,101,99,116,111,114,105,101,115,10,32,32,45,83,44,32,45,45,115,117,102,102,105,120,61,83,85,70,32,32,117,115,101,32,115,117,102,102,105,120,32,83,85,70,32,111,110,32,99,111,109,112,114,101,115,115,101,100,32,102,105,108,101,115,10,32,32,32,32,32,32,45,45,115,121,110,99,104,114,111,110,111,117,115,32,115,121,110,99,104,114,111,110,111,117,115,32,111,117,116,112,117,116,32,40,115,97,102,101,114,32,105,102,32,115,121,115,116,101,109,32,99,114,97,115,104,101,115,44,32,98,117,116,32,115,108,111,119,101,114,41,10,32,32,45,116,44,32,45,45,116,101,115,116,32,32,32,32,32,32,32,32,116,101,115,116,32,99,111,109,112,114,101,115,115,101,100,32,102,105,108,101,32,105,110,116,101,103,114,105,116,121,10,32,32,45,118,44,32,45,45,118,101,114,98,111,115,101,32,32,32,32,32,118,101,114,98,111,115,101,32,109,111,100,101,10,32,32,32,32,32,32,45,45,104,101,108,112,32,32,32,32,32,32,32,32,100,105,115,112,108,97,121,32,116,104,105,115,32,104,101,108,112,32,97,110,100,32,101,120,105,116,10,32,32,32,32,32,32,45,45,118,101,114,115,105,111,110,32,32,32,32,32,100,105,115,112,108,97,121,32,118,101,114,115,105,111,110,32,105,110,102,111,114,109,97,116,105,111,110,32,97,110,100,32,101,120,105,116,10,10,87,105,116,104,32,110,111,32,70,73,76,69,44,32,111,114,32,119,104,101,110,32,70,73,76,69,32,105,115,32,45,44,32,114,101,97,100,32,115,116,97,110,100,97,114,100,32,105,110,112,117,116,46,10,10,82,101,112,111,114,116,32,98,117,103,115,32,116,111,32,60,98,117,103,45,103,122,105,112,64,103,110,117,46,111,114,103,62,46,0];

const RBOXC_ZCAT_VERSION: &[u8] = &[122,99,97,116,32,40,103,122,105,112,41,32,49,46,49,52,10,67,111,112,121,114,105,103,104,116,32,40,67,41,32,50,48,50,53,32,70,114,101,101,32,83,111,102,116,119,97,114,101,32,70,111,117,110,100,97,116,105,111,110,44,32,73,110,99,46,10,84,104,105,115,32,105,115,32,102,114,101,101,32,115,111,102,116,119,97,114,101,46,32,32,89,111,117,32,109,97,121,32,114,101,100,105,115,116,114,105,98,117,116,101,32,99,111,112,105,101,115,32,111,102,32,105,116,32,117,110,100,101,114,32,116,104,101,32,116,101,114,109,115,32,111,102,10,116,104,101,32,71,78,85,32,71,101,110,101,114,97,108,32,80,117,98,108,105,99,32,76,105,99,101,110,115,101,32,60,104,116,116,112,115,58,47,47,119,119,119,46,103,110,117,46,111,114,103,47,108,105,99,101,110,115,101,115,47,103,112,108,46,104,116,109,108,62,46,10,84,104,101,114,101,32,105,115,32,78,79,32,87,65,82,82,65,78,84,89,44,32,116,111,32,116,104,101,32,101,120,116,101,110,116,32,112,101,114,109,105,116,116,101,100,32,98,121,32,108,97,119,46,10,10,87,114,105,116,116,101,110,32,98,121,32,80,97,117,108,32,69,103,103,101,114,116,46,0];

const RBOXC_ZCAT_USAGE: &[u8] = &[32,91,79,80,84,73,79,78,93,46,46,46,32,91,70,73,76,69,93,46,46,46,10,85,110,99,111,109,112,114,101,115,115,32,70,73,76,69,115,32,116,111,32,115,116,97,110,100,97,114,100,32,111,117,116,112,117,116,46,10,10,32,32,45,102,44,32,45,45,102,111,114,99,101,32,32,32,32,32,32,32,102,111,114,99,101,59,32,114,101,97,100,32,99,111,109,112,114,101,115,115,101,100,32,100,97,116,97,32,101,118,101,110,32,102,114,111,109,32,97,32,116,101,114,109,105,110,97,108,10,32,32,45,108,44,32,45,45,108,105,115,116,32,32,32,32,32,32,32,32,108,105,115,116,32,99,111,109,112,114,101,115,115,101,100,32,102,105,108,101,32,99,111,110,116,101,110,116,115,10,32,32,45,113,44,32,45,45,113,117,105,101,116,32,32,32,32,32,32,32,115,117,112,112,114,101,115,115,32,97,108,108,32,119,97,114,110,105,110,103,115,10,32,32,45,114,44,32,45,45,114,101,99,117,114,115,105,118,101,32,32,32,111,112,101,114,97,116,101,32,114,101,99,117,114,115,105,118,101,108,121,32,111,110,32,100,105,114,101,99,116,111,114,105,101,115,10,32,32,45,83,44,32,45,45,115,117,102,102,105,120,61,83,85,70,32,32,117,115,101,32,115,117,102,102,105,120,32,83,85,70,32,111,110,32,99,111,109,112,114,101,115,115,101,100,32,102,105,108,101,115,10,32,32,32,32,32,32,45,45,115,121,110,99,104,114,111,110,111,117,115,32,115,121,110,99,104,114,111,110,111,117,115,32,111,117,116,112,117,116,32,40,115,97,102,101,114,32,105,102,32,115,121,115,116,101,109,32,99,114,97,115,104,101,115,44,32,98,117,116,32,115,108,111,119,101,114,41,10,32,32,45,116,44,32,45,45,116,101,115,116,32,32,32,32,32,32,32,32,116,101,115,116,32,99,111,109,112,114,101,115,115,101,100,32,102,105,108,101,32,105,110,116,101,103,114,105,116,121,10,32,32,45,118,44,32,45,45,118,101,114,98,111,115,101,32,32,32,32,32,118,101,114,98,111,115,101,32,109,111,100,101,10,32,32,32,32,32,32,45,45,104,101,108,112,32,32,32,32,32,32,32,32,100,105,115,112,108,97,121,32,116,104,105,115,32,104,101,108,112,32,97,110,100,32,101,120,105,116,10,32,32,32,32,32,32,45,45,118,101,114,115,105,111,110,32,32,32,32,32,100,105,115,112,108,97,121,32,118,101,114,115,105,111,110,32,105,110,102,111,114,109,97,116,105,111,110,32,97,110,100,32,101,120,105,116,10,10,87,105,116,104,32,110,111,32,70,73,76,69,44,32,111,114,32,119,104,101,110,32,70,73,76,69,32,105,115,32,45,44,32,114,101,97,100,32,115,116,97,110,100,97,114,100,32,105,110,112,117,116,46,10,10,82,101,112,111,114,116,32,98,117,103,115,32,116,111,32,60,98,117,103,45,103,122,105,112,64,103,110,117,46,111,114,103,62,46,0];

// SPDX-License-Identifier: GPL-3.0-or-later
// Adapt the pinned GNU shell aliases' first-argument help/version checks,
// then insert their options before internal dispatch to the Rust gzip entry.
static mut RBOXC_GZIP_ALIAS_ARGV: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut();
extern "C" fn rboxc_gzip_release_alias() {
    unsafe {
    let saved = *libc::__errno_location();
    libc::free(RBOXC_GZIP_ALIAS_ARGV.cast());
    RBOXC_GZIP_ALIAS_ARGV = ::core::ptr::null_mut();
    *libc::__errno_location() = saved;
    }
}
unsafe fn rboxc_gzip_alias(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
    zcat: bool) -> ::core::ffi::c_int {
    if argc > 1 {
        let first = *argv.add(1);
        let version = libc::strcmp(first, b"--version\0".as_ptr().cast()) == 0;
        let help = libc::strcmp(first, b"--help\0".as_ptr().cast()) == 0;
        if version || help {
            let text = if version {
                if zcat { RBOXC_ZCAT_VERSION } else { RBOXC_GUNZIP_VERSION }
            } else {
                if zcat { RBOXC_ZCAT_USAGE } else { RBOXC_GUNZIP_USAGE }
            };
            let output = stdout.cast::<libc::FILE>();
            let status = if version {
                libc::fprintf(output, b"%s\n\0".as_ptr().cast(), text.as_ptr())
            } else {
                libc::fprintf(output, b"Usage: %s%s\n\0".as_ptr().cast(), *argv, text.as_ptr())
            };
            let flushed = libc::fflush(output);
            return if status < 0 || flushed != 0 { 1 } else { 0 };
        }
    }
    let Some(count) = argc.checked_add(1) else { return 1; };
    let Some(bytes) = (count as usize + 1).checked_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()) else { return 1; };
    let adjusted = libc::malloc(bytes).cast::<*mut ::core::ffi::c_char>();
    if adjusted.is_null() { return 1; }
    RBOXC_GZIP_ALIAS_ARGV = adjusted;
    libc::atexit(rboxc_gzip_release_alias);
    *adjusted = b"gzip\0".as_ptr().cast_mut().cast();
    *adjusted.add(1) = if zcat { b"-cd\0".as_ptr() } else { b"-d\0".as_ptr() }.cast_mut().cast();
    for index in 1..argc as usize { *adjusted.add(index+1) = *argv.add(index); }
    *adjusted.add(count as usize) = ::core::ptr::null_mut();
    let status = single_binary_main_gzip(count, adjusted);
    rboxc_gzip_release_alias();
    status
}
pub unsafe extern "C" fn single_binary_main_gunzip(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_gzip_alias(argc, argv, false)
}
pub unsafe extern "C" fn single_binary_main_uncompress(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_gzip_alias(argc, argv, false)
}
pub unsafe extern "C" fn single_binary_main_zcat(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_gzip_alias(argc, argv, true)
}

