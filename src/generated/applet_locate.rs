// Generated from pinned GNU Findutils 4.11.0 by scripts/translate-findutils.py.
// Source SHA-256: b4337f86080ed949dd737fd87839cc6a0c3eb3c58dcc82489726f3f13c3bfca4
/* locate -- search databases for filenames that match patterns
   Copyright (C) 1994-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct re_dfa_t { _opaque: [u8; 0] }
#[repr(C)]
pub struct quoting_options { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_rpl_fcntl"]
    fn rpl_fcntl(fd: ::core::ffi::c_int, action: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn setgroups(__n: size_t, __groups: *const __gid_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_rpl_re_set_syntax"]
    fn rpl_re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    #[link_name = "rboxc_findutils_locate_rpl_re_compile_pattern"]
    fn rpl_re_compile_pattern(
        __pattern: *const ::core::ffi::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_rpl_re_search"]
    fn rpl_re_search(
        __buffer: *mut re_pattern_buffer,
        __String: *const ::core::ffi::c_char,
        __length: regoff_t,
        __start: regoff_t,
        __range: regoff_t,
        __regs: *mut re_registers,
    ) -> regoff_t;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
    fn setgid(__gid: __gid_t) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fgetc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn ungetc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strndup(__string: *const ::core::ffi::c_char, __n: size_t) -> *mut ::core::ffi::c_char;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strstr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcasestr(
        __haystack: *const ::core::ffi::c_char,
        __needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(_: *mut ::core::ffi::c_void);
    #[link_name = "rboxc_findutils_locate_mbsstr"]
    fn mbsstr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_mbscasestr"]
    fn mbscasestr(
        haystack: *const ::core::ffi::c_char,
        needle: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> ::core::ffi::c_double;
    fn strftime(
        __s: *mut ::core::ffi::c_char,
        __maxsize: size_t,
        __format: *const ::core::ffi::c_char,
        __tp: *const tm,
    ) -> size_t;
    fn localtime(__timer: *const time_t) -> *mut tm;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_xstrtoumax"]
    fn xstrtoumax(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut uintmax_t,
        _: *const ::core::ffi::c_char,
    ) -> strtol_error;
    fn fnmatch(
        __pattern: *const ::core::ffi::c_char,
        __name: *const ::core::ffi::c_char,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_findutils_locate_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_findutils_locate_x2nrealloc"]
    fn x2nrealloc(
        p: *mut ::core::ffi::c_void,
        pn: *mut size_t,
        s: size_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_findutils_locate_open_safer"]
    fn open_safer(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_human_readable"]
    fn human_readable(
        _: uintmax_t,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: uintmax_t,
        _: uintmax_t,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_last_component"]
    fn last_component(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_findutils_locate_clone_quoting_options"]
    fn clone_quoting_options(o: *mut quoting_options) -> *mut quoting_options;
    #[link_name = "rboxc_findutils_locate_quotearg_n_style"]
    fn quotearg_n_style(
        n: ::core::ffi::c_int,
        s: quoting_style,
        arg: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_get_regex_type"]
    fn get_regex_type(s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_xstrtol_fatal"]
    fn xstrtol_fatal(
        _: strtol_error,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_char,
        _: *const option,
        _: *const ::core::ffi::c_char,
    );
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    fn dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn dcngettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid1: *const ::core::ffi::c_char,
        __msgid2: *const ::core::ffi::c_char,
        __n: ::core::ffi::c_ulong,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bindtextdomain(
        __domainname: *const ::core::ffi::c_char,
        __dirname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_locate_explain_how_to_report_bugs"]
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_display_findutils_version"]
    fn display_findutils_version(official_name: *const ::core::ffi::c_char);
    #[link_name = "rboxc_findutils_locate_getword"]
    fn getword(
        fp: *mut FILE,
        filename: *const ::core::ffi::c_char,
        maxvalue: size_t,
        endian_state_flag: *mut GetwordEndianState,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_print_quoted"]
    fn print_quoted(
        fp: *mut FILE,
        qopts: *const quoting_options,
        dest_is_tty: bool,
        format: *const ::core::ffi::c_char,
        s: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_locate_splitstring"]
    fn splitstring(
        s: *const ::core::ffi::c_char,
        separators: *const ::core::ffi::c_char,
        first: bool,
        pos: *mut size_t,
        len: *mut size_t,
    ) -> bool;
}
pub type __uint64_t = u64;
pub type __dev_t = ::core::ffi::c_ulong;
pub type __uid_t = ::core::ffi::c_uint;
pub type __gid_t = ::core::ffi::c_uint;
pub type __ino_t = ::core::ffi::c_ulong;
pub type __mode_t = ::core::ffi::c_uint;
pub type __nlink_t = ::core::ffi::c_ulong;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type size_t = usize;
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: ::core::ffi::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
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
pub type __re_size_t = size_t;
pub type __re_long_size_t = size_t;
pub type reg_syntax_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct re_pattern_buffer {
    pub buffer: *mut re_dfa_t,
    pub allocated: __re_long_size_t,
    pub used: __re_long_size_t,
    pub syntax: reg_syntax_t,
    pub fastmap: *mut ::core::ffi::c_char,
    pub translate: *mut ::core::ffi::c_uchar,
    pub re_nsub: size_t,
    #[bitfield(name = "can_be_null", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "regs_allocated", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "fastmap_accurate", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "no_sub", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "not_bol", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "not_eol", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "newline_anchor", ty = "::core::ffi::c_uint", bits = "7..=7")]
    pub can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type regoff_t = ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: ::core::ffi::c_int,
    pub tm_min: ::core::ffi::c_int,
    pub tm_hour: ::core::ffi::c_int,
    pub tm_mday: ::core::ffi::c_int,
    pub tm_mon: ::core::ffi::c_int,
    pub tm_year: ::core::ffi::c_int,
    pub tm_wday: ::core::ffi::c_int,
    pub tm_yday: ::core::ffi::c_int,
    pub tm_isdst: ::core::ffi::c_int,
    pub tm_gmtoff: ::core::ffi::c_long,
    pub tm_zone: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct strtol_error(pub ::core::ffi::c_uint);
impl strtol_error {
    pub const LONGINT_OK: Self = Self(0);
    pub const LONGINT_OVERFLOW: Self = Self(1);
    pub const LONGINT_INVALID_SUFFIX_CHAR: Self = Self(2);
    pub const LONGINT_INVALID_SUFFIX_CHAR_WITH_OVERFLOW: Self = Self(3);
    pub const LONGINT_INVALID: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const human_ceiling: Self = Self(0);
    pub const human_round_to_nearest: Self = Self(1);
    pub const human_floor: Self = Self(2);
    pub const human_group_digits: Self = Self(4);
    pub const human_suppress_point_zero: Self = Self(8);
    pub const human_autoscale: Self = Self(16);
    pub const human_base_1024: Self = Self(32);
    pub const human_space_before_unit: Self = Self(64);
    pub const human_SI: Self = Self(128);
    pub const human_B: Self = Self(256);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct quoting_style(pub ::core::ffi::c_uint);
impl quoting_style {
    pub const literal_quoting_style: Self = Self(0);
    pub const shell_quoting_style: Self = Self(1);
    pub const shell_always_quoting_style: Self = Self(2);
    pub const shell_escape_quoting_style: Self = Self(3);
    pub const shell_escape_always_quoting_style: Self = Self(4);
    pub const c_quoting_style: Self = Self(5);
    pub const c_maybe_quoting_style: Self = Self(6);
    pub const escape_quoting_style: Self = Self(7);
    pub const locale_quoting_style: Self = Self(8);
    pub const clocale_quoting_style: Self = Self(9);
    pub const custom_quoting_style: Self = Self(10);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct GetwordEndianState(pub ::core::ffi::c_uint);
impl GetwordEndianState {
    pub const GetwordEndianStateInitial: Self = Self(0);
    pub const GetwordEndianStateNative: Self = Self(1);
    pub const GetwordEndianStateSwab: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct visit_result(pub ::core::ffi::c_uint);
impl visit_result {
    pub const VISIT_CONTINUE: Self = Self(1);
    pub const VISIT_ACCEPTED: Self = Self(2);
    pub const VISIT_REJECTED: Self = Self(4);
    pub const VISIT_ABORT: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ExistenceCheckType(pub ::core::ffi::c_uint);
impl ExistenceCheckType {
    pub const ACCEPT_EITHER: Self = Self(0);
    pub const ACCEPT_EXISTING: Self = Self(1);
    pub const ACCEPT_NON_EXISTING: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locate_limits {
    pub limit: uintmax_t,
    pub items_accepted: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct process_data {
    pub c: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub len: ::core::ffi::c_int,
    pub original_filename: *mut ::core::ffi::c_char,
    pub pathsize: size_t,
    pub munged_filename: *mut ::core::ffi::c_char,
    pub fp: *mut FILE,
    pub dbfile: *const ::core::ffi::c_char,
    pub endian_state: GetwordEndianState,
    pub bigram1: [::core::ffi::c_char; 128],
    pub bigram2: [::core::ffi::c_char; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct locate_stats {
    pub compressed_bytes: uintmax_t,
    pub total_filename_count: uintmax_t,
    pub total_filename_length: uintmax_t,
    pub whitespace_count: uintmax_t,
    pub newline_count: uintmax_t,
    pub highbit_filename_count: uintmax_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const TIME_BUF_LEN: Self = Self(20);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub uch: [::core::ffi::c_uchar; 4],
    pub ui: ::core::ffi::c_uint,
}
pub type processfunc = Option<unsafe extern "C" fn(*mut process_data) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct visitor {
    pub inspector: visitfunc,
    pub context: *mut ::core::ffi::c_void,
    pub next: *mut visitor,
}
pub type visitfunc =
    Option<unsafe extern "C" fn(*mut process_data, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regular_expression {
    pub regex: re_pattern_buffer,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const REGEXTYPE_OPTION: Self = Self(128);
    pub const MAX_DB_AGE: Self = Self(129);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __O_LARGEFILE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_LARGEFILE: ::core::ffi::c_int = __O_LARGEFILE;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const RE_BACKSLASH_ESCAPE_IN_LISTS: ::core::ffi::c_ulong = 1 as ::core::ffi::c_ulong;
pub const RE_BK_PLUS_QM: ::core::ffi::c_ulong =
    RE_BACKSLASH_ESCAPE_IN_LISTS << 1 as ::core::ffi::c_int;
pub const RE_CHAR_CLASSES: ::core::ffi::c_ulong = RE_BK_PLUS_QM << 1 as ::core::ffi::c_int;
pub const RE_CONTEXT_INDEP_ANCHORS: ::core::ffi::c_ulong =
    RE_CHAR_CLASSES << 1 as ::core::ffi::c_int;
pub const RE_CONTEXT_INDEP_OPS: ::core::ffi::c_ulong =
    RE_CONTEXT_INDEP_ANCHORS << 1 as ::core::ffi::c_int;
pub const RE_CONTEXT_INVALID_OPS: ::core::ffi::c_ulong =
    RE_CONTEXT_INDEP_OPS << 1 as ::core::ffi::c_int;
pub const RE_DOT_NEWLINE: ::core::ffi::c_ulong = RE_CONTEXT_INVALID_OPS << 1 as ::core::ffi::c_int;
pub const RE_DOT_NOT_NULL: ::core::ffi::c_ulong = RE_DOT_NEWLINE << 1 as ::core::ffi::c_int;
pub const RE_HAT_LISTS_NOT_NEWLINE: ::core::ffi::c_ulong =
    RE_DOT_NOT_NULL << 1 as ::core::ffi::c_int;
pub const RE_INTERVALS: ::core::ffi::c_ulong = RE_HAT_LISTS_NOT_NEWLINE << 1 as ::core::ffi::c_int;
pub const RE_LIMITED_OPS: ::core::ffi::c_ulong = RE_INTERVALS << 1 as ::core::ffi::c_int;
pub const RE_NEWLINE_ALT: ::core::ffi::c_ulong = RE_LIMITED_OPS << 1 as ::core::ffi::c_int;
pub const RE_NO_BK_BRACES: ::core::ffi::c_ulong = RE_NEWLINE_ALT << 1 as ::core::ffi::c_int;
pub const RE_NO_BK_PARENS: ::core::ffi::c_ulong = RE_NO_BK_BRACES << 1 as ::core::ffi::c_int;
pub const RE_NO_BK_REFS: ::core::ffi::c_ulong = RE_NO_BK_PARENS << 1 as ::core::ffi::c_int;
pub const RE_NO_BK_VBAR: ::core::ffi::c_ulong = RE_NO_BK_REFS << 1 as ::core::ffi::c_int;
pub const RE_NO_EMPTY_RANGES: ::core::ffi::c_ulong = RE_NO_BK_VBAR << 1 as ::core::ffi::c_int;
pub const RE_UNMATCHED_RIGHT_PAREN_ORD: ::core::ffi::c_ulong =
    RE_NO_EMPTY_RANGES << 1 as ::core::ffi::c_int;
pub const RE_NO_POSIX_BACKTRACKING: ::core::ffi::c_ulong =
    RE_UNMATCHED_RIGHT_PAREN_ORD << 1 as ::core::ffi::c_int;
pub const RE_NO_GNU_OPS: ::core::ffi::c_ulong = RE_NO_POSIX_BACKTRACKING << 1 as ::core::ffi::c_int;
pub const RE_DEBUG: ::core::ffi::c_ulong = RE_NO_GNU_OPS << 1 as ::core::ffi::c_int;
pub const RE_INVALID_INTERVAL_ORD: ::core::ffi::c_ulong = RE_DEBUG << 1 as ::core::ffi::c_int;
pub const RE_ICASE: ::core::ffi::c_ulong = RE_INVALID_INTERVAL_ORD << 1 as ::core::ffi::c_int;
pub const RE_SYNTAX_EMACS: ::core::ffi::c_ulong = RE_CHAR_CLASSES | RE_INTERVALS;
pub const SIGKILL: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn rpl_realloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    return realloc(ptr, if size != 0 { size } else { 1 as size_t });
}
pub const FNM_CASEFOLD: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const LOCATEDB_MAGIC: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\0LOCATE02\0") };
pub const LOCATEDB_ESCAPE: ::core::ffi::c_int = 0x80 as ::core::ffi::c_int;
pub const LOCATEDB_OLD_ESCAPE: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const LOCATEDB_OLD_OFFSET: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const SLOCATE_DB_MAGIC_LEN: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
static mut warn_number_units: ::core::ffi::c_uint = 8 as ::core::ffi::c_uint;
static mut warn_name_units: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"days\0") };
pub const SECONDS_PER_UNIT: ::core::ffi::c_int =
    60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int * 24 as ::core::ffi::c_int;
static mut check_existence: ExistenceCheckType = ExistenceCheckType::ACCEPT_EITHER;
static mut follow_symlinks: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut separator: ::core::ffi::c_int = '\n' as ::core::ffi::c_int;
static mut quote_opts: *mut quoting_options = ::core::ptr::null_mut::<quoting_options>();
static mut stdout_is_a_tty: bool = false;
static mut print_quoted_filename: bool = false;
static mut results_were_filtered: bool = false;
static mut selected_secure_db: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
unsafe extern "C" fn set_max_db_age(mut s: *const ::core::ffi::c_char) {
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_ulong = 0;
    if 0 as ::core::ffi::c_int == *s as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"The argument for option --max-database-age must not be empty\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"The argument for option --max-database-age must not be empty\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    val = strtoul(s, &raw mut end, 10 as ::core::ffi::c_int);
    if ULONG_MAX == val && ERANGE == *__errno_location()
        || 0 as ::core::ffi::c_ulong == val && EINVAL == *__errno_location()
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Invalid argument %s for option --max-database-age\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                quotearg_n_style(
                    0 as ::core::ffi::c_int,
                    quoting_style::locale_quoting_style,
                    s,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid argument %s for option --max-database-age\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        s,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else if *end != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Invalid argument %s for option --max-database-age\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                quotearg_n_style(
                    0 as ::core::ffi::c_int,
                    quoting_style::locale_quoting_style,
                    s,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid argument %s for option --max-database-age\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        s,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else {
        warn_number_units = val as ::core::ffi::c_uint;
    };
}
unsafe extern "C" fn get_short(mut fp: *mut FILE) -> ::core::ffi::c_short {
    let mut x: ::core::ffi::c_short = 0;
    x = ((fgetc(fp) as ::core::ffi::c_schar as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as ::core::ffi::c_short;
    x = (x as ::core::ffi::c_int | fgetc(fp) & 0xff as ::core::ffi::c_int) as ::core::ffi::c_short;
    return x;
}
static mut metacharacters: *const ::core::ffi::c_char =
    b"*?[]\\\0".as_ptr() as *const ::core::ffi::c_char;
unsafe extern "C" fn contains_metacharacter(
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if (strpbrk(s, metacharacters) as *const ::core::ffi::c_char).is_null() {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn locate_read_str(
    mut buf: *mut *mut ::core::ffi::c_char,
    mut siz: *mut size_t,
    mut fp: *mut FILE,
    mut delimiter: ::core::ffi::c_int,
    mut offs: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sz: size_t = 0 as size_t;
    let mut nread: ::core::ffi::c_int = 0;
    let mut needed: size_t = 0;
    nread = getdelim(&raw mut p, &raw mut sz, delimiter, fp) as ::core::ffi::c_int;
    if nread >= 0 as ::core::ffi::c_int {
        '_c2rust_label: {
            if !p.is_null() {
            } else {
                __assert_fail(
                    b"p != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    243 as ::core::ffi::c_uint,
                    b"int locate_read_str(char **, size_t *, FILE *, int, int)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        needed = ((offs + nread) as ::core::ffi::c_uint).wrapping_add(1 as ::core::ffi::c_uint)
            as size_t;
        if needed > *siz {
            let mut pnew: *mut ::core::ffi::c_char =
                rpl_realloc(*buf as *mut ::core::ffi::c_void, needed) as *mut ::core::ffi::c_char;
            if pnew.is_null() {
                return -1 as ::core::ffi::c_int;
            } else {
                *siz = needed;
                *buf = pnew;
            }
        }
        memcpy(
            (*buf).offset(offs as isize) as *mut ::core::ffi::c_void,
            p as *const ::core::ffi::c_void,
            (nread + 1 as ::core::ffi::c_int) as size_t,
        );
        free(p as *mut ::core::ffi::c_void);
    }
    return nread;
}
static mut limits: locate_limits = locate_limits {
    limit: 0,
    items_accepted: 0,
};
static mut statistics: locate_stats = locate_stats {
    compressed_bytes: 0,
    total_filename_count: 0,
    total_filename_length: 0,
    whitespace_count: 0,
    newline_count: 0,
    highbit_filename_count: 0,
};
static mut inspectors: *mut visitor = ::core::ptr::null_mut::<visitor>();
static mut lastinspector: *mut visitor = ::core::ptr::null_mut::<visitor>();
static mut past_pat_inspector: *mut visitor = ::core::ptr::null_mut::<visitor>();
#[inline]
unsafe extern "C" fn visit(
    mut p: *const visitor,
    mut accept_flags: ::core::ffi::c_int,
    mut procdata: *mut process_data,
    stop: *const visitor,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = accept_flags;
    while accept_flags & result != 0 && stop != p {
        result = (*p).inspector.expect("non-null function pointer")(procdata, (*p).context);
        p = (*p).next;
    }
    return result;
}
unsafe extern "C" fn process_simple(mut procdata: *mut process_data) -> ::core::ffi::c_int {
    return visit(
        inspectors,
        visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int
            | visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int,
        procdata,
        ::core::ptr::null::<visitor>(),
    );
}
unsafe extern "C" fn process_or(mut procdata: *mut process_data) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = visit(
        inspectors,
        visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int
            | visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int,
        procdata,
        past_pat_inspector,
    );
    if result == visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int {
        result = visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    }
    if result
        & (visit_result::VISIT_ABORT.0 as ::core::ffi::c_int
            | visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int)
        != 0
    {
        return result;
    }
    result = visit(
        past_pat_inspector,
        visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int,
        procdata,
        ::core::ptr::null::<visitor>(),
    );
    if visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int == result {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    } else {
        return result;
    };
}
unsafe extern "C" fn process_and(mut procdata: *mut process_data) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = 0;
    result = visit(
        inspectors,
        visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int
            | visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int,
        procdata,
        past_pat_inspector,
    );
    if result == visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int {
        result = visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    }
    if result
        & (visit_result::VISIT_ABORT.0 as ::core::ffi::c_int
            | visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int)
        != 0
    {
        return result;
    }
    result = visit(
        past_pat_inspector,
        visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int,
        procdata,
        ::core::ptr::null::<visitor>(),
    );
    if visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int == result {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    } else {
        return result;
    };
}
static mut mainprocessor: processfunc = None;
unsafe extern "C" fn add_visitor(mut r#fn: visitfunc, mut context: *mut ::core::ffi::c_void) {
    let mut p: *mut visitor = xmalloc(::core::mem::size_of::<visitor>()) as *mut visitor;
    (*p).inspector = r#fn;
    (*p).context = context;
    (*p).next = ::core::ptr::null_mut::<visitor>();
    if lastinspector.is_null() {
        inspectors = p;
        lastinspector = inspectors;
    } else {
        (*lastinspector).next = p;
        lastinspector = p;
    };
}
unsafe extern "C" fn visit_justprint_quoted(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    print_quoted(
        stdout,
        quote_opts,
        stdout_is_a_tty,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        (*procdata).original_filename,
    );
    putchar(separator);
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn visit_justprint_unquoted(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    fputs((*procdata).original_filename, stdout);
    putchar(separator);
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn toolong(mut procdata: *mut process_data) {
    if 0 != 0 {
        error(
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"locate database %s contains a filename longer than locate can handle\0".as_ptr()
                    as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            ),
            (*procdata).dbfile,
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
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"locate database %s contains a filename longer than locate can handle\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                (*procdata).dbfile,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
}
unsafe extern "C" fn extend(mut procdata: *mut process_data, mut siz1: size_t, mut siz2: size_t) {
    if (SIZE_MAX as size_t).wrapping_sub(siz1) < siz2 {
        toolong(procdata);
    } else if (*procdata).pathsize < siz1.wrapping_add(siz2) {
        (*procdata).pathsize = siz1.wrapping_add(siz2);
        (*procdata).original_filename = x2nrealloc(
            (*procdata).original_filename as *mut ::core::ffi::c_void,
            &raw mut (*procdata).pathsize,
            1 as size_t,
        ) as *mut ::core::ffi::c_char;
    }
}
unsafe extern "C" fn visit_old_format(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut i: size_t = 0;
    if EOF == (*procdata).c {
        return visit_result::VISIT_ABORT.0 as ::core::ffi::c_int;
    }
    if (*procdata).c == LOCATEDB_OLD_ESCAPE {
        let mut minval: ::core::ffi::c_int = 0;
        let mut maxval: ::core::ffi::c_int = 0;
        let mut word: ::core::ffi::c_int = 0;
        (*procdata).count -= LOCATEDB_OLD_OFFSET;
        minval = 0 as ::core::ffi::c_int - (*procdata).count;
        if (*procdata).count >= 0 as ::core::ffi::c_int {
            maxval = (*procdata).len - (*procdata).count;
        } else {
            maxval = (*procdata).len - 0 as ::core::ffi::c_int;
        }
        word = getword(
            (*procdata).fp,
            (*procdata).dbfile,
            maxval as size_t,
            &raw mut (*procdata).endian_state,
        );
        '_c2rust_label: {
            if word >= minval {
            } else {
                __assert_fail(
                    b"word >= minval\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    476 as ::core::ffi::c_uint,
                    b"int visit_old_format(struct process_data *, void *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*procdata).count += word;
        '_c2rust_label_0: {
            if (*procdata).count >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"procdata->count >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    478 as ::core::ffi::c_uint,
                    b"int visit_old_format(struct process_data *, void *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
    } else {
        (*procdata).count += (*procdata).c - LOCATEDB_OLD_OFFSET;
        '_c2rust_label_1: {
            if (*procdata).count >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"procdata->count >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    483 as ::core::ffi::c_uint,
                    b"int visit_old_format(struct process_data *, void *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
    }
    i = (*procdata).count as size_t;
    loop {
        (*procdata).c = getc((*procdata).fp);
        if (*procdata).c <= LOCATEDB_OLD_ESCAPE {
            break;
        }
        if EOF == (*procdata).c {
            break;
        }
        if (*procdata).c < 0o200 as ::core::ffi::c_int {
            extend(procdata, i, 1 as size_t);
            let c2rust_fresh0 = i;
            i = i.wrapping_add(1);
            *(*procdata).original_filename.offset(c2rust_fresh0 as isize) =
                (*procdata).c as ::core::ffi::c_char;
        } else {
            extend(procdata, i, 2 as size_t);
            (*procdata).c &= 0o177 as ::core::ffi::c_int;
            let c2rust_fresh1 = i;
            i = i.wrapping_add(1);
            *(*procdata).original_filename.offset(c2rust_fresh1 as isize) =
                (*procdata).bigram1[(*procdata).c as usize];
            let c2rust_fresh2 = i;
            i = i.wrapping_add(1);
            *(*procdata).original_filename.offset(c2rust_fresh2 as isize) =
                (*procdata).bigram2[(*procdata).c as usize];
        }
    }
    extend(procdata, i, 1 as size_t);
    *(*procdata).original_filename.offset(i as isize) = 0 as ::core::ffi::c_char;
    (*procdata).len = i as ::core::ffi::c_int;
    (*procdata).munged_filename = (*procdata).original_filename;
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn visit_locate02_format(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut nread: ::core::ffi::c_int = 0;
    if (*procdata).c == LOCATEDB_ESCAPE {
        (*procdata).count += get_short((*procdata).fp) as ::core::ffi::c_int;
    } else if (*procdata).c > 127 as ::core::ffi::c_int {
        (*procdata).count += (*procdata).c - 256 as ::core::ffi::c_int;
    } else {
        (*procdata).count += (*procdata).c;
    }
    if (*procdata).count > (*procdata).len || (*procdata).count < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"locate database %s is corrupt or invalid\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                quotearg_n_style(
                    0 as ::core::ffi::c_int,
                    quoting_style::locale_quoting_style,
                    (*procdata).dbfile,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"locate database %s is corrupt or invalid\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        (*procdata).dbfile,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    nread = locate_read_str(
        &raw mut (*procdata).original_filename,
        &raw mut (*procdata).pathsize,
        (*procdata).fp,
        0 as ::core::ffi::c_int,
        (*procdata).count,
    );
    if nread < 1 as ::core::ffi::c_int {
        return visit_result::VISIT_ABORT.0 as ::core::ffi::c_int;
    }
    (*procdata).c = getc((*procdata).fp);
    (*procdata).len = (*procdata).count + nread - 1 as ::core::ffi::c_int;
    if (*procdata).len < 1 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"locate database %s is corrupt or invalid\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                quotearg_n_style(
                    0 as ::core::ffi::c_int,
                    quoting_style::locale_quoting_style,
                    (*procdata).dbfile,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"locate database %s is corrupt or invalid\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        (*procdata).dbfile,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    s = (*procdata)
        .original_filename
        .offset((*procdata).len as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    '_c2rust_label_1: {
        if *s.offset(0isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"s[0] != '\\0'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                566 as ::core::ffi::c_uint,
                b"int visit_locate02_format(struct process_data *, void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if *s.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"s[1] == '\\0'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                567 as ::core::ffi::c_uint,
                b"int visit_locate02_format(struct process_data *, void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if *s.offset(2isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"s[2] == '\\0'\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                568 as ::core::ffi::c_uint,
                b"int visit_locate02_format(struct process_data *, void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*procdata).munged_filename = (*procdata).original_filename;
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn visit_basename(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    (*procdata).munged_filename = last_component((*procdata).original_filename);
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn visit_existing_follow(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if stat((*procdata).original_filename, &raw mut st) != 0 as ::core::ffi::c_int {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_non_existing_follow(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if stat((*procdata).original_filename, &raw mut st) == 0 as ::core::ffi::c_int {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_existing_nofollow(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if lstat((*procdata).original_filename, &raw mut st) != 0 as ::core::ffi::c_int {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_non_existing_nofollow(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut st: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    if lstat((*procdata).original_filename, &raw mut st) == 0 as ::core::ffi::c_int {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_substring_match_nocasefold_wide(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pattern: *const ::core::ffi::c_char = context as *const ::core::ffi::c_char;
    if !mbsstr((*procdata).munged_filename, pattern).is_null() {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_substring_match_nocasefold_narrow(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pattern: *const ::core::ffi::c_char = context as *const ::core::ffi::c_char;
    '_c2rust_label: {
        if __ctype_get_mb_cur_max() == 1 as size_t {
        } else {
            __assert_fail(
                b"MB_CUR_MAX == 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                688 as ::core::ffi::c_uint,
                b"int visit_substring_match_nocasefold_narrow(struct process_data *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if !strstr((*procdata).munged_filename, pattern).is_null() {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_substring_match_casefold_wide(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pattern: *const ::core::ffi::c_char = context as *const ::core::ffi::c_char;
    if !mbscasestr((*procdata).munged_filename, pattern).is_null() {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_substring_match_casefold_narrow(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pattern: *const ::core::ffi::c_char = context as *const ::core::ffi::c_char;
    '_c2rust_label: {
        if __ctype_get_mb_cur_max() == 1 as size_t {
        } else {
            __assert_fail(
                b"MB_CUR_MAX == 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                712 as ::core::ffi::c_uint,
                b"int visit_substring_match_casefold_narrow(struct process_data *, void *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if !strcasestr((*procdata).munged_filename, pattern).is_null() {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_globmatch_nofold(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut glob: *const ::core::ffi::c_char = context as *const ::core::ffi::c_char;
    if fnmatch(glob, (*procdata).munged_filename, 0 as ::core::ffi::c_int)
        != 0 as ::core::ffi::c_int
    {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_globmatch_casefold(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut glob: *const ::core::ffi::c_char = context as *const ::core::ffi::c_char;
    if fnmatch(glob, (*procdata).munged_filename, FNM_CASEFOLD) != 0 as ::core::ffi::c_int {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_regex(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut regular_expression = context as *mut regular_expression;
    let len: size_t = strlen((*procdata).munged_filename);
    let mut rv: ::core::ffi::c_int = rpl_re_search(
        &raw mut (*p).regex,
        (*procdata).munged_filename,
        len as regoff_t,
        0 as regoff_t,
        len as regoff_t,
        NULL as *mut re_registers,
    ) as ::core::ffi::c_int;
    if rv < 0 as ::core::ffi::c_int {
        return visit_result::VISIT_REJECTED.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_ACCEPTED.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_stats(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut locate_stats = context as *mut locate_stats;
    let mut len: size_t = strlen((*procdata).original_filename);
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut highbit: ::core::ffi::c_int = 0;
    let mut whitespace: ::core::ffi::c_int = 0;
    let mut newline: ::core::ffi::c_int = 0;
    (*p).total_filename_count = (*p).total_filename_count.wrapping_add(1);
    (*p).total_filename_length = ((*p).total_filename_length as ::core::ffi::c_ulong)
        .wrapping_add(len as ::core::ffi::c_ulong) as uintmax_t;
    newline = 0 as ::core::ffi::c_int;
    whitespace = newline;
    highbit = whitespace;
    s = (*procdata).original_filename;
    while *s != 0 {
        if *s as ::core::ffi::c_int & 128 as ::core::ffi::c_int != 0 {
            highbit = 1 as ::core::ffi::c_int;
        }
        if '\n' as ::core::ffi::c_int == *s as ::core::ffi::c_int {
            whitespace = 1 as ::core::ffi::c_int;
            newline = whitespace;
        } else if *(*__ctype_b_loc())
            .offset(*s as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            whitespace = 1 as ::core::ffi::c_int;
        }
        s = s.offset(1);
    }
    if highbit != 0 {
        (*p).highbit_filename_count = (*p).highbit_filename_count.wrapping_add(1);
    }
    if whitespace != 0 {
        (*p).whitespace_count = (*p).whitespace_count.wrapping_add(1);
    }
    if newline != 0 {
        (*p).newline_count = (*p).newline_count.wrapping_add(1);
    }
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn visit_limit(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut locate_limits = context as *mut locate_limits;
    (*p).items_accepted = (*p).items_accepted.wrapping_add(1);
    if (*p).items_accepted >= (*p).limit {
        return visit_result::VISIT_ABORT.0 as ::core::ffi::c_int;
    } else {
        return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn visit_count(
    mut procdata: *mut process_data,
    mut context: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut p: *mut locate_limits = context as *mut locate_limits;
    (*p).items_accepted = (*p).items_accepted.wrapping_add(1);
    return visit_result::VISIT_CONTINUE.0 as ::core::ffi::c_int;
}
unsafe extern "C" fn print_stats(
    mut argc: ::core::ffi::c_int,
    mut database_file_size: size_t,
    mut database_mtime: *const timespec,
) {
    let mut hbuf1: [::core::ffi::c_char; 652] = [0; 652];
    let mut hbuf2: [::core::ffi::c_char; 652] = [0; 652];
    let mut hbuf3: [::core::ffi::c_char; 652] = [0; 652];
    let mut hbuf4: [::core::ffi::c_char; 652] = [0; 652];
    if !database_mtime.is_null() {
        let mut ptm: *const tm = localtime(&raw const (*database_mtime).tv_sec);
        if !ptm.is_null() {
            let mut whenbuf: [::core::ffi::c_char; 20] = [0; 20];
            let mut printed: size_t = strftime(
                &raw mut whenbuf as *mut ::core::ffi::c_char,
                C2Rust_Unnamed_1::TIME_BUF_LEN.0 as ::core::ffi::c_int as size_t,
                b"%Y:%m:%d %H:%M:%S\0".as_ptr() as *const ::core::ffi::c_char,
                ptm,
            );
            '_c2rust_label: {
                if printed
                    == (C2Rust_Unnamed_1::TIME_BUF_LEN.0 as ::core::ffi::c_int
                        - 1 as ::core::ffi::c_int) as size_t
                {
                } else {
                    __assert_fail(
                        b"printed == TIME_BUF_LEN-1\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        843 as ::core::ffi::c_uint,
                        b"void print_stats(int, size_t, const struct timespec *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_0: {
                if whenbuf[(C2Rust_Unnamed_1::TIME_BUF_LEN.0 as ::core::ffi::c_int
                    - 1 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"whenbuf[TIME_BUF_LEN-1] == 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        844 as ::core::ffi::c_uint,
                        b"void print_stats(int, size_t, const struct timespec *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            '_c2rust_label_1: {
                if whenbuf[(C2Rust_Unnamed_1::TIME_BUF_LEN.0 as ::core::ffi::c_int
                    - 2 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                } else {
                    __assert_fail(
                        b"whenbuf[TIME_BUF_LEN-2] != 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        845 as ::core::ffi::c_uint,
                        b"void print_stats(int, size_t, const struct timespec *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            printf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Database was last modified at %s.%09ld\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                &raw mut whenbuf as *mut ::core::ffi::c_char,
                (*database_mtime).tv_nsec,
            );
            printed = strftime(
                &raw mut whenbuf as *mut ::core::ffi::c_char,
                C2Rust_Unnamed_1::TIME_BUF_LEN.0 as ::core::ffi::c_int as size_t,
                b"%z\0".as_ptr() as *const ::core::ffi::c_char,
                ptm,
            );
            '_c2rust_label_2: {
                if printed == 5 as size_t {
                } else {
                    __assert_fail(
                        b"printed == 5\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        849 as ::core::ffi::c_uint,
                        b"void print_stats(int, size_t, const struct timespec *)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            printf(
                b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut whenbuf as *mut ::core::ffi::c_char,
            );
        }
    }
    printf(
        dcngettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Locate database size: %s byte\n\0".as_ptr() as *const ::core::ffi::c_char,
            b"Locate database size: %s bytes\n\0".as_ptr() as *const ::core::ffi::c_char,
            database_file_size as ::core::ffi::c_ulong,
            LC_MESSAGES,
        ),
        human_readable(
            database_file_size as uintmax_t,
            &raw mut hbuf1 as *mut ::core::ffi::c_char,
            C2Rust_Unnamed_0::human_ceiling.0 as ::core::ffi::c_int,
            1 as uintmax_t,
            1 as uintmax_t,
        ),
    );
    printf(
        if results_were_filtered as ::core::ffi::c_int != 0 {
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Matching Filenames: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            )
        } else {
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"All Filenames: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            )
        },
        human_readable(
            statistics.total_filename_count,
            &raw mut hbuf1 as *mut ::core::ffi::c_char,
            C2Rust_Unnamed_0::human_ceiling.0 as ::core::ffi::c_int,
            1 as uintmax_t,
            1 as uintmax_t,
        ),
    );
    printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"File names have a cumulative length of %s bytes.\nOf those file names,\n\n\t%s contain whitespace, \n\t%s contain newline characters, \n\tand %s contain characters with the high bit set.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        human_readable(
            statistics.total_filename_length,
            &raw mut hbuf1 as *mut ::core::ffi::c_char,
            C2Rust_Unnamed_0::human_ceiling.0 as ::core::ffi::c_int,
            1 as uintmax_t,
            1 as uintmax_t,
        ),
        human_readable(
            statistics.whitespace_count,
            &raw mut hbuf2 as *mut ::core::ffi::c_char,
            C2Rust_Unnamed_0::human_ceiling.0 as ::core::ffi::c_int,
            1 as uintmax_t,
            1 as uintmax_t,
        ),
        human_readable(
            statistics.newline_count,
            &raw mut hbuf3 as *mut ::core::ffi::c_char,
            C2Rust_Unnamed_0::human_ceiling.0 as ::core::ffi::c_int,
            1 as uintmax_t,
            1 as uintmax_t,
        ),
        human_readable(
            statistics.highbit_filename_count,
            &raw mut hbuf4 as *mut ::core::ffi::c_char,
            C2Rust_Unnamed_0::human_ceiling.0 as ::core::ffi::c_int,
            1 as uintmax_t,
            1 as uintmax_t,
        ),
    );
    if argc == 0 {
        if results_were_filtered {
            printf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Some filenames may have been filtered out, so we cannot compute the compression ratio.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        } else if statistics.total_filename_length != 0 {
            printf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Compression ratio %4.2f%% (higher is better)\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                100.0f64
                    * (statistics.total_filename_length as ::core::ffi::c_double
                        - database_file_size as ::core::ffi::c_double)
                    / statistics.total_filename_length as ::core::ffi::c_double,
            );
        } else {
            printf(dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Compression ratio is undefined\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ));
        }
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn looking_at_gnu_locatedb(
    mut data: *const ::core::ffi::c_char,
    mut len: size_t,
) -> ::core::ffi::c_int {
    if len < ::core::mem::size_of::<[::core::ffi::c_char; 10]>() {
        return 0 as ::core::ffi::c_int;
    } else if 0 as ::core::ffi::c_int
        == memcmp(
            data as *const ::core::ffi::c_void,
            LOCATEDB_MAGIC.as_ptr() as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[::core::ffi::c_char; 10]>(),
        )
    {
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn looking_at_slocate_locatedb(
    mut filename: *const ::core::ffi::c_char,
    mut data: *const ::core::ffi::c_char,
    mut len: size_t,
    mut seclevel: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if len <= 2 as size_t {
        } else {
            __assert_fail(
                b"len <= 2\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                    as *const ::core::ffi::c_char,
                933 as ::core::ffi::c_uint,
                b"int looking_at_slocate_locatedb(const char *, const char *, size_t, int *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if len < 2 as size_t {
        return 0 as ::core::ffi::c_int;
    } else if 0 as ::core::ffi::c_int == *data.offset(1isize) as ::core::ffi::c_int {
        if *(*__ctype_b_loc())
            .offset(*data.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
        {
            *seclevel = *data.offset(0isize) as ::core::ffi::c_int - '0' as ::core::ffi::c_int;
            if *seclevel > 1 as ::core::ffi::c_int {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"locate database %s looks like an slocate database but it seems to have security level %c, which GNU findutils does not currently support\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            filename,
                        ),
                        *data.offset(1isize) as ::core::ffi::c_int,
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
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"locate database %s looks like an slocate database but it seems to have security level %c, which GNU findutils does not currently support\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                filename,
                            ),
                            *data.offset(1isize) as ::core::ffi::c_int,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                return 1 as ::core::ffi::c_int;
            } else {
                return 1 as ::core::ffi::c_int;
            }
        } else {
            return 0 as ::core::ffi::c_int;
        }
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn i_am_little_endian() -> ::core::ffi::c_int {
    let mut u: C2Rust_Unnamed_2 = C2Rust_Unnamed_2 { uch: [0; 4] };
    u.ui = 0 as ::core::ffi::c_uint;
    u.uch[0usize] = 1 as ::core::ffi::c_uchar;
    u.uch[3usize] = 0 as ::core::ffi::c_uchar;
    u.uch[2usize] = u.uch[3usize];
    u.uch[1usize] = u.uch[2usize];
    return (u.ui == 1 as ::core::ffi::c_uint) as ::core::ffi::c_int;
}
unsafe extern "C" fn search_one_database(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut dbfile: *const ::core::ffi::c_char,
    mut fp: *mut FILE,
    mut filesize: off_t,
    mut database_mtime: *const timespec,
    mut ignore_case: ::core::ffi::c_int,
    mut enable_print: ::core::ffi::c_int,
    mut basename_only: ::core::ffi::c_int,
    mut use_limit: ::core::ffi::c_int,
    mut plimit: *mut locate_limits,
    mut stats: ::core::ffi::c_int,
    mut op_and: ::core::ffi::c_int,
    mut regex: ::core::ffi::c_int,
    mut regex_options: ::core::ffi::c_int,
) -> ::core::ffi::c_ulong {
    let mut pathpart: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut argn: ::core::ffi::c_int = 0;
    let mut nread: ::core::ffi::c_int = 0;
    rboxc_release_locate_path();
    RBOXC_PROCDATA = process_data {
        c: 0,
        count: 0,
        len: 0,
        original_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pathsize: 0,
        munged_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fp: ::core::ptr::null_mut::<FILE>(),
        dbfile: ::core::ptr::null::<::core::ffi::c_char>(),
        endian_state: GetwordEndianState::GetwordEndianStateInitial,
        bigram1: [0; 128],
        bigram2: [0; 128],
    };
    let mut slocate_seclevel: ::core::ffi::c_int = 0;
    let mut oldformat: ::core::ffi::c_int = 0;
    let mut slocatedb_format: ::core::ffi::c_int = 0;
    let mut pvis: *mut visitor = ::core::ptr::null_mut::<visitor>();
    let mut format_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut do_check_existence: ExistenceCheckType = ExistenceCheckType::ACCEPT_EITHER;
    do_check_existence = check_existence;
    if ignore_case != 0 {
        regex_options = (regex_options as ::core::ffi::c_ulong | RE_ICASE) as ::core::ffi::c_int;
    }
    oldformat = 0 as ::core::ffi::c_int;
    RBOXC_PROCDATA.endian_state = GetwordEndianState::GetwordEndianStateInitial;
    RBOXC_PROCDATA.count = 0 as ::core::ffi::c_int;
    RBOXC_PROCDATA.len = RBOXC_PROCDATA.count;
    RBOXC_PROCDATA.dbfile = dbfile;
    RBOXC_PROCDATA.fp = fp;
    inspectors = ::core::ptr::null_mut::<visitor>();
    lastinspector = ::core::ptr::null_mut::<visitor>();
    past_pat_inspector = ::core::ptr::null_mut::<visitor>();
    results_were_filtered = r#false != 0;
    RBOXC_PROCDATA.pathsize = 128 as size_t;
    RBOXC_PROCDATA.original_filename = xmalloc(RBOXC_PROCDATA.pathsize) as *mut ::core::ffi::c_char;
    nread = fread(
        RBOXC_PROCDATA.original_filename as *mut ::core::ffi::c_void,
        1 as size_t,
        SLOCATE_DB_MAGIC_LEN as size_t,
        RBOXC_PROCDATA.fp,
    ) as ::core::ffi::c_int;
    slocate_seclevel = 0 as ::core::ffi::c_int;
    if looking_at_slocate_locatedb(
        RBOXC_PROCDATA.dbfile,
        RBOXC_PROCDATA.original_filename,
        nread as size_t,
        &raw mut slocate_seclevel,
    ) != 0
    {
        if slocate_seclevel > 1 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is an slocate database of unsupported security level %d; skipping it.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        RBOXC_PROCDATA.dbfile,
                    ),
                    slocate_seclevel,
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
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s is an slocate database of unsupported security level %d; skipping it.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            RBOXC_PROCDATA.dbfile,
                        ),
                        slocate_seclevel,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            return 0 as ::core::ffi::c_ulong;
        } else if slocate_seclevel > 0 as ::core::ffi::c_int {
            if ExistenceCheckType::ACCEPT_NON_EXISTING.0 == check_existence.0 {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"You specified the -E option, but that option cannot be used with slocate-format databases with a non-zero security level.  No results will be generated for this database.\n\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
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
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"You specified the -E option, but that option cannot be used with slocate-format databases with a non-zero security level.  No results will be generated for this database.\n\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                return 0 as ::core::ffi::c_ulong;
            }
            if ExistenceCheckType::ACCEPT_EXISTING.0 != do_check_existence.0 {
                if enable_print != 0 || stats != 0 {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"%s is an slocate database.  Turning on the '-e' option.\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                RBOXC_PROCDATA.dbfile,
                            ),
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
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"%s is an slocate database.  Turning on the '-e' option.\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                quotearg_n_style(
                                    0 as ::core::ffi::c_int,
                                    quoting_style::locale_quoting_style,
                                    RBOXC_PROCDATA.dbfile,
                                ),
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
                do_check_existence = ExistenceCheckType::ACCEPT_EXISTING;
            }
        }
        add_visitor(
            Some(
                visit_locate02_format
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
        format_name = b"slocate\0".as_ptr() as *const ::core::ffi::c_char;
        slocatedb_format = 1 as ::core::ffi::c_int;
    } else {
        let mut nread2: ::core::ffi::c_int = 0;
        slocatedb_format = 0 as ::core::ffi::c_int;
        extend(
            &raw mut RBOXC_PROCDATA,
            ::core::mem::size_of::<[::core::ffi::c_char; 10]>(),
            0 as size_t,
        );
        nread2 = fread(
            RBOXC_PROCDATA.original_filename.offset(nread as isize) as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[::core::ffi::c_char; 10]>().wrapping_sub(nread as size_t),
            RBOXC_PROCDATA.fp,
        ) as ::core::ffi::c_int;
        if looking_at_gnu_locatedb(RBOXC_PROCDATA.original_filename, (nread + nread2) as size_t) != 0 {
            add_visitor(
                Some(
                    visit_locate02_format
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                NULL,
            );
            format_name = b"GNU LOCATE02\0".as_ptr() as *const ::core::ffi::c_char;
        } else {
            let mut i: ::core::ffi::c_int = 0;
            nread += nread2;
            extend(&raw mut RBOXC_PROCDATA, 256 as size_t, 0 as size_t);
            if nread < 256 as ::core::ffi::c_int {
                let mut more_read: ::core::ffi::c_int = fread(
                    RBOXC_PROCDATA.original_filename.offset(nread as isize) as *mut ::core::ffi::c_void,
                    1 as size_t,
                    (256 as ::core::ffi::c_int - nread) as size_t,
                    RBOXC_PROCDATA.fp,
                ) as ::core::ffi::c_int;
                if more_read + nread != 256 as ::core::ffi::c_int {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"Old-format locate database %s is too short to be valid\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                dbfile,
                            ),
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
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Old-format locate database %s is too short to be valid\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                quotearg_n_style(
                                    0 as ::core::ffi::c_int,
                                    quoting_style::locale_quoting_style,
                                    dbfile,
                                ),
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
            }
            i = 0 as ::core::ffi::c_int;
            while i < 128 as ::core::ffi::c_int {
                RBOXC_PROCDATA.bigram1[i as usize] = *RBOXC_PROCDATA
                    .original_filename
                    .offset((i << 1 as ::core::ffi::c_int) as isize);
                RBOXC_PROCDATA.bigram2[i as usize] = *RBOXC_PROCDATA
                    .original_filename
                    .offset(((i << 1 as ::core::ffi::c_int) + 1 as ::core::ffi::c_int) as isize);
                i += 1;
            }
            format_name = b"old\0".as_ptr() as *const ::core::ffi::c_char;
            oldformat = 1 as ::core::ffi::c_int;
            add_visitor(
                Some(
                    visit_old_format
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                NULL,
            );
        }
    }
    if basename_only != 0 {
        add_visitor(
            Some(
                visit_basename
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            NULL,
        );
    }
    argn = 0 as ::core::ffi::c_int;
    while argn < argc {
        results_were_filtered = r#true != 0;
        pathpart = *argv.offset(argn as isize);
        if regex != 0 {
            let mut p: *mut regular_expression =
                xmalloc(::core::mem::size_of::<regular_expression>()) as *mut regular_expression;
            let mut error_message: *const ::core::ffi::c_char =
                ::core::ptr::null::<::core::ffi::c_char>();
            memset(
                &raw mut (*p).regex as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<re_pattern_buffer>(),
            );
            RBOXC_REGEXES.push(p);
            rpl_re_set_syntax(regex_options as reg_syntax_t);
            (*p).regex.allocated = 100 as __re_long_size_t;
            (*p).regex.buffer = xmalloc((*p).regex.allocated) as *mut re_dfa_t;
            (*p).regex.fastmap = ::core::ptr::null_mut::<::core::ffi::c_char>();
            (*p).regex.syntax = regex_options as reg_syntax_t;
            (*p).regex.translate = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
            error_message = rpl_re_compile_pattern(pathpart, strlen(pathpart), &raw mut (*p).regex);
            if !error_message.is_null() {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        error_message,
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
                            error_message,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            } else {
                add_visitor(
                    Some(
                        visit_regex
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    p as *mut ::core::ffi::c_void,
                );
            }
        } else if contains_metacharacter(pathpart) != 0 {
            if ignore_case != 0 {
                add_visitor(
                    Some(
                        visit_globmatch_casefold
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    pathpart as *mut ::core::ffi::c_void,
                );
            } else {
                add_visitor(
                    Some(
                        visit_globmatch_nofold
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    pathpart as *mut ::core::ffi::c_void,
                );
            }
        } else {
            let mut matcher: visitfunc = None;
            if 1 as size_t == __ctype_get_mb_cur_max() {
                matcher = (if ignore_case != 0 {
                    Some(
                        visit_substring_match_casefold_narrow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    )
                } else {
                    Some(
                        visit_substring_match_nocasefold_narrow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    )
                }) as visitfunc;
            } else {
                matcher = (if ignore_case != 0 {
                    Some(
                        visit_substring_match_casefold_wide
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    )
                } else {
                    Some(
                        visit_substring_match_nocasefold_wide
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    )
                }) as visitfunc;
            }
            add_visitor(matcher, pathpart as *mut ::core::ffi::c_void);
        }
        argn += 1;
    }
    pvis = lastinspector;
    match do_check_existence {
        ExistenceCheckType::ACCEPT_EXISTING => {
            results_were_filtered = r#true != 0;
            if follow_symlinks != 0 {
                add_visitor(
                    Some(
                        visit_existing_follow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    NULL,
                );
            } else {
                add_visitor(
                    Some(
                        visit_existing_nofollow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    NULL,
                );
            }
        }
        ExistenceCheckType::ACCEPT_NON_EXISTING => {
            results_were_filtered = r#true != 0;
            if follow_symlinks != 0 {
                add_visitor(
                    Some(
                        visit_non_existing_follow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    NULL,
                );
            } else {
                add_visitor(
                    Some(
                        visit_non_existing_nofollow
                            as unsafe extern "C" fn(
                                *mut process_data,
                                *mut ::core::ffi::c_void,
                            )
                                -> ::core::ffi::c_int,
                    ),
                    NULL,
                );
            }
        }
        ExistenceCheckType::ACCEPT_EITHER | _ => {}
    }
    if stats != 0 {
        add_visitor(
            Some(
                visit_stats
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut statistics as *mut ::core::ffi::c_void,
        );
    }
    if enable_print != 0 {
        if print_quoted_filename {
            add_visitor(
                Some(
                    visit_justprint_quoted
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                NULL,
            );
        } else {
            add_visitor(
                Some(
                    visit_justprint_unquoted
                        as unsafe extern "C" fn(
                            *mut process_data,
                            *mut ::core::ffi::c_void,
                        ) -> ::core::ffi::c_int,
                ),
                NULL,
            );
        }
    }
    if use_limit != 0 {
        add_visitor(
            Some(
                visit_limit
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            plimit as *mut ::core::ffi::c_void,
        );
    } else {
        add_visitor(
            Some(
                visit_count
                    as unsafe extern "C" fn(
                        *mut process_data,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            plimit as *mut ::core::ffi::c_void,
        );
    }
    if argc > 1 as ::core::ffi::c_int {
        past_pat_inspector = (*pvis).next;
        if op_and != 0 {
            mainprocessor =
                Some(process_and as unsafe extern "C" fn(*mut process_data) -> ::core::ffi::c_int)
                    as processfunc;
        } else {
            mainprocessor =
                Some(process_or as unsafe extern "C" fn(*mut process_data) -> ::core::ffi::c_int)
                    as processfunc;
        }
    } else {
        mainprocessor =
            Some(process_simple as unsafe extern "C" fn(*mut process_data) -> ::core::ffi::c_int)
                as processfunc;
    }
    if stats != 0 {
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Database %s is in the %s format.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            RBOXC_PROCDATA.dbfile,
            format_name,
        );
    }
    RBOXC_PROCDATA.c = getc(RBOXC_PROCDATA.fp);
    if slocatedb_format != 0 && RBOXC_PROCDATA.c != EOF {
        ungetc(RBOXC_PROCDATA.c, RBOXC_PROCDATA.fp);
        RBOXC_PROCDATA.c = 0 as ::core::ffi::c_int;
    }
    while RBOXC_PROCDATA.c != EOF
        && visit_result::VISIT_ABORT.0 as ::core::ffi::c_int
            != mainprocessor.expect("non-null function pointer")(&raw mut RBOXC_PROCDATA)
    {}
    if stats != 0 {
        if oldformat != 0 {
            let mut host_little_endian: ::core::ffi::c_int = i_am_little_endian();
            let mut little: *const ::core::ffi::c_char = dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"The database has little-endian machine-word encoding.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            );
            let mut big: *const ::core::ffi::c_char = dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"The database has big-endian machine-word encoding.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            );
            if GetwordEndianState::GetwordEndianStateNative.0 == RBOXC_PROCDATA.endian_state.0 {
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if host_little_endian != 0 { little } else { big },
                );
            } else if GetwordEndianState::GetwordEndianStateSwab.0 == RBOXC_PROCDATA.endian_state.0 {
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if host_little_endian != 0 { big } else { little },
                );
            } else {
                printf(dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"The database machine-word encoding order is not obvious.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ));
            }
        }
        if filesize != 0 || !database_mtime.is_null() {
            print_stats(argc, filesize as size_t, database_mtime);
        }
    }
    if ferror(RBOXC_PROCDATA.fp) != 0 {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                quotearg_n_style(
                    0 as ::core::ffi::c_int,
                    quoting_style::locale_quoting_style,
                    RBOXC_PROCDATA.dbfile,
                ),
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
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        RBOXC_PROCDATA.dbfile,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        return 0 as ::core::ffi::c_ulong;
    }
    return (*plimit).items_accepted as ::core::ffi::c_ulong;
}
unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) -> ! {
    if status != EXIT_SUCCESS {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [-d path | --database=path] [-e | -E | --[non-]existing]\n      [-i | --ignore-case] [-w | --wholename] [-b | --basename] \n      [--limit=N | -l N] [-S | --statistics] [-0 | --null] [-c | --count]\n      [-P | -H | --nofollow] [-L | --follow] [-m | --mmap] [-s | --stdio]\n      [-A | --all] [-p | --print] [-r | --regex] [--regextype=TYPE]\n      [--max-database-age D] [--version] [--help]\n      pattern...\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        program_name,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
static mut longopts: [option; 23] = [
    option {
        name: b"database\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"existing\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'e' as ::core::ffi::c_int,
    },
    option {
        name: b"non-existing\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'E' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-case\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'i' as ::core::ffi::c_int,
    },
    option {
        name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'A' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'h' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"null\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: '0' as ::core::ffi::c_int,
    },
    option {
        name: b"count\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"wholename\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'w' as ::core::ffi::c_int,
    },
    option {
        name: b"wholepath\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'w' as ::core::ffi::c_int,
    },
    option {
        name: b"basename\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"print\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"stdio\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"mmap\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'm' as ::core::ffi::c_int,
    },
    option {
        name: b"limit\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"regex\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"regextype\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_3::REGEXTYPE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"statistics\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'S' as ::core::ffi::c_int,
    },
    option {
        name: b"follow\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'L' as ::core::ffi::c_int,
    },
    option {
        name: b"nofollow\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'P' as ::core::ffi::c_int,
    },
    option {
        name: b"max-database-age\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_3::MAX_DB_AGE.0 as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn drop_privs() -> ::core::ffi::c_int {
    let mut what: *const ::core::ffi::c_char = b"failed\0".as_ptr() as *const ::core::ffi::c_char;
    let orig_euid: uid_t = geteuid();
    let uid: uid_t = getuid();
    let gid: gid_t = getgid();
    '_fail: {
        if 0 as uid_t == orig_euid {
            let mut groups: [gid_t; 1] = [0; 1];
            groups[0usize] = gid;
            if 0 as ::core::ffi::c_int != setgroups(1 as size_t, &raw mut groups as *mut gid_t) {
                what = dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"failed to drop group privileges\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                );
                break '_fail;
            }
        }
        if uid != orig_euid {
            if 0 as uid_t != uid {
                *__errno_location() = 0 as ::core::ffi::c_int;
                if 0 as ::core::ffi::c_int != setuid(getuid()) {
                    what = dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"failed to drop setuid privileges\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    );
                    break '_fail;
                } else if 0 as ::core::ffi::c_int == setuid(0 as __uid_t) {
                    what = dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Failed to fully drop privileges\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    );
                    *__errno_location() = 0 as ::core::ffi::c_int;
                    break '_fail;
                }
            }
        }
        *__errno_location() = 0 as ::core::ffi::c_int;
        if 0 as ::core::ffi::c_int != setgid(gid) {
            what = dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"failed to drop setgid privileges\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            );
        } else {
            return 0 as ::core::ffi::c_int;
        }
    }
    if 0 != 0 {
        error(
            1 as ::core::ffi::c_int,
            *__errno_location(),
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            quotearg_n_style(
                0 as ::core::ffi::c_int,
                quoting_style::locale_quoting_style,
                what,
            ),
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
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                quotearg_n_style(
                    0 as ::core::ffi::c_int,
                    quoting_style::locale_quoting_style,
                    what,
                ),
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
    abort();
}
unsafe extern "C" fn opendb(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = open_safer(name, O_RDONLY | O_LARGEFILE);
    if fd >= 0 as ::core::ffi::c_int {
        if 0 as ::core::ffi::c_int != rpl_fcntl(fd, F_SETFD, FD_CLOEXEC) {
            close(fd);
            fd = -1 as ::core::ffi::c_int;
        }
    }
    return fd;
}
unsafe extern "C" fn cleanup_quote_opts() {
    free(quote_opts as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn dolocate(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut secure_db_fd: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut path_element_pos: size_t = 0;
    let mut path_element_len: size_t = 0;
    let mut user_selected_locate_path: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut db_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut path_separators: *const ::core::ffi::c_char =
        b":\0".as_ptr() as *const ::core::ffi::c_char;
    let mut found: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut ignore_case: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut print: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut just_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut basename_only: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut use_limit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut regex: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut regex_options: ::core::ffi::c_int = RE_SYNTAX_EMACS as ::core::ffi::c_int;
    let mut stats: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut op_and: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut did_stdin: bool = r#false != 0;
    if !(*argv.offset(0isize)).is_null() {
        set_program_name(*argv.offset(0isize));
    } else {
        set_program_name(b"locate\0".as_ptr() as *const ::core::ffi::c_char);
    }
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    quote_opts = clone_quoting_options(::core::ptr::null_mut::<quoting_options>());
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0
        || atexit(Some(cleanup_quote_opts as unsafe extern "C" fn() -> ())) != 0
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"The atexit library function failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"The atexit library function failed\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    if libc::atexit(rboxc_release_locate_resources) != 0 { return 1; }
    limits.limit = 0 as uintmax_t;
    limits.items_accepted = 0 as uintmax_t;
    print_quoted_filename = r#true != 0;
    user_selected_locate_path = getenv(b"LOCATE_PATH\0".as_ptr() as *const ::core::ffi::c_char);
    check_existence = ExistenceCheckType::ACCEPT_EITHER;
    loop {
        let mut opti: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        let mut optc: ::core::ffi::c_int = getopt_long(
            argc,
            argv,
            b"Abcd:eEil:prsm0SwHPL\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            &raw mut opti,
        );
        if optc == -1 as ::core::ffi::c_int {
            break;
        }
        match optc {
            48 => {
                separator = 0 as ::core::ffi::c_int;
                print_quoted_filename = r#false != 0;
            }
            65 => {
                op_and = 1 as ::core::ffi::c_int;
            }
            98 => {
                basename_only = 1 as ::core::ffi::c_int;
            }
            99 => {
                just_count = 1 as ::core::ffi::c_int;
            }
            100 => {
                user_selected_locate_path = optarg;
                '_c2rust_label_0: {
                    if !optarg.is_null() {
                    } else {
                        __assert_fail(
                            b"optarg != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/opt/src/findutils-4.11.0/locate/locate.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1609 as ::core::ffi::c_uint,
                            b"int dolocate(int, char **, int)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
            101 => {
                check_existence = ExistenceCheckType::ACCEPT_EXISTING;
            }
            69 => {
                check_existence = ExistenceCheckType::ACCEPT_NON_EXISTING;
            }
            105 => {
                ignore_case = 1 as ::core::ffi::c_int;
            }
            104 => {
                usage(EXIT_SUCCESS);
            }
            129 => {
                set_max_db_age(optarg);
            }
            112 => {
                print = 1 as ::core::ffi::c_int;
            }
            118 => {
                display_findutils_version(b"locate\0".as_ptr() as *const ::core::ffi::c_char);
                return 0 as ::core::ffi::c_int;
            }
            119 => {
                basename_only = 0 as ::core::ffi::c_int;
            }
            114 => {
                regex = 1 as ::core::ffi::c_int;
            }
            128 => {
                regex_options = get_regex_type(optarg);
            }
            83 => {
                stats = 1 as ::core::ffi::c_int;
            }
            76 => {
                follow_symlinks = 1 as ::core::ffi::c_int;
            }
            80 | 72 => {
                follow_symlinks = 0 as ::core::ffi::c_int;
            }
            108 => {
                let mut end: *mut ::core::ffi::c_char = optarg;
                let mut err: strtol_error = xstrtoumax(
                    optarg,
                    &raw mut end,
                    10 as ::core::ffi::c_int,
                    &raw mut limits.limit,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
                if strtol_error::LONGINT_OK.0 != err.0 {
                    xstrtol_fatal(
                        err,
                        opti,
                        optc as ::core::ffi::c_char,
                        &raw const longopts as *const option,
                        optarg,
                    );
                }
                use_limit = 1 as ::core::ffi::c_int;
            }
            115 => {}
            109 => {}
            _ => {
                usage(EXIT_FAILURE);
            }
        }
    }
    if !user_selected_locate_path.is_null() {
        if secure_db_fd >= 0 as ::core::ffi::c_int {
            close(secure_db_fd);
            secure_db_fd = -1 as ::core::ffi::c_int;
        }
    }
    if just_count == 0 && stats == 0 {
        print = 1 as ::core::ffi::c_int;
    }
    if stats != 0 {
        if optind == argc {
            use_limit = 0 as ::core::ffi::c_int;
        }
    } else if just_count == 0 && optind == argc {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"pattern argument expected\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
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
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"pattern argument expected\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        usage(EXIT_FAILURE);
    }
    if 1 as ::core::ffi::c_int == isatty(STDOUT_FILENO) {
        stdout_is_a_tty = r#true != 0;
    } else {
        stdout_is_a_tty = r#false != 0;
    }
    if !user_selected_locate_path.is_null() {
        splitstring(
            user_selected_locate_path,
            path_separators,
            r#true != 0,
            &raw mut path_element_pos,
            &raw mut path_element_len,
        );
    }
    while use_limit == 0 || limits.limit > limits.items_accepted {
        let mut st: stat = stat {
            st_dev: 0,
            st_ino: 0,
            st_nlink: 0,
            st_mode: 0,
            st_uid: 0,
            st_gid: 0,
            __pad0: 0,
            st_rdev: 0,
            st_size: 0,
            st_blksize: 0,
            st_blocks: 0,
            st_atim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_mtim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            st_ctim: timespec {
                tv_sec: 0,
                tv_nsec: 0,
            },
            __glibc_reserved: [0; 3],
        };
        let mut database_mtime: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        let mut have_mtime: ::core::ffi::c_int = 0;
        let mut fd: ::core::ffi::c_int = 0;
        let mut filesize: off_t = 0;
        statistics.highbit_filename_count = 0 as uintmax_t;
        statistics.newline_count = statistics.highbit_filename_count;
        statistics.whitespace_count = statistics.newline_count;
        statistics.total_filename_length = statistics.whitespace_count;
        statistics.total_filename_count = statistics.total_filename_length;
        statistics.compressed_bytes = statistics.total_filename_count;
        if !user_selected_locate_path.is_null() {
            if 1 as size_t == path_element_len
                && '-' as ::core::ffi::c_int
                    == *user_selected_locate_path.offset(path_element_pos as isize)
                        as ::core::ffi::c_int
            {
                if did_stdin {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"warning: the locate database can only be read from standard input once.\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
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
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"warning: the locate database can only be read from standard input once.\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                    return 0 as ::core::ffi::c_int;
                } else {
                    db_name = b"<stdin>\0".as_ptr() as *const ::core::ffi::c_char;
                    fd = 0 as ::core::ffi::c_int;
                    did_stdin = r#true != 0;
                }
            } else {
                if 0 as size_t == path_element_len
                    || 1 as size_t == path_element_len
                        && '.' as ::core::ffi::c_int
                            == *user_selected_locate_path.offset(path_element_pos as isize)
                                as ::core::ffi::c_int
                {
                    db_name = LOCATE_DB.as_ptr();
                } else {
                    RBOXC_DBPATH = strndup(
                        user_selected_locate_path.offset(path_element_pos as isize),
                        path_element_len,
                    );
                    db_name = RBOXC_DBPATH;
                }
                fd = opendb(db_name);
                if fd < 0 as ::core::ffi::c_int {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            *__errno_location(),
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                db_name,
                            ),
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
                                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                quotearg_n_style(
                                    0 as ::core::ffi::c_int,
                                    quoting_style::locale_quoting_style,
                                    db_name,
                                ),
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                    return 0 as ::core::ffi::c_int;
                }
            }
        } else if -1 as ::core::ffi::c_int == secure_db_fd {
            break;
        } else {
            db_name = selected_secure_db;
            fd = secure_db_fd;
            secure_db_fd = -1 as ::core::ffi::c_int;
        }
        if fstat(fd, &raw mut st) != 0 {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        db_name,
                    ),
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
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            db_name,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            filesize = 0 as ::core::ffi::c_int as off_t;
            have_mtime = 0 as ::core::ffi::c_int;
        } else {
            let mut now: time_t = 0;
            filesize = st.st_size as off_t;
            database_mtime = get_stat_mtime(&raw mut st);
            have_mtime = 1 as ::core::ffi::c_int;
            if -1 as ::core::ffi::c_int as time_t == time(&raw mut now) {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"time system call failed\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
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
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"time system call failed\0".as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            } else {
                let mut age: ::core::ffi::c_double = difftime(now, st.st_mtim.tv_sec);
                let mut warn_seconds: ::core::ffi::c_double =
                    (SECONDS_PER_UNIT as ::core::ffi::c_uint).wrapping_mul(warn_number_units)
                        as ::core::ffi::c_double;
                if age > warn_seconds {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"warning: database %s is more than %u %s old (actual age is %.1f %s)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                db_name,
                            ),
                            warn_number_units,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                &raw const warn_name_units as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            age
                                / (60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int
                                    * 24 as ::core::ffi::c_int) as ::core::ffi::c_double,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                &raw const warn_name_units as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
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
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"warning: database %s is more than %u %s old (actual age is %.1f %s)\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                quotearg_n_style(
                                    0 as ::core::ffi::c_int,
                                    quoting_style::locale_quoting_style,
                                    db_name,
                                ),
                                warn_number_units,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    &raw const warn_name_units as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                age
                                    / (60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int
                                        * 24 as ::core::ffi::c_int) as ::core::ffi::c_double,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    &raw const warn_name_units as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
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
        fp = fdopen(fd, b"r\0".as_ptr() as *const ::core::ffi::c_char);
        RBOXC_DBFILE = fp;
        if fp.is_null() {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        db_name,
                    ),
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
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            db_name,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            return 0 as ::core::ffi::c_int;
        }
        found = search_one_database(
            argc - optind,
            argv.offset(optind as isize),
            db_name,
            fp,
            filesize,
            if have_mtime != 0 {
                &raw mut database_mtime
            } else {
                ::core::ptr::null_mut::<timespec>()
            },
            ignore_case,
            print,
            basename_only,
            use_limit,
            &raw mut limits,
            stats,
            op_and,
            regex,
            regex_options,
        );
        RBOXC_DBFILE = ::core::ptr::null_mut();
        if fclose(fp) == EOF {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        db_name,
                    ),
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
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            db_name,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            return 0 as ::core::ffi::c_int;
        }
        if !RBOXC_DBPATH.is_null() {
            free(RBOXC_DBPATH as *mut ::core::ffi::c_void);
            RBOXC_DBPATH = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        if user_selected_locate_path.is_null() {
            break;
        }
        if !splitstring(
            user_selected_locate_path,
            path_separators,
            r#false != 0,
            &raw mut path_element_pos,
            &raw mut path_element_len,
        ) {
            break;
        }
    }
    if just_count != 0 {
        printf(b"%lu\n\0".as_ptr() as *const ::core::ffi::c_char, found);
    }
    if found != 0 || use_limit != 0 && limits.limit == 0 as uintmax_t || stats != 0 {
        return 0 as ::core::ffi::c_int;
    } else {
        return 1 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn open_secure_db() -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut secure_db_list: [*const ::core::ffi::c_char; 3] = [
        LOCATE_DB.as_ptr(),
        b"/var/lib/slocate/slocate.db\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ];
    i = 0 as ::core::ffi::c_int;
    while !secure_db_list[i as usize].is_null() {
        fd = opendb(secure_db_list[i as usize]);
        if fd >= 0 as ::core::ffi::c_int {
            selected_secure_db = secure_db_list[i as usize];
            return fd;
        }
        i += 1;
    }
    return -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn rboxc_findutils_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut dbfd: ::core::ffi::c_int = open_secure_db();
    drop_privs();
    return dolocate(argc, argv, dbfd);
}
pub const LOCATE_DB: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/findutils/var/locatedb\0",
    )
};
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/findutils/share/locale\0",
    )
};
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"findutils\0") };

extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
static mut RBOXC_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_findutils_error_prefix() {
    libc::fprintf(stderr.cast(), b"%s: \0".as_ptr().cast(), RBOXC_INVOCATION);
}

static mut RBOXC_PROCDATA: process_data = process_data {
        c: 0,
        count: 0,
        len: 0,
        original_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        pathsize: 0,
        munged_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        fp: ::core::ptr::null_mut::<FILE>(),
        dbfile: ::core::ptr::null::<::core::ffi::c_char>(),
        endian_state: GetwordEndianState::GetwordEndianStateInitial,
        bigram1: [0; 128],
        bigram2: [0; 128],
    };
extern "C" {
    #[link_name = "rboxc_findutils_locate_rpl_regfree"]
    fn rboxc_locate_regfree(regex: *mut re_pattern_buffer);
}
static mut RBOXC_REGEXES: Vec<*mut regular_expression> = Vec::new();
extern "C" fn rboxc_release_locate_path() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        while let Some(regex) = RBOXC_REGEXES.pop() {
            rboxc_locate_regfree(&raw mut (*regex).regex);
            libc::free(regex.cast());
        }
        // Visitor data is borrowed except for regexes, which are owned above.
        while !inspectors.is_null() {
            let node = inspectors;
            inspectors = (*node).next;
            libc::free(node.cast());
        }
        lastinspector = ::core::ptr::null_mut();
        past_pat_inspector = ::core::ptr::null_mut();
        libc::free(RBOXC_PROCDATA.original_filename.cast());
        RBOXC_PROCDATA.original_filename = ::core::ptr::null_mut();
        *libc::__errno_location() = saved_errno;
    }
}

static mut RBOXC_DBPATH: *mut ::core::ffi::c_char = ::core::ptr::null_mut();
static mut RBOXC_DBFILE: *mut FILE = ::core::ptr::null_mut();
extern "C" fn rboxc_release_locate_resources() {
    rboxc_release_locate_path();
    unsafe {
        let saved_errno = *libc::__errno_location();
        if !RBOXC_DBFILE.is_null() {
            let owned = RBOXC_DBFILE;
            RBOXC_DBFILE = ::core::ptr::null_mut();
            libc::fclose(owned.cast());
        }
        libc::free(RBOXC_DBPATH.cast());
        RBOXC_DBPATH = ::core::ptr::null_mut();
        *libc::__errno_location() = saved_errno;
    }
}

#[no_mangle]
pub unsafe extern "C" fn single_binary_main_locate(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    RBOXC_INVOCATION = if argv.is_null() || (*argv).is_null() {
        b"locate\0".as_ptr().cast()
    } else { *argv };
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_findutils_error_prefix);
    }
    if libc::atexit(rboxc_release_locate_resources) != 0 { return 1; }
    rboxc_findutils_main_inner(argc, argv)
}
