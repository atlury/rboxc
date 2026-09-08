// Generated from pinned GNU ping6 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 537479de25eb9e853d2d23d2d17a30c93dea37193735a70cb3f741b68950a771
/*
  Copyright (C) 2005-2026 Free Software Foundation, Inc.

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
    fn pselect(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *const timespec,
        __sigmask: *const __sigset_t,
    ) -> ::core::ffi::c_int;
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
    fn recvmsg(
        __fd: ::core::ffi::c_int,
        __message: *mut msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
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
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
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
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn setbuf(__stream: *mut FILE, __buf: *mut ::core::ffi::c_char);
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
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    #[link_name = "rboxc_ping6_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_ping6_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_ping6_argp_error"]
    fn argp_error(__state: *const argp_state, __fmt: *const ::core::ffi::c_char, ...);
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_ping6_timespec_add"]
    fn timespec_add(_: timespec, _: timespec) -> timespec;
    #[link_name = "rboxc_ping6_timespec_sub"]
    fn timespec_sub(_: timespec, _: timespec) -> timespec;
    #[link_name = "rboxc_ping6_current_timespec"]
    fn current_timespec() -> timespec;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_ping6_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_ping6_nsqrt"]
    fn nsqrt(a: ::core::ffi::c_double, prec: ::core::ffi::c_double) -> ::core::ffi::c_double;
    #[link_name = "rboxc_ping6_ping_cvt_number"]
    fn ping_cvt_number(
        rpl_optarg: *const ::core::ffi::c_char,
        maxval: size_t,
        allow_zero: ::core::ffi::c_int,
    ) -> size_t;
    #[link_name = "rboxc_ping6_init_data_buffer"]
    fn init_data_buffer(pat: *mut ::core::ffi::c_uchar, len: size_t);
    #[link_name = "rboxc_ping6_decode_pattern"]
    fn decode_pattern(
        text: *const ::core::ffi::c_char,
        pattern_len_0: *mut ::core::ffi::c_int,
        pattern_data: *mut ::core::ffi::c_uchar,
    );
    #[link_name = "rboxc_ping6__ping_setbuf"]
    fn _ping_setbuf(p: *mut PING, use_ipv6: bool) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping6_ping_set_data"]
    fn ping_set_data(
        p: *mut PING,
        data: *mut ::core::ffi::c_void,
        off: size_t,
        len: size_t,
        use_ipv6: bool,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping6_ping_set_count"]
    fn ping_set_count(ping_0: *mut PING, count_0: size_t);
    #[link_name = "rboxc_ping6_ping_set_sockopt"]
    fn ping_set_sockopt(
        ping_0: *mut PING,
        opt: ::core::ffi::c_int,
        val: *mut ::core::ffi::c_void,
        valsize: ::core::ffi::c_int,
    );
    #[link_name = "rboxc_ping6_ping_set_interval"]
    fn ping_set_interval(ping_0: *mut PING, interval_0: size_t);
    #[link_name = "rboxc_ping6_ping_unset_data"]
    fn ping_unset_data(p: *mut PING);
    #[link_name = "rboxc_ping6_ping_timeout_p"]
    fn ping_timeout_p(start_time: *mut timespec, timeout_0: ::core::ffi::c_int) -> bool;
    #[link_name = "rboxc_ping6_ipaddr2str"]
    fn ipaddr2str(from: *mut sockaddr, fromlen: socklen_t) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_ping6_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
}
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
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
pub struct msghdr {
    pub msg_name: *mut ::core::ffi::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::core::ffi::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::core::ffi::c_int,
    pub cmsg_type: ::core::ffi::c_int,
    pub __cmsg_data: [::core::ffi::c_uchar; 0],
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const IPPROTO_HOPOPTS: Self = Self(0);
    pub const IPPROTO_ROUTING: Self = Self(43);
    pub const IPPROTO_FRAGMENT: Self = Self(44);
    pub const IPPROTO_ICMPV6: Self = Self(58);
    pub const IPPROTO_NONE: Self = Self(59);
    pub const IPPROTO_DSTOPTS: Self = Self(60);
    pub const IPPROTO_MH: Self = Self(135);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdr {
    pub ip6_ctlun: C2Rust_Unnamed_2,
    pub ip6_src: in6_addr,
    pub ip6_dst: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub ip6_un1: ip6_hdrctl,
    pub ip6_un2_vfc: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ip6_hdrctl {
    pub ip6_un1_flow: uint32_t,
    pub ip6_un1_plen: uint16_t,
    pub ip6_un1_nxt: uint8_t,
    pub ip6_un1_hlim: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_filter {
    pub icmp6_filt: [uint32_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp6_hdr {
    pub icmp6_type: uint8_t,
    pub icmp6_code: uint8_t,
    pub icmp6_cksum: uint16_t,
    pub icmp6_dataun: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub icmp6_un_data32: [uint32_t; 1],
    pub icmp6_un_data16: [uint16_t; 2],
    pub icmp6_un_data8: [uint8_t; 4],
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
pub type n_time = uint32_t;
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
pub struct icmp_header {
    pub icmp_type: ::core::ffi::c_uchar,
    pub icmp_code: ::core::ffi::c_uchar,
    pub icmp_cksum: ::core::ffi::c_ushort,
    pub icmp_hun: C2Rust_Unnamed_5,
    pub icmp_dun: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
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
pub union C2Rust_Unnamed_5 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ping_stat {
    pub tmin: ::core::ffi::c_double,
    pub tmax: ::core::ffi::c_double,
    pub tsum: ::core::ffi::c_double,
    pub tsumsq: ::core::ffi::c_double,
}
pub type ping_efp6 = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
        *mut sockaddr_in6,
        *mut sockaddr_in6,
        *mut icmp6_hdr,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type ping_efp = Option<
    unsafe extern "C" fn(
        ::core::ffi::c_int,
        *mut ::core::ffi::c_void,
        *mut sockaddr_in,
        *mut sockaddr_in,
        *mut ip,
        *mut icmphdr_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub union event {
    pub handler6: ping_efp6,
    pub handler: ping_efp,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ping_address {
    pub ping_sockaddr: sockaddr_in,
    pub ping_sockaddr6: sockaddr_in6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ping_data {
    pub ping_fd: ::core::ffi::c_int,
    pub ping_type: ::core::ffi::c_int,
    pub ping_count: size_t,
    pub ping_start_time: timespec,
    pub ping_interval: size_t,
    pub ping_dest: ping_address,
    pub ping_hostname: *mut ::core::ffi::c_char,
    pub ping_datalen: size_t,
    pub ping_ident: ::core::ffi::c_int,
    pub ping_event: event,
    pub ping_closure: *mut ::core::ffi::c_void,
    pub ping_cktab_size: ::core::ffi::c_int,
    pub ping_cktab: *mut ::core::ffi::c_char,
    pub ping_buffer: *mut ::core::ffi::c_uchar,
    pub ping_from: ping_address,
    pub ping_num_xmit: size_t,
    pub ping_num_recv: size_t,
    pub ping_num_rept: size_t,
}
pub type PING = ping_data;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_diag {
    pub r#type: ::core::ffi::c_int,
    pub func: Option<unsafe extern "C" fn(*mut icmp6_hdr) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_code_descr {
    pub code: ::core::ffi::c_int,
    pub diag: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const ARG_HOPLIMIT: Self = Self(256);
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
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
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_DEBUG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_DONTROUTE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(
    mut __mhdr: *mut msghdr,
    mut __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    let mut __msg_control_ptr: *mut ::core::ffi::c_uchar =
        (*__mhdr).msg_control as *mut ::core::ffi::c_uchar;
    let mut __cmsg_ptr: *mut ::core::ffi::c_uchar = __cmsg as *mut ::core::ffi::c_uchar;
    let mut __size_needed: size_t = ::core::mem::size_of::<cmsghdr>().wrapping_add(
        ::core::mem::size_of::<size_t>().wrapping_sub(
            (*__cmsg).cmsg_len & ::core::mem::size_of::<size_t>().wrapping_sub(1 as size_t),
        ) & ::core::mem::size_of::<size_t>().wrapping_sub(1 as size_t),
    );
    if (*__cmsg).cmsg_len < ::core::mem::size_of::<cmsghdr>() {
        return ::core::ptr::null_mut::<cmsghdr>();
    }
    if (__msg_control_ptr
        .offset((*__mhdr).msg_controllen as isize)
        .offset_from(__cmsg_ptr) as size_t)
        < __size_needed
        || (__msg_control_ptr
            .offset((*__mhdr).msg_controllen as isize)
            .offset_from(__cmsg_ptr) as size_t)
            .wrapping_sub(__size_needed)
            < (*__cmsg).cmsg_len
    {
        return ::core::ptr::null_mut::<cmsghdr>();
    }
    __cmsg = (__cmsg as *mut ::core::ffi::c_uchar).offset(
        ((*__cmsg)
            .cmsg_len
            .wrapping_add(::core::mem::size_of::<size_t>())
            .wrapping_sub(1 as size_t)
            & !::core::mem::size_of::<size_t>().wrapping_sub(1usize)) as isize,
    ) as *mut cmsghdr;
    return __cmsg;
}
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const IPV6_UNICAST_HOPS: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const IPV6_RECVHOPLIMIT: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const IPV6_HOPLIMIT: ::core::ffi::c_int = 52 as ::core::ffi::c_int;
pub const IPV6_TCLASS: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ICMP6_FILTER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICMP6_PACKET_TOO_BIG: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICMP6_TIME_EXCEEDED: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICMP6_PARAM_PROB: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ICMP6_ECHO_REQUEST: ::core::ffi::c_int = 128;
pub const ICMP6_ECHO_REPLY: ::core::ffi::c_int = 129;
pub const ICMP6_DST_UNREACH_NOROUTE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH_ADMIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH_BEYONDSCOPE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH_ADDR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH_NOPORT: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const ICMP6_TIME_EXCEED_TRANSIT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICMP6_TIME_EXCEED_REASSEMBLY: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICMP6_PARAMPROB_HEADER: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ICMP6_PARAMPROB_NEXTHEADER: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICMP6_PARAMPROB_OPTION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AI_CANONNAME: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const AI_IDN: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const AI_CANONIDN: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const EAI_SYSTEM: ::core::ffi::c_int = -11 as ::core::ffi::c_int;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NI_IDN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EPERM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _IOLBF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_KEY_NO_ARGS: ::core::ffi::c_int = 16777218;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[inline]
unsafe extern "C" fn timespec_sign(mut a: timespec) -> ::core::ffi::c_int {
    return (a.tv_sec | a.tv_nsec > 0 as ::core::ffi::c_long) as ::core::ffi::c_int
        - (a.tv_sec | a.tv_nsec < 0 as ::core::ffi::c_long) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn timespectod(mut a: timespec) -> ::core::ffi::c_double {
    return a.tv_sec as ::core::ffi::c_double + a.tv_nsec as ::core::ffi::c_double / 1e9f64;
}
pub const MAXIPLEN: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const ICMP_MINLEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const ICMP_TSLEN: usize =
    8usize.wrapping_add(3usize.wrapping_mul(::core::mem::size_of::<n_time>()));
pub const MAXWAIT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAXPATTERN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OPT_FLOOD: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPT_INTERVAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPT_NUMERIC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPT_QUIET: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPT_VERBOSE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const OPT_TCLASS: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const PING_CKTABSIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const DEFAULT_PING_COUNT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PING_HEADER_LEN: usize = if USE_IPV6 != 0 {
    ::core::mem::size_of::<icmp6_hdr>()
} else {
    ICMP_MINLEN as usize
};
pub const PING_DATALEN: usize = 64usize.wrapping_sub(PING_HEADER_LEN);
pub const PING_DEFAULT_INTERVAL: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const PING_PRECISION: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const PING_MIN_USER_INTERVAL: ::core::ffi::c_int =
    200000 as ::core::ffi::c_int / PING_PRECISION;
pub const PING_MAX_DATALEN: usize = 65535usize.wrapping_sub(::core::mem::size_of::<icmp6_hdr>());
pub const USE_IPV6: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH_POLICYFAIL: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const ICMP6_DST_UNREACH_REJECTROUTE: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut ping: *mut PING = ::core::ptr::null_mut::<PING>();
#[export_name = "rboxc_ping6_is_root"]
pub static mut is_root: bool = r#false != 0;
#[export_name = "rboxc_ping6_data_buffer"]
pub static mut data_buffer: *mut ::core::ffi::c_uchar =
    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
#[export_name = "rboxc_ping6_patptr"]
pub static mut patptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
#[export_name = "rboxc_ping6_one"]
pub static mut one: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_ping6_pattern_len"]
pub static mut pattern_len: ::core::ffi::c_int = MAXPATTERN;
#[export_name = "rboxc_ping6_data_length"]
pub static mut data_length: size_t = 0;
#[export_name = "rboxc_ping6_count"]
pub static mut count: size_t = DEFAULT_PING_COUNT as size_t;
#[export_name = "rboxc_ping6_interval"]
pub static mut interval: size_t = 0;
#[export_name = "rboxc_ping6_socket_type"]
pub static mut socket_type: ::core::ffi::c_int = 0;
#[export_name = "rboxc_ping6_timeout"]
pub static mut timeout: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_ping6_hoplimit"]
pub static mut hoplimit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_ping6_options"]
pub static mut options: ::core::ffi::c_uint = 0;
static mut preload: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
#[export_name = "rboxc_ping6_tclass"]
pub static mut tclass: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_ping6_args_doc"]
pub static mut args_doc: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"HOST ...\0") };
#[export_name = "rboxc_ping6_doc"]
pub static mut doc: [::core::ffi::c_char; 114] = unsafe {
    ::core::mem::transmute::<
        [u8; 114],
        [::core::ffi::c_char; 114],
    >(
        *b"Send ICMP ECHO_REQUEST packets to network hosts.\x0BOptions marked with (root only) are available only to superuser.\0",
    )
};
#[export_name = "rboxc_ping6_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 2] = [
    b"Jeroen Dekkers\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut argp_options: [argp_option; 18] = [
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Options valid for all request types:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_0,
    },
    argp_option {
        name: b"count\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'c' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"stop after sending NUMBER packets\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'd' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"set the SO_DEBUG option\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"hoplimit\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_6::ARG_HOPLIMIT.0 as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"specify N as hop-limit\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"interval\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'i' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"wait NUMBER seconds between sending each packet\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"numeric\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'n' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not resolve host addresses\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ignore-routing\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'r' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"send directly to a host on an attached network\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"tos\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'T' as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set traffic class to N\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'w' as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"stop after N seconds\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ttl\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_6::ARG_HOPLIMIT.0 as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"synonym for --hoplimit\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'v' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"verbose output\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Options valid for --echo requests:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP,
    },
    argp_option {
        name: b"flood\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"flood ping (root only)\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"preload\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'l' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"send NUMBER packets as fast as possible before falling into normal mode of behavior (root only)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"pattern\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: b"PATTERN\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"fill ICMP packet with given pattern (hex)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'q' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"quiet output\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"size\0".as_ptr() as *const ::core::ffi::c_char,
        key: 's' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"send NUMBER data octets\0".as_ptr() as *const ::core::ffi::c_char,
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
pub const GRP_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GRP: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut pattern: [::core::ffi::c_uchar; 16] = [0; 16];
    's_162: {
        match key {
            99 => {
                count = ping_cvt_number(arg, 0 as size_t, 0 as ::core::ffi::c_int);
                break 's_162;
            }
            100 => {
                socket_type |= SO_DEBUG;
                break 's_162;
            }
            102 => {
                if !is_root {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"flooding needs root privilege\0".as_ptr()
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
                                b"flooding needs root privilege\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
                options |= OPT_FLOOD as ::core::ffi::c_uint;
                setbuf(stdout, NULL as *mut ::core::ffi::c_char);
                break 's_162;
            }
            105 => {
                options |= OPT_INTERVAL as ::core::ffi::c_uint;
                interval = ping_cvt_number(arg, 0 as size_t, 0 as ::core::ffi::c_int);
                interval = interval.wrapping_mul(PING_PRECISION as size_t);
                if !is_root && interval < PING_MIN_USER_INTERVAL as size_t {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"option value too small: %s\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"option value too small: %s\0".as_ptr()
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
                break 's_162;
            }
            108 => {
                if !is_root {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"preloading needs root privilege\0".as_ptr()
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
                                b"preloading needs root privilege\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
                preload = strtoul(arg, &raw mut endptr, 0 as ::core::ffi::c_int);
                if *endptr as ::core::ffi::c_int != 0 || preload > INT_MAX as ::core::ffi::c_ulong {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"preload size too large\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"preload size too large\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
                break 's_162;
            }
            110 => {
                options |= OPT_NUMERIC as ::core::ffi::c_uint;
                break 's_162;
            }
            112 => {
                decode_pattern(
                    arg,
                    &raw mut pattern_len,
                    &raw mut pattern as *mut ::core::ffi::c_uchar,
                );
                patptr = &raw mut pattern as *mut ::core::ffi::c_uchar;
                break 's_162;
            }
            113 => {
                options |= OPT_QUIET as ::core::ffi::c_uint;
                break 's_162;
            }
            114 => {
                socket_type |= SO_DONTROUTE;
                break 's_162;
            }
            115 => {
                data_length = ping_cvt_number(arg, PING_MAX_DATALEN, 1 as ::core::ffi::c_int);
                break 's_162;
            }
            84 => {
                options |= OPT_TCLASS as ::core::ffi::c_uint;
                tclass = ping_cvt_number(arg, 0 as size_t, 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_162;
            }
            118 => {
                options |= OPT_VERBOSE as ::core::ffi::c_uint;
                break 's_162;
            }
            119 => {
                timeout = ping_cvt_number(arg, INT_MAX as size_t, 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_162;
            }
            256 => {
                hoplimit = ping_cvt_number(arg, 255 as size_t, 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_162;
            }
            ARGP_KEY_NO_ARGS => {
                argp_error(
                    state,
                    b"missing host operand\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        return ARGP_ERR_UNKNOWN;
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
pub unsafe extern "C" fn single_binary_main_ping6(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    set_program_name(*argv.offset(0isize));
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    if getuid() == 0 as __uid_t {
        is_root = r#true != 0;
    }
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"ping6\0".as_ptr() as *const ::core::ffi::c_char,
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
    ping = ping_init(0 as ::core::ffi::c_int, getpid());
    if ping.is_null() {
        exit(EXIT_FAILURE);
    }
    setsockopt(
        (*ping).ping_fd,
        SOL_SOCKET,
        SO_BROADCAST,
        &raw mut one as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
    if setuid(getuid()) != 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    setvbuf(
        stdout,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        _IOLBF,
        0 as size_t,
    );
    argc -= index;
    argv = argv.offset(index as isize);
    if count != 0 as size_t {
        ping_set_count(ping, count);
    }
    if socket_type != 0 as ::core::ffi::c_int {
        ping_set_sockopt(
            ping,
            socket_type,
            &raw mut one as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
        );
    }
    if options & OPT_INTERVAL as ::core::ffi::c_uint != 0 {
        ping_set_interval(ping, interval);
    }
    if hoplimit > 0 as ::core::ffi::c_int {
        if setsockopt(
            (*ping).ping_fd,
            C2Rust_Unnamed_0::IPPROTO_IPV6.0 as ::core::ffi::c_int,
            IPV6_UNICAST_HOPS,
            &raw mut hoplimit as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"setsockopt(IPV6_HOPLIMIT)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"setsockopt(IPV6_HOPLIMIT)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if options & OPT_TCLASS as ::core::ffi::c_uint != 0 {
        if setsockopt(
            (*ping).ping_fd,
            C2Rust_Unnamed_0::IPPROTO_IPV6.0 as ::core::ffi::c_int,
            IPV6_TCLASS,
            &raw mut tclass as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"setsockopt(IPV6_TCLASS)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"setsockopt(IPV6_TCLASS)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    init_data_buffer(patptr, pattern_len as size_t);
    loop {
        let c2rust_fresh0 = argc;
        argc -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        let c2rust_fresh1 = argv;
        argv = argv.offset(1);
        status |= ping_echo(*c2rust_fresh1);
        ping_reset(ping);
    }
    return status;
}
static mut stop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn sig_int(mut signal_0: ::core::ffi::c_int) {
    ::core::ptr::write_volatile(&raw mut stop, 1 as ::core::ffi::c_int);
}
unsafe extern "C" fn ping_run(
    mut ping_0: *mut PING,
    mut finish: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
) -> ::core::ffi::c_int {
    let mut fdset: fd_set = fd_set { fds_bits: [0; 16] };
    let mut fdmax: ::core::ffi::c_int = 0;
    let mut resp_time: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut last: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut intvl: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut now: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    let mut t: *mut timespec = ::core::ptr::null_mut::<timespec>();
    let mut finishing: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nresp: size_t = 0 as size_t;
    let mut i: ::core::ffi::c_ulong = 0;
    signal(
        SIGINT,
        Some(sig_int as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    fdmax = (*ping_0).ping_fd + 1 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_ulong;
    while i < preload {
        send_echo(ping_0);
        i = i.wrapping_add(1);
    }
    if options & OPT_FLOOD as ::core::ffi::c_uint != 0 {
        intvl.tv_sec = 0 as __time_t;
        intvl.tv_nsec = 1e7f64 as __syscall_slong_t;
    } else {
        intvl.tv_sec = (*ping_0)
            .ping_interval
            .wrapping_div(PING_PRECISION as size_t) as __time_t;
        intvl.tv_nsec = ((*ping_0)
            .ping_interval
            .wrapping_rem(PING_PRECISION as size_t)
            as ::core::ffi::c_double
            * (1e9f64 / PING_PRECISION as ::core::ffi::c_double))
            as __syscall_slong_t;
    }
    last = current_timespec();
    send_echo(ping_0);
    while ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const stop) == 0 {
        let mut n: ::core::ffi::c_int = 0;
        let mut __i: ::core::ffi::c_uint = 0;
        let mut __arr: *mut fd_set = &raw mut fdset;
        __i = 0 as ::core::ffi::c_uint;
        while (__i as usize)
            < ::core::mem::size_of::<fd_set>().wrapping_div(::core::mem::size_of::<__fd_mask>())
        {
            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
            __i = __i.wrapping_add(1);
        }
        fdset.fds_bits[((*ping_0).ping_fd / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << (*ping_0).ping_fd % __NFDBITS) as __fd_mask;
        now = current_timespec();
        resp_time = timespec_sub(timespec_add(last, intvl), now);
        if timespec_sign(resp_time) == -1 as ::core::ffi::c_int {
            resp_time.tv_nsec = 0 as __syscall_slong_t;
            resp_time.tv_sec = resp_time.tv_nsec as __time_t;
        }
        n = pselect(
            fdmax,
            &raw mut fdset,
            ::core::ptr::null_mut::<fd_set>(),
            ::core::ptr::null_mut::<fd_set>(),
            &raw mut resp_time,
            ::core::ptr::null::<__sigset_t>(),
        );
        if n < 0 as ::core::ffi::c_int {
            if *__errno_location() != EINTR {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"pselect failed\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"pselect failed\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        } else if n == 1 as ::core::ffi::c_int {
            if ping_recv(ping_0) == 0 as ::core::ffi::c_int {
                nresp = nresp.wrapping_add(1);
            }
            if t.is_null() {
                now = current_timespec();
                t = &raw mut now;
            }
            if ping_timeout_p(&raw mut (*ping_0).ping_start_time, timeout) {
                break;
            }
            if (*ping_0).ping_count != 0 && nresp >= (*ping_0).ping_count {
                break;
            }
        } else {
            if (*ping_0).ping_count == 0 || (*ping_0).ping_num_xmit < (*ping_0).ping_count {
                send_echo(ping_0);
                if options & OPT_QUIET as ::core::ffi::c_uint == 0
                    && options & OPT_FLOOD as ::core::ffi::c_uint != 0
                {
                    putchar('.' as ::core::ffi::c_int);
                }
                if ping_timeout_p(&raw mut (*ping_0).ping_start_time, timeout) {
                    break;
                }
            } else {
                if finishing != 0 {
                    break;
                }
                finishing = 1 as ::core::ffi::c_int;
                intvl.tv_sec = MAXWAIT as __time_t;
            }
            last = current_timespec();
        }
    }
    ping_unset_data(ping_0);
    if finish.is_some() {
        return Some(finish.expect("non-null function pointer"))
            .expect("non-null function pointer")();
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn send_echo(mut ping_0: *mut PING) -> ::core::ffi::c_int {
    let mut off: size_t = 0 as size_t;
    let mut rc: ::core::ffi::c_int = 0;
    if data_length >= ::core::mem::size_of::<timeval>() {
        let mut now: timespec = current_timespec();
        let mut tv: timeval = timeval {
            tv_sec: now.tv_sec,
            tv_usec: now.tv_nsec / 1000 as __suseconds_t,
        };
        ping_set_data(
            ping_0,
            &raw mut tv as *mut ::core::ffi::c_void,
            0 as size_t,
            ::core::mem::size_of::<timeval>(),
            USE_IPV6 != 0,
        );
        off = (off as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<timeval>() as ::core::ffi::c_ulong)
            as size_t;
    }
    if !data_buffer.is_null() {
        ping_set_data(
            ping_0,
            data_buffer as *mut ::core::ffi::c_void,
            off,
            if data_length > off {
                data_length.wrapping_sub(off)
            } else {
                data_length
            },
            USE_IPV6 != 0,
        );
    }
    rc = ping_xmit(ping_0);
    if rc < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"sending packet\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"sending packet\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return rc;
}
unsafe extern "C" fn ping_finish() -> ::core::ffi::c_int {
    fflush(stdout);
    printf(
        b"--- %s ping statistics ---\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*ping).ping_hostname,
    );
    printf(
        b"%zu packets transmitted, \0".as_ptr() as *const ::core::ffi::c_char,
        (*ping).ping_num_xmit,
    );
    printf(
        b"%zu packets received, \0".as_ptr() as *const ::core::ffi::c_char,
        (*ping).ping_num_recv,
    );
    if (*ping).ping_num_rept != 0 {
        printf(
            b"+%zu duplicates, \0".as_ptr() as *const ::core::ffi::c_char,
            (*ping).ping_num_rept,
        );
    }
    if (*ping).ping_num_xmit != 0 {
        if (*ping).ping_num_recv > (*ping).ping_num_xmit {
            printf(b"-- somebody's printing up packets!\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            printf(
                b"%d%% packet loss\0".as_ptr() as *const ::core::ffi::c_char,
                (*ping)
                    .ping_num_xmit
                    .wrapping_sub((*ping).ping_num_recv)
                    .wrapping_mul(100 as size_t)
                    .wrapping_div((*ping).ping_num_xmit) as ::core::ffi::c_int,
            );
        }
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ping_echo(mut hostname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut buffer: [::core::ffi::c_char; 256] = [0; 256];
    let mut ping_stat: ping_stat = ping_stat {
        tmin: 0.,
        tmax: 0.,
        tsum: 0.,
        tsumsq: 0.,
    };
    let mut status: ::core::ffi::c_int = 0;
    if options & OPT_FLOOD as ::core::ffi::c_uint != 0
        && options & OPT_INTERVAL as ::core::ffi::c_uint != 0
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"-f and -i incompatible options\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"-f and -i incompatible options\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    memset(
        &raw mut ping_stat as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ping_stat>(),
    );
    ping_stat.tmin = 999999999.0f64;
    (*ping).ping_datalen = data_length;
    (*ping).ping_closure = &raw mut ping_stat as *mut ::core::ffi::c_void;
    if ping_set_dest(ping, hostname) != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"unknown host %s\0".as_ptr() as *const ::core::ffi::c_char,
                hostname,
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
                    b"unknown host %s\0".as_ptr() as *const ::core::ffi::c_char,
                    hostname,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    err = getnameinfo(
        &raw mut (*ping).ping_dest.ping_sockaddr6 as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in6>() as socklen_t,
        &raw mut buffer as *mut ::core::ffi::c_char,
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
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"getnameinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                errmsg,
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
                    b"getnameinfo: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    errmsg,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    printf(
        b"PING %s (%s): %zu data bytes\0".as_ptr() as *const ::core::ffi::c_char,
        (*ping).ping_hostname,
        &raw mut buffer as *mut ::core::ffi::c_char,
        data_length,
    );
    if options & OPT_VERBOSE as ::core::ffi::c_uint != 0 {
        printf(
            b", id 0x%04x = %u\0".as_ptr() as *const ::core::ffi::c_char,
            (*ping).ping_ident,
            (*ping).ping_ident,
        );
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    status = ping_run(
        ping,
        Some(echo_finish as unsafe extern "C" fn() -> ::core::ffi::c_int),
    );
    free((*ping).ping_hostname as *mut ::core::ffi::c_void);
    return status;
}
unsafe extern "C" fn ping_reset(mut p: *mut PING) {
    (*p).ping_num_xmit = 0 as size_t;
    (*p).ping_num_recv = 0 as size_t;
    (*p).ping_num_rept = 0 as size_t;
}
unsafe extern "C" fn print_echo(
    mut dupflag: ::core::ffi::c_int,
    mut hops: ::core::ffi::c_int,
    mut ping_stat: *mut ping_stat,
    mut dest: *mut sockaddr_in6,
    mut from: *mut sockaddr_in6,
    mut icmp6: *mut icmp6_hdr,
    mut datalen: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut buf: [::core::ffi::c_char; 256] = [0; 256];
    let mut timing: bool = r#false != 0;
    let mut triptime: ::core::ffi::c_double = 0.0f64;
    if (datalen as usize).wrapping_sub(::core::mem::size_of::<icmp6_hdr>())
        >= ::core::mem::size_of::<timeval>()
    {
        let mut tv: timeval = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        let mut ts: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        timing = r#true != 0;
        memcpy(
            &raw mut tv as *mut ::core::ffi::c_void,
            icmp6.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            ::core::mem::size_of::<timeval>(),
        );
        ts = timespec_sub(
            current_timespec(),
            timespec {
                tv_sec: tv.tv_sec,
                tv_nsec: tv.tv_usec * 1000 as __syscall_slong_t,
            },
        );
        triptime = timespectod(ts) * 1000.0f64;
        (*ping_stat).tsum += triptime;
        (*ping_stat).tsumsq += triptime * triptime;
        if triptime < (*ping_stat).tmin {
            (*ping_stat).tmin = triptime;
        }
        if triptime > (*ping_stat).tmax {
            (*ping_stat).tmax = triptime;
        }
    }
    if options & OPT_QUIET as ::core::ffi::c_uint != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if options & OPT_FLOOD as ::core::ffi::c_uint != 0 {
        putchar('\u{8}' as ::core::ffi::c_int);
        return 0 as ::core::ffi::c_int;
    }
    err = getnameinfo(
        from as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in6>() as socklen_t,
        &raw mut buf as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 256]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        if options & OPT_NUMERIC as ::core::ffi::c_uint != 0 {
            NI_NUMERICHOST
        } else {
            NI_IDN
        },
    );
    if err != 0 {
        let mut errmsg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if err == EAI_SYSTEM {
            errmsg = strerror(*__errno_location());
        } else {
            errmsg = gai_strerror(err);
        }
        fprintf(
            stderr,
            b"ping: getnameinfo: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            errmsg,
        );
        strcpy(
            &raw mut buf as *mut ::core::ffi::c_char,
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    printf(
        b"%d bytes from %s: icmp_seq=%u\0".as_ptr() as *const ::core::ffi::c_char,
        datalen,
        &raw mut buf as *mut ::core::ffi::c_char,
        __bswap_16((*icmp6).icmp6_dataun.icmp6_un_data16[1usize]) as ::core::ffi::c_int,
    );
    if hops >= 0 as ::core::ffi::c_int {
        printf(b" ttl=%d\0".as_ptr() as *const ::core::ffi::c_char, hops);
    }
    if timing {
        printf(
            b" time=%.3f ms\0".as_ptr() as *const ::core::ffi::c_char,
            triptime,
        );
    }
    if dupflag != 0 {
        printf(b" (DUP!)\0".as_ptr() as *const ::core::ffi::c_char);
    }
    putchar('\n' as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
static mut icmp_dest_unreach_desc: [icmp_code_descr; 7] = [
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_NOROUTE,
        diag: b"No route to destination\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_ADMIN,
        diag: b"Destination administratively prohibited\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_BEYONDSCOPE,
        diag: b"Beyond scope of source address\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_ADDR,
        diag: b"Address unreachable\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_NOPORT,
        diag: b"Port unreachable\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_POLICYFAIL,
        diag: b"Source address failed ingress/egress policy\0".as_ptr()
            as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_DST_UNREACH_REJECTROUTE,
        diag: b"Reject route to destination\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
unsafe extern "C" fn print_dst_unreach(mut icmp6: *mut icmp6_hdr) {
    let mut p: *mut icmp_code_descr = ::core::ptr::null_mut::<icmp_code_descr>();
    printf(b"Destination unreachable: \0".as_ptr() as *const ::core::ffi::c_char);
    p = &raw mut icmp_dest_unreach_desc as *mut icmp_code_descr;
    while p
        < (&raw mut icmp_dest_unreach_desc as *mut icmp_code_descr).offset(
            ::core::mem::size_of::<[icmp_code_descr; 7]>()
                .wrapping_div(::core::mem::size_of::<icmp_code_descr>()) as isize,
        )
    {
        if (*p).code == (*icmp6).icmp6_code as ::core::ffi::c_int {
            puts((*p).diag);
            return;
        }
        p = p.offset(1);
    }
    printf(
        b"Unknown code %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*icmp6).icmp6_code as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn print_packet_too_big(mut icmp6: *mut icmp6_hdr) {
    printf(
        b"Packet too big, mtu=%u\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*icmp6).icmp6_dataun.icmp6_un_data32[0usize],
    );
}
static mut icmp_time_exceeded_desc: [icmp_code_descr; 2] = [
    icmp_code_descr {
        code: ICMP6_TIME_EXCEED_TRANSIT,
        diag: b"Hop limit exceeded\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_TIME_EXCEED_REASSEMBLY,
        diag: b"Fragment reassembly timeout\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
unsafe extern "C" fn print_time_exceeded(mut icmp6: *mut icmp6_hdr) {
    let mut p: *mut icmp_code_descr = ::core::ptr::null_mut::<icmp_code_descr>();
    printf(b"Time exceeded: \0".as_ptr() as *const ::core::ffi::c_char);
    p = &raw mut icmp_time_exceeded_desc as *mut icmp_code_descr;
    while p
        < (&raw mut icmp_time_exceeded_desc as *mut icmp_code_descr).offset(
            ::core::mem::size_of::<[icmp_code_descr; 2]>()
                .wrapping_div(::core::mem::size_of::<icmp_code_descr>()) as isize,
        )
    {
        if (*p).code == (*icmp6).icmp6_code as ::core::ffi::c_int {
            puts((*p).diag);
            return;
        }
        p = p.offset(1);
    }
    printf(
        b"Unknown code %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*icmp6).icmp6_code as ::core::ffi::c_int,
    );
}
static mut icmp_param_prob_desc: [icmp_code_descr; 3] = [
    icmp_code_descr {
        code: ICMP6_PARAMPROB_HEADER,
        diag: b"Erroneous header field\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_PARAMPROB_NEXTHEADER,
        diag: b"Unrecognized Next Header type\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
    icmp_code_descr {
        code: ICMP6_PARAMPROB_OPTION,
        diag: b"Unrecognized IPv6 option\0".as_ptr() as *const ::core::ffi::c_char
            as *mut ::core::ffi::c_char,
    },
];
unsafe extern "C" fn print_param_prob(mut icmp6: *mut icmp6_hdr) {
    let mut p: *mut icmp_code_descr = ::core::ptr::null_mut::<icmp_code_descr>();
    printf(b"Parameter problem: \0".as_ptr() as *const ::core::ffi::c_char);
    p = &raw mut icmp_param_prob_desc as *mut icmp_code_descr;
    while p
        < (&raw mut icmp_param_prob_desc as *mut icmp_code_descr).offset(
            ::core::mem::size_of::<[icmp_code_descr; 3]>()
                .wrapping_div(::core::mem::size_of::<icmp_code_descr>()) as isize,
        )
    {
        if (*p).code == (*icmp6).icmp6_code as ::core::ffi::c_int {
            puts((*p).diag);
            return;
        }
        p = p.offset(1);
    }
    printf(
        b"Unknown code %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*icmp6).icmp6_code as ::core::ffi::c_int,
    );
}
#[export_name = "rboxc_ping6_print_ip_data"]
pub unsafe extern "C" fn print_ip_data(mut icmp6: *mut icmp6_hdr) {
    let mut j: size_t = 0;
    let mut ip: *mut ip6_hdr = (icmp6 as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<icmp6_hdr>() as isize)
        as *mut ip6_hdr;
    let mut src: [::core::ffi::c_char; 46] = [0; 46];
    let mut dst: [::core::ffi::c_char; 46] = [0; 46];
    inet_ntop(
        AF_INET6,
        &raw mut (*ip).ip6_dst as *const ::core::ffi::c_void,
        &raw mut dst as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
    );
    inet_ntop(
        AF_INET6,
        &raw mut (*ip).ip6_src as *const ::core::ffi::c_void,
        &raw mut src as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
    );
    printf(b"IP Header Dump:\n \0".as_ptr() as *const ::core::ffi::c_char);
    j = 0 as size_t;
    while j < ::core::mem::size_of::<ip6_hdr>()
        .wrapping_sub(::core::mem::size_of::<in6_addr>())
        .wrapping_sub(::core::mem::size_of::<in6_addr>())
    {
        printf(
            b"%02x%s\0".as_ptr() as *const ::core::ffi::c_char,
            *(ip as *mut ::core::ffi::c_uchar).offset(j as isize) as ::core::ffi::c_int,
            if j.wrapping_rem(2 as size_t) != 0 {
                b" \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        j = j.wrapping_add(1);
    }
    printf(b"(src) (dst)\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(b"Vr TC Flow Plen Nxt Hop Src\t\t  Dst\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b" %1x %02x %04x %4hu %3hhu %3hhu %s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        __bswap_32((*ip).ip6_ctlun.ip6_un1.ip6_un1_flow) >> 28 as ::core::ffi::c_int,
        (__bswap_32((*ip).ip6_ctlun.ip6_un1.ip6_un1_flow) & 0xfffffff as __uint32_t)
            >> 20 as ::core::ffi::c_int,
        __bswap_32((*ip).ip6_ctlun.ip6_un1.ip6_un1_flow) & 0xfffff as __uint32_t,
        __bswap_16((*ip).ip6_ctlun.ip6_un1.ip6_un1_plen) as ::core::ffi::c_int,
        (*ip).ip6_ctlun.ip6_un1.ip6_un1_nxt as ::core::ffi::c_int,
        (*ip).ip6_ctlun.ip6_un1.ip6_un1_hlim as ::core::ffi::c_int,
        &raw mut src as *mut ::core::ffi::c_char,
        &raw mut dst as *mut ::core::ffi::c_char,
    );
    match (*ip).ip6_ctlun.ip6_un1.ip6_un1_nxt as ::core::ffi::c_int {
        58 => {
            let mut hdr: *mut icmp6_hdr = (ip as *mut ::core::ffi::c_uchar)
                .offset(::core::mem::size_of::<ip6_hdr>() as isize)
                as *mut icmp6_hdr;
            printf(
                b"ICMP: type %hhu, code %hhu, size %hu\0".as_ptr() as *const ::core::ffi::c_char,
                (*hdr).icmp6_type as ::core::ffi::c_int,
                (*hdr).icmp6_code as ::core::ffi::c_int,
                __bswap_16((*ip).ip6_ctlun.ip6_un1.ip6_un1_plen) as ::core::ffi::c_int,
            );
            match (*hdr).icmp6_type as ::core::ffi::c_int {
                ICMP6_ECHO_REQUEST | ICMP6_ECHO_REPLY => {
                    printf(
                        b", id 0x%04x, seq 0x%04x\0".as_ptr() as *const ::core::ffi::c_char,
                        __bswap_16((*hdr).icmp6_dataun.icmp6_un_data16[0usize])
                            as ::core::ffi::c_int,
                        __bswap_16((*hdr).icmp6_dataun.icmp6_un_data16[1usize])
                            as ::core::ffi::c_int,
                    );
                }
                _ => {}
            }
        }
        _ => {}
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
static mut icmp_diag: [icmp_diag; 4] = [
    icmp_diag {
        r#type: ICMP6_DST_UNREACH,
        func: Some(print_dst_unreach as unsafe extern "C" fn(*mut icmp6_hdr) -> ()),
    },
    icmp_diag {
        r#type: ICMP6_PACKET_TOO_BIG,
        func: Some(print_packet_too_big as unsafe extern "C" fn(*mut icmp6_hdr) -> ()),
    },
    icmp_diag {
        r#type: ICMP6_TIME_EXCEEDED,
        func: Some(print_time_exceeded as unsafe extern "C" fn(*mut icmp6_hdr) -> ()),
    },
    icmp_diag {
        r#type: ICMP6_PARAM_PROB,
        func: Some(print_param_prob as unsafe extern "C" fn(*mut icmp6_hdr) -> ()),
    },
];
unsafe extern "C" fn print_icmp_error(
    mut from: *mut sockaddr_in6,
    mut icmp6: *mut icmp6_hdr,
    mut len: ::core::ffi::c_int,
) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut icmp_diag = ::core::ptr::null_mut::<icmp_diag>();
    s = ipaddr2str(
        from as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in6>() as socklen_t,
    );
    printf(
        b"%d bytes from %s: \0".as_ptr() as *const ::core::ffi::c_char,
        len,
        s,
    );
    free(s as *mut ::core::ffi::c_void);
    p = &raw mut icmp_diag as *mut icmp_diag as *mut icmp_diag;
    while p
        < (&raw mut icmp_diag as *mut icmp_diag).offset(
            ::core::mem::size_of::<[icmp_diag; 4]>()
                .wrapping_div(::core::mem::size_of::<icmp_diag>()) as isize,
        )
    {
        if (*p).r#type == (*icmp6).icmp6_type as ::core::ffi::c_int {
            (*p).func.expect("non-null function pointer")(icmp6);
            if options & OPT_VERBOSE as ::core::ffi::c_uint != 0 {
                print_ip_data(icmp6);
            }
            return;
        }
        p = p.offset(1);
    }
    printf(
        b"Unknown ICMP type: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*icmp6).icmp6_type as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn echo_finish() -> ::core::ffi::c_int {
    ping_finish();
    if (*ping).ping_num_recv != 0 && data_length >= ::core::mem::size_of::<timeval>() {
        let mut ping_stat: *mut ping_stat = (*ping).ping_closure as *mut ping_stat;
        let mut total: ::core::ffi::c_double =
            (*ping).ping_num_recv.wrapping_add((*ping).ping_num_rept) as ::core::ffi::c_double;
        let mut avg: ::core::ffi::c_double = (*ping_stat).tsum / total;
        let mut vari: ::core::ffi::c_double = (*ping_stat).tsumsq / total - avg * avg;
        printf(
            b"round-trip min/avg/max/stddev = %.3f/%.3f/%.3f/%.3f ms\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (*ping_stat).tmin,
            avg,
            (*ping_stat).tmax,
            nsqrt(vari, 0.0005f64),
        );
    }
    return ((*ping).ping_num_recv == 0 as size_t) as ::core::ffi::c_int;
}
unsafe extern "C" fn ping_init(
    mut r#type: ::core::ffi::c_int,
    mut ident: ::core::ffi::c_int,
) -> *mut PING {
    let mut fd: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    let on: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut p: *mut PING = ::core::ptr::null_mut::<PING>();
    let mut filter: icmp6_filter = icmp6_filter { icmp6_filt: [0; 8] };
    fd = socket(
        PF_INET6,
        __socket_type::SOCK_RAW.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_1::IPPROTO_ICMPV6.0 as ::core::ffi::c_int,
    );
    if fd < 0 as ::core::ffi::c_int {
        if *__errno_location() == EPERM || *__errno_location() == EACCES {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"raw socket\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"raw socket\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        return ::core::ptr::null_mut::<PING>();
    }
    memset(
        &raw mut filter as *mut ::core::ffi::c_void,
        0xff as ::core::ffi::c_int,
        ::core::mem::size_of::<icmp6_filter>(),
    );
    filter.icmp6_filt[(129 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize] = (filter
        .icmp6_filt[(129 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << (129 as ::core::ffi::c_int & 31 as ::core::ffi::c_int)))
        as uint32_t;
    filter.icmp6_filt[(1 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize] = (filter
        .icmp6_filt[(1 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << (1 as ::core::ffi::c_int & 31 as ::core::ffi::c_int)))
        as uint32_t;
    filter.icmp6_filt[(2 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize] = (filter
        .icmp6_filt[(2 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << (2 as ::core::ffi::c_int & 31 as ::core::ffi::c_int)))
        as uint32_t;
    filter.icmp6_filt[(3 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize] = (filter
        .icmp6_filt[(3 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << (3 as ::core::ffi::c_int & 31 as ::core::ffi::c_int)))
        as uint32_t;
    filter.icmp6_filt[(4 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize] = (filter
        .icmp6_filt[(4 as ::core::ffi::c_int >> 5 as ::core::ffi::c_int) as usize]
        as ::core::ffi::c_uint
        & !((1 as ::core::ffi::c_uint) << (4 as ::core::ffi::c_int & 31 as ::core::ffi::c_int)))
        as uint32_t;
    err = setsockopt(
        fd,
        C2Rust_Unnamed_1::IPPROTO_ICMPV6.0 as ::core::ffi::c_int,
        ICMP6_FILTER,
        &raw mut filter as *const ::core::ffi::c_void,
        ::core::mem::size_of::<icmp6_filter>() as socklen_t,
    );
    if err != 0 {
        close(fd);
        return ::core::ptr::null_mut::<PING>();
    }
    err = setsockopt(
        fd,
        C2Rust_Unnamed_0::IPPROTO_IPV6.0 as ::core::ffi::c_int,
        IPV6_RECVHOPLIMIT,
        &raw const on as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
    if err != 0 {
        close(fd);
        return ::core::ptr::null_mut::<PING>();
    }
    p = malloc(::core::mem::size_of::<PING>()) as *mut PING;
    if p.is_null() {
        close(fd);
        return ::core::ptr::null_mut::<PING>();
    }
    memset(
        p as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<PING>(),
    );
    (*p).ping_fd = fd;
    (*p).ping_count = DEFAULT_PING_COUNT as size_t;
    (*p).ping_interval = PING_DEFAULT_INTERVAL as size_t;
    (*p).ping_datalen = ::core::mem::size_of::<timeval>() as size_t;
    (*p).ping_ident = ident & 0xffff as ::core::ffi::c_int;
    (*p).ping_cktab_size = PING_CKTABSIZE;
    (*p).ping_start_time = current_timespec();
    return p;
}
unsafe extern "C" fn ping_xmit(mut p: *mut PING) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut buflen: ::core::ffi::c_int = 0;
    let mut icmp6: *mut icmp6_hdr = ::core::ptr::null_mut::<icmp6_hdr>();
    if _ping_setbuf(p, USE_IPV6 != 0) != 0 {
        return -1 as ::core::ffi::c_int;
    }
    buflen = (*p)
        .ping_datalen
        .wrapping_add(::core::mem::size_of::<icmp6_hdr>()) as ::core::ffi::c_int;
    let mut n: ::core::ffi::c_int = (*p)
        .ping_num_xmit
        .wrapping_rem((8 as ::core::ffi::c_int * (*p).ping_cktab_size) as size_t)
        as ::core::ffi::c_int;
    *(*p)
        .ping_cktab
        .offset((n >> 3 as ::core::ffi::c_int) as isize) = (*(*p)
        .ping_cktab
        .offset((n >> 3 as ::core::ffi::c_int) as isize)
        as ::core::ffi::c_int
        & !((1 as ::core::ffi::c_int) << (n & 0x7 as ::core::ffi::c_int)))
        as ::core::ffi::c_char;
    icmp6 = (*p).ping_buffer as *mut icmp6_hdr;
    (*icmp6).icmp6_type = ICMP6_ECHO_REQUEST as uint8_t;
    (*icmp6).icmp6_code = 0 as uint8_t;
    (*icmp6).icmp6_cksum = 0 as uint16_t;
    (*icmp6).icmp6_dataun.icmp6_un_data16[0usize] =
        __bswap_16((*p).ping_ident as __uint16_t) as uint16_t;
    (*icmp6).icmp6_dataun.icmp6_un_data16[1usize] =
        __bswap_16((*p).ping_num_xmit as __uint16_t) as uint16_t;
    i = sendto(
        (*p).ping_fd,
        (*p).ping_buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        buflen as size_t,
        0 as ::core::ffi::c_int,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut (*p).ping_dest.ping_sockaddr6 as *mut sockaddr,
        },
        ::core::mem::size_of::<sockaddr_in6>() as socklen_t,
    ) as ::core::ffi::c_int;
    if i < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    } else {
        (*p).ping_num_xmit = (*p).ping_num_xmit.wrapping_add(1);
        if i != buflen {
            printf(
                b"ping: wrote %s %d chars, ret=%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*p).ping_hostname,
                buflen,
                i,
            );
        }
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn my_echo_reply(
    mut p: *mut PING,
    mut icmp6: *mut icmp6_hdr,
) -> ::core::ffi::c_int {
    let mut orig_ip: *mut ip6_hdr = icmp6.offset(1 as ::core::ffi::c_int as isize) as *mut ip6_hdr;
    let mut orig_icmp: *mut icmp6_hdr =
        orig_ip.offset(1 as ::core::ffi::c_int as isize) as *mut icmp6_hdr;
    return (({
        let mut __a: *const in6_addr = &raw mut (*orig_ip).ip6_dst as *const in6_addr;
        let mut __b: *const in6_addr =
            &raw mut (*ping).ping_dest.ping_sockaddr6.sin6_addr as *const in6_addr;
        ((*__a).__in6_u.__u6_addr32[0usize] == (*__b).__in6_u.__u6_addr32[0usize]
            && (*__a).__in6_u.__u6_addr32[1usize] == (*__b).__in6_u.__u6_addr32[1usize]
            && (*__a).__in6_u.__u6_addr32[2usize] == (*__b).__in6_u.__u6_addr32[2usize]
            && (*__a).__in6_u.__u6_addr32[3usize] == (*__b).__in6_u.__u6_addr32[3usize])
            as ::core::ffi::c_int
    }) != 0
        && (*orig_ip).ip6_ctlun.ip6_un1.ip6_un1_nxt as ::core::ffi::c_int
            == C2Rust_Unnamed_1::IPPROTO_ICMPV6.0 as ::core::ffi::c_int
        && (*orig_icmp).icmp6_type as ::core::ffi::c_int == ICMP6_ECHO_REQUEST
        && (*orig_icmp).icmp6_dataun.icmp6_un_data16[0usize] as ::core::ffi::c_int
            == __bswap_16((*p).ping_ident as __uint16_t) as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn ping_recv(mut p: *mut PING) -> ::core::ffi::c_int {
    let mut dupflag: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut hops: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut msg: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut iov: iovec = iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    };
    let mut icmp6: *mut icmp6_hdr = ::core::ptr::null_mut::<icmp6_hdr>();
    let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
    let mut cmsg_data: [::core::ffi::c_char; 1024] = [0; 1024];
    iov.iov_base = (*p).ping_buffer as *mut ::core::ffi::c_void;
    iov.iov_len = if true {
        (*p).ping_datalen
            .wrapping_add(::core::mem::size_of::<icmp6_hdr>())
    } else {
        (MAXIPLEN as size_t)
            .wrapping_add((*p).ping_datalen)
            .wrapping_add(ICMP_TSLEN)
    };
    msg.msg_name = &raw mut (*p).ping_from.ping_sockaddr6 as *mut ::core::ffi::c_void;
    msg.msg_namelen = ::core::mem::size_of::<sockaddr_in6>() as socklen_t;
    msg.msg_iov = &raw mut iov;
    msg.msg_iovlen = 1 as size_t;
    msg.msg_control = &raw mut cmsg_data as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    msg.msg_controllen = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t;
    msg.msg_flags = 0 as ::core::ffi::c_int;
    n = recvmsg((*p).ping_fd, &raw mut msg, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if n < 0 as ::core::ffi::c_int {
        return -1 as ::core::ffi::c_int;
    }
    cmsg = if msg.msg_controllen >= ::core::mem::size_of::<cmsghdr>() {
        msg.msg_control as *mut cmsghdr
    } else {
        ::core::ptr::null_mut::<cmsghdr>()
    };
    while !cmsg.is_null() {
        if (*cmsg).cmsg_level == C2Rust_Unnamed_0::IPPROTO_IPV6.0 as ::core::ffi::c_int
            && (*cmsg).cmsg_type == IPV6_HOPLIMIT
        {
            hops = *(&raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar
                as *mut ::core::ffi::c_int);
            break;
        } else {
            cmsg = __cmsg_nxthdr(&raw mut msg, cmsg);
        }
    }
    icmp6 = (*p).ping_buffer as *mut icmp6_hdr;
    if (*icmp6).icmp6_type as ::core::ffi::c_int == ICMP6_ECHO_REPLY {
        if __bswap_16((*icmp6).icmp6_dataun.icmp6_un_data16[0usize]) as ::core::ffi::c_int
            != (*p).ping_ident
        {
            return -1 as ::core::ffi::c_int;
        }
        if *(*p).ping_cktab.offset(
            (__bswap_16((*icmp6).icmp6_dataun.icmp6_un_data16[1usize]) as ::core::ffi::c_int
                % (8 as ::core::ffi::c_int * (*p).ping_cktab_size)
                >> 3 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
            & (1 as ::core::ffi::c_int)
                << (__bswap_16((*icmp6).icmp6_dataun.icmp6_un_data16[1usize]) as ::core::ffi::c_int
                    % (8 as ::core::ffi::c_int * (*p).ping_cktab_size)
                    & 0x7 as ::core::ffi::c_int)
            != 0
        {
            (*p).ping_num_rept = (*p).ping_num_rept.wrapping_add(1);
            dupflag = 1 as ::core::ffi::c_int;
        } else {
            let mut n_0: ::core::ffi::c_int =
                __bswap_16((*icmp6).icmp6_dataun.icmp6_un_data16[1usize]) as ::core::ffi::c_int
                    % (8 as ::core::ffi::c_int * (*p).ping_cktab_size);
            *(*p)
                .ping_cktab
                .offset((n_0 >> 3 as ::core::ffi::c_int) as isize) = (*(*p)
                .ping_cktab
                .offset((n_0 >> 3 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << (n_0 & 0x7 as ::core::ffi::c_int))
                as ::core::ffi::c_char;
            (*p).ping_num_recv = (*p).ping_num_recv.wrapping_add(1);
            dupflag = 0 as ::core::ffi::c_int;
        }
        print_echo(
            dupflag,
            hops,
            (*p).ping_closure as *mut ping_stat,
            &raw mut (*p).ping_dest.ping_sockaddr6,
            &raw mut (*p).ping_from.ping_sockaddr6,
            icmp6,
            n,
        );
    } else {
        if my_echo_reply(p, icmp6) == 0 {
            return -1 as ::core::ffi::c_int;
        }
        print_icmp_error(&raw mut (*p).ping_from.ping_sockaddr6, icmp6, n);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn ping_set_dest(
    mut ping_0: *mut PING,
    mut host: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0;
    let mut result: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
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
    let mut rhost: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    rhost = ::core::ptr::null_mut::<::core::ffi::c_char>();
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>(),
    );
    hints.ai_family = AF_INET6;
    hints.ai_flags = AI_CANONNAME;
    hints.ai_flags |= AI_IDN;
    hints.ai_flags |= AI_CANONIDN;
    err = getaddrinfo(
        host,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut hints,
        &raw mut result,
    );
    if err != 0 {
        free(rhost as *mut ::core::ffi::c_void);
        return 1 as ::core::ffi::c_int;
    }
    memcpy(
        &raw mut (*ping_0).ping_dest.ping_sockaddr6 as *mut ::core::ffi::c_void,
        (*result).ai_addr as *const ::core::ffi::c_void,
        (*result).ai_addrlen as size_t,
    );
    if !(*result).ai_canonname.is_null() {
        (*ping_0).ping_hostname = strdup((*result).ai_canonname);
    } else {
        (*ping_0).ping_hostname = strdup(host);
    }
    freeaddrinfo(result);
    if (*ping_0).ping_hostname.is_null() {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn c2rust_run_static_initializers() {
    data_length = PING_DATALEN;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];
