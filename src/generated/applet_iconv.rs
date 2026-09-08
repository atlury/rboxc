// Generated from pinned GNU iconv 2.43 by scripts/translate-entry-provider.py.
// Source SHA-256: b304ba8dbeb9b4e4bc56592e0be3197c191c9f9fed0f29e495cc960b36e6c4cd
/* Convert text in given files from the specified from-set to the to-set.
   Copyright (C) 1998-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published
   by the Free Software Foundation; version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <https://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct _IO_jump_t { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn tmpfile() -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn asprintf(
        __ptr: *mut *mut ::core::ffi::c_char,
        __fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn putchar(__c: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut program_invocation_short_name: *mut ::core::ffi::c_char;
    fn argp_parse(
        __argp: *const argp,
        __argc: ::core::ffi::c_int,
        __argv: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    fn argp_help(
        __argp: *const argp,
        __stream: *mut FILE,
        __flags: ::core::ffi::c_uint,
        __name: *mut ::core::ffi::c_char,
    );
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn open64(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn iconv_close(__cd: iconv_t) -> ::core::ffi::c_int;
    fn iconv_open(
        __tocode: *const ::core::ffi::c_char,
        __fromcode: *const ::core::ffi::c_char,
    ) -> iconv_t;
    fn iconv(
        __cd: iconv_t,
        __inbuf: *mut *mut ::core::ffi::c_char,
        __inbytesleft: *mut size_t,
        __outbuf: *mut *mut ::core::ffi::c_char,
        __outbytesleft: *mut size_t,
    ) -> size_t;
    fn nl_langinfo(__item: nl_item) -> *mut ::core::ffi::c_char;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn tsearch(
        __key: *const ::core::ffi::c_void,
        __rootp: *mut *mut ::core::ffi::c_void,
        __compar: __compar_fn_t,
    ) -> *mut ::core::ffi::c_void;
    fn twalk(__root: *const ::core::ffi::c_void, __action: __action_fn_t);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn statx(
        __dirfd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
        __mask: ::core::ffi::c_uint,
        __buf: *mut statx,
    ) -> ::core::ffi::c_int;
    fn __isoc23_strtol(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strverscmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn lseek(
        __fd: ::core::ffi::c_int,
        __offset: __off64_t,
        __whence: ::core::ffi::c_int,
    ) -> __off64_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn __dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static _libc_intl_domainname: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_iconv_charmap_read"]
    fn charmap_read(
        filename: *const ::core::ffi::c_char,
        verbose_0: ::core::ffi::c_int,
        error_not_found: ::core::ffi::c_int,
        be_quiet: ::core::ffi::c_int,
        use_default: ::core::ffi::c_int,
    ) -> *mut charmap_t;
    #[link_name = "rboxc_iconv_verbose"]
    static mut verbose: ::core::ffi::c_int;
    #[link_name = "rboxc_iconv_charmap_conversion"]
    fn charmap_conversion(
        from_code_0: *const ::core::ffi::c_char,
        from_charmap: *mut charmap_t,
        to_code_0: *const ::core::ffi::c_char,
        to_charmap: *mut charmap_t,
        argc: ::core::ffi::c_int,
        remaining: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        output_file_0: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __gconv_open(
        conv_spec: *mut gconv_spec,
        handle: *mut __gconv_t,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __gconv_create_spec(
        conv_spec: *mut gconv_spec,
        fromcode: *const ::core::ffi::c_char,
        tocode: *const ::core::ffi::c_char,
    ) -> *mut gconv_spec;
    fn __gconv_destroy_spec(conv_spec: *mut gconv_spec);
    fn __gconv_get_cache() -> *mut ::core::ffi::c_void;
    fn __gconv_get_modules_db() -> *mut gconv_module;
    fn __gconv_get_alias_db() -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: ::core::ffi::c_int,
    pub __value: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __wch: ::core::ffi::c_uint,
    pub __wchb: [::core::ffi::c_char; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_wide_data {
    pub _IO_read_ptr: *mut wchar_t,
    pub _IO_read_end: *mut wchar_t,
    pub _IO_read_base: *mut wchar_t,
    pub _IO_write_base: *mut wchar_t,
    pub _IO_write_ptr: *mut wchar_t,
    pub _IO_write_end: *mut wchar_t,
    pub _IO_buf_base: *mut wchar_t,
    pub _IO_buf_end: *mut wchar_t,
    pub _IO_save_base: *mut wchar_t,
    pub _IO_backup_base: *mut wchar_t,
    pub _IO_save_end: *mut wchar_t,
    pub _IO_state: __mbstate_t,
    pub _IO_last_state: __mbstate_t,
    pub _codecvt: _IO_codecvt,
    pub _shortbuf: [wchar_t; 1],
    pub _wide_vtable: *const _IO_jump_t,
}
pub type wchar_t = ::libc::wchar_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_codecvt {
    pub __cd_in: _IO_iconv_t,
    pub __cd_out: _IO_iconv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_iconv_t {
    pub step: *mut __gconv_step,
    pub step_data: __gconv_step_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gconv_step_data {
    pub __outbuf: *mut ::core::ffi::c_uchar,
    pub __outbufend: *mut ::core::ffi::c_uchar,
    pub __flags: ::core::ffi::c_int,
    pub __invocation_counter: ::core::ffi::c_int,
    pub __internal_use: ::core::ffi::c_int,
    pub __statep: *mut __mbstate_t,
    pub __state: __mbstate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gconv_step {
    pub __shlib_handle: *mut __gconv_loaded_object,
    pub __modname: *const ::core::ffi::c_char,
    pub __counter: ::core::ffi::c_int,
    pub __from_name: *mut ::core::ffi::c_char,
    pub __to_name: *mut ::core::ffi::c_char,
    pub __fct: __gconv_fct,
    pub __btowc_fct: __gconv_btowc_fct,
    pub __init_fct: __gconv_init_fct,
    pub __end_fct: __gconv_end_fct,
    pub __min_needed_from: ::core::ffi::c_int,
    pub __max_needed_from: ::core::ffi::c_int,
    pub __min_needed_to: ::core::ffi::c_int,
    pub __max_needed_to: ::core::ffi::c_int,
    pub __stateful: ::core::ffi::c_int,
    pub __data: *mut ::core::ffi::c_void,
}
pub type __gconv_end_fct = Option<unsafe extern "C" fn(*mut __gconv_step) -> ()>;
pub type __gconv_init_fct = Option<unsafe extern "C" fn(*mut __gconv_step) -> ::core::ffi::c_int>;
pub type __gconv_btowc_fct =
    Option<unsafe extern "C" fn(*mut __gconv_step, ::core::ffi::c_uchar) -> wint_t>;
pub type wint_t = ::core::ffi::c_uint;
pub type __gconv_fct = Option<
    unsafe extern "C" fn(
        *mut __gconv_step,
        *mut __gconv_step_data,
        *mut *const ::core::ffi::c_uchar,
        *const ::core::ffi::c_uchar,
        *mut *mut ::core::ffi::c_uchar,
        *mut size_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gconv_loaded_object {
    pub name: *const ::core::ffi::c_char,
    pub counter: ::core::ffi::c_int,
    pub handle: *mut ::core::ffi::c_void,
    pub fct: __gconv_fct,
    pub init_fct: __gconv_init_fct,
    pub end_fct: __gconv_end_fct,
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut FILE,
    pub _pos: ::core::ffi::c_int,
}
pub type FILE = _IO_FILE;
pub type off64_t = __off64_t;
pub type ssize_t = isize;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: ::core::ffi::c_long,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut ::core::ffi::c_char,
    pub next_free: *mut ::core::ffi::c_char,
    pub chunk_limit: *mut ::core::ffi::c_char,
    pub temp: C2Rust_Unnamed_0,
    pub alignment_mask: ::core::ffi::c_int,
    pub chunkfun: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, ::core::ffi::c_long) -> *mut _obstack_chunk,
    >,
    pub freefun: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut _obstack_chunk) -> ()>,
    pub extra_arg: *mut ::core::ffi::c_void,
    #[bitfield(name = "use_extra_arg", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "maybe_empty_object",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "alloc_failed", ty = "::core::ffi::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut ::core::ffi::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [::core::ffi::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub tempint: ::core::ffi::c_long,
    pub tempptr: *mut ::core::ffi::c_void,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const __GCONV_OK: Self = Self(0);
    pub const __GCONV_NOCONV: Self = Self(1);
    pub const __GCONV_NODB: Self = Self(2);
    pub const __GCONV_NOMEM: Self = Self(3);
    pub const __GCONV_EMPTY_INPUT: Self = Self(4);
    pub const __GCONV_FULL_OUTPUT: Self = Self(5);
    pub const __GCONV_ILLEGAL_INPUT: Self = Self(6);
    pub const __GCONV_INCOMPLETE_INPUT: Self = Self(7);
    pub const __GCONV_ILLEGAL_DESCRIPTOR: Self = Self(8);
    pub const __GCONV_INTERNAL_ERROR: Self = Self(9);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gconv_info {
    pub __nsteps: size_t,
    pub __steps: *mut __gconv_step,
    pub __data: [__gconv_step_data; 0],
}
pub type __gconv_t = *mut __gconv_info;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
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
pub type iconv_t = *mut ::core::ffi::c_void;
pub type nl_item = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const ABDAY_1: Self = Self(131072);
    pub const ABDAY_2: Self = Self(131073);
    pub const ABDAY_3: Self = Self(131074);
    pub const ABDAY_4: Self = Self(131075);
    pub const ABDAY_5: Self = Self(131076);
    pub const ABDAY_6: Self = Self(131077);
    pub const ABDAY_7: Self = Self(131078);
    pub const DAY_1: Self = Self(131079);
    pub const DAY_2: Self = Self(131080);
    pub const DAY_3: Self = Self(131081);
    pub const DAY_4: Self = Self(131082);
    pub const DAY_5: Self = Self(131083);
    pub const DAY_6: Self = Self(131084);
    pub const DAY_7: Self = Self(131085);
    pub const ABMON_1: Self = Self(131086);
    pub const ABMON_2: Self = Self(131087);
    pub const ABMON_3: Self = Self(131088);
    pub const ABMON_4: Self = Self(131089);
    pub const ABMON_5: Self = Self(131090);
    pub const ABMON_6: Self = Self(131091);
    pub const ABMON_7: Self = Self(131092);
    pub const ABMON_8: Self = Self(131093);
    pub const ABMON_9: Self = Self(131094);
    pub const ABMON_10: Self = Self(131095);
    pub const ABMON_11: Self = Self(131096);
    pub const ABMON_12: Self = Self(131097);
    pub const MON_1: Self = Self(131098);
    pub const MON_2: Self = Self(131099);
    pub const MON_3: Self = Self(131100);
    pub const MON_4: Self = Self(131101);
    pub const MON_5: Self = Self(131102);
    pub const MON_6: Self = Self(131103);
    pub const MON_7: Self = Self(131104);
    pub const MON_8: Self = Self(131105);
    pub const MON_9: Self = Self(131106);
    pub const MON_10: Self = Self(131107);
    pub const MON_11: Self = Self(131108);
    pub const MON_12: Self = Self(131109);
    pub const AM_STR: Self = Self(131110);
    pub const PM_STR: Self = Self(131111);
    pub const D_T_FMT: Self = Self(131112);
    pub const D_FMT: Self = Self(131113);
    pub const T_FMT: Self = Self(131114);
    pub const T_FMT_AMPM: Self = Self(131115);
    pub const ERA: Self = Self(131116);
    pub const __ERA_YEAR: Self = Self(131117);
    pub const ERA_D_FMT: Self = Self(131118);
    pub const ALT_DIGITS: Self = Self(131119);
    pub const ERA_D_T_FMT: Self = Self(131120);
    pub const ERA_T_FMT: Self = Self(131121);
    pub const _NL_TIME_ERA_NUM_ENTRIES: Self = Self(131122);
    pub const _NL_TIME_ERA_ENTRIES: Self = Self(131123);
    pub const _NL_WABDAY_1: Self = Self(131124);
    pub const _NL_WABDAY_2: Self = Self(131125);
    pub const _NL_WABDAY_3: Self = Self(131126);
    pub const _NL_WABDAY_4: Self = Self(131127);
    pub const _NL_WABDAY_5: Self = Self(131128);
    pub const _NL_WABDAY_6: Self = Self(131129);
    pub const _NL_WABDAY_7: Self = Self(131130);
    pub const _NL_WDAY_1: Self = Self(131131);
    pub const _NL_WDAY_2: Self = Self(131132);
    pub const _NL_WDAY_3: Self = Self(131133);
    pub const _NL_WDAY_4: Self = Self(131134);
    pub const _NL_WDAY_5: Self = Self(131135);
    pub const _NL_WDAY_6: Self = Self(131136);
    pub const _NL_WDAY_7: Self = Self(131137);
    pub const _NL_WABMON_1: Self = Self(131138);
    pub const _NL_WABMON_2: Self = Self(131139);
    pub const _NL_WABMON_3: Self = Self(131140);
    pub const _NL_WABMON_4: Self = Self(131141);
    pub const _NL_WABMON_5: Self = Self(131142);
    pub const _NL_WABMON_6: Self = Self(131143);
    pub const _NL_WABMON_7: Self = Self(131144);
    pub const _NL_WABMON_8: Self = Self(131145);
    pub const _NL_WABMON_9: Self = Self(131146);
    pub const _NL_WABMON_10: Self = Self(131147);
    pub const _NL_WABMON_11: Self = Self(131148);
    pub const _NL_WABMON_12: Self = Self(131149);
    pub const _NL_WMON_1: Self = Self(131150);
    pub const _NL_WMON_2: Self = Self(131151);
    pub const _NL_WMON_3: Self = Self(131152);
    pub const _NL_WMON_4: Self = Self(131153);
    pub const _NL_WMON_5: Self = Self(131154);
    pub const _NL_WMON_6: Self = Self(131155);
    pub const _NL_WMON_7: Self = Self(131156);
    pub const _NL_WMON_8: Self = Self(131157);
    pub const _NL_WMON_9: Self = Self(131158);
    pub const _NL_WMON_10: Self = Self(131159);
    pub const _NL_WMON_11: Self = Self(131160);
    pub const _NL_WMON_12: Self = Self(131161);
    pub const _NL_WAM_STR: Self = Self(131162);
    pub const _NL_WPM_STR: Self = Self(131163);
    pub const _NL_WD_T_FMT: Self = Self(131164);
    pub const _NL_WD_FMT: Self = Self(131165);
    pub const _NL_WT_FMT: Self = Self(131166);
    pub const _NL_WT_FMT_AMPM: Self = Self(131167);
    pub const _NL_WERA_YEAR: Self = Self(131168);
    pub const _NL_WERA_D_FMT: Self = Self(131169);
    pub const _NL_WALT_DIGITS: Self = Self(131170);
    pub const _NL_WERA_D_T_FMT: Self = Self(131171);
    pub const _NL_WERA_T_FMT: Self = Self(131172);
    pub const _NL_TIME_WEEK_NDAYS: Self = Self(131173);
    pub const _NL_TIME_WEEK_1STDAY: Self = Self(131174);
    pub const _NL_TIME_WEEK_1STWEEK: Self = Self(131175);
    pub const _NL_TIME_FIRST_WEEKDAY: Self = Self(131176);
    pub const _NL_TIME_FIRST_WORKDAY: Self = Self(131177);
    pub const _NL_TIME_CAL_DIRECTION: Self = Self(131178);
    pub const _NL_TIME_TIMEZONE: Self = Self(131179);
    pub const _DATE_FMT: Self = Self(131180);
    pub const _NL_W_DATE_FMT: Self = Self(131181);
    pub const _NL_TIME_CODESET: Self = Self(131182);
    pub const __ALTMON_1: Self = Self(131183);
    pub const __ALTMON_2: Self = Self(131184);
    pub const __ALTMON_3: Self = Self(131185);
    pub const __ALTMON_4: Self = Self(131186);
    pub const __ALTMON_5: Self = Self(131187);
    pub const __ALTMON_6: Self = Self(131188);
    pub const __ALTMON_7: Self = Self(131189);
    pub const __ALTMON_8: Self = Self(131190);
    pub const __ALTMON_9: Self = Self(131191);
    pub const __ALTMON_10: Self = Self(131192);
    pub const __ALTMON_11: Self = Self(131193);
    pub const __ALTMON_12: Self = Self(131194);
    pub const _NL_WALTMON_1: Self = Self(131195);
    pub const _NL_WALTMON_2: Self = Self(131196);
    pub const _NL_WALTMON_3: Self = Self(131197);
    pub const _NL_WALTMON_4: Self = Self(131198);
    pub const _NL_WALTMON_5: Self = Self(131199);
    pub const _NL_WALTMON_6: Self = Self(131200);
    pub const _NL_WALTMON_7: Self = Self(131201);
    pub const _NL_WALTMON_8: Self = Self(131202);
    pub const _NL_WALTMON_9: Self = Self(131203);
    pub const _NL_WALTMON_10: Self = Self(131204);
    pub const _NL_WALTMON_11: Self = Self(131205);
    pub const _NL_WALTMON_12: Self = Self(131206);
    pub const _NL_ABALTMON_1: Self = Self(131207);
    pub const _NL_ABALTMON_2: Self = Self(131208);
    pub const _NL_ABALTMON_3: Self = Self(131209);
    pub const _NL_ABALTMON_4: Self = Self(131210);
    pub const _NL_ABALTMON_5: Self = Self(131211);
    pub const _NL_ABALTMON_6: Self = Self(131212);
    pub const _NL_ABALTMON_7: Self = Self(131213);
    pub const _NL_ABALTMON_8: Self = Self(131214);
    pub const _NL_ABALTMON_9: Self = Self(131215);
    pub const _NL_ABALTMON_10: Self = Self(131216);
    pub const _NL_ABALTMON_11: Self = Self(131217);
    pub const _NL_ABALTMON_12: Self = Self(131218);
    pub const _NL_WABALTMON_1: Self = Self(131219);
    pub const _NL_WABALTMON_2: Self = Self(131220);
    pub const _NL_WABALTMON_3: Self = Self(131221);
    pub const _NL_WABALTMON_4: Self = Self(131222);
    pub const _NL_WABALTMON_5: Self = Self(131223);
    pub const _NL_WABALTMON_6: Self = Self(131224);
    pub const _NL_WABALTMON_7: Self = Self(131225);
    pub const _NL_WABALTMON_8: Self = Self(131226);
    pub const _NL_WABALTMON_9: Self = Self(131227);
    pub const _NL_WABALTMON_10: Self = Self(131228);
    pub const _NL_WABALTMON_11: Self = Self(131229);
    pub const _NL_WABALTMON_12: Self = Self(131230);
    pub const _NL_NUM_LC_TIME: Self = Self(131231);
    pub const _NL_COLLATE_NRULES: Self = Self(196608);
    pub const _NL_COLLATE_RULESETS: Self = Self(196609);
    pub const _NL_COLLATE_TABLEMB: Self = Self(196610);
    pub const _NL_COLLATE_WEIGHTMB: Self = Self(196611);
    pub const _NL_COLLATE_EXTRAMB: Self = Self(196612);
    pub const _NL_COLLATE_INDIRECTMB: Self = Self(196613);
    pub const _NL_COLLATE_GAP1: Self = Self(196614);
    pub const _NL_COLLATE_GAP2: Self = Self(196615);
    pub const _NL_COLLATE_GAP3: Self = Self(196616);
    pub const _NL_COLLATE_TABLEWC: Self = Self(196617);
    pub const _NL_COLLATE_WEIGHTWC: Self = Self(196618);
    pub const _NL_COLLATE_EXTRAWC: Self = Self(196619);
    pub const _NL_COLLATE_INDIRECTWC: Self = Self(196620);
    pub const _NL_COLLATE_SYMB_HASH_SIZEMB: Self = Self(196621);
    pub const _NL_COLLATE_SYMB_TABLEMB: Self = Self(196622);
    pub const _NL_COLLATE_SYMB_EXTRAMB: Self = Self(196623);
    pub const _NL_COLLATE_COLLSEQMB: Self = Self(196624);
    pub const _NL_COLLATE_COLLSEQWC: Self = Self(196625);
    pub const _NL_COLLATE_CODESET: Self = Self(196626);
    pub const _NL_NUM_LC_COLLATE: Self = Self(196627);
    pub const _NL_CTYPE_CLASS: Self = Self(0);
    pub const _NL_CTYPE_TOUPPER: Self = Self(1);
    pub const _NL_CTYPE_GAP1: Self = Self(2);
    pub const _NL_CTYPE_TOLOWER: Self = Self(3);
    pub const _NL_CTYPE_GAP2: Self = Self(4);
    pub const _NL_CTYPE_CLASS32: Self = Self(5);
    pub const _NL_CTYPE_GAP3: Self = Self(6);
    pub const _NL_CTYPE_GAP4: Self = Self(7);
    pub const _NL_CTYPE_GAP5: Self = Self(8);
    pub const _NL_CTYPE_GAP6: Self = Self(9);
    pub const _NL_CTYPE_CLASS_NAMES: Self = Self(10);
    pub const _NL_CTYPE_MAP_NAMES: Self = Self(11);
    pub const _NL_CTYPE_WIDTH: Self = Self(12);
    pub const _NL_CTYPE_MB_CUR_MAX: Self = Self(13);
    pub const _NL_CTYPE_CODESET_NAME: Self = Self(14);
    pub const CODESET: Self = Self(14);
    pub const _NL_CTYPE_TOUPPER32: Self = Self(15);
    pub const _NL_CTYPE_TOLOWER32: Self = Self(16);
    pub const _NL_CTYPE_CLASS_OFFSET: Self = Self(17);
    pub const _NL_CTYPE_MAP_OFFSET: Self = Self(18);
    pub const _NL_CTYPE_INDIGITS_MB_LEN: Self = Self(19);
    pub const _NL_CTYPE_INDIGITS0_MB: Self = Self(20);
    pub const _NL_CTYPE_INDIGITS1_MB: Self = Self(21);
    pub const _NL_CTYPE_INDIGITS2_MB: Self = Self(22);
    pub const _NL_CTYPE_INDIGITS3_MB: Self = Self(23);
    pub const _NL_CTYPE_INDIGITS4_MB: Self = Self(24);
    pub const _NL_CTYPE_INDIGITS5_MB: Self = Self(25);
    pub const _NL_CTYPE_INDIGITS6_MB: Self = Self(26);
    pub const _NL_CTYPE_INDIGITS7_MB: Self = Self(27);
    pub const _NL_CTYPE_INDIGITS8_MB: Self = Self(28);
    pub const _NL_CTYPE_INDIGITS9_MB: Self = Self(29);
    pub const _NL_CTYPE_INDIGITS_WC_LEN: Self = Self(30);
    pub const _NL_CTYPE_INDIGITS0_WC: Self = Self(31);
    pub const _NL_CTYPE_INDIGITS1_WC: Self = Self(32);
    pub const _NL_CTYPE_INDIGITS2_WC: Self = Self(33);
    pub const _NL_CTYPE_INDIGITS3_WC: Self = Self(34);
    pub const _NL_CTYPE_INDIGITS4_WC: Self = Self(35);
    pub const _NL_CTYPE_INDIGITS5_WC: Self = Self(36);
    pub const _NL_CTYPE_INDIGITS6_WC: Self = Self(37);
    pub const _NL_CTYPE_INDIGITS7_WC: Self = Self(38);
    pub const _NL_CTYPE_INDIGITS8_WC: Self = Self(39);
    pub const _NL_CTYPE_INDIGITS9_WC: Self = Self(40);
    pub const _NL_CTYPE_OUTDIGIT0_MB: Self = Self(41);
    pub const _NL_CTYPE_OUTDIGIT1_MB: Self = Self(42);
    pub const _NL_CTYPE_OUTDIGIT2_MB: Self = Self(43);
    pub const _NL_CTYPE_OUTDIGIT3_MB: Self = Self(44);
    pub const _NL_CTYPE_OUTDIGIT4_MB: Self = Self(45);
    pub const _NL_CTYPE_OUTDIGIT5_MB: Self = Self(46);
    pub const _NL_CTYPE_OUTDIGIT6_MB: Self = Self(47);
    pub const _NL_CTYPE_OUTDIGIT7_MB: Self = Self(48);
    pub const _NL_CTYPE_OUTDIGIT8_MB: Self = Self(49);
    pub const _NL_CTYPE_OUTDIGIT9_MB: Self = Self(50);
    pub const _NL_CTYPE_OUTDIGIT0_WC: Self = Self(51);
    pub const _NL_CTYPE_OUTDIGIT1_WC: Self = Self(52);
    pub const _NL_CTYPE_OUTDIGIT2_WC: Self = Self(53);
    pub const _NL_CTYPE_OUTDIGIT3_WC: Self = Self(54);
    pub const _NL_CTYPE_OUTDIGIT4_WC: Self = Self(55);
    pub const _NL_CTYPE_OUTDIGIT5_WC: Self = Self(56);
    pub const _NL_CTYPE_OUTDIGIT6_WC: Self = Self(57);
    pub const _NL_CTYPE_OUTDIGIT7_WC: Self = Self(58);
    pub const _NL_CTYPE_OUTDIGIT8_WC: Self = Self(59);
    pub const _NL_CTYPE_OUTDIGIT9_WC: Self = Self(60);
    pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: Self = Self(61);
    pub const _NL_CTYPE_TRANSLIT_FROM_IDX: Self = Self(62);
    pub const _NL_CTYPE_TRANSLIT_FROM_TBL: Self = Self(63);
    pub const _NL_CTYPE_TRANSLIT_TO_IDX: Self = Self(64);
    pub const _NL_CTYPE_TRANSLIT_TO_TBL: Self = Self(65);
    pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: Self = Self(66);
    pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: Self = Self(67);
    pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: Self = Self(68);
    pub const _NL_CTYPE_TRANSLIT_IGNORE: Self = Self(69);
    pub const _NL_CTYPE_MAP_TO_NONASCII: Self = Self(70);
    pub const _NL_CTYPE_NONASCII_CASE: Self = Self(71);
    pub const _NL_CTYPE_EXTRA_MAP_1: Self = Self(72);
    pub const _NL_CTYPE_EXTRA_MAP_2: Self = Self(73);
    pub const _NL_CTYPE_EXTRA_MAP_3: Self = Self(74);
    pub const _NL_CTYPE_EXTRA_MAP_4: Self = Self(75);
    pub const _NL_CTYPE_EXTRA_MAP_5: Self = Self(76);
    pub const _NL_CTYPE_EXTRA_MAP_6: Self = Self(77);
    pub const _NL_CTYPE_EXTRA_MAP_7: Self = Self(78);
    pub const _NL_CTYPE_EXTRA_MAP_8: Self = Self(79);
    pub const _NL_CTYPE_EXTRA_MAP_9: Self = Self(80);
    pub const _NL_CTYPE_EXTRA_MAP_10: Self = Self(81);
    pub const _NL_CTYPE_EXTRA_MAP_11: Self = Self(82);
    pub const _NL_CTYPE_EXTRA_MAP_12: Self = Self(83);
    pub const _NL_CTYPE_EXTRA_MAP_13: Self = Self(84);
    pub const _NL_CTYPE_EXTRA_MAP_14: Self = Self(85);
    pub const _NL_NUM_LC_CTYPE: Self = Self(86);
    pub const __INT_CURR_SYMBOL: Self = Self(262144);
    pub const __CURRENCY_SYMBOL: Self = Self(262145);
    pub const __MON_DECIMAL_POINT: Self = Self(262146);
    pub const __MON_THOUSANDS_SEP: Self = Self(262147);
    pub const __MON_GROUPING: Self = Self(262148);
    pub const __POSITIVE_SIGN: Self = Self(262149);
    pub const __NEGATIVE_SIGN: Self = Self(262150);
    pub const __INT_FRAC_DIGITS: Self = Self(262151);
    pub const __FRAC_DIGITS: Self = Self(262152);
    pub const __P_CS_PRECEDES: Self = Self(262153);
    pub const __P_SEP_BY_SPACE: Self = Self(262154);
    pub const __N_CS_PRECEDES: Self = Self(262155);
    pub const __N_SEP_BY_SPACE: Self = Self(262156);
    pub const __P_SIGN_POSN: Self = Self(262157);
    pub const __N_SIGN_POSN: Self = Self(262158);
    pub const _NL_MONETARY_CRNCYSTR: Self = Self(262159);
    pub const __INT_P_CS_PRECEDES: Self = Self(262160);
    pub const __INT_P_SEP_BY_SPACE: Self = Self(262161);
    pub const __INT_N_CS_PRECEDES: Self = Self(262162);
    pub const __INT_N_SEP_BY_SPACE: Self = Self(262163);
    pub const __INT_P_SIGN_POSN: Self = Self(262164);
    pub const __INT_N_SIGN_POSN: Self = Self(262165);
    pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: Self = Self(262166);
    pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: Self = Self(262167);
    pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: Self = Self(262168);
    pub const _NL_MONETARY_DUO_FRAC_DIGITS: Self = Self(262169);
    pub const _NL_MONETARY_DUO_P_CS_PRECEDES: Self = Self(262170);
    pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: Self = Self(262171);
    pub const _NL_MONETARY_DUO_N_CS_PRECEDES: Self = Self(262172);
    pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: Self = Self(262173);
    pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: Self = Self(262174);
    pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: Self = Self(262175);
    pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: Self = Self(262176);
    pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: Self = Self(262177);
    pub const _NL_MONETARY_DUO_P_SIGN_POSN: Self = Self(262178);
    pub const _NL_MONETARY_DUO_N_SIGN_POSN: Self = Self(262179);
    pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: Self = Self(262180);
    pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: Self = Self(262181);
    pub const _NL_MONETARY_UNO_VALID_FROM: Self = Self(262182);
    pub const _NL_MONETARY_UNO_VALID_TO: Self = Self(262183);
    pub const _NL_MONETARY_DUO_VALID_FROM: Self = Self(262184);
    pub const _NL_MONETARY_DUO_VALID_TO: Self = Self(262185);
    pub const _NL_MONETARY_CONVERSION_RATE: Self = Self(262186);
    pub const _NL_MONETARY_DECIMAL_POINT_WC: Self = Self(262187);
    pub const _NL_MONETARY_THOUSANDS_SEP_WC: Self = Self(262188);
    pub const _NL_MONETARY_CODESET: Self = Self(262189);
    pub const _NL_NUM_LC_MONETARY: Self = Self(262190);
    pub const __DECIMAL_POINT: Self = Self(65536);
    pub const RADIXCHAR: Self = Self(65536);
    pub const __THOUSANDS_SEP: Self = Self(65537);
    pub const THOUSEP: Self = Self(65537);
    pub const __GROUPING: Self = Self(65538);
    pub const _NL_NUMERIC_DECIMAL_POINT_WC: Self = Self(65539);
    pub const _NL_NUMERIC_THOUSANDS_SEP_WC: Self = Self(65540);
    pub const _NL_NUMERIC_CODESET: Self = Self(65541);
    pub const _NL_NUM_LC_NUMERIC: Self = Self(65542);
    pub const __YESEXPR: Self = Self(327680);
    pub const __NOEXPR: Self = Self(327681);
    pub const __YESSTR: Self = Self(327682);
    pub const __NOSTR: Self = Self(327683);
    pub const _NL_MESSAGES_CODESET: Self = Self(327684);
    pub const _NL_NUM_LC_MESSAGES: Self = Self(327685);
    pub const _NL_PAPER_HEIGHT: Self = Self(458752);
    pub const _NL_PAPER_WIDTH: Self = Self(458753);
    pub const _NL_PAPER_CODESET: Self = Self(458754);
    pub const _NL_NUM_LC_PAPER: Self = Self(458755);
    pub const _NL_NAME_NAME_FMT: Self = Self(524288);
    pub const _NL_NAME_NAME_GEN: Self = Self(524289);
    pub const _NL_NAME_NAME_MR: Self = Self(524290);
    pub const _NL_NAME_NAME_MRS: Self = Self(524291);
    pub const _NL_NAME_NAME_MISS: Self = Self(524292);
    pub const _NL_NAME_NAME_MS: Self = Self(524293);
    pub const _NL_NAME_CODESET: Self = Self(524294);
    pub const _NL_NUM_LC_NAME: Self = Self(524295);
    pub const _NL_ADDRESS_POSTAL_FMT: Self = Self(589824);
    pub const _NL_ADDRESS_COUNTRY_NAME: Self = Self(589825);
    pub const _NL_ADDRESS_COUNTRY_POST: Self = Self(589826);
    pub const _NL_ADDRESS_COUNTRY_AB2: Self = Self(589827);
    pub const _NL_ADDRESS_COUNTRY_AB3: Self = Self(589828);
    pub const _NL_ADDRESS_COUNTRY_CAR: Self = Self(589829);
    pub const _NL_ADDRESS_COUNTRY_NUM: Self = Self(589830);
    pub const _NL_ADDRESS_COUNTRY_ISBN: Self = Self(589831);
    pub const _NL_ADDRESS_LANG_NAME: Self = Self(589832);
    pub const _NL_ADDRESS_LANG_AB: Self = Self(589833);
    pub const _NL_ADDRESS_LANG_TERM: Self = Self(589834);
    pub const _NL_ADDRESS_LANG_LIB: Self = Self(589835);
    pub const _NL_ADDRESS_CODESET: Self = Self(589836);
    pub const _NL_NUM_LC_ADDRESS: Self = Self(589837);
    pub const _NL_TELEPHONE_TEL_INT_FMT: Self = Self(655360);
    pub const _NL_TELEPHONE_TEL_DOM_FMT: Self = Self(655361);
    pub const _NL_TELEPHONE_INT_SELECT: Self = Self(655362);
    pub const _NL_TELEPHONE_INT_PREFIX: Self = Self(655363);
    pub const _NL_TELEPHONE_CODESET: Self = Self(655364);
    pub const _NL_NUM_LC_TELEPHONE: Self = Self(655365);
    pub const _NL_MEASUREMENT_MEASUREMENT: Self = Self(720896);
    pub const _NL_MEASUREMENT_CODESET: Self = Self(720897);
    pub const _NL_NUM_LC_MEASUREMENT: Self = Self(720898);
    pub const _NL_IDENTIFICATION_TITLE: Self = Self(786432);
    pub const _NL_IDENTIFICATION_SOURCE: Self = Self(786433);
    pub const _NL_IDENTIFICATION_ADDRESS: Self = Self(786434);
    pub const _NL_IDENTIFICATION_CONTACT: Self = Self(786435);
    pub const _NL_IDENTIFICATION_EMAIL: Self = Self(786436);
    pub const _NL_IDENTIFICATION_TEL: Self = Self(786437);
    pub const _NL_IDENTIFICATION_FAX: Self = Self(786438);
    pub const _NL_IDENTIFICATION_LANGUAGE: Self = Self(786439);
    pub const _NL_IDENTIFICATION_TERRITORY: Self = Self(786440);
    pub const _NL_IDENTIFICATION_AUDIENCE: Self = Self(786441);
    pub const _NL_IDENTIFICATION_APPLICATION: Self = Self(786442);
    pub const _NL_IDENTIFICATION_ABBREVIATION: Self = Self(786443);
    pub const _NL_IDENTIFICATION_REVISION: Self = Self(786444);
    pub const _NL_IDENTIFICATION_DATE: Self = Self(786445);
    pub const _NL_IDENTIFICATION_CATEGORY: Self = Self(786446);
    pub const _NL_IDENTIFICATION_CODESET: Self = Self(786447);
    pub const _NL_NUM_LC_IDENTIFICATION: Self = Self(786448);
    pub const _NL_NUM: Self = Self(786449);
}
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VISIT(pub ::core::ffi::c_uint);
impl VISIT {
    pub const preorder: Self = Self(0);
    pub const postorder: Self = Self(1);
    pub const endorder: Self = Self(2);
    pub const leaf: Self = Self(3);
}
pub type __action_fn_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, VISIT, ::core::ffi::c_int) -> ()>;
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx_timestamp {
    pub tv_sec: __int64_t,
    pub tv_nsec: __uint32_t,
    pub __statx_timestamp_pad1: [__int32_t; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statx {
    pub stx_mask: __uint32_t,
    pub stx_blksize: __uint32_t,
    pub stx_attributes: __uint64_t,
    pub stx_nlink: __uint32_t,
    pub stx_uid: __uint32_t,
    pub stx_gid: __uint32_t,
    pub stx_mode: __uint16_t,
    pub __statx_pad1: [__uint16_t; 1],
    pub stx_ino: __uint64_t,
    pub stx_size: __uint64_t,
    pub stx_blocks: __uint64_t,
    pub stx_attributes_mask: __uint64_t,
    pub stx_atime: statx_timestamp,
    pub stx_btime: statx_timestamp,
    pub stx_ctime: statx_timestamp,
    pub stx_mtime: statx_timestamp,
    pub stx_rdev_major: __uint32_t,
    pub stx_rdev_minor: __uint32_t,
    pub stx_dev_major: __uint32_t,
    pub stx_dev_minor: __uint32_t,
    pub stx_mnt_id: __uint64_t,
    pub stx_dio_mem_align: __uint32_t,
    pub stx_dio_offset_align: __uint32_t,
    pub stx_subvol: __uint64_t,
    pub stx_atomic_write_unit_min: __uint32_t,
    pub stx_atomic_write_unit_max: __uint32_t,
    pub stx_atomic_write_segments_max: __uint32_t,
    pub stx_dio_read_offset_align: __uint32_t,
    pub stx_atomic_write_unit_max_opt: __uint32_t,
    pub __statx_pad2: __uint32_t,
    pub __statx_pad3: [__uint64_t; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub size: ::core::ffi::c_ulong,
    pub filled: ::core::ffi::c_ulong,
    pub first: *mut ::core::ffi::c_void,
    pub table: *mut ::core::ffi::c_void,
    pub mem_pool: obstack,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charseq {
    pub name: *const ::core::ffi::c_char,
    pub ucs4: uint32_t,
    pub nbytes: ::core::ffi::c_int,
    pub bytes: [::core::ffi::c_uchar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct width_rule {
    pub from: *mut charseq,
    pub to: *mut charseq,
    pub width: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct charmap_t {
    pub code_set_name: *const ::core::ffi::c_char,
    pub repertoiremap: *const ::core::ffi::c_char,
    pub mb_cur_min: ::core::ffi::c_int,
    pub mb_cur_max: ::core::ffi::c_int,
    pub width_rules: *mut width_rule,
    pub nwidth_rules: size_t,
    pub nwidth_rules_max: size_t,
    pub width_default: ::core::ffi::c_uint,
    pub mem_pool: obstack,
    pub char_table: hash_table,
    pub byte_table: hash_table,
    pub ucs4_table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gconv_alias {
    pub fromname: *mut ::core::ffi::c_char,
    pub toname: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gconv_module {
    pub from_string: *const ::core::ffi::c_char,
    pub to_string: *const ::core::ffi::c_char,
    pub cost_hi: ::core::ffi::c_int,
    pub cost_lo: ::core::ffi::c_int,
    pub module_name: *const ::core::ffi::c_char,
    pub left: *mut gconv_module,
    pub same: *mut gconv_module,
    pub right: *mut gconv_module,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gconv_spec {
    pub fromcode: *mut ::core::ffi::c_char,
    pub tocode: *mut ::core::ffi::c_char,
    pub translit: bool,
    pub ignore: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const __GCONV_ENCOUNTERED_ILLEGAL_INPUT: Self = Self(1073741824);
}
pub type gidx_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gconvcache_header {
    pub magic: uint32_t,
    pub string_offset: gidx_t,
    pub hash_offset: gidx_t,
    pub hash_size: gidx_t,
    pub module_offset: gidx_t,
    pub otherconv_offset: gidx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_entry {
    pub string_offset: gidx_t,
    pub module_idx: gidx_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const OPT_VERBOSE: Self = Self(1000);
    pub const OPT_BUFFER_SIZE: Self = Self(1001);
}
pub const BUFSIZ: ::core::ffi::c_int = 8192 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EBADF: ::core::ffi::c_int = 9;
pub const EINVAL: ::core::ffi::c_int = 22;
pub const ENOSPC: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const EILSEQ: ::core::ffi::c_int = 84;
pub const OPTION_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_KEY_HELP_EXTRA: ::core::ffi::c_int = 33554436;
pub const ARGP_HELP_SEE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_EXCL: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const O_TRUNC: ::core::ffi::c_int = 0o1000 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const AT_EMPTY_PATH: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STATX_MODE: ::core::ffi::c_uint = 0x2 as ::core::ffi::c_uint;
pub const STATX_INO: ::core::ffi::c_uint = 0x100 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const VERSION: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"2.43\0") };
unsafe extern "C" fn __gconv_has_illegal_input(mut cd: __gconv_t) -> bool {
    let mut i: size_t = 0 as size_t;
    while i < (*cd).__nsteps {
        if (*(&raw mut (*cd).__data as *mut __gconv_step_data).offset(i as isize)).__flags
            & C2Rust_Unnamed_4::__GCONV_ENCOUNTERED_ILLEGAL_INPUT.0 as ::core::ffi::c_int
            != 0
        {
            return r#true != 0;
        }
        i = i.wrapping_add(1);
    }
    return r#false != 0;
}
pub const PKGVERSION: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"(GNU libc) \0") };
pub const REPORT_BUGS_TO: [::core::ffi::c_char; 46] = unsafe {
    ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
        *b"<https://www.gnu.org/software/libc/bugs.html>\0",
    )
};
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_iconv_argp_program_version_hook"]
pub static mut argp_program_version_hook: Option<
    unsafe extern "C" fn(*mut FILE, *mut argp_state) -> (),
> = Some(print_version as unsafe extern "C" fn(*mut FILE, *mut argp_state) -> ());
pub const OPT_LIST: ::core::ffi::c_int = 108;
static mut options: [argp_option; 12] = [
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Input/Output format specification:\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"from-code\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"encoding of original text\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"to-code\0".as_ptr() as *const ::core::ffi::c_char,
        key: 't' as ::core::ffi::c_int,
        arg: b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"encoding for output\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Information:\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"list\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'l' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"list all known coded character sets\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Output control:\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'c' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"omit invalid characters from output\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"output\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'o' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"output file\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
        key: 's' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"suppress warnings\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_5::OPT_VERBOSE.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"print progress information\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"buffer-size\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_5::OPT_BUFFER_SIZE.0 as ::core::ffi::c_int,
        arg: b"BYTE-COUNT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_HIDDEN,
        doc: b"size of in-memory scratch buffer\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: 0,
    },
];
static mut doc: [::core::ffi::c_char; 62] = unsafe {
    ::core::mem::transmute::<[u8; 62], [::core::ffi::c_char; 62]>(
        *b"Convert encoding of given files from one encoding to another.\0",
    )
};
static mut args_doc: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"[FILE...]\0") };
static mut argp: argp = unsafe {
    argp {
        options: &raw const options as *const argp_option,
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
        help_filter: Some(
            more_help
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_char,
        ),
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
static mut from_code: *const ::core::ffi::c_char = b"\0".as_ptr() as *const ::core::ffi::c_char;
static mut to_code: *const ::core::ffi::c_char = b"\0".as_ptr() as *const ::core::ffi::c_char;
static mut output_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut list: ::core::ffi::c_int = 0;
#[export_name = "rboxc_iconv_omit_invalid"]
pub static mut omit_invalid: ::core::ffi::c_int = 0;
static mut current_input_file_index: ::core::ffi::c_int = 0;
static mut output_buffer_size: size_t =
    (1024 as ::core::ffi::c_int * 1024 as ::core::ffi::c_int) as size_t;
unsafe extern "C" fn rboxc_iconv_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut status: ::core::ffi::c_int = EXIT_SUCCESS;
    let mut cd: __gconv_t = ::core::ptr::null_mut::<__gconv_info>();
    let mut from_charmap: *mut charmap_t = ::core::ptr::null_mut::<charmap_t>();
    let mut to_charmap: *mut charmap_t = ::core::ptr::null_mut::<charmap_t>();
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    textdomain(&raw const _libc_intl_domainname as *const ::core::ffi::c_char);
    argp_parse(
        &raw mut argp,
        argc,
        argv,
        0 as ::core::ffi::c_uint,
        &raw mut current_input_file_index,
        NULL,
    );
    if list != 0 {
        print_known_names();
        exit(EXIT_SUCCESS);
    }
    if !strchr(from_code, '/' as ::core::ffi::c_int).is_null() {
        from_charmap = charmap_read(
            from_code,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    if !strchr(to_code, '/' as ::core::ffi::c_int).is_null() {
        to_charmap = charmap_read(
            to_code,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    if !from_charmap.is_null() || !to_charmap.is_null() {
        status = charmap_conversion(
            from_code,
            from_charmap,
            to_code,
            to_charmap,
            argc,
            current_input_file_index,
            argv,
            output_file,
        );
    } else {
        let mut conv_spec: gconv_spec = gconv_spec {
            fromcode: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            tocode: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            translit: false,
            ignore: false,
        };
        let mut res: ::core::ffi::c_int = 0;
        if __gconv_create_spec(&raw mut conv_spec, from_code, to_code).is_null() {
            error(
                EXIT_FAILURE,
                *__errno_location(),
                __dcgettext(
                    &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                    b"failed to start conversion processing\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(1 as ::core::ffi::c_int);
        }
        if omit_invalid != 0 {
            conv_spec.ignore = r#true != 0;
        }
        res = __gconv_open(&raw mut conv_spec, &raw mut cd, 0 as ::core::ffi::c_int);
        __gconv_destroy_spec(&raw mut conv_spec);
        if res == C2Rust_Unnamed_1::__GCONV_OK.0 as ::core::ffi::c_int {
            RBOXC_ICONV_OWNED = cd;
        }
        if res != C2Rust_Unnamed_1::__GCONV_OK.0 as ::core::ffi::c_int {
            if res == C2Rust_Unnamed_1::__GCONV_NOCONV.0 as ::core::ffi::c_int
                || res == C2Rust_Unnamed_1::__GCONV_NODB.0 as ::core::ffi::c_int
            {
                let mut from_wrong: bool =
                    rboxc_iconv_encoding_unavailable(b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char, from_code);
                let mut to_wrong: bool =
                    rboxc_iconv_encoding_unavailable(to_code, b"UTF-8\0".as_ptr() as *const ::core::ffi::c_char);
                let mut from_pretty: *const ::core::ffi::c_char =
                    if *from_code.offset(0isize) as ::core::ffi::c_int != 0 {
                        from_code
                    } else {
                        nl_langinfo(C2Rust_Unnamed_3::CODESET.0 as nl_item)
                            as *const ::core::ffi::c_char
                    };
                let mut to_pretty: *const ::core::ffi::c_char =
                    if *to_code.offset(0isize) as ::core::ffi::c_int != 0 {
                        to_code
                    } else {
                        nl_langinfo(C2Rust_Unnamed_3::CODESET.0 as nl_item)
                            as *const ::core::ffi::c_char
                    };
                if from_wrong {
                    if to_wrong {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            __dcgettext(
                                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                b"conversions from `%s' and to `%s' are not supported\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ),
                            from_pretty,
                            to_pretty,
                        );
                    } else {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            __dcgettext(
                                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                b"conversion from `%s' is not supported\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ),
                            from_pretty,
                        );
                    }
                } else if to_wrong {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        __dcgettext(
                            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                            b"conversion to `%s' is not supported\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        to_pretty,
                    );
                } else {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        __dcgettext(
                            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                            b"conversion from `%s' to `%s' is not supported\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        from_pretty,
                        to_pretty,
                    );
                }
                argp_help(
                    &raw mut argp,
                    stderr,
                    ARGP_HELP_SEE as ::core::ffi::c_uint,
                    program_invocation_short_name,
                );
                exit(1 as ::core::ffi::c_int);
            } else {
                error(
                    EXIT_FAILURE,
                    *__errno_location(),
                    __dcgettext(
                        &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                        b"failed to start conversion processing\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                );
            }
        }
        prepare_output_file(argv);
        if current_input_file_index == argc {
            if process_file(cd as iconv_t, stdin) != 0 as ::core::ffi::c_int {
                status = EXIT_FAILURE;
            }
        } else {
            's_247: loop {
                let mut fd: ::core::ffi::c_int = 0;
                let mut ret: ::core::ffi::c_int = 0;
                if verbose != 0 {
                    fprintf(
                        stderr,
                        b"%s:\n\0".as_ptr() as *const ::core::ffi::c_char,
                        *argv.offset(current_input_file_index as isize),
                    );
                }
                's_176: {
                    if strcmp(
                        *argv.offset(current_input_file_index as isize),
                        b"-\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        fd = STDIN_FILENO;
                    } else {
                        fd = open(*argv.offset(current_input_file_index as isize), O_RDONLY);
                        if fd == -1 as ::core::ffi::c_int {
                            error(
                                0 as ::core::ffi::c_int,
                                *__errno_location(),
                                __dcgettext(
                                    &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                    b"cannot open input file `%s'\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    LC_MESSAGES,
                                ),
                                *argv.offset(current_input_file_index as isize),
                            );
                            status = EXIT_FAILURE;
                            break 's_176;
                        }
                    }
                    ret = process_fd(cd as iconv_t, fd);
                    if fd != STDIN_FILENO {
                        close(fd);
                    }
                    if ret != 0 as ::core::ffi::c_int {
                        status = EXIT_FAILURE;
                        if ret < 0 as ::core::ffi::c_int {
                            break 's_247;
                        }
                    }
                }
                current_input_file_index += 1;
                if current_input_file_index >= argc {
                    break;
                }
            }
        }
        if __gconv_has_illegal_input(cd) {
            status = EXIT_FAILURE;
        }
        close_output_file(cd, status);
    }
    return status;
}
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        102 => {
            from_code = arg;
        }
        116 => {
            to_code = arg;
        }
        111 => {
            output_file = arg;
        }
        115 => {}
        99 => {
            omit_invalid = 1 as ::core::ffi::c_int;
        }
        1001 => {
            let mut i: ::core::ffi::c_int = __isoc23_strtol(
                arg,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                10 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int;
            if i <= 0 as ::core::ffi::c_int {
                error(
                    EXIT_FAILURE,
                    0 as ::core::ffi::c_int,
                    __dcgettext(
                        &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                        b"invalid buffer size: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    arg,
                );
            }
            output_buffer_size = i as size_t;
        }
        1000 => {
            verbose = 1 as ::core::ffi::c_int;
        }
        OPT_LIST => {
            list = 1 as ::core::ffi::c_int;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    return 0 as error_t;
}
unsafe extern "C" fn more_help(
    mut key: ::core::ffi::c_int,
    mut text: *const ::core::ffi::c_char,
    mut input: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_char {
    let mut tp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match key {
        ARGP_KEY_HELP_EXTRA => {
            if asprintf(
                &raw mut tp,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"For bug reporting instructions, please see:\n%s.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                REPORT_BUGS_TO.as_ptr(),
            ) < 0 as ::core::ffi::c_int
            {
                return ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            return tp;
        }
        _ => {}
    }
    return text as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn print_version(mut stream: *mut FILE, mut state: *mut argp_state) {
    fprintf(
        stream,
        b"iconv %s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        PKGVERSION.as_ptr(),
        VERSION.as_ptr(),
    );
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Copyright (C) %s Free Software Foundation, Inc.\nThis is free software; see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        b"2024\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Written by %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        b"Ulrich Drepper\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
static mut last_overlapping_file_index: ::core::ffi::c_int = 0;
static mut output_using_temporary_file: bool = false;
static mut output_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
static mut output_buffer_start: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut output_buffer_current: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut output_buffer_remaining: size_t = 0;
static mut copy_buffer_size: size_t = BUFSIZ as size_t;
unsafe extern "C" fn output_error() {
    error(
        EXIT_FAILURE,
        *__errno_location(),
        __dcgettext(
            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
            b"cannot open output file\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
}
unsafe extern "C" fn input_error(mut path: *const ::core::ffi::c_char) {
    error(
        0 as ::core::ffi::c_int,
        *__errno_location(),
        __dcgettext(
            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
            b"cannot open input file `%s'\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        path,
    );
}
unsafe extern "C" fn open_output_direct() {
    output_fd = open64(
        output_file,
        O_WRONLY | O_CREAT | O_TRUNC,
        0o666 as ::core::ffi::c_int,
    );
    if output_fd < 0 as ::core::ffi::c_int {
        output_error();
    }
}
unsafe extern "C" fn prepare_output_file(mut argv: *mut *mut ::core::ffi::c_char) {
    if copy_buffer_size > output_buffer_size {
        copy_buffer_size = output_buffer_size;
    }
    if output_file.is_null()
        || strcmp(output_file, b"-\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        output_fd = STDOUT_FILENO;
        output_buffer_size = copy_buffer_size;
    } else {
        output_fd = open64(
            output_file,
            O_WRONLY | O_CREAT | O_EXCL,
            0o666 as ::core::ffi::c_int,
        );
        if output_fd >= 0 as ::core::ffi::c_int {
            output_buffer_size = copy_buffer_size;
        } else {
            let mut st: statx = statx {
                stx_mask: 0,
                stx_blksize: 0,
                stx_attributes: 0,
                stx_nlink: 0,
                stx_uid: 0,
                stx_gid: 0,
                stx_mode: 0,
                __statx_pad1: [0; 1],
                stx_ino: 0,
                stx_size: 0,
                stx_blocks: 0,
                stx_attributes_mask: 0,
                stx_atime: statx_timestamp {
                    tv_sec: 0,
                    tv_nsec: 0,
                    __statx_timestamp_pad1: [0; 1],
                },
                stx_btime: statx_timestamp {
                    tv_sec: 0,
                    tv_nsec: 0,
                    __statx_timestamp_pad1: [0; 1],
                },
                stx_ctime: statx_timestamp {
                    tv_sec: 0,
                    tv_nsec: 0,
                    __statx_timestamp_pad1: [0; 1],
                },
                stx_mtime: statx_timestamp {
                    tv_sec: 0,
                    tv_nsec: 0,
                    __statx_timestamp_pad1: [0; 1],
                },
                stx_rdev_major: 0,
                stx_rdev_minor: 0,
                stx_dev_major: 0,
                stx_dev_minor: 0,
                stx_mnt_id: 0,
                stx_dio_mem_align: 0,
                stx_dio_offset_align: 0,
                stx_subvol: 0,
                stx_atomic_write_unit_min: 0,
                stx_atomic_write_unit_max: 0,
                stx_atomic_write_segments_max: 0,
                stx_dio_read_offset_align: 0,
                stx_atomic_write_unit_max_opt: 0,
                __statx_pad2: 0,
                __statx_pad3: [0; 8],
            };
            if statx(
                AT_FDCWD,
                output_file,
                0 as ::core::ffi::c_int,
                STATX_INO | STATX_MODE,
                &raw mut st,
            ) != 0 as ::core::ffi::c_int
            {
                output_error();
            }
            let mut out_dev_minor: uint32_t = st.stx_dev_minor;
            let mut out_dev_major: uint32_t = st.stx_dev_major;
            let mut out_ino: uint64_t = st.stx_ino;
            let mut idx: ::core::ffi::c_int = current_input_file_index;
            while !((*argv.offset(idx as isize)).is_null() && idx != current_input_file_index) {
                let mut ret: ::core::ffi::c_int = 0;
                if (*argv.offset(idx as isize)).is_null()
                    || strcmp(
                        *argv.offset(idx as isize),
                        b"-\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                {
                    ret = statx(
                        STDIN_FILENO,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                        AT_EMPTY_PATH,
                        STATX_INO,
                        &raw mut st,
                    );
                } else {
                    ret = statx(
                        AT_FDCWD,
                        *argv.offset(idx as isize),
                        0 as ::core::ffi::c_int,
                        STATX_INO,
                        &raw mut st,
                    );
                }
                if ret != 0 as ::core::ffi::c_int {
                    input_error(*argv.offset(idx as isize));
                    exit(EXIT_FAILURE);
                }
                if out_dev_minor == st.stx_dev_minor
                    && out_dev_major == st.stx_dev_major
                    && out_ino == st.stx_ino
                {
                    if (*argv.offset(idx as isize)).is_null() {
                        last_overlapping_file_index = INT_MAX;
                    } else {
                        last_overlapping_file_index = idx;
                    }
                }
                if (*argv.offset(idx as isize)).is_null() {
                    break;
                }
                idx += 1;
            }
            if last_overlapping_file_index == 0 as ::core::ffi::c_int {
                open_output_direct();
                output_buffer_size = copy_buffer_size;
            }
        }
    }
    output_buffer_start = malloc(output_buffer_size) as *mut ::core::ffi::c_char;
    if output_buffer_start.is_null() {
        output_error();
    }
    output_buffer_current = output_buffer_start;
    output_buffer_remaining = output_buffer_size;
}
unsafe extern "C" fn write_fully(
    mut fd: ::core::ffi::c_int,
    mut first: *const ::core::ffi::c_char,
    mut last: *const ::core::ffi::c_char,
) {
    while first < last {
        let mut ret: ssize_t = write(
            fd,
            first as *const ::core::ffi::c_void,
            last.offset_from(first) as size_t,
        );
        if ret == 0 as ssize_t {
            *__errno_location() = ENOSPC;
            output_error();
        }
        if ret < 0 as ssize_t {
            error(
                EXIT_FAILURE,
                *__errno_location(),
                __dcgettext(
                    &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                    b"conversion stopped due to problem in writing the output\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        }
        first = first.offset(ret as isize);
    }
}
unsafe extern "C" fn flush_output() {
    let mut temporary_file_not_needed: bool =
        current_input_file_index > last_overlapping_file_index;
    if output_fd < 0 as ::core::ffi::c_int {
        if temporary_file_not_needed {
            open_output_direct();
        } else {
            let mut fp: *mut FILE = tmpfile();
            if fp.is_null() {
                output_error();
            }
            output_fd = dup(fileno(fp));
            if output_fd < 0 as ::core::ffi::c_int {
                output_error();
            }
            fclose(fp);
            output_using_temporary_file = r#true != 0;
        }
        output_buffer_size = copy_buffer_size;
    } else if output_using_temporary_file as ::core::ffi::c_int != 0
        && temporary_file_not_needed as ::core::ffi::c_int != 0
    {
        let mut temp_fd: ::core::ffi::c_int = output_fd;
        open_output_direct();
        if lseek(temp_fd, 0 as __off64_t, SEEK_SET) < 0 as __off64_t {
            output_error();
        }
        loop {
            let mut buf: [::core::ffi::c_char; 8192] = [0; 8192];
            let mut ret: ssize_t = read(
                temp_fd,
                &raw mut buf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[::core::ffi::c_char; 8192]>(),
            );
            if ret < 0 as ssize_t {
                output_error();
            }
            if ret == 0 as ssize_t {
                break;
            }
            write_fully(
                output_fd,
                &raw mut buf as *mut ::core::ffi::c_char,
                (&raw mut buf as *mut ::core::ffi::c_char).offset(ret as isize),
            );
        }
        close(temp_fd);
        output_using_temporary_file = r#false != 0;
        output_buffer_size = copy_buffer_size;
    }
    write_fully(output_fd, output_buffer_start, output_buffer_current);
    output_buffer_current = output_buffer_start;
    output_buffer_remaining = output_buffer_size;
}
unsafe extern "C" fn close_output_file(mut cd: __gconv_t, mut status: ::core::ffi::c_int) {
    if status != EXIT_SUCCESS
        && omit_invalid == 0
        && (output_using_temporary_file as ::core::ffi::c_int != 0
            || output_fd < 0 as ::core::ffi::c_int)
    {
        // GNU intentionally skips the flush to preserve overlapping input.
        // The anonymous spool has no remaining consumer on this return path.
        if output_fd >= 0 {
            let saved = *__errno_location();
            close(output_fd);
            output_fd = -1;
            *__errno_location() = saved;
        }
        return;
    }
    let mut n: size_t = iconv(
        cd as iconv_t,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ::core::ptr::null_mut::<size_t>(),
        &raw mut output_buffer_current,
        &raw mut output_buffer_remaining,
    );
    if n == -1 as ::core::ffi::c_int as size_t && *__errno_location() == E2BIG {
        flush_output();
        n = iconv(
            cd as iconv_t,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            ::core::ptr::null_mut::<size_t>(),
            &raw mut output_buffer_current,
            &raw mut output_buffer_remaining,
        );
    }
    let mut saved_errno: ::core::ffi::c_int = *__errno_location();
    flush_output();
    if n == -1 as ::core::ffi::c_int as size_t && omit_invalid == 0 {
        *__errno_location() = saved_errno;
        output_error();
    }
    if output_fd == STDOUT_FILENO {
        output_fd = dup(output_fd);
        if output_fd < 0 as ::core::ffi::c_int {
            output_error();
        }
    }
    if close(output_fd) < 0 as ::core::ffi::c_int {
        output_error();
    }
}
unsafe extern "C" fn process_block(
    mut cd: iconv_t,
    mut addr: *mut *mut ::core::ffi::c_char,
    mut len: *mut size_t,
    mut file_offset: off64_t,
    mut incomplete: *mut bool,
) -> ::core::ffi::c_int {
    let mut start: *const ::core::ffi::c_char = *addr;
    let mut n: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *len > 0 as size_t {
        n = iconv(
            cd,
            addr,
            len,
            &raw mut output_buffer_current,
            &raw mut output_buffer_remaining,
        );
        if n == -1 as ::core::ffi::c_int as size_t
            && omit_invalid != 0
            && *__errno_location() == EILSEQ
        {
            ret = 1 as ::core::ffi::c_int;
            if *len == 0 as size_t {
                n = 0 as size_t;
            } else {
                *__errno_location() = E2BIG;
            }
        }
        if n != -1 as ::core::ffi::c_int as size_t {
            break;
        }
        if *__errno_location() == E2BIG {
            flush_output();
        } else {
            match *__errno_location() {
                EILSEQ => {
                    if omit_invalid == 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            __dcgettext(
                                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                b"illegal input sequence at position %lld\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ),
                            (file_offset as isize + (*addr).offset_from(start))
                                as ::core::ffi::c_longlong,
                        );
                    }
                }
                EINVAL => {
                    *incomplete = r#true != 0;
                    return ret;
                }
                EBADF => {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        __dcgettext(
                            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                            b"internal error (illegal descriptor)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                    );
                }
                _ => {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        __dcgettext(
                            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                            b"unknown iconv() error %d\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        *__errno_location(),
                    );
                }
            }
            return -1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
unsafe extern "C" fn process_fd(mut cd: iconv_t, mut fd: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut inbuf: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut inbuf_end: *mut ::core::ffi::c_char = (&raw mut inbuf as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as isize);
    let mut inbuf_used: size_t = 0 as size_t;
    let mut file_offset: off64_t = 0 as off64_t;
    let mut status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut incomplete: bool = r#false != 0;
    loop {
        let mut p: *mut ::core::ffi::c_char =
            (&raw mut inbuf as *mut ::core::ffi::c_char).offset(inbuf_used as isize);
        let mut read_ret: ssize_t = read(
            fd,
            p as *mut ::core::ffi::c_void,
            inbuf_end.offset_from(p) as size_t,
        );
        if read_ret == 0 as ssize_t {
            if incomplete {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    __dcgettext(
                        &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                        b"incomplete character or shift sequence at end of buffer\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                );
                return 1 as ::core::ffi::c_int;
            }
            return 0 as ::core::ffi::c_int;
        }
        if read_ret < 0 as ssize_t {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                __dcgettext(
                    &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                    b"error while reading the input\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            return -1 as ::core::ffi::c_int;
        }
        inbuf_used = inbuf_used.wrapping_add(read_ret as size_t);
        incomplete = r#false != 0;
        p = &raw mut inbuf as *mut ::core::ffi::c_char;
        let mut ret: ::core::ffi::c_int = process_block(
            cd,
            &raw mut p,
            &raw mut inbuf_used,
            file_offset,
            &raw mut incomplete,
        );
        if ret != 0 as ::core::ffi::c_int {
            status = ret;
            if ret < 0 as ::core::ffi::c_int {
                break;
            }
        }
        memmove(
            &raw mut inbuf as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            inbuf_used,
        );
        file_offset = (file_offset as size_t)
            .wrapping_add((read_ret as size_t).wrapping_sub(inbuf_used))
            as off64_t;
    }
    return status;
}
unsafe extern "C" fn process_file(mut cd: iconv_t, mut input: *mut FILE) -> ::core::ffi::c_int {
    return process_fd(cd, fileno(input));
}
static mut printlist: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
static mut column: size_t = 0;
static mut not_first: ::core::ffi::c_int = 0;
unsafe extern "C" fn insert_print_list(
    mut nodep: *const ::core::ffi::c_void,
    mut value: VISIT,
    mut level: ::core::ffi::c_int,
) {
    if value.0 == VISIT::leaf.0 || value.0 == VISIT::postorder.0 {
        let mut s: *const gconv_alias = *(nodep as *mut *const gconv_alias);
        tsearch(
            (*s).fromname as *const ::core::ffi::c_void,
            &raw mut printlist,
            ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
                >,
                __compar_fn_t,
            >(Some(
                strverscmp
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                    ) -> ::core::ffi::c_int,
            )),
        );
    }
}
unsafe extern "C" fn do_print_human(
    mut nodep: *const ::core::ffi::c_void,
    mut value: VISIT,
    mut level: ::core::ffi::c_int,
) {
    if value.0 == VISIT::leaf.0 || value.0 == VISIT::postorder.0 {
        let mut s: *const ::core::ffi::c_char = *(nodep as *mut *const ::core::ffi::c_char);
        let mut len: size_t = strlen(s);
        let mut cnt: size_t = 0;
        while len > 0 as size_t
            && *s.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                == '/' as ::core::ffi::c_int
        {
            len = len.wrapping_sub(1);
        }
        cnt = 0 as size_t;
        while cnt < len {
            if *(*__ctype_b_loc()).offset(*s.offset(cnt as isize) as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed_2::_ISalnum.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                break;
            }
            cnt = cnt.wrapping_add(1);
        }
        if cnt == len {
            return;
        }
        if not_first != 0 {
            putchar(',' as ::core::ffi::c_int);
            column = column.wrapping_add(1);
            if column > 2 as size_t && column.wrapping_add(len) > 77 as size_t {
                fputs(b"\n  \0".as_ptr() as *const ::core::ffi::c_char, stdout);
                column = 2 as size_t;
            } else {
                putchar(' ' as ::core::ffi::c_int);
                column = column.wrapping_add(1);
            }
        } else {
            not_first = 1 as ::core::ffi::c_int;
        }
        fwrite(s as *const ::core::ffi::c_void, len, 1 as size_t, stdout);
        column = column.wrapping_add(len);
    }
}
unsafe extern "C" fn do_print(
    mut nodep: *const ::core::ffi::c_void,
    mut value: VISIT,
    mut level: ::core::ffi::c_int,
) {
    if value.0 == VISIT::leaf.0 || value.0 == VISIT::postorder.0 {
        let mut s: *const ::core::ffi::c_char = *(nodep as *mut *const ::core::ffi::c_char);
        puts(s);
    }
}
unsafe extern "C" fn add_known_names(mut node: *mut gconv_module) {
    if !(*node).left.is_null() {
        add_known_names((*node).left);
    }
    if !(*node).right.is_null() {
        add_known_names((*node).right);
    }
    loop {
        if strcmp(
            (*node).from_string,
            b"INTERNAL\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            tsearch(
                (*node).from_string as *const ::core::ffi::c_void,
                &raw mut printlist,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                    >,
                    __compar_fn_t,
                >(Some(
                    strverscmp
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                )),
            );
        }
        if strcmp(
            (*node).to_string,
            b"INTERNAL\0".as_ptr() as *const ::core::ffi::c_char,
        ) != 0 as ::core::ffi::c_int
        {
            tsearch(
                (*node).to_string as *const ::core::ffi::c_void,
                &raw mut printlist,
                ::core::mem::transmute::<
                    Option<
                        unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                    >,
                    __compar_fn_t,
                >(Some(
                    strverscmp
                        as unsafe extern "C" fn(
                            *const ::core::ffi::c_char,
                            *const ::core::ffi::c_char,
                        ) -> ::core::ffi::c_int,
                )),
            );
        }
        node = (*node).same;
        if node.is_null() {
            break;
        }
    }
}
unsafe extern "C" fn insert_cache() {
    let mut header: *const gconvcache_header = ::core::ptr::null::<gconvcache_header>();
    let mut strtab: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut hashtab: *const hash_entry = ::core::ptr::null::<hash_entry>();
    let mut cnt: size_t = 0;
    header = __gconv_get_cache() as *const gconvcache_header;
    strtab = (header as *mut ::core::ffi::c_char)
        .offset((*header).string_offset as ::core::ffi::c_int as isize);
    hashtab = (header as *mut ::core::ffi::c_char)
        .offset((*header).hash_offset as ::core::ffi::c_int as isize)
        as *mut hash_entry;
    cnt = 0 as size_t;
    while cnt < (*header).hash_size as size_t {
        if (*hashtab.offset(cnt as isize)).string_offset as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            let mut str: *const ::core::ffi::c_char = strtab.offset(
                (*hashtab.offset(cnt as isize)).string_offset as ::core::ffi::c_int as isize,
            );
            if strcmp(str, b"INTERNAL\0".as_ptr() as *const ::core::ffi::c_char)
                != 0 as ::core::ffi::c_int
            {
                tsearch(
                    str as *const ::core::ffi::c_void,
                    &raw mut printlist,
                    ::core::mem::transmute::<
                        Option<
                            unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                            ) -> ::core::ffi::c_int,
                        >,
                        __compar_fn_t,
                    >(Some(
                        strverscmp
                            as unsafe extern "C" fn(
                                *const ::core::ffi::c_char,
                                *const ::core::ffi::c_char,
                            )
                                -> ::core::ffi::c_int,
                    )),
                );
            }
        }
        cnt = cnt.wrapping_add(1);
    }
}
unsafe extern "C" fn print_known_names() {
    let mut h: iconv_t = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut cache: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    h = iconv_open(
        b"L1\0".as_ptr() as *const ::core::ffi::c_char,
        b"L1\0".as_ptr() as *const ::core::ffi::c_char,
    );
    iconv_close(h);
    cache = __gconv_get_cache();
    if !cache.is_null() {
        insert_cache();
    } else {
        let mut modules: *mut gconv_module = ::core::ptr::null_mut::<gconv_module>();
        twalk(
            __gconv_get_alias_db(),
            Some(
                insert_print_list
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        VISIT,
                        ::core::ffi::c_int,
                    ) -> (),
            ),
        );
        modules = __gconv_get_modules_db();
        if !modules.is_null() {
            add_known_names(modules);
        }
    }
    let mut human_readable: bool = isatty(fileno(stdout)) != 0;
    if human_readable {
        fputs(
            __dcgettext(
                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                b"The following list contains all the coded character sets known.  This does\nnot necessarily mean that all combinations of these names can be used for\nthe FROM and TO command line parameters.  One coded character set can be\nlisted with several different names (aliases).\n\n  \0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            stdout,
        );
    }
    column = 2 as size_t;
    twalk(
        printlist,
        if human_readable as ::core::ffi::c_int != 0 {
            Some(
                do_print_human
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        VISIT,
                        ::core::ffi::c_int,
                    ) -> (),
            )
        } else {
            Some(
                do_print
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        VISIT,
                        ::core::ffi::c_int,
                    ) -> (),
            )
        },
    );
    if human_readable as ::core::ffi::c_int != 0 && column != 0 as size_t {
        puts(b"\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;

extern "C" {
    #[link_name = "program_invocation_name"]
    static mut RBOXC_LIBC_INVOCATION: *mut ::core::ffi::c_char;
    #[link_name = "program_invocation_short_name"]
    static mut RBOXC_LIBC_SHORT_INVOCATION: *mut ::core::ffi::c_char;
    #[link_name = "error_print_progname"]
    static mut RBOXC_ERROR_PRINT_PROGNAME: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut RBOXC_ERROR_STDERR: *mut libc::FILE;

    #[link_name = "argp_program_version_hook"]
    static mut RBOXC_LIBC_VERSION_HOOK: Option<unsafe extern "C" fn(*mut FILE, *mut argp_state)>;
}
unsafe extern "C" fn rboxc_glibc_error_prefix() {
    libc::fprintf(RBOXC_ERROR_STDERR, b"%s: \0".as_ptr().cast(), RBOXC_LIBC_INVOCATION);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_iconv(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    RBOXC_LIBC_INVOCATION = *argv;
    let bytes = ::core::ffi::CStr::from_ptr(*argv).to_bytes();
    let offset = bytes.iter().rposition(|b| *b == b'/').map_or(0, |i| i+1);
    RBOXC_LIBC_SHORT_INVOCATION = (*argv).add(offset);
    RBOXC_ERROR_PRINT_PROGNAME = Some(rboxc_glibc_error_prefix);
    RBOXC_LIBC_VERSION_HOOK = argp_program_version_hook;
    if rboxc_iconv_at_exit(rboxc_iconv_release) != 0 { return 1; }
    rboxc_iconv_main_inner(argc, argv)
}

static mut RBOXC_ICONV_OWNED: __gconv_t = ::core::ptr::null_mut();
extern "C" {
    #[link_name = "atexit"]
    fn rboxc_iconv_at_exit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;
    #[link_name = "tdestroy"]
    fn rboxc_iconv_destroy_tree(root: *mut ::core::ffi::c_void,
        free_key: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void)>);
}
// Name strings belong to the libc conversion database, not this print tree.
unsafe extern "C" fn rboxc_iconv_borrowed_key(_: *mut ::core::ffi::c_void) {}
unsafe extern "C" fn rboxc_iconv_release() {
    let saved = *__errno_location();
    if !RBOXC_ICONV_OWNED.is_null() {
        iconv_close(RBOXC_ICONV_OWNED.cast());
        RBOXC_ICONV_OWNED = ::core::ptr::null_mut();
    }
    libc::free(output_buffer_start.cast());
    output_buffer_start = ::core::ptr::null_mut();
    if !printlist.is_null() {
        rboxc_iconv_destroy_tree(printlist, Some(rboxc_iconv_borrowed_key));
        printlist = ::core::ptr::null_mut();
    }
    *__errno_location() = saved;
}
unsafe fn rboxc_iconv_encoding_unavailable(to: *const ::core::ffi::c_char,
    from: *const ::core::ffi::c_char) -> bool {
    let handle = iconv_open(to, from);
    let saved = *__errno_location();
    let failed = handle == ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(usize::MAX);
    if !failed { iconv_close(handle); }
    *__errno_location() = saved;
    failed && saved == EINVAL
}
