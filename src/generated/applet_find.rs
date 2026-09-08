// Generated from pinned GNU Findutils 4.11.0 by scripts/translate-findutils.py.
// Source SHA-256: 822dc3163b0e849bb0e8c4cb15e41fe3ca2dd0d5a64a3986f3eeefb88dc83ac3
/* find -- search for files in a directory hierarchy (fts version)
   Copyright (C) 1990-2026 Free Software Foundation, Inc.

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
pub struct argv_iterator { _opaque: [u8; 0] }
#[repr(C)]
pub struct __dirstream { _opaque: [u8; 0] }
#[repr(C)]
pub struct cycle_check_state { _opaque: [u8; 0] }
#[repr(C)]
pub struct hash_table { _opaque: [u8; 0] }
#[repr(C)]
pub struct quoting_options { _opaque: [u8; 0] }
#[repr(C)]
pub struct re_dfa_t { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn ctime(__timer: *const time_t) -> *mut ::core::ffi::c_char;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut stdin: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_argv_iter_free"]
    fn argv_iter_free(_: *mut argv_iterator);
    #[link_name = "rboxc_findutils_find_argv_iter_init_argv"]
    fn argv_iter_init_argv(argv: *mut *mut ::core::ffi::c_char) -> *mut argv_iterator;
    #[link_name = "rboxc_findutils_find_argv_iter_init_stream"]
    fn argv_iter_init_stream(fp: *mut FILE) -> *mut argv_iterator;
    #[link_name = "rboxc_findutils_find_argv_iter"]
    fn argv_iter(_: *mut argv_iterator, _: *mut argv_iter_err) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_find_argv_iter_n_args"]
    fn argv_iter_n_args(_: *const argv_iterator) -> size_t;
    #[link_name = "rboxc_findutils_find_set_cloexec_flag"]
    fn set_cloexec_flag(desc: ::core::ffi::c_int, value: bool) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_dup_cloexec"]
    fn dup_cloexec(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_findutils_find_rpl_fts_close"]
    fn rpl_fts_close(_: *mut FTS) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_rpl_fts_open"]
    fn rpl_fts_open(
        _: *const *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> ::core::ffi::c_int,
        >,
    ) -> *mut FTS;
    #[link_name = "rboxc_findutils_find_rpl_fts_read"]
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    #[link_name = "rboxc_findutils_find_rpl_fts_set"]
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_quotearg_n_style"]
    fn quotearg_n_style(
        n: ::core::ffi::c_int,
        s: quoting_style,
        arg: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_find_xalloc_die"]
    fn xalloc_die();
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_findutils_find_sharefile_init"]
    fn sharefile_init(mode: *const ::core::ffi::c_char) -> sharefile_handle;
    #[link_name = "rboxc_findutils_find_debug_stat"]
    fn debug_stat(file: *const ::core::ffi::c_char, bufp: *mut stat) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_cleanup"]
    fn cleanup();
    #[link_name = "rboxc_findutils_find_show_success_rates"]
    fn show_success_rates(node: *const predicate);
    #[link_name = "rboxc_findutils_find_build_expression_tree"]
    fn build_expression_tree(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
        end_of_leading_options: ::core::ffi::c_int,
    ) -> *mut predicate;
    #[link_name = "rboxc_findutils_find_get_eval_tree"]
    fn get_eval_tree() -> *mut predicate;
    #[link_name = "rboxc_findutils_find_complete_pending_execdirs"]
    fn complete_pending_execdirs();
    #[link_name = "rboxc_findutils_find_safely_quote_err_filename"]
    fn safely_quote_err_filename(
        n: ::core::ffi::c_int,
        arg: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_find_record_initial_cwd"]
    fn record_initial_cwd();
    #[link_name = "rboxc_findutils_find_nonfatal_target_file_error"]
    fn nonfatal_target_file_error(
        errno_value: ::core::ffi::c_int,
        name: *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_findutils_find_process_leading_options"]
    fn process_leading_options(
        argc: ::core::ffi::c_int,
        argv: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_find_set_option_defaults"]
    fn set_option_defaults(p: *mut options);
    #[link_name = "rboxc_findutils_find_apply_predicate"]
    fn apply_predicate(
        pathname: *const ::core::ffi::c_char,
        stat_buf: *mut stat,
        p: *mut predicate,
    ) -> bool;
    #[link_name = "rboxc_findutils_find_digest_mode"]
    fn digest_mode(
        mode: *mut mode_t,
        pathname: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        pstat: *mut stat,
        leaf: bool,
    ) -> bool;
    #[link_name = "rboxc_findutils_find_looks_like_expression"]
    fn looks_like_expression(arg: *const ::core::ffi::c_char, leading: bool) -> bool;
    #[link_name = "rboxc_findutils_find_options"]
    static mut options: options;
    #[link_name = "rboxc_findutils_find_state"]
    static mut state: state;
    #[link_name = "rboxc_findutils_find_remember_non_cloexec_fds"]
    fn remember_non_cloexec_fds();
    #[link_name = "rboxc_findutils_find_fd_leak_check_is_enabled"]
    fn fd_leak_check_is_enabled() -> bool;
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
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn bindtextdomain(
        __domainname: *const ::core::ffi::c_char,
        __dirname: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub type ptrdiff_t = isize;
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
pub struct argv_iter_err(pub ::core::ffi::c_uint);
impl argv_iter_err {
    pub const AI_ERR_OK: Self = Self(1);
    pub const AI_ERR_EOF: Self = Self(2);
    pub const AI_ERR_MEM: Self = Self(3);
    pub const AI_ERR_READ: Self = Self(4);
}
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct I_ring {
    pub ir_data: [::core::ffi::c_int; 4],
    pub ir_default_val: ::core::ffi::c_int,
    pub ir_front: ::core::ffi::c_uint,
    pub ir_back: ::core::ffi::c_uint,
    pub ir_empty: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FTS {
    pub fts_cur: *mut _ftsent,
    pub fts_child: *mut _ftsent,
    pub fts_array: *mut *mut _ftsent,
    pub fts_dev: dev_t,
    pub fts_path: *mut ::core::ffi::c_char,
    pub fts_rfd: ::core::ffi::c_int,
    pub fts_cwd_fd: ::core::ffi::c_int,
    pub fts_pathlen: size_t,
    pub fts_nitems: size_t,
    pub fts_compar: Option<
        unsafe extern "C" fn(*mut *const _ftsent, *mut *const _ftsent) -> ::core::ffi::c_int,
    >,
    pub fts_options: ::core::ffi::c_int,
    pub fts_leaf_optimization_works_ht: *mut hash_table,
    pub fts_cycle: C2Rust_Unnamed,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub ht: *mut hash_table,
    pub state: *mut cycle_check_state,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _ftsent {
    pub fts_cycle: *mut _ftsent,
    pub fts_parent: *mut _ftsent,
    pub fts_link: *mut _ftsent,
    pub fts_dirp: *mut DIR,
    pub fts_number: ::core::ffi::c_long,
    pub fts_pointer: *mut ::core::ffi::c_void,
    pub fts_accpath: *mut ::core::ffi::c_char,
    pub fts_path: *mut ::core::ffi::c_char,
    pub fts_errno: ::core::ffi::c_int,
    pub fts_symfd: ::core::ffi::c_int,
    pub fts_pathlen: size_t,
    pub fts_fts: *mut FTS,
    pub fts_level: ptrdiff_t,
    pub fts_namelen: size_t,
    pub fts_info: ::core::ffi::c_ushort,
    pub fts_flags: ::core::ffi::c_ushort,
    pub fts_instr: ::core::ffi::c_ushort,
    pub fts_statp: [stat; 1],
    pub fts_name: [::core::ffi::c_char; 0],
}
pub type FTSENT = _ftsent;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct saved_cwd {
    pub desc: ::core::ffi::c_int,
    pub name: *mut ::core::ffi::c_char,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: size_t,
    pub cmd_argv: *mut *mut ::core::ffi::c_char,
    pub cmd_argv_alloc: size_t,
    pub argbuf: *mut ::core::ffi::c_char,
    pub cmd_argv_chars: size_t,
    pub cmd_initial_argv_chars: size_t,
    pub usercontext: *mut ::core::ffi::c_void,
    pub todo: ::core::ffi::c_int,
    pub dir_fd: ::core::ffi::c_int,
    pub largest_successful_arg_count: size_t,
    pub smallest_failed_arg_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: ::core::ffi::c_int,
    pub posix_arg_size_max: size_t,
    pub posix_arg_size_min: size_t,
    pub arg_max: size_t,
    pub max_arg_count: size_t,
    pub rplen: size_t,
    pub replace_pat: *const ::core::ffi::c_char,
    pub initial_argc: size_t,
    pub exec_callback: Option<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub lines_per_exec: ::core::ffi::c_ulong,
    pub args_per_exec: size_t,
}
pub type sharefile_handle = *mut ::core::ffi::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate {
    pub pred_func: PRED_FUNC,
    pub p_name: *const ::core::ffi::c_char,
    pub p_type: predicate_type,
    pub p_prec: predicate_precedence,
    pub side_effects: bool,
    pub no_default_print: bool,
    pub need_stat: bool,
    pub need_type: bool,
    pub need_inum: bool,
    pub p_cost: EvaluationCost,
    pub est_success_rate: ::core::ffi::c_float,
    pub artificial: bool,
    pub arg_text: *const ::core::ffi::c_char,
    pub args: C2Rust_Unnamed_0,
    pub pred_next: *mut predicate,
    pub pred_left: *mut predicate,
    pub pred_right: *mut predicate,
    pub perf: predicate_performance_info,
    pub parser_entry: *const parser_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct parser_table {
    pub r#type: arg_type,
    pub parser_name: *const ::core::ffi::c_char,
    pub parser_func: PARSE_FUNC,
    pub pred_func: PRED_FUNC,
}
pub type PRED_FUNC =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat, *mut predicate) -> bool>;
pub type PARSE_FUNC = Option<
    unsafe extern "C" fn(
        *const parser_table,
        *mut *mut ::core::ffi::c_char,
        *mut ::core::ffi::c_int,
    ) -> bool,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct arg_type(pub ::core::ffi::c_uint);
impl arg_type {
    pub const ARG_OPTION: Self = Self(0);
    pub const ARG_NOOP: Self = Self(1);
    pub const ARG_POSITIONAL_OPTION: Self = Self(2);
    pub const ARG_TEST: Self = Self(3);
    pub const ARG_SPECIAL_PARSE: Self = Self(4);
    pub const ARG_PUNCTUATION: Self = Self(5);
    pub const ARG_ACTION: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct predicate_performance_info {
    pub visits: ::core::ffi::c_ulong,
    pub successes: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub str: *const ::core::ffi::c_char,
    pub regex: *mut re_pattern_buffer,
    pub exec_vec: exec_val,
    pub numinfo: long_val,
    pub size: size_val,
    pub uid: uid_t,
    pub gid: gid_t,
    pub reftime: time_val,
    pub perm: perm_val,
    pub samefileid: samefile_file_id,
    pub types: [bool; 7],
    pub printf_vec: format_val,
    pub scontext: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct format_val {
    pub segment: *mut segment,
    pub stream: *mut FILE,
    pub filename: *const ::core::ffi::c_char,
    pub dest_is_tty: bool,
    pub quote_opts: *mut quoting_options,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub segkind: SegmentKind,
    pub format_char: [::core::ffi::c_char; 2],
    pub text: *mut ::core::ffi::c_char,
    pub text_len: ::core::ffi::c_int,
    pub next: *mut segment,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SegmentKind(pub ::core::ffi::c_uint);
impl SegmentKind {
    pub const KIND_PLAIN: Self = Self(0);
    pub const KIND_STOP: Self = Self(1);
    pub const KIND_FORMAT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct samefile_file_id {
    pub ino: ino_t,
    pub dev: dev_t,
    pub fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct perm_val {
    pub kind: permissions_type,
    pub val: [mode_t; 2],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct permissions_type(pub ::core::ffi::c_uint);
impl permissions_type {
    pub const PERM_AT_LEAST: Self = Self(0);
    pub const PERM_ANY: Self = Self(1);
    pub const PERM_EXACT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct time_val {
    pub xval: xval,
    pub kind: comparison_type,
    pub ts: timespec,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct comparison_type(pub ::core::ffi::c_uint);
impl comparison_type {
    pub const COMP_GT: Self = Self(0);
    pub const COMP_LT: Self = Self(1);
    pub const COMP_EQ: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct xval(pub ::core::ffi::c_uint);
impl xval {
    pub const XVAL_ATIME: Self = Self(0);
    pub const XVAL_BIRTHTIME: Self = Self(1);
    pub const XVAL_CTIME: Self = Self(2);
    pub const XVAL_MTIME: Self = Self(3);
    pub const XVAL_TIME: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct size_val {
    pub kind: comparison_type,
    pub blocksize: ::core::ffi::c_int,
    pub size: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct long_val {
    pub kind: comparison_type,
    pub negative: bool,
    pub l_val: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exec_val {
    pub multiple: bool,
    pub ctl: buildcmd_control,
    pub state: buildcmd_state,
    pub replace_vec: *mut *mut ::core::ffi::c_char,
    pub num_args: ::core::ffi::c_int,
    pub close_stdin: bool,
    pub wd_for_exec: *mut saved_cwd,
    pub last_child_status: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EvaluationCost(pub ::core::ffi::c_uint);
impl EvaluationCost {
    pub const NeedsNothing: Self = Self(0);
    pub const NeedsInodeNumber: Self = Self(1);
    pub const NeedsType: Self = Self(2);
    pub const NeedsStatInfo: Self = Self(3);
    pub const NeedsLinkName: Self = Self(4);
    pub const NeedsAccessInfo: Self = Self(5);
    pub const NeedsSyncDiskHit: Self = Self(6);
    pub const NeedsEventualExec: Self = Self(7);
    pub const NeedsImmediateExec: Self = Self(8);
    pub const NeedsUserInteraction: Self = Self(9);
    pub const NeedsUnknown: Self = Self(10);
    pub const NumEvaluationCosts: Self = Self(11);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct predicate_precedence(pub ::core::ffi::c_uint);
impl predicate_precedence {
    pub const NO_PREC: Self = Self(0);
    pub const COMMA_PREC: Self = Self(1);
    pub const OR_PREC: Self = Self(2);
    pub const AND_PREC: Self = Self(3);
    pub const NEGATE_PREC: Self = Self(4);
    pub const MAX_PREC: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct predicate_type(pub ::core::ffi::c_uint);
impl predicate_type {
    pub const NO_TYPE: Self = Self(0);
    pub const PRIMARY_TYPE: Self = Self(1);
    pub const UNI_OP: Self = Self(2);
    pub const BI_OP: Self = Self(3);
    pub const OPEN_PAREN: Self = Self(4);
    pub const CLOSE_PAREN: Self = Self(5);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub do_dir_first: bool,
    pub explicit_depth: bool,
    pub maxdepth: ::core::ffi::c_int,
    pub mindepth: ::core::ffi::c_int,
    pub no_leaf_check: bool,
    pub mount: bool,
    pub xdev: bool,
    pub ignore_readdir_race: bool,
    pub literal_control_chars: bool,
    pub warnings: bool,
    pub posixly_correct: bool,
    pub start_time: timespec,
    pub cur_day_start: timespec,
    pub full_days: bool,
    pub output_block_size: ::core::ffi::c_int,
    pub debug_options: ::core::ffi::c_ulong,
    pub symlink_handling: SymlinkOption,
    pub xstat:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
    pub open_nofollow_available: bool,
    pub regex_options: ::core::ffi::c_int,
    pub x_getfilecon: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *const ::core::ffi::c_char,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub optimisation_level: ::core::ffi::c_ushort,
    pub err_quoting_style: quoting_style,
    pub files0_from: *const ::core::ffi::c_char,
    pub ok_prompt_stdin: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct SymlinkOption(pub ::core::ffi::c_uint);
impl SymlinkOption {
    pub const SYMLINK_NEVER_DEREF: Self = Self(0);
    pub const SYMLINK_ALWAYS_DEREF: Self = Self(1);
    pub const SYMLINK_DEREF_ARGSONLY: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DebugOption(pub ::core::ffi::c_int);
impl DebugOption {
    pub const DebugNone: Self = Self(0);
    pub const DebugExpressionTree: Self = Self(1);
    pub const DebugStat: Self = Self(2);
    pub const DebugSearch: Self = Self(4);
    pub const DebugTreeOpt: Self = Self(8);
    pub const DebugHelp: Self = Self(16);
    pub const DebugExec: Self = Self(32);
    pub const DebugSuccessRates: Self = Self(64);
    pub const DebugTime: Self = Self(128);
    pub const DebugAll: Self = Self(-17);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct state {
    pub curdepth: ::core::ffi::c_int,
    pub have_stat: bool,
    pub have_type: bool,
    pub r#type: mode_t,
    pub rel_pathname: *const ::core::ffi::c_char,
    pub cwd_dir_fd: ::core::ffi::c_int,
    pub starting_path_length: ::core::ffi::c_int,
    pub stop_at_current_level: bool,
    pub exit_status: ::core::ffi::c_int,
    pub execdirs_outstanding: bool,
    pub shared_files: sharefile_handle,
    pub already_issued_stat_error_msg: bool,
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const INT_MIN: ::core::ffi::c_int = -__INT_MAX__ - 1 as ::core::ffi::c_int;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FTS_COMFOLLOW: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS_LOGICAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS_NOSTAT: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const FTS_PHYSICAL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const FTS_XDEV: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const FTS_TIGHT_CYCLE_CHECK: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const FTS_CWDFD: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const FTS_VERBATIM: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const FTS_MOUNT: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const FTS_D: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS_DC: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FTS_DNR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FTS_DP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FTS_ERR: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const FTS_NS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const FTS_NSOK: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const FTS_SLNONE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const FTS_AGAIN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const FTS_SKIP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
static mut ftsoptions: ::core::ffi::c_int =
    FTS_NOSTAT | FTS_TIGHT_CYCLE_CHECK | FTS_CWDFD | FTS_VERBATIM;
static mut prev_depth: ::core::ffi::c_int = INT_MIN;
static mut curr_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
unsafe extern "C" fn left_dir() {
    if ftsoptions & FTS_CWDFD != 0 {
        if curr_fd >= 0 as ::core::ffi::c_int {
            close(curr_fd);
            curr_fd = -1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn inside_dir(mut dir_fd: ::core::ffi::c_int) {
    if ftsoptions & FTS_CWDFD != 0 {
        '_c2rust_label: {
            if dir_fd == -100 as ::core::ffi::c_int || dir_fd >= 0 as ::core::ffi::c_int {
            } else {
                __assert_fail(
                    b"dir_fd == AT_FDCWD || dir_fd >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/find/ftsfind.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    105 as ::core::ffi::c_uint,
                    b"void inside_dir(int)\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        state.cwd_dir_fd = dir_fd;
        if curr_fd < 0 as ::core::ffi::c_int {
            if AT_FDCWD == dir_fd {
                curr_fd = AT_FDCWD;
            } else if dir_fd >= 0 as ::core::ffi::c_int {
                curr_fd = dup_cloexec(dir_fd);
            } else {
                '_c2rust_label_0: {
                    if curr_fd >= 0 as ::core::ffi::c_int || dir_fd >= 0 as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"curr_fd >= 0 || dir_fd >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/opt/src/findutils-4.11.0/find/ftsfind.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            123 as ::core::ffi::c_uint,
                            b"void inside_dir(int)\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                };
            }
        }
    }
}
unsafe extern "C" fn get_fts_info_name(mut info: ::core::ffi::c_int) -> *const ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 14] = [0; 14];
    match info {
        1 => return b"FTS_D\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"FTS_DC\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"FTS_DEFAULT\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"FTS_DNR\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"FTS_DOT\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"FTS_DP\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"FTS_ERR\0".as_ptr() as *const ::core::ffi::c_char,
        8 => return b"FTS_F\0".as_ptr() as *const ::core::ffi::c_char,
        9 => return b"FTS_INIT\0".as_ptr() as *const ::core::ffi::c_char,
        10 => return b"FTS_NS\0".as_ptr() as *const ::core::ffi::c_char,
        11 => return b"FTS_NSOK\0".as_ptr() as *const ::core::ffi::c_char,
        12 => return b"FTS_SL\0".as_ptr() as *const ::core::ffi::c_char,
        13 => return b"FTS_SLNONE\0".as_ptr() as *const ::core::ffi::c_char,
        14 => return b"FTS_W\0".as_ptr() as *const ::core::ffi::c_char,
        _ => {
            sprintf(
                &raw mut buf as *mut ::core::ffi::c_char,
                b"[%d]\0".as_ptr() as *const ::core::ffi::c_char,
                info,
            );
            return &raw mut buf as *mut ::core::ffi::c_char;
        }
    };
}
unsafe extern "C" fn visit(mut p: *mut FTS, mut ent: *mut FTSENT, mut pstat: *mut stat) {
    let mut eval_tree: *mut predicate = ::core::ptr::null_mut::<predicate>();
    state.have_stat = (*ent).fts_info as ::core::ffi::c_int != FTS_NS
        && (*ent).fts_info as ::core::ffi::c_int != FTS_NSOK;
    state.rel_pathname = (*ent).fts_accpath;
    state.cwd_dir_fd = (*p).fts_cwd_fd;
    eval_tree = get_eval_tree();
    apply_predicate((*ent).fts_path, pstat, eval_tree);
    if state.stop_at_current_level {
        rpl_fts_set(p, ent, FTS_SKIP);
    }
}
unsafe extern "C" fn issue_loop_warning(mut ent: *mut FTSENT) {
    if (*(&raw mut (*ent).fts_statp as *mut stat)).st_mode & __S_IFMT as __mode_t
        == 0o120000 as __mode_t
    {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                safely_quote_err_filename(0 as ::core::ffi::c_int, (*ent).fts_path),
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
                        b"Symbolic link %s is part of a loop in the directory hierarchy; we have already visited the directory to which it points.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    safely_quote_err_filename(0 as ::core::ffi::c_int, (*ent).fts_path),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"File system loop detected; the following directory is part of the cycle: %s\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                safely_quote_err_filename(0 as ::core::ffi::c_int, (*ent).fts_path),
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
                        b"File system loop detected; the following directory is part of the cycle: %s\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    safely_quote_err_filename(0 as ::core::ffi::c_int, (*ent).fts_path),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    };
}
unsafe extern "C" fn symlink_loop(mut name: *const ::core::ffi::c_char) -> bool {
    let mut stbuf: stat = stat {
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
    let rv: ::core::ffi::c_int =
        options.xstat.expect("non-null function pointer")(name, &raw mut stbuf);
    return 0 as ::core::ffi::c_int != rv && ELOOP == *__errno_location();
}
unsafe extern "C" fn consider_visiting(mut p: *mut FTS, mut ent: *mut FTSENT) {
    let mut statbuf: stat = stat {
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
    let mut mode: mode_t = 0;
    let mut ignore: ::core::ffi::c_int = 0;
    let mut isdir: ::core::ffi::c_int = 0;
    if options.debug_options & DebugOption::DebugSearch.0 as ::core::ffi::c_ulong != 0 {
        fprintf(
            stderr,
            b"consider_visiting (early): %s: fts_info=%-6s, fts_level=%2d, prev_depth=%d fts_path=%s, fts_accpath=%s\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            quotearg_n_style(
                0 as ::core::ffi::c_int,
                options.err_quoting_style,
                (*ent).fts_path,
            ),
            get_fts_info_name((*ent).fts_info as ::core::ffi::c_int),
            (*ent).fts_level as ::core::ffi::c_int,
            prev_depth,
            quotearg_n_style(
                1 as ::core::ffi::c_int,
                options.err_quoting_style,
                (*ent).fts_path,
            ),
            quotearg_n_style(
                2 as ::core::ffi::c_int,
                options.err_quoting_style,
                (*ent).fts_accpath,
            ),
        );
    }
    if (*ent).fts_info as ::core::ffi::c_int == FTS_DP {
        left_dir();
    } else if (*ent).fts_level > prev_depth as ptrdiff_t || (*ent).fts_level == 0 as ptrdiff_t {
        left_dir();
    }
    inside_dir((*p).fts_cwd_fd);
    prev_depth = (*ent).fts_level as ::core::ffi::c_int;
    statbuf.st_ino = (*(&raw mut (*ent).fts_statp as *mut stat)).st_ino;
    if (*ent).fts_info as ::core::ffi::c_int == FTS_ERR {
        nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        return;
    }
    if (*ent).fts_info as ::core::ffi::c_int == FTS_DNR {
        if ENOENT == (*ent).fts_errno && options.ignore_readdir_race as ::core::ffi::c_int != 0 {
            return;
        }
        nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        if options.do_dir_first {
            return;
        }
    } else if (*ent).fts_info as ::core::ffi::c_int == FTS_DC {
        issue_loop_warning(ent);
        state.exit_status = EXIT_FAILURE;
        return;
    } else if (*ent).fts_info as ::core::ffi::c_int == FTS_SLNONE {
        if symlink_loop((*ent).fts_accpath) {
            nonfatal_target_file_error(ELOOP, (*ent).fts_path);
            return;
        }
    } else if (*ent).fts_info as ::core::ffi::c_int == FTS_NS {
        if (*ent).fts_level == 0 as ptrdiff_t {
            nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
            return;
        } else if symlink_loop((*ent).fts_accpath) {
            nonfatal_target_file_error(ELOOP, (*ent).fts_path);
            return;
        } else {
            if ENOENT == (*ent).fts_errno && options.ignore_readdir_race as ::core::ffi::c_int != 0
            {
                return;
            }
            nonfatal_target_file_error((*ent).fts_errno, (*ent).fts_path);
        }
    }
    if (*ent).fts_info as ::core::ffi::c_int == FTS_NSOK
        || (*ent).fts_info as ::core::ffi::c_int == FTS_NS
    {
        '_c2rust_label: {
            if !state.have_stat {
            } else {
                __assert_fail(
                    b"!state.have_stat\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/find/ftsfind.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    360 as ::core::ffi::c_uint,
                    b"void consider_visiting(FTS *, FTSENT *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_0: {
            if (*ent).fts_info as ::core::ffi::c_int == 11 as ::core::ffi::c_int
                || state.r#type == 0 as mode_t
            {
            } else {
                __assert_fail(
                    b"ent->fts_info == FTS_NSOK || state.type == 0\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/find/ftsfind.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    361 as ::core::ffi::c_uint,
                    b"void consider_visiting(FTS *, FTSENT *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        mode = state.r#type;
    } else {
        state.have_stat = r#true != 0;
        state.have_type = r#true != 0;
        statbuf = *(&raw mut (*ent).fts_statp as *mut stat);
        mode = statbuf.st_mode as mode_t;
        state.r#type = mode;
        if 0 as mode_t == mode {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"WARNING: file %s appears to have mode 0000\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        options.err_quoting_style,
                        (*ent).fts_path,
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
                            b"WARNING: file %s appears to have mode 0000\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            options.err_quoting_style,
                            (*ent).fts_path,
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
    state.curdepth = (*ent).fts_level as ::core::ffi::c_int;
    if mode != 0 {
        if !digest_mode(
            &raw mut mode,
            (*ent).fts_path,
            &raw mut (*ent).fts_name as *mut ::core::ffi::c_char,
            &raw mut statbuf,
            false,
        ) {
            return;
        }
    }
    ignore = 0 as ::core::ffi::c_int;
    isdir = (mode & __S_IFMT as mode_t == 0o40000 as mode_t
        || FTS_D == (*ent).fts_info as ::core::ffi::c_int
        || FTS_DP == (*ent).fts_info as ::core::ffi::c_int
        || FTS_DC == (*ent).fts_info as ::core::ffi::c_int) as ::core::ffi::c_int;
    if isdir != 0 && (*ent).fts_info as ::core::ffi::c_int == FTS_NSOK {
        rpl_fts_set(p, ent, FTS_AGAIN);
        return;
    }
    if options.maxdepth >= 0 as ::core::ffi::c_int {
        if (*ent).fts_level >= options.maxdepth as ptrdiff_t {
            rpl_fts_set(p, ent, FTS_SKIP);
            if (*ent).fts_level > options.maxdepth as ptrdiff_t {
                ignore = 1 as ::core::ffi::c_int;
            }
        }
    }
    if (*ent).fts_info as ::core::ffi::c_int == FTS_D && !options.do_dir_first {
        ignore = 1 as ::core::ffi::c_int;
    } else if (*ent).fts_info as ::core::ffi::c_int == FTS_DP
        && options.do_dir_first as ::core::ffi::c_int != 0
    {
        ignore = 1 as ::core::ffi::c_int;
    } else if (*ent).fts_level < options.mindepth as ptrdiff_t {
        ignore = 1 as ::core::ffi::c_int;
    }
    if options.debug_options & DebugOption::DebugSearch.0 as ::core::ffi::c_ulong != 0 {
        fprintf(
            stderr,
            b"consider_visiting (late): %s: fts_info=%-6s, isdir=%d ignore=%d have_stat=%d have_type=%d \n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            quotearg_n_style(
                0 as ::core::ffi::c_int,
                options.err_quoting_style,
                (*ent).fts_path,
            ),
            get_fts_info_name((*ent).fts_info as ::core::ffi::c_int),
            isdir,
            ignore,
            state.have_stat as ::core::ffi::c_int,
            state.have_type as ::core::ffi::c_int,
        );
    }
    if ignore == 0 {
        visit(p, ent, &raw mut statbuf);
    }
    if (*ent).fts_info as ::core::ffi::c_int == FTS_DP {
        state.stop_at_current_level = r#false != 0;
    }
}
unsafe extern "C" fn find(mut arg: *mut ::core::ffi::c_char) -> bool {
    let mut arglist: [*mut ::core::ffi::c_char; 2] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
    let mut p: *mut FTS = ::core::ptr::null_mut::<FTS>();
    let mut ent: *mut FTSENT = ::core::ptr::null_mut::<FTSENT>();
    state.starting_path_length = strlen(arg) as ::core::ffi::c_int;
    inside_dir(AT_FDCWD);
    arglist[0usize] = arg;
    arglist[1usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match options.symlink_handling {
        SymlinkOption::SYMLINK_ALWAYS_DEREF => {
            ftsoptions |= FTS_COMFOLLOW | FTS_LOGICAL;
        }
        SymlinkOption::SYMLINK_DEREF_ARGSONLY => {
            ftsoptions |= FTS_COMFOLLOW | FTS_PHYSICAL;
        }
        SymlinkOption::SYMLINK_NEVER_DEREF => {
            ftsoptions |= FTS_PHYSICAL;
        }
        _ => {}
    }
    if options.mount {
        ftsoptions |= FTS_MOUNT;
    }
    if options.xdev {
        ftsoptions |= FTS_XDEV;
    }
    p = rpl_fts_open(
        &raw mut arglist as *mut *mut ::core::ffi::c_char,
        ftsoptions,
        None,
    );
    if p.is_null() {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"cannot search %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                safely_quote_err_filename(0 as ::core::ffi::c_int, arg),
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
                        b"cannot search %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    safely_quote_err_filename(0 as ::core::ffi::c_int, arg),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        state.exit_status = EXIT_FAILURE;
    } else {
        let mut level: ::core::ffi::c_int = INT_MIN;
        loop {
            *__errno_location() = 0 as ::core::ffi::c_int;
            ent = rpl_fts_read(p);
            if ent.is_null() {
                break;
            }
            if state.execdirs_outstanding as ::core::ffi::c_int != 0
                && (*ent).fts_level as ::core::ffi::c_int != level
            {
                complete_pending_execdirs();
            }
            level = (*ent).fts_level as ::core::ffi::c_int;
            state.already_issued_stat_error_msg = r#false != 0;
            state.have_stat = r#false != 0;
            state.have_type = (*(&raw mut (*ent).fts_statp as *mut stat)).st_mode != 0;
            state.r#type = (if state.have_type as ::core::ffi::c_int != 0 {
                (*(&raw mut (*ent).fts_statp as *mut stat)).st_mode
            } else {
                0 as __mode_t
            }) as mode_t;
            consider_visiting(p, ent);
        }
        if *__errno_location() != 0 {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"failed to read file names from file system at or below %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    safely_quote_err_filename(0 as ::core::ffi::c_int, arg),
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
                        b"failed to read file names from file system at or below %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        safely_quote_err_filename(0 as ::core::ffi::c_int, arg),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            state.exit_status = EXIT_FAILURE;
            return r#false != 0;
        }
        if 0 as ::core::ffi::c_int != rpl_fts_close(p) {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"failed to restore working directory after searching %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    arg,
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
                            b"failed to restore working directory after searching %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        arg,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            state.exit_status = EXIT_FAILURE;
            return r#false != 0;
        }
        p = ::core::ptr::null_mut::<FTS>();
    }
    return r#true != 0;
}
unsafe extern "C" fn process_all_startpoints(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> bool {
    let mut argv_starting_points: bool = (0 as ::core::ffi::c_int) < argc
        && !looks_like_expression(*argv.offset(0isize), r#true != 0);
    let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut files0_filename_quoted: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut ai: *mut argv_iterator = ::core::ptr::null_mut::<argv_iterator>();
    if !options.files0_from.is_null() {
        if argv_starting_points {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"extra operand %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    safely_quote_err_filename(0 as ::core::ffi::c_int, *argv.offset(0isize)),
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
                            b"extra operand %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        safely_quote_err_filename(0 as ::core::ffi::c_int, *argv.offset(0isize)),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"file operands cannot be combined with -files0-from\0".as_ptr()
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
                            b"file operands cannot be combined with -files0-from\0".as_ptr()
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
        if 0 as ::core::ffi::c_int
            == strcmp(
                options.files0_from,
                b"-\0".as_ptr() as *const ::core::ffi::c_char,
            )
        {
            if options.ok_prompt_stdin {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"when -ok or -okdir is in use, the option -files0-from cannot also read from standard input\0"
                                .as_ptr() as *const ::core::ffi::c_char,
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
                                b"when -ok or -okdir is in use, the option -files0-from cannot also read from standard input\0"
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
            }
            files0_filename_quoted = safely_quote_err_filename(
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"(standard input)\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            stream = stdin;
        } else {
            files0_filename_quoted =
                safely_quote_err_filename(0 as ::core::ffi::c_int, options.files0_from);
            stream = fopen(
                options.files0_from,
                b"r\0".as_ptr() as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if stream.is_null() {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"cannot open %s for reading\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        files0_filename_quoted,
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
                                b"cannot open %s for reading\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            files0_filename_quoted,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            let fd: ::core::ffi::c_int = fileno(stream);
            '_c2rust_label_3: {
                if fd >= 0 as ::core::ffi::c_int {
                } else {
                    __assert_fail(
                        b"fd >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/find/ftsfind.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        594 as ::core::ffi::c_uint,
                        b"_Bool process_all_startpoints(int, char **)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            if options.ok_prompt_stdin {
                let mut sb1: stat = stat {
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
                let mut sb2: stat = stat {
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
                if fstat(fd, &raw mut sb1) == 0 as ::core::ffi::c_int
                    && fstat(STDIN_FILENO, &raw mut sb2) == 0 as ::core::ffi::c_int
                    && sb1.st_dev ^ sb2.st_dev | sb1.st_ino ^ sb2.st_ino == 0
                {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"when -ok or -okdir is in use, the option -files0-from cannot also read from standard input, but the file (%s) used for -files0-from appears to refer to the same file as the standard input\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            files0_filename_quoted,
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
                                    b"when -ok or -okdir is in use, the option -files0-from cannot also read from standard input, but the file (%s) used for -files0-from appears to refer to the same file as the standard input\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                files0_filename_quoted,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
            }
            set_cloexec_flag(fd, r#true != 0);
        }
        ai = argv_iter_init_stream(stream);
    } else {
        if !argv_starting_points {
            let mut defaultpath: [::core::ffi::c_char; 2] =
                ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b".\0");
            return find(&raw mut defaultpath as *mut ::core::ffi::c_char);
        }
        ai = argv_iter_init_argv(argv);
    }
    if ai.is_null() {
        xalloc_die();
    }
    let mut ok: bool = r#true != 0;
    loop {
        let mut ai_err: argv_iter_err = argv_iter_err(0);
        let mut file_name: *mut ::core::ffi::c_char = argv_iter(ai, &raw mut ai_err);
        if file_name.is_null() {
            match ai_err {
                argv_iter_err::AI_ERR_EOF => {
                    break;
                }
                argv_iter_err::AI_ERR_READ => {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            *__errno_location(),
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"%s: read error\0".as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            files0_filename_quoted,
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
                                    b"%s: read error\0".as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                files0_filename_quoted,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                    state.exit_status = EXIT_FAILURE;
                    ok = r#false != 0;
                    break;
                }
                argv_iter_err::AI_ERR_MEM => {
                    xalloc_die();
                }
                _ => {}
            }
            '_c2rust_label_6: {
                if (b"unexpected error code from argv_iter\0".as_ptr()
                    as *const ::core::ffi::c_char)
                    .is_null()
                {
                } else {
                    __assert_fail(
                        b"!\"unexpected error code from argv_iter\"\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/find/ftsfind.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        658 as ::core::ffi::c_uint,
                        b"_Bool process_all_startpoints(int, char **)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
        }
        if *file_name.offset(0isize) == 0 {
            if options.files0_from.is_null() {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        2 as ::core::ffi::c_int,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        safely_quote_err_filename(0 as ::core::ffi::c_int, file_name),
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
                            2 as ::core::ffi::c_int,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            safely_quote_err_filename(0 as ::core::ffi::c_int, file_name),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            } else {
                let mut file_number: ::core::ffi::c_ulong =
                    argv_iter_n_args(ai) as ::core::ffi::c_ulong;
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"%s:%lu: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        files0_filename_quoted,
                        file_number,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"invalid zero-length file name\0".as_ptr()
                                as *const ::core::ffi::c_char,
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
                            b"%s:%lu: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            files0_filename_quoted,
                            file_number,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"invalid zero-length file name\0".as_ptr()
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
            state.exit_status = EXIT_FAILURE;
            ok = r#false != 0;
        } else {
            if options.files0_from.is_null()
                && looks_like_expression(file_name, r#true != 0) as ::core::ffi::c_int != 0
            {
                break;
            }
            state.starting_path_length = strlen(file_name) as ::core::ffi::c_int;
            if find(file_name) {
                continue;
            }
            ok = r#false != 0;
            break;
        }
    }
    argv_iter_free(ai);
    if ok as ::core::ffi::c_int != 0
        && !options.files0_from.is_null()
        && (ferror(stream) != 0 || fclose(stream) != 0 as ::core::ffi::c_int)
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"error reading %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                files0_filename_quoted,
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
                        b"error reading %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    files0_filename_quoted,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return ok;
}
unsafe extern "C" fn rboxc_findutils_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut end_of_leading_options: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut eval_tree: *mut predicate = ::core::ptr::null_mut::<predicate>();
    if !(*argv.offset(0isize)).is_null() {
        set_program_name(*argv.offset(0isize));
    } else {
        set_program_name(b"find\0".as_ptr() as *const ::core::ffi::c_char);
    }
    record_initial_cwd();
    state.already_issued_stat_error_msg = r#false != 0;
    state.exit_status = EXIT_SUCCESS;
    state.execdirs_outstanding = r#false != 0;
    state.cwd_dir_fd = AT_FDCWD;
    if fd_leak_check_is_enabled() {
        remember_non_cloexec_fds();
    }
    state.shared_files = sharefile_init(b"w\0".as_ptr() as *const ::core::ffi::c_char);
    if state.shared_files.is_null() {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Failed to initialize shared-file hash table\0".as_ptr()
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
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Failed to initialize shared-file hash table\0".as_ptr()
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
    set_option_defaults(&raw mut options);
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    if atexit(Some(close_stdout as unsafe extern "C" fn() -> ())) != 0 {
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
    if libc::atexit(rboxc_release_initial_wd) != 0 { return 1; }
    end_of_leading_options = process_leading_options(argc, argv);
    if options.debug_options & DebugOption::DebugStat.0 as ::core::ffi::c_ulong != 0 {
        options.xstat = Some(
            debug_stat
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut stat,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int,
            >;
    }
    if options.debug_options & DebugOption::DebugTime.0 as ::core::ffi::c_ulong != 0 {
        fprintf(
            stderr,
            b"cur_day_start = %s\0".as_ptr() as *const ::core::ffi::c_char,
            ctime(&raw mut options.cur_day_start.tv_sec),
        );
    }
    eval_tree = build_expression_tree(argc, argv, end_of_leading_options);
    !options.open_nofollow_available;
    if process_all_startpoints(
        argc - end_of_leading_options,
        argv.offset(end_of_leading_options as isize),
    ) {
        show_success_rates(eval_tree);
        cleanup();
    }
    return state.exit_status;
}
#[export_name = "rboxc_findutils_find_is_fts_cwdfd_enabled"]
pub unsafe extern "C" fn is_fts_cwdfd_enabled() -> bool {
    return ftsoptions & FTS_CWDFD != 0;
}
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/findutils/share/locale\0",
    )
};
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"findutils\0") };

extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
static mut RBOXC_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_findutils_error_prefix() {
    libc::fprintf(stderr.cast(), b"%s: \0".as_ptr().cast(), RBOXC_INVOCATION);
}

extern "C" {
    #[link_name = "rboxc_findutils_find_initial_wd"]
    static mut rboxc_initial_wd: *mut ::core::ffi::c_void;
    #[link_name = "rboxc_findutils_find_free_cwd"]
    fn rboxc_free_cwd(directory: *mut ::core::ffi::c_void);
    #[link_name = "rboxc_findutils_find_sharefile_destroy"]
    fn rboxc_sharefile_destroy(files: *mut ::core::ffi::c_void);
}
extern "C" fn rboxc_release_initial_wd() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        let files = state.shared_files;
        if !files.is_null() {
            state.shared_files = ::core::ptr::null_mut();
            rboxc_sharefile_destroy(files.cast());
            libc::free(files.cast());
        }
        let directory = rboxc_initial_wd;
        if !directory.is_null() {
            rboxc_initial_wd = ::core::ptr::null_mut();
            rboxc_free_cwd(directory);
            libc::free(directory);
        }
        *libc::__errno_location() = saved_errno;
    }
}

#[no_mangle]
pub unsafe extern "C" fn single_binary_main_find(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    RBOXC_INVOCATION = if argv.is_null() || (*argv).is_null() {
        b"find\0".as_ptr().cast()
    } else { *argv };
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_findutils_error_prefix);
    }
    if libc::atexit(rboxc_release_initial_wd) != 0 { return 1; }
    rboxc_findutils_main_inner(argc, argv)
}
