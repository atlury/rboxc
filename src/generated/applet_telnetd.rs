// Generated from pinned GNU telnetd 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: fb2ba6010d5f82a139ce7637780d04a9dae3236dd5c26332b9c823371ea7f7d4
/*
  Copyright (C) 1993-2026 Free Software Foundation, Inc.

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
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
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
pub struct gl_set_impl { _opaque: [u8; 0] }
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
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getpeername(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
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
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn free(_: *mut ::core::ffi::c_void);
    #[link_name = "rboxc_telnetd_rpl_ioctl"]
    fn rpl_ioctl(fd: ::core::ffi::c_int, request: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_gl_hash_set_implementation"]
    static gl_hash_set_implementation: gl_set_implementation;
    #[link_name = "rboxc_telnetd_xalloc_die"]
    fn xalloc_die();
    #[link_name = "rboxc_telnetd_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnetd_hash_string"]
    fn hash_string(s: *const ::core::ffi::c_char, tablesize: size_t) -> size_t;
    #[link_name = "rboxc_telnetd_fatal"]
    fn fatal(f: ::core::ffi::c_int, msg: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_telnetd_io_setup"]
    fn io_setup();
    #[link_name = "rboxc_telnetd_set_neturg"]
    fn set_neturg();
    #[link_name = "rboxc_telnetd_net_output_datalen"]
    fn net_output_datalen(buf: *const ::core::ffi::c_void, l: size_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_io_drain"]
    fn io_drain();
    #[link_name = "rboxc_telnetd_ptyflush"]
    fn ptyflush();
    #[link_name = "rboxc_telnetd_netclear"]
    fn netclear();
    #[link_name = "rboxc_telnetd_netflush"]
    fn netflush();
    #[link_name = "rboxc_telnetd_init_termbuf"]
    fn init_termbuf();
    #[link_name = "rboxc_telnetd_expand_line"]
    fn expand_line(fmt: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnetd_cleanup"]
    fn cleanup(_: ::core::ffi::c_int);
    #[link_name = "rboxc_telnetd_copy_termbuf"]
    fn copy_termbuf();
    #[link_name = "rboxc_telnetd_get_slc_defaults"]
    fn get_slc_defaults();
    #[link_name = "rboxc_telnetd_localstat"]
    fn localstat();
    #[link_name = "rboxc_telnetd_send_do"]
    fn send_do(_: ::core::ffi::c_int, _: ::core::ffi::c_int);
    #[link_name = "rboxc_telnetd_send_will"]
    fn send_will(_: ::core::ffi::c_int, _: ::core::ffi::c_int);
    #[link_name = "rboxc_telnetd_telrcv"]
    fn telrcv();
    #[link_name = "rboxc_telnetd_willoption"]
    fn willoption(_: ::core::ffi::c_int);
    #[link_name = "rboxc_telnetd_startslave"]
    fn startslave(
        host: *mut ::core::ffi::c_char,
        autologin: ::core::ffi::c_int,
        autoname: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_getterminaltype"]
    fn getterminaltype(user_name_0: *mut ::core::ffi::c_char, len: size_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_net_input_level"]
    fn net_input_level() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_net_output_level"]
    fn net_output_level() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_net_read"]
    fn net_read() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_net_buffer_is_full"]
    fn net_buffer_is_full() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_net_output_byte"]
    fn net_output_byte(c: ::core::ffi::c_int);
    #[link_name = "rboxc_telnetd_pty_input_level"]
    fn pty_input_level() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_pty_read"]
    fn pty_read() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_pty_output_level"]
    fn pty_output_level() -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_pty_get_char"]
    fn pty_get_char(peek: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_pty_input_putback"]
    fn pty_input_putback(str: *const ::core::ffi::c_char, len: size_t) -> ::core::ffi::c_int;
    fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnetd_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_telnetd_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_telnetd_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_telnetd_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_telnetd_setsig"]
    fn setsig(sig: ::core::ffi::c_int, handler: sighandler_t) -> sighandler_t;
    #[link_name = "rboxc_telnetd_localhost"]
    fn localhost() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnetd_default_program_authors"]
    static mut default_program_authors: [*const ::core::ffi::c_char; 0];
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
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
pub type socklen_t = __socklen_t;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type sighandler_t = __sighandler_t;
pub type error_t = ::core::ffi::c_int;
pub type cc_t = ::core::ffi::c_uchar;
pub type gl_setelement_equals_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> bool>;
pub type gl_setelement_hashcode_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> size_t>;
pub type gl_setelement_dispose_fn = Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> ()>;
pub type gl_set_t = *mut gl_set_impl;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_set_implementation {
    pub nx_create_empty: Option<
        unsafe extern "C" fn(
            gl_set_implementation_t,
            gl_setelement_equals_fn,
            gl_setelement_hashcode_fn,
            gl_setelement_dispose_fn,
        ) -> gl_set_t,
    >,
    pub size: Option<unsafe extern "C" fn(gl_set_t) -> size_t>,
    pub search: Option<unsafe extern "C" fn(gl_set_t, *const ::core::ffi::c_void) -> bool>,
    pub nx_add:
        Option<unsafe extern "C" fn(gl_set_t, *const ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub remove_elt: Option<unsafe extern "C" fn(gl_set_t, *const ::core::ffi::c_void) -> bool>,
    pub set_free: Option<unsafe extern "C" fn(gl_set_t) -> ()>,
    pub iterator: Option<unsafe extern "C" fn(gl_set_t) -> gl_set_iterator_t>,
    pub iterator_next: Option<
        unsafe extern "C" fn(*mut gl_set_iterator_t, *mut *const ::core::ffi::c_void) -> bool,
    >,
    pub iterator_free: Option<unsafe extern "C" fn(*mut gl_set_iterator_t) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_set_iterator_t {
    pub vtable: *const gl_set_implementation,
    pub set: gl_set_t,
    pub count: size_t,
    pub p: *mut ::core::ffi::c_void,
    pub q: *mut ::core::ffi::c_void,
    pub i: size_t,
    pub j: size_t,
}
pub type gl_set_implementation_t = *const gl_set_implementation;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gl_set_impl_base {
    pub vtable: *const gl_set_implementation,
    pub equals_fn: gl_setelement_equals_fn,
    pub dispose_fn: gl_setelement_dispose_fn,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct telnetd_clocks {
    pub system: ::core::ffi::c_int,
    pub echotoggle: ::core::ffi::c_int,
    pub modenegotiated: ::core::ffi::c_int,
    pub didnetreceive: ::core::ffi::c_int,
    pub ttypesubopt: ::core::ffi::c_int,
    pub tspeedsubopt: ::core::ffi::c_int,
    pub environsubopt: ::core::ffi::c_int,
    pub oenvironsubopt: ::core::ffi::c_int,
    pub xdisplocsubopt: ::core::ffi::c_int,
    pub baseline: ::core::ffi::c_int,
    pub gotDM: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slcent {
    pub flag: ::core::ffi::c_uchar,
    pub val: cc_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct slcfun {
    pub defset: slcent,
    pub current: slcent,
    pub sptr: *mut cc_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const ACCEPT_ENV_OPTION: Self = Self(256);
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_KEEPALIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SO_OOBINLINE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const EAI_SYSTEM: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NI_NAMEREQD: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SIGTSTP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIGTTOU: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const TIOCPKT: ::core::ffi::c_int = 0x5420 as ::core::ffi::c_int;
pub const FIONBIO: ::core::ffi::c_int = 0x5421 as ::core::ffi::c_int;
pub const TIOCPKT_FLUSHWRITE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TIOCPKT_NOSTOP: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TIOCPKT_DOSTOP: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const TIOCPKT_IOCTL: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_NOTICE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LOG_DAEMON: ::core::ffi::c_int = (3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_AUTH: ::core::ffi::c_int = (4 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_ODELAY: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const IAC: ::core::ffi::c_int = 255 as ::core::ffi::c_int;
pub const SB: ::core::ffi::c_int = 250 as ::core::ffi::c_int;
pub const DM: ::core::ffi::c_int = 242 as ::core::ffi::c_int;
pub const SE: ::core::ffi::c_int = 240 as ::core::ffi::c_int;
pub const TELOPT_ECHO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TELOPT_SGA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const TELOPT_STATUS: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const TELOPT_TM: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const TELOPT_NAWS: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const TELOPT_LFLOW: ::core::ffi::c_int = 33 as ::core::ffi::c_int;
pub const TELOPT_LINEMODE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const LFLOW_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LFLOW_ON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn gl_set_nx_create_empty(
    mut implementation: gl_set_implementation_t,
    mut equals_fn: gl_setelement_equals_fn,
    mut hashcode_fn: gl_setelement_hashcode_fn,
    mut dispose_fn: gl_setelement_dispose_fn,
) -> gl_set_t {
    return (*implementation)
        .nx_create_empty
        .expect("non-null function pointer")(
        implementation, equals_fn, hashcode_fn, dispose_fn
    );
}
#[inline]
unsafe extern "C" fn gl_set_nx_add(
    mut set: gl_set_t,
    mut elt: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return (*(*(set as *const gl_set_impl_base)).vtable)
        .nx_add
        .expect("non-null function pointer")(set, elt);
}
#[inline]
unsafe extern "C" fn gl_set_create_empty(
    mut implementation: gl_set_implementation_t,
    mut equals_fn: gl_setelement_equals_fn,
    mut hashcode_fn: gl_setelement_hashcode_fn,
    mut dispose_fn: gl_setelement_dispose_fn,
) -> gl_set_t {
    let mut result: gl_set_t =
        gl_set_nx_create_empty(implementation, equals_fn, hashcode_fn, dispose_fn);
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
#[inline]
unsafe extern "C" fn gl_set_add(mut set: gl_set_t, mut elt: *const ::core::ffi::c_void) -> bool {
    let mut result: ::core::ffi::c_int = gl_set_nx_add(set, elt);
    if result < 0 as ::core::ffi::c_int {
        xalloc_die();
    }
    return result != 0;
}
pub const UNAME_IM_PREFIX: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"\r\n\0") };
pub const UNAME_IM_SUFFIX: [::core::ffi::c_char; 15] = unsafe {
    ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b" (%l) (%t)\r\n\r\n\0")
};
pub const MY_STATE_WILL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MY_STATE_DO: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const MY_WANT_STATE_DO: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const REAL_LINEMODE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const NO_AUTOKLUDGE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPTION_ARG_OPTIONAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
#[export_name = "rboxc_telnetd_login_invocation"]
pub static mut login_invocation: *mut ::core::ffi::c_char =
    b"/usr/bin/login -p -h %h %?u{-f -- %u}{-- %U}\0".as_ptr() as *const ::core::ffi::c_char
        as *mut ::core::ffi::c_char;
#[export_name = "rboxc_telnetd_keepalive"]
pub static mut keepalive: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_telnetd_reverse_lookup"]
pub static mut reverse_lookup: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_telnetd_alwayslinemode"]
pub static mut alwayslinemode: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_lmodetype"]
pub static mut lmodetype: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_hostinfo"]
pub static mut hostinfo: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_telnetd_pending_sigchld"]
pub static mut pending_sigchld: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_telnetd_net"]
pub static mut net: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_pty"]
pub static mut pty: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_remote_hostname"]
pub static mut remote_hostname: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_telnetd_local_hostname"]
pub static mut local_hostname: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_telnetd_user_name"]
pub static mut user_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_telnetd_line"]
pub static mut line: [::core::ffi::c_char; 256] = [0; 256];
#[export_name = "rboxc_telnetd_options"]
pub static mut options: [::core::ffi::c_char; 256] = [0; 256];
#[export_name = "rboxc_telnetd_do_dont_resp"]
pub static mut do_dont_resp: [::core::ffi::c_char; 256] = [0; 256];
#[export_name = "rboxc_telnetd_will_wont_resp"]
pub static mut will_wont_resp: [::core::ffi::c_char; 256] = [0; 256];
#[export_name = "rboxc_telnetd_linemode"]
pub static mut linemode: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_uselinemode"]
pub static mut uselinemode: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_editmode"]
pub static mut editmode: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_useeditmode"]
pub static mut useeditmode: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_flowmode"]
pub static mut flowmode: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_restartany"]
pub static mut restartany: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_diagnostic"]
pub static mut diagnostic: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_slctab"]
pub static mut slctab: [slcfun; 19] = [slcfun {
    defset: slcent { flag: 0, val: 0 },
    current: slcent { flag: 0, val: 0 },
    sptr: ::core::ptr::null_mut::<cc_t>(),
}; 19];
#[export_name = "rboxc_telnetd_terminaltype"]
pub static mut terminaltype: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_telnetd_SYNCHing"]
pub static mut SYNCHing: ::core::ffi::c_int = 0;
#[export_name = "rboxc_telnetd_clocks"]
pub static mut clocks: telnetd_clocks = telnetd_clocks {
    system: 0,
    echotoggle: 0,
    modenegotiated: 0,
    didnetreceive: 0,
    ttypesubopt: 0,
    tspeedsubopt: 0,
    environsubopt: 0,
    oenvironsubopt: 0,
    xdisplocsubopt: 0,
    baseline: 0,
    gotDM: 0,
};
#[export_name = "rboxc_telnetd_accept_env_set"]
pub static mut accept_env_set: gl_set_t = ::core::ptr::null_mut::<gl_set_impl>();
unsafe extern "C" fn string_hashcode(mut s: *const ::core::ffi::c_void) -> size_t {
    return hash_string(
        s as *const ::core::ffi::c_char,
        strlen(s as *const ::core::ffi::c_char),
    );
}
unsafe extern "C" fn string_equals(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> bool {
    return strcmp(
        a as *const ::core::ffi::c_char,
        b as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int;
}
static mut argp_options: [argp_option; 7] = [
    argp_option {
        name: b"accept-env\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_0::ACCEPT_ENV_OPTION.0 as ::core::ffi::c_int,
        arg: b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"accept the environment variable from clients\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID,
    },
    argp_option {
        name: b"exec-login\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'E' as ::core::ffi::c_int,
        arg: b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set program to be executed instead of standard login(1)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID,
    },
    argp_option {
        name: b"no-hostinfo\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'h' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not print host information before login has been completed\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID,
    },
    argp_option {
        name: b"linemode\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'l' as ::core::ffi::c_int,
        arg: b"MODE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"set line mode\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID,
    },
    argp_option {
        name: b"no-keepalive\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'n' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"disable TCP keep-alive\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID,
    },
    argp_option {
        name: b"reverse-lookup\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'U' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"refuse connections from addresses that cannot be mapped back into a symbolic name\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID,
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
pub const GRID: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        256 => {
            if accept_env_set.is_null() {
                accept_env_set = gl_set_create_empty(
                    &raw const gl_hash_set_implementation,
                    Some(
                        string_equals
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_void,
                                *const ::core::ffi::c_void,
                            ) -> bool,
                    ),
                    Some(
                        string_hashcode
                            as unsafe extern "C" fn(*const ::core::ffi::c_void) -> size_t,
                    ),
                    None,
                );
            }
            gl_set_add(accept_env_set, arg as *const ::core::ffi::c_void);
        }
        69 => {
            login_invocation = arg;
        }
        104 => {
            hostinfo = 0 as ::core::ffi::c_int;
        }
        108 => {
            parse_linemode(arg);
        }
        110 => {
            keepalive = 0 as ::core::ffi::c_int;
        }
        85 => {
            reverse_lookup = 1 as ::core::ffi::c_int;
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
        args_doc: ::core::ptr::null::<::core::ffi::c_char>(),
        doc: b"DARPA telnet protocol server\0".as_ptr() as *const ::core::ffi::c_char,
        children: ::core::ptr::null::<argp_child>(),
        help_filter: None,
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_telnetd(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    set_program_name(*argv.offset(0isize));
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"telnetd\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut default_program_authors as *mut *const ::core::ffi::c_char,
    );
    openlog(
        b"telnetd\0".as_ptr() as *const ::core::ffi::c_char,
        LOG_PID | LOG_ODELAY,
        LOG_DAEMON,
    );
    rpl_argp_parse(
        &raw mut argp,
        argc,
        argv,
        0 as ::core::ffi::c_uint,
        &raw mut index,
        NULL,
    );
    if argc != index {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"junk arguments in the command line\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"junk arguments in the command line\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    telnetd_setup(0 as ::core::ffi::c_int);
    return telnetd_run();
}
unsafe extern "C" fn parse_linemode(mut str: *mut ::core::ffi::c_char) {
    if str.is_null() {
        alwayslinemode = 1 as ::core::ffi::c_int;
    } else if strcmp(str, b"nokludge\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        lmodetype = NO_AUTOKLUDGE;
    } else {
        syslog(
            LOG_NOTICE,
            b"invalid argument to --linemode: %s\0".as_ptr() as *const ::core::ffi::c_char,
            str,
        );
    };
}
unsafe extern "C" fn telnetd_setup(mut fd: ::core::ffi::c_int) {
    let mut saddr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut buf2: [::core::ffi::c_char; 256] = [0; 256];
    let mut err: ::core::ffi::c_int = 0;
    let mut on: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut len: socklen_t = 0;
    let mut uname_0: [::core::ffi::c_char; 256] = [0; 256];
    let mut level: ::core::ffi::c_int = 0;
    len = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    if getpeername(
        fd,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut saddr as *mut sockaddr,
        },
        &raw mut len,
    ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_ERR,
            b"getpeername: %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    err = getnameinfo(
        &raw mut saddr as *mut sockaddr,
        len,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NUMERICHOST,
    );
    if err != 0 {
        let mut errmsg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if err == EAI_SYSTEM {
            errmsg = strerror(*__errno_location());
        } else {
            errmsg = gai_strerror(err);
        }
        syslog(
            LOG_AUTH | LOG_NOTICE,
            b"Cannot get address: %s\0".as_ptr() as *const ::core::ffi::c_char,
            errmsg,
        );
        fatal(
            fd,
            b"Cannot get address.\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
    }
    err = getnameinfo(
        &raw mut saddr as *mut sockaddr,
        len,
        &raw mut buf2 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NAMEREQD,
    );
    if reverse_lookup != 0 {
        let mut result: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut aip: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        if err != 0 {
            let mut errmsg_0: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            if err == EAI_SYSTEM {
                errmsg_0 = strerror(*__errno_location());
            } else {
                errmsg_0 = gai_strerror(err);
            }
            syslog(
                LOG_AUTH | LOG_NOTICE,
                b"Can't resolve %s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut buf as *mut ::core::ffi::c_char,
                errmsg_0,
            );
            fatal(
                fd,
                b"Cannot resolve address.\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        remote_hostname = xstrdup(&raw mut buf2 as *mut ::core::ffi::c_char);
        err = getaddrinfo(
            remote_hostname,
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<addrinfo>(),
            &raw mut result,
        );
        if err != 0 {
            let mut errmsg_1: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            if err == EAI_SYSTEM {
                errmsg_1 = strerror(*__errno_location());
            } else {
                errmsg_1 = gai_strerror(err);
            }
            syslog(
                LOG_AUTH | LOG_NOTICE,
                b"Forward resolve of %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                remote_hostname,
                errmsg_1,
            );
            fatal(
                fd,
                b"Cannot resolve address.\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        aip = result;
        while !aip.is_null() {
            if (*aip).ai_family == saddr.ss_family as ::core::ffi::c_int {
                if (*aip).ai_family == AF_INET
                    && memcmp(
                        &raw mut (*((*aip).ai_addr as *mut sockaddr_in)).sin_addr
                            as *const ::core::ffi::c_void,
                        &raw mut (*(&raw mut saddr as *mut sockaddr_in)).sin_addr
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<in_addr>(),
                    ) == 0
                {
                    break;
                }
                if (*aip).ai_family == AF_INET6
                    && memcmp(
                        &raw mut (*((*aip).ai_addr as *mut sockaddr_in6)).sin6_addr
                            as *const ::core::ffi::c_void,
                        &raw mut (*(&raw mut saddr as *mut sockaddr_in6)).sin6_addr
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<in6_addr>(),
                    ) == 0
                {
                    break;
                }
            }
            aip = (*aip).ai_next;
        }
        if aip.is_null() {
            syslog(
                LOG_AUTH | LOG_NOTICE,
                b"No address of %s matched %s\0".as_ptr() as *const ::core::ffi::c_char,
                remote_hostname,
                &raw mut buf as *mut ::core::ffi::c_char,
            );
            fatal(
                fd,
                b"Cannot resolve address.\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        freeaddrinfo(result);
    } else if err == 0 {
        remote_hostname = xstrdup(&raw mut buf2 as *mut ::core::ffi::c_char);
    } else {
        remote_hostname = xstrdup(&raw mut buf as *mut ::core::ffi::c_char);
    }
    if keepalive != 0
        && setsockopt(
            fd,
            SOL_SOCKET,
            SO_KEEPALIVE,
            &raw mut on as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_WARNING,
            b"setsockopt (SO_KEEPALIVE): %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    net = fd;
    local_hostname = localhost();
    io_setup();
    static mut dummy_environ: [*mut ::core::ffi::c_char; 1] =
        [::core::ptr::null_mut::<::core::ffi::c_char>()];
    environ = &raw mut dummy_environ as *mut *mut ::core::ffi::c_char;
    uname_0[0usize] = 0 as ::core::ffi::c_char;
    level = getterminaltype(
        &raw mut uname_0 as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>(),
    );
    setenv(
        b"TERM\0".as_ptr() as *const ::core::ffi::c_char,
        if !terminaltype.is_null() {
            terminaltype as *const ::core::ffi::c_char
        } else {
            b"network\0".as_ptr() as *const ::core::ffi::c_char
        },
        1 as ::core::ffi::c_int,
    );
    if uname_0[0usize] != 0 {
        user_name = xstrdup(&raw mut uname_0 as *mut ::core::ffi::c_char);
    }
    pty = startslave(remote_hostname, level, user_name);
    rpl_ioctl(pty, TIOCPKT, &raw mut on as *mut ::core::ffi::c_char);
    rpl_ioctl(pty, FIONBIO, &raw mut on as *mut ::core::ffi::c_char);
    rpl_ioctl(net, FIONBIO, &raw mut on as *mut ::core::ffi::c_char);
    setsockopt(
        net,
        SOL_SOCKET,
        SO_OOBINLINE,
        &raw mut on as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
    signal(
        SIGTSTP,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    signal(
        SIGTTOU,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    setsig(
        SIGCHLD,
        Some(chld_is_done as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
unsafe extern "C" fn telnetd_run() -> ::core::ffi::c_int {
    let mut nfd: ::core::ffi::c_int = 0;
    get_slc_defaults();
    if options[3usize] as ::core::ffi::c_int & MY_STATE_WILL == 0 {
        send_will(TELOPT_SGA, 1 as ::core::ffi::c_int);
    }
    send_do(TELOPT_ECHO, 1 as ::core::ffi::c_int);
    if options[34usize] as ::core::ffi::c_int & MY_STATE_DO == 0 {
        linemode = 0 as ::core::ffi::c_int;
        editmode = 0 as ::core::ffi::c_int;
        send_do(TELOPT_LINEMODE, 1 as ::core::ffi::c_int);
    }
    send_do(TELOPT_NAWS, 1 as ::core::ffi::c_int);
    send_will(TELOPT_STATUS, 1 as ::core::ffi::c_int);
    flowmode = 1 as ::core::ffi::c_int;
    restartany = -1 as ::core::ffi::c_int;
    send_do(TELOPT_LFLOW, 1 as ::core::ffi::c_int);
    while options[31usize] as ::core::ffi::c_int + 0x4 as ::core::ffi::c_int
        & 0x8 as ::core::ffi::c_int
        != 0
    {
        io_drain();
    }
    if options[1usize] as ::core::ffi::c_int & MY_WANT_STATE_DO != 0
        && options[31usize] as ::core::ffi::c_int & MY_STATE_DO != 0
    {
        while options[1usize] as ::core::ffi::c_int + 0x4 as ::core::ffi::c_int
            & 0x8 as ::core::ffi::c_int
            != 0
        {
            io_drain();
        }
    }
    if options[1usize] as ::core::ffi::c_int & MY_WANT_STATE_DO != 0 {
        willoption(TELOPT_ECHO);
    }
    if options[1usize] as ::core::ffi::c_int & MY_STATE_WILL == 0 {
        send_will(TELOPT_ECHO, 1 as ::core::ffi::c_int);
    }
    if lmodetype < REAL_LINEMODE {
        send_do(TELOPT_TM, 1 as ::core::ffi::c_int);
    }
    telrcv();
    if hostinfo != 0 {
        print_hostinfo();
    }
    init_termbuf();
    localstat();
    nfd = if net > pty { net } else { pty } + 1 as ::core::ffi::c_int;
    loop {
        let mut ibits: fd_set = fd_set { fds_bits: [0; 16] };
        let mut obits: fd_set = fd_set { fds_bits: [0; 16] };
        let mut xbits: fd_set = fd_set { fds_bits: [0; 16] };
        let mut c: ::core::ffi::c_int = 0;
        if net_input_level() < 0 as ::core::ffi::c_int
            && pty_input_level() < 0 as ::core::ffi::c_int
        {
            break;
        }
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut ibits;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        let mut __i_0: ::core::ffi::c_uint = 0;
        let mut __arr_0: *mut fd_set = &raw mut obits;
        __i_0 = 0 as ::core::ffi::c_uint;
        while (__i_0 as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr_0).fds_bits[__i_0 as usize] = 0 as __fd_mask;
            __i_0 = __i_0.wrapping_add(1);
        }
        let mut __i_1: ::core::ffi::c_uint = 0;
        let mut __arr_1: *mut fd_set = &raw mut xbits;
        __i_1 = 0 as ::core::ffi::c_uint;
        while (__i_1 as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr_1).fds_bits[__i_1 as usize] = 0 as __fd_mask;
            __i_1 = __i_1.wrapping_add(1);
        }
        if net_output_level() != 0 || pty_input_level() > 0 as ::core::ffi::c_int {
            obits.fds_bits[(net / __NFDBITS) as usize] |=
                ((1 as ::core::ffi::c_ulong) << net % __NFDBITS) as __fd_mask;
        } else {
            ibits.fds_bits[(pty / __NFDBITS) as usize] |=
                ((1 as ::core::ffi::c_ulong) << pty % __NFDBITS) as __fd_mask;
        }
        if pty_output_level() != 0 || net_input_level() > 0 as ::core::ffi::c_int {
            obits.fds_bits[(pty / __NFDBITS) as usize] |=
                ((1 as ::core::ffi::c_ulong) << pty % __NFDBITS) as __fd_mask;
        } else {
            ibits.fds_bits[(net / __NFDBITS) as usize] |=
                ((1 as ::core::ffi::c_ulong) << net % __NFDBITS) as __fd_mask;
        }
        if SYNCHing == 0 {
            xbits.fds_bits[(net / __NFDBITS) as usize] |=
                ((1 as ::core::ffi::c_ulong) << net % __NFDBITS) as __fd_mask;
        }
        c = select(
            nfd,
            &raw mut ibits,
            &raw mut obits,
            &raw mut xbits,
            ::core::ptr::null_mut::<timeval>(),
        );
        if c <= 0 as ::core::ffi::c_int {
            if c == -1 as ::core::ffi::c_int && *__errno_location() == EINTR {
                continue;
            }
            sleep(5 as ::core::ffi::c_uint);
        } else {
            if xbits.fds_bits[(net / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << net % __NFDBITS) as __fd_mask
                != 0 as __fd_mask
            {
                SYNCHing = 1 as ::core::ffi::c_int;
            }
            if ibits.fds_bits[(net / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << net % __NFDBITS) as __fd_mask
                != 0 as __fd_mask
            {
                net_read();
            }
            if ibits.fds_bits[(pty / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << pty % __NFDBITS) as __fd_mask
                != 0 as __fd_mask
            {
                if pty_read() < 0 as ::core::ffi::c_int {
                    break;
                }
                c = pty_get_char(1 as ::core::ffi::c_int);
                if c & TIOCPKT_IOCTL != 0 {
                    pty_get_char(0 as ::core::ffi::c_int);
                    copy_termbuf();
                    localstat();
                }
                if c & TIOCPKT_FLUSHWRITE != 0 {
                    static mut flushdata: [::core::ffi::c_char; 2] =
                        [IAC as ::core::ffi::c_char, DM as ::core::ffi::c_char];
                    pty_get_char(0 as ::core::ffi::c_int);
                    netclear();
                    net_output_datalen(
                        &raw mut flushdata as *mut ::core::ffi::c_char
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[::core::ffi::c_char; 2]>(),
                    );
                    set_neturg();
                }
                if options[33usize] as ::core::ffi::c_int & MY_STATE_DO != 0
                    && c & (TIOCPKT_NOSTOP | TIOCPKT_DOSTOP) != 0
                {
                    let mut newflow: ::core::ffi::c_int = if c & TIOCPKT_DOSTOP != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    };
                    if newflow != flowmode {
                        let mut data: [::core::ffi::c_char; 7] = [0; 7];
                        sprintf(
                            &raw mut data as *mut ::core::ffi::c_char,
                            b"%c%c%c%c%c%c\0".as_ptr() as *const ::core::ffi::c_char,
                            IAC,
                            SB,
                            TELOPT_LFLOW,
                            if flowmode != 0 { LFLOW_ON } else { LFLOW_OFF },
                            IAC,
                            SE,
                        );
                        net_output_datalen(
                            &raw mut data as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[::core::ffi::c_char; 7]>(),
                        );
                    }
                }
                pty_get_char(0 as ::core::ffi::c_int);
            }
            while pty_input_level() > 0 as ::core::ffi::c_int {
                if net_buffer_is_full() != 0 {
                    break;
                }
                c = pty_get_char(0 as ::core::ffi::c_int);
                if c == IAC {
                    net_output_byte(c);
                }
                net_output_byte(c);
                if c == '\r' as ::core::ffi::c_int
                    && options[0usize] as ::core::ffi::c_int & MY_STATE_WILL == 0
                {
                    if pty_input_level() > 0 as ::core::ffi::c_int
                        && pty_get_char(1 as ::core::ffi::c_int) == '\n' as ::core::ffi::c_int
                    {
                        net_output_byte(pty_get_char(0 as ::core::ffi::c_int));
                    } else {
                        net_output_byte(0 as ::core::ffi::c_int);
                    }
                }
            }
            if obits.fds_bits[(net / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << net % __NFDBITS) as __fd_mask
                != 0 as __fd_mask
                && net_output_level() > 0 as ::core::ffi::c_int
            {
                netflush();
            }
            if net_input_level() > 0 as ::core::ffi::c_int {
                telrcv();
            }
            if obits.fds_bits[(pty / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << pty % __NFDBITS) as __fd_mask
                != 0 as __fd_mask
                && pty_output_level() > 0 as ::core::ffi::c_int
            {
                ptyflush();
            }
            if pending_sigchld != 0 {
                if net_output_level() > 0 as ::core::ffi::c_int {
                    netflush();
                }
                cleanup(SIGCHLD);
            }
        }
    }
    cleanup(0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn print_hostinfo() {
    let mut im: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut u: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    if uname(&raw mut u) >= 0 as ::core::ffi::c_int {
        im = malloc(
            strlen(UNAME_IM_PREFIX.as_ptr())
                .wrapping_add(strlen(&raw mut u.sysname as *mut ::core::ffi::c_char))
                .wrapping_add(1 as size_t)
                .wrapping_add(strlen(&raw mut u.release as *mut ::core::ffi::c_char))
                .wrapping_add(strlen(UNAME_IM_SUFFIX.as_ptr()))
                .wrapping_add(1 as size_t),
        ) as *mut ::core::ffi::c_char;
        if !im.is_null() {
            sprintf(
                im,
                b"%s%s %s%s\0".as_ptr() as *const ::core::ffi::c_char,
                UNAME_IM_PREFIX.as_ptr(),
                &raw mut u.sysname as *mut ::core::ffi::c_char,
                &raw mut u.release as *mut ::core::ffi::c_char,
                UNAME_IM_SUFFIX.as_ptr(),
            );
        }
    }
    if im.is_null() {
        im = xstrdup(b"\r\n\r\nUNIX (%l) (%t)\r\n\r\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    str = expand_line(im);
    free(im as *mut ::core::ffi::c_void);
    pty_input_putback(str, strlen(str));
    free(str as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn chld_is_done(mut sig: ::core::ffi::c_int) {
    pending_sigchld = 1 as ::core::ffi::c_int;
}
