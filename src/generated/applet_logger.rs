// Generated from pinned GNU logger 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: c1543a9f8ce4605d3d1d4ab0196c9853a4768a330b18fed6dc8e95160158bfdd
/*
  Copyright (C) 2009-2026 Free Software Foundation, Inc.

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
use ::c2rust_bitfields;
extern "C" {
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
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn send(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
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
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn inet_aton(__cp: *const ::core::ffi::c_char, __inp: *mut in_addr) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn freopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn asprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    #[link_name = "rboxc_logger_rpl_getline"]
    fn rpl_getline(
        lineptr: *mut *mut ::core::ffi::c_char,
        linesize: *mut size_t,
        stream: *mut FILE,
    ) -> ssize_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut ::core::ffi::c_char;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    #[link_name = "rboxc_logger_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_logger_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_logger_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_logger_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_logger_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_logger_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_logger_umaxtostr"]
    fn umaxtostr(_: uintmax_t, _: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
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
    pub __in6_u: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
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
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _code {
    pub c_name: *mut ::core::ffi::c_char,
    pub c_val: ::core::ffi::c_int,
}
pub type CODE = _code;
#[derive(Copy, Clone)]
#[repr(C)]
pub union logger_sockaddr {
    pub sa: sockaddr,
    pub sinet: sockaddr_in,
    pub sinet6: sockaddr_in6,
    pub sunix: sockaddr_un,
}
pub const PATH_LOG: [::core::ffi::c_char; 9] = _PATH_LOG;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_LOCAL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_UNIX: ::core::ffi::c_int = PF_LOCAL;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_UNIX: ::core::ffi::c_int = PF_UNIX;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const INADDR_ANY: in_addr_t = 0 as ::core::ffi::c_int as in_addr_t;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const OPTION_ARG_OPTIONAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const _PATH_LOG: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"/dev/log\0") };
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
#[export_name = "rboxc_logger_prioritynames"]
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
pub const LOG_FACMASK: ::core::ffi::c_int = 0x3f8 as ::core::ffi::c_int;
#[export_name = "rboxc_logger_facilitynames"]
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
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_PERROR: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
static mut tag: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut logflags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut pri: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int
    & LOG_FACMASK
    | 5 as ::core::ffi::c_int & LOG_PRIMASK;
static mut host: *mut ::core::ffi::c_char = PATH_LOG.as_ptr() as *mut ::core::ffi::c_char;
static mut unixsock: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut source: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut pidstr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut host_family: ::core::ffi::c_int = AF_UNSPEC;
#[export_name = "rboxc_logger_decode"]
pub unsafe extern "C" fn decode(
    mut name: *mut ::core::ffi::c_char,
    mut codetab: *mut CODE,
    mut what: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cp: *mut CODE = ::core::ptr::null_mut::<CODE>();
    if *(*__ctype_b_loc()).offset(*name as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & C2Rust_Unnamed_0::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut n: ::core::ffi::c_ulong = strtoul(name, &raw mut p, 0 as ::core::ffi::c_int);
        if *p as ::core::ffi::c_int != 0 || n > LOG_FACMASK as ::core::ffi::c_ulong {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"invalid %s number: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    what,
                    name,
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
                        b"invalid %s number: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        what,
                        name,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        return n as ::core::ffi::c_int;
    }
    cp = codetab;
    while !(*cp).c_name.is_null() {
        if strcasecmp(name, (*cp).c_name) == 0 as ::core::ffi::c_int {
            return (*cp).c_val;
        }
        cp = cp.offset(1);
    }
    if 0 != 0 {
        error(
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            b"unknown %s name: %s\0".as_ptr() as *const ::core::ffi::c_char,
            what,
            name,
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
                b"unknown %s name: %s\0".as_ptr() as *const ::core::ffi::c_char,
                what,
                name,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
    return -1 as ::core::ffi::c_int;
}
#[export_name = "rboxc_logger_parse_level"]
pub unsafe extern "C" fn parse_level(mut str: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fac: ::core::ffi::c_int = 0;
    let mut prio: ::core::ffi::c_int = LOG_NOTICE;
    p = strchr(str, '.' as ::core::ffi::c_int);
    if !p.is_null() {
        let c2rust_fresh0 = p;
        p = p.offset(1);
        *c2rust_fresh0 = 0 as ::core::ffi::c_char;
    }
    fac = decode(
        str,
        &raw mut facilitynames as *mut CODE,
        b"facility\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !p.is_null() {
        prio = decode(
            p,
            &raw mut prioritynames as *mut CODE,
            b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return fac & LOG_FACMASK | prio & LOG_PRIMASK;
}
#[export_name = "rboxc_logger_fd"]
pub static mut fd: ::core::ffi::c_int = 0;
unsafe extern "C" fn open_socket() {
    let mut sockaddr: logger_sockaddr = logger_sockaddr {
        sa: sockaddr {
            sa_family: 0,
            sa_data: [0; 14],
        },
    };
    let mut socklen: socklen_t = 0;
    let mut family: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if !host.is_null() && !strchr(host, '/' as ::core::ffi::c_int).is_null() || !unixsock.is_null()
    {
        let mut len: size_t = 0;
        if !unixsock.is_null() {
            host = unixsock;
        }
        len = strlen(host);
        if len >= ::core::mem::size_of::<[::core::ffi::c_char; 108]>() {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"UNIX socket name too long\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"UNIX socket name too long\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        strcpy(
            &raw mut sockaddr.sunix.sun_path as *mut ::core::ffi::c_char,
            host,
        );
        sockaddr.sunix.sun_family = AF_UNIX as sa_family_t;
        family = PF_UNIX;
        socklen = ::core::mem::size_of::<sockaddr_un>() as socklen_t;
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
        let mut ai: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if *host as ::core::ffi::c_int == '[' as ::core::ffi::c_int {
            host = host.offset(1);
            p = strchr(host, ']' as ::core::ffi::c_int);
            if !p.is_null() {
                let c2rust_fresh1 = p;
                p = p.offset(1);
                *c2rust_fresh1 = '\0' as ::core::ffi::c_char;
                if *p as ::core::ffi::c_int == ':' as ::core::ffi::c_int {
                    p = p.offset(1);
                } else {
                    p = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
        } else {
            p = strrchr(host, ':' as ::core::ffi::c_int);
            if !p.is_null() {
                let c2rust_fresh2 = p;
                p = p.offset(1);
                *c2rust_fresh2 = 0 as ::core::ffi::c_char;
            }
        }
        if p.is_null() {
            p = b"syslog\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        memset(
            &raw mut hints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<addrinfo>(),
        );
        hints.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
        hints.ai_family = host_family;
        ret = getaddrinfo(host, p, &raw mut hints, &raw mut res);
        if ret < 0 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"%s:%s, %s\0".as_ptr() as *const ::core::ffi::c_char,
                    host,
                    p,
                    gai_strerror(ret),
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
                        b"%s:%s, %s\0".as_ptr() as *const ::core::ffi::c_char,
                        host,
                        p,
                        gai_strerror(ret),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        ai = res;
        's_212: while !ai.is_null() {
            fd = socket((*ai).ai_family, (*ai).ai_socktype, (*ai).ai_protocol);
            's_140: {
                if fd >= 0 as ::core::ffi::c_int {
                    if !source.is_null() {
                        let mut ret_0: ::core::ffi::c_int = 0;
                        let mut tips: addrinfo = addrinfo {
                            ai_flags: 0,
                            ai_family: 0,
                            ai_socktype: 0,
                            ai_protocol: 0,
                            ai_addrlen: 0,
                            ai_addr: ::core::ptr::null_mut::<sockaddr>(),
                            ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                            ai_next: ::core::ptr::null_mut::<addrinfo>(),
                        };
                        let mut a: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
                        memset(
                            &raw mut tips as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<addrinfo>(),
                        );
                        tips.ai_family = (*ai).ai_family;
                        ret_0 = getaddrinfo(
                            source,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            &raw mut tips,
                            &raw mut a,
                        );
                        if ret_0 != 0 {
                            close(fd);
                            break 's_140;
                        } else if bind(
                            fd,
                            __CONST_SOCKADDR_ARG {
                                __sockaddr__: (*a).ai_addr,
                            },
                            (*a).ai_addrlen,
                        ) != 0
                        {
                            freeaddrinfo(a);
                            close(fd);
                            break 's_140;
                        } else {
                            freeaddrinfo(a);
                        }
                    }
                    if connect(
                        fd,
                        __CONST_SOCKADDR_ARG {
                            __sockaddr__: (*ai).ai_addr,
                        },
                        (*ai).ai_addrlen,
                    ) == 0
                    {
                        break 's_212;
                    }
                    close(fd);
                }
            }
            ai = (*ai).ai_next;
        }
        if !res.is_null() {
            freeaddrinfo(res);
        }
        if ai.is_null() {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    99 as ::core::ffi::c_int,
                    b"%s:%s\0".as_ptr() as *const ::core::ffi::c_char,
                    host,
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
                        99 as ::core::ffi::c_int,
                        b"%s:%s\0".as_ptr() as *const ::core::ffi::c_char,
                        host,
                        p,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        return;
    }
    fd = socket(
        family,
        __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot create socket\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"cannot create socket\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    if family == PF_INET {
        let mut s: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        s.sin_family = AF_INET as sa_family_t;
        if !source.is_null() {
            if inet_aton(source, &raw mut s.sin_addr) != 1 as ::core::ffi::c_int {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"invalid source address\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"invalid source address\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        } else {
            s.sin_addr.s_addr = INADDR_ANY;
        }
        s.sin_port = 0 as in_port_t;
        if bind(
            fd,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: &raw mut s as *mut sockaddr,
            },
            ::core::mem::size_of::<sockaddr_in>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"cannot bind to source address\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"cannot bind to source address\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if connect(
        fd,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut sockaddr.sa,
        },
        socklen,
    ) != 0
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot connect\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"cannot connect\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
unsafe extern "C" fn send_to_syslog(mut msg: *const ::core::ffi::c_char) {
    let mut pbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut now: time_t = time(::core::ptr::null_mut::<time_t>());
    let mut len: size_t = 0;
    let mut rc: ssize_t = 0;
    if logflags & LOG_PID != 0 {
        rc = asprintf(
            &raw mut pbuf,
            b"<%d>%.15s %s[%s]: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pri,
            ctime(&raw mut now).offset(4 as ::core::ffi::c_int as isize),
            tag,
            pidstr,
            msg,
        ) as ssize_t;
    } else {
        rc = asprintf(
            &raw mut pbuf,
            b"<%d>%.15s %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
            pri,
            ctime(&raw mut now).offset(4 as ::core::ffi::c_int as isize),
            tag,
            msg,
        ) as ssize_t;
    }
    if rc == -1 as ssize_t {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot format message\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"cannot format message\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    len = strlen(pbuf);
    if logflags & LOG_PERROR != 0 {
        let mut iov: [iovec; 2] = [iovec {
            iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            iov_len: 0,
        }; 2];
        let mut ioptr: *mut iovec = ::core::ptr::null_mut::<iovec>();
        let mut msglen: size_t = strlen(msg);
        ioptr = &raw mut iov as *mut iovec;
        (*ioptr).iov_base = msg as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
        (*ioptr).iov_len = msglen;
        if msglen > 0 as size_t
            && *msg.offset(msglen.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                != '\n' as ::core::ffi::c_int
        {
            ioptr = ioptr.offset(1);
            (*ioptr).iov_base = b"\n\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char
                as *mut ::core::ffi::c_void;
            (*ioptr).iov_len = 1 as size_t;
        }
        writev(
            fileno(stderr),
            &raw mut iov as *mut iovec,
            (ioptr.offset_from(&raw mut iov as *mut iovec) + 1isize) as ::core::ffi::c_int,
        );
    }
    rc = send(
        fd,
        pbuf as *const ::core::ffi::c_void,
        len,
        0 as ::core::ffi::c_int,
    );
    free(pbuf as *mut ::core::ffi::c_void);
    if rc == -1 as ssize_t {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                b"send failed\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"send failed\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else if rc != len as ssize_t {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                b"sent less bytes than expected (%lu vs. %lu)\0".as_ptr()
                    as *const ::core::ffi::c_char,
                rc as ::core::ffi::c_ulong,
                len as ::core::ffi::c_ulong,
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
                    b"sent less bytes than expected (%lu vs. %lu)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    rc as ::core::ffi::c_ulong,
                    len as ::core::ffi::c_ulong,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
#[export_name = "rboxc_logger_args_doc"]
pub static mut args_doc: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[MESSAGE]\0") };
#[export_name = "rboxc_logger_doc"]
pub static mut doc: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"Send messages to syslog\0")
};
static mut argp_options: [argp_option; 11] = [
    argp_option {
        name: b"ipv4\0".as_ptr() as *const ::core::ffi::c_char,
        key: '4' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"use IPv4 for logging to host\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"ipv6\0".as_ptr() as *const ::core::ffi::c_char,
        key: '6' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"use IPv6 with a host target\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"host\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'h' as ::core::ffi::c_int,
        arg: b"HOST\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"log to HOST instead of to the default /dev/log\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"unix\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'u' as ::core::ffi::c_int,
        arg: b"SOCK\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"log to UNIX socket SOCK instead of /dev/log\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"source\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'S' as ::core::ffi::c_int,
        arg: b"IP\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set source IP address\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"id\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'i' as ::core::ffi::c_int,
        arg: b"PID\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"log the process id with every line\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"stderr\0".as_ptr() as *const ::core::ffi::c_char,
        key: 's' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"copy the message to stderr\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"log the content of FILE\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"priority\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: b"PRI\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"log with priority PRI\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"tag\0".as_ptr() as *const ::core::ffi::c_char,
        key: 't' as ::core::ffi::c_int,
        arg: b"TAG\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"prepend every line with TAG\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP,
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
pub const GRP: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        52 => {
            host_family = AF_INET;
        }
        54 => {
            host_family = AF_INET6;
        }
        104 => {
            host = arg;
            unixsock = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        117 => {
            unixsock = arg;
            host = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        83 => {
            source = arg;
        }
        105 => {
            logflags |= LOG_PID;
            if !arg.is_null() {
                pidstr = arg;
            } else {
                let mut buf: [::core::ffi::c_char; 21] = [0; 21];
                arg = umaxtostr(
                    getpid() as uintmax_t,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
                pidstr = xstrdup(arg);
            }
        }
        115 => {
            logflags |= LOG_PERROR;
        }
        102 => {
            if strcmp(arg, b"-\0".as_ptr() as *const ::core::ffi::c_char) != 0
                && freopen(arg, b"r\0".as_ptr() as *const ::core::ffi::c_char, stdin).is_null()
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        arg,
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
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            arg,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        }
        112 => {
            pri = parse_level(arg);
        }
        116 => {
            tag = arg;
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
#[export_name = "rboxc_logger_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 2] = [
    b"Sergey Poznyakoff\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_logger(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    set_program_name(*argv.offset(0isize));
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"logger\0".as_ptr() as *const ::core::ffi::c_char,
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
    argc -= index;
    argv = argv.offset(index as isize);
    if tag.is_null() {
        tag = getenv(b"USER\0".as_ptr() as *const ::core::ffi::c_char);
        if tag.is_null() {
            let mut pw: *mut passwd = getpwuid(getuid());
            if !pw.is_null() {
                tag = xstrdup((*pw).pw_name);
            } else {
                tag = xstrdup(b"none\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
    }
    open_socket();
    if argc > 0 as ::core::ffi::c_int {
        let mut i: ::core::ffi::c_int = 0;
        let mut len: size_t = 0 as size_t;
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        i = 0 as ::core::ffi::c_int;
        while i < argc {
            len = len.wrapping_add(strlen(*argv.offset(i as isize)).wrapping_add(1 as size_t));
            i += 1;
        }
        buf = xmalloc(len) as *mut ::core::ffi::c_char;
        i = 0 as ::core::ffi::c_int;
        p = buf;
        while i < argc {
            len = strlen(*argv.offset(i as isize));
            memcpy(
                p as *mut ::core::ffi::c_void,
                *argv.offset(i as isize) as *const ::core::ffi::c_void,
                len,
            );
            p = p.offset(len as isize);
            let c2rust_fresh3 = p;
            p = p.offset(1);
            *c2rust_fresh3 = ' ' as ::core::ffi::c_char;
            i += 1;
        }
        *p.offset(-1isize) = 0 as ::core::ffi::c_char;
        send_to_syslog(buf);
    } else {
        let mut size: size_t = 0 as size_t;
        while rpl_getline(&raw mut buf, &raw mut size, stdin) > 0 as ssize_t {
            send_to_syslog(buf);
        }
    }
    free(buf as *mut ::core::ffi::c_void);
    exit(EXIT_SUCCESS);
}
