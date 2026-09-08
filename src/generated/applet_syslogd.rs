// Generated from pinned GNU syslogd 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 5508d256758a41a4f9b074d6eec86983995d7e955d8e16aa727510556b88596e
/* syslogd - log system messages
  Copyright (C) 1997-2026 Free Software Foundation, Inc.

  This file is part of GNU Inetutils.

  GNU Inetutils is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or (at
  your option) any later version.

  GNU Inetutils is distributed in the hope that it will be useful, but
  WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
  General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with this program.  If not, see `http://www.gnu.org/licenses/'. */
#[repr(C)]
pub struct sockaddr_x25 { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_ns { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_iso { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_ipx { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_inarp { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_eon { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_dl { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_ax25 { _opaque: [u8; 0] }
#[repr(C)]
pub struct sockaddr_at { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct __dirstream { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
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
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn alarm(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn getpid() -> __pid_t;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fsync(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut ::core::ffi::c_char;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lchmod(__file: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn sendto(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn writev(
        __fd: ::core::ffi::c_int,
        __iovec: *const iovec,
        __count: ::core::ffi::c_int,
    ) -> ssize_t;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut ::core::ffi::c_char,
        __hostlen: socklen_t,
        __serv: *mut ::core::ffi::c_char,
        __servlen: socklen_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn asprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut program_invocation_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_syslogd_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_syslogd_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_syslogd_argp_error"]
    fn argp_error(__state: *const argp_state, __fmt: *const ::core::ffi::c_char, ...);
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_syslogd_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_syslogd_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_syslogd_localhost"]
    fn localhost() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_syslogd_inetutils_ttymsg"]
    fn inetutils_ttymsg(
        _: *mut iovec,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_syslogd_default_program_authors"]
    static mut default_program_authors: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_syslogd_read_utmp"]
    fn read_utmp(
        file: *const ::core::ffi::c_char,
        n_entries: *mut idx_t,
        utmp_buf: *mut *mut STRUCT_UTMP,
        options: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_syslogd_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_syslogd_waitdaemon"]
    fn waitdaemon(
        nochdir: ::core::ffi::c_int,
        noclose: ::core::ffi::c_int,
        maxwait: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
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
pub type __socklen_t = ::core::ffi::c_uint;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
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
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
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
pub type ptrdiff_t = isize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __socket_type(pub ::core::ffi::c_uint);
impl __socket_type {
    pub const SOCK_STREAM: Self = Self(1);
    pub const SOCK_DGRAM: Self = Self(2);
    pub const SOCK_RAW: Self = Self(3);
    pub const SOCK_RDM: Self = Self(4);
    pub const SOCK_SEQPACKET: Self = Self(5);
    pub const SOCK_DCCP: Self = Self(6);
    pub const SOCK_PACKET: Self = Self(10);
    pub const SOCK_CLOEXEC: Self = Self(524288);
    pub const SOCK_NONBLOCK: Self = Self(2048);
}
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: sa_family_t,
    pub sun_path: [::core::ffi::c_char; 108],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type uint32_t = u32;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type uint16_t = u16;
pub type uint8_t = u8;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const IPPROTO_IP: Self = Self(0);
    pub const IPPROTO_ICMP: Self = Self(1);
    pub const IPPROTO_IGMP: Self = Self(2);
    pub const IPPROTO_IPIP: Self = Self(4);
    pub const IPPROTO_TCP: Self = Self(6);
    pub const IPPROTO_EGP: Self = Self(8);
    pub const IPPROTO_PUP: Self = Self(12);
    pub const IPPROTO_UDP: Self = Self(17);
    pub const IPPROTO_IDP: Self = Self(22);
    pub const IPPROTO_TP: Self = Self(29);
    pub const IPPROTO_DCCP: Self = Self(33);
    pub const IPPROTO_IPV6: Self = Self(41);
    pub const IPPROTO_RSVP: Self = Self(46);
    pub const IPPROTO_GRE: Self = Self(47);
    pub const IPPROTO_ESP: Self = Self(50);
    pub const IPPROTO_AH: Self = Self(51);
    pub const IPPROTO_MTP: Self = Self(92);
    pub const IPPROTO_BEETPH: Self = Self(94);
    pub const IPPROTO_ENCAP: Self = Self(98);
    pub const IPPROTO_PIM: Self = Self(103);
    pub const IPPROTO_COMP: Self = Self(108);
    pub const IPPROTO_L2TP: Self = Self(115);
    pub const IPPROTO_SCTP: Self = Self(132);
    pub const IPPROTO_UDPLITE: Self = Self(136);
    pub const IPPROTO_MPLS: Self = Self(137);
    pub const IPPROTO_ETHERNET: Self = Self(143);
    pub const IPPROTO_RAW: Self = Self(255);
    pub const IPPROTO_SMC: Self = Self(256);
    pub const IPPROTO_MPTCP: Self = Self(262);
    pub const IPPROTO_MAX: Self = Self(263);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const _ISupper: Self = Self(256);
    pub const _ISlower: Self = Self(512);
    pub const _ISalpha: Self = Self(1024);
    pub const _ISdigit: Self = Self(2048);
    pub const _ISxdigit: Self = Self(4096);
    pub const _ISspace: Self = Self(8192);
    pub const _ISprint: Self = Self(16384);
    pub const _ISgraph: Self = Self(32768);
    pub const _ISblank: Self = Self(1);
    pub const _IScntrl: Self = Self(2);
    pub const _ISpunct: Self = Self(4);
    pub const _ISalnum: Self = Self(8);
}
pub type error_t = ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _code {
    pub c_name: *mut ::core::ffi::c_char,
    pub c_val: ::core::ffi::c_int,
}
pub type CODE = _code;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_utmp {
    pub ut_user: *mut ::core::ffi::c_char,
    pub ut_id: *mut ::core::ffi::c_char,
    pub ut_line: *mut ::core::ffi::c_char,
    pub ut_host: *mut ::core::ffi::c_char,
    pub ut_ts: timespec,
    pub ut_pid: pid_t,
    pub ut_session: pid_t,
    pub ut_type: ::core::ffi::c_short,
    pub ut_exit: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub e_termination: ::core::ffi::c_int,
    pub e_exit: ::core::ffi::c_int,
}
pub type STRUCT_UTMP = gl_utmp;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_14(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_14 {
    pub const READ_UTMP_CHECK_PIDS: Self = Self(1);
    pub const READ_UTMP_USER_PROCESS: Self = Self(2);
    pub const READ_UTMP_BOOT_TIME: Self = Self(4);
    pub const READ_UTMP_NO_BOOT_TIME: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct funix {
    pub name: *const ::core::ffi::c_char,
    pub fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct filed {
    pub f_next: *mut filed,
    pub f_type: ::core::ffi::c_short,
    pub f_file: ::core::ffi::c_short,
    pub f_time: time_t,
    pub f_pmask: [::core::ffi::c_uchar; 25],
    pub f_un: C2Rust_Unnamed_15,
    pub f_prevline: [::core::ffi::c_char; 240],
    pub f_lasttime: [::core::ffi::c_char; 16],
    pub f_prevhost: *mut ::core::ffi::c_char,
    pub f_progname: *mut ::core::ffi::c_char,
    pub f_prognlen: ::core::ffi::c_int,
    pub f_prevpri: ::core::ffi::c_int,
    pub f_prevlen: ::core::ffi::c_int,
    pub f_prevcount: ::core::ffi::c_int,
    pub f_repeatcount: size_t,
    pub f_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub f_user: C2Rust_Unnamed_17,
    pub f_forw: C2Rust_Unnamed_16,
    pub f_fname: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub f_hname: *mut ::core::ffi::c_char,
    pub f_addr: sockaddr_storage,
    pub f_addrlen: socklen_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub f_nusers: ::core::ffi::c_int,
    pub f_unames: *mut *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_18(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_18 {
    pub const OPT_NO_FORWARD: Self = Self(256);
    pub const OPT_NO_KLOG: Self = Self(257);
    pub const OPT_NO_UNIXAF: Self = Self(258);
    pub const OPT_IPANY: Self = Self(259);
}
pub const PATH_CONSOLE: [::core::ffi::c_char; 13] = _PATH_CONSOLE;
pub const PATH_LOG: [::core::ffi::c_char; 9] = _PATH_LOG;
pub const PATH_KLOG: [::core::ffi::c_char; 11] = _PATH_KLOG;
pub const _PATH_CONSOLE: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"/dev/console\0") };
pub const _PATH_KLOG: [::core::ffi::c_char; 11] =
    unsafe { ::core::mem::transmute::<[u8; 11], [::core::ffi::c_char; 11]>(*b"/proc/kmsg\0") };
pub const _PATH_UTMP: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"/var/run/utmp\0") };
pub const IOVCNT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MAXLINE: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const MAXSVLINE: ::core::ffi::c_int = 240 as ::core::ffi::c_int;
pub const DEFUPRI: ::core::ffi::c_int = LOG_USER | LOG_NOTICE;
pub const DEFSPRI: ::core::ffi::c_int = LOG_KERN | LOG_CRIT;
pub const TIMERINTVL: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const TTYMSGTIME: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const _POSIX2_LINE_MAX: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
pub const LINE_MAX: ::core::ffi::c_int = _POSIX2_LINE_MAX;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const POLLPRI: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const POLLERR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const POLLHUP: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const POLLNVAL: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const IPV6_V6ONLY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const AI_PASSIVE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AI_CANONNAME: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const AI_ADDRCONFIG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EAI_NONAME: ::core::ffi::c_int = -2;
pub const EAI_AGAIN: ::core::ffi::c_int = -3;
pub const EAI_MEMORY: ::core::ffi::c_int = -10;
pub const EAI_NODATA: ::core::ffi::c_int = -5;
pub const EAI_ADDRFAMILY: ::core::ffi::c_int = -9;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NI_NAMEREQD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const _IOLBF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EIO: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const _PATH_LOG: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"/dev/log\0") };
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn rpl_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return realloc(ptr, if size != 0 { size } else { 1 as size_t });
}
pub const LOG_EMERG: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_ALERT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOG_CRIT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_NOTICE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LOG_INFO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOG_DEBUG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const LOG_PRIMASK: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
pub const INTERNAL_NOPRI: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const INTERNAL_MARK: ::core::ffi::c_int =
    (24 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int | 0 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_prioritynames"]
pub static mut prioritynames: [CODE; 13] = [
    _code {
        c_name: b"alert\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_ALERT,
    },
    _code {
        c_name: b"crit\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_CRIT,
    },
    _code {
        c_name: b"debug\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_DEBUG,
    },
    _code {
        c_name: b"emerg\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_EMERG,
    },
    _code {
        c_name: b"err\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_ERR,
    },
    _code {
        c_name: b"error\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_ERR,
    },
    _code {
        c_name: b"info\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_INFO,
    },
    _code {
        c_name: b"none\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: INTERNAL_NOPRI,
    },
    _code {
        c_name: b"notice\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_NOTICE,
    },
    _code {
        c_name: b"panic\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_EMERG,
    },
    _code {
        c_name: b"warn\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_WARNING,
    },
    _code {
        c_name: b"warning\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_WARNING,
    },
    _code {
        c_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        c_val: -1 as ::core::ffi::c_int,
    },
];
pub const LOG_KERN: ::core::ffi::c_int = (0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_USER: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_MAIL: ::core::ffi::c_int = (2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_AUTH: ::core::ffi::c_int = (4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_SYSLOG: ::core::ffi::c_int = (5 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LPR: ::core::ffi::c_int = (6 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_NEWS: ::core::ffi::c_int = (7 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_UUCP: ::core::ffi::c_int = (8 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_CRON: ::core::ffi::c_int = (9 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_AUTHPRIV: ::core::ffi::c_int = (10 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_FTP: ::core::ffi::c_int = (11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL0: ::core::ffi::c_int = (16 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL1: ::core::ffi::c_int = (17 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL2: ::core::ffi::c_int = (18 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL3: ::core::ffi::c_int = (19 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL4: ::core::ffi::c_int = (20 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL5: ::core::ffi::c_int = (21 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL6: ::core::ffi::c_int = (22 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_LOCAL7: ::core::ffi::c_int = (23 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_NFACILITIES: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const LOG_FACMASK: ::core::ffi::c_int = 0x3f8 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_facilitynames"]
pub static mut facilitynames: [CODE; 23] = [
    _code {
        c_name: b"auth\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_AUTH,
    },
    _code {
        c_name: b"authpriv\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_AUTHPRIV,
    },
    _code {
        c_name: b"cron\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_CRON,
    },
    _code {
        c_name: b"daemon\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_DAEMON,
    },
    _code {
        c_name: b"ftp\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_FTP,
    },
    _code {
        c_name: b"kern\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_KERN,
    },
    _code {
        c_name: b"lpr\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LPR,
    },
    _code {
        c_name: b"mail\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_MAIL,
    },
    _code {
        c_name: b"mark\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: INTERNAL_MARK,
    },
    _code {
        c_name: b"news\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_NEWS,
    },
    _code {
        c_name: b"security\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_AUTH,
    },
    _code {
        c_name: b"syslog\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_SYSLOG,
    },
    _code {
        c_name: b"user\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_USER,
    },
    _code {
        c_name: b"uucp\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_UUCP,
    },
    _code {
        c_name: b"local0\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL0,
    },
    _code {
        c_name: b"local1\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL1,
    },
    _code {
        c_name: b"local2\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL2,
    },
    _code {
        c_name: b"local3\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL3,
    },
    _code {
        c_name: b"local4\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL4,
    },
    _code {
        c_name: b"local5\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL5,
    },
    _code {
        c_name: b"local6\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL6,
    },
    _code {
        c_name: b"local7\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        c_val: LOG_LOCAL7,
    },
    _code {
        c_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        c_val: -1 as ::core::ffi::c_int,
    },
];
pub const _PATH_UTMPX: [::core::ffi::c_char; 14] = _PATH_UTMP;
pub const UTMPX_FILE: [::core::ffi::c_char; 14] = _PATH_UTMPX;
pub const UTMP_FILE: [::core::ffi::c_char; 14] = UTMPX_FILE;
#[export_name = "rboxc_syslogd_facilities_seen"]
pub static mut facilities_seen: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_selector"]
pub static mut selector: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_ConfFile"]
pub static mut ConfFile: *const ::core::ffi::c_char = PATH_LOGCONF.as_ptr();
#[export_name = "rboxc_syslogd_ConfDir"]
pub static mut ConfDir: *const ::core::ffi::c_char = PATH_LOGCONFD.as_ptr();
#[export_name = "rboxc_syslogd_PidFile"]
pub static mut PidFile: *const ::core::ffi::c_char = PATH_LOGPID.as_ptr();
#[export_name = "rboxc_syslogd_ctty"]
pub static mut ctty: [::core::ffi::c_char; 13] = PATH_CONSOLE;
static mut dbg_output: ::core::ffi::c_int = 0;
static mut restart: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_funix"]
pub static mut funix: *mut funix = ::core::ptr::null_mut::<funix>();
#[export_name = "rboxc_syslogd_nfunix"]
pub static mut nfunix: size_t = 0;
pub const IGN_CONS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SYNC_FILE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const ADDDATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MARK: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_Files"]
pub static mut Files: *mut filed = ::core::ptr::null_mut::<filed>();
#[export_name = "rboxc_syslogd_consfile"]
pub static mut consfile: filed = filed {
    f_next: ::core::ptr::null_mut::<filed>(),
    f_type: 0,
    f_file: 0,
    f_time: 0,
    f_pmask: [0; 25],
    f_un: C2Rust_Unnamed_15 {
        f_user: C2Rust_Unnamed_17 {
            f_nusers: 0,
            f_unames: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        },
    },
    f_prevline: [0; 240],
    f_lasttime: [0; 16],
    f_prevhost: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    f_progname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    f_prognlen: 0,
    f_prevpri: 0,
    f_prevlen: 0,
    f_prevcount: 0,
    f_repeatcount: 0,
    f_flags: 0,
};
pub const F_UNUSED: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_FILE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const F_TTY: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const F_CONSOLE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_FORW: ::core::ffi::c_int = 4;
pub const F_USERS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const F_WALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const F_FORW_SUSP: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const F_FORW_UNKN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const F_PIPE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_TypeNames"]
pub static mut TypeNames: [*const ::core::ffi::c_char; 10] = [
    b"UNUSED\0".as_ptr() as *const ::core::ffi::c_char,
    b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
    b"TTY\0".as_ptr() as *const ::core::ffi::c_char,
    b"CONSOLE\0".as_ptr() as *const ::core::ffi::c_char,
    b"FORW\0".as_ptr() as *const ::core::ffi::c_char,
    b"USERS\0".as_ptr() as *const ::core::ffi::c_char,
    b"WALL\0".as_ptr() as *const ::core::ffi::c_char,
    b"FORW(SUSPENDED)\0".as_ptr() as *const ::core::ffi::c_char,
    b"FORW(UNKNOWN)\0".as_ptr() as *const ::core::ffi::c_char,
    b"PIPE\0".as_ptr() as *const ::core::ffi::c_char,
];
pub const OMIT_SYNC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const INET_SUSPEND_TIME: ::core::ffi::c_int = 180 as ::core::ffi::c_int;
pub const INET_RETRY_MAX: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_repeatinterval"]
pub static mut repeatinterval: [::core::ffi::c_int; 2] =
    [30 as ::core::ffi::c_int, 60 as ::core::ffi::c_int];
pub const MAXREPEAT: usize = ::core::mem::size_of::<[::core::ffi::c_int; 2]>()
    .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>())
    .wrapping_sub(1usize);
pub const LIST_DELIMITER: ::core::ffi::c_int = ':' as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_LocalHostName"]
pub static mut LocalHostName: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_LocalDomain"]
pub static mut LocalDomain: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_BindAddress"]
pub static mut BindAddress: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_BindPort"]
pub static mut BindPort: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_addrstr"]
pub static mut addrstr: [::core::ffi::c_char; 46] = [0; 46];
#[export_name = "rboxc_syslogd_addrname"]
pub static mut addrname: [::core::ffi::c_char; 1025] = [0; 1025];
#[export_name = "rboxc_syslogd_usefamily"]
pub static mut usefamily: ::core::ffi::c_int = AF_INET;
#[export_name = "rboxc_syslogd_finet"]
pub static mut finet: [::core::ffi::c_int; 2] =
    [-1 as ::core::ffi::c_int, -1 as ::core::ffi::c_int];
pub const IU_FD_IP4: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IU_FD_IP6: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_fklog"]
pub static mut fklog: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_LogPortText"]
pub static mut LogPortText: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_LogForwardPort"]
pub static mut LogForwardPort: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_Initialized"]
pub static mut Initialized: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_MarkInterval"]
pub static mut MarkInterval: ::core::ffi::c_int =
    20 as ::core::ffi::c_int * 60 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_MarkSeq"]
pub static mut MarkSeq: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_Debug"]
pub static mut Debug: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_AcceptRemote"]
pub static mut AcceptRemote: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_StripDomains"]
pub static mut StripDomains: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_LocalHosts"]
pub static mut LocalHosts: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
#[export_name = "rboxc_syslogd_NoDetach"]
pub static mut NoDetach: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_NoHops"]
pub static mut NoHops: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_NoKLog"]
pub static mut NoKLog: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_NoUnixAF"]
pub static mut NoUnixAF: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_NoForward"]
pub static mut NoForward: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_now"]
pub static mut now: time_t = 0;
#[export_name = "rboxc_syslogd_force_sync"]
pub static mut force_sync: ::core::ffi::c_int = 0;
#[export_name = "rboxc_syslogd_set_local_time"]
pub static mut set_local_time: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_syslogd_args_doc"]
pub static mut args_doc: [::core::ffi::c_char; 1] =
    unsafe { ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0") };
#[export_name = "rboxc_syslogd_doc"]
pub static mut doc: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"Log system messages.\0")
};
static mut argp_options: [argp_option; 23] = [
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'a' as ::core::ffi::c_int,
        arg: b"SOCKET\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"add unix socket to listen to (up to 19)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'l' as ::core::ffi::c_int,
        arg: b"HOSTLIST\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"log hosts in HOSTLIST by their hostname\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 's' as ::core::ffi::c_int,
        arg: b"DOMAINLIST\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"list of domains which should be stripped from the FQDN of hosts before logging their name\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'd' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"print debug information (implies --no-detach)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"hop\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'h' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"forward messages from remote hosts\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"inet\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'r' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"receive remote messages via internet domain socket\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ipv4\0".as_ptr() as *const ::core::ffi::c_char,
        key: '4' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"restrict to IPv4 transport (default)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ipv6\0".as_ptr() as *const ::core::ffi::c_char,
        key: '6' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"restrict to IPv6 transport\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ipany\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_18::OPT_IPANY.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"allow transport with IPv4 and IPv6\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"bind\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'b' as ::core::ffi::c_int,
        arg: b"ADDR\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"bind listener to this address/name\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"bind-port\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'B' as ::core::ffi::c_int,
        arg: b"PORT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"bind listener to this port\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"mark\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'm' as ::core::ffi::c_int,
        arg: b"INTVL\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"specify timestamp interval in minutes (0 for no timestamping)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-detach\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'n' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not enter daemon mode\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-forward\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_18::OPT_NO_FORWARD.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not forward any messages (overrides --hop)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-klog\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_18::OPT_NO_KLOG.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not listen to kernel log device /proc/kmsg\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-unixaf\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_18::OPT_NO_UNIXAF.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not listen on unix domain sockets (overrides -a and -p)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"pidfile\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'P' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"override pidfile (default: /root/rboxc/build/oracle/inetutils/var/run/syslog.pid)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rcfile\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"override configuration file (default: /root/rboxc/build/oracle/inetutils/etc/syslog.conf)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rcdir\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'D' as ::core::ffi::c_int,
        arg: b"DIR\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"override configuration directory (default: /root/rboxc/build/oracle/inetutils/etc/syslog.d)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"socket\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"override default unix domain socket /dev/log\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"sync\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'S' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"force a file sync on every line\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"local-time\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'T' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"set local time on received messages\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: 0 as ::core::ffi::c_int,
    },
];
pub const GRP: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut v: ::core::ffi::c_int = 0;
    match key {
        97 => {
            add_funix(arg);
        }
        108 => {
            LocalHosts = crunch_list(LocalHosts, arg);
        }
        115 => {
            StripDomains = crunch_list(StripDomains, arg);
        }
        100 => {
            Debug = 1 as ::core::ffi::c_int;
            NoDetach = 1 as ::core::ffi::c_int;
        }
        104 => {
            NoHops = 0 as ::core::ffi::c_int;
        }
        114 => {
            AcceptRemote = 1 as ::core::ffi::c_int;
        }
        52 => {
            usefamily = AF_INET;
        }
        54 => {
            usefamily = AF_INET6;
        }
        259 => {
            usefamily = AF_UNSPEC;
        }
        98 => {
            BindAddress = arg;
        }
        66 => {
            BindPort = arg;
        }
        109 => {
            v = strtol(arg, &raw mut endptr, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *endptr != 0 {
                argp_error(
                    state,
                    b"invalid value (`%s' near `%s')\0".as_ptr() as *const ::core::ffi::c_char,
                    arg,
                    endptr,
                );
            }
            MarkInterval = v * 60 as ::core::ffi::c_int;
        }
        110 => {
            NoDetach = 1 as ::core::ffi::c_int;
        }
        256 => {
            NoForward = 1 as ::core::ffi::c_int;
        }
        257 => {
            NoKLog = 1 as ::core::ffi::c_int;
        }
        258 => {
            NoUnixAF = 1 as ::core::ffi::c_int;
        }
        80 => {
            PidFile = arg;
        }
        102 => {
            ConfFile = arg;
        }
        68 => {
            ConfDir = arg;
        }
        112 => {
            (*funix.offset(0isize)).name = arg;
            (*funix.offset(0isize)).fd = -1 as ::core::ffi::c_int;
        }
        83 => {
            force_sync = 1 as ::core::ffi::c_int;
        }
        84 => {
            set_local_time = 1 as ::core::ffi::c_int;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    return 0 as error_t;
}
static mut argp: argp = unsafe {
    argp {
        options: &raw const argp_options as *mut argp_option,
        parser: Some(
            parse_opt
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut argp_state,
                ) -> error_t,
        ),
        args_doc: &raw const args_doc as *const ::core::ffi::c_char,
        doc: &raw const doc as *const ::core::ffi::c_char,
        children: ::core::ptr::null::<argp_child>(),
        help_filter: None,
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_syslogd(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut kline: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut kline_len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ppid: pid_t = 0 as pid_t;
    let mut fdarray: *mut pollfd = ::core::ptr::null_mut::<pollfd>();
    let mut nfds: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
        sa_mask: sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    set_program_name(*argv.offset(0isize));
    add_funix(PATH_LOG.as_ptr());
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"syslogd\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut default_program_authors as *mut *const ::core::ffi::c_char,
    );
    rpl_argp_parse(
        &raw mut argp,
        argc,
        argv,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        NULL,
    );
    find_inet_port(BindPort);
    if NoDetach == 0 {
        signal(
            SIGTERM,
            Some(doexit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
        ppid = waitdaemon(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            30 as ::core::ffi::c_int,
        ) as pid_t;
        if ppid < 0 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"could not become daemon\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"could not become daemon\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    } else {
        if Debug != 0 {
            dbg_output = 1 as ::core::ffi::c_int;
        }
        setvbuf(
            stdout,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            _IOLBF,
            0 as size_t,
        );
    }
    LocalHostName = localhost();
    if LocalHostName.is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"can't get local host name\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"can't get local host name\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    p = strchr(LocalHostName, '.' as ::core::ffi::c_int);
    if !p.is_null() {
        let c2rust_fresh16 = p;
        p = p.offset(1);
        *c2rust_fresh16 = '\0' as ::core::ffi::c_char;
        LocalDomain = p;
    } else {
        let mut hints: addrinfo = addrinfo {
            ai_flags: 0,
            ai_family: 0,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: ::core::ptr::null_mut::<sockaddr>(),
            ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ai_next: ::core::ptr::null_mut::<addrinfo>(),
        };
        let mut rp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut err: ::core::ffi::c_int = 0;
        memset(
            &raw mut hints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<addrinfo>(),
        );
        hints.ai_family = AF_UNSPEC;
        hints.ai_flags = AI_CANONNAME;
        err = getaddrinfo(
            LocalHostName,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut hints,
            &raw mut rp,
        );
        if err == 0 as ::core::ffi::c_int {
            free(LocalHostName as *mut ::core::ffi::c_void);
            LocalHostName = strdup((*rp).ai_canonname);
            p = strchr(LocalHostName, '.' as ::core::ffi::c_int);
            if !p.is_null() {
                let c2rust_fresh17 = p;
                p = p.offset(1);
                *c2rust_fresh17 = '\0' as ::core::ffi::c_char;
                LocalDomain = p;
            }
            freeaddrinfo(rp);
        }
        if LocalDomain.is_null() {
            LocalDomain = strdup(b"\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    consfile.f_type = F_CONSOLE as ::core::ffi::c_short;
    consfile.f_un.f_fname = strdup(&raw mut ctty as *mut ::core::ffi::c_char);
    signal(
        SIGTERM,
        Some(die as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGINT,
        if NoDetach != 0 {
            Some(die as unsafe extern "C" fn(::core::ffi::c_int) -> ())
        } else {
            ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            )
        },
    );
    signal(
        SIGQUIT,
        if NoDetach != 0 {
            Some(die as unsafe extern "C" fn(::core::ffi::c_int) -> ())
        } else {
            ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
                1 as ::core::ffi::c_int as ::libc::intptr_t,
            )
        },
    );
    sa.sa_flags = SA_RESTART;
    sigemptyset(&raw mut sa.sa_mask);
    sa.__sigaction_handler.sa_handler =
        Some(domark as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
    sigaction(SIGALRM, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
    sa.__sigaction_handler.sa_handler = (if NoDetach != 0 {
        Some(dbg_toggle as unsafe extern "C" fn(::core::ffi::c_int) -> ())
    } else {
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t)
    }) as __sighandler_t;
    sigaction(SIGUSR1, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
    alarm(TIMERINTVL as ::core::ffi::c_uint);
    fdarray = malloc(
        nfunix
            .wrapping_add(3 as size_t)
            .wrapping_mul(::core::mem::size_of::<pollfd>()),
    ) as *mut pollfd;
    if fdarray.is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"can't allocate fd table\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"can't allocate fd table\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    init(0 as ::core::ffi::c_int);
    if NoKLog == 0 {
        fklog = open(PATH_KLOG.as_ptr(), O_RDONLY, 0 as ::core::ffi::c_int);
        if fklog >= 0 as ::core::ffi::c_int {
            (*fdarray.offset(nfds as isize)).fd = fklog;
            (*fdarray.offset(nfds as isize)).events = (POLLIN | POLLPRI) as ::core::ffi::c_short;
            nfds = nfds.wrapping_add(1);
            dbg_printf(
                b"Klog open %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                PATH_KLOG.as_ptr(),
            );
        } else {
            dbg_printf(
                b"Can't open %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                PATH_KLOG.as_ptr(),
                strerror(*__errno_location()),
            );
        }
    }
    if NoUnixAF == 0 {
        i = 0 as size_t;
        while i < nfunix {
            (*funix.offset(i as isize)).fd = create_unix_socket((*funix.offset(i as isize)).name);
            if (*funix.offset(i as isize)).fd >= 0 as ::core::ffi::c_int {
                (*fdarray.offset(nfds as isize)).fd = (*funix.offset(i as isize)).fd;
                (*fdarray.offset(nfds as isize)).events =
                    (POLLIN | POLLPRI) as ::core::ffi::c_short;
                nfds = nfds.wrapping_add(1);
                dbg_printf(
                    b"Opened UNIX socket `%s'.\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*funix.offset(i as isize)).name,
                );
            } else {
                dbg_printf(
                    b"Can't open %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*funix.offset(i as isize)).name,
                    strerror(*__errno_location()),
                );
            }
            i = i.wrapping_add(1);
        }
    }
    if AcceptRemote != 0 {
        create_inet_socket(usefamily, &raw mut finet as *mut ::core::ffi::c_int);
        if finet[IU_FD_IP4 as usize] >= 0 as ::core::ffi::c_int {
            (*fdarray.offset(nfds as isize)).fd = finet[IU_FD_IP4 as usize];
            (*fdarray.offset(nfds as isize)).events = (POLLIN | POLLPRI) as ::core::ffi::c_short;
            nfds = nfds.wrapping_add(1);
            dbg_printf(b"Opened syslog UDP/IPv4 port.\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if finet[IU_FD_IP6 as usize] >= 0 as ::core::ffi::c_int {
            (*fdarray.offset(nfds as isize)).fd = finet[IU_FD_IP6 as usize];
            (*fdarray.offset(nfds as isize)).events = (POLLIN | POLLPRI) as ::core::ffi::c_short;
            nfds = nfds.wrapping_add(1);
            dbg_printf(b"Opened syslog UDP/IPv6 port.\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if finet[IU_FD_IP4 as usize] < 0 as ::core::ffi::c_int
            && finet[IU_FD_IP6 as usize] < 0 as ::core::ffi::c_int
        {
            dbg_printf(
                b"Can't open UDP port: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                strerror(*__errno_location()),
            );
        }
    }
    fp = fopen(PidFile, b"w\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if !fp.is_null() {
        fprintf(
            fp,
            b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
            getpid(),
        );
        fclose(fp);
    }
    dbg_printf(b"off & running....\n\0".as_ptr() as *const ::core::ffi::c_char);
    sa.__sigaction_handler.sa_handler =
        Some(trigger_restart as unsafe extern "C" fn(::core::ffi::c_int) -> ()) as __sighandler_t;
    sigaction(SIGHUP, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
    if NoDetach != 0 {
        dbg_output = 1 as ::core::ffi::c_int;
        dbg_printf(
            b"Debugging is disabled. Send SIGUSR1 to PID=%d to turn on debugging.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            getpid(),
        );
        dbg_output = 0 as ::core::ffi::c_int;
    }
    if ppid != 0 {
        kill(ppid, SIGTERM);
    }
    loop {
        let mut nready: ::core::ffi::c_int = 0;
        nready = poll(fdarray, nfds, -1 as ::core::ffi::c_int);
        if nready == 0 as ::core::ffi::c_int {
            continue;
        }
        if restart != 0 {
            dbg_printf(b"\nReceived SIGHUP, restarting syslogd.\n\0".as_ptr()
                as *const ::core::ffi::c_char);
            init(0 as ::core::ffi::c_int);
            restart = 0 as ::core::ffi::c_int;
        } else if nready < 0 as ::core::ffi::c_int {
            if *__errno_location() != EINTR {
                logerror(b"poll\0".as_ptr() as *const ::core::ffi::c_char);
            }
        } else {
            i = 0 as size_t;
            while i < nfds as size_t {
                if (*fdarray.offset(i as isize)).revents as ::core::ffi::c_int & (POLLIN | POLLPRI)
                    != 0
                {
                    let mut result: ::core::ffi::c_int = 0;
                    let mut len: socklen_t = 0;
                    if (*fdarray.offset(i as isize)).fd != -1 as ::core::ffi::c_int {
                        if (*fdarray.offset(i as isize)).fd == fklog {
                            result = read(
                                (*fdarray.offset(i as isize)).fd,
                                (&raw mut kline as *mut ::core::ffi::c_char)
                                    .offset(kline_len as isize)
                                    as *mut ::core::ffi::c_void,
                                ::core::mem::size_of::<[::core::ffi::c_char; 1025]>()
                                    .wrapping_sub(kline_len as size_t)
                                    .wrapping_sub(1 as size_t),
                            ) as ::core::ffi::c_int;
                            if result > 0 as ::core::ffi::c_int {
                                kline_len += result;
                            } else if result < 0 as ::core::ffi::c_int
                                && *__errno_location() != EINTR
                            {
                                logerror(b"klog\0".as_ptr() as *const ::core::ffi::c_char);
                                fklog = -1 as ::core::ffi::c_int;
                                (*fdarray.offset(i as isize)).fd = fklog;
                            }
                            loop {
                                let mut bol: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                let mut eol: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                kline[kline_len as usize] = '\0' as ::core::ffi::c_char;
                                bol = &raw mut kline as *mut ::core::ffi::c_char;
                                eol = strchr(
                                    &raw mut kline as *mut ::core::ffi::c_char,
                                    '\n' as ::core::ffi::c_int,
                                );
                                while !eol.is_null() {
                                    let c2rust_fresh18 = eol;
                                    eol = eol.offset(1);
                                    *c2rust_fresh18 = '\0' as ::core::ffi::c_char;
                                    kline_len = (kline_len as ::core::ffi::c_long
                                        - eol.offset_from(bol) as ::core::ffi::c_long)
                                        as ::core::ffi::c_int;
                                    printsys(bol);
                                    bol = eol;
                                    eol = strchr(bol, '\n' as ::core::ffi::c_int);
                                }
                                while kline_len != 0 && *bol == 0 {
                                    bol = bol.offset(1);
                                    kline_len -= 1;
                                }
                                if kline_len == 0 {
                                    break;
                                }
                                if bol != &raw mut kline as *mut ::core::ffi::c_char {
                                    memmove(
                                        &raw mut kline as *mut ::core::ffi::c_char
                                            as *mut ::core::ffi::c_void,
                                        bol as *const ::core::ffi::c_void,
                                        kline_len as size_t,
                                    );
                                } else {
                                    if kline_len < MAXLINE {
                                        break;
                                    }
                                    printsys(&raw mut kline as *mut ::core::ffi::c_char);
                                    if kline[0usize] as ::core::ffi::c_int
                                        == '<' as ::core::ffi::c_int
                                        && *(*__ctype_b_loc())
                                            .offset(kline[1usize] as ::core::ffi::c_int as isize)
                                            as ::core::ffi::c_int
                                            & C2Rust_Unnamed_12::_ISdigit.0 as ::core::ffi::c_int
                                                as ::core::ffi::c_ushort
                                                as ::core::ffi::c_int
                                            != 0
                                        && kline[2usize] as ::core::ffi::c_int
                                            == '>' as ::core::ffi::c_int
                                    {
                                        kline_len = 3 as ::core::ffi::c_int;
                                    } else {
                                        kline_len = 0 as ::core::ffi::c_int;
                                    }
                                }
                            }
                        } else if (*fdarray.offset(i as isize)).fd == finet[IU_FD_IP4 as usize]
                            || (*fdarray.offset(i as isize)).fd == finet[IU_FD_IP6 as usize]
                        {
                            let mut frominet: sockaddr_storage = sockaddr_storage {
                                ss_family: 0,
                                __ss_padding: [0; 118],
                                __ss_align: 0,
                            };
                            len = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
                            memset(
                                &raw mut line as *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                '\0' as ::core::ffi::c_int,
                                ::core::mem::size_of::<[::core::ffi::c_char; 1025]>(),
                            );
                            result = recvfrom(
                                (*fdarray.offset(i as isize)).fd,
                                &raw mut line as *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                MAXLINE as size_t,
                                0 as ::core::ffi::c_int,
                                __SOCKADDR_ARG {
                                    __sockaddr__: &raw mut frominet as *mut sockaddr,
                                },
                                &raw mut len,
                            ) as ::core::ffi::c_int;
                            if result > 0 as ::core::ffi::c_int {
                                line[result as usize] = '\0' as ::core::ffi::c_char;
                                printline(
                                    cvthname(&raw mut frominet as *mut sockaddr, len),
                                    &raw mut line as *mut ::core::ffi::c_char,
                                );
                            } else if result < 0 as ::core::ffi::c_int
                                && *__errno_location() != EINTR
                            {
                                logerror(b"recvfrom inet\0".as_ptr() as *const ::core::ffi::c_char);
                            }
                        } else {
                            let mut fromunix: sockaddr_un = sockaddr_un {
                                sun_family: 0,
                                sun_path: [0; 108],
                            };
                            len = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
                            result = recvfrom(
                                (*fdarray.offset(i as isize)).fd,
                                &raw mut line as *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                MAXLINE as size_t,
                                0 as ::core::ffi::c_int,
                                __SOCKADDR_ARG {
                                    __sockaddr__: &raw mut fromunix as *mut sockaddr,
                                },
                                &raw mut len,
                            ) as ::core::ffi::c_int;
                            if result > 0 as ::core::ffi::c_int {
                                line[result as usize] = '\0' as ::core::ffi::c_char;
                                printline(LocalHostName, &raw mut line as *mut ::core::ffi::c_char);
                            } else if result < 0 as ::core::ffi::c_int
                                && *__errno_location() != EINTR
                            {
                                logerror(b"recvfrom unix\0".as_ptr() as *const ::core::ffi::c_char);
                            }
                        }
                    }
                } else if (*fdarray.offset(i as isize)).revents as ::core::ffi::c_int & POLLNVAL
                    != 0
                {
                    logerror(b"poll nval\n\0".as_ptr() as *const ::core::ffi::c_char);
                    (*fdarray.offset(i as isize)).fd = -1 as ::core::ffi::c_int;
                } else if (*fdarray.offset(i as isize)).revents as ::core::ffi::c_int & POLLERR != 0
                {
                    logerror(b"poll err\n\0".as_ptr() as *const ::core::ffi::c_char);
                } else if (*fdarray.offset(i as isize)).revents as ::core::ffi::c_int & POLLHUP != 0
                {
                    logerror(b"poll hup\n\0".as_ptr() as *const ::core::ffi::c_char);
                }
                i = i.wrapping_add(1);
            }
        }
    }
}
unsafe extern "C" fn add_funix(mut name: *const ::core::ffi::c_char) {
    funix = rpl_realloc(
        funix as *mut ::core::ffi::c_void,
        nfunix
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<funix>()),
    ) as *mut funix;
    if funix.is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot allocate space for unix sockets\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"cannot allocate space for unix sockets\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    (*funix.offset(nfunix as isize)).name = name;
    (*funix.offset(nfunix as isize)).fd = -1 as ::core::ffi::c_int;
    nfunix = nfunix.wrapping_add(1);
}
unsafe extern "C" fn create_unix_socket(
    mut path: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut sunx: sockaddr_un = sockaddr_un {
        sun_family: 0,
        sun_path: [0; 108],
    };
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    if *path.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    if strlen(path) >= ::core::mem::size_of::<[::core::ffi::c_char; 108]>() {
        snprintf(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1025]>(),
            b"UNIX socket name too long: %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
        );
        logerror(&raw mut line as *mut ::core::ffi::c_char);
        return -1 as ::core::ffi::c_int;
    }
    unlink(path);
    memset(
        &raw mut sunx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_un>(),
    );
    sunx.sun_family = AF_UNIX as sa_family_t;
    strncpy(
        &raw mut sunx.sun_path as *mut ::core::ffi::c_char,
        path,
        ::core::mem::size_of::<[::core::ffi::c_char; 108]>().wrapping_sub(1 as size_t),
    );
    fd = socket(
        AF_UNIX,
        __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int
        || bind(
            fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut sunx as *mut sockaddr,
            },
            (2 as size_t).wrapping_add(strlen(&raw mut sunx.sun_path as *mut ::core::ffi::c_char))
                as socklen_t,
        ) < 0 as ::core::ffi::c_int
        || lchmod(path, 0o666 as __mode_t) < 0 as ::core::ffi::c_int
    {
        snprintf(
            &raw mut line as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1025]>(),
            b"cannot create %s\0".as_ptr() as *const ::core::ffi::c_char,
            path,
        );
        logerror(&raw mut line as *mut ::core::ffi::c_char);
        dbg_printf(
            b"cannot create %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            path,
            strerror(*__errno_location()),
        );
        close(fd);
        fd = -1 as ::core::ffi::c_int;
    }
    return fd;
}
unsafe extern "C" fn create_inet_socket(
    mut af: ::core::ffi::c_int,
    mut fd46: *mut ::core::ffi::c_int,
) {
    let mut err: ::core::ffi::c_int = 0;
    let mut fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    let mut rp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut ai: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    *fd46.offset(IU_FD_IP6 as isize) = -1 as ::core::ffi::c_int;
    *fd46.offset(IU_FD_IP4 as isize) = *fd46.offset(IU_FD_IP6 as isize);
    if LogPortText.is_null() {
        dbg_printf(b"No listen port has been accepted.\n\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_family = af;
    hints.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
    hints.ai_flags = AI_PASSIVE;
    err = getaddrinfo(BindAddress, LogPortText, &raw mut hints, &raw mut rp);
    if err != 0 {
        logerror(b"lookup error, suspending inet service\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
    ai = rp;
    while !ai.is_null() {
        let mut yes: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        fd = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
        if fd >= 0 as ::core::ffi::c_int {
            err = setsockopt(
                fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                &raw mut yes as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
            );
            if err < 0 as ::core::ffi::c_int {
                logerror(b"failed to set SO_REUSEADDR\0".as_ptr() as *const ::core::ffi::c_char);
            }
            if (*ai).ai_family == AF_INET6 {
                setsockopt(
                    fd,
                    C2Rust_Unnamed_11::IPPROTO_IPV6.0 as ::core::ffi::c_int,
                    IPV6_V6ONLY,
                    &raw mut yes as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                );
            }
            if bind(
                fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: (*ai).ai_addr,
                },
                (*ai).ai_addrlen,
            ) < 0 as ::core::ffi::c_int
            {
                close(fd);
                fd = -1 as ::core::ffi::c_int;
            } else if (*ai).ai_family == AF_INET
                && *fd46.offset(IU_FD_IP4 as isize) < 0 as ::core::ffi::c_int
            {
                *fd46.offset(IU_FD_IP4 as isize) = fd;
            } else if (*ai).ai_family == AF_INET6
                && *fd46.offset(IU_FD_IP6 as isize) < 0 as ::core::ffi::c_int
            {
                *fd46.offset(IU_FD_IP6 as isize) = fd;
            }
        }
        ai = (*ai).ai_next;
    }
    freeaddrinfo(rp);
    if *fd46.offset(IU_FD_IP4 as isize) < 0 as ::core::ffi::c_int
        && *fd46.offset(IU_FD_IP6 as isize) < 0 as ::core::ffi::c_int
    {
        logerror(b"inet service, failed lookup.\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
}
#[export_name = "rboxc_syslogd_crunch_list"]
pub unsafe extern "C" fn crunch_list(
    mut oldlist: *mut *mut ::core::ffi::c_char,
    mut list: *mut ::core::ffi::c_char,
) -> *mut *mut ::core::ffi::c_char {
    let mut count: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = list;
    while *p.offset(strlen(p).wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
        == LIST_DELIMITER
    {
        *p.offset(strlen(p).wrapping_sub(1 as size_t) as isize) = '\0' as ::core::ffi::c_char;
    }
    while *p.offset(0isize) as ::core::ffi::c_int == LIST_DELIMITER {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return oldlist;
    }
    count = 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while *p.offset(i as isize) != 0 {
        if *p.offset(i as isize) as ::core::ffi::c_int == LIST_DELIMITER {
            count += 1;
        }
        i += 1;
    }
    i = 0 as ::core::ffi::c_int;
    while !oldlist.is_null() && !(*oldlist.offset(i as isize)).is_null() {
        i += 1;
    }
    oldlist = rpl_realloc(
        oldlist as *mut ::core::ffi::c_void,
        ((i + count + 1 as ::core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    ) as *mut *mut ::core::ffi::c_char;
    if oldlist.is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"can't allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"can't allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    count = i;
    loop {
        q = strchr(p, ':' as ::core::ffi::c_int);
        if q.is_null() {
            break;
        }
        *oldlist.offset(count as isize) = malloc(
            ((q.offset_from(p) + 1isize) as size_t)
                .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>()),
        ) as *mut ::core::ffi::c_char;
        if (*oldlist.offset(count as isize)).is_null() {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"can't allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"can't allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        strncpy(
            *oldlist.offset(count as isize),
            p,
            q.offset_from(p) as size_t,
        );
        *(*oldlist.offset(count as isize)).offset(q.offset_from(p) as isize) =
            '\0' as ::core::ffi::c_char;
        count += 1;
        p = q;
        p = p.offset(1);
    }
    *oldlist.offset(count as isize) = xmalloc(
        strlen(p)
            .wrapping_add(1 as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>()),
    ) as *mut ::core::ffi::c_char;
    if (*oldlist.offset(count as isize)).is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"can't allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"can't allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    strcpy(*oldlist.offset(count as isize), p);
    count += 1;
    *oldlist.offset(count as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if Debug != 0 {
        count = 0 as ::core::ffi::c_int;
        while !(*oldlist.offset(count as isize)).is_null() {
            printf(
                b"#%d: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                count,
                *oldlist.offset(count as isize),
            );
            count += 1;
        }
    }
    return oldlist;
}
#[export_name = "rboxc_syslogd_printline"]
pub unsafe extern "C" fn printline(
    mut hname: *const ::core::ffi::c_char,
    mut msg: *const ::core::ffi::c_char,
) {
    let mut c: ::core::ffi::c_int = 0;
    let mut pri: ::core::ffi::c_int = 0;
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line: [::core::ffi::c_char; 1026] = [0; 1026];
    pri = DEFUPRI;
    p = msg;
    if *p as ::core::ffi::c_int == '<' as ::core::ffi::c_int {
        pri = 0 as ::core::ffi::c_int;
        loop {
            p = p.offset(1);
            if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & C2Rust_Unnamed_12::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                == 0
            {
                break;
            }
            pri = 10 as ::core::ffi::c_int * pri
                + (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
        }
        if *p as ::core::ffi::c_int == '>' as ::core::ffi::c_int {
            p = p.offset(1);
        }
    }
    if pri & !(LOG_FACMASK | LOG_PRIMASK) != 0 {
        pri = DEFUPRI;
    }
    if (pri & LOG_FACMASK) >> 3 as ::core::ffi::c_int > LOG_NFACILITIES {
        pri = DEFUPRI;
    }
    if (pri & LOG_FACMASK) >> 3 as ::core::ffi::c_int == LOG_KERN >> 3 as ::core::ffi::c_int {
        pri =
            (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int | pri & 0x7 as ::core::ffi::c_int;
    }
    q = &raw mut line as *mut ::core::ffi::c_char;
    loop {
        let c2rust_fresh7 = p;
        p = p.offset(1);
        c = *c2rust_fresh7 as ::core::ffi::c_int;
        if !(c != '\0' as ::core::ffi::c_int
            && q < (&raw mut line as *mut ::core::ffi::c_char).offset(
                ::core::mem::size_of::<[::core::ffi::c_char; 1026]>()
                    .wrapping_sub(1usize)
                    .wrapping_sub(1usize) as isize,
            ))
        {
            break;
        }
        if *(*__ctype_b_loc()).offset(c as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed_12::_IScntrl.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            if c == '\n' as ::core::ffi::c_int {
                let c2rust_fresh8 = q;
                q = q.offset(1);
                *c2rust_fresh8 = ' ' as ::core::ffi::c_char;
            } else if c == '\t' as ::core::ffi::c_int {
                let c2rust_fresh9 = q;
                q = q.offset(1);
                *c2rust_fresh9 = '\t' as ::core::ffi::c_char;
            } else if c > 0o177 as ::core::ffi::c_int {
                let c2rust_fresh10 = q;
                q = q.offset(1);
                *c2rust_fresh10 = c as ::core::ffi::c_char;
            } else {
                let c2rust_fresh11 = q;
                q = q.offset(1);
                *c2rust_fresh11 = '^' as ::core::ffi::c_char;
                let c2rust_fresh12 = q;
                q = q.offset(1);
                *c2rust_fresh12 = (c ^ 0o100 as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
        } else {
            let c2rust_fresh13 = q;
            q = q.offset(1);
            *c2rust_fresh13 = c as ::core::ffi::c_char;
        }
    }
    *q = '\0' as ::core::ffi::c_char;
    if force_sync != 0 {
        logmsg(
            pri,
            &raw mut line as *mut ::core::ffi::c_char,
            hname,
            SYNC_FILE,
        );
    } else {
        logmsg(
            pri,
            &raw mut line as *mut ::core::ffi::c_char,
            hname,
            0 as ::core::ffi::c_int,
        );
    };
}
#[export_name = "rboxc_syslogd_printsys"]
pub unsafe extern "C" fn printsys(mut msg: *const ::core::ffi::c_char) {
    let mut c: ::core::ffi::c_int = 0;
    let mut pri: ::core::ffi::c_int = 0;
    let mut flags: ::core::ffi::c_int = 0;
    let mut lp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    strcpy(
        &raw mut line as *mut ::core::ffi::c_char,
        b"vmunix: \0".as_ptr() as *const ::core::ffi::c_char,
    );
    lp = (&raw mut line as *mut ::core::ffi::c_char)
        .offset(strlen(&raw mut line as *mut ::core::ffi::c_char) as isize);
    p = msg;
    while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        flags = SYNC_FILE | ADDDATE;
        pri = DEFSPRI;
        if *p as ::core::ffi::c_int == '<' as ::core::ffi::c_int {
            pri = 0 as ::core::ffi::c_int;
            loop {
                p = p.offset(1);
                if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed_12::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    == 0
                {
                    break;
                }
                pri = 10 as ::core::ffi::c_int * pri
                    + (*p as ::core::ffi::c_int - '0' as ::core::ffi::c_int);
            }
            if *p as ::core::ffi::c_int == '>' as ::core::ffi::c_int {
                p = p.offset(1);
            }
        } else {
            flags |= IGN_CONS;
        }
        if pri & !(LOG_FACMASK | LOG_PRIMASK) != 0 {
            pri = DEFSPRI;
        }
        q = lp;
        while *p as ::core::ffi::c_int != '\0' as ::core::ffi::c_int
            && {
                let c2rust_fresh14 = p;
                p = p.offset(1);
                c = *c2rust_fresh14 as ::core::ffi::c_int;
                c != '\n' as ::core::ffi::c_int
            }
            && q < (&raw mut line as *mut ::core::ffi::c_char).offset(MAXLINE as isize)
        {
            let c2rust_fresh15 = q;
            q = q.offset(1);
            *c2rust_fresh15 = c as ::core::ffi::c_char;
        }
        *q = '\0' as ::core::ffi::c_char;
        logmsg(
            pri,
            &raw mut line as *mut ::core::ffi::c_char,
            LocalHostName,
            flags,
        );
    }
}
#[export_name = "rboxc_syslogd_textpri"]
pub unsafe extern "C" fn textpri(mut pri: ::core::ffi::c_int) -> *mut ::core::ffi::c_char {
    static mut res: [::core::ffi::c_char; 20] = [0; 20];
    let mut c_pri: *mut CODE = ::core::ptr::null_mut::<CODE>();
    let mut c_fac: *mut CODE = ::core::ptr::null_mut::<CODE>();
    c_fac = &raw mut facilitynames as *mut CODE;
    while !(*c_fac).c_name.is_null()
        && !((*c_fac).c_val
            == ((pri & LOG_FACMASK) >> 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
    {
        c_fac = c_fac.offset(1);
    }
    c_pri = &raw mut prioritynames as *mut CODE;
    while !(*c_pri).c_name.is_null() && !((*c_pri).c_val == pri & LOG_PRIMASK) {
        c_pri = c_pri.offset(1);
    }
    snprintf(
        &raw mut res as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 20]>(),
        b"%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*c_fac).c_name,
        (*c_pri).c_name,
    );
    return &raw mut res as *mut ::core::ffi::c_char;
}
#[export_name = "rboxc_syslogd_logmsg"]
pub unsafe extern "C" fn logmsg(
    mut pri: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
    mut from: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
) {
    let mut f: *mut filed = ::core::ptr::null_mut::<filed>();
    let mut fac: ::core::ffi::c_int = 0;
    let mut msglen: ::core::ffi::c_int = 0;
    let mut prilev: ::core::ffi::c_int = 0;
    let mut sigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut osigs: sigset_t = sigset_t { __val: [0; 16] };
    let mut timestamp: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    dbg_printf(
        b"(logmsg): %s (%d), flags %x, from %s, msg %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        textpri(pri),
        pri,
        flags,
        from,
        msg,
    );
    sigemptyset(&raw mut sigs);
    sigaddset(&raw mut sigs, SIGHUP);
    sigaddset(&raw mut sigs, SIGALRM);
    sigprocmask(SIG_BLOCK, &raw mut sigs, &raw mut osigs);
    msglen = strlen(msg) as ::core::ffi::c_int;
    if msglen < 16 as ::core::ffi::c_int
        || *msg.offset(3isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        || *msg.offset(6isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        || *msg.offset(9isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        || *msg.offset(12isize) as ::core::ffi::c_int != ':' as ::core::ffi::c_int
        || *msg.offset(15isize) as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        flags |= ADDDATE;
    }
    time(&raw mut now);
    if flags & ADDDATE != 0 {
        timestamp = ctime(&raw mut now).offset(4 as ::core::ffi::c_int as isize);
    } else {
        if set_local_time != 0 {
            timestamp = ctime(&raw mut now).offset(4 as ::core::ffi::c_int as isize);
        } else {
            timestamp = msg;
        }
        msg = msg.offset(16 as ::core::ffi::c_int as isize);
        msglen -= 16 as ::core::ffi::c_int;
    }
    if flags & MARK != 0 {
        fac = (((24 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int | 0 as ::core::ffi::c_int)
            & LOG_FACMASK)
            >> 3 as ::core::ffi::c_int;
    } else {
        fac = (pri & LOG_FACMASK) >> 3 as ::core::ffi::c_int;
    }
    prilev = pri & LOG_PRIMASK;
    if Initialized == 0 {
        f = &raw mut consfile;
        (*f).f_file = open(
            &raw mut ctty as *mut ::core::ffi::c_char,
            O_WRONLY,
            0 as ::core::ffi::c_int,
        ) as ::core::ffi::c_short;
        (*f).f_prevhost = strdup(LocalHostName);
        if (*f).f_file as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
            fprintlog(f, from, flags, msg);
            close((*f).f_file as ::core::ffi::c_int);
        }
        sigprocmask(
            SIG_SETMASK,
            &raw mut osigs,
            ::core::ptr::null_mut::<sigset_t>(),
        );
        return;
    }
    f = Files;
    while !f.is_null() {
        's_124: {
            if (*f).f_pmask[fac as usize] as ::core::ffi::c_int
                & (1 as ::core::ffi::c_int) << prilev
                != 0
            {
                if !((*f).f_type as ::core::ffi::c_int == F_CONSOLE && flags & IGN_CONS != 0) {
                    if !(flags & MARK != 0
                        && now - (*f).f_time < (MarkInterval / 2 as ::core::ffi::c_int) as time_t)
                    {
                        if !(*f).f_progname.is_null() {
                            if strncmp(msg, (*f).f_progname, (*f).f_prognlen as size_t) != 0 {
                                break 's_124;
                            } else if *(*__ctype_b_loc())
                                .offset(*msg.offset((*f).f_prognlen as isize) as ::core::ffi::c_int
                                    as isize)
                                as ::core::ffi::c_int
                                & C2Rust_Unnamed_12::_ISalnum.0 as ::core::ffi::c_int
                                    as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0
                                || *msg.offset((*f).f_prognlen as isize) as ::core::ffi::c_int
                                    == '-' as ::core::ffi::c_int
                                || *msg.offset((*f).f_prognlen as isize) as ::core::ffi::c_int
                                    == '_' as ::core::ffi::c_int
                            {
                                break 's_124;
                            }
                        }
                        if flags & MARK == 0 as ::core::ffi::c_int
                            && msglen == (*f).f_prevlen
                            && !(*f).f_prevhost.is_null()
                            && strcmp(msg, &raw mut (*f).f_prevline as *mut ::core::ffi::c_char)
                                == 0
                            && strcmp(from, (*f).f_prevhost) == 0
                        {
                            strncpy(
                                &raw mut (*f).f_lasttime as *mut ::core::ffi::c_char,
                                timestamp,
                                ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                    .wrapping_sub(1 as size_t),
                            );
                            (*f).f_prevcount += 1;
                            dbg_printf(
                                b"msg repeated %d times, %ld sec of %d\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*f).f_prevcount,
                                now - (*f).f_time,
                                repeatinterval[(*f).f_repeatcount],
                            );
                            if now > (*f).f_time + repeatinterval[(*f).f_repeatcount] as time_t {
                                fprintlog(f, from, flags, NULL as *mut ::core::ffi::c_char);
                                (*f).f_repeatcount = (*f).f_repeatcount.wrapping_add(1);
                                if (*f).f_repeatcount > MAXREPEAT {
                                    (*f).f_repeatcount = MAXREPEAT as size_t;
                                }
                            }
                        } else {
                            if (*f).f_prevcount != 0 {
                                fprintlog(
                                    f,
                                    from,
                                    0 as ::core::ffi::c_int,
                                    NULL as *mut ::core::ffi::c_char,
                                );
                            }
                            (*f).f_repeatcount = 0 as size_t;
                            strncpy(
                                &raw mut (*f).f_lasttime as *mut ::core::ffi::c_char,
                                timestamp,
                                ::core::mem::size_of::<[::core::ffi::c_char; 16]>()
                                    .wrapping_sub(1 as size_t),
                            );
                            free((*f).f_prevhost as *mut ::core::ffi::c_void);
                            (*f).f_prevhost = strdup(from);
                            if msglen < MAXSVLINE {
                                (*f).f_prevlen = msglen;
                                (*f).f_prevpri = pri;
                                strcpy(&raw mut (*f).f_prevline as *mut ::core::ffi::c_char, msg);
                                fprintlog(f, from, flags, NULL as *mut ::core::ffi::c_char);
                            } else {
                                (*f).f_prevline[0usize] = 0 as ::core::ffi::c_char;
                                (*f).f_prevlen = 0 as ::core::ffi::c_int;
                                fprintlog(f, from, flags, msg);
                            }
                        }
                    }
                }
            }
        }
        f = (*f).f_next;
    }
    sigprocmask(
        SIG_SETMASK,
        &raw mut osigs,
        ::core::ptr::null_mut::<sigset_t>(),
    );
}
#[export_name = "rboxc_syslogd_fprintlog"]
pub unsafe extern "C" fn fprintlog(
    mut f: *mut filed,
    mut from: *const ::core::ffi::c_char,
    mut flags: ::core::ffi::c_int,
    mut msg: *const ::core::ffi::c_char,
) {
    let mut iov: [iovec; 6] = [iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    }; 6];
    let mut v: *mut iovec = ::core::ptr::null_mut::<iovec>();
    let mut l: ::core::ffi::c_int = 0;
    let mut line: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut repbuf: [::core::ffi::c_char; 80] = [0; 80];
    let mut greetings: [::core::ffi::c_char; 200] = [0; 200];
    let mut fwd_suspend: time_t = 0;
    v = &raw mut iov as *mut iovec;
    memset(
        v as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<iovec>().wrapping_mul(IOVCNT as size_t),
    );
    if (*f).f_type as ::core::ffi::c_int == F_WALL {
        (*v).iov_base = &raw mut greetings as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        snprintf(
            &raw mut greetings as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 200]>(),
            b"\r\n\x07Message from syslogd@%s at %.24s ...\r\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*f).f_prevhost,
            ctime(&raw mut now),
        );
        (*v).iov_len = strlen(&raw mut greetings as *mut ::core::ffi::c_char);
        v = v.offset(1);
        (*v).iov_base = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void;
        (*v).iov_len = 0 as size_t;
        v = v.offset(1);
    } else {
        (*v).iov_base =
            &raw mut (*f).f_lasttime as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*v).iov_len =
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>().wrapping_sub(1usize) as size_t;
        v = v.offset(1);
        (*v).iov_base = b" \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void;
        (*v).iov_len = 1 as size_t;
        v = v.offset(1);
    }
    if !(*f).f_prevhost.is_null() {
        (*v).iov_base = (*f).f_prevhost as *mut ::core::ffi::c_void;
        (*v).iov_len = strlen((*v).iov_base as *const ::core::ffi::c_char);
        v = v.offset(1);
    }
    (*v).iov_base = b" \0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
        as *mut ::core::ffi::c_void;
    (*v).iov_len = 1 as size_t;
    v = v.offset(1);
    if !msg.is_null() {
        (*v).iov_base = msg as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*v).iov_len = strlen(msg);
    } else if (*f).f_prevcount > 1 as ::core::ffi::c_int {
        (*v).iov_base = &raw mut repbuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        snprintf(
            &raw mut repbuf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
            b"last message repeated %d times\0".as_ptr() as *const ::core::ffi::c_char,
            (*f).f_prevcount,
        );
        (*v).iov_len = strlen(&raw mut repbuf as *mut ::core::ffi::c_char);
    } else {
        (*v).iov_base =
            &raw mut (*f).f_prevline as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*v).iov_len = (*f).f_prevlen as size_t;
    }
    v = v.offset(1);
    dbg_printf(
        b"Logging to %s\0".as_ptr() as *const ::core::ffi::c_char,
        TypeNames[(*f).f_type as usize],
    );
    's_606: {
        '_f_forw: {
            match (*f).f_type as ::core::ffi::c_int {
                F_UNUSED => {
                    (*f).f_time = now;
                    dbg_printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    break 's_606;
                }
                F_FORW_SUSP => {
                    fwd_suspend = time(::core::ptr::null_mut::<time_t>()) - (*f).f_time;
                    if fwd_suspend >= INET_SUSPEND_TIME as time_t {
                        dbg_printf(b"\nForwarding suspension over, retrying FORW \0".as_ptr()
                            as *const ::core::ffi::c_char);
                        (*f).f_type = F_FORW as ::core::ffi::c_short;
                        break '_f_forw;
                    } else {
                        dbg_printf(
                            b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            (*f).f_un.f_forw.f_hname,
                        );
                        dbg_printf(
                            b"Forwarding suspension not over, time left: %d.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            INET_SUSPEND_TIME as time_t - fwd_suspend,
                        );
                        break 's_606;
                    }
                }
                F_FORW_UNKN => {
                    dbg_printf(
                        b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*f).f_un.f_forw.f_hname,
                    );
                    fwd_suspend = time(::core::ptr::null_mut::<time_t>()) - (*f).f_time;
                    if fwd_suspend >= INET_SUSPEND_TIME as time_t {
                        let mut hints: addrinfo = addrinfo {
                            ai_flags: 0,
                            ai_family: 0,
                            ai_socktype: 0,
                            ai_protocol: 0,
                            ai_addrlen: 0,
                            ai_addr: ::core::ptr::null_mut::<sockaddr>(),
                            ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            ai_next: ::core::ptr::null_mut::<addrinfo>(),
                        };
                        let mut rp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
                        let mut err: ::core::ffi::c_int = 0;
                        memset(
                            &raw mut hints as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<addrinfo>(),
                        );
                        hints.ai_family = usefamily;
                        if usefamily == AF_UNSPEC {
                            hints.ai_flags |= AI_ADDRCONFIG;
                        }
                        err = getaddrinfo(
                            (*f).f_un.f_forw.f_hname,
                            LogForwardPort,
                            &raw mut hints,
                            &raw mut rp,
                        );
                        if err != 0 {
                            dbg_printf(
                                b"Failure: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                gai_strerror(err),
                            );
                            dbg_printf(
                                b"Retries: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                                (*f).f_prevcount,
                            );
                            (*f).f_prevcount -= 1;
                            if (*f).f_prevcount < 0 as ::core::ffi::c_int {
                                (*f).f_type = F_UNUSED as ::core::ffi::c_short;
                                free((*f).f_un.f_forw.f_hname as *mut ::core::ffi::c_void);
                                (*f).f_un.f_forw.f_hname =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                            }
                            break 's_606;
                        } else {
                            dbg_printf(
                                b"%s found, resuming.\n\0".as_ptr() as *const ::core::ffi::c_char,
                                (*f).f_un.f_forw.f_hname,
                            );
                            (*f).f_un.f_forw.f_addrlen = (*rp).ai_addrlen;
                            memcpy(
                                &raw mut (*f).f_un.f_forw.f_addr as *mut ::core::ffi::c_void,
                                (*rp).ai_addr as *const ::core::ffi::c_void,
                                (*rp).ai_addrlen as size_t,
                            );
                            freeaddrinfo(rp);
                            (*f).f_prevcount = 0 as ::core::ffi::c_int;
                            (*f).f_type = F_FORW as ::core::ffi::c_short;
                            break '_f_forw;
                        }
                    } else {
                        dbg_printf(
                            b"Forwarding suspension not over, time left: %d\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            INET_SUSPEND_TIME as time_t - fwd_suspend,
                        );
                        break 's_606;
                    }
                }
                F_FORW => {
                    break '_f_forw;
                }
                F_CONSOLE => {
                    (*f).f_time = now;
                    if flags & IGN_CONS != 0 {
                        dbg_printf(b" (ignored)\n\0".as_ptr() as *const ::core::ffi::c_char);
                        break 's_606;
                    }
                }
                F_TTY | F_FILE | F_PIPE => {}
                F_USERS | F_WALL => {
                    (*f).f_time = now;
                    dbg_printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    (*v).iov_base = b"\r\n\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char
                        as *mut ::core::ffi::c_void;
                    (*v).iov_len = 2 as size_t;
                    wallmsg(f, &raw mut iov as *mut iovec);
                    break 's_606;
                }
                _ => {
                    break 's_606;
                }
            }
            (*f).f_time = now;
            dbg_printf(
                b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*f).f_un.f_fname,
            );
            if (*f).f_type as ::core::ffi::c_int == F_TTY
                || (*f).f_type as ::core::ffi::c_int == F_CONSOLE
            {
                (*v).iov_base = b"\r\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void;
                (*v).iov_len = 2 as size_t;
            } else {
                (*v).iov_base = b"\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char
                    as *mut ::core::ffi::c_void;
                (*v).iov_len = 1 as size_t;
            }
            loop {
                if writev(
                    (*f).f_file as ::core::ffi::c_int,
                    &raw mut iov as *mut iovec,
                    IOVCNT,
                ) < 0 as ssize_t
                {
                    let mut e_0: ::core::ffi::c_int = *__errno_location();
                    if (*f).f_type as ::core::ffi::c_int == F_PIPE && e_0 == EAGAIN {
                        break 's_606;
                    }
                    close((*f).f_file as ::core::ffi::c_int);
                    if (e_0 == EIO || e_0 == EBADF)
                        && ((*f).f_type as ::core::ffi::c_int == F_TTY
                            || (*f).f_type as ::core::ffi::c_int == F_CONSOLE)
                    {
                        (*f).f_file = open(
                            (*f).f_un.f_fname,
                            O_WRONLY | O_APPEND,
                            0 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_short;
                        if ((*f).f_file as ::core::ffi::c_int) >= 0 as ::core::ffi::c_int {
                            continue;
                        }
                        (*f).f_type = F_UNUSED as ::core::ffi::c_short;
                        logerror((*f).f_un.f_fname);
                        free((*f).f_un.f_fname as *mut ::core::ffi::c_void);
                        (*f).f_un.f_fname = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        break 's_606;
                    } else {
                        (*f).f_type = F_UNUSED as ::core::ffi::c_short;
                        *__errno_location() = e_0;
                        logerror((*f).f_un.f_fname);
                        free((*f).f_un.f_fname as *mut ::core::ffi::c_void);
                        (*f).f_un.f_fname = ::core::ptr::null_mut::<::core::ffi::c_char>();
                        break 's_606;
                    }
                } else {
                    if flags & SYNC_FILE != 0 && (*f).f_flags & OMIT_SYNC == 0 {
                        fsync((*f).f_file as ::core::ffi::c_int);
                    }
                    break 's_606;
                }
            }
        }
        dbg_printf(
            b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*f).f_un.f_forw.f_hname,
        );
        if strcasecmp(from, LocalHostName) != 0 && NoHops != 0 {
            dbg_printf(b"Not forwarding remote message.\n\0".as_ptr() as *const ::core::ffi::c_char);
        } else if NoForward != 0 {
            dbg_printf(
                b"Not forwarding because forwarding is disabled.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            let mut temp_finet: ::core::ffi::c_int = 0;
            let mut pfinet: *mut ::core::ffi::c_int = ::core::ptr::null_mut::<::core::ffi::c_int>();
            if (*f).f_un.f_forw.f_addr.ss_family as ::core::ffi::c_int == AF_INET {
                pfinet = (&raw mut finet as *mut ::core::ffi::c_int).offset(IU_FD_IP4 as isize);
            } else {
                pfinet = (&raw mut finet as *mut ::core::ffi::c_int).offset(IU_FD_IP6 as isize);
            }
            temp_finet = *pfinet;
            if temp_finet < 0 as ::core::ffi::c_int {
                let mut err_0: ::core::ffi::c_int = 0;
                let mut hints_0: addrinfo = addrinfo {
                    ai_flags: 0,
                    ai_family: 0,
                    ai_socktype: 0,
                    ai_protocol: 0,
                    ai_addrlen: 0,
                    ai_addr: ::core::ptr::null_mut::<sockaddr>(),
                    ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ai_next: ::core::ptr::null_mut::<addrinfo>(),
                };
                let mut rp_0: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
                memset(
                    &raw mut hints_0 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<addrinfo>(),
                );
                hints_0.ai_family = (*f).f_un.f_forw.f_addr.ss_family as ::core::ffi::c_int;
                hints_0.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
                hints_0.ai_flags = AI_PASSIVE;
                err_0 = getaddrinfo(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    LogForwardPort,
                    &raw mut hints_0,
                    &raw mut rp_0,
                );
                if err_0 != 0 {
                    dbg_printf(
                        b"Not forwarding due to lookup failure: %s.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        gai_strerror(err_0),
                    );
                    break 's_606;
                } else {
                    temp_finet =
                        socket((*rp_0).ai_family, (*rp_0).ai_socktype, (*rp_0).ai_protocol);
                    if temp_finet < 0 as ::core::ffi::c_int {
                        dbg_printf(b"Not forwarding due to socket failure.\n\0".as_ptr()
                            as *const ::core::ffi::c_char);
                        freeaddrinfo(rp_0);
                        break 's_606;
                    } else {
                        err_0 = bind(
                            temp_finet,
                            __CONST_SOCKADDR_ARG {
                                __sockaddr__: (*rp_0).ai_addr,
                            },
                            (*rp_0).ai_addrlen,
                        );
                        freeaddrinfo(rp_0);
                        if err_0 != 0 {
                            dbg_printf(
                                b"Not forwarding due to bind error: %s.\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                strerror(*__errno_location()),
                            );
                            break 's_606;
                        }
                    }
                }
            }
            (*f).f_time = now;
            snprintf(
                &raw mut line as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1025]>(),
                b"<%d>%.15s %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*f).f_prevpri,
                iov[0usize].iov_base as *mut ::core::ffi::c_char,
                iov[4usize].iov_base as *mut ::core::ffi::c_char,
            );
            l = strlen(&raw mut line as *mut ::core::ffi::c_char) as ::core::ffi::c_int;
            if l > MAXLINE {
                l = MAXLINE;
            }
            if sendto(
                temp_finet,
                &raw mut line as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                l as size_t,
                0 as ::core::ffi::c_int,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &raw mut (*f).f_un.f_forw.f_addr as *mut sockaddr,
                },
                (*f).f_un.f_forw.f_addrlen,
            ) != l as ssize_t
            {
                let mut e: ::core::ffi::c_int = *__errno_location();
                dbg_printf(
                    b"INET sendto error: %d = %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                    e,
                    strerror(e),
                );
                (*f).f_type = F_FORW_SUSP as ::core::ffi::c_short;
                *__errno_location() = e;
                logerror(b"sendto\0".as_ptr() as *const ::core::ffi::c_char);
            }
            if *pfinet < 0 as ::core::ffi::c_int {
                close(temp_finet);
            }
        }
    }
    if (*f).f_type as ::core::ffi::c_int != F_FORW_UNKN {
        (*f).f_prevcount = 0 as ::core::ffi::c_int;
    }
}
#[export_name = "rboxc_syslogd_wallmsg"]
pub unsafe extern "C" fn wallmsg(mut f: *mut filed, mut iov: *mut iovec) {
    static mut reenter: ::core::ffi::c_int = 0;
    let mut utp: *mut STRUCT_UTMP = ::core::ptr::null_mut::<STRUCT_UTMP>();
    let mut utmpbuf: *mut STRUCT_UTMP = ::core::ptr::null_mut::<STRUCT_UTMP>();
    let mut utmp_count: idx_t = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut line: [::core::ffi::c_char; 9] = [0; 9];
    let c2rust_fresh6 = reenter;
    reenter += 1;
    if c2rust_fresh6 != 0 {
        return;
    }
    if read_utmp(
        UTMP_FILE.as_ptr(),
        &raw mut utmp_count,
        &raw mut utmpbuf,
        C2Rust_Unnamed_14::READ_UTMP_USER_PROCESS.0 as ::core::ffi::c_int
            | C2Rust_Unnamed_14::READ_UTMP_CHECK_PIDS.0 as ::core::ffi::c_int,
    ) < 0 as ::core::ffi::c_int
    {
        logerror(b"opening utmp file\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
    utp = utmpbuf;
    while utp < utmpbuf.offset(utmp_count as isize) {
        strncpy(
            &raw mut line as *mut ::core::ffi::c_char,
            (*utp).ut_line,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
        );
        line[::core::mem::size_of::<*mut ::core::ffi::c_char>()] = '\0' as ::core::ffi::c_char;
        if (*f).f_type as ::core::ffi::c_int == F_WALL {
            p = inetutils_ttymsg(
                iov,
                IOVCNT,
                &raw mut line as *mut ::core::ffi::c_char,
                TTYMSGTIME,
            );
            if !p.is_null() {
                *__errno_location() = 0 as ::core::ffi::c_int;
                logerror(p);
            }
        } else {
            i = 0 as ::core::ffi::c_int;
            while i < (*f).f_un.f_user.f_nusers {
                if strncmp(
                    *(*f).f_un.f_user.f_unames.offset(i as isize),
                    (*utp).ut_user,
                    ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
                ) == 0
                {
                    p = inetutils_ttymsg(
                        iov,
                        IOVCNT,
                        &raw mut line as *mut ::core::ffi::c_char,
                        TTYMSGTIME,
                    );
                    if !p.is_null() {
                        *__errno_location() = 0 as ::core::ffi::c_int;
                        logerror(p);
                    }
                    break;
                } else {
                    i += 1;
                }
            }
        }
        utp = utp.offset(1);
    }
    free(utmpbuf as *mut ::core::ffi::c_void);
    reenter = 0 as ::core::ffi::c_int;
}
#[export_name = "rboxc_syslogd_cvthname"]
pub unsafe extern "C" fn cvthname(
    mut f: *mut sockaddr,
    mut len: socklen_t,
) -> *const ::core::ffi::c_char {
    let mut err: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    err = getnameinfo(
        f,
        len,
        &raw mut addrstr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NUMERICHOST,
    );
    if err != 0 {
        dbg_printf(
            b"Malformed from address: %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
            gai_strerror(err),
        );
        return b"???\0".as_ptr() as *const ::core::ffi::c_char;
    }
    dbg_printf(
        b"cvthname(%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut addrstr as *mut ::core::ffi::c_char,
    );
    err = getnameinfo(
        f,
        len,
        &raw mut addrname as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1025]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NAMEREQD,
    );
    if err != 0 {
        dbg_printf(
            b"Host name for your address (%s) unknown.\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut addrstr as *mut ::core::ffi::c_char,
        );
        return &raw mut addrstr as *mut ::core::ffi::c_char;
    }
    p = strchr(
        &raw mut addrname as *mut ::core::ffi::c_char,
        '.' as ::core::ffi::c_int,
    );
    if !p.is_null() {
        if strcasecmp(p.offset(1 as ::core::ffi::c_int as isize), LocalDomain)
            == 0 as ::core::ffi::c_int
        {
            *p = '\0' as ::core::ffi::c_char;
        } else {
            let mut count: ::core::ffi::c_int = 0;
            if !StripDomains.is_null() {
                count = 0 as ::core::ffi::c_int;
                while !(*StripDomains.offset(count as isize)).is_null() {
                    if strcasecmp(
                        p.offset(1 as ::core::ffi::c_int as isize),
                        *StripDomains.offset(count as isize),
                    ) == 0 as ::core::ffi::c_int
                    {
                        *p = '\0' as ::core::ffi::c_char;
                        return &raw mut addrname as *mut ::core::ffi::c_char;
                    }
                    count += 1;
                }
            }
            if !LocalHosts.is_null() {
                count = 0 as ::core::ffi::c_int;
                while !(*LocalHosts.offset(count as isize)).is_null() {
                    if strcasecmp(
                        &raw mut addrname as *mut ::core::ffi::c_char,
                        *LocalHosts.offset(count as isize),
                    ) == 0 as ::core::ffi::c_int
                    {
                        *p = '\0' as ::core::ffi::c_char;
                        return &raw mut addrname as *mut ::core::ffi::c_char;
                    }
                    count += 1;
                }
            }
        }
    }
    return &raw mut addrname as *mut ::core::ffi::c_char;
}
#[export_name = "rboxc_syslogd_domark"]
pub unsafe extern "C" fn domark(mut signo: ::core::ffi::c_int) {
    let mut f: *mut filed = ::core::ptr::null_mut::<filed>();
    now = time(NULL as *mut time_t);
    if MarkInterval > 0 as ::core::ffi::c_int {
        MarkSeq += TIMERINTVL;
        if MarkSeq >= MarkInterval {
            logmsg(
                LOG_INFO,
                b"-- MARK --\0".as_ptr() as *const ::core::ffi::c_char,
                LocalHostName,
                ADDDATE | MARK,
            );
            MarkSeq = 0 as ::core::ffi::c_int;
        }
    }
    f = Files;
    while !f.is_null() {
        if (*f).f_prevcount != 0
            && now >= (*f).f_time + repeatinterval[(*f).f_repeatcount] as time_t
        {
            dbg_printf(
                b"flush %s: repeated %d times, %d sec.\n\0".as_ptr() as *const ::core::ffi::c_char,
                TypeNames[(*f).f_type as usize],
                (*f).f_prevcount,
                repeatinterval[(*f).f_repeatcount],
            );
            fprintlog(
                f,
                LocalHostName,
                0 as ::core::ffi::c_int,
                NULL as *mut ::core::ffi::c_char,
            );
            (*f).f_repeatcount = (*f).f_repeatcount.wrapping_add(1);
            if (*f).f_repeatcount > MAXREPEAT {
                (*f).f_repeatcount = MAXREPEAT as size_t;
            }
        }
        f = (*f).f_next;
    }
    alarm(TIMERINTVL as ::core::ffi::c_uint);
}
#[export_name = "rboxc_syslogd_logerror"]
pub unsafe extern "C" fn logerror(mut r#type: *const ::core::ffi::c_char) {
    let mut buf: [::core::ffi::c_char; 100] = [0; 100];
    if *__errno_location() != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"syslogd: %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            r#type,
            strerror(*__errno_location()),
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"syslogd: %s\0".as_ptr() as *const ::core::ffi::c_char,
            r#type,
        );
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    dbg_printf(
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut buf as *mut ::core::ffi::c_char,
    );
    logmsg(
        LOG_SYSLOG | LOG_ERR,
        &raw mut buf as *mut ::core::ffi::c_char,
        LocalHostName,
        ADDDATE,
    );
}
#[export_name = "rboxc_syslogd_doexit"]
pub unsafe extern "C" fn doexit(mut signo: ::core::ffi::c_int) {
    _exit(EXIT_SUCCESS);
}
#[export_name = "rboxc_syslogd_die"]
pub unsafe extern "C" fn die(mut signo: ::core::ffi::c_int) {
    let mut f: *mut filed = ::core::ptr::null_mut::<filed>();
    let mut was_initialized: ::core::ffi::c_int = Initialized;
    let mut buf: [::core::ffi::c_char; 100] = [0; 100];
    let mut i: size_t = 0;
    Initialized = 0 as ::core::ffi::c_int;
    f = Files;
    while !f.is_null() {
        if (*f).f_prevcount != 0 {
            fprintlog(
                f,
                LocalHostName,
                0 as ::core::ffi::c_int,
                NULL as *mut ::core::ffi::c_char,
            );
        }
        f = (*f).f_next;
    }
    Initialized = was_initialized;
    if signo != 0 {
        dbg_printf(
            b"%s: exiting on signal %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_invocation_name,
            signo,
        );
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 100]>(),
            b"exiting on signal %d\0".as_ptr() as *const ::core::ffi::c_char,
            signo,
        );
        *__errno_location() = 0 as ::core::ffi::c_int;
        logerror(&raw mut buf as *mut ::core::ffi::c_char);
    }
    if fklog >= 0 as ::core::ffi::c_int {
        close(fklog);
    }
    i = 0 as size_t;
    while i < nfunix {
        if (*funix.offset(i as isize)).fd >= 0 as ::core::ffi::c_int {
            close((*funix.offset(i as isize)).fd);
            if !(*funix.offset(i as isize)).name.is_null() {
                unlink((*funix.offset(i as isize)).name);
            }
        }
        i = i.wrapping_add(1);
    }
    if finet[IU_FD_IP4 as usize] >= 0 as ::core::ffi::c_int {
        close(finet[IU_FD_IP4 as usize]);
    }
    if finet[IU_FD_IP6 as usize] >= 0 as ::core::ffi::c_int {
        close(finet[IU_FD_IP6 as usize]);
    }
    exit(EXIT_SUCCESS);
}
unsafe extern "C" fn load_conffile(
    mut filename: *const ::core::ffi::c_char,
    mut nextp: *mut *mut filed,
) -> ::core::ffi::c_int {
    let mut cf: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut f: *mut filed = ::core::ptr::null_mut::<filed>();
    let mut line_max: size_t = LINE_MAX as size_t;
    let mut cbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cline: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cont_line: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    cf = fopen(filename, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if cf.is_null() {
        dbg_printf(
            b"cannot open %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            filename,
        );
        if (*nextp).is_null() {
            f = calloc(1 as size_t, ::core::mem::size_of::<filed>()) as *mut filed;
            cfline(
                b"*.ERR\t/dev/console\0".as_ptr() as *const ::core::ffi::c_char,
                f,
            );
            (*f).f_next = calloc(1 as size_t, ::core::mem::size_of::<filed>()) as *mut filed;
            cfline(
                b"*.PANIC\t*\0".as_ptr() as *const ::core::ffi::c_char,
                (*f).f_next,
            );
            *nextp = f;
        }
        Initialized = 1 as ::core::ffi::c_int;
        return 1 as ::core::ffi::c_int;
    }
    cbuf = malloc(line_max) as *mut ::core::ffi::c_char;
    if cbuf.is_null() {
        dbg_printf(
            b"cannot allocate space for configuration\n\0".as_ptr() as *const ::core::ffi::c_char
        );
        fclose(cf);
        return 0 as ::core::ffi::c_int;
    }
    cline = cbuf;
    free(selector as *mut ::core::ffi::c_void);
    selector = ::core::ptr::null_mut::<::core::ffi::c_char>();
    while !fgets(
        cline,
        line_max.wrapping_sub(cline.offset_from(cbuf) as size_t) as ::core::ffi::c_int,
        cf,
    )
    .is_null()
    {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut len: size_t = strlen(cline);
        if cont_line != 0 {
            let mut start: *mut ::core::ffi::c_char = cline;
            while *start as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *start as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                start = start.offset(1);
            }
            len = len.wrapping_sub(start.offset_from(cline) as size_t);
            memmove(
                cline as *mut ::core::ffi::c_void,
                start as *const ::core::ffi::c_void,
                len.wrapping_add(1 as size_t),
            );
            cont_line = 0 as ::core::ffi::c_int;
        }
        if strchr(cline, '\n' as ::core::ffi::c_int).is_null() {
            let mut offset: size_t = cline.offset_from(cbuf) as size_t;
            let mut tmp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            tmp = rpl_realloc(
                cbuf as *mut ::core::ffi::c_void,
                line_max.wrapping_mul(2 as size_t),
            ) as *mut ::core::ffi::c_char;
            if tmp.is_null() {
                dbg_printf(b"cannot allocate space configuration\n\0".as_ptr()
                    as *const ::core::ffi::c_char);
                fclose(cf);
                free(cbuf as *mut ::core::ffi::c_void);
                return 0 as ::core::ffi::c_int;
            } else {
                cbuf = tmp;
            }
            line_max = line_max.wrapping_mul(2 as size_t);
            cline = cbuf
                .offset(offset as isize)
                .offset(len as isize)
                .offset(-(1 as ::core::ffi::c_int as isize));
        } else {
            cline = cbuf;
            p = cline;
            while *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_12::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == '!' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '#' as ::core::ffi::c_int
                    && *p.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                        == '!' as ::core::ffi::c_int
            {
                p = p.offset(1);
                if *p as ::core::ffi::c_int == '!' as ::core::ffi::c_int {
                    p = p.offset(1);
                }
                while *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed_12::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
                {
                    p = p.offset(1);
                }
                if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    continue;
                }
                free(selector as *mut ::core::ffi::c_void);
                selector = ::core::ptr::null_mut::<::core::ffi::c_char>();
                if *p as ::core::ffi::c_int != '*' as ::core::ffi::c_int {
                    let mut sep: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    sep = strchr(p, ',' as ::core::ffi::c_int);
                    if !sep.is_null() {
                        *sep = '\0' as ::core::ffi::c_char;
                    }
                    sep = strpbrk(p, b" \t\n\r\0".as_ptr() as *const ::core::ffi::c_char);
                    if !sep.is_null() {
                        *sep = '\0' as ::core::ffi::c_char;
                    }
                    selector = strdup(p);
                }
            } else {
                if *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                    || *p as ::core::ffi::c_int == '#' as ::core::ffi::c_int
                {
                    continue;
                }
                memmove(
                    cline as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    strlen(p).wrapping_add(1 as size_t),
                );
                p = strchr(cline, '\0' as ::core::ffi::c_int);
                loop {
                    p = p.offset(-1);
                    if *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_12::_ISspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort as ::core::ffi::c_int
                        == 0
                    {
                        break;
                    }
                }
                if *p as ::core::ffi::c_int == '\\' as ::core::ffi::c_int {
                    *p = '\0' as ::core::ffi::c_char;
                    cline = p;
                    cont_line = 1 as ::core::ffi::c_int;
                } else {
                    p = p.offset(1);
                    *p = '\0' as ::core::ffi::c_char;
                    f = calloc(1 as size_t, ::core::mem::size_of::<filed>()) as *mut filed;
                    cfline(cbuf, f);
                    (*f).f_next = *nextp;
                    *nextp = f;
                }
            }
        }
    }
    fclose(cf);
    free(cbuf as *mut ::core::ffi::c_void);
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn load_confdir(
    mut dirname: *const ::core::ffi::c_char,
    mut nextp: *mut *mut filed,
) -> ::core::ffi::c_int {
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut dent: *mut dirent = ::core::ptr::null_mut::<dirent>();
    let mut dir: *mut DIR = ::core::ptr::null_mut::<DIR>();
    dir = opendir(dirname);
    if dir.is_null() {
        dbg_printf(
            b"cannot open %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            dirname,
        );
        return 1 as ::core::ffi::c_int;
    }
    loop {
        dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        let mut st: stat = stat {
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
        let mut file: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if asprintf(
            &raw mut file,
            b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
            dirname,
            &raw mut (*dent).d_name as *mut ::core::ffi::c_char,
        ) < 0 as ::core::ffi::c_int
        {
            dbg_printf(
                b"cannot allocate space for configuration filename\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return 0 as ::core::ffi::c_int;
        }
        if stat(file, &raw mut st) != 0 as ::core::ffi::c_int {
            dbg_printf(
                b"cannot stat file configuration file\n\0".as_ptr() as *const ::core::ffi::c_char
            );
        } else {
            if st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
                found += 1;
                rc += load_conffile(file, nextp);
            }
            free(file as *mut ::core::ffi::c_void);
        }
    }
    closedir(dir);
    return if found != 0 {
        rc
    } else {
        1 as ::core::ffi::c_int
    };
}
#[export_name = "rboxc_syslogd_init"]
pub unsafe extern "C" fn init(mut signo: ::core::ffi::c_int) {
    let mut rc: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut f: *mut filed = ::core::ptr::null_mut::<filed>();
    let mut next: *mut filed = ::core::ptr::null_mut::<filed>();
    let mut nextp: *mut *mut filed = ::core::ptr::null_mut::<*mut filed>();
    dbg_printf(b"init\n\0".as_ptr() as *const ::core::ffi::c_char);
    Initialized = 0 as ::core::ffi::c_int;
    f = Files;
    while !f.is_null() {
        let mut j: ::core::ffi::c_int = 0;
        if (*f).f_prevcount != 0 {
            fprintlog(
                f,
                LocalHostName,
                0 as ::core::ffi::c_int,
                NULL as *mut ::core::ffi::c_char,
            );
        }
        match (*f).f_type as ::core::ffi::c_int {
            F_FILE | F_TTY | F_CONSOLE | F_PIPE => {
                free((*f).f_un.f_fname as *mut ::core::ffi::c_void);
                close((*f).f_file as ::core::ffi::c_int);
            }
            F_FORW | F_FORW_SUSP | F_FORW_UNKN => {
                free((*f).f_un.f_forw.f_hname as *mut ::core::ffi::c_void);
            }
            F_USERS => {
                j = 0 as ::core::ffi::c_int;
                while j < (*f).f_un.f_user.f_nusers {
                    free(*(*f).f_un.f_user.f_unames.offset(j as isize) as *mut ::core::ffi::c_void);
                    j += 1;
                }
                free((*f).f_un.f_user.f_unames as *mut ::core::ffi::c_void);
            }
            _ => {}
        }
        free((*f).f_progname as *mut ::core::ffi::c_void);
        free((*f).f_prevhost as *mut ::core::ffi::c_void);
        next = (*f).f_next;
        free(f as *mut ::core::ffi::c_void);
        f = next;
    }
    Files = ::core::ptr::null_mut::<filed>();
    nextp = &raw mut Files;
    facilities_seen = 0 as ::core::ffi::c_int;
    rc = load_conffile(ConfFile, nextp);
    ret = load_confdir(ConfDir, nextp);
    if ret == 0 {
        rc = 0 as ::core::ffi::c_int;
    }
    Initialized = 1 as ::core::ffi::c_int;
    if Debug != 0 {
        f = Files;
        while !f.is_null() {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i <= LOG_NFACILITIES {
                if (*f).f_pmask[i as usize] as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    dbg_printf(b" X \0".as_ptr() as *const ::core::ffi::c_char);
                } else {
                    dbg_printf(
                        b"%2x \0".as_ptr() as *const ::core::ffi::c_char,
                        (*f).f_pmask[i as usize] as ::core::ffi::c_int,
                    );
                }
                i += 1;
            }
            dbg_printf(
                b"%s: \0".as_ptr() as *const ::core::ffi::c_char,
                TypeNames[(*f).f_type as usize],
            );
            match (*f).f_type as ::core::ffi::c_int {
                F_FILE | F_TTY | F_CONSOLE | F_PIPE => {
                    dbg_printf(
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*f).f_un.f_fname,
                    );
                }
                F_FORW | F_FORW_SUSP | F_FORW_UNKN => {
                    dbg_printf(
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        (*f).f_un.f_forw.f_hname,
                    );
                }
                F_USERS => {
                    i = 0 as ::core::ffi::c_int;
                    while i < (*f).f_un.f_user.f_nusers {
                        dbg_printf(
                            b"%s, \0".as_ptr() as *const ::core::ffi::c_char,
                            *(*f).f_un.f_user.f_unames.offset(i as isize),
                        );
                        i += 1;
                    }
                }
                _ => {}
            }
            dbg_printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            f = (*f).f_next;
        }
    }
    if AcceptRemote != 0 {
        logmsg(
            LOG_SYSLOG | LOG_INFO,
            b"syslogd (GNU inetutils 2.8): restart (remote reception)\0".as_ptr()
                as *const ::core::ffi::c_char,
            LocalHostName,
            ADDDATE,
        );
    } else {
        logmsg(
            LOG_SYSLOG | LOG_INFO,
            b"syslogd (GNU inetutils 2.8): restart\0".as_ptr() as *const ::core::ffi::c_char,
            LocalHostName,
            ADDDATE,
        );
    }
    if rc == 0 {
        logmsg(
            LOG_SYSLOG | LOG_ERR,
            b"syslogd: Incomplete configuration.\0".as_ptr() as *const ::core::ffi::c_char,
            LocalHostName,
            ADDDATE,
        );
    }
    dbg_printf(b"syslogd: restarted\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[export_name = "rboxc_syslogd_cfline"]
pub unsafe extern "C" fn cfline(mut line: *const ::core::ffi::c_char, mut f: *mut filed) {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    let mut rp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut i: ::core::ffi::c_int = 0;
    let mut pri: ::core::ffi::c_int = 0;
    let mut negate_pri: ::core::ffi::c_int = 0;
    let mut excl_pri: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let mut pri_set: ::core::ffi::c_uint = 0;
    let mut pri_clear: ::core::ffi::c_uint = 0;
    let mut bp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut ebuf: [::core::ffi::c_char; 200] = [0; 200];
    dbg_printf(
        b"cfline(%s)%s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        line,
        if !selector.is_null() {
            b" tagged \0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !selector.is_null() {
            selector as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    *__errno_location() = 0 as ::core::ffi::c_int;
    memset(
        f as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<filed>(),
    );
    i = 0 as ::core::ffi::c_int;
    while i <= LOG_NFACILITIES {
        (*f).f_pmask[i as usize] = 0 as ::core::ffi::c_uchar;
        (*f).f_flags = 0 as ::core::ffi::c_int;
        i += 1;
    }
    p = line;
    while *p as ::core::ffi::c_int != 0
        && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
    {
        q = p;
        while *q as ::core::ffi::c_int != 0
            && *q as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
            && {
                let c2rust_fresh0 = q;
                q = q.offset(1);
                *c2rust_fresh0 as ::core::ffi::c_int != '.' as ::core::ffi::c_int
            }
        {}
        bp = &raw mut buf as *mut ::core::ffi::c_char;
        while *q as ::core::ffi::c_int != 0
            && (strchr(
                b"\t ,;\0".as_ptr() as *const ::core::ffi::c_char,
                *q as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char)
                .is_null()
        {
            let c2rust_fresh1 = q;
            q = q.offset(1);
            let c2rust_fresh2 = bp;
            bp = bp.offset(1);
            *c2rust_fresh2 = *c2rust_fresh1;
        }
        *bp = '\0' as ::core::ffi::c_char;
        while *q as ::core::ffi::c_int != 0
            && !(strchr(
                b",;\0".as_ptr() as *const ::core::ffi::c_char,
                *q as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char)
                .is_null()
        {
            q = q.offset(1);
        }
        bp = &raw mut buf as *mut ::core::ffi::c_char;
        excl_pri = 0 as ::core::ffi::c_int;
        negate_pri = excl_pri;
        while *bp as ::core::ffi::c_int == '!' as ::core::ffi::c_int
            || *bp as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        {
            let c2rust_fresh3 = bp;
            bp = bp.offset(1);
            match *c2rust_fresh3 as ::core::ffi::c_int {
                33 => {
                    negate_pri = 1 as ::core::ffi::c_int;
                }
                61 => {
                    excl_pri = 1 as ::core::ffi::c_int;
                }
                _ => {}
            }
        }
        if *bp as ::core::ffi::c_int == '*' as ::core::ffi::c_int {
            pri_clear = 0 as ::core::ffi::c_uint;
            pri_set = (((1 as ::core::ffi::c_int)
                << 0x7 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                - 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
        } else {
            pri = decode(bp, &raw mut prioritynames as *mut CODE);
            if pri < 0 as ::core::ffi::c_int || pri > LOG_PRIMASK && pri != INTERNAL_NOPRI {
                snprintf(
                    &raw mut ebuf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 200]>(),
                    b"unknown priority name \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                    bp,
                );
                logerror(&raw mut ebuf as *mut ::core::ffi::c_char);
                return;
            }
            if pri == INTERNAL_NOPRI {
                pri_clear = 255 as ::core::ffi::c_uint;
                pri_set = 0 as ::core::ffi::c_uint;
            } else {
                pri_clear = 0 as ::core::ffi::c_uint;
                pri_set = (if excl_pri != 0 {
                    (1 as ::core::ffi::c_int) << pri
                } else {
                    ((1 as ::core::ffi::c_int) << pri + 1 as ::core::ffi::c_int)
                        - 1 as ::core::ffi::c_int
                }) as ::core::ffi::c_uint;
            }
        }
        if negate_pri != 0 {
            let mut exchange: ::core::ffi::c_uint = pri_set;
            pri_set = pri_clear;
            pri_clear = exchange;
        }
        while *p as ::core::ffi::c_int != 0
            && (strchr(
                b"\t .;\0".as_ptr() as *const ::core::ffi::c_char,
                *p as ::core::ffi::c_int,
            ) as *const ::core::ffi::c_char)
                .is_null()
        {
            bp = &raw mut buf as *mut ::core::ffi::c_char;
            while *p as ::core::ffi::c_int != 0
                && (strchr(
                    b"\t ,;.\0".as_ptr() as *const ::core::ffi::c_char,
                    *p as ::core::ffi::c_int,
                ) as *const ::core::ffi::c_char)
                    .is_null()
            {
                let c2rust_fresh4 = p;
                p = p.offset(1);
                let c2rust_fresh5 = bp;
                bp = bp.offset(1);
                *c2rust_fresh5 = *c2rust_fresh4;
            }
            *bp = '\0' as ::core::ffi::c_char;
            if *(&raw mut buf as *mut ::core::ffi::c_char) as ::core::ffi::c_int
                == '*' as ::core::ffi::c_int
            {
                i = 0 as ::core::ffi::c_int;
                while i <= LOG_NFACILITIES {
                    if !(buf[1usize] as ::core::ffi::c_int == '*' as ::core::ffi::c_int
                        && (1 as ::core::ffi::c_int) << i & facilities_seen != 0)
                    {
                        (*f).f_pmask[i as usize] = ((*f).f_pmask[i as usize] as ::core::ffi::c_uint
                            & !pri_clear)
                            as ::core::ffi::c_uchar;
                        (*f).f_pmask[i as usize] = ((*f).f_pmask[i as usize] as ::core::ffi::c_uint
                            | pri_set)
                            as ::core::ffi::c_uchar;
                    }
                    i += 1;
                }
            } else {
                i = decode(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    &raw mut facilitynames as *mut CODE,
                );
                if i < 0 as ::core::ffi::c_int || i > LOG_NFACILITIES << 3 as ::core::ffi::c_int {
                    snprintf(
                        &raw mut ebuf as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 200]>(),
                        b"unknown facility name \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                    logerror(&raw mut ebuf as *mut ::core::ffi::c_char);
                    return;
                }
                (*f).f_pmask[((i & LOG_FACMASK) >> 3 as ::core::ffi::c_int) as usize] =
                    ((*f).f_pmask[((i & LOG_FACMASK) >> 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_uint
                        & !pri_clear) as ::core::ffi::c_uchar;
                (*f).f_pmask[((i & LOG_FACMASK) >> 3 as ::core::ffi::c_int) as usize] =
                    ((*f).f_pmask[((i & LOG_FACMASK) >> 3 as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_uint
                        | pri_set) as ::core::ffi::c_uchar;
                facilities_seen |=
                    (1 as ::core::ffi::c_int) << ((i & LOG_FACMASK) >> 3 as ::core::ffi::c_int);
            }
            while *p as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
        }
        p = q;
    }
    while *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    if *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        (*f).f_flags |= OMIT_SYNC;
        p = p.offset(1);
    }
    if strlen(p) == 0 {
        (*f).f_type = F_UNUSED as ::core::ffi::c_short;
        logerror(b"empty action field\0".as_ptr() as *const ::core::ffi::c_char);
        return;
    }
    match *p as ::core::ffi::c_int {
        64 => {
            p = p.offset(1);
            (*f).f_un.f_forw.f_hname = strdup(p);
            memset(
                &raw mut hints as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<addrinfo>(),
            );
            hints.ai_family = usefamily;
            hints.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
            if usefamily == AF_UNSPEC {
                hints.ai_flags |= AI_ADDRCONFIG;
            }
            (*f).f_un.f_forw.f_addrlen = 0 as socklen_t;
            memset(
                &raw mut (*f).f_un.f_forw.f_addr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_storage>(),
            );
            err = getaddrinfo(p, LogForwardPort, &raw mut hints, &raw mut rp);
            if err != 0 {
                match err {
                    EAI_AGAIN | EAI_MEMORY => {
                        (*f).f_type = F_FORW_UNKN as ::core::ffi::c_short;
                        (*f).f_prevcount = INET_RETRY_MAX;
                    }
                    EAI_NONAME | EAI_NODATA | EAI_ADDRFAMILY | _ => {
                        (*f).f_type = F_UNUSED as ::core::ffi::c_short;
                    }
                }
                (*f).f_time = time(::core::ptr::null_mut::<time_t>());
            } else {
                (*f).f_type = F_FORW as ::core::ffi::c_short;
                (*f).f_un.f_forw.f_addrlen = (*rp).ai_addrlen;
                memcpy(
                    &raw mut (*f).f_un.f_forw.f_addr as *mut ::core::ffi::c_void,
                    (*rp).ai_addr as *const ::core::ffi::c_void,
                    (*rp).ai_addrlen as size_t,
                );
                freeaddrinfo(rp);
            }
        }
        124 => {
            (*f).f_un.f_fname = strdup(p);
            p = p.offset(1);
            (*f).f_file = open(p, O_RDWR | O_NONBLOCK) as ::core::ffi::c_short;
            if ((*f).f_file as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                (*f).f_type = F_UNUSED as ::core::ffi::c_short;
                logerror(p);
                free((*f).f_un.f_fname as *mut ::core::ffi::c_void);
                (*f).f_un.f_fname = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else if strcmp(p, &raw mut ctty as *mut ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*f).f_type = F_CONSOLE as ::core::ffi::c_short;
            } else if isatty((*f).f_file as ::core::ffi::c_int) != 0 {
                (*f).f_type = F_TTY as ::core::ffi::c_short;
            } else {
                (*f).f_type = F_PIPE as ::core::ffi::c_short;
            }
        }
        47 => {
            (*f).f_un.f_fname = strdup(p);
            (*f).f_file = open(
                p,
                O_WRONLY | O_APPEND | O_CREAT,
                0o644 as ::core::ffi::c_int,
            ) as ::core::ffi::c_short;
            if ((*f).f_file as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
                (*f).f_type = F_UNUSED as ::core::ffi::c_short;
                logerror(p);
                free((*f).f_un.f_fname as *mut ::core::ffi::c_void);
                (*f).f_un.f_fname = ::core::ptr::null_mut::<::core::ffi::c_char>();
            } else if strcmp(p, &raw mut ctty as *mut ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                (*f).f_type = F_CONSOLE as ::core::ffi::c_short;
            } else if isatty((*f).f_file as ::core::ffi::c_int) != 0 {
                (*f).f_type = F_TTY as ::core::ffi::c_short;
            } else {
                (*f).f_type = F_FILE as ::core::ffi::c_short;
            }
        }
        42 => {
            (*f).f_type = F_WALL as ::core::ffi::c_short;
        }
        _ => {
            (*f).f_un.f_user.f_nusers = 1 as ::core::ffi::c_int;
            q = p;
            while *q != 0 {
                if *q as ::core::ffi::c_int == ',' as ::core::ffi::c_int {
                    (*f).f_un.f_user.f_nusers += 1;
                }
                q = q.offset(1);
            }
            (*f).f_un.f_user.f_unames = malloc(
                ((*f).f_un.f_user.f_nusers as size_t)
                    .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
            ) as *mut *mut ::core::ffi::c_char;
            i = 0 as ::core::ffi::c_int;
            while *p != 0 {
                q = p;
                while *q as ::core::ffi::c_int != 0
                    && *q as ::core::ffi::c_int != ',' as ::core::ffi::c_int
                {
                    q = q.offset(1);
                }
                *(*f).f_un.f_user.f_unames.offset(i as isize) =
                    malloc((q.offset_from(p) + 1isize) as size_t) as *mut ::core::ffi::c_char;
                if !(*(*f).f_un.f_user.f_unames.offset(i as isize)).is_null() {
                    strncpy(
                        *(*f).f_un.f_user.f_unames.offset(i as isize),
                        p,
                        q.offset_from(p) as size_t,
                    );
                    *(*(*f).f_un.f_user.f_unames.offset(i as isize))
                        .offset(q.offset_from(p) as isize) = '\0' as ::core::ffi::c_char;
                }
                while *q as ::core::ffi::c_int == ',' as ::core::ffi::c_int
                    || *q as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                {
                    q = q.offset(1);
                }
                p = q;
                i += 1;
            }
            (*f).f_type = F_USERS as ::core::ffi::c_short;
        }
    }
    if !selector.is_null() {
        (*f).f_progname = strdup(selector);
        (*f).f_prognlen = strlen(selector) as ::core::ffi::c_int;
    } else {
        (*f).f_progname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    };
}
#[export_name = "rboxc_syslogd_decode"]
pub unsafe extern "C" fn decode(
    mut name: *const ::core::ffi::c_char,
    mut codetab: *mut CODE,
) -> ::core::ffi::c_int {
    let mut c: *mut CODE = ::core::ptr::null_mut::<CODE>();
    if *(*__ctype_b_loc()).offset(*name as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & C2Rust_Unnamed_12::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        return atoi(name);
    }
    c = codetab;
    while !(*c).c_name.is_null() {
        if strcasecmp(name, (*c).c_name) == 0 {
            return (*c).c_val;
        }
        c = c.offset(1);
    }
    return -1 as ::core::ffi::c_int;
}
#[export_name = "rboxc_syslogd_dbg_toggle"]
pub unsafe extern "C" fn dbg_toggle(mut signo: ::core::ffi::c_int) {
    let mut dbg_save: ::core::ffi::c_int = dbg_output;
    dbg_output = 1 as ::core::ffi::c_int;
    dbg_printf(
        b"Switching dbg_output to %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
        if dbg_save == 0 as ::core::ffi::c_int {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    dbg_output = if dbg_save == 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn dbg_printf(mut fmt: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut ap: ::core::ffi::VaList;
    if !(NoDetach != 0 && dbg_output != 0) {
        return;
    }
    ap = c2rust_args.clone();
    vfprintf(stdout, fmt, ap);
    fflush(stdout);
}
#[export_name = "rboxc_syslogd_trigger_restart"]
pub unsafe extern "C" fn trigger_restart(mut signo: ::core::ffi::c_int) {
    restart = 1 as ::core::ffi::c_int;
}
#[export_name = "rboxc_syslogd_find_inet_port"]
pub unsafe extern "C" fn find_inet_port(mut port: *const ::core::ffi::c_char) {
    let mut err: ::core::ffi::c_int = 0;
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: ::core::ptr::null_mut::<sockaddr>(),
        ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ai_next: ::core::ptr::null_mut::<addrinfo>(),
    };
    let mut ai: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    LogForwardPort = b"514\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
    hints.ai_flags = AI_PASSIVE;
    err = getaddrinfo(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"syslog\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut hints,
        &raw mut ai,
    );
    if err == 0 as ::core::ffi::c_int {
        LogForwardPort =
            b"syslog\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        freeaddrinfo(ai);
    }
    LogPortText = port as *mut ::core::ffi::c_char;
    if LogPortText.is_null() {
        LogPortText = LogForwardPort;
        return;
    }
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_family = AF_UNSPEC;
    hints.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
    hints.ai_flags = AI_PASSIVE;
    err = getaddrinfo(
        ::core::ptr::null::<::core::ffi::c_char>(),
        LogPortText,
        &raw mut hints,
        &raw mut ai,
    );
    if err != 0 as ::core::ffi::c_int {
        LogPortText = ::core::ptr::null_mut::<::core::ffi::c_char>();
    } else {
        freeaddrinfo(ai);
    };
}
pub const PATH_LOGCONF: [::core::ffi::c_char; 51] = unsafe {
    ::core::mem::transmute::<[u8; 51], [::core::ffi::c_char; 51]>(
        *b"/root/rboxc/build/oracle/inetutils/etc/syslog.conf\0",
    )
};
pub const PATH_LOGCONFD: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/inetutils/etc/syslog.d\0",
    )
};
pub const PATH_LOGPID: [::core::ffi::c_char; 54] = unsafe {
    ::core::mem::transmute::<[u8; 54], [::core::ffi::c_char; 54]>(
        *b"/root/rboxc/build/oracle/inetutils/var/run/syslog.pid\0",
    )
};
