// Generated from pinned GNU ifconfig 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 2b44ccfd85279bfaeeb68c315b2dcf93a1d36659c23f7cf3d3542875492785cc
/* ifconfig.c -- network interface configuration utility
  Copyright (C) 2001-2026 Free Software Foundation, Inc.

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
    static mut stdout: *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn if_nametoindex(__ifname: *const ::core::ffi::c_char) -> ::core::ffi::c_uint;
    #[link_name = "rboxc_ifconfig_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_ifconfig_ifs"]
    static mut ifs: *mut ifconfig;
    #[link_name = "rboxc_ifconfig_nifs"]
    static mut nifs: ::core::ffi::c_int;
    #[link_name = "rboxc_ifconfig_list_mode"]
    static mut list_mode: ::core::ffi::c_int;
    #[link_name = "rboxc_ifconfig_parse_cmdline"]
    fn parse_cmdline(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_ifconfig_configure_if"]
    fn configure_if(sfd: ::core::ffi::c_int, ifp: *mut ifconfig) -> ::core::ffi::c_int;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub struct ifconfig {
    pub name: *mut ::core::ffi::c_char,
    pub valid: ::core::ffi::c_int,
    pub system: *mut system_ifconfig,
    pub format: *const ::core::ffi::c_char,
    pub af: sa_family_t,
    pub address: *mut ::core::ffi::c_char,
    pub netmask: *mut ::core::ffi::c_char,
    pub dstaddr: *mut ::core::ffi::c_char,
    pub brdaddr: *mut ::core::ffi::c_char,
    pub mtu: ::core::ffi::c_int,
    pub metric: ::core::ffi::c_int,
    pub setflags: ::core::ffi::c_int,
    pub clrflags: ::core::ffi::c_int,
    pub hwaddr: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct system_ifconfig {
    pub valid: ::core::ffi::c_int,
    pub txqlen: ::core::ffi::c_int,
}
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_ifconfig(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sfd: ::core::ffi::c_int = 0;
    let mut ifp: *mut ifconfig = ::core::ptr::null_mut::<ifconfig>();
    set_program_name(*argv.offset(0isize));
    parse_cmdline(argc, argv);
    sfd = socket(
        AF_INET,
        __socket_type::SOCK_STREAM.0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if sfd < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                b"socket error\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"socket error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        exit(EXIT_FAILURE);
    }
    ifp = ifs;
    while ifp < ifs.offset(nifs as isize) {
        if list_mode != 0 {
            if if_nametoindex((*ifp).name) != 0 {
                let c2rust_fresh0 = err;
                err += 1;
                if c2rust_fresh0 != 0 {
                    putchar(' ' as ::core::ffi::c_int);
                }
                printf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, (*ifp).name);
            }
        } else {
            err = configure_if(sfd, ifp);
            if err != 0 {
                break;
            }
        }
        ifp = ifp.offset(1);
    }
    if list_mode != 0 && err != 0 {
        putchar('\n' as ::core::ffi::c_int);
    }
    close(sfd);
    return err;
}
