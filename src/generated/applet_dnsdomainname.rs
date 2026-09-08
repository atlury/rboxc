// Generated from pinned GNU dnsdomainname 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 12142f25967227c23e93fb988bdf11765d0edb0924e3d579f7fa0b76f311e2d8
/*
  Copyright (C) 2012-2026 Free Software Foundation, Inc.

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
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    #[link_name = "rboxc_dnsdomainname_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_dnsdomainname_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn free(_: *mut ::core::ffi::c_void);
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_dnsdomainname_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_dnsdomainname_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_dnsdomainname_xgethostname"]
    fn xgethostname() -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AI_CANONNAME: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
#[export_name = "rboxc_dnsdomainname_doc"]
pub static mut doc: [::core::ffi::c_char; 246] = unsafe {
    ::core::mem::transmute::<
        [u8; 246],
        [::core::ffi::c_char; 246],
    >(
        *b"Show domain part of the system's fully qualified host name.\n\nThe tool uses gethostname to get the host name of the system\nand getaddrinfo to resolve it into a canonical name.  The part\nafter the first period ('.') of the canonical name is shown.\0",
    )
};
#[export_name = "rboxc_dnsdomainname_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 2] = [
    b"Simon Josefsson\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut argp: argp = unsafe {
    argp {
        options: ::core::ptr::null::<argp_option>(),
        parser: None,
        args_doc: ::core::ptr::null::<::core::ffi::c_char>(),
        doc: &raw const doc as *const ::core::ffi::c_char,
        children: ::core::ptr::null::<argp_child>(),
        help_filter: None,
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
#[export_name = "rboxc_dnsdomainname_dnsdomainname"]
pub unsafe extern "C" fn dnsdomainname() {
    let mut host_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    let mut dn: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut rc: ::core::ffi::c_int = 0;
    host_name = xgethostname();
    if host_name.is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot determine host name\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"cannot determine host name\0".as_ptr() as *const ::core::ffi::c_char,
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
    hints.ai_flags = AI_CANONNAME;
    rc = getaddrinfo(
        host_name,
        ::core::ptr::null::<::core::ffi::c_char>(),
        &raw mut hints,
        &raw mut res,
    );
    if rc != 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
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
                    0 as ::core::ffi::c_int,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    gai_strerror(rc),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    dn = strchr((*res).ai_canonname, '.' as ::core::ffi::c_int);
    if !dn.is_null() {
        puts(dn.offset(1 as ::core::ffi::c_int as isize));
    }
    free(host_name as *mut ::core::ffi::c_void);
    freeaddrinfo(res);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_dnsdomainname(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    set_program_name(*argv.offset(0isize));
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"dnsdomainname\0".as_ptr() as *const ::core::ffi::c_char,
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
    dnsdomainname();
    exit(EXIT_SUCCESS);
}
