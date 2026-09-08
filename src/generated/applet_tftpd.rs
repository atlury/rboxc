// Generated from pinned GNU tftpd 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 3de7d6ffd21efaeeb95be236e82957e29e0fea072a0b2d03ccdefea2d506f859
/*
  Copyright (C) 1995-2026 Free Software Foundation, Inc.

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
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn alarm(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getuid() -> __uid_t;
    fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
    fn setgid(__gid: __gid_t) -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    fn chroot(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tftpd_rpl_ioctl"]
    fn rpl_ioctl(fd: ::core::ffi::c_int, request: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
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
    fn recvfrom(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn openlog(
        __ident: *const ::core::ffi::c_char,
        __option: ::core::ffi::c_int,
        __facility: ::core::ffi::c_int,
    );
    fn syslog(__pri: ::core::ffi::c_int, __fmt: *const ::core::ffi::c_char, ...);
    fn getgrnam(__name: *const ::core::ffi::c_char) -> *mut group;
    fn getpwnam(__name: *const ::core::ffi::c_char) -> *mut passwd;
    #[link_name = "rboxc_tftpd_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tftpd_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_tftpd_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_tftpd_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tftpd_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_tftpd_default_program_authors"]
    static mut default_program_authors: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_tftpd_peer"]
    static mut peer: ::core::ffi::c_int;
    #[link_name = "rboxc_tftpd_chrootdir"]
    static mut chrootdir: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tftpd_group"]
    static mut group: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tftpd_user"]
    static mut user: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tftpd_buf"]
    static mut buf: [::core::ffi::c_char; 516];
    #[link_name = "rboxc_tftpd_from"]
    static mut from: sockaddr_storage;
    #[link_name = "rboxc_tftpd_fromlen"]
    static mut fromlen: socklen_t;
    #[link_name = "rboxc_tftpd_tftp"]
    fn tftp(_: *mut tftphdr, _: ::core::ffi::c_int);
    #[link_name = "rboxc_tftpd_dirs"]
    static mut dirs: [dirlist; 21];
    #[link_name = "rboxc_tftpd_nak"]
    fn nak(_: ::core::ffi::c_int);
    #[link_name = "rboxc_tftpd_argp"]
    static mut argp: argp;
}
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type size_t = usize;
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
#[repr(C, packed)]
pub struct tftphdr {
    pub th_opcode: ::core::ffi::c_short,
    pub th_u1: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union C2Rust_Unnamed_0 {
    pub tu_padding: [::core::ffi::c_char; 3],
    pub th_u2: C2Rust_Unnamed_1,
    pub tu_stuff: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct C2Rust_Unnamed_1 {
    pub th_u3: C2Rust_Unnamed_2,
    pub tu_data: [::core::ffi::c_char; 0],
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub union C2Rust_Unnamed_2 {
    pub tu_block: ::core::ffi::c_ushort,
    pub tu_code: ::core::ffi::c_short,
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
pub struct group {
    pub gr_name: *mut ::core::ffi::c_char,
    pub gr_passwd: *mut ::core::ffi::c_char,
    pub gr_gid: __gid_t,
    pub gr_mem: *mut *mut ::core::ffi::c_char,
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
pub struct dirlist {
    pub name: *mut ::core::ffi::c_char,
    pub len: ::core::ffi::c_int,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const FIONBIO: ::core::ffi::c_int = 0x5421 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const RRQ: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const WRQ: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const EACCESS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOUSER: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LOG_ERR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const LOG_FTP: ::core::ffi::c_int = (11 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const LOG_PID: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const LOG_NDELAY: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const DEFAULT_USER: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"nobody\0") };
pub const MAXDIRS: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_tftpd(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    let mut tp: *mut tftphdr = ::core::ptr::null_mut::<tftphdr>();
    let mut on: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut sin: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    user = xstrdup(DEFAULT_USER.as_ptr());
    set_program_name(*argv.offset(0isize));
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"tftpd\0".as_ptr() as *const ::core::ffi::c_char,
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
    openlog(
        b"tftpd\0".as_ptr() as *const ::core::ffi::c_char,
        LOG_NDELAY | LOG_PID,
        LOG_FTP,
    );
    if index < argc {
        let mut dirp: *mut dirlist = ::core::ptr::null_mut::<dirlist>();
        dirp = &raw mut dirs as *mut dirlist as *mut dirlist;
        while index < argc && dirp < (&raw mut dirs as *mut dirlist).offset(MAXDIRS as isize) {
            if *(*argv.offset(index as isize)).offset(0isize) as ::core::ffi::c_int
                == '/' as ::core::ffi::c_int
            {
                (*dirp).name = *argv.offset(index as isize);
                (*dirp).len = strlen((*dirp).name) as ::core::ffi::c_int;
                dirp = dirp.offset(1);
            }
            index += 1;
        }
    }
    on = 1 as ::core::ffi::c_int;
    if rpl_ioctl(0 as ::core::ffi::c_int, FIONBIO, &raw mut on) < 0 as ::core::ffi::c_int {
        syslog(
            LOG_ERR,
            b"ioctl(FIONBIO): %m\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    fromlen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    n = recvfrom(
        0 as ::core::ffi::c_int,
        &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 516]>(),
        0 as ::core::ffi::c_int,
        __SOCKADDR_ARG {
            __sockaddr__: &raw mut from as *mut sockaddr,
        },
        &raw mut fromlen,
    ) as ::core::ffi::c_int;
    if n < 0 as ::core::ffi::c_int {
        syslog(
            LOG_ERR,
            b"recvfrom: %m\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    let mut pid: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut j: socklen_t = 0;
    i = 1 as ::core::ffi::c_int;
    while i < 20 as ::core::ffi::c_int {
        pid = fork() as ::core::ffi::c_int;
        if pid >= 0 as ::core::ffi::c_int {
            break;
        }
        sleep(i as ::core::ffi::c_uint);
        j = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
        i = recvfrom(
            0 as ::core::ffi::c_int,
            &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 516]>(),
            0 as ::core::ffi::c_int,
            __SOCKADDR_ARG {
                __sockaddr__: &raw mut from as *mut sockaddr,
            },
            &raw mut j,
        ) as ::core::ffi::c_int;
        if i > 0 as ::core::ffi::c_int {
            n = i;
            fromlen = j;
        }
        i += 1;
    }
    if pid < 0 as ::core::ffi::c_int {
        syslog(
            LOG_ERR,
            b"fork: %m\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    } else if pid != 0 as ::core::ffi::c_int {
        exit(EXIT_SUCCESS);
    }
    alarm(0 as ::core::ffi::c_uint);
    close(0 as ::core::ffi::c_int);
    close(1 as ::core::ffi::c_int);
    peer = socket(
        from.ss_family as ::core::ffi::c_int,
        __socket_type::SOCK_DGRAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if peer < 0 as ::core::ffi::c_int {
        syslog(
            LOG_ERR,
            b"socket: %m\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    memset(
        &raw mut sin as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>(),
    );
    sin.ss_family = from.ss_family;
    if bind(
        peer,
        __CONST_SOCKADDR_ARG {
            __sockaddr__: &raw mut sin as *mut sockaddr,
        },
        fromlen,
    ) < 0 as ::core::ffi::c_int
    {
        syslog(
            LOG_ERR,
            b"bind: %m\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(EXIT_FAILURE);
    }
    if !chrootdir.is_null() && *chrootdir as ::core::ffi::c_int != 0 {
        let mut pwd: *mut passwd = ::core::ptr::null_mut::<passwd>();
        let mut grp: *mut group = ::core::ptr::null_mut::<group>();
        if getuid() == 0 {
            pwd = getpwnam(user);
            if pwd.is_null() {
                syslog(
                    LOG_ERR,
                    b"getpwnam('%s'): %m\0".as_ptr() as *const ::core::ffi::c_char,
                    user,
                );
                nak(ENOUSER);
                exit(EXIT_FAILURE);
            }
            if !group.is_null() && *group as ::core::ffi::c_int != 0 {
                grp = getgrnam(group);
                if grp.is_null() {
                    syslog(
                        LOG_ERR,
                        b"getgrnam('%s'): %m\0".as_ptr() as *const ::core::ffi::c_char,
                        group,
                    );
                    nak(ENOUSER);
                    exit(EXIT_FAILURE);
                }
            }
        }
        if chroot(chrootdir) != 0 || chdir(b"/\0".as_ptr() as *const ::core::ffi::c_char) != 0 {
            syslog(
                LOG_ERR,
                b"chroot('%s'): %m\0".as_ptr() as *const ::core::ffi::c_char,
                chrootdir,
            );
            nak(EACCESS);
            exit(EXIT_FAILURE);
        }
        if !pwd.is_null() {
            if !grp.is_null() {
                if setgid((*grp).gr_gid) != 0 {
                    syslog(
                        LOG_ERR,
                        b"setgid: %m\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    nak(ENOUSER);
                    exit(EXIT_FAILURE);
                }
            } else if setgid((*pwd).pw_gid) != 0 {
                syslog(
                    LOG_ERR,
                    b"setgid: %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
                nak(ENOUSER);
                exit(EXIT_FAILURE);
            }
            if setuid((*pwd).pw_uid) != 0 {
                syslog(
                    LOG_ERR,
                    b"setuid: %m\0".as_ptr() as *const ::core::ffi::c_char,
                );
                nak(ENOUSER);
                exit(EXIT_FAILURE);
            }
        }
    }
    tp = &raw mut buf as *mut ::core::ffi::c_char as *mut tftphdr;
    (*tp).th_opcode = __bswap_16((*tp).th_opcode as __uint16_t) as ::core::ffi::c_short;
    if (*tp).th_opcode as ::core::ffi::c_int == RRQ || (*tp).th_opcode as ::core::ffi::c_int == WRQ
    {
        tftp(tp, n);
    }
    exit(EXIT_FAILURE);
}
