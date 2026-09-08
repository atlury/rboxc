// Generated from pinned GNU traceroute 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 3217039483becdd23aab7d0a0ed29810f25eaa37f3f0aa10befe4ba2540f8cd3
/*
  Copyright (C) 2007-2026 Free Software Foundation, Inc.

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
    fn pselect(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::core::ffi::c_int;
    fn getpid() -> __pid_t;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
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
    fn inet_ntoa(__in: in_addr) -> *mut ::core::ffi::c_char;
    fn gethostbyaddr(
        __addr: *const ::core::ffi::c_void,
        __len: __socklen_t,
        __type: ::core::ffi::c_int,
    ) -> *mut hostent;
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
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn strcat(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(_: *mut ::core::ffi::c_void);
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_traceroute_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_traceroute_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_traceroute_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_traceroute_argp_error"]
    fn argp_error(__state: *const argp_state, __fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_traceroute_timespec_sub"]
    fn timespec_sub(_: timespec, _: timespec) -> timespec;
    #[link_name = "rboxc_traceroute_current_timespec"]
    fn current_timespec() -> timespec;
    #[link_name = "rboxc_traceroute_icmp_generic_decode"]
    fn icmp_generic_decode(
        buffer: *mut ::core::ffi::c_uchar,
        bufsize: size_t,
        ipp: *mut *mut ip,
        icmpp: *mut *mut icmphdr_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_traceroute_icmp_echo_encode"]
    fn icmp_echo_encode(
        buffer: *mut ::core::ffi::c_uchar,
        bufsize: size_t,
        ident: ::core::ffi::c_int,
        seqno_0: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_traceroute_xrealloc"]
    fn xrealloc(p: *mut ::core::ffi::c_void, s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_traceroute_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_traceroute_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
}
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
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
pub type n_time = uint32_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct ip {
    #[bitfield(name = "ip_hl", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "ip_v", ty = "::core::ffi::c_uint", bits = "4..=7")]
    pub ip_hl_ip_v: [u8; 1],
    pub ip_tos: uint8_t,
    pub ip_len: ::core::ffi::c_ushort,
    pub ip_id: ::core::ffi::c_ushort,
    pub ip_off: ::core::ffi::c_ushort,
    pub ip_ttl: uint8_t,
    pub ip_p: uint8_t,
    pub ip_sum: ::core::ffi::c_ushort,
    pub ip_src: in_addr,
    pub ip_dst: in_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostent {
    pub h_name: *mut ::core::ffi::c_char,
    pub h_aliases: *mut *mut ::core::ffi::c_char,
    pub h_addrtype: ::core::ffi::c_int,
    pub h_length: ::core::ffi::c_int,
    pub h_addr_list: *mut *mut ::core::ffi::c_char,
}
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
pub struct icmp_header {
    pub icmp_type: ::core::ffi::c_uchar,
    pub icmp_code: ::core::ffi::c_uchar,
    pub icmp_cksum: ::core::ffi::c_ushort,
    pub icmp_hun: C2Rust_Unnamed_2,
    pub icmp_dun: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub id_ts: id_ts,
    pub id_ip: id_ip,
    pub id_mask: ::core::ffi::c_ulong,
    pub id_data: [::core::ffi::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct id_ip {
    pub idi_ip: ip,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct id_ts {
    pub its_otime: n_time,
    pub its_rtime: n_time,
    pub its_ttime: n_time,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub ih_pptr: ::core::ffi::c_uchar,
    pub ih_gwaddr: in_addr,
    pub ih_idseq: ih_idseq,
    pub ih_void: ::core::ffi::c_int,
    pub ih_pmtu: ih_pmtu,
    pub ih_rtradv: ih_rtradv,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_rtradv {
    pub irt_num_addrs: ::core::ffi::c_uchar,
    pub irt_wpa: ::core::ffi::c_uchar,
    pub irt_lifetime: ::core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_pmtu {
    pub ipm_void: ::core::ffi::c_ushort,
    pub ipm_nextmtu: ::core::ffi::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ih_idseq {
    pub icd_id: ::core::ffi::c_ushort,
    pub icd_seq: ::core::ffi::c_ushort,
}
pub type icmphdr_t = icmp_header;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct trace_type(pub ::core::ffi::c_uint);
impl trace_type {
    pub const TRACE_UDP: Self = Self(0);
    pub const TRACE_ICMP: Self = Self(1);
    pub const TRACE_1393: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct trace {
    pub icmpfd: ::core::ffi::c_int,
    pub udpfd: ::core::ffi::c_int,
    pub r#type: trace_type,
    pub no_ident: ::core::ffi::c_int,
    pub to: sockaddr_in,
    pub from: sockaddr_in,
    pub ttl: ::core::ffi::c_int,
    pub tsent: timespec,
}
pub type trace_t = trace;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const OPT_RESOLVE: Self = Self(256);
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const IP_OPTIONS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const IP_TOS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IP_TTL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IPOPT_LSRR: ::core::ffi::c_int = 131 as ::core::ffi::c_int;
pub const IPOPT_OPTVAL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const IPOPT_OLEN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IPOPT_OFFSET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IPOPT_MINOFF: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const MAX_IPOPTLEN: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const IPPORT_RESERVED: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
pub const AI_CANONNAME: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const AI_IDN: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const AI_CANONIDN: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EPROTONOSUPPORT: ::core::ffi::c_int = 93 as ::core::ffi::c_int;
pub const ECONNRESET: ::core::ffi::c_int = 104;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_KEY_ARG: ::core::ffi::c_int = 0;
pub const ARGP_KEY_SUCCESS: ::core::ffi::c_int = 16777220;
#[inline]
unsafe extern "C" fn timespectod(mut a: timespec) -> ::core::ffi::c_double {
    return a.tv_sec as ::core::ffi::c_double + a.tv_nsec as ::core::ffi::c_double / 1e9f64;
}
pub const ICMP_ECHOREPLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICMP_DEST_UNREACH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICMP_PORT_UNREACH: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICMP_TIME_EXCEEDED: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const TRACE_UDP_PORT: ::core::ffi::c_int = 33434 as ::core::ffi::c_int;
pub const TRACE_TTL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TIME_INTERVAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[export_name = "rboxc_traceroute_stop"]
pub static mut stop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_traceroute_pid"]
pub static mut pid: ::core::ffi::c_int = 0;
#[export_name = "rboxc_traceroute_seqno"]
pub static mut seqno: ::core::ffi::c_int = 0;
static mut hostname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_traceroute_addrstr"]
pub static mut addrstr: [::core::ffi::c_char; 46] = [0; 46];
#[export_name = "rboxc_traceroute_dest"]
pub static mut dest: sockaddr_in = sockaddr_in {
    sin_family: 0,
    sin_port: 0,
    sin_addr: in_addr { s_addr: 0 },
    sin_zero: [0; 8],
};
#[export_name = "rboxc_traceroute_len_ip_opts"]
pub static mut len_ip_opts: size_t = 0 as size_t;
#[export_name = "rboxc_traceroute_ip_opts"]
pub static mut ip_opts: [::core::ffi::c_char; 40] = [0; 40];
#[export_name = "rboxc_traceroute_unreach_sign"]
pub static mut unreach_sign: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"NHPPFS**U**TTXXX\0")
};
static mut opt_type: trace_type = trace_type::TRACE_UDP;
#[export_name = "rboxc_traceroute_opt_port"]
pub static mut opt_port: ::core::ffi::c_int = TRACE_UDP_PORT;
#[export_name = "rboxc_traceroute_opt_max_hops"]
pub static mut opt_max_hops: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
static mut opt_max_tries: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
#[export_name = "rboxc_traceroute_opt_resolve_hostnames"]
pub static mut opt_resolve_hostnames: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_traceroute_opt_tos"]
pub static mut opt_tos: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_traceroute_opt_ttl"]
pub static mut opt_ttl: ::core::ffi::c_int = TRACE_TTL;
#[export_name = "rboxc_traceroute_opt_wait"]
pub static mut opt_wait: ::core::ffi::c_int = TIME_INTERVAL;
#[export_name = "rboxc_traceroute_opt_gateways"]
pub static mut opt_gateways: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_traceroute_args_doc"]
pub static mut args_doc: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"HOST\0") };
#[export_name = "rboxc_traceroute_doc"]
pub static mut doc: [::core::ffi::c_char; 47] = unsafe {
    ::core::mem::transmute::<[u8; 47], [::core::ffi::c_char; 47]>(
        *b"Print the route packets trace to network host.\0",
    )
};
#[export_name = "rboxc_traceroute_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 2] = [
    b"Elian Gidoni\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut argp_options: [argp_option; 11] = [
    argp_option {
        name: b"first-hop\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: b"NUM\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set initial hop distance, i.e., time-to-live\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"gateways\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'g' as ::core::ffi::c_int,
        arg: b"GATES\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"list of gateways for loose source routing\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"icmp\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'I' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"use ICMP ECHO as probe\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"max-hop\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'm' as ::core::ffi::c_int,
        arg: b"NUM\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set maximal hop count (default: 64)\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"port\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: b"PORT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use destination PORT port (default: 33434)\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"resolve-hostnames\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OPT_RESOLVE.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"resolve hostnames\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"tos\0".as_ptr() as *const ::core::ffi::c_char,
        key: 't' as ::core::ffi::c_int,
        arg: b"NUM\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set type of service (TOS) to NUM\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"tries\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'q' as ::core::ffi::c_int,
        arg: b"NUM\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"send NUM probe packets per hop (default: 3)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"type\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'M' as ::core::ffi::c_int,
        arg: b"METHOD\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use METHOD (`icmp' or `udp') for traceroute operations, defaulting to `udp'\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"wait\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'w' as ::core::ffi::c_int,
        arg: b"NUM\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"wait NUM seconds for response (default: 3)\0".as_ptr() as *const ::core::ffi::c_char,
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
    static mut host_is_given: bool = r#false != 0;
    match key {
        102 => {
            opt_ttl = strtol(arg, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int != 0
                || opt_ttl <= 0 as ::core::ffi::c_int
                || opt_ttl > 255 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"impossible distance `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
                            0 as ::core::ffi::c_int,
                            b"impossible distance `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
        103 => {
            if !opt_gateways.is_null() {
                let mut len: size_t = 0 as size_t;
                len = strlen(opt_gateways);
                opt_gateways = xrealloc(
                    opt_gateways as *mut ::core::ffi::c_void,
                    len.wrapping_add(strlen(arg)).wrapping_add(3 as size_t),
                ) as *mut ::core::ffi::c_char;
                *opt_gateways.offset(len as isize) = ',' as ::core::ffi::c_char;
                *opt_gateways.offset(len.wrapping_add(1 as size_t) as isize) =
                    '\0' as ::core::ffi::c_char;
                strcat(opt_gateways, arg);
            } else {
                opt_gateways = xstrdup(arg);
            }
        }
        73 => {
            opt_type = trace_type::TRACE_ICMP;
        }
        109 => {
            opt_max_hops = strtol(arg, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int != 0
                || opt_max_hops <= 0 as ::core::ffi::c_int
                || opt_max_hops > 255 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"invalid hops value `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
                            0 as ::core::ffi::c_int,
                            b"invalid hops value `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
            opt_port = strtol(arg, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int != 0
                || opt_port <= 0 as ::core::ffi::c_int
                || opt_port > 65536 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"invalid port number `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
                            0 as ::core::ffi::c_int,
                            b"invalid port number `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
        113 => {
            opt_max_tries = strtol(arg, &raw mut p, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *p != 0 {
                argp_error(
                    state,
                    b"invalid value (`%s' near `%s')\0".as_ptr() as *const ::core::ffi::c_char,
                    arg,
                    p,
                );
            }
            if opt_max_tries < 1 as ::core::ffi::c_int || opt_max_tries > 10 as ::core::ffi::c_int {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"number of tries should be between 1 and 10\0".as_ptr()
                            as *const ::core::ffi::c_char,
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
                            b"number of tries should be between 1 and 10\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        }
        116 => {
            opt_tos = strtol(arg, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int != 0
                || opt_tos < 0 as ::core::ffi::c_int
                || opt_tos > 255 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"invalid TOS value `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
                            0 as ::core::ffi::c_int,
                            b"invalid TOS value `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
        77 => {
            if strcmp(arg, b"icmp\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                opt_type = trace_type::TRACE_ICMP;
            } else if strcmp(arg, b"udp\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
            {
                opt_type = trace_type::TRACE_UDP;
            } else {
                argp_error(
                    state,
                    b"invalid method\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
        119 => {
            opt_wait = strtol(arg, &raw mut p, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            if *p as ::core::ffi::c_int != 0
                || opt_wait < 0 as ::core::ffi::c_int
                || opt_wait > 60 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"ridiculous waiting time `%s'\0".as_ptr() as *const ::core::ffi::c_char,
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
                            0 as ::core::ffi::c_int,
                            b"ridiculous waiting time `%s'\0".as_ptr()
                                as *const ::core::ffi::c_char,
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
        256 => {
            opt_resolve_hostnames = 1 as ::core::ffi::c_int;
        }
        ARGP_KEY_ARG => {
            host_is_given = r#true != 0;
            hostname = xstrdup(arg);
        }
        ARGP_KEY_SUCCESS => {
            if !host_is_given {
                argp_error(
                    state,
                    b"missing host operand\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
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
pub unsafe extern "C" fn single_binary_main_traceroute(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut hop: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut rhost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut trace: trace_t = trace_t {
        icmpfd: 0,
        udpfd: 0,
        r#type: trace_type::TRACE_UDP,
        no_ident: 0,
        to: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
        from: sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        },
        ttl: 0,
        tsent: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    };
    set_program_name(*argv.offset(0isize));
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    pid = getpid() & 0xffff as ::core::ffi::c_int;
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"traceroute\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut program_authors as *mut *const ::core::ffi::c_char,
    );
    rpl_argp_parse(
        &raw mut argp,
        argc,
        argv,
        0 as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        NULL,
    );
    if hostname.is_null() || *hostname as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"unknown host\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"unknown host\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_family = AF_INET;
    hints.ai_flags = AI_CANONNAME;
    hints.ai_flags |= AI_IDN;
    hints.ai_flags |= AI_CANONIDN;
    rhost = hostname;
    rc = getaddrinfo(
        rhost,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut hints,
        &raw mut res,
    );
    if rc != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"unknown host\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"unknown host\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    memcpy(
        &raw mut dest as *mut ::core::ffi::c_void,
        (*res).ai_addr as *const ::core::ffi::c_void,
        (*res).ai_addrlen as size_t,
    );
    dest.sin_port = __bswap_16(opt_port as __uint16_t) as in_port_t;
    getnameinfo(
        (*res).ai_addr,
        (*res).ai_addrlen,
        &raw mut addrstr as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NUMERICHOST,
    );
    printf(
        b"traceroute to %s (%s), %d hops max\n\0".as_ptr() as *const ::core::ffi::c_char,
        if !(*res).ai_canonname.is_null() {
            (*res).ai_canonname
        } else {
            rhost
        },
        &raw mut addrstr as *mut ::core::ffi::c_char,
        opt_max_hops,
    );
    free(rhost as *mut ::core::ffi::c_void);
    freeaddrinfo(res);
    trace_ip_opts(&raw mut dest);
    trace_init(&raw mut trace, dest, opt_type);
    hop = 1 as ::core::ffi::c_int;
    seqno = -1 as ::core::ffi::c_int;
    while stop == 0 {
        if hop > opt_max_hops {
            exit(EXIT_FAILURE);
        }
        do_try(&raw mut trace, hop, opt_max_hops, opt_max_tries);
        trace_inc_ttl(&raw mut trace);
        trace_inc_port(&raw mut trace);
        hop += 1;
    }
    exit(EXIT_SUCCESS);
}
#[export_name = "rboxc_traceroute_do_try"]
pub unsafe extern "C" fn do_try(
    mut trace: *mut trace_t,
    hop: ::core::ffi::c_int,
    max_hops: ::core::ffi::c_int,
    max_tries: ::core::ffi::c_int,
) {
    let mut readset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut ret: ::core::ffi::c_int = 0;
    let mut tries: ::core::ffi::c_int = 0;
    let mut readonly: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut timeout: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut diff: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut triptime: ::core::ffi::c_double = 0.0f64;
    let mut prev_addr: uint32_t = 0 as uint32_t;
    printf(b" %2d  \0".as_ptr() as *const ::core::ffi::c_char, hop);
    tries = 0 as ::core::ffi::c_int;
    while tries < max_tries {
        let mut save_errno: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = trace_icmp_sock(trace);
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut readset;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        readset.fds_bits[(fd / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << fd % __NFDBITS) as __fd_mask;
        timeout = timespec {
            tv_sec: opt_wait as __time_t,
            tv_nsec: 0 as __syscall_slong_t,
        };
        if readonly == 0 {
            trace_write(trace);
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        ret = pselect(
            fd + 1 as ::core::ffi::c_int,
            &raw mut readset,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            &raw mut timeout,
            ::core::ptr::null::<__sigset_t>(),
        );
        save_errno = *__errno_location();
        diff = timespec_sub(current_timespec(), (*trace).tsent);
        's_17: {
            if ret < 0 as ::core::ffi::c_int {
                match save_errno {
                    EINTR => {}
                    _ => {
                        if 0 != 0 {
                            error(
                                1 as ::core::ffi::c_int,
                                *__errno_location(),
                                b"select failed\0".as_ptr() as *const ::core::ffi::c_char,
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
                                    b"select failed\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                    }
                }
            } else if ret == 0 as ::core::ffi::c_int {
                printf(b" * \0".as_ptr() as *const ::core::ffi::c_char);
                fflush(stdout);
            } else if readset.fds_bits[(fd / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << fd % __NFDBITS) as __fd_mask
                != 0 as __fd_mask
            {
                let mut rc: ::core::ffi::c_int = 0;
                let mut r#type: ::core::ffi::c_int = 0;
                let mut code: ::core::ffi::c_int = 0;
                triptime = timespectod(diff) * 1000.0f64;
                rc = trace_read(trace, &raw mut r#type, &raw mut code);
                if rc < 0 as ::core::ffi::c_int {
                    tries -= 1;
                    readonly = 1 as ::core::ffi::c_int;
                    break 's_17;
                } else {
                    if tries == 0 as ::core::ffi::c_int
                        || prev_addr != (*trace).from.sin_addr.s_addr
                    {
                        printf(
                            b" %s \0".as_ptr() as *const ::core::ffi::c_char,
                            inet_ntoa((*trace).from.sin_addr),
                        );
                        if opt_resolve_hostnames != 0 {
                            printf(
                                b"(%s) \0".as_ptr() as *const ::core::ffi::c_char,
                                get_hostname(&raw mut (*trace).from.sin_addr),
                            );
                        }
                    }
                    printf(
                        b" %.3fms \0".as_ptr() as *const ::core::ffi::c_char,
                        triptime,
                    );
                    if rc > 0 as ::core::ffi::c_int && r#type == ICMP_DEST_UNREACH {
                        printf(
                            b"!%c \0".as_ptr() as *const ::core::ffi::c_char,
                            unreach_sign[(code & 0xf as ::core::ffi::c_int) as usize]
                                as ::core::ffi::c_int,
                        );
                    }
                    prev_addr = (*trace).from.sin_addr.s_addr as uint32_t;
                }
            }
            readonly = 0 as ::core::ffi::c_int;
            fflush(stdout);
        }
        tries += 1;
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
#[export_name = "rboxc_traceroute_get_hostname"]
pub unsafe extern "C" fn get_hostname(mut addr: *mut in_addr) -> *mut ::core::ffi::c_char {
    let mut info: *mut hostent = gethostbyaddr(
        addr as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<in_addr>() as __socklen_t,
        AF_INET,
    );
    if !info.is_null() {
        return (*info).h_name;
    }
    return inet_ntoa(*addr);
}
#[export_name = "rboxc_traceroute_trace_init"]
pub unsafe extern "C" fn trace_init(mut t: *mut trace_t, to: sockaddr_in, r#type: trace_type) {
    let mut fd: ::core::ffi::c_int = 0;
    let mut ttlp: *const ::core::ffi::c_int = ::core::ptr::null::<::core::ffi::c_int>();
    '_c2rust_label: {
        if !t.is_null() {
        } else {
            __assert_fail(
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/inetutils-2.8/src/traceroute.c\0".as_ptr() as *const ::core::ffi::c_char,
                467 as ::core::ffi::c_uint,
                b"void trace_init(trace_t *, const struct sockaddr_in, const enum trace_type)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    ttlp = &raw mut (*t).ttl;
    (*t).r#type = r#type;
    (*t).to = to;
    (*t).ttl = opt_ttl;
    (*t).no_ident = 0 as ::core::ffi::c_int;
    if (*t).r#type.0 == trace_type::TRACE_UDP.0 {
        (*t).udpfd = socket(
            PF_INET,
            __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        if (*t).udpfd < 0 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"socket\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"socket\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        if setsockopt(
            (*t).udpfd,
            C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
            IP_TTL,
            ttlp as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"setsockopt\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"setsockopt\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if (*t).r#type.0 == trace_type::TRACE_ICMP.0 || (*t).r#type.0 == trace_type::TRACE_UDP.0 {
        let mut protocol: *mut protoent =
            getprotobyname(b"icmp\0".as_ptr() as *const ::core::ffi::c_char);
        if !protocol.is_null() {
            (*t).icmpfd = socket(
                PF_INET,
                __socket_type::SOCK_RAW.0 as ::core::ffi::c_int,
                (*protocol).p_proto,
            );
            if (*t).icmpfd < 0 as ::core::ffi::c_int
                && (*__errno_location() == EPERM || *__errno_location() == EACCES)
            {
                *__errno_location() = 0 as ::core::ffi::c_int;
                (*t).icmpfd = socket(
                    PF_INET,
                    __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
                    (*protocol).p_proto,
                );
                (*t).no_ident += 1;
                if *__errno_location() == EPROTONOSUPPORT {
                    *__errno_location() = EPERM;
                }
            }
            if (*t).icmpfd < 0 as ::core::ffi::c_int {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"socket\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"socket\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            if setsockopt(
                (*t).icmpfd,
                C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
                IP_TTL,
                ttlp as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
            ) < 0 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"setsockopt\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"setsockopt\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        } else {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"can't find supplied protocol 'icmp'\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"can't find supplied protocol 'icmp'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    fd = if (*t).r#type.0 == trace_type::TRACE_UDP.0 {
        (*t).udpfd
    } else {
        (*t).icmpfd
    };
    if opt_tos >= 0 as ::core::ffi::c_int {
        if setsockopt(
            fd,
            C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
            IP_TOS,
            &raw mut opt_tos as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"setsockopt(IP_TOS)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"setsockopt(IP_TOS)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if len_ip_opts != 0 {
        if setsockopt(
            fd,
            C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
            IP_OPTIONS,
            &raw mut ip_opts as *const ::core::ffi::c_void,
            len_ip_opts as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"setsockopt(IPOPT_LSRR)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"setsockopt(IPOPT_LSRR)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
}
#[export_name = "rboxc_traceroute_trace_port"]
pub unsafe extern "C" fn trace_port(mut t: *mut trace_t, port: ::core::ffi::c_ushort) {
    '_c2rust_label: {
        if !t.is_null() {
        } else {
            __assert_fail(
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/inetutils-2.8/src/traceroute.c\0".as_ptr() as *const ::core::ffi::c_char,
                543 as ::core::ffi::c_uint,
                b"void trace_port(trace_t *, const unsigned short)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (port as ::core::ffi::c_int) < IPPORT_RESERVED {
        (*t).to.sin_port = TRACE_UDP_PORT as in_port_t;
    } else {
        (*t).to.sin_port = port as in_port_t;
    };
}
#[export_name = "rboxc_traceroute_trace_read"]
pub unsafe extern "C" fn trace_read(
    mut t: *mut trace_t,
    mut r#type: *mut ::core::ffi::c_int,
    mut code: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut data: [::core::ffi::c_uchar; 136] = [0; 136];
    let mut ip: *mut ip = ::core::ptr::null_mut::<ip>();
    let mut ic: *mut icmphdr_t = ::core::ptr::null_mut::<icmphdr_t>();
    let mut siz: socklen_t = 0;
    '_c2rust_label: {
        if !t.is_null() {
        } else {
            __assert_fail(
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/inetutils-2.8/src/traceroute.c\0".as_ptr() as *const ::core::ffi::c_char,
                568 as ::core::ffi::c_uint,
                b"int trace_read(trace_t *, int *, int *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    siz = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
    len = recvfrom(
        (*t).icmpfd,
        &raw mut data as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char
            as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 136]>(),
        0 as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut (*t).from as *mut sockaddr,
        },
        &raw mut siz,
    ) as ::core::ffi::c_int;
    if len < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"recvfrom\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"recvfrom\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    icmp_generic_decode(
        &raw mut data as *mut ::core::ffi::c_uchar,
        ::core::mem::size_of::<[::core::ffi::c_uchar; 136]>(),
        &raw mut ip,
        &raw mut ic,
    );
    *r#type = (*ic).icmp_type as ::core::ffi::c_int;
    *code = (*ic).icmp_code as ::core::ffi::c_int;
    match (*t).r#type {
        trace_type::TRACE_UDP => {
            let mut port: *mut ::core::ffi::c_ushort =
                ::core::ptr::null_mut::<::core::ffi::c_ushort>();
            if (*ic).icmp_type as ::core::ffi::c_int != ICMP_TIME_EXCEEDED
                && (*ic).icmp_type as ::core::ffi::c_int != ICMP_DEST_UNREACH
            {
                return -1 as ::core::ffi::c_int;
            }
            port = (&raw mut (*ic).icmp_dun.id_ip.idi_ip as *mut ::core::ffi::c_void)
                .offset(
                    (((*ic).icmp_dun.id_ip.idi_ip.ip_hl() as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int) as isize,
                )
                .offset(::core::mem::size_of::<in_port_t>() as isize)
                as *mut ::core::ffi::c_ushort;
            if *port as ::core::ffi::c_int != (*t).to.sin_port as ::core::ffi::c_int {
                return -1 as ::core::ffi::c_int;
            }
            if (*ic).icmp_type as ::core::ffi::c_int == ICMP_DEST_UNREACH {
                stop = 1 as ::core::ffi::c_int;
            }
            if (*ic).icmp_type as ::core::ffi::c_int == ICMP_DEST_UNREACH
                && (*ic).icmp_code as ::core::ffi::c_int != ICMP_PORT_UNREACH
            {
                rc = 1 as ::core::ffi::c_int;
            }
        }
        trace_type::TRACE_ICMP => {
            if !((*ic).icmp_type as ::core::ffi::c_int == ICMP_TIME_EXCEEDED
                || (*ic).icmp_type as ::core::ffi::c_int == ICMP_ECHOREPLY
                || (*ic).icmp_type as ::core::ffi::c_int == ICMP_DEST_UNREACH)
            {
                return -1 as ::core::ffi::c_int;
            }
            if (*ic).icmp_type as ::core::ffi::c_int == ICMP_ECHOREPLY
                && (__bswap_16((*ic).icmp_hun.ih_idseq.icd_seq as __uint16_t) as ::core::ffi::c_int
                    != seqno
                    || __bswap_16((*ic).icmp_hun.ih_idseq.icd_id as __uint16_t)
                        as ::core::ffi::c_int
                        != pid
                        && (*t).no_ident == 0 as ::core::ffi::c_int)
            {
                return -1 as ::core::ffi::c_int;
            }
            if (*ic).icmp_type as ::core::ffi::c_int == ICMP_TIME_EXCEEDED
                || (*ic).icmp_type as ::core::ffi::c_int == ICMP_DEST_UNREACH
            {
                let mut seq: ::core::ffi::c_ushort = 0;
                let mut ident: ::core::ffi::c_ushort = 0;
                let mut old_ip: *mut ip = ::core::ptr::null_mut::<ip>();
                let mut old_icmp: *mut icmphdr_t = ::core::ptr::null_mut::<icmphdr_t>();
                old_ip = &raw mut (*ic).icmp_dun.id_ip.idi_ip;
                old_icmp = (old_ip as *mut ::core::ffi::c_void).offset(
                    (((*old_ip).ip_hl() as ::core::ffi::c_int) << 2 as ::core::ffi::c_int) as isize,
                ) as *mut icmphdr_t;
                seq = __bswap_16((*old_icmp).icmp_hun.ih_idseq.icd_seq as __uint16_t)
                    as ::core::ffi::c_ushort;
                ident = __bswap_16((*old_icmp).icmp_hun.ih_idseq.icd_id as __uint16_t)
                    as ::core::ffi::c_ushort;
                if ident as ::core::ffi::c_int != pid
                    || (*ic).icmp_type as ::core::ffi::c_int == ICMP_TIME_EXCEEDED
                        && seq as ::core::ffi::c_int != seqno
                {
                    return -1 as ::core::ffi::c_int;
                }
            }
            if (*ip).ip_src.s_addr == dest.sin_addr.s_addr
                || (*ic).icmp_type as ::core::ffi::c_int == ICMP_DEST_UNREACH
            {
                stop = 1 as ::core::ffi::c_int;
            }
            if (*ic).icmp_type as ::core::ffi::c_int == ICMP_DEST_UNREACH {
                rc = 1 as ::core::ffi::c_int;
            }
        }
        _ => {}
    }
    return rc;
}
#[export_name = "rboxc_traceroute_trace_write"]
pub unsafe extern "C" fn trace_write(mut t: *mut trace_t) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !t.is_null() {
        } else {
            __assert_fail(
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/inetutils-2.8/src/traceroute.c\0".as_ptr() as *const ::core::ffi::c_char,
                666 as ::core::ffi::c_uint,
                b"int trace_write(trace_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    match (*t).r#type {
        trace_type::TRACE_UDP => {
            let mut data: [::core::ffi::c_char; 9] =
                ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"SUPERMAN\0");
            len = sendto(
                (*t).udpfd,
                &raw mut data as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 9]>(),
                0 as ::core::ffi::c_int,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &raw mut (*t).to as *mut sockaddr,
                },
                ::core::mem::size_of::<sockaddr_in>() as socklen_t,
            ) as ::core::ffi::c_int;
            if len < 0 as ::core::ffi::c_int {
                match *__errno_location() {
                    ECONNRESET => {}
                    _ => {
                        if 0 != 0 {
                            error(
                                1 as ::core::ffi::c_int,
                                *__errno_location(),
                                b"sendto\0".as_ptr() as *const ::core::ffi::c_char,
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
                                    b"sendto\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                    }
                }
            }
            (*t).tsent = current_timespec();
        }
        trace_type::TRACE_ICMP => {
            let mut hdr: icmphdr_t = icmphdr_t {
                icmp_type: 0,
                icmp_code: 0,
                icmp_cksum: 0,
                icmp_hun: C2Rust_Unnamed_2 { ih_pptr: 0 },
                icmp_dun: C2Rust_Unnamed_1 {
                    id_ts: id_ts {
                        its_otime: 0,
                        its_rtime: 0,
                        its_ttime: 0,
                    },
                },
            };
            let mut i: ::core::ffi::c_uint = 0;
            i = 0 as ::core::ffi::c_uint;
            while (i as usize) < ::core::mem::size_of::<icmphdr_t>() {
                *(&raw mut hdr as *mut ::core::ffi::c_char).offset(i as isize) =
                    i as ::core::ffi::c_char;
                i = i.wrapping_add(1);
            }
            if (*t).no_ident != 0 {
                *(&raw mut hdr as *mut ::core::ffi::c_int).offset(
                    12usize.wrapping_div(::core::mem::size_of::<::core::ffi::c_int>()) as isize,
                ) = dest.sin_addr.s_addr as ::core::ffi::c_int;
            }
            seqno += 1;
            if icmp_echo_encode(
                &raw mut hdr as *mut ::core::ffi::c_uchar,
                ::core::mem::size_of::<icmphdr_t>(),
                pid,
                seqno,
            ) != 0
            {
                return -1 as ::core::ffi::c_int;
            }
            len = sendto(
                (*t).icmpfd,
                &raw mut hdr as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<icmphdr_t>(),
                0 as ::core::ffi::c_int,
                __CONST_SOCKADDR_ARG {
                    __sockaddr__: &raw mut (*t).to as *mut sockaddr,
                },
                ::core::mem::size_of::<sockaddr_in>() as socklen_t,
            ) as ::core::ffi::c_int;
            if len < 0 as ::core::ffi::c_int {
                match *__errno_location() {
                    ECONNRESET => {}
                    _ => {
                        if 0 != 0 {
                            error(
                                1 as ::core::ffi::c_int,
                                *__errno_location(),
                                b"sendto\0".as_ptr() as *const ::core::ffi::c_char,
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
                                    b"sendto\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                    }
                }
            }
            (*t).tsent = current_timespec();
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "rboxc_traceroute_trace_udp_sock"]
pub unsafe extern "C" fn trace_udp_sock(mut t: *mut trace_t) -> ::core::ffi::c_int {
    return if !t.is_null() {
        (*t).udpfd
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[export_name = "rboxc_traceroute_trace_icmp_sock"]
pub unsafe extern "C" fn trace_icmp_sock(mut t: *mut trace_t) -> ::core::ffi::c_int {
    return if !t.is_null() {
        (*t).icmpfd
    } else {
        -1 as ::core::ffi::c_int
    };
}
#[export_name = "rboxc_traceroute_trace_inc_ttl"]
pub unsafe extern "C" fn trace_inc_ttl(mut t: *mut trace_t) {
    let mut fd: ::core::ffi::c_int = 0;
    let mut ttlp: *const ::core::ffi::c_int = ::core::ptr::null::<::core::ffi::c_int>();
    '_c2rust_label: {
        if !t.is_null() {
        } else {
            __assert_fail(
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/inetutils-2.8/src/traceroute.c\0".as_ptr() as *const ::core::ffi::c_char,
                755 as ::core::ffi::c_uint,
                b"void trace_inc_ttl(trace_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    ttlp = &raw mut (*t).ttl;
    (*t).ttl += 1;
    fd = if (*t).r#type.0 == trace_type::TRACE_UDP.0 {
        (*t).udpfd
    } else {
        (*t).icmpfd
    };
    if setsockopt(
        fd,
        C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
        IP_TTL,
        ttlp as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"setsockopt\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"setsockopt\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
#[export_name = "rboxc_traceroute_trace_inc_port"]
pub unsafe extern "C" fn trace_inc_port(mut t: *mut trace_t) {
    '_c2rust_label: {
        if !t.is_null() {
        } else {
            __assert_fail(
                b"t\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/inetutils-2.8/src/traceroute.c\0".as_ptr() as *const ::core::ffi::c_char,
                767 as ::core::ffi::c_uint,
                b"void trace_inc_port(trace_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*t).r#type.0 == trace_type::TRACE_UDP.0 {
        (*t).to.sin_port = __bswap_16(
            (__bswap_16((*t).to.sin_port) as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
                as __uint16_t,
        ) as in_port_t;
    }
}
#[export_name = "rboxc_traceroute_trace_ip_opts"]
pub unsafe extern "C" fn trace_ip_opts(mut to: *mut sockaddr_in) {
    if !opt_gateways.is_null() && *opt_gateways as ::core::ffi::c_int != 0 {
        let mut gateway: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut optbase: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
        let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        memset(
            &raw mut hints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<addrinfo>(),
        );
        hints.ai_family = AF_INET;
        hints.ai_socktype = __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int;
        memset(
            &raw mut ip_opts as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[::core::ffi::c_char; 40]>(),
        );
        optbase = &raw mut ip_opts as *mut ::core::ffi::c_char;
        gateway = opt_gateways;
        *optbase.offset(IPOPT_OPTVAL as isize) = IPOPT_LSRR as ::core::ffi::c_char;
        *optbase.offset(IPOPT_OLEN as isize) =
            (IPOPT_MINOFF - 1 as ::core::ffi::c_int) as ::core::ffi::c_char;
        *optbase.offset(IPOPT_OFFSET as isize) = IPOPT_MINOFF as ::core::ffi::c_char;
        while !gateway.is_null()
            && *gateway as ::core::ffi::c_int != 0
            && (*optbase.offset(IPOPT_OFFSET as isize) as ::core::ffi::c_int)
                < (MAX_IPOPTLEN as usize).wrapping_sub(::core::mem::size_of::<in_addr>())
                    as ::core::ffi::c_int
        {
            let mut rc: ::core::ffi::c_int = 0;
            let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
            p = strpbrk(gateway, b" ,;:\0".as_ptr() as *const ::core::ffi::c_char);
            if !p.is_null() {
                let c2rust_fresh0 = p;
                p = p.offset(1);
                *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
            }
            rc = getaddrinfo(
                gateway,
                ::core::ptr::null::<::core::ffi::c_char>(),
                &raw mut hints,
                &raw mut res,
            );
            if rc != 0 {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"gateway `%s' %s\0".as_ptr() as *const ::core::ffi::c_char,
                        gateway,
                        gai_strerror(rc),
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
                            b"gateway `%s' %s\0".as_ptr() as *const ::core::ffi::c_char,
                            gateway,
                            gai_strerror(rc),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            memcpy(
                optbase.offset(*optbase.offset(IPOPT_OLEN as isize) as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void,
                &raw mut (*((*res).ai_addr as *mut sockaddr_in)).sin_addr
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<in_addr>(),
            );
            freeaddrinfo(res);
            *optbase.offset(IPOPT_OLEN as isize) = (*optbase.offset(IPOPT_OLEN as isize)
                as ::core::ffi::c_ulong)
                .wrapping_add(::core::mem::size_of::<in_addr>() as ::core::ffi::c_ulong)
                as ::core::ffi::c_char;
            gateway = p;
        }
        if !gateway.is_null() && *gateway as ::core::ffi::c_int != 0 {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"too many gateways specified\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"too many gateways specified\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        memcpy(
            optbase.offset(*optbase.offset(IPOPT_OLEN as isize) as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            &raw mut (*to).sin_addr as *const ::core::ffi::c_void,
            ::core::mem::size_of::<in_addr>(),
        );
        *optbase.offset(IPOPT_OLEN as isize) = (*optbase.offset(IPOPT_OLEN as isize)
            as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<in_addr>() as ::core::ffi::c_ulong)
            as ::core::ffi::c_char;
        len_ip_opts = (*optbase.offset(IPOPT_OLEN as isize) as ::core::ffi::c_int
            + 1 as ::core::ffi::c_int) as size_t;
    }
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
