// Generated from pinned GNU inetd 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 6fc00b52c811c0bbf2e88f660873bfbe035a3befc8f7d25d5a2020ea0115ad4f
/*
  Copyright (C) 1994-2026 Free Software Foundation, Inc.

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
pub struct sockaddr_un { _opaque: [u8; 0] }
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
pub struct __dirstream { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sigprocmask(
        __how: ::core::ffi::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> ::core::ffi::c_int;
    fn sigsuspend(__set: *const sigset_t) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn alarm(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execv(
        __path: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn getpid() -> __pid_t;
    fn setsid() -> __pid_t;
    fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
    fn setgid(__gid: __gid_t) -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    #[link_name = "rboxc_inetd_daemon"]
    fn daemon(__nochdir: ::core::ffi::c_int, __noclose: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut ::core::ffi::c_char;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
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
    fn getsockname(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn getpeername(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
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
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn accept(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn wait3(
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
        __usage: *mut rusage,
    ) -> __pid_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn opendir(__name: *const ::core::ffi::c_char) -> *mut DIR;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn getprotobyname(__name: *const ::core::ffi::c_char) -> *mut protoent;
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
    fn getpwnam(__name: *const ::core::ffi::c_char) -> *mut passwd;
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
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_inetd_rpl_getline"]
    fn rpl_getline(
        lineptr: *mut *mut ::core::ffi::c_char,
        linesize: *mut size_t,
        stream: *mut FILE,
    ) -> ssize_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strspn(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    #[link_name = "rboxc_inetd_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_inetd_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_inetd_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_inetd_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn getgrnam(__name: *const ::core::ffi::c_char) -> *mut group;
    fn initgroups(__user: *const ::core::ffi::c_char, __group: __gid_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_inetd_argcv_get"]
    fn argcv_get(
        command: *const ::core::ffi::c_char,
        delim: *const ::core::ffi::c_char,
        argc: *mut ::core::ffi::c_int,
        argv: *mut *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_inetd_argcv_free"]
    fn argcv_free(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
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
pub type __suseconds_t = ::core::ffi::c_long;
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
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2Rust_Unnamed_24,
    pub c2rust_unnamed_0: C2Rust_Unnamed_23,
    pub c2rust_unnamed_1: C2Rust_Unnamed_22,
    pub c2rust_unnamed_2: C2Rust_Unnamed_21,
    pub c2rust_unnamed_3: C2Rust_Unnamed_20,
    pub c2rust_unnamed_4: C2Rust_Unnamed_19,
    pub c2rust_unnamed_5: C2Rust_Unnamed_18,
    pub c2rust_unnamed_6: C2Rust_Unnamed_17,
    pub c2rust_unnamed_7: C2Rust_Unnamed_16,
    pub c2rust_unnamed_8: C2Rust_Unnamed_15,
    pub c2rust_unnamed_9: C2Rust_Unnamed_14,
    pub c2rust_unnamed_10: C2Rust_Unnamed_13,
    pub c2rust_unnamed_11: C2Rust_Unnamed_12,
    pub c2rust_unnamed_12: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub ru_nivcsw: ::core::ffi::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub ru_nvcsw: ::core::ffi::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub ru_nsignals: ::core::ffi::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_14 {
    pub ru_msgrcv: ::core::ffi::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub ru_msgsnd: ::core::ffi::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_16 {
    pub ru_oublock: ::core::ffi::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_17 {
    pub ru_inblock: ::core::ffi::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_18 {
    pub ru_nswap: ::core::ffi::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_19 {
    pub ru_majflt: ::core::ffi::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_20 {
    pub ru_minflt: ::core::ffi::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_21 {
    pub ru_isrss: ::core::ffi::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_22 {
    pub ru_idrss: ::core::ffi::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_23 {
    pub ru_ixrss: ::core::ffi::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_24 {
    pub ru_maxrss: ::core::ffi::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_25(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_25 {
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: ::core::ffi::c_ushort,
    pub d_type: ::core::ffi::c_uchar,
    pub d_name: [::core::ffi::c_char; 256],
}
pub type DIR = __dirstream;
pub type error_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct protoent {
    pub p_name: *mut ::core::ffi::c_char,
    pub p_aliases: *mut *mut ::core::ffi::c_char,
    pub p_proto: ::core::ffi::c_int,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_26(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_26 {
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
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_27(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_27 {
    pub const OPT_ENVIRON: Self = Self(256);
    pub const OPT_FOREGROUND: Self = Self(257);
    pub const OPT_RESOLVE: Self = Self(258);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct servtab {
    pub se_file: *const ::core::ffi::c_char,
    pub se_line: ::core::ffi::c_int,
    pub se_node: *mut ::core::ffi::c_char,
    pub se_service: *mut ::core::ffi::c_char,
    pub se_socktype: ::core::ffi::c_int,
    pub se_proto: *mut ::core::ffi::c_char,
    pub se_wait: pid_t,
    pub se_max: ::core::ffi::c_uint,
    pub se_checked: ::core::ffi::c_short,
    pub se_user: *mut ::core::ffi::c_char,
    pub se_group: *mut ::core::ffi::c_char,
    pub se_bi: *mut biltin,
    pub se_server: *mut ::core::ffi::c_char,
    pub se_argv: *mut *mut ::core::ffi::c_char,
    pub se_argc: size_t,
    pub se_fd: ::core::ffi::c_int,
    pub se_type: ::core::ffi::c_int,
    pub se_family: sa_family_t,
    pub se_v4mapped: ::core::ffi::c_char,
    pub se_ctrladdr: sockaddr_storage,
    pub se_addrlen: socklen_t,
    pub se_refcnt: ::core::ffi::c_uint,
    pub se_count: ::core::ffi::c_uint,
    pub se_time: timeval,
    pub se_next: *mut servtab,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct biltin {
    pub bi_service: *const ::core::ffi::c_char,
    pub bi_socktype: ::core::ffi::c_int,
    pub bi_fork: ::core::ffi::c_short,
    pub bi_wait: ::core::ffi::c_short,
    pub bi_fn: Option<unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()>,
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as __uint32_t) >> 24 as ::core::ffi::c_int
        | (__bsx & 0xff0000 as __uint32_t) >> 8 as ::core::ffi::c_int
        | (__bsx & 0xff00 as __uint32_t) << 8 as ::core::ffi::c_int
        | (__bsx & 0xff as __uint32_t) << 24 as ::core::ffi::c_int;
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGALRM: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const SIG_BLOCK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIG_SETMASK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_DEBUG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const WNOHANG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const EADDRNOTAVAIL: ::core::ffi::c_int = 99 as ::core::ffi::c_int;
pub const IPV6_V6ONLY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const AI_PASSIVE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AI_NUMERICHOST: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const AI_V4MAPPED: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const EAI_SYSTEM: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const EAI_ADDRFAMILY: ::core::ffi::c_int = -9 as ::core::ffi::c_int;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NI_NUMERICSERV: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_CRIT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_INFO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_NOWAIT: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPTION_ARG_OPTIONAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const TOOMANY: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const CNT_INTVL: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const RETRYTIME: ::core::ffi::c_int = 60 as ::core::ffi::c_int * 10 as ::core::ffi::c_int;
#[export_name = "rboxc_inetd_debug"]
pub static mut debug: bool = r#false != 0;
#[export_name = "rboxc_inetd_foreground"]
pub static mut foreground: bool = r#false != 0;
#[export_name = "rboxc_inetd_nsock"]
pub static mut nsock: ::core::ffi::c_int = 0;
#[export_name = "rboxc_inetd_maxsock"]
pub static mut maxsock: ::core::ffi::c_int = 0;
#[export_name = "rboxc_inetd_allsock"]
pub static mut allsock: fd_set = fd_set { fds_bits: [0; 16] };
#[export_name = "rboxc_inetd_options"]
pub static mut options: ::core::ffi::c_int = 0;
#[export_name = "rboxc_inetd_timingout"]
pub static mut timingout: ::core::ffi::c_int = 0;
#[export_name = "rboxc_inetd_toomany"]
pub static mut toomany: ::core::ffi::c_uint = TOOMANY as ::core::ffi::c_uint;
#[export_name = "rboxc_inetd_Argv"]
pub static mut Argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
#[export_name = "rboxc_inetd_LastArg"]
pub static mut LastArg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_inetd_config_files"]
pub static mut config_files: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
static mut env_option: bool = r#false != 0;
static mut resolve_option: bool = r#false != 0;
static mut pidfile_option: bool = r#true != 0;
static mut pid_file: *const ::core::ffi::c_char = PATH_INETDPID.as_ptr();
#[export_name = "rboxc_inetd_args_doc"]
pub static mut args_doc: [::core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(*b"[CONF-FILE [CONF-DIR]]...\0")
};
#[export_name = "rboxc_inetd_doc"]
pub static mut doc: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"Internet super-server.\0")
};
#[export_name = "rboxc_inetd_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 10] = [
    b"Alain Magloire\0".as_ptr() as *const ::core::ffi::c_char,
    b"Alfred M. Szmidt\0".as_ptr() as *const ::core::ffi::c_char,
    b"Debarshi Ray\0".as_ptr() as *const ::core::ffi::c_char,
    b"Jakob 'sparky' Kaivo\0".as_ptr() as *const ::core::ffi::c_char,
    b"Jeff Bailey\0".as_ptr() as *const ::core::ffi::c_char,
    b"Jeroen Dekkers\0".as_ptr() as *const ::core::ffi::c_char,
    b"Marcus Brinkmann\0".as_ptr() as *const ::core::ffi::c_char,
    b"Sergey Poznyakoff\0".as_ptr() as *const ::core::ffi::c_char,
    b"others\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut argp_options: [argp_option; 7] = [
    argp_option {
        name: b"foreground\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_27::OPT_FOREGROUND.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"run in foreground mode\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'd' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"turn on debugging, run in foreground mode\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"environment\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_27::OPT_ENVIRON.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"pass local and remote socket information in environment variables\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"pidfile\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: b"PIDFILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"override pidfile (default: \"/root/rboxc/build/oracle/inetutils/var/run/inetd.pid\")\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rate\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'R' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"maximum invocation rate (per minute)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"resolve\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_27::OPT_RESOLVE.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"resolve IP addresses when setting environment variables (see --environment)\0"
            .as_ptr() as *const ::core::ffi::c_char,
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
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut number: ::core::ffi::c_int = 0;
    match key {
        100 => {
            debug = r#true != 0;
            foreground = r#true != 0;
            options |= SO_DEBUG;
        }
        257 => {
            foreground = r#true != 0;
        }
        256 => {
            env_option = r#true != 0;
        }
        112 => {
            if !arg.is_null() && strlen(arg) != 0 {
                pid_file = arg;
            } else {
                pidfile_option = r#false != 0;
            }
        }
        82 => {
            number = strtol(arg, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if number < 1 as ::core::ffi::c_int || *p as ::core::ffi::c_int != 0 {
                syslog(
                    LOG_ERR,
                    b"-R %s: bad value for service invocation rate\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    arg,
                );
            } else {
                toomany = number as ::core::ffi::c_uint;
            }
        }
        258 => {
            resolve_option = r#true != 0;
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
#[export_name = "rboxc_inetd_servtab"]
pub static mut servtab: *mut servtab = ::core::ptr::null_mut::<servtab>();
pub const NORM_TYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const MUX_TYPE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MUXPLUS_TYPE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[export_name = "rboxc_inetd_biltins"]
pub static mut biltins: [biltin; 12] = [
    biltin {
        bi_service: b"echo\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        bi_fork: 1 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(echo_stream as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"echo\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(echo_dg as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"discard\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        bi_fork: 1 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(discard_stream as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"discard\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(discard_dg as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"time\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(
            machtime_stream as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> (),
        ),
    },
    biltin {
        bi_service: b"time\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(machtime_dg as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"daytime\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(daytime_stream as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"daytime\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(daytime_dg as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"chargen\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        bi_fork: 1 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(chargen_stream as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"chargen\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(chargen_dg as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: b"tcpmux\0".as_ptr() as *const ::core::ffi::c_char,
        bi_socktype: __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        bi_fork: 1 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: Some(tcpmux as unsafe extern "C" fn(::core::ffi::c_int, *mut servtab) -> ()),
    },
    biltin {
        bi_service: ::core::ptr::null::<::core::ffi::c_char>(),
        bi_socktype: 0 as ::core::ffi::c_int,
        bi_fork: 0 as ::core::ffi::c_short,
        bi_wait: 0 as ::core::ffi::c_short,
        bi_fn: None,
    },
];
#[export_name = "rboxc_inetd_bi_lookup"]
pub unsafe extern "C" fn bi_lookup(mut sep: *const servtab) -> *mut biltin {
    let mut bi: *mut biltin = ::core::ptr::null_mut::<biltin>();
    bi = &raw mut biltins as *mut biltin as *mut biltin;
    while !(*bi).bi_service.is_null() {
        if (*bi).bi_socktype == (*sep).se_socktype
            && strcmp((*bi).bi_service, (*sep).se_service) == 0 as ::core::ffi::c_int
        {
            return bi;
        }
        bi = bi.offset(1);
    }
    return ::core::ptr::null_mut::<biltin>();
}
#[export_name = "rboxc_inetd_signal_set_handler"]
pub unsafe extern "C" fn signal_set_handler(
    mut signo: ::core::ffi::c_int,
    mut handler: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
) {
    let mut sa: sigaction = sigaction {
        __sigaction_handler: C2Rust_Unnamed_9 { sa_handler: None },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    memset(
        &raw mut sa as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sigaction>(),
    );
    sigemptyset(&raw mut sa.sa_mask);
    sigaddset(&raw mut sa.sa_mask, signo);
    sa.sa_flags = SA_RESTART;
    sa.__sigaction_handler.sa_handler = handler as __sighandler_t;
    sigaction(signo, &raw mut sa, ::core::ptr::null_mut::<sigaction>());
}
#[export_name = "rboxc_inetd_signal_block"]
pub unsafe extern "C" fn signal_block(mut old_status: *mut sigset_t) {
    let mut sigs: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&raw mut sigs);
    sigaddset(&raw mut sigs, SIGCHLD);
    sigaddset(&raw mut sigs, SIGHUP);
    sigaddset(&raw mut sigs, SIGALRM);
    sigprocmask(SIG_BLOCK, &raw mut sigs, old_status);
}
#[export_name = "rboxc_inetd_signal_unblock"]
pub unsafe extern "C" fn signal_unblock(mut status: *mut sigset_t) {
    if !status.is_null() {
        sigprocmask(SIG_SETMASK, status, ::core::ptr::null_mut::<sigset_t>());
    } else {
        let mut empty: sigset_t = __sigset_t { __val: [0; 16] };
        sigemptyset(&raw mut empty);
        sigprocmask(
            SIG_SETMASK,
            &raw mut empty,
            ::core::ptr::null_mut::<sigset_t>(),
        );
    };
}
#[export_name = "rboxc_inetd_run_service"]
pub unsafe extern "C" fn run_service(mut ctrl: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut pwd: *mut passwd = ::core::ptr::null_mut::<passwd>();
    let mut grp: *mut group = ::core::ptr::null_mut::<group>();
    let mut buf: [::core::ffi::c_char; 50] = [0; 50];
    if !(*sep).se_bi.is_null() {
        Some((*(*sep).se_bi).bi_fn.expect("non-null function pointer"))
            .expect("non-null function pointer")(ctrl, sep);
    } else {
        if debug {
            fprintf(
                stderr,
                b"%d execl %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                getpid(),
                (*sep).se_server,
            );
        }
        dup2(ctrl, 0 as ::core::ffi::c_int);
        close(ctrl);
        dup2(0 as ::core::ffi::c_int, 1 as ::core::ffi::c_int);
        dup2(0 as ::core::ffi::c_int, 2 as ::core::ffi::c_int);
        pwd = getpwnam((*sep).se_user);
        if pwd.is_null() {
            syslog(
                LOG_ERR,
                b"%s/%s: %s: No such user\0".as_ptr() as *const ::core::ffi::c_char,
                (*sep).se_service,
                (*sep).se_proto,
                (*sep).se_user,
            );
            if (*sep).se_socktype != __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int {
                recv(
                    0 as ::core::ffi::c_int,
                    &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 50]>(),
                    0 as ::core::ffi::c_int,
                );
            }
            _exit(EXIT_FAILURE);
        }
        if !(*sep).se_group.is_null() && *(*sep).se_group as ::core::ffi::c_int != 0 {
            grp = getgrnam((*sep).se_group);
            if grp.is_null() {
                syslog(
                    LOG_ERR,
                    b"%s/%s: %s: No such group\0".as_ptr() as *const ::core::ffi::c_char,
                    (*sep).se_service,
                    (*sep).se_proto,
                    (*sep).se_group,
                );
                if (*sep).se_socktype != __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int {
                    recv(
                        0 as ::core::ffi::c_int,
                        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_char; 50]>(),
                        0 as ::core::ffi::c_int,
                    );
                }
                _exit(EXIT_FAILURE);
            }
        }
        if (*pwd).pw_uid != 0 {
            if !grp.is_null() && (*grp).gr_gid != 0 {
                if setgid((*grp).gr_gid) < 0 as ::core::ffi::c_int {
                    syslog(
                        LOG_ERR,
                        b"%s: can't set gid %d: %m\0".as_ptr() as *const ::core::ffi::c_char,
                        (*sep).se_service,
                        (*grp).gr_gid,
                    );
                    _exit(EXIT_FAILURE);
                }
            } else if setgid((*pwd).pw_gid) < 0 as ::core::ffi::c_int {
                syslog(
                    LOG_ERR,
                    b"%s: can't set gid %d: %m\0".as_ptr() as *const ::core::ffi::c_char,
                    (*sep).se_service,
                    (*pwd).pw_gid,
                );
                _exit(EXIT_FAILURE);
            }
            initgroups(
                (*pwd).pw_name,
                if !grp.is_null() && (*grp).gr_gid != 0 {
                    (*grp).gr_gid
                } else {
                    (*pwd).pw_gid
                },
            );
            if setuid((*pwd).pw_uid) < 0 as ::core::ffi::c_int {
                syslog(
                    LOG_ERR,
                    b"%s: can't set uid %d: %m\0".as_ptr() as *const ::core::ffi::c_char,
                    (*sep).se_service,
                    (*pwd).pw_uid,
                );
                _exit(EXIT_FAILURE);
            }
        }
        execv(
            (*sep).se_server,
            (*sep).se_argv as *const *mut ::core::ffi::c_char,
        );
        if (*sep).se_socktype != __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int {
            recv(
                0 as ::core::ffi::c_int,
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 50]>(),
                0 as ::core::ffi::c_int,
            );
        }
        syslog(
            LOG_ERR,
            b"cannot execute %s: %m\0".as_ptr() as *const ::core::ffi::c_char,
            (*sep).se_server,
        );
        _exit(EXIT_FAILURE);
    };
}
#[export_name = "rboxc_inetd_reapchild"]
pub unsafe extern "C" fn reapchild(mut signo: ::core::ffi::c_int) {
    let mut status: ::core::ffi::c_int = 0;
    let mut pid: pid_t = 0;
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    loop {
        pid = wait3(&raw mut status, WNOHANG, ::core::ptr::null_mut::<rusage>()) as pid_t;
        if pid <= 0 as ::core::ffi::c_int {
            break;
        }
        if debug {
            fprintf(
                stderr,
                b"%d reaped, status %#x\n\0".as_ptr() as *const ::core::ffi::c_char,
                pid,
                status,
            );
        }
        sep = servtab as *mut servtab;
        while !sep.is_null() {
            if (*sep).se_wait == pid {
                if status != 0 {
                    syslog(
                        LOG_WARNING,
                        b"%s: exit status 0x%x\0".as_ptr() as *const ::core::ffi::c_char,
                        (*sep).se_server,
                        status,
                    );
                }
                if debug {
                    fprintf(
                        stderr,
                        b"restored %s, fd %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*sep).se_service,
                        (*sep).se_fd,
                    );
                }
                allsock.fds_bits[((*sep).se_fd / __NFDBITS) as usize] |=
                    ((1 as ::core::ffi::c_ulong) << (*sep).se_fd % __NFDBITS) as __fd_mask;
                nsock += 1;
                (*sep).se_wait = 1 as ::core::ffi::c_int as pid_t;
            }
            sep = (*sep).se_next;
        }
    }
}
#[export_name = "rboxc_inetd_newstr"]
pub unsafe extern "C" fn newstr(mut cp: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = strdup(if !cp.is_null() {
        cp
    } else {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    });
    if !s.is_null() {
        return s;
    }
    syslog(
        LOG_ERR,
        b"strdup: %m\0".as_ptr() as *const ::core::ffi::c_char,
    );
    exit(-1 as ::core::ffi::c_int);
}
#[export_name = "rboxc_inetd_dupmem"]
pub unsafe extern "C" fn dupmem(mut pptr: *mut *mut ::core::ffi::c_void, mut size: size_t) {
    let mut ptr: *mut ::core::ffi::c_void = malloc(size);
    if ptr.is_null() {
        syslog(
            LOG_ERR,
            b"dupmem: %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(-1 as ::core::ffi::c_int);
    }
    memcpy(ptr, *pptr, size);
    *pptr = ptr;
}
#[export_name = "rboxc_inetd_dupstr"]
pub unsafe extern "C" fn dupstr(mut pstr: *mut *mut ::core::ffi::c_char) {
    if !(*pstr).is_null() {
        dupmem(
            pstr as *mut *mut ::core::ffi::c_void,
            strlen(*pstr).wrapping_add(1 as size_t),
        );
    }
}
#[export_name = "rboxc_inetd_print_service"]
pub unsafe extern "C" fn print_service(
    mut action: *const ::core::ffi::c_char,
    mut sep: *mut servtab,
) {
    fprintf(
        stderr,
        b"%s:%d: %s: %s:%s proto=%s, wait=%d, max=%u, user=%s group=%s builtin=%s server=%s\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*sep).se_file,
        (*sep).se_line,
        action,
        if (*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE {
            if (*sep).se_type == MUXPLUS_TYPE {
                b"tcpmuxplus\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"tcpmux\0".as_ptr() as *const ::core::ffi::c_char
            }
        } else if !(*sep).se_node.is_null() {
            (*sep).se_node as *const ::core::ffi::c_char
        } else {
            b"*\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*sep).se_service,
        (*sep).se_proto,
        (*sep).se_wait,
        (*sep).se_max,
        (*sep).se_user,
        (*sep).se_group,
        if !(*sep).se_bi.is_null() {
            (*(*sep).se_bi).bi_service
        } else {
            b"no\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*sep).se_server,
    );
}
#[export_name = "rboxc_inetd_setup"]
pub unsafe extern "C" fn setup(mut sep: *mut servtab) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut on: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    loop {
        (*sep).se_fd = socket(
            (*sep).se_family as ::core::ffi::c_int,
            (*sep).se_socktype,
            0 as ::core::ffi::c_int,
        );
        if (*sep).se_fd < 0 as ::core::ffi::c_int {
            if *__errno_location() == EAFNOSUPPORT
                && (*sep).se_family as ::core::ffi::c_int == AF_INET6
                && (*sep).se_v4mapped as ::core::ffi::c_int != 0
            {
                (*sep).se_family = AF_INET as sa_family_t;
            } else {
                if debug {
                    fprintf(
                        stderr,
                        b"socket failed on %s/%s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*sep).se_service,
                        (*sep).se_proto,
                        strerror(*__errno_location()),
                    );
                }
                syslog(
                    LOG_ERR,
                    b"%s/%s: socket: %m\0".as_ptr() as *const ::core::ffi::c_char,
                    (*sep).se_service,
                    (*sep).se_proto,
                );
                return 1 as ::core::ffi::c_int;
            }
        } else {
            if (*sep).se_family as ::core::ffi::c_int == AF_INET6 {
                let mut val: ::core::ffi::c_int = if (*sep).se_v4mapped as ::core::ffi::c_int != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                };
                if setsockopt(
                    (*sep).se_fd,
                    C2Rust_Unnamed_25::IPPROTO_IPV6.0 as ::core::ffi::c_int,
                    IPV6_V6ONLY,
                    &raw mut val as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                ) < 0 as ::core::ffi::c_int
                {
                    syslog(
                        LOG_ERR,
                        b"setsockopt (IPV6_V6ONLY): %m\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            if strncmp(
                (*sep).se_proto,
                b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
                && options & SO_DEBUG != 0
            {
                if setsockopt(
                    (*sep).se_fd,
                    SOL_SOCKET,
                    SO_DEBUG,
                    &raw mut on as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
                ) < 0 as ::core::ffi::c_int
                    && *__errno_location() != EACCES
                {
                    syslog(
                        LOG_ERR,
                        b"setsockopt (SO_DEBUG): %m\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            }
            err = setsockopt(
                (*sep).se_fd,
                SOL_SOCKET,
                SO_REUSEADDR,
                &raw mut on as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
            );
            if err < 0 as ::core::ffi::c_int {
                syslog(
                    LOG_ERR,
                    b"setsockopt (SO_REUSEADDR): %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            err = bind(
                (*sep).se_fd,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &raw mut (*sep).se_ctrladdr as *mut sockaddr,
                },
                (*sep).se_addrlen,
            );
            if err < 0 as ::core::ffi::c_int {
                if (*__errno_location() == EADDRNOTAVAIL || *__errno_location() == EAFNOSUPPORT)
                    && (*sep).se_family as ::core::ffi::c_int == AF_INET6
                    && (*sep).se_v4mapped as ::core::ffi::c_int != 0
                {
                    (*sep).se_family = AF_INET as sa_family_t;
                    close((*sep).se_fd);
                } else {
                    if !(*sep).se_node.is_null() {
                        if debug {
                            fprintf(
                                stderr,
                                b"bind failed for %s %s/%s: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*sep).se_node,
                                (*sep).se_service,
                                (*sep).se_proto,
                                strerror(*__errno_location()),
                            );
                        }
                        syslog(
                            LOG_ERR,
                            b"%s %s/%s: bind: %m\0".as_ptr() as *const ::core::ffi::c_char,
                            (*sep).se_node,
                            (*sep).se_service,
                            (*sep).se_proto,
                        );
                    } else {
                        if debug {
                            fprintf(
                                stderr,
                                b"bind failed for %s/%s: %s\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                (*sep).se_service,
                                (*sep).se_proto,
                                strerror(*__errno_location()),
                            );
                        }
                        syslog(
                            LOG_ERR,
                            b"%s/%s: bind: %m\0".as_ptr() as *const ::core::ffi::c_char,
                            (*sep).se_service,
                            (*sep).se_proto,
                        );
                    }
                    close((*sep).se_fd);
                    (*sep).se_fd = -1 as ::core::ffi::c_int;
                    if timingout == 0 {
                        timingout = 1 as ::core::ffi::c_int;
                        alarm(RETRYTIME as ::core::ffi::c_uint);
                    }
                    return 1 as ::core::ffi::c_int;
                }
            } else {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
}
#[export_name = "rboxc_inetd_servent_setup"]
pub unsafe extern "C" fn servent_setup(mut sep: *mut servtab) {
    (*sep).se_checked = 1 as ::core::ffi::c_short;
    if (*sep).se_fd == -1 as ::core::ffi::c_int && setup(sep) == 0 as ::core::ffi::c_int {
        if (*sep).se_socktype == __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int {
            listen((*sep).se_fd, 10 as ::core::ffi::c_int);
        }
        allsock.fds_bits[((*sep).se_fd / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << (*sep).se_fd % __NFDBITS) as __fd_mask;
        nsock += 1;
        if (*sep).se_fd > maxsock {
            maxsock = (*sep).se_fd;
        }
        if debug {
            fprintf(
                stderr,
                b"registered %s on %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*sep).se_server,
                (*sep).se_fd,
            );
        }
    }
}
#[export_name = "rboxc_inetd_retry"]
pub unsafe extern "C" fn retry(mut signo: ::core::ffi::c_int) {
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    timingout = 0 as ::core::ffi::c_int;
    sep = servtab as *mut servtab;
    while !sep.is_null() {
        if (*sep).se_fd == -1 as ::core::ffi::c_int
            && !((*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE)
        {
            setup(sep);
        }
        sep = (*sep).se_next;
    }
}
#[export_name = "rboxc_inetd_close_sep"]
pub unsafe extern "C" fn close_sep(mut sep: *mut servtab) {
    if (*sep).se_fd >= 0 as ::core::ffi::c_int {
        nsock -= 1;
        allsock.fds_bits[((*sep).se_fd / __NFDBITS) as usize] &=
            !(((1 as ::core::ffi::c_ulong) << (*sep).se_fd % __NFDBITS) as __fd_mask);
        close((*sep).se_fd);
        (*sep).se_fd = -1 as ::core::ffi::c_int;
    }
    (*sep).se_count = 0 as ::core::ffi::c_uint;
    if (*sep).se_wait > 1 as ::core::ffi::c_int {
        (*sep).se_wait = 1 as ::core::ffi::c_int as pid_t;
    }
}
#[export_name = "rboxc_inetd_enter"]
pub unsafe extern "C" fn enter(mut cp: *mut servtab) -> *mut servtab {
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    let mut sigstatus: sigset_t = __sigset_t { __val: [0; 16] };
    let mut i: size_t = 0;
    sep = servtab as *mut servtab;
    while !sep.is_null() {
        if memcmp(
            &raw mut (*sep).se_ctrladdr as *const ::core::ffi::c_void,
            &raw mut (*cp).se_ctrladdr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<sockaddr_storage>(),
        ) == 0 as ::core::ffi::c_int
            && strcmp((*sep).se_service, (*cp).se_service) == 0 as ::core::ffi::c_int
            && strcmp((*sep).se_proto, (*cp).se_proto) == 0 as ::core::ffi::c_int
            && ((*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE) as ::core::ffi::c_int
                == ((*cp).se_type == MUX_TYPE || (*cp).se_type == MUXPLUS_TYPE)
                    as ::core::ffi::c_int
        {
            break;
        }
        sep = (*sep).se_next;
    }
    if !sep.is_null() {
        signal_block(&raw mut sigstatus);
        if (*cp).se_bi.is_null()
            && ((*sep).se_wait == 1 as ::core::ffi::c_int
                || (*cp).se_wait == 0 as ::core::ffi::c_int)
        {
            (*sep).se_wait = (*cp).se_wait;
        }
        if !(*cp).se_user.is_null() {
            let mut c: *mut ::core::ffi::c_char = (*sep).se_user;
            (*sep).se_user = (*cp).se_user;
            (*cp).se_user = c;
        }
        if !(*cp).se_group.is_null() {
            let mut c_0: *mut ::core::ffi::c_char = (*sep).se_group;
            (*sep).se_group = (*cp).se_group;
            (*cp).se_group = c_0;
        }
        if !(*cp).se_server.is_null() {
            let mut c_1: *mut ::core::ffi::c_char = (*sep).se_server;
            (*sep).se_server = (*cp).se_server;
            (*cp).se_server = c_1;
        }
        argcv_free((*sep).se_argc as ::core::ffi::c_int, (*sep).se_argv);
        (*sep).se_argc = (*cp).se_argc;
        (*sep).se_argv = (*cp).se_argv;
        (*cp).se_argc = 0 as size_t;
        (*cp).se_argv = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        (*sep).se_checked = 1 as ::core::ffi::c_short;
        signal_unblock(&raw mut sigstatus);
        if debug {
            print_service(b"REDO\0".as_ptr() as *const ::core::ffi::c_char, sep);
        }
        return sep;
    }
    if debug {
        print_service(b"ADD \0".as_ptr() as *const ::core::ffi::c_char, cp);
    }
    sep = malloc(::core::mem::size_of::<servtab>()) as *mut servtab;
    if sep.is_null() {
        syslog(
            LOG_ERR,
            b"Out of memory.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(-1 as ::core::ffi::c_int);
    }
    *sep = *cp;
    dupstr(&raw mut (*sep).se_node);
    dupstr(&raw mut (*sep).se_service);
    dupstr(&raw mut (*sep).se_proto);
    dupstr(&raw mut (*sep).se_user);
    dupstr(&raw mut (*sep).se_group);
    dupstr(&raw mut (*sep).se_server);
    dupmem(
        &raw mut (*sep).se_argv as *mut *mut ::core::ffi::c_void,
        (*sep)
            .se_argc
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    );
    i = 0 as size_t;
    while i < (*sep).se_argc {
        dupstr((*sep).se_argv.offset(i as isize));
        i = i.wrapping_add(1);
    }
    (*sep).se_fd = -1 as ::core::ffi::c_int;
    signal_block(&raw mut sigstatus);
    (*sep).se_next = servtab as *mut servtab;
    servtab = sep as *mut servtab;
    signal_unblock(&raw mut sigstatus);
    return sep;
}
pub const IPV4_NUMCHARS: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b".0123456789\0") };
pub const IPV6_NUMCHARS: [::core::ffi::c_char; 25] = unsafe {
    ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(*b".:0123456789abcdefABCDEF\0")
};
#[export_name = "rboxc_inetd_inetd_getaddrinfo"]
pub unsafe extern "C" fn inetd_getaddrinfo(
    mut sep: *mut servtab,
    mut proto: ::core::ffi::c_int,
    mut result: *mut *mut addrinfo,
) -> ::core::ffi::c_int {
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
    let mut numeric_address: bool = r#false != 0;
    if !(*sep).se_node.is_null()
        && (strspn((*sep).se_node, IPV4_NUMCHARS.as_ptr()) as size_t == strlen((*sep).se_node)
            || !strchr((*sep).se_node, ':' as ::core::ffi::c_int).is_null()
                && strspn((*sep).se_node, IPV6_NUMCHARS.as_ptr()) != 0)
    {
        numeric_address = r#true != 0;
    } else if debug as ::core::ffi::c_int != 0 && !(*sep).se_node.is_null() {
        fprintf(
            stderr,
            b"Resolving address: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*sep).se_node,
        );
    }
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_flags = AI_PASSIVE;
    if (*sep).se_v4mapped as ::core::ffi::c_int != 0
        && (*sep).se_family as ::core::ffi::c_int != AF_INET
    {
        hints.ai_flags |= AI_V4MAPPED;
    }
    if numeric_address {
        hints.ai_flags |= AI_NUMERICHOST;
    }
    hints.ai_family = (*sep).se_family as ::core::ffi::c_int;
    hints.ai_socktype = (*sep).se_socktype;
    hints.ai_protocol = proto;
    return getaddrinfo((*sep).se_node, (*sep).se_service, &raw mut hints, result);
}
#[export_name = "rboxc_inetd_expand_enter"]
pub unsafe extern "C" fn expand_enter(mut sep: *mut servtab) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut result: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut rp: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut proto: *mut protoent = ::core::ptr::null_mut::<protoent>();
    let mut cp: *mut servtab = ::core::ptr::null_mut::<servtab>();
    if strncmp(
        (*sep).se_proto,
        b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        proto = getprotobyname(b"tcp\0".as_ptr() as *const ::core::ffi::c_char);
    } else if strncmp(
        (*sep).se_proto,
        b"udp\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        proto = getprotobyname(b"udp\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        proto = getprotobyname((*sep).se_proto);
    }
    if proto.is_null() {
        syslog(
            LOG_ERR,
            b"%s: Unknown protocol\0".as_ptr() as *const ::core::ffi::c_char,
            (*sep).se_proto,
        );
        return 1 as ::core::ffi::c_int;
    }
    err = inetd_getaddrinfo(sep, (*proto).p_proto, &raw mut result);
    if err == EAI_ADDRFAMILY
        && (*sep).se_family as ::core::ffi::c_int == AF_INET6
        && (*sep).se_v4mapped as ::core::ffi::c_int != 0
    {
        (*sep).se_family = AF_INET as sa_family_t;
        err = inetd_getaddrinfo(sep, (*proto).p_proto, &raw mut result);
    }
    if err != 0 {
        let mut errmsg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if err == EAI_SYSTEM {
            errmsg = strerror(*__errno_location());
        } else {
            errmsg = gai_strerror(err);
        }
        if !(*sep).se_node.is_null() {
            if debug {
                fprintf(
                    stderr,
                    b"resolution of %s %s/%s failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*sep).se_node,
                    (*sep).se_service,
                    (*sep).se_proto,
                    errmsg,
                );
            }
            syslog(
                LOG_ERR,
                b"%s %s/%s: getaddrinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*sep).se_node,
                (*sep).se_service,
                (*sep).se_proto,
                errmsg,
            );
        } else {
            if debug {
                fprintf(
                    stderr,
                    b"resolution of %s/%s failed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*sep).se_service,
                    (*sep).se_proto,
                    errmsg,
                );
            }
            syslog(
                LOG_ERR,
                b"%s/%s: getaddrinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                (*sep).se_service,
                (*sep).se_proto,
                errmsg,
            );
        }
        return 1 as ::core::ffi::c_int;
    }
    rp = result;
    while !rp.is_null() {
        memset(
            &raw mut (*sep).se_ctrladdr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_storage>(),
        );
        memcpy(
            &raw mut (*sep).se_ctrladdr as *mut ::core::ffi::c_void,
            (*rp).ai_addr as *const ::core::ffi::c_void,
            (*rp).ai_addrlen as size_t,
        );
        (*sep).se_addrlen = (*rp).ai_addrlen;
        cp = enter(sep);
        servent_setup(cp);
        rp = (*rp).ai_next;
    }
    freeaddrinfo(result);
    return 0 as ::core::ffi::c_int;
}
#[export_name = "rboxc_inetd_global_serv_node"]
pub static mut global_serv_node: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_inetd_serv_node"]
pub static mut serv_node: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_inetd_serv_node_offset"]
pub static mut serv_node_offset: size_t = 0;
#[export_name = "rboxc_inetd_linebuf"]
pub static mut linebuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_inetd_linebufsize"]
pub static mut linebufsize: size_t = 0 as size_t;
#[export_name = "rboxc_inetd_setconfig"]
pub unsafe extern "C" fn setconfig(mut file: *const ::core::ffi::c_char) -> *mut FILE {
    return fopen(file, b"r\0".as_ptr() as *const ::core::ffi::c_char);
}
#[export_name = "rboxc_inetd_endconfig"]
pub unsafe extern "C" fn endconfig(mut fconfig: *mut FILE) {
    if !fconfig.is_null() {
        fclose(fconfig);
    }
}
#[export_name = "rboxc_inetd_freeconfig"]
pub unsafe extern "C" fn freeconfig(mut cp: *mut servtab) {
    free((*cp).se_node as *mut ::core::ffi::c_void);
    free((*cp).se_service as *mut ::core::ffi::c_void);
    free((*cp).se_proto as *mut ::core::ffi::c_void);
    free((*cp).se_user as *mut ::core::ffi::c_void);
    free((*cp).se_group as *mut ::core::ffi::c_void);
    free((*cp).se_server as *mut ::core::ffi::c_void);
    argcv_free((*cp).se_argc as ::core::ffi::c_int, (*cp).se_argv);
}
pub const INETD_SERVICE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INETD_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const INETD_PROTOCOL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INETD_WAIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const INETD_USER: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const INETD_SERVER_PATH: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const INETD_SERVER_ARGS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const INETD_FIELDS_MIN: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[export_name = "rboxc_inetd_next_node_sep"]
pub unsafe extern "C" fn next_node_sep(mut sep: *mut servtab) -> *mut servtab {
    if !serv_node.is_null() {
        let mut i: size_t = strcspn(
            serv_node.offset(serv_node_offset as isize),
            b",\0".as_ptr() as *const ::core::ffi::c_char,
        ) as size_t;
        (*sep).se_node = malloc(i.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
        if (*sep).se_node.is_null() {
            syslog(
                LOG_ERR,
                b"malloc: %m\0".as_ptr() as *const ::core::ffi::c_char,
            );
            exit(-1 as ::core::ffi::c_int);
        }
        memcpy(
            (*sep).se_node as *mut ::core::ffi::c_void,
            serv_node.offset(serv_node_offset as isize) as *const ::core::ffi::c_void,
            i,
        );
        *(*sep).se_node.offset(i as isize) = 0 as ::core::ffi::c_char;
        serv_node_offset = serv_node_offset.wrapping_add(i);
        if *serv_node.offset(serv_node_offset as isize) != 0 {
            serv_node_offset = serv_node_offset.wrapping_add(1);
        } else {
            free(serv_node as *mut ::core::ffi::c_void);
            serv_node = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    return sep;
}
#[export_name = "rboxc_inetd_getconfigent"]
pub unsafe extern "C" fn getconfigent(
    mut fconfig: *mut FILE,
    mut file: *const ::core::ffi::c_char,
    mut line: *mut size_t,
) -> *mut servtab {
    static mut serv: servtab = servtab {
        se_file: ::core::ptr::null::<::core::ffi::c_char>(),
        se_line: 0,
        se_node: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        se_service: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        se_socktype: 0,
        se_proto: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        se_wait: 0,
        se_max: 0,
        se_checked: 0,
        se_user: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        se_group: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        se_bi: ::core::ptr::null_mut::<biltin>(),
        se_server: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        se_argv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        se_argc: 0,
        se_fd: 0,
        se_type: 0,
        se_family: 0,
        se_v4mapped: 0,
        se_ctrladdr: sockaddr_storage {
            ss_family: 0,
            __ss_padding: [0; 118],
            __ss_align: 0,
        },
        se_addrlen: 0,
        se_refcnt: 0,
        se_count: 0,
        se_time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        se_next: ::core::ptr::null_mut::<servtab>(),
    };
    let mut sep: *mut servtab = &raw mut serv;
    let mut argc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0;
    let mut argv: *mut *mut ::core::ffi::c_char =
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut node: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut service: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut TCPMUX_TOKEN: [::core::ffi::c_char; 8] =
        unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"tcpmux/\0") };
    if !serv_node.is_null() {
        return next_node_sep(sep);
    }
    memset(
        sep as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<servtab>(),
    );
    loop {
        argcv_free(argc, argv);
        freeconfig(sep);
        memset(
            sep as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<servtab>(),
        );
        loop {
            let mut n: ssize_t = rpl_getline(&raw mut linebuf, &raw mut linebufsize, fconfig);
            if n < 0 as ssize_t {
                return ::core::ptr::null_mut::<servtab>();
            } else {
                if n != 0 as ssize_t {
                    if *linebuf.offset(n - 1 as ssize_t) as ::core::ffi::c_int
                        == '\n' as ::core::ffi::c_int
                    {
                        *linebuf.offset(n - 1 as ssize_t) = 0 as ::core::ffi::c_char;
                    }
                    *line = (*line).wrapping_add(1);
                }
                if !(*linebuf as ::core::ffi::c_int == '#' as ::core::ffi::c_int
                    || *linebuf as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                {
                    break;
                }
            }
        }
        if argcv_get(
            linebuf,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut argc,
            &raw mut argv,
        ) != 0
        {
            continue;
        }
        if argc < INETD_FIELDS_MIN {
            if argc == 1 as ::core::ffi::c_int
                && *(*argv.offset(0isize))
                    .offset(strlen(*argv.offset(0isize)).wrapping_sub(1 as size_t) as isize)
                    as ::core::ffi::c_int
                    == ':' as ::core::ffi::c_int
            {
                *(*argv.offset(0isize))
                    .offset(strlen(*argv.offset(0isize)).wrapping_sub(1 as size_t) as isize) =
                    0 as ::core::ffi::c_char;
                free(global_serv_node as *mut ::core::ffi::c_void);
                if strcmp(
                    *argv.offset(0isize),
                    b"*\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0
                {
                    global_serv_node = newstr(*argv.offset(0isize));
                }
            } else {
                syslog(
                    LOG_ERR,
                    b"%s:%lu: not enough fields\0".as_ptr() as *const ::core::ffi::c_char,
                    file,
                    *line as ::core::ffi::c_ulong,
                );
            }
        } else {
            (*sep).se_file = file;
            (*sep).se_line = *line as ::core::ffi::c_int;
            node = *argv.offset(INETD_SERVICE as isize);
            service = strrchr(node, ':' as ::core::ffi::c_int);
            if service.is_null() {
                if !global_serv_node.is_null() {
                    node = global_serv_node;
                    serv_node = newstr(node);
                    serv_node_offset = 0 as size_t;
                } else {
                    node = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                service = *argv.offset(INETD_SERVICE as isize);
            } else {
                let c2rust_fresh2 = service;
                service = service.offset(1);
                *c2rust_fresh2 = 0 as ::core::ffi::c_char;
                if strcmp(node, b"*\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    node = ::core::ptr::null_mut::<::core::ffi::c_char>();
                } else {
                    serv_node = newstr(node);
                    serv_node_offset = 0 as size_t;
                }
            }
            if strncmp(
                service,
                &raw mut TCPMUX_TOKEN as *mut ::core::ffi::c_char,
                MUX_LEN,
            ) == 0 as ::core::ffi::c_int
            {
                let mut c: *mut ::core::ffi::c_char = service.offset(MUX_LEN as isize);
                if *c as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
                    (*sep).se_type = MUXPLUS_TYPE;
                    c = c.offset(1);
                } else {
                    (*sep).se_type = MUX_TYPE;
                }
                (*sep).se_service = newstr(c);
            } else {
                (*sep).se_service = newstr(service);
                (*sep).se_type = NORM_TYPE;
            }
            if strcmp(
                *argv.offset(INETD_SOCKET as isize),
                b"stream\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_socktype = __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int;
            } else if strcmp(
                *argv.offset(INETD_SOCKET as isize),
                b"dgram\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
            } else if strcmp(
                *argv.offset(INETD_SOCKET as isize),
                b"rdm\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_socktype = __socket_type::SOCK_RDM.0 as ::core::ffi::c_int;
            } else if strcmp(
                *argv.offset(INETD_SOCKET as isize),
                b"seqpacket\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_socktype = __socket_type::SOCK_SEQPACKET.0 as ::core::ffi::c_int;
            } else if strcmp(
                *argv.offset(INETD_SOCKET as isize),
                b"raw\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_socktype = __socket_type::SOCK_RAW.0 as ::core::ffi::c_int;
            } else {
                syslog(
                    LOG_WARNING,
                    b"%s:%lu: bad socket type\0".as_ptr() as *const ::core::ffi::c_char,
                    file,
                    *line as ::core::ffi::c_ulong,
                );
                (*sep).se_socktype = -1 as ::core::ffi::c_int;
            }
            (*sep).se_proto = newstr(*argv.offset(INETD_PROTOCOL as isize));
            (*sep).se_family = AF_INET as sa_family_t;
            (*sep).se_v4mapped = 1 as ::core::ffi::c_char;
            if strncmp(
                (*sep).se_proto,
                b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0 as ::core::ffi::c_int
                || strncmp(
                    (*sep).se_proto,
                    b"udp\0".as_ptr() as *const ::core::ffi::c_char,
                    3 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                if *(*sep).se_proto.offset(3isize) as ::core::ffi::c_int
                    == '6' as ::core::ffi::c_int
                {
                    (*sep).se_family = AF_INET6 as sa_family_t;
                    (*sep).se_v4mapped = 0 as ::core::ffi::c_char;
                    if strcmp(
                        (*sep).se_proto.offset(3isize),
                        b"6only\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        (*sep).se_v4mapped = 0 as ::core::ffi::c_char;
                    }
                } else if *(*sep).se_proto.offset(3isize) as ::core::ffi::c_int
                    == '4' as ::core::ffi::c_int
                {
                    (*sep).se_family = AF_INET as sa_family_t;
                }
            }
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            p = strchr(*argv.offset(3isize), '.' as ::core::ffi::c_int);
            if !p.is_null() {
                let c2rust_fresh3 = p;
                p = p.offset(1);
                *c2rust_fresh3 = 0 as ::core::ffi::c_char;
            }
            if strcmp(
                *argv.offset(INETD_WAIT as isize),
                b"wait\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_wait = 1 as ::core::ffi::c_int as pid_t;
            } else if strcmp(
                *argv.offset(INETD_WAIT as isize),
                b"nowait\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_wait = 0 as ::core::ffi::c_int as pid_t;
            } else {
                syslog(
                    LOG_WARNING,
                    b"%s:%lu: bad wait type\0".as_ptr() as *const ::core::ffi::c_char,
                    file,
                    *line as ::core::ffi::c_ulong,
                );
            }
            if !p.is_null() {
                (*sep).se_max =
                    strtoul(p, &raw mut q, 10 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                if *q != 0 {
                    syslog(
                        LOG_WARNING,
                        b"%s:%lu: invalid number (%s)\0".as_ptr() as *const ::core::ffi::c_char,
                        file,
                        *line as ::core::ffi::c_ulong,
                        p,
                    );
                }
            }
            if (*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE {
                (*sep).se_wait = 0 as ::core::ffi::c_int as pid_t;
                if strncmp(
                    (*sep).se_proto,
                    b"tcp\0".as_ptr() as *const ::core::ffi::c_char,
                    3 as size_t,
                ) != 0
                {
                    syslog(
                        LOG_ERR,
                        b"%s:%lu: bad protocol for tcpmux service %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        file,
                        *line as ::core::ffi::c_ulong,
                        (*sep).se_service,
                    );
                    continue;
                } else if (*sep).se_socktype != __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int {
                    syslog(
                        LOG_ERR,
                        b"%s:%lu: bad socket type for tcpmux service %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        file,
                        *line as ::core::ffi::c_ulong,
                        (*sep).se_service,
                    );
                    continue;
                }
            }
            let mut p_0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*sep).se_user = newstr(*argv.offset(INETD_USER as isize));
            p_0 = strchr((*sep).se_user, ':' as ::core::ffi::c_int);
            if p_0.is_null() {
                p_0 = strchr((*sep).se_user, '.' as ::core::ffi::c_int);
            }
            if !p_0.is_null() {
                *p_0 = '\0' as ::core::ffi::c_char;
                p_0 = p_0.offset(1);
                (*sep).se_group = newstr(p_0);
            } else {
                (*sep).se_group = newstr(::core::ptr::null::<::core::ffi::c_char>());
            }
            (*sep).se_server = newstr(*argv.offset(INETD_SERVER_PATH as isize));
            if strcmp(
                (*sep).se_server,
                b"internal\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                (*sep).se_bi = bi_lookup(sep) as *mut biltin;
                if (*sep).se_bi.is_null() {
                    syslog(
                        LOG_ERR,
                        b"%s:%lu: internal service %s unknown\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        file,
                        *line as ::core::ffi::c_ulong,
                        (*sep).se_service,
                    );
                    continue;
                } else {
                    (*sep).se_wait = (*(*sep).se_bi).bi_wait as pid_t;
                }
            } else {
                (*sep).se_bi = ::core::ptr::null_mut::<biltin>();
            }
            (*sep).se_argc = (argc - INETD_FIELDS_MIN + 1 as ::core::ffi::c_int) as size_t;
            (*sep).se_argv = calloc(
                (*sep).se_argc.wrapping_add(1 as size_t),
                ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
            ) as *mut *mut ::core::ffi::c_char;
            if (*sep).se_argv.is_null() {
                syslog(
                    LOG_ERR,
                    b"%s:%lu: Out of memory.\0".as_ptr() as *const ::core::ffi::c_char,
                    file,
                    *line as ::core::ffi::c_ulong,
                );
                exit(-1 as ::core::ffi::c_int);
            }
            i = 0 as size_t;
            while i < (*sep).se_argc {
                *(*sep).se_argv.offset(i as isize) =
                    *argv.offset((INETD_SERVER_ARGS as size_t).wrapping_add(i) as isize);
                *argv.offset((INETD_SERVER_ARGS as size_t).wrapping_add(i) as isize) =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                i = i.wrapping_add(1);
            }
            if (*sep).se_argc == 1 as size_t {
                let mut argv0: *const ::core::ffi::c_char =
                    strrchr((*sep).se_server, '/' as ::core::ffi::c_int);
                if !argv0.is_null() {
                    argv0 = argv0.offset(1);
                } else {
                    argv0 = (*sep).se_server;
                }
                *(*sep).se_argv.offset(0isize) = newstr(argv0);
            }
            *(*sep).se_argv.offset(i as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
            break;
        }
    }
    argcv_free(argc, argv);
    return next_node_sep(sep);
}
pub const MUX_LEN: usize = ::core::mem::size_of::<[::core::ffi::c_char; 8]>().wrapping_sub(1usize);
#[export_name = "rboxc_inetd_nextconfig"]
pub unsafe extern "C" fn nextconfig(mut file: *const ::core::ffi::c_char) {
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    let mut sepp: *mut *mut servtab = ::core::ptr::null_mut::<*mut servtab>();
    let mut pwd: *mut passwd = ::core::ptr::null_mut::<passwd>();
    let mut grp: *mut group = ::core::ptr::null_mut::<group>();
    let mut fconfig: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut sigstatus: sigset_t = __sigset_t { __val: [0; 16] };
    let mut line: size_t = 0 as size_t;
    fconfig = setconfig(file);
    if fconfig.is_null() {
        syslog(
            LOG_ERR,
            b"%s: %m\0".as_ptr() as *const ::core::ffi::c_char,
            file,
        );
        return;
    }
    loop {
        sep = getconfigent(fconfig, file, &raw mut line);
        if sep.is_null() {
            break;
        }
        pwd = getpwnam((*sep).se_user);
        if pwd.is_null() {
            syslog(
                LOG_ERR,
                b"%s/%s: No such user '%s', service ignored\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*sep).se_service,
                (*sep).se_proto,
                (*sep).se_user,
            );
        } else {
            if !(*sep).se_group.is_null() && *(*sep).se_group as ::core::ffi::c_int != 0 {
                grp = getgrnam((*sep).se_group);
                if grp.is_null() {
                    syslog(
                        LOG_ERR,
                        b"%s/%s: No such group '%s', service ignored\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*sep).se_service,
                        (*sep).se_proto,
                        (*sep).se_group,
                    );
                    continue;
                }
            }
            if (*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE {
                (*sep).se_fd = -1 as ::core::ffi::c_int;
                (*sep).se_checked = 1 as ::core::ffi::c_short;
                enter(sep);
            } else {
                expand_enter(sep);
            }
            if !serv_node.is_null() {
                free((*sep).se_node as *mut ::core::ffi::c_void);
            } else {
                freeconfig(sep);
            }
        }
    }
    endconfig(fconfig);
    signal_block(&raw mut sigstatus);
    sepp = &raw mut servtab as *mut *mut servtab;
    loop {
        sep = *sepp;
        if sep.is_null() {
            break;
        }
        if (*sep).se_checked != 0 {
            sepp = &raw mut (*sep).se_next;
        } else {
            *sepp = (*sep).se_next;
            if (*sep).se_fd >= 0 as ::core::ffi::c_int {
                close_sep(sep);
            }
            if debug {
                print_service(b"FREE\0".as_ptr() as *const ::core::ffi::c_char, sep);
            }
            freeconfig(sep);
            free(sep as *mut ::core::ffi::c_void);
        }
    }
    signal_unblock(&raw mut sigstatus);
}
#[export_name = "rboxc_inetd_fix_tcpmux"]
pub unsafe extern "C" fn fix_tcpmux() {
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    let mut need_tcpmux: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut has_tcpmux: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    sep = servtab as *mut servtab;
    while !sep.is_null() {
        if (*sep).se_checked != 0 {
            if (*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE {
                if has_tcpmux != 0 {
                    return;
                }
                need_tcpmux = 1 as ::core::ffi::c_int;
            }
            if strcmp(
                (*sep).se_service,
                b"tcpmux\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                if need_tcpmux != 0 {
                    return;
                }
                has_tcpmux = 1 as ::core::ffi::c_int;
            }
        }
        sep = (*sep).se_next;
    }
    if need_tcpmux != 0 && has_tcpmux == 0 {
        let mut serv: servtab = servtab {
            se_file: ::core::ptr::null::<::core::ffi::c_char>(),
            se_line: 0,
            se_node: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            se_service: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            se_socktype: 0,
            se_proto: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            se_wait: 0,
            se_max: 0,
            se_checked: 0,
            se_user: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            se_group: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            se_bi: ::core::ptr::null_mut::<biltin>(),
            se_server: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            se_argv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            se_argc: 0,
            se_fd: 0,
            se_type: 0,
            se_family: 0,
            se_v4mapped: 0,
            se_ctrladdr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            se_addrlen: 0,
            se_refcnt: 0,
            se_count: 0,
            se_time: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            se_next: ::core::ptr::null_mut::<servtab>(),
        };
        memset(
            &raw mut serv as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<servtab>(),
        );
        serv.se_file = b"fix_tcpmux\0".as_ptr() as *const ::core::ffi::c_char;
        serv.se_service = newstr(b"tcpmux\0".as_ptr() as *const ::core::ffi::c_char);
        serv.se_socktype = __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int;
        serv.se_checked = 1 as ::core::ffi::c_short;
        serv.se_user = newstr(b"root\0".as_ptr() as *const ::core::ffi::c_char);
        serv.se_group = newstr(::core::ptr::null::<::core::ffi::c_char>());
        serv.se_bi = bi_lookup(&raw mut serv) as *mut biltin;
        if serv.se_bi.is_null() {
            freeconfig(&raw mut serv);
            if debug {
                fprintf(
                    stderr,
                    b"INTERNAL ERROR: could not find tcpmux built-in\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
            syslog(
                LOG_ERR,
                b"INTERNAL ERROR: could not find tcpmux built-in\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            return;
        }
        serv.se_wait = (*serv.se_bi).bi_wait as pid_t;
        serv.se_server = newstr(b"internal\0".as_ptr() as *const ::core::ffi::c_char);
        serv.se_fd = -1 as ::core::ffi::c_int;
        serv.se_type = NORM_TYPE;
        serv.se_proto = newstr(b"tcp6\0".as_ptr() as *const ::core::ffi::c_char);
        serv.se_family = AF_INET6 as sa_family_t;
        serv.se_v4mapped = 1 as ::core::ffi::c_char;
        if debug {
            fprintf(
                stderr,
                b"inserting default tcpmux entry\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        syslog(
            LOG_INFO,
            b"inserting default tcpmux entry\0".as_ptr() as *const ::core::ffi::c_char,
        );
        expand_enter(&raw mut serv);
    }
}
#[export_name = "rboxc_inetd_config"]
pub unsafe extern "C" fn config(mut signo: ::core::ffi::c_int) {
    let mut i: ::core::ffi::c_int = 0;
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
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    sep = servtab as *mut servtab;
    while !sep.is_null() {
        (*sep).se_checked = 0 as ::core::ffi::c_short;
        sep = (*sep).se_next;
    }
    i = 0 as ::core::ffi::c_int;
    while !(*config_files.offset(i as isize)).is_null() {
        let mut statbuf: stat = stat {
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
        if stat(*config_files.offset(i as isize), &raw mut statbuf) == 0 as ::core::ffi::c_int {
            if statbuf.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
                let mut dirp: *mut DIR = opendir(*config_files.offset(i as isize));
                if !dirp.is_null() {
                    let mut dp: *mut dirent = ::core::ptr::null_mut::<dirent>();
                    loop {
                        dp = readdir(dirp);
                        if dp.is_null() {
                            break;
                        }
                        let mut path: *mut ::core::ffi::c_char = calloc(
                            strlen(*config_files.offset(i as isize))
                                .wrapping_add(strlen(
                                    &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                                ))
                                .wrapping_add(2 as size_t),
                            1 as size_t,
                        )
                            as *mut ::core::ffi::c_char;
                        if !path.is_null() {
                            sprintf(
                                path,
                                b"%s/%s\0".as_ptr() as *const ::core::ffi::c_char,
                                *config_files.offset(i as isize),
                                &raw mut (*dp).d_name as *mut ::core::ffi::c_char,
                            );
                            if stat(path, &raw mut stats) == 0 as ::core::ffi::c_int
                                && stats.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                            {
                                nextconfig(path);
                            }
                            free(path as *mut ::core::ffi::c_void);
                        }
                    }
                    closedir(dirp);
                }
            } else if statbuf.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
                nextconfig(*config_files.offset(i as isize));
            }
        } else if signo == 0 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"inetd: %s, %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                *config_files.offset(i as isize),
                strerror(*__errno_location()),
            );
        } else {
            syslog(
                LOG_ERR,
                b"%s: %m\0".as_ptr() as *const ::core::ffi::c_char,
                *config_files.offset(i as isize),
            );
        }
        i += 1;
    }
    free(linebuf as *mut ::core::ffi::c_void);
    linebuf = ::core::ptr::null_mut::<::core::ffi::c_char>();
    linebufsize = 0 as size_t;
    fix_tcpmux();
}
#[export_name = "rboxc_inetd_set_proc_title"]
pub unsafe extern "C" fn set_proc_title(
    mut a: *mut ::core::ffi::c_char,
    mut s: ::core::ffi::c_int,
) {
    let mut size: socklen_t = 0;
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut saddr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut buf: [::core::ffi::c_char; 80] = [0; 80];
    cp = *Argv.offset(0isize);
    size = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    if getpeername(
        s,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut saddr as *mut sockaddr,
        },
        &raw mut size,
    ) == 0 as ::core::ffi::c_int
    {
        let mut err: ::core::ffi::c_int = 0;
        let mut buf2: [::core::ffi::c_char; 80] = [0; 80];
        err = getnameinfo(
            &raw mut saddr as *mut sockaddr,
            ::core::mem::size_of::<sockaddr_storage>() as socklen_t,
            &raw mut buf2 as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>() as socklen_t,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            0 as socklen_t,
            NI_NUMERICHOST,
        );
        if err == 0 {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
                b"-%s [%s]\0".as_ptr() as *const ::core::ffi::c_char,
                a,
                &raw mut buf2 as *mut ::core::ffi::c_char,
            );
        } else {
            snprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
                b"-%s\0".as_ptr() as *const ::core::ffi::c_char,
                a,
            );
        }
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 80]>(),
            b"-%s\0".as_ptr() as *const ::core::ffi::c_char,
            a,
        );
    }
    strncpy(
        cp,
        &raw mut buf as *mut ::core::ffi::c_char,
        LastArg.offset_from(cp) as size_t,
    );
    cp = cp.offset(strlen(cp) as isize);
    while cp < LastArg {
        let c2rust_fresh1 = cp;
        cp = cp.offset(1);
        *c2rust_fresh1 = ' ' as ::core::ffi::c_char;
    }
}
#[export_name = "rboxc_inetd_echo_stream"]
pub unsafe extern "C" fn echo_stream(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut i: ::core::ffi::c_int = 0;
    set_proc_title((*sep).se_service, s);
    loop {
        i = read(
            s,
            &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        ) as ::core::ffi::c_int;
        if !(i > 0 as ::core::ffi::c_int
            && write(
                s,
                &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                i as size_t,
            ) > 0 as ssize_t)
        {
            break;
        }
    }
    exit(EXIT_SUCCESS);
}
#[export_name = "rboxc_inetd_echo_dg"]
pub unsafe extern "C" fn echo_dg(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut i: ::core::ffi::c_int = 0;
    let mut size: socklen_t = 0;
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    size = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    i = recvfrom(
        s,
        &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
        0 as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        &raw mut size,
    ) as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        return;
    }
    sendto(
        s,
        &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        i as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_storage>() as socklen_t,
    );
}
#[export_name = "rboxc_inetd_discard_stream"]
pub unsafe extern "C" fn discard_stream(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut ret: ::core::ffi::c_int = 0;
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    set_proc_title((*sep).se_service, s);
    loop {
        loop {
            ret = read(
                s,
                &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            ) as ::core::ffi::c_int;
            if ret <= 0 as ::core::ffi::c_int {
                break;
            }
        }
        if ret == 0 as ::core::ffi::c_int || *__errno_location() != EINTR {
            break;
        }
    }
    exit(EXIT_SUCCESS);
}
#[export_name = "rboxc_inetd_discard_dg"]
pub unsafe extern "C" fn discard_dg(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut buffer: [::core::ffi::c_char; 8192] = [0; 8192];
    read(
        s,
        &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
    );
}
pub const LINESIZ: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
#[export_name = "rboxc_inetd_ring"]
pub static mut ring: [::core::ffi::c_char; 128] = [0; 128];
#[export_name = "rboxc_inetd_endring"]
pub static mut endring: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_inetd_initring"]
pub unsafe extern "C" fn initring() {
    let mut i: ::core::ffi::c_int = 0;
    endring = &raw mut ring as *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i <= 128 as ::core::ffi::c_int {
        if *(*__ctype_b_loc()).offset(i as isize) as ::core::ffi::c_int
            & C2Rust_Unnamed_26::_ISprint.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            let c2rust_fresh0 = endring;
            endring = endring.offset(1);
            *c2rust_fresh0 = i as ::core::ffi::c_char;
        }
        i += 1;
    }
}
#[export_name = "rboxc_inetd_chargen_stream"]
pub unsafe extern "C" fn chargen_stream(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut len: ::core::ffi::c_int = 0;
    let mut rs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut text: [::core::ffi::c_char; 74] = [0; 74];
    set_proc_title((*sep).se_service, s);
    if endring.is_null() {
        initring();
        rs = &raw mut ring as *mut ::core::ffi::c_char;
    }
    text[LINESIZ as usize] = '\r' as ::core::ffi::c_char;
    text[(LINESIZ + 1 as ::core::ffi::c_int) as usize] = '\n' as ::core::ffi::c_char;
    rs = &raw mut ring as *mut ::core::ffi::c_char;
    loop {
        len = endring.offset_from(rs) as ::core::ffi::c_int;
        if len >= LINESIZ {
            memmove(
                &raw mut text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                rs as *const ::core::ffi::c_void,
                LINESIZ as size_t,
            );
        } else {
            memmove(
                &raw mut text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                rs as *const ::core::ffi::c_void,
                len as size_t,
            );
            memmove(
                (&raw mut text as *mut ::core::ffi::c_char).offset(len as isize)
                    as *mut ::core::ffi::c_void,
                &raw mut ring as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                (LINESIZ - len) as size_t,
            );
        }
        rs = rs.offset(1);
        if rs == endring {
            rs = &raw mut ring as *mut ::core::ffi::c_char;
        }
        if write(
            s,
            &raw mut text as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 74]>(),
        ) as usize
            != ::core::mem::size_of::<[::core::ffi::c_char; 74]>()
        {
            break;
        }
    }
    exit(EXIT_SUCCESS);
}
#[export_name = "rboxc_inetd_chargen_dg"]
pub unsafe extern "C" fn chargen_dg(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    static mut rs: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut len: ::core::ffi::c_int = 0;
    let mut size: socklen_t = 0;
    let mut text: [::core::ffi::c_char; 74] = [0; 74];
    if endring.is_null() {
        initring();
        rs = &raw mut ring as *mut ::core::ffi::c_char;
    }
    size = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    if recvfrom(
        s,
        &raw mut text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 74]>(),
        0 as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        &raw mut size,
    ) < 0 as ssize_t
    {
        return;
    }
    len = endring.offset_from(rs) as ::core::ffi::c_int;
    if len >= LINESIZ {
        memmove(
            &raw mut text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            rs as *const ::core::ffi::c_void,
            LINESIZ as size_t,
        );
    } else {
        memmove(
            &raw mut text as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            rs as *const ::core::ffi::c_void,
            len as size_t,
        );
        memmove(
            (&raw mut text as *mut ::core::ffi::c_char).offset(len as isize)
                as *mut ::core::ffi::c_void,
            &raw mut ring as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            (LINESIZ - len) as size_t,
        );
    }
    rs = rs.offset(1);
    if rs == endring {
        rs = &raw mut ring as *mut ::core::ffi::c_char;
    }
    text[LINESIZ as usize] = '\r' as ::core::ffi::c_char;
    text[(LINESIZ + 1 as ::core::ffi::c_int) as usize] = '\n' as ::core::ffi::c_char;
    sendto(
        s,
        &raw mut text as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 74]>(),
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_storage>() as socklen_t,
    );
}
#[export_name = "rboxc_inetd_machtime"]
pub unsafe extern "C" fn machtime() -> ::core::ffi::c_long {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    if gettimeofday(&raw mut tv, NULL) < 0 as ::core::ffi::c_int {
        if debug {
            fprintf(
                stderr,
                b"Unable to get time of day\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        return 0 as ::core::ffi::c_long;
    }
    return __bswap_32(
        (tv.tv_sec as ::core::ffi::c_ulong).wrapping_add(
            (25567 as ::core::ffi::c_int as ::core::ffi::c_ulong)
                .wrapping_mul(24 as ::core::ffi::c_ulong)
                .wrapping_mul(60 as ::core::ffi::c_ulong)
                .wrapping_mul(60 as ::core::ffi::c_ulong),
        ) as ::core::ffi::c_long as __uint32_t,
    ) as ::core::ffi::c_long;
}
#[export_name = "rboxc_inetd_machtime_stream"]
pub unsafe extern "C" fn machtime_stream(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut result: ::core::ffi::c_long = 0;
    result = machtime();
    write(
        s,
        &raw mut result as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_long>(),
    );
}
#[export_name = "rboxc_inetd_machtime_dg"]
pub unsafe extern "C" fn machtime_dg(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut result: ::core::ffi::c_long = 0;
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut size: socklen_t = 0;
    size = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    if recvfrom(
        s,
        &raw mut result as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_long>(),
        0 as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        &raw mut size,
    ) < 0 as ssize_t
    {
        return;
    }
    result = machtime();
    sendto(
        s,
        &raw mut result as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_long>(),
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_storage>() as socklen_t,
    );
}
#[export_name = "rboxc_inetd_daytime_stream"]
pub unsafe extern "C" fn daytime_stream(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
    let mut lclock: time_t = 0;
    lclock = time(::core::ptr::null_mut::<time_t>());
    sprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"%.24s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        ctime(&raw mut lclock),
    );
    write(
        s,
        &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut buffer as *mut ::core::ffi::c_char),
    );
}
#[export_name = "rboxc_inetd_daytime_dg"]
pub unsafe extern "C" fn daytime_dg(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
    let mut lclock: time_t = 0;
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut size: socklen_t = 0;
    lclock = time(::core::ptr::null_mut::<time_t>());
    size = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    if recvfrom(
        s,
        &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
        0 as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        &raw mut size,
    ) < 0 as ssize_t
    {
        return;
    }
    sprintf(
        &raw mut buffer as *mut ::core::ffi::c_char,
        b"%.24s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
        ctime(&raw mut lclock),
    );
    sendto(
        s,
        &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut buffer as *mut ::core::ffi::c_char),
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut sa as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_storage>() as socklen_t,
    );
}
unsafe extern "C" fn fd_getline(
    mut fd: ::core::ffi::c_int,
    mut buf: *mut ::core::ffi::c_char,
    mut len: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = 0;
    loop {
        n = read(fd, buf as *mut ::core::ffi::c_void, (len - count) as size_t)
            as ::core::ffi::c_int;
        if n == 0 as ::core::ffi::c_int {
            return count;
        }
        if n < 0 as ::core::ffi::c_int {
            return -1 as ::core::ffi::c_int;
        }
        loop {
            n -= 1;
            if n < 0 as ::core::ffi::c_int {
                break;
            }
            if *buf as ::core::ffi::c_int == '\r' as ::core::ffi::c_int
                || *buf as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
                || *buf as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
            {
                return count;
            }
            count += 1;
            buf = buf.offset(1);
        }
        if count >= len {
            break;
        }
    }
    return count;
}
pub const MAX_SERV_LEN: ::core::ffi::c_int = 256 as ::core::ffi::c_int + 2 as ::core::ffi::c_int;
#[export_name = "rboxc_inetd_tcpmux"]
pub unsafe extern "C" fn tcpmux(mut s: ::core::ffi::c_int, mut sep: *mut servtab) {
    let mut service: [::core::ffi::c_char; 259] = [0; 259];
    let mut len: ::core::ffi::c_int = 0;
    len = fd_getline(
        s,
        &raw mut service as *mut ::core::ffi::c_char,
        MAX_SERV_LEN,
    );
    if len < 0 as ::core::ffi::c_int {
        write(
            s,
            b"-Error reading service name\r\n\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 30]>().wrapping_sub(1 as size_t),
        );
        _exit(EXIT_FAILURE);
    }
    service[len as usize] = '\0' as ::core::ffi::c_char;
    if debug {
        fprintf(
            stderr,
            b"tcpmux: someone wants %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut service as *mut ::core::ffi::c_char,
        );
    }
    if strcasecmp(
        &raw mut service as *mut ::core::ffi::c_char,
        b"help\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0
    {
        sep = servtab as *mut servtab;
        while !sep.is_null() {
            if (*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE {
                write(
                    s,
                    (*sep).se_service as *const ::core::ffi::c_void,
                    strlen((*sep).se_service),
                );
                write(
                    s,
                    b"\r\n\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 3]>().wrapping_sub(1 as size_t),
                );
            }
            sep = (*sep).se_next;
        }
        _exit(EXIT_FAILURE);
    }
    sep = servtab as *mut servtab;
    while !sep.is_null() {
        if ((*sep).se_type == MUX_TYPE || (*sep).se_type == MUXPLUS_TYPE)
            && strcasecmp(
                &raw mut service as *mut ::core::ffi::c_char,
                (*sep).se_service,
            ) == 0
        {
            if (*sep).se_type == MUXPLUS_TYPE {
                write(
                    s,
                    b"+Go\r\n\0".as_ptr() as *const ::core::ffi::c_char
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[::core::ffi::c_char; 6]>().wrapping_sub(1 as size_t),
                );
            }
            run_service(s, sep);
            return;
        }
        sep = (*sep).se_next;
    }
    write(
        s,
        b"-Service not available\r\n\0".as_ptr() as *const ::core::ffi::c_char
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 25]>().wrapping_sub(1 as size_t),
    );
    exit(EXIT_FAILURE);
}
#[export_name = "rboxc_inetd_prepenv"]
pub unsafe extern "C" fn prepenv(
    mut ctrl: ::core::ffi::c_int,
    mut sa_client: *mut sockaddr,
    mut sa_len: socklen_t,
) {
    let mut str: [::core::ffi::c_char; 16] = [0; 16];
    let mut ip: [::core::ffi::c_char; 184] = [0; 184];
    let mut ret: ::core::ffi::c_int = 0;
    let mut sa_server: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut len: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    setenv(
        b"PROTO\0".as_ptr() as *const ::core::ffi::c_char,
        b"TCP\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    unsetenv(b"TCPLOCALIP\0".as_ptr() as *const ::core::ffi::c_char);
    unsetenv(b"TCPLOCALHOST\0".as_ptr() as *const ::core::ffi::c_char);
    unsetenv(b"TCPLOCALPORT\0".as_ptr() as *const ::core::ffi::c_char);
    unsetenv(b"TCPREMOTEIP\0".as_ptr() as *const ::core::ffi::c_char);
    unsetenv(b"TCPREMOTEPORT\0".as_ptr() as *const ::core::ffi::c_char);
    unsetenv(b"TCPREMOTEHOST\0".as_ptr() as *const ::core::ffi::c_char);
    if getsockname(
        ctrl,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut sa_server as *mut sockaddr,
        },
        &raw mut len,
    ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_WARNING,
            b"getsockname(): %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        ret = getnameinfo(
            &raw mut sa_server as *mut sockaddr,
            len,
            &raw mut ip as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 184]>() as socklen_t,
            &raw mut str as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as socklen_t,
            NI_NUMERICHOST | NI_NUMERICSERV,
        );
        if ret == 0 as ::core::ffi::c_int {
            if setenv(
                b"TCPLOCALIP\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut ip as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                syslog(
                    LOG_WARNING,
                    b"setenv (TCPLOCALIP): %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else if debug {
                fprintf(
                    stderr,
                    b"Assigned TCPLOCALIP = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ip as *mut ::core::ffi::c_char,
                );
            }
            if setenv(
                b"TCPLOCALPORT\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut str as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                syslog(
                    LOG_WARNING,
                    b"setenv (TCPLOCALPORT): %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else {
            syslog(
                LOG_WARNING,
                b"getnameinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                gai_strerror(ret),
            );
        }
        if resolve_option {
            ret = getnameinfo(
                &raw mut sa_server as *mut sockaddr,
                len,
                &raw mut ip as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 184]>() as socklen_t,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as socklen_t,
                0 as ::core::ffi::c_int,
            );
            if ret != 0 as ::core::ffi::c_int {
                syslog(
                    LOG_WARNING,
                    b"getnameinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    gai_strerror(ret),
                );
            } else if setenv(
                b"TCPLOCALHOST\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut ip as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                syslog(
                    LOG_WARNING,
                    b"setenv(TCPLOCALHOST): %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
    ret = getnameinfo(
        sa_client,
        sa_len,
        &raw mut ip as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 184]>() as socklen_t,
        &raw mut str as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as socklen_t,
        NI_NUMERICHOST | NI_NUMERICSERV,
    );
    if ret == 0 as ::core::ffi::c_int {
        if setenv(
            b"TCPREMOTEIP\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut ip as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            syslog(
                LOG_WARNING,
                b"setenv(TCPREMOTEIP): %m\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if debug {
            fprintf(
                stderr,
                b"Assigned TCPREMOTEIP = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut ip as *mut ::core::ffi::c_char,
            );
        }
        if setenv(
            b"TCPREMOTEPORT\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut str as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            syslog(
                LOG_WARNING,
                b"setenv(TCPREMOTEPORT): %m\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if resolve_option {
            ret = getnameinfo(
                sa_client,
                sa_len,
                &raw mut ip as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 184]>() as socklen_t,
                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                0 as socklen_t,
                0 as ::core::ffi::c_int,
            );
            if ret != 0 as ::core::ffi::c_int {
                syslog(
                    LOG_WARNING,
                    b"getnameinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    gai_strerror(ret),
                );
            } else if setenv(
                b"TCPREMOTEHOST\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut ip as *mut ::core::ffi::c_char,
                1 as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                syslog(
                    LOG_WARNING,
                    b"setenv(TCPREMOTEHOST): %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else if debug {
                fprintf(
                    stderr,
                    b"Assigned TCPREMOTEHOST = %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ip as *mut ::core::ffi::c_char,
                );
            }
        }
    } else {
        syslog(
            LOG_WARNING,
            b"getnameinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
            gai_strerror(ret),
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_inetd(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut envp: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut sep: *mut servtab = ::core::ptr::null_mut::<servtab>();
    let mut dofork: ::core::ffi::c_int = 0;
    let mut pid: pid_t = 0;
    set_program_name(*argv.offset(0isize));
    Argv = argv as *mut *mut ::core::ffi::c_char;
    if envp.is_null() || (*envp).is_null() {
        envp = argv;
    }
    while !(*envp).is_null() {
        envp = envp.offset(1);
    }
    LastArg = (*envp.offset(-1isize)).offset(strlen(*envp.offset(-1isize)) as isize);
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"inetd\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut program_authors as *mut *const ::core::ffi::c_char,
    );
    rpl_argp_parse(
        &raw mut argp,
        argc,
        argv,
        0 as ::core::ffi::c_uint,
        &raw mut index,
        NULL,
    );
    if resolve_option {
        env_option = r#true != 0;
    }
    if index < argc {
        let mut i: ::core::ffi::c_int = 0;
        config_files = calloc(
            (argc - index + 1 as ::core::ffi::c_int) as size_t,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
        ) as *mut *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        while index < argc {
            *config_files.offset(i as isize) = strdup(*argv.offset(index as isize));
            index += 1;
            i += 1;
        }
    } else {
        config_files = calloc(
            3 as size_t,
            ::core::mem::size_of::<*mut ::core::ffi::c_char>(),
        ) as *mut *mut ::core::ffi::c_char;
        *config_files.offset(0isize) = newstr(PATH_INETDCONF.as_ptr());
        *config_files.offset(1isize) = newstr(PATH_INETDDIR.as_ptr());
    }
    if !foreground {
        if daemon(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
            syslog(
                LOG_DAEMON | LOG_ERR,
                b"%s: Unable to enter daemon mode, %m\0".as_ptr() as *const ::core::ffi::c_char,
                *argv.offset(0isize),
            );
            exit(EXIT_FAILURE);
        }
    }
    openlog(
        b"inetd\0".as_ptr() as *const ::core::ffi::c_char,
        LOG_PID | LOG_NOWAIT,
        LOG_DAEMON,
    );
    if pidfile_option {
        let mut fp: *mut FILE = fopen(pid_file, b"w\0".as_ptr() as *const ::core::ffi::c_char);
        if !fp.is_null() {
            if debug {
                fprintf(
                    stderr,
                    b"Using pid-file at \"%s\".\n\0".as_ptr() as *const ::core::ffi::c_char,
                    pid_file,
                );
            }
            fprintf(
                fp,
                b"%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                getpid(),
            );
            fclose(fp);
        } else {
            syslog(
                LOG_CRIT,
                b"can't open %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                pid_file,
                strerror(*__errno_location()),
            );
        }
    }
    signal_set_handler(
        SIGALRM,
        Some(retry as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    config(0 as ::core::ffi::c_int);
    signal_set_handler(
        SIGHUP,
        Some(config as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal_set_handler(
        SIGCHLD,
        Some(reapchild as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal_set_handler(
        SIGPIPE,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    let mut dummy: [::core::ffi::c_char; 100] = [0; 100];
    memset(
        &raw mut dummy as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        'x' as ::core::ffi::c_int,
        (DUMMYSIZE - 1 as ::core::ffi::c_int) as size_t,
    );
    dummy[(DUMMYSIZE - 1 as ::core::ffi::c_int) as usize] = '\0' as ::core::ffi::c_char;
    setenv(
        b"inetd_dummy\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut dummy as *mut ::core::ffi::c_char,
        1 as ::core::ffi::c_int,
    );
    loop {
        let mut n: ::core::ffi::c_int = 0;
        let mut ctrl: ::core::ffi::c_int = 0;
        let mut readable: fd_set = fd_set { fds_bits: [0; 16] };
        if nsock == 0 as ::core::ffi::c_int {
            let mut stat_0: sigset_t = __sigset_t { __val: [0; 16] };
            sigemptyset(&raw mut stat_0);
            signal_block(::core::ptr::null_mut::<sigset_t>());
            while nsock == 0 as ::core::ffi::c_int {
                sigsuspend(&raw mut stat_0);
            }
            signal_unblock(::core::ptr::null_mut::<sigset_t>());
        }
        readable = allsock;
        n = select(
            maxsock + 1 as ::core::ffi::c_int,
            &raw mut readable,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<timeval>(),
        );
        if n <= 0 as ::core::ffi::c_int {
            if n < 0 as ::core::ffi::c_int && *__errno_location() != EINTR {
                syslog(
                    LOG_WARNING,
                    b"select: %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            sleep(1 as ::core::ffi::c_uint);
        } else {
            sep = servtab as *mut servtab;
            while n != 0 && !sep.is_null() {
                's_263: {
                    if (*sep).se_fd != -1 as ::core::ffi::c_int
                        && readable.fds_bits[((*sep).se_fd / __NFDBITS) as usize]
                            & ((1 as ::core::ffi::c_ulong) << (*sep).se_fd % __NFDBITS) as __fd_mask
                            != 0 as __fd_mask
                    {
                        n -= 1;
                        if debug {
                            fprintf(
                                stderr,
                                b"someone wants %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                (*sep).se_service,
                            );
                        }
                        if (*sep).se_wait == 0
                            && (*sep).se_socktype
                                == __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int
                        {
                            let mut sa_client: sockaddr_storage = sockaddr_storage {
                                ss_family: 0,
                                __ss_padding: [0; 118],
                                __ss_align: 0,
                            };
                            let mut len: socklen_t =
                                ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
                            ctrl = accept(
                                (*sep).se_fd,
                                __SOCKADDR_ARG {
                                    __sockaddr__: &raw mut sa_client as *mut sockaddr,
                                },
                                &raw mut len,
                            );
                            if debug {
                                fprintf(
                                    stderr,
                                    b"accept, ctrl %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                                    ctrl,
                                );
                            }
                            if ctrl < 0 as ::core::ffi::c_int {
                                if *__errno_location() != EINTR {
                                    syslog(
                                        LOG_WARNING,
                                        b"accept (for %s): %m\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*sep).se_service,
                                    );
                                }
                                break 's_263;
                            } else if env_option {
                                prepenv(ctrl, &raw mut sa_client as *mut sockaddr, len);
                            }
                        } else {
                            ctrl = (*sep).se_fd;
                        }
                        signal_block(::core::ptr::null_mut::<sigset_t>());
                        pid = 0 as ::core::ffi::c_int as pid_t;
                        dofork = ((*sep).se_bi.is_null()
                            || (*(*sep).se_bi).bi_fork as ::core::ffi::c_int != 0)
                            as ::core::ffi::c_int;
                        if dofork != 0 {
                            let c2rust_fresh4 = (*sep).se_count;
                            (*sep).se_count = (*sep).se_count.wrapping_add(1);
                            if c2rust_fresh4 == 0 as ::core::ffi::c_uint {
                                gettimeofday(&raw mut (*sep).se_time, NULL);
                            } else if (*sep).se_max != 0 && (*sep).se_count > (*sep).se_max
                                || (*sep).se_count >= toomany
                            {
                                let mut now: timeval = timeval {
                                    tv_sec: 0,
                                    tv_usec: 0,
                                };
                                gettimeofday(&raw mut now, NULL);
                                if now.tv_sec - (*sep).se_time.tv_sec > CNT_INTVL as __time_t {
                                    (*sep).se_time = now;
                                    (*sep).se_count = 1 as ::core::ffi::c_uint;
                                } else {
                                    syslog(
                                        LOG_ERR,
                                        b"%s/%s server failing (looping), service terminated\0"
                                            .as_ptr()
                                            as *const ::core::ffi::c_char,
                                        (*sep).se_service,
                                        (*sep).se_proto,
                                    );
                                    close_sep(sep);
                                    if (*sep).se_wait == 0
                                        && (*sep).se_socktype
                                            == __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int
                                    {
                                        close(ctrl);
                                    }
                                    signal_unblock(::core::ptr::null_mut::<sigset_t>());
                                    if timingout == 0 {
                                        timingout = 1 as ::core::ffi::c_int;
                                        alarm(RETRYTIME as ::core::ffi::c_uint);
                                    }
                                    break 's_263;
                                }
                            }
                            pid = fork() as pid_t;
                        }
                        if pid < 0 as ::core::ffi::c_int {
                            syslog(
                                LOG_ERR,
                                b"fork: %m\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            if (*sep).se_wait == 0
                                && (*sep).se_socktype
                                    == __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int
                            {
                                close(ctrl);
                            }
                            signal_unblock(::core::ptr::null_mut::<sigset_t>());
                            sleep(1 as ::core::ffi::c_uint);
                        } else {
                            if pid != 0 && (*sep).se_wait != 0 {
                                (*sep).se_wait = pid;
                                if (*sep).se_fd >= 0 as ::core::ffi::c_int {
                                    allsock.fds_bits[((*sep).se_fd / __NFDBITS) as usize] &=
                                        !(((1 as ::core::ffi::c_ulong) << (*sep).se_fd % __NFDBITS)
                                            as __fd_mask);
                                    nsock -= 1;
                                }
                            }
                            signal_unblock(::core::ptr::null_mut::<sigset_t>());
                            if pid == 0 as ::core::ffi::c_int {
                                if debug as ::core::ffi::c_int != 0 && dofork != 0 {
                                    setsid();
                                }
                                if dofork != 0 {
                                    let mut sock: ::core::ffi::c_int = 0;
                                    if debug {
                                        fprintf(
                                            stderr,
                                            b"+ Closing from %d\n\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            maxsock,
                                        );
                                    }
                                    sock = maxsock;
                                    while sock > 2 as ::core::ffi::c_int {
                                        if sock != ctrl {
                                            close(sock);
                                        }
                                        sock -= 1;
                                    }
                                }
                                run_service(ctrl, sep);
                            }
                            if (*sep).se_wait == 0
                                && (*sep).se_socktype
                                    == __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int
                            {
                                close(ctrl);
                            }
                        }
                    }
                }
                sep = (*sep).se_next;
            }
        }
    }
}
pub const DUMMYSIZE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const PATH_INETDCONF: [::core::ffi::c_char; 50] = unsafe {
    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
        *b"/root/rboxc/build/oracle/inetutils/etc/inetd.conf\0",
    )
};
pub const PATH_INETDDIR: [::core::ffi::c_char; 47] = unsafe {
    ::core::mem::transmute::<[u8; 47], [::core::ffi::c_char; 47]>(
        *b"/root/rboxc/build/oracle/inetutils/etc/inetd.d\0",
    )
};
pub const PATH_INETDPID: [::core::ffi::c_char; 53] = unsafe {
    ::core::mem::transmute::<[u8; 53], [::core::ffi::c_char; 53]>(
        *b"/root/rboxc/build/oracle/inetutils/var/run/inetd.pid\0",
    )
};
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
