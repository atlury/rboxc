// Generated from pinned GNU Diffutils 3.12 by scripts/translate-diffutils.py.
// Source SHA-256: 62e1f9753eb2be18685aca5995ab418038b269442c54c56077d8ef56427bd1c4
/* GNU sdiff - side-by-side merge of file differences

   Copyright (C) 1992-1996, 1998, 2001-2002, 2004, 2006-2007, 2009-2013,
   2015-2025 Free Software Foundation, Inc.

   This file is part of GNU DIFF.

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
use ::libc;
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn raise(__sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn mkstemp(__template: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strtoimax(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> intmax_t;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn rawmemchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn stpcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn free(_: *mut ::core::ffi::c_void);
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
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fputs_unlocked(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread_unlocked(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __uflow(_: *mut FILE) -> ::core::ffi::c_int;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_c_stack_action"]
    fn c_stack_action(
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_last_component"]
    fn last_component(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_base_len"]
    fn base_len(filename: *const ::core::ffi::c_char) -> size_t;
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
    #[link_name = "rboxc_diffutils_xpalloc"]
    fn xpalloc(
        pa: *mut ::core::ffi::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xstdopen"]
    fn xstdopen();
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
pub type pid_t = __pid_t;
pub type ptrdiff_t = isize;
pub type size_t = usize;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
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
pub type intmax_t = ::libc::intmax_t;
pub type idx_t = ptrdiff_t;
pub type lin = ptrdiff_t;
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
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
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
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const SDIFF_BUFSIZE: Self = Self(65536);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct line_filter {
    pub infile: *mut FILE,
    pub bufpos: *mut ::core::ffi::c_char,
    pub buffer: *mut ::core::ffi::c_char,
    pub buflim: *mut ::core::ffi::c_char,
}
pub type sighandler = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const NUM_SIGS: Self = Self(7);
    pub const handler_index_of_SIGINT: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_13 {
    pub const DIFF_PROGRAM_OPTION: Self = Self(128);
    pub const HELP_OPTION: Self = Self(129);
    pub const STRIP_TRAILING_CR_OPTION: Self = Self(130);
    pub const TABSIZE_OPTION: Self = Self(131);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIGXFSZ: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const SIGXCPU: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTRDIFF_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const DEFAULT_DIFF_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"diff\0") };
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const LIN_MAX: ::core::ffi::c_long = PTRDIFF_MAX;
pub const _IO_EOF_SEEN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const P_tmpdir: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"/tmp\0") };
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
unsafe extern "C" fn getchar_unlocked() -> ::core::ffi::c_int {
    return if ((*stdin)._IO_read_ptr >= (*stdin)._IO_read_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __uflow(stdin)
    } else {
        let c2rust_fresh1 = (*stdin)._IO_read_ptr;
        (*stdin)._IO_read_ptr = (*stdin)._IO_read_ptr.offset(1);
        *(c2rust_fresh1 as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int
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
#[inline]
unsafe extern "C" fn c_isspace(mut c: ::core::ffi::c_int) -> bool {
    match c {
        32 | 9 | 10 | 11 | 12 | 13 => return r#true != 0,
        _ => return r#false != 0,
    };
}
static mut PROGRAM_NAME: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"sdiff\0") };
static mut editor_program: *const ::core::ffi::c_char = DEFAULT_EDITOR_PROGRAM.as_ptr();
static mut diffargv: *mut *const ::core::ffi::c_char =
    ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
static mut tmpname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut tmp: *mut FILE = ::core::ptr::null_mut::<FILE>();
static mut diffpid: pid_t = 0;
static mut sigs: [::core::ffi::c_int; 7] =
    [SIGHUP, SIGQUIT, SIGTERM, SIGXCPU, SIGXFSZ, SIGPIPE, SIGINT];
static mut initial_action: [sigaction; 7] = [sigaction {
    __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
}; 7];
unsafe extern "C" fn initial_handler(mut i: ::core::ffi::c_int) -> sighandler {
    return initial_action[i as usize].__sigaction_handler.sa_handler;
}
static mut output: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut suppress_common_lines: bool = false;
static mut shortopts: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"abBdEHiI:lo:stvw:WZ\0")
};
static mut longopts: [option; 21] = [
    option {
        name: b"diff-program\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_13::DIFF_PROGRAM_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"expand-tabs\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 't' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_13::HELP_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-all-space\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'W' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-blank-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'B' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-case\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'i' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-matching-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'I' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-space-change\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-tab-expansion\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'E' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-trailing-space\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'Z' as ::core::ffi::c_int,
    },
    option {
        name: b"left-column\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"minimal\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"output\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'o' as ::core::ffi::c_int,
    },
    option {
        name: b"speed-large-files\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'H' as ::core::ffi::c_int,
    },
    option {
        name: b"strip-trailing-cr\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_13::STRIP_TRAILING_CR_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"suppress-common-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"tabsize\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_13::TABSIZE_OPTION.0 as ::core::ffi::c_int,
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
        name: b"width\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'w' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        fatal(b"write failed\0".as_ptr() as *const ::core::ffi::c_char);
    } else if fclose(stdout) != 0 as ::core::ffi::c_int {
        perror_fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"standard output\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
}
static mut option_help_msgid: [*const ::core::ffi::c_char; 26] = [
    b"-o, --output=FILE            operate interactively, sending output to FILE\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-i, --ignore-case            consider upper- and lower-case to be the same\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-E, --ignore-tab-expansion   ignore changes due to tab expansion\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-Z, --ignore-trailing-space  ignore white space at line end\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-b, --ignore-space-change    ignore changes in the amount of white space\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-W, --ignore-all-space       ignore all white space\0".as_ptr() as *const ::core::ffi::c_char,
    b"-B, --ignore-blank-lines     ignore changes whose lines are all blank\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-I, --ignore-matching-lines=RE  ignore changes all whose lines match RE\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --strip-trailing-cr      strip trailing carriage return on input\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-a, --text                   treat all files as text\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-w, --width=NUM              output at most NUM (default 130) print columns\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-l, --left-column            output only the left column of common lines\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-s, --suppress-common-lines  do not output common lines\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-t, --expand-tabs            expand tabs to spaces in output\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --tabsize=NUM            tab stops at every NUM (default 8) print columns\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-d, --minimal                try hard to find a smaller set of changes\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-H, --speed-large-files      assume large files, many scattered small changes\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --diff-program=PROGRAM   use PROGRAM to compare files\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"    --help                   display this help and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-v, --version                output version information and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... FILE1 FILE2\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        squote(0 as ::core::ffi::c_int, program_name),
    );
    printf(
        b"%s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Side-by-side merge of differences between FILE1 and FILE2.\0".as_ptr()
                as *const ::core::ffi::c_char,
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
    printf(
        b"\n%s\n%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"If a FILE is '-', read standard input.\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    emit_bug_reporting_address();
}
unsafe extern "C" fn cleanup(mut signo: ::core::ffi::c_int) {
    if (0 as ::core::ffi::c_int) < ::core::ptr::read_volatile::<pid_t>(&raw const diffpid) {
        kill(
            ::core::ptr::read_volatile::<pid_t>(&raw const diffpid),
            SIGPIPE,
        );
    }
    if !::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname).is_null() {
        unlink(::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(
            &raw const tmpname,
        ));
    }
}
unsafe extern "C" fn exiterr() {
    cleanup(0 as ::core::ffi::c_int);
    untrapsig(0 as ::core::ffi::c_int);
    checksigs();
    exit(C2Rust_Unnamed_10::EXIT_TROUBLE.0 as ::core::ffi::c_int);
}
unsafe extern "C" fn fatal(mut msgid: *const ::core::ffi::c_char) {
    if 0 != 0 {
        error(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                msgid,
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
    exiterr();
}
unsafe extern "C" fn perror_fatal(mut msg: *const ::core::ffi::c_char) {
    let mut e: ::core::ffi::c_int = *__errno_location();
    checksigs();
    if 0 != 0 {
        error(
            0 as ::core::ffi::c_int,
            e,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            msg,
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
                e,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                msg,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
    exiterr();
}
unsafe extern "C" fn check_child_status(
    mut werrno: ::core::ffi::c_int,
    mut wstatus: ::core::ffi::c_int,
    mut max_ok_status: ::core::ffi::c_int,
    mut subsidiary_program: *const ::core::ffi::c_char,
) {
    let mut status: ::core::ffi::c_int =
        if werrno == 0 && wstatus & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (wstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
        } else {
            INT_MAX
        };
    if max_ok_status < status {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
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
                quote(subsidiary_program),
                status,
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
                    quote(subsidiary_program),
                    status,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        exiterr();
    }
}
unsafe extern "C" fn ck_fopen(
    mut fname: *const ::core::ffi::c_char,
    mut r#type: *const ::core::ffi::c_char,
) -> *mut FILE {
    let mut r: *mut FILE = fopen(fname, r#type);
    if r.is_null() {
        perror_fatal(squote(0 as ::core::ffi::c_int, fname));
    }
    return r;
}
unsafe extern "C" fn ck_fclose(mut f: *mut FILE) {
    if fclose(f) != 0 {
        perror_fatal(b"fclose\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
unsafe extern "C" fn ck_fread(
    mut buf: *mut ::core::ffi::c_char,
    mut size: idx_t,
    mut f: *mut FILE,
) -> idx_t {
    let mut r: idx_t = (if 0 != 0
        && 0 != 0
        && ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(size as size_t) <= 8 as size_t
        && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
    {
        ({
            let mut __ptr: *mut ::core::ffi::c_char = buf;
            let mut __stream: *mut FILE = f;
            let mut __cnt: size_t = 0;
            __cnt = ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(size as size_t);
            while __cnt > 0 as size_t {
                let mut __c: ::core::ffi::c_int = getc_unlocked(__stream);
                if __c == EOF {
                    break;
                }
                let c2rust_fresh5 = __ptr;
                __ptr = __ptr.offset(1);
                *c2rust_fresh5 = __c as ::core::ffi::c_char;
                __cnt = __cnt.wrapping_sub(1);
            }
            ::core::mem::size_of::<::core::ffi::c_char>()
                .wrapping_mul(size as size_t)
                .wrapping_sub(__cnt)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>())
        })
    } else if 0 != 0 && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
        || 0 != 0 && size as size_t == 0 as size_t
    {
        0 as ::core::ffi::c_int as size_t
    } else {
        fread_unlocked(
            buf as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>(),
            size as size_t,
            f,
        )
    }) as idx_t;
    if r == 0 as idx_t && ferror_unlocked(f) != 0 {
        perror_fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"read failed\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
    return r;
}
unsafe extern "C" fn ck_fwrite(
    mut buf: *const ::core::ffi::c_char,
    mut size: idx_t,
    mut f: *mut FILE,
) {
    if if 0 != 0
        && 0 != 0
        && ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(size as size_t) <= 8 as size_t
        && ::core::mem::size_of::<::core::ffi::c_char>() != 0 as size_t
    {
        ({
            let mut __ptr: *const ::core::ffi::c_char = buf;
            let mut __stream: *mut FILE = f;
            let mut __cnt: size_t = 0;
            __cnt = ::core::mem::size_of::<::core::ffi::c_char>().wrapping_mul(size as size_t);
            while __cnt > 0 as size_t {
                let c2rust_fresh4 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*c2rust_fresh4 as ::core::ffi::c_int, __stream) == EOF {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
            }
            ::core::mem::size_of::<::core::ffi::c_char>()
                .wrapping_mul(size as size_t)
                .wrapping_sub(__cnt)
                .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>())
        })
    } else if 0 != 0 && ::core::mem::size_of::<::core::ffi::c_char>() == 0 as size_t
        || 0 != 0 && size as size_t == 0 as size_t
    {
        0 as ::core::ffi::c_int as size_t
    } else {
        fwrite_unlocked(
            buf as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_char>(),
            size as size_t,
            f,
        )
    } != size as size_t
    {
        perror_fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"write failed\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
}
unsafe extern "C" fn ck_fflush(mut f: *mut FILE) {
    if fflush_unlocked(f) != 0 as ::core::ffi::c_int {
        perror_fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"write failed\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
}
unsafe extern "C" fn expand_name(
    mut name: *mut ::core::ffi::c_char,
    mut is_dir: bool,
    mut other_name: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if strcmp(name, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        fatal(b"cannot interactively merge standard input\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !is_dir {
        return name;
    } else {
        let mut base: *const ::core::ffi::c_char = last_component(other_name);
        let mut namelen: idx_t = strlen(name) as idx_t;
        let mut baselen: idx_t = base_len(base) as idx_t;
        let mut insert_slash: bool = *last_component(name) as ::core::ffi::c_int != 0
            && *name.offset((namelen - 1 as idx_t) as isize) as ::core::ffi::c_int
                != '/' as ::core::ffi::c_int;
        let mut r: *mut ::core::ffi::c_char =
            ximalloc(namelen + insert_slash as idx_t + baselen + 1 as idx_t)
                as *mut ::core::ffi::c_char;
        let mut p: *mut ::core::ffi::c_char = stpcpy(r, name);
        *p = '/' as ::core::ffi::c_char;
        p = mempcpy(
            p.offset(insert_slash as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
            base as *const ::core::ffi::c_void,
            baselen as size_t,
        ) as *mut ::core::ffi::c_char;
        *p = '\0' as ::core::ffi::c_char;
        return r;
    };
}
unsafe extern "C" fn lf_init(mut lf: *mut line_filter, mut infile: *mut FILE) {
    (*lf).infile = infile;
    (*lf).buflim = ximalloc(
        (C2Rust_Unnamed_11::SDIFF_BUFSIZE.0 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as idx_t,
    ) as *mut ::core::ffi::c_char;
    (*lf).buffer = (*lf).buflim;
    (*lf).bufpos = (*lf).buffer;
    *(*lf).buflim.offset(0isize) = '\n' as ::core::ffi::c_char;
}
unsafe extern "C" fn lf_refill(mut lf: *mut line_filter) -> idx_t {
    let mut s: idx_t = ck_fread(
        (*lf).buffer,
        C2Rust_Unnamed_11::SDIFF_BUFSIZE.0 as ::core::ffi::c_int as idx_t,
        (*lf).infile,
    );
    (*lf).bufpos = (*lf).buffer;
    (*lf).buflim = (*lf).buffer.offset(s as isize);
    *(*lf).buflim.offset(0isize) = '\n' as ::core::ffi::c_char;
    checksigs();
    return s;
}
unsafe extern "C" fn lf_copy(mut lf: *mut line_filter, mut lines: lin, mut outfile: *mut FILE) {
    let mut start: *mut ::core::ffi::c_char = (*lf).bufpos;
    while lines != 0 {
        (*lf).bufpos = rawmemchr(
            (*lf).bufpos as *const ::core::ffi::c_void,
            '\n' as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
        if (*lf).bufpos == (*lf).buflim {
            ck_fwrite(start, (*lf).buflim.offset_from(start), outfile);
            if lf_refill(lf) == 0 {
                return;
            }
            start = (*lf).bufpos;
        } else {
            lines -= 1;
            (*lf).bufpos = (*lf).bufpos.offset(1);
        }
    }
    ck_fwrite(start, (*lf).bufpos.offset_from(start), outfile);
}
unsafe extern "C" fn lf_skip(mut lf: *mut line_filter, mut lines: lin) {
    while lines != 0 {
        (*lf).bufpos = rawmemchr(
            (*lf).bufpos as *const ::core::ffi::c_void,
            '\n' as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
        if (*lf).bufpos == (*lf).buflim {
            if lf_refill(lf) == 0 {
                break;
            }
        } else {
            lines -= 1;
            (*lf).bufpos = (*lf).bufpos.offset(1);
        }
    }
}
unsafe extern "C" fn lf_snarf(
    mut lf: *mut line_filter,
    mut buffer: *mut ::core::ffi::c_char,
    mut bufsize: idx_t,
) -> ::core::ffi::c_int {
    loop {
        let mut start: *mut ::core::ffi::c_char = (*lf).bufpos;
        let mut next: *mut ::core::ffi::c_char = rawmemchr(
            start as *const ::core::ffi::c_void,
            '\n' as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
        let mut s: idx_t = next.offset_from(start);
        if bufsize <= s {
            return 0 as ::core::ffi::c_int;
        }
        buffer = mempcpy(
            buffer as *mut ::core::ffi::c_void,
            start as *const ::core::ffi::c_void,
            s as size_t,
        ) as *mut ::core::ffi::c_char;
        bufsize -= s;
        if next < (*lf).buflim {
            *buffer = '\0' as ::core::ffi::c_char;
            (*lf).bufpos = next.offset(1 as ::core::ffi::c_int as isize);
            return 1 as ::core::ffi::c_int;
        }
        if lf_refill(lf) == 0 {
            return if s != 0 { 0 as ::core::ffi::c_int } else { EOF };
        }
    }
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
pub unsafe extern "C" fn single_binary_main_sdiff(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ::core::ptr::write_volatile(
        &raw mut exit_failure,
        C2Rust_Unnamed_10::EXIT_TROUBLE.0 as ::core::ffi::c_int,
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
    c_stack_action(Some(
        cleanup as unsafe extern "C" fn(::core::ffi::c_int) -> (),
    ));
    xstdopen();
    let mut prog: *const ::core::ffi::c_char =
        getenv(b"EDITOR\0".as_ptr() as *const ::core::ffi::c_char);
    if !prog.is_null() {
        editor_program = prog;
    }
    diffarg(DEFAULT_DIFF_PROGRAM.as_ptr());
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            &raw const shortopts as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if 0 as ::core::ffi::c_int > c {
            break;
        }
        match c {
            97 => {
                diffarg(b"-a\0".as_ptr() as *const ::core::ffi::c_char);
            }
            98 => {
                diffarg(b"-b\0".as_ptr() as *const ::core::ffi::c_char);
            }
            66 => {
                diffarg(b"-B\0".as_ptr() as *const ::core::ffi::c_char);
            }
            100 => {
                diffarg(b"-d\0".as_ptr() as *const ::core::ffi::c_char);
            }
            69 => {
                diffarg(b"-E\0".as_ptr() as *const ::core::ffi::c_char);
            }
            72 => {
                diffarg(b"-H\0".as_ptr() as *const ::core::ffi::c_char);
            }
            105 => {
                diffarg(b"-i\0".as_ptr() as *const ::core::ffi::c_char);
            }
            73 => {
                diffarg(b"-I\0".as_ptr() as *const ::core::ffi::c_char);
                diffarg(optarg);
            }
            108 => {
                diffarg(b"--left-column\0".as_ptr() as *const ::core::ffi::c_char);
            }
            111 => {
                output = optarg;
            }
            115 => {
                suppress_common_lines = r#true != 0;
            }
            116 => {
                diffarg(b"-t\0".as_ptr() as *const ::core::ffi::c_char);
            }
            118 => {
                version_etc(
                    stdout,
                    &raw const PROGRAM_NAME as *const ::core::ffi::c_char,
                    PACKAGE_NAME.as_ptr(),
                    Version,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Thomas Lord\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    nullptr,
                );
                check_stdout();
                return EXIT_SUCCESS;
            }
            119 => {
                diffarg(b"-W\0".as_ptr() as *const ::core::ffi::c_char);
                diffarg(optarg);
            }
            87 => {
                diffarg(b"-w\0".as_ptr() as *const ::core::ffi::c_char);
            }
            90 => {
                diffarg(b"-Z\0".as_ptr() as *const ::core::ffi::c_char);
            }
            128 => {
                *diffargv.offset(0isize) = optarg;
            }
            129 => {
                usage();
                check_stdout();
                return EXIT_SUCCESS;
            }
            130 => {
                diffarg(b"--strip-trailing-cr\0".as_ptr() as *const ::core::ffi::c_char);
            }
            131 => {
                diffarg(b"--tabsize\0".as_ptr() as *const ::core::ffi::c_char);
                diffarg(optarg);
            }
            _ => {
                try_help(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
    if argc - optind != 2 as ::core::ffi::c_int {
        if argc - optind < 2 as ::core::ffi::c_int {
            try_help(
                b"missing operand after %s\0".as_ptr() as *const ::core::ffi::c_char,
                quote(*argv.offset((argc - 1 as ::core::ffi::c_int) as isize)),
            );
        } else {
            try_help(
                b"extra operand %s\0".as_ptr() as *const ::core::ffi::c_char,
                quote(*argv.offset((optind + 2 as ::core::ffi::c_int) as isize)),
            );
        }
    }
    if output.is_null() {
        if suppress_common_lines {
            diffarg(b"--suppress-common-lines\0".as_ptr() as *const ::core::ffi::c_char);
        }
        diffarg(b"-y\0".as_ptr() as *const ::core::ffi::c_char);
        diffarg(b"--\0".as_ptr() as *const ::core::ffi::c_char);
        diffarg(*argv.offset(optind as isize));
        diffarg(*argv.offset((optind + 1 as ::core::ffi::c_int) as isize));
        diffarg(::core::ptr::null::<::core::ffi::c_char>());
        execvp(
            *diffargv.offset(0isize),
            diffargv as *mut *mut ::core::ffi::c_char as *const *mut ::core::ffi::c_char,
        );
        perror_fatal(squote(0 as ::core::ffi::c_int, *diffargv.offset(0isize)));
    } else {
        let mut leftdir: bool = diraccess(*argv.offset(optind as isize));
        let mut rightdir: bool =
            diraccess(*argv.offset((optind + 1 as ::core::ffi::c_int) as isize));
        if leftdir as ::core::ffi::c_int & rightdir as ::core::ffi::c_int != 0 {
            fatal(b"both files to be compared are directories\0".as_ptr()
                as *const ::core::ffi::c_char);
        }
        let mut lname: *const ::core::ffi::c_char = expand_name(
            *argv.offset(optind as isize),
            leftdir,
            *argv.offset((optind + 1 as ::core::ffi::c_int) as isize),
        );
        let mut rname: *const ::core::ffi::c_char = expand_name(
            *argv.offset((optind + 1 as ::core::ffi::c_int) as isize),
            rightdir,
            *argv.offset(optind as isize),
        );
        let mut left: *mut FILE = ck_fopen(lname, b"re\0".as_ptr() as *const ::core::ffi::c_char);
        let mut right: *mut FILE = ck_fopen(rname, b"re\0".as_ptr() as *const ::core::ffi::c_char);
        let mut out: *mut FILE = ck_fopen(output, b"we\0".as_ptr() as *const ::core::ffi::c_char);
        diffarg(b"--sdiff-merge-assist\0".as_ptr() as *const ::core::ffi::c_char);
        diffarg(b"--\0".as_ptr() as *const ::core::ffi::c_char);
        diffarg(*argv.offset(optind as isize));
        diffarg(*argv.offset((optind + 1 as ::core::ffi::c_int) as isize));
        diffarg(::core::ptr::null::<::core::ffi::c_char>());
        trapsigs();
        let mut diffout: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut diff_fds: [::core::ffi::c_int; 2] = [0; 2];
        if pipe(&raw mut diff_fds as *mut ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
            perror_fatal(b"pipe\0".as_ptr() as *const ::core::ffi::c_char);
        }
        ::core::ptr::write_volatile(&raw mut diffpid, fork() as pid_t);
        if ::core::ptr::read_volatile::<pid_t>(&raw const diffpid) < 0 as ::core::ffi::c_int {
            perror_fatal(b"fork\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if ::core::ptr::read_volatile::<pid_t>(&raw const diffpid) == 0 {
            if initial_handler(C2Rust_Unnamed_12::handler_index_of_SIGINT.0 as ::core::ffi::c_int)
                != ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                    1 as ::core::ffi::c_int as ::libc::intptr_t,
                )
            {
                signal_handler(
                    SIGINT,
                    ::core::mem::transmute::<::libc::intptr_t, sighandler>(
                        1 as ::core::ffi::c_int as ::libc::intptr_t,
                    ),
                );
            }
            signal_handler(SIGPIPE, SIG_DFL);
            close(diff_fds[0usize]);
            if diff_fds[1usize] != STDOUT_FILENO {
                dup2(diff_fds[1usize], STDOUT_FILENO);
                close(diff_fds[1usize]);
            }
            execvp(
                *diffargv.offset(0isize),
                diffargv as *mut *mut ::core::ffi::c_char as *const *mut ::core::ffi::c_char,
            );
            _exit(if *__errno_location() == ENOENT {
                127 as ::core::ffi::c_int
            } else {
                126 as ::core::ffi::c_int
            });
        }
        close(diff_fds[1usize]);
        diffout = fdopen(
            diff_fds[0usize],
            b"r\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if diffout.is_null() {
            perror_fatal(b"fdopen\0".as_ptr() as *const ::core::ffi::c_char);
        }
        let mut lfilt: line_filter = line_filter {
            infile: ::core::ptr::null_mut::<FILE>(),
            bufpos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            buflim: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let mut rfilt: line_filter = line_filter {
            infile: ::core::ptr::null_mut::<FILE>(),
            bufpos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            buflim: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        let mut diff_filt: line_filter = line_filter {
            infile: ::core::ptr::null_mut::<FILE>(),
            bufpos: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            buflim: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        lf_init(&raw mut diff_filt, diffout);
        lf_init(&raw mut lfilt, left);
        lf_init(&raw mut rfilt, right);
        let mut interact_ok: bool = interact(
            &raw mut diff_filt,
            &raw mut lfilt,
            lname,
            &raw mut rfilt,
            rname,
            out,
        );
        ck_fclose(left);
        ck_fclose(right);
        ck_fclose(out);
        let mut wstatus: ::core::ffi::c_int = 0;
        let mut werrno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ck_fclose(diffout);
        while waitpid(
            ::core::ptr::read_volatile::<pid_t>(&raw const diffpid),
            &raw mut wstatus,
            0 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            if *__errno_location() == EINTR {
                checksigs();
            } else {
                perror_fatal(b"waitpid\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
        ::core::ptr::write_volatile(&raw mut diffpid, 0 as ::core::ffi::c_int as pid_t);
        if !::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname).is_null() {
            unlink(::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(
                &raw const tmpname,
            ));
            ::core::ptr::write_volatile(
                &raw mut tmpname,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
            );
        }
        if !interact_ok {
            exiterr();
        }
        check_child_status(werrno, wstatus, EXIT_FAILURE, *diffargv.offset(0isize));
        untrapsig(0 as ::core::ffi::c_int);
        checksigs();
        exit((wstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int);
    }
    return EXIT_SUCCESS;
}
unsafe extern "C" fn diffarg(mut a: *const ::core::ffi::c_char) {
    static mut diffargs: idx_t = 0;
    static mut diffarglim: idx_t = 0;
    if diffargs == diffarglim {
        diffargv = xpalloc(
            diffargv as *mut ::core::ffi::c_void,
            &raw mut diffarglim,
            1 as idx_t,
            -1 as ptrdiff_t,
            ::core::mem::size_of::<*const ::core::ffi::c_char>() as idx_t,
        ) as *mut *const ::core::ffi::c_char;
    }
    let c2rust_fresh6 = diffargs;
    diffargs += 1;
    *diffargv.offset(c2rust_fresh6 as isize) = a;
}
static mut ignore_SIGINT: bool = false;
static mut signal_received: ::core::ffi::c_int = 0;
static mut sigs_trapped: bool = false;
unsafe extern "C" fn catchsig(mut s: ::core::ffi::c_int) {
    if !(s == SIGINT
        && ::core::ptr::read_volatile::<bool>(&raw const ignore_SIGINT) as ::core::ffi::c_int != 0)
    {
        ::core::ptr::write_volatile(&raw mut signal_received, s);
    }
}
static mut catchaction: sigaction = sigaction {
    __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
    sa_mask: __sigset_t { __val: [0; 16] },
    sa_flags: 0,
    sa_restorer: None,
};
unsafe extern "C" fn signal_handler(mut sig: ::core::ffi::c_int, mut handler: sighandler) {
    catchaction.__sigaction_handler.sa_handler = handler as __sighandler_t;
    sigaction(
        sig,
        &raw mut catchaction,
        ::core::ptr::null_mut::<sigaction>(),
    );
}
unsafe extern "C" fn trapsigs() {
    catchaction.sa_flags = SA_RESTART;
    sigemptyset(&raw mut catchaction.sa_mask);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_12::NUM_SIGS.0 as ::core::ffi::c_int {
        sigaddset(&raw mut catchaction.sa_mask, sigs[i as usize]);
        i += 1;
    }
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < C2Rust_Unnamed_12::NUM_SIGS.0 as ::core::ffi::c_int {
        sigaction(
            sigs[i_0 as usize],
            ::core::ptr::null::<sigaction>(),
            (&raw mut initial_action as *mut sigaction).offset(i_0 as isize),
        );
        if initial_handler(i_0)
            != ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            )
        {
            signal_handler(
                sigs[i_0 as usize],
                Some(catchsig as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
            );
        }
        i_0 += 1;
    }
    signal(SIGCHLD, SIG_DFL);
    sigs_trapped = r#true != 0;
}
unsafe extern "C" fn untrapsig(mut s: ::core::ffi::c_int) {
    if sigs_trapped {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < C2Rust_Unnamed_12::NUM_SIGS.0 as ::core::ffi::c_int {
            if (s == 0 || sigs[i as usize] == s)
                && initial_handler(i)
                    != ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                        1 as ::core::ffi::c_int as ::libc::intptr_t,
                    )
            {
                sigaction(
                    sigs[i as usize],
                    (&raw mut initial_action as *mut sigaction).offset(i as isize),
                    ::core::ptr::null_mut::<sigaction>(),
                );
            }
            i += 1;
        }
    }
}
unsafe extern "C" fn checksigs() {
    let mut s: ::core::ffi::c_int =
        ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const signal_received);
    if s != 0 {
        cleanup(0 as ::core::ffi::c_int);
        untrapsig(s);
        raise(s);
        exit(C2Rust_Unnamed_10::EXIT_TROUBLE.0 as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn give_help() {
    fprintf(
        stderr,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"ed:\tEdit then use both versions, each decorated with a header.\neb:\tEdit then use both versions.\nel or e1:\tEdit then use the left version.\ner or e2:\tEdit then use the right version.\ne:\tDiscard both versions then edit a new one.\nl or 1:\tUse the left version.\nr or 2:\tUse the right version.\ns:\tSilently include common lines.\nv:\tVerbosely include common lines.\nq:\tQuit.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
}
unsafe extern "C" fn skip_white() -> ::core::ffi::c_int {
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = getchar_unlocked();
        if !c_isspace(c) || c == '\n' as ::core::ffi::c_int {
            break;
        }
        checksigs();
    }
    if ferror_unlocked(stdin) != 0 {
        perror_fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"read failed\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
    return c;
}
unsafe extern "C" fn flush_line() {
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = getchar_unlocked();
        if !(c != '\n' as ::core::ffi::c_int && c != EOF) {
            break;
        }
    }
    if ferror_unlocked(stdin) != 0 {
        perror_fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"read failed\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
}
unsafe extern "C" fn edit(
    mut left: *mut line_filter,
    mut lname: *const ::core::ffi::c_char,
    mut lline: lin,
    mut llen: lin,
    mut right: *mut line_filter,
    mut rname: *const ::core::ffi::c_char,
    mut rline: lin,
    mut rlen: lin,
    mut outfile: *mut FILE,
) -> bool {
    loop {
        let mut cmd0: ::core::ffi::c_int = 0;
        let mut cmd1: ::core::ffi::c_int = 0;
        let mut gotcmd: bool = r#false != 0;
        's_162: while !gotcmd {
            if putchar_unlocked('%' as ::core::ffi::c_int) != '%' as ::core::ffi::c_int {
                perror_fatal(dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"write failed\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ));
            }
            ck_fflush(stdout);
            cmd0 = skip_white();
            'c_13540: {
                match cmd0 {
                    49 | 50 | 108 | 114 | 115 | 118 | 113 => {
                        if skip_white() != '\n' as ::core::ffi::c_int {
                            give_help();
                            flush_line();
                            continue 's_162;
                        } else {
                            gotcmd = r#true != 0;
                            continue 's_162;
                        }
                    }
                    101 => {
                        cmd1 = skip_white();
                        match cmd1 {
                            49 | 50 | 98 | 100 | 108 | 114 => {
                                if skip_white() != '\n' as ::core::ffi::c_int {
                                    give_help();
                                    flush_line();
                                    continue 's_162;
                                } else {
                                    gotcmd = r#true != 0;
                                    continue 's_162;
                                }
                            }
                            10 => {
                                gotcmd = r#true != 0;
                                continue 's_162;
                            }
                            _ => {
                                give_help();
                                flush_line();
                                continue 's_162;
                            }
                        }
                    }
                    EOF => {
                        if feof_unlocked(stdin) != 0 {
                            gotcmd = r#true != 0;
                            cmd0 = 'q' as ::core::ffi::c_int;
                            continue 's_162;
                        }
                    }
                    10 => {
                        break 'c_13540;
                    }
                    _ => {}
                }
                flush_line();
            }
            give_help();
        }
        let mut argv: [*mut ::core::ffi::c_char; 3] =
            [::core::ptr::null_mut::<::core::ffi::c_char>(); 3];
        let mut wstatus: ::core::ffi::c_int = 0;
        let mut werrno: ::core::ffi::c_int = 0;
        let mut pid: pid_t = 0;
        let mut buf: [::core::ffi::c_char; 65536] = [0; 65536];
        match cmd0 {
            49 | 108 => {
                lf_copy(left, llen, outfile);
                lf_skip(right, rlen);
                return r#true != 0;
            }
            50 | 114 => {
                lf_copy(right, rlen, outfile);
                lf_skip(left, llen);
                return r#true != 0;
            }
            115 => {
                suppress_common_lines = r#true != 0;
            }
            118 => {
                suppress_common_lines = r#false != 0;
            }
            113 => return r#false != 0,
            101 => {
                if !::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname)
                    .is_null()
                {
                    tmp = fopen(
                        ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname),
                        b"we\0".as_ptr() as *const ::core::ffi::c_char,
                    ) as *mut FILE;
                } else {
                    let mut fd: ::core::ffi::c_int = temporary_file();
                    if fd < 0 as ::core::ffi::c_int {
                        perror_fatal(b"mkstemp\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    tmp = fdopen(fd, b"w\0".as_ptr() as *const ::core::ffi::c_char);
                }
                if tmp.is_null() {
                    perror_fatal(squote(
                        0 as ::core::ffi::c_int,
                        ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname),
                    ));
                }
                's_295: {
                    match cmd1 {
                        100 => {
                            if llen != 0 {
                                if llen == 1 as lin {
                                    fprintf(
                                        tmp,
                                        b"--- %s %td\n\0".as_ptr() as *const ::core::ffi::c_char,
                                        lname,
                                        lline,
                                    );
                                } else {
                                    fprintf(
                                        tmp,
                                        b"--- %s %td,%td\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        lname,
                                        lline,
                                        lline + llen - 1 as lin,
                                    );
                                }
                            }
                        }
                        49 | 98 | 108 => {}
                        _ => {
                            lf_skip(left, llen);
                            break 's_295;
                        }
                    }
                    lf_copy(left, llen, tmp);
                }
                's_345: {
                    match cmd1 {
                        100 => {
                            if rlen != 0 {
                                if rlen == 1 as lin {
                                    fprintf(
                                        tmp,
                                        b"+++ %s %td\n\0".as_ptr() as *const ::core::ffi::c_char,
                                        rname,
                                        rline,
                                    );
                                } else {
                                    fprintf(
                                        tmp,
                                        b"+++ %s %td,%td\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        rname,
                                        rline,
                                        rline + rlen - 1 as lin,
                                    );
                                }
                            }
                        }
                        50 | 98 | 114 => {}
                        _ => {
                            lf_skip(right, rlen);
                            break 's_345;
                        }
                    }
                    lf_copy(right, rlen, tmp);
                }
                ck_fclose(tmp);
                ::core::ptr::write_volatile(&raw mut ignore_SIGINT, r#true != 0);
                checksigs();
                argv = [
                    editor_program as *mut ::core::ffi::c_char,
                    ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                ];
                wstatus = 0;
                werrno = 0 as ::core::ffi::c_int;
                pid = fork();
                if pid == 0 as ::core::ffi::c_int {
                    execvp(
                        editor_program,
                        &raw mut argv as *mut *mut ::core::ffi::c_char
                            as *const *mut ::core::ffi::c_char,
                    );
                    _exit(if *__errno_location() == ENOENT {
                        127 as ::core::ffi::c_int
                    } else {
                        126 as ::core::ffi::c_int
                    });
                }
                if pid < 0 as ::core::ffi::c_int {
                    perror_fatal(b"fork\0".as_ptr() as *const ::core::ffi::c_char);
                }
                while waitpid(pid, &raw mut wstatus, 0 as ::core::ffi::c_int)
                    < 0 as ::core::ffi::c_int
                {
                    if *__errno_location() == EINTR {
                        checksigs();
                    } else {
                        perror_fatal(b"waitpid\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                }
                ::core::ptr::write_volatile(&raw mut ignore_SIGINT, r#false != 0);
                check_child_status(werrno, wstatus, EXIT_SUCCESS, editor_program);
                buf = [0; 65536];
                tmp = ck_fopen(
                    ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmpname),
                    b"re\0".as_ptr() as *const ::core::ffi::c_char,
                );
                let mut size: idx_t = 0;
                loop {
                    size = ck_fread(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        C2Rust_Unnamed_11::SDIFF_BUFSIZE.0 as ::core::ffi::c_int as idx_t,
                        tmp,
                    );
                    if size == 0 as idx_t {
                        break;
                    }
                    checksigs();
                    ck_fwrite(&raw mut buf as *mut ::core::ffi::c_char, size, outfile);
                }
                ck_fclose(tmp);
                return r#true != 0;
            }
            _ => {
                give_help();
            }
        }
    }
}
unsafe extern "C" fn interact(
    mut diff: *mut line_filter,
    mut left: *mut line_filter,
    mut lname: *const ::core::ffi::c_char,
    mut right: *mut line_filter,
    mut rname: *const ::core::ffi::c_char,
    mut outfile: *mut FILE,
) -> bool {
    let mut lline: lin = 1 as lin;
    let mut rline: lin = 1 as lin;
    loop {
        let mut diff_help: [::core::ffi::c_char; 256] = [0; 256];
        let mut snarfed: ::core::ffi::c_int = lf_snarf(
            diff,
            &raw mut diff_help as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as idx_t,
        );
        if snarfed <= 0 as ::core::ffi::c_int {
            return snarfed != 0 as ::core::ffi::c_int;
        }
        checksigs();
        if diff_help[0usize] as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
            puts(
                (&raw mut diff_help as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
            );
        } else {
            *__errno_location() = 0 as ::core::ffi::c_int;
            let mut numend: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut val: intmax_t = strtoimax(
                (&raw mut diff_help as *mut ::core::ffi::c_char)
                    .offset(1 as ::core::ffi::c_int as isize),
                &raw mut numend,
                10 as ::core::ffi::c_int,
            );
            if !(0 as intmax_t <= val && val <= LIN_MAX as intmax_t)
                || *__errno_location() != 0
                || *numend as ::core::ffi::c_int != ',' as ::core::ffi::c_int
            {
                fatal(&raw mut diff_help as *mut ::core::ffi::c_char);
            }
            let mut llen: lin = val as lin;
            val = strtoimax(
                numend.offset(1 as ::core::ffi::c_int as isize),
                &raw mut numend,
                10 as ::core::ffi::c_int,
            );
            if !(0 as intmax_t <= val && val <= LIN_MAX as intmax_t)
                || *__errno_location() != 0
                || *numend as ::core::ffi::c_int != 0
            {
                fatal(&raw mut diff_help as *mut ::core::ffi::c_char);
            }
            let mut rlen: lin = val as lin;
            let mut lenmax: lin = if llen > rlen { llen } else { rlen };
            match diff_help[0usize] as ::core::ffi::c_int {
                105 => {
                    if suppress_common_lines {
                        lf_skip(diff, lenmax);
                    } else {
                        lf_copy(diff, lenmax, stdout);
                    }
                    lf_copy(left, llen, outfile);
                    lf_skip(right, rlen);
                }
                99 => {
                    lf_copy(diff, lenmax, stdout);
                    if !edit(left, lname, lline, llen, right, rname, rline, rlen, outfile) {
                        return r#false != 0;
                    }
                }
                _ => {
                    fatal(&raw mut diff_help as *mut ::core::ffi::c_char);
                }
            }
            lline += llen;
            rline += rlen;
        }
    }
}
unsafe extern "C" fn diraccess(mut dir: *const ::core::ffi::c_char) -> bool {
    let mut buf: stat = stat {
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
    return stat(dir, &raw mut buf) == 0 as ::core::ffi::c_int
        && buf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t;
}
pub const TMPDIR_ENV: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"TMPDIR\0") };
unsafe extern "C" fn temporary_file() -> ::core::ffi::c_int {
    let mut tmpdir: *const ::core::ffi::c_char = getenv(TMPDIR_ENV.as_ptr());
    let mut dir: *const ::core::ffi::c_char = if !tmpdir.is_null() {
        tmpdir
    } else {
        P_tmpdir.as_ptr()
    };
    let mut buf: *mut ::core::ffi::c_char = xmalloc(
        strlen(dir)
            .wrapping_add(1 as size_t)
            .wrapping_add(5 as size_t)
            .wrapping_add(6 as size_t)
            .wrapping_add(1 as size_t),
    ) as *mut ::core::ffi::c_char;
    strcpy(
        stpcpy(buf, dir),
        b"/sdiffXXXXXX\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut fd: ::core::ffi::c_int = mkstemp(buf);
    if fd < 0 as ::core::ffi::c_int {
        free(buf as *mut ::core::ffi::c_void);
    } else {
        ::core::ptr::write_volatile(&raw mut tmpname, buf);
    }
    return fd;
}
pub const nullptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const DEFAULT_EDITOR_PROGRAM: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"ed\0") };
pub const GNULIB_LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"diffutils\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"GNU diffutils\0") };
