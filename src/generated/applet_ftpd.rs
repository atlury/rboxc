// Generated from pinned GNU ftpd 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 8e34ad26ee07e8d2d4c9f1411590055ae596174dabd0c563c0fa6b0f43aebb50
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
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn getpid() -> __pid_t;
    #[link_name = "rboxc_ftpd_rpl_fcntl"]
    fn rpl_fcntl(fd: ::core::ffi::c_int, action: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn tzset();
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
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn freopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_ftpd_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_ftpd_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_ftpd_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_ftpd_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_ftpd_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_ftpd_localhost"]
    fn localhost() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_ftpd_default_program_authors"]
    static mut default_program_authors: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_ftpd_display_file"]
    fn display_file(
        name: *const ::core::ffi::c_char,
        code: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ftpd_perror_reply"]
    fn perror_reply(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char);
    #[link_name = "rboxc_ftpd_reply"]
    fn reply(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_ftpd_his_addr"]
    static mut his_addr: sockaddr_storage;
    #[link_name = "rboxc_ftpd_his_addrlen"]
    static mut his_addrlen: socklen_t;
    #[link_name = "rboxc_ftpd_no_version"]
    static mut no_version: ::core::ffi::c_int;
    #[link_name = "rboxc_ftpd_hostname"]
    static mut hostname: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_ftpd_server_mode"]
    fn server_mode(
        pidfile: *const ::core::ffi::c_char,
        phis_addr: *mut sockaddr,
        phis_addrlen: *mut socklen_t,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ftpd_cred"]
    static mut cred: credentials;
    #[link_name = "rboxc_ftpd_ctrl_addr"]
    static mut ctrl_addr: sockaddr_storage;
    #[link_name = "rboxc_ftpd_ctrl_addrlen"]
    static mut ctrl_addrlen: socklen_t;
    #[link_name = "rboxc_ftpd_daemon_mode"]
    static mut daemon_mode: ::core::ffi::c_int;
    #[link_name = "rboxc_ftpd_pid_file"]
    static mut pid_file: *const ::core::ffi::c_char;
    #[link_name = "rboxc_ftpd_dolog"]
    fn dolog(_: *mut sockaddr, _: socklen_t, _: *mut credentials);
    #[link_name = "rboxc_ftpd_lostconn"]
    fn lostconn(_: ::core::ffi::c_int);
    #[link_name = "rboxc_ftpd_myoob"]
    fn myoob(_: ::core::ffi::c_int);
    #[link_name = "rboxc_ftpd_sigquit"]
    fn sigquit(_: ::core::ffi::c_int);
    #[link_name = "rboxc_ftpd_argp"]
    static mut argp: argp;
    #[link_name = "rboxc_ftpd_rboxc_native_ftpd_loop"]
    fn rboxc_native_ftpd_loop();
}
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __socklen_t = ::core::ffi::c_uint;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
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
pub type error_t = ::core::ffi::c_int;
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
pub struct credentials {
    pub name: *mut ::core::ffi::c_char,
    pub homedir: *mut ::core::ffi::c_char,
    pub rootdir: *mut ::core::ffi::c_char,
    pub shell: *mut ::core::ffi::c_char,
    pub remotehost: *mut ::core::ffi::c_char,
    pub passwd: *mut ::core::ffi::c_char,
    pub pass: *mut ::core::ffi::c_char,
    pub message: *mut ::core::ffi::c_char,
    pub uid: uid_t,
    pub gid: gid_t,
    pub guest: ::core::ffi::c_int,
    pub dochroot: ::core::ffi::c_int,
    pub logged_in: ::core::ffi::c_int,
    pub delayed_reject: ::core::ffi::c_int,
    pub expired: ::core::ffi::c_int,
    pub auth_type: ::core::ffi::c_int,
}
pub const PATH_DEVNULL: [::core::ffi::c_char; 10] = _PATH_DEVNULL;
pub const PATH_NOLOGIN: [::core::ffi::c_char; 13] = _PATH_NOLOGIN;
pub const _PATH_DEVNULL: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"/dev/null\0") };
pub const _PATH_NOLOGIN: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"/etc/nologin\0") };
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGURG: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __F_SETOWN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const F_SETOWN: ::core::ffi::c_int = __F_SETOWN;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_KEEPALIVE: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const SO_OOBINLINE: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const IP_TOS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IPTOS_LOWDELAY: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_WARNING: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const LOG_FTP: ::core::ffi::c_int = (11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_NDELAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
unsafe extern "C" fn rboxc_ftpd_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut envp: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    set_program_name(*argv.offset(0isize));
    tzset();
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"ftpd\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut default_program_authors as *mut *const ::core::ffi::c_char,
    );
    rpl_argp_parse(
        &raw mut argp,
        argc,
        argv,
        0 as ::core::ffi::c_uint,
        &raw mut index,
        NULL,
    );
    if argc - index != 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"surplus arguments; try `%s --help' for more info\0".as_ptr()
                    as *const ::core::ffi::c_char,
                program_name,
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
                    b"surplus arguments; try `%s --help' for more info\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    program_name,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    openlog(
        b"ftpd\0".as_ptr() as *const ::core::ffi::c_char,
        LOG_PID | LOG_NDELAY,
        LOG_FTP,
    );
    freopen(
        PATH_DEVNULL.as_ptr(),
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
        stderr,
    );
    if daemon_mode != 0 {
        his_addrlen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
        if server_mode(
            pid_file,
            &raw mut his_addr as *mut sockaddr,
            &raw mut his_addrlen,
            argv,
        ) < 0 as ::core::ffi::c_int
        {
            exit(EXIT_FAILURE);
        }
    } else {
        his_addrlen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
        if getpeername(
            STDIN_FILENO,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut his_addr as *mut sockaddr,
            },
            &raw mut his_addrlen,
        ) < 0 as ::core::ffi::c_int
        {
            syslog(
                LOG_ERR,
                b"getpeername (%s): %m\0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
            );
            exit(EXIT_FAILURE);
        }
    }
    signal(
        SIGHUP,
        Some(sigquit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGINT,
        Some(sigquit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGQUIT,
        Some(sigquit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGTERM,
        Some(sigquit as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGPIPE,
        Some(lostconn as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGCHLD,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    if signal(
        SIGURG,
        Some(myoob as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    ) == ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        -1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        syslog(
            LOG_ERR,
            b"signal: %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    ctrl_addrlen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    if getsockname(
        STDIN_FILENO,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut ctrl_addr as *mut sockaddr,
        },
        &raw mut ctrl_addrlen,
    ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_ERR,
            b"getsockname (%s): %m\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
        );
        exit(EXIT_FAILURE);
    }
    if ctrl_addr.ss_family as ::core::ffi::c_int == AF_INET {
        let mut tos: ::core::ffi::c_int = IPTOS_LOWDELAY;
        if setsockopt(
            STDIN_FILENO,
            C2Rust_Unnamed_0::IPPROTO_IP.0 as ::core::ffi::c_int,
            IP_TOS,
            &raw mut tos as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            syslog(
                LOG_WARNING,
                b"setsockopt (IP_TOS): %m\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    let mut on: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if setsockopt(
        STDIN_FILENO,
        SOL_SOCKET,
        SO_OOBINLINE,
        &raw mut on as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_ERR,
            b"setsockopt: %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut keepalive: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if setsockopt(
        STDIN_FILENO,
        SOL_SOCKET,
        SO_KEEPALIVE,
        &raw mut keepalive as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_WARNING,
            b"setsockopt (SO_KEEPALIVE): %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if rpl_fcntl(STDIN_FILENO, F_SETOWN, getpid()) == -1 as ::core::ffi::c_int {
        syslog(
            LOG_ERR,
            b"fcntl F_SETOWN: %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    dolog(
        &raw mut his_addr as *mut sockaddr,
        his_addrlen,
        &raw mut cred,
    );
    if display_file(PATH_NOLOGIN.as_ptr(), 530 as ::core::ffi::c_int) == 0 as ::core::ffi::c_int {
        reply(
            530 as ::core::ffi::c_int,
            b"System not available.\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_SUCCESS);
    }
    hostname = localhost();
    if hostname.is_null() {
        perror_reply(
            550 as ::core::ffi::c_int,
            b"Local resource failure: malloc\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    display_file(PATH_FTPWELCOME.as_ptr(), 220 as ::core::ffi::c_int);
    if no_version == 0 {
        reply(
            220 as ::core::ffi::c_int,
            b"%s FTP server (%s %s) ready.\0".as_ptr() as *const ::core::ffi::c_char,
            hostname,
            PACKAGE_NAME.as_ptr(),
            PACKAGE_VERSION.as_ptr(),
        );
    } else {
        reply(
            220 as ::core::ffi::c_int,
            b"%s FTP server ready.\0".as_ptr() as *const ::core::ffi::c_char,
            hostname,
        );
    }
    rboxc_native_ftpd_loop();
    panic!("Reached end of non-void function without returning");
}
pub const PATH_FTPWELCOME: [::core::ffi::c_char; 50] = unsafe {
    ::core::mem::transmute::<[u8; 50], [::core::ffi::c_char; 50]>(
        *b"/root/rboxc/build/oracle/inetutils/etc/ftpwelcome\0",
    )
};
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"GNU inetutils\0") };
pub const PACKAGE_VERSION: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"2.8\0") };

extern "C" { #[link_name = "environ"] static mut RBOXC_FTPD_ENVIRON: *mut *mut ::core::ffi::c_char; }
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_ftpd(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_ftpd_main_inner(argc, argv, RBOXC_FTPD_ENVIRON)
}
