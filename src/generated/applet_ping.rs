// Generated from pinned GNU ping 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 38545cc8ae73463a6ef61b0832fe9016b727bed00bcf7e58db57090722858e20
/*
  Copyright (C) 1998-2026 Free Software Foundation, Inc.

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
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn getpid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn free(_: *mut ::core::ffi::c_void);
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn setvbuf(
        __stream: *mut FILE,
        __buf: *mut ::core::ffi::c_char,
        __modes: ::core::ffi::c_int,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_ping_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_ping_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_ping_argp_error"]
    fn argp_error(__state: *const argp_state, __fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_ping_timespec_add"]
    fn timespec_add(_: timespec, _: timespec) -> timespec;
    #[link_name = "rboxc_ping_timespec_sub"]
    fn timespec_sub(_: timespec, _: timespec) -> timespec;
    #[link_name = "rboxc_ping_current_timespec"]
    fn current_timespec() -> timespec;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_ping_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_ping_ping_cvt_number"]
    fn ping_cvt_number(
        rpl_optarg: *const ::core::ffi::c_char,
        maxval: size_t,
        allow_zero: ::core::ffi::c_int,
    ) -> size_t;
    #[link_name = "rboxc_ping_init_data_buffer"]
    fn init_data_buffer(pat: *mut ::core::ffi::c_uchar, len: size_t);
    #[link_name = "rboxc_ping_decode_pattern"]
    fn decode_pattern(
        text: *const ::core::ffi::c_char,
        pattern_len_0: *mut ::core::ffi::c_int,
        pattern_data: *mut ::core::ffi::c_uchar,
    );
    #[link_name = "rboxc_ping_ping_set_data"]
    fn ping_set_data(
        p: *mut PING,
        data: *mut ::core::ffi::c_void,
        off: size_t,
        len: size_t,
        use_ipv6: bool,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping_ping_set_count"]
    fn ping_set_count(ping_0: *mut PING, count_0: size_t);
    #[link_name = "rboxc_ping_ping_set_sockopt"]
    fn ping_set_sockopt(
        ping_0: *mut PING,
        opt: ::core::ffi::c_int,
        val: *mut ::core::ffi::c_void,
        valsize: ::core::ffi::c_int,
    );
    #[link_name = "rboxc_ping_ping_set_interval"]
    fn ping_set_interval(ping_0: *mut PING, interval_0: size_t);
    #[link_name = "rboxc_ping_ping_unset_data"]
    fn ping_unset_data(p: *mut PING);
    #[link_name = "rboxc_ping_ping_timeout_p"]
    fn ping_timeout_p(start_time: *mut timespec, timeout_0: ::core::ffi::c_int) -> bool;
    #[link_name = "rboxc_ping_ping_init"]
    fn ping_init(r#type: ::core::ffi::c_int, ident: ::core::ffi::c_int) -> *mut PING;
    #[link_name = "rboxc_ping_ping_reset"]
    fn ping_reset(p: *mut PING);
    #[link_name = "rboxc_ping_ping_recv"]
    fn ping_recv(p: *mut PING) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping_ping_xmit"]
    fn ping_xmit(p: *mut PING) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_ping_ping_echo"]
    fn ping_echo(hostname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping_ping_timestamp"]
    fn ping_timestamp(hostname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ping_ping_address"]
    fn ping_address(hostname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type socklen_t = __socklen_t;
pub type sa_family_t = ::core::ffi::c_ushort;
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
pub struct icmp6_hdr {
    pub icmp6_type: uint8_t,
    pub icmp6_code: uint8_t,
    pub icmp6_cksum: uint16_t,
    pub icmp6_dataun: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub icmp6_un_data32: [uint32_t; 1],
    pub icmp6_un_data16: [uint16_t; 2],
    pub icmp6_un_data8: [uint8_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct icmp_header {
    pub icmp_type: ::core::ffi::c_uchar,
    pub icmp_code: ::core::ffi::c_uchar,
    pub icmp_cksum: ::core::ffi::c_ushort,
    pub icmp_hun: C2Rust_Unnamed_3,
    pub icmp_dun: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
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
pub union C2Rust_Unnamed_3 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const ARG_ECHO: Self = Self(256);
    pub const ARG_ADDRESS: Self = Self(257);
    pub const ARG_TIMESTAMP: Self = Self(258);
    pub const ARG_ROUTERDISCOVERY: Self = Self(259);
    pub const ARG_TTL: Self = Self(260);
    pub const ARG_IPTIMESTAMP: Self = Self(261);
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SO_DEBUG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_DONTROUTE: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const SO_BROADCAST: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const IP_TOS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IP_TTL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const _IOLBF: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const OPTION_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_KEY_NO_ARGS: ::core::ffi::c_int = 16777218;
#[inline]
unsafe extern "C" fn timespec_sign(mut a: timespec) -> ::core::ffi::c_int {
    return (a.tv_sec | a.tv_nsec > 0 as ::core::ffi::c_long) as ::core::ffi::c_int
        - (a.tv_sec | a.tv_nsec < 0 as ::core::ffi::c_long) as ::core::ffi::c_int;
}
pub const ICMP_ECHO: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAXIPLEN: ::core::ffi::c_int = 60 as ::core::ffi::c_int;
pub const MAXICMPLEN: ::core::ffi::c_int = 76 as ::core::ffi::c_int;
pub const ICMP_MINLEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const MAXWAIT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const MAXPATTERN: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const OPT_FLOOD: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPT_INTERVAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPT_NUMERIC: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPT_QUIET: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPT_RROUTE: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const OPT_VERBOSE: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const OPT_IPTIMESTAMP: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const SOPT_TSONLY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SOPT_TSADDR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const DEFAULT_PING_COUNT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PING_HEADER_LEN: usize = if USE_IPV6 != 0 {
    ::core::mem::size_of::<icmp6_hdr>()
} else {
    ICMP_MINLEN as usize
};
pub const PING_DATALEN: usize = 64usize.wrapping_sub(PING_HEADER_LEN);
pub const PING_PRECISION: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const PING_MIN_USER_INTERVAL: ::core::ffi::c_int =
    200000 as ::core::ffi::c_int / PING_PRECISION;
pub const USE_IPV6: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PING_MAX_DATALEN: ::core::ffi::c_int =
    65535 as ::core::ffi::c_int - MAXIPLEN - MAXICMPLEN;
#[export_name = "rboxc_ping_ping"]
pub static mut ping: *mut PING = ::core::ptr::null_mut::<PING>();
#[export_name = "rboxc_ping_is_root"]
pub static mut is_root: bool = r#false != 0;
#[export_name = "rboxc_ping_data_buffer"]
pub static mut data_buffer: *mut ::core::ffi::c_uchar =
    ::core::ptr::null_mut::<::core::ffi::c_uchar>();
#[export_name = "rboxc_ping_patptr"]
pub static mut patptr: *mut ::core::ffi::c_uchar = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
#[export_name = "rboxc_ping_pattern_len"]
pub static mut pattern_len: ::core::ffi::c_int = MAXPATTERN;
#[export_name = "rboxc_ping_socket_type"]
pub static mut socket_type: ::core::ffi::c_int = 0;
#[export_name = "rboxc_ping_count"]
pub static mut count: size_t = DEFAULT_PING_COUNT as size_t;
#[export_name = "rboxc_ping_interval"]
pub static mut interval: size_t = 0;
#[export_name = "rboxc_ping_data_length"]
pub static mut data_length: size_t = 0;
#[export_name = "rboxc_ping_options"]
pub static mut options: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_ping_suboptions"]
pub static mut suboptions: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_ping_preload"]
pub static mut preload: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
#[export_name = "rboxc_ping_tos"]
pub static mut tos: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_ping_ttl"]
pub static mut ttl: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_ping_timeout"]
pub static mut timeout: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_ping_linger"]
pub static mut linger: ::core::ffi::c_int = MAXWAIT;
#[export_name = "rboxc_ping_ping_type"]
pub static mut ping_type: Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
> = Some(ping_echo as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int);
#[export_name = "rboxc_ping_args_doc"]
pub static mut args_doc: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"HOST ...\0") };
#[export_name = "rboxc_ping_doc"]
pub static mut doc: [::core::ffi::c_char; 114] = unsafe {
    ::core::mem::transmute::<
        [u8; 114],
        [::core::ffi::c_char; 114],
    >(
        *b"Send ICMP ECHO_REQUEST packets to network hosts.\x0BOptions marked with (root only) are available only to superuser.\0",
    )
};
#[export_name = "rboxc_ping_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 2] = [
    b"Sergey Poznyakoff\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut argp_options: [argp_option; 27] = [
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Options controlling ICMP request types:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_1,
    },
    argp_option {
        name: b"address\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_ADDRESS.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"send ICMP_ADDRESS packets (root only)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"echo\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_ECHO.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"send ICMP_ECHO packets (default)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"mask\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_ADDRESS.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"same as --address\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_TIMESTAMP.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"send ICMP_TIMESTAMP packets\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"type\0".as_ptr() as *const ::core::ffi::c_char,
        key: 't' as ::core::ffi::c_int,
        arg: b"TYPE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"send TYPE packets\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"router\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_ROUTERDISCOVERY.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: b"send ICMP_ROUTERDISCOVERY packets (root only)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_1 + 1 as ::core::ffi::c_int,
    },
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
        arg: b"NUM\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set type of service (TOS) to NUM\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ttl\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_TTL.0 as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"specify N as time-to-live\0".as_ptr() as *const ::core::ffi::c_char,
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
        name: b"timeout\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'w' as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"stop after N seconds\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"linger\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'W' as ::core::ffi::c_int,
        arg: b"N\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"number of seconds to wait for response\0".as_ptr()
            as *const ::core::ffi::c_char,
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
        name: b"route\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'R' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"record route\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRP + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ip-timestamp\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_4::ARG_IPTIMESTAMP.0 as ::core::ffi::c_int,
        arg: b"FLAG\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"IP timestamp of type FLAG, which is one of \"tsonly\" and \"tsaddr\"\0"
            .as_ptr() as *const ::core::ffi::c_char,
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
pub const GRP_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const GRP_0: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const GRP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    let mut endptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    static mut pattern: [::core::ffi::c_uchar; 16] = [0; 16];
    let mut v: ::core::ffi::c_double = 0.;
    's_209: {
        match key {
            99 => {
                count = ping_cvt_number(arg, 0 as size_t, 1 as ::core::ffi::c_int);
                break 's_209;
            }
            100 => {
                socket_type |= SO_DEBUG;
                break 's_209;
            }
            105 => {
                v = strtod(arg, &raw mut endptr);
                if *endptr != 0 {
                    argp_error(
                        state,
                        b"invalid value (`%s' near `%s')\0".as_ptr() as *const ::core::ffi::c_char,
                        arg,
                        endptr,
                    );
                }
                options |= OPT_INTERVAL as ::core::ffi::c_uint;
                interval = (v * PING_PRECISION as ::core::ffi::c_double) as size_t;
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
                break 's_209;
            }
            114 => {
                socket_type |= SO_DONTROUTE;
                break 's_209;
            }
            115 => {
                data_length =
                    ping_cvt_number(arg, PING_MAX_DATALEN as size_t, 1 as ::core::ffi::c_int);
                break 's_209;
            }
            110 => {
                options |= OPT_NUMERIC as ::core::ffi::c_uint;
                break 's_209;
            }
            112 => {
                decode_pattern(
                    arg,
                    &raw mut pattern_len,
                    &raw mut pattern as *mut ::core::ffi::c_uchar,
                );
                patptr = &raw mut pattern as *mut ::core::ffi::c_uchar;
                break 's_209;
            }
            113 => {
                options |= OPT_QUIET as ::core::ffi::c_uint;
                break 's_209;
            }
            84 => {
                tos = ping_cvt_number(arg, 255 as size_t, 1 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_209;
            }
            119 => {
                timeout = ping_cvt_number(arg, INT_MAX as size_t, 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_209;
            }
            82 => {
                options |= OPT_RROUTE as ::core::ffi::c_uint;
                break 's_209;
            }
            87 => {
                linger = ping_cvt_number(arg, INT_MAX as size_t, 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_209;
            }
            118 => {
                options |= OPT_VERBOSE as ::core::ffi::c_uint;
                break 's_209;
            }
            108 => {
                preload = strtoul(arg, &raw mut endptr, 0 as ::core::ffi::c_int);
                if *endptr as ::core::ffi::c_int != 0 || preload > INT_MAX as ::core::ffi::c_ulong {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"invalid preload value (%s)\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"invalid preload value (%s)\0".as_ptr()
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
                break 's_209;
            }
            102 => {
                options |= OPT_FLOOD as ::core::ffi::c_uint;
                break 's_209;
            }
            116 => {
                ping_type = decode_type(arg);
                break 's_209;
            }
            256 => {
                ping_type = decode_type(b"echo\0".as_ptr() as *const ::core::ffi::c_char);
                break 's_209;
            }
            258 => {
                ping_type = decode_type(b"timestamp\0".as_ptr() as *const ::core::ffi::c_char);
                break 's_209;
            }
            257 => {
                ping_type = decode_type(b"address\0".as_ptr() as *const ::core::ffi::c_char);
                break 's_209;
            }
            259 => {
                ping_type = decode_type(b"router\0".as_ptr() as *const ::core::ffi::c_char);
                break 's_209;
            }
            260 => {
                ttl = ping_cvt_number(arg, 255 as size_t, 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int;
                break 's_209;
            }
            261 => {
                options |= OPT_IPTIMESTAMP as ::core::ffi::c_uint;
                suboptions |= decode_ip_timestamp(arg) as ::core::ffi::c_uint;
                break 's_209;
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
pub unsafe extern "C" fn single_binary_main_ping(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut one: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    set_program_name(*argv.offset(0isize));
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    if getuid() == 0 as __uid_t {
        is_root = r#true != 0;
    }
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"ping\0".as_ptr() as *const ::core::ffi::c_char,
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
    ping = ping_init(ICMP_ECHO, getpid());
    if ping.is_null() {
        exit(EXIT_FAILURE);
    }
    ping_set_sockopt(
        ping,
        SO_BROADCAST,
        &raw mut one as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as ::core::ffi::c_int,
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
    argv = argv.offset(index as isize);
    argc -= index;
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
    if ttl > 0 as ::core::ffi::c_int {
        if setsockopt(
            (*ping).ping_fd,
            C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
            IP_TTL,
            &raw mut ttl as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"setsockopt(IP_TTL)\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"setsockopt(IP_TTL)\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if tos >= 0 as ::core::ffi::c_int {
        if setsockopt(
            (*ping).ping_fd,
            C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
            IP_TOS,
            &raw mut tos as *const ::core::ffi::c_void,
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
    init_data_buffer(patptr, pattern_len as size_t);
    loop {
        let c2rust_fresh0 = argc;
        argc -= 1;
        if c2rust_fresh0 == 0 {
            break;
        }
        let c2rust_fresh1 = argv;
        argv = argv.offset(1);
        status |= Some(ping_type.expect("non-null function pointer"))
            .expect("non-null function pointer")(*c2rust_fresh1);
        ping_reset(ping);
    }
    free(ping as *mut ::core::ffi::c_void);
    free(data_buffer as *mut ::core::ffi::c_void);
    return status;
}
#[export_name = "rboxc_ping_decode_type"]
pub unsafe extern "C" fn decode_type(
    mut arg: *const ::core::ffi::c_char,
) -> Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int> {
    let mut ping_type_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
    > = None;
    if strcasecmp(arg, b"echo\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        ping_type_0 =
            Some(ping_echo as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int)
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>;
    } else if strcasecmp(arg, b"timestamp\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ping_type_0 = Some(
            ping_timestamp as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>;
    } else if strcasecmp(arg, b"address\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ping_type_0 = Some(
            ping_address as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>;
    } else if strcasecmp(arg, b"mask\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        ping_type_0 = Some(
            ping_address as unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_char) -> ::core::ffi::c_int>;
    } else {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"unsupported packet type: %s\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"unsupported packet type: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    arg,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return ping_type_0;
}
unsafe extern "C" fn decode_ip_timestamp(mut arg: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut sopt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strcasecmp(arg, b"tsonly\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        sopt = SOPT_TSONLY;
    } else if strcasecmp(arg, b"tsaddr\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        sopt = SOPT_TSADDR;
    } else {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"unsupported timestamp type: %s\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"unsupported timestamp type: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    arg,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return sopt;
}
#[export_name = "rboxc_ping_stop"]
pub static mut stop: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_ping_sig_int"]
pub unsafe extern "C" fn sig_int(mut signal_0: ::core::ffi::c_int) {
    ::core::ptr::write_volatile(&raw mut stop, 1 as ::core::ffi::c_int);
}
#[export_name = "rboxc_ping_ping_run"]
pub unsafe extern "C" fn ping_run(
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
    let mut i: size_t = 0;
    signal(
        SIGINT,
        Some(sig_int as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    fdmax = (*ping_0).ping_fd + 1 as ::core::ffi::c_int;
    i = 0 as size_t;
    while i < preload as size_t {
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
                intvl.tv_sec = linger as __time_t;
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
#[export_name = "rboxc_ping_ping_finish"]
pub unsafe extern "C" fn ping_finish() -> ::core::ffi::c_int {
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
            printf(
                b"-- somebody is printing forged packets!\0".as_ptr() as *const ::core::ffi::c_char
            );
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
