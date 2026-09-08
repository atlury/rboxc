// Generated from pinned GNU telnet 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: 6e299cafcfa9b1ac940769dbb0a2ef81d7eee20b97863fecd30f31e8529b9c4f
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
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnet_autologin"]
    static mut autologin: ::core::ffi::c_int;
    #[link_name = "rboxc_telnet_rlogin"]
    static mut rlogin: cc_t;
    #[link_name = "rboxc_telnet_prompt"]
    static mut prompt: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnet_TerminalSaveState"]
    fn TerminalSaveState();
    #[link_name = "rboxc_telnet_tninit"]
    fn tninit();
    #[link_name = "rboxc_telnet_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_telnet_rpl_argp_parse"]
    fn rpl_argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_telnet_argp_program_bug_address"]
    static mut argp_program_bug_address: *const ::core::ffi::c_char;
    #[link_name = "rboxc_telnet_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_telnet_default_program_authors"]
    static mut default_program_authors: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_telnet_family"]
    static mut family: ::core::ffi::c_int;
    #[link_name = "rboxc_telnet_user"]
    static mut user: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnet_srcaddr"]
    static mut srcaddr: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_telnet_argp"]
    static mut argp: argp;
    #[link_name = "rboxc_telnet_rboxc_native_telnet_connect"]
    fn rboxc_native_telnet_connect(
        count: ::core::ffi::c_int,
        args: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_telnet_rboxc_native_telnet_loop"]
    fn rboxc_native_telnet_loop();
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
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
pub type cc_t = ::core::ffi::c_uchar;
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
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const _POSIX_VDISABLE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_telnet(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut index: ::core::ffi::c_int = 0;
    set_program_name(*argv.offset(0isize));
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    tninit();
    TerminalSaveState();
    prompt = strrchr(*argv.offset(0isize), '/' as ::core::ffi::c_int);
    if !prompt.is_null() {
        prompt = prompt.offset(1);
    } else {
        prompt = *argv.offset(0isize);
    }
    user = ::core::ptr::null_mut::<::core::ffi::c_char>();
    rlogin = (if strncmp(
        prompt,
        b"rlog\0".as_ptr() as *const ::core::ffi::c_char,
        4 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        '~' as ::core::ffi::c_int
    } else {
        _POSIX_VDISABLE
    }) as cc_t;
    autologin = -1 as ::core::ffi::c_int;
    argp_program_bug_address = b"<bug-inetutils@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
    argp_version_setup(
        b"telnet\0".as_ptr() as *const ::core::ffi::c_char,
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
    if autologin == -1 as ::core::ffi::c_int {
        autologin = if rlogin as ::core::ffi::c_int == _POSIX_VDISABLE {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
    }
    argc -= index;
    argv = argv.offset(index as isize);
    if argc != 0 {
        let mut args: [*mut ::core::ffi::c_char; 10] =
            [::core::ptr::null_mut::<::core::ffi::c_char>(); 10];
        let mut argp_0: *mut *mut ::core::ffi::c_char =
            &raw mut args as *mut *mut ::core::ffi::c_char;
        if argc > 2 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"too many arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"too many arguments\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
        let c2rust_fresh0 = argp_0;
        argp_0 = argp_0.offset(1);
        *c2rust_fresh0 = prompt;
        if !user.is_null() {
            let c2rust_fresh1 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh1 =
                b"-l\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            let c2rust_fresh2 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh2 = user;
        }
        if !srcaddr.is_null() {
            let c2rust_fresh3 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh3 =
                b"-b\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
            let c2rust_fresh4 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh4 = srcaddr;
        }
        if family == 4 as ::core::ffi::c_int {
            let c2rust_fresh5 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh5 =
                b"-4\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        } else if family == 6 as ::core::ffi::c_int {
            let c2rust_fresh6 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh6 =
                b"-6\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        let c2rust_fresh7 = argp_0;
        argp_0 = argp_0.offset(1);
        *c2rust_fresh7 = *argv.offset(0isize);
        if argc > 1 as ::core::ffi::c_int {
            let c2rust_fresh8 = argp_0;
            argp_0 = argp_0.offset(1);
            *c2rust_fresh8 = *argv.offset(1isize);
        }
        *argp_0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return rboxc_native_telnet_connect(
            argp_0.offset_from(&raw mut args as *mut *mut ::core::ffi::c_char)
                as ::core::ffi::c_int,
            &raw mut args as *mut *mut ::core::ffi::c_char,
        );
    }
    rboxc_native_telnet_loop();
    panic!("Reached end of non-void function without returning");
}
