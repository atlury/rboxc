// Generated from pinned GNU Diffutils 3.12 by scripts/translate-diffutils.py.
// Source SHA-256: f89740750bda61c5fabc71ea26c6ea3a9e4f8623a1e765680e241ac8c559d13e
/* GNU diff - compare files line by line

   Copyright (C) 1988-1989, 1992-1994, 1996, 1998, 2001-2002, 2004, 2006-2007,
   2009-2013, 2015-2025 Free Software Foundation, Inc.

   This file is part of GNU DIFF.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct __dirstream { _opaque: [u8; 0] }
#[repr(C)]
pub struct incomplete { _opaque: [u8; 0] }
#[repr(C)]
pub struct re_dfa_t { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct exclude { _opaque: [u8; 0] }
#[repr(C)]
pub struct allocator { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn readlinkat(
        __fd: ::core::ffi::c_int,
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn openat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strtoimax(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> intmax_t;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn stpcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn free(_: *mut ::core::ffi::c_void);
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
    #[link_name = "rboxc_diffutils_Version"]
    static mut Version: *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_same_file"]
    fn same_file(_: *const stat, _: *const stat) -> bool;
    #[link_name = "rboxc_diffutils_stat_size"]
    fn stat_size(_: *const stat) -> off_t;
    fn re_set_syntax(__syntax: reg_syntax_t) -> reg_syntax_t;
    fn re_compile_pattern(
        __pattern: *const ::core::ffi::c_char,
        __length: size_t,
        __buffer: *mut re_pattern_buffer,
    ) -> *const ::core::ffi::c_char;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fputs_unlocked(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fwrite_unlocked(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_diff_2_files"]
    fn diff_2_files(_: *mut comparison) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_diff_dirs"]
    fn diff_dirs(_: *mut comparison) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_find_dir_file_pathname"]
    fn find_dir_file_pathname(
        _: *mut file_data,
        _: *const ::core::ffi::c_char,
        _: *mut detype,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_pr_program"]
    static pr_program: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_diffutils_cleanup_signal_handlers"]
    fn cleanup_signal_handlers();
    #[link_name = "rboxc_diffutils_fatal"]
    fn fatal(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_message"]
    fn message(_: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_diffutils_perror_with_name"]
    fn perror_with_name(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_pfatal_with_name"]
    fn pfatal_with_name(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_print_message_queue"]
    fn print_message_queue();
    #[link_name = "rboxc_diffutils_set_color_palette"]
    fn set_color_palette(palette: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_c_stack_action"]
    fn c_stack_action(
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_careadlinkat"]
    fn careadlinkat(
        fd: ::core::ffi::c_int,
        filename: *const ::core::ffi::c_char,
        buffer: *mut ::core::ffi::c_char,
        buffer_size: size_t,
        alloc: *const allocator,
        preadlinkat: Option<
            unsafe extern "C" fn(
                ::core::ffi::c_int,
                *const ::core::ffi::c_char,
                *mut ::core::ffi::c_char,
                size_t,
            ) -> ssize_t,
        >,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_squote"]
    fn squote(_: ::core::ffi::c_int, _: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_try_help"]
    fn try_help(_: *const ::core::ffi::c_char, _: *const ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_last_component"]
    fn last_component(filename: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_diffutils_new_exclude"]
    fn new_exclude() -> *mut exclude;
    #[link_name = "rboxc_diffutils_add_exclude"]
    fn add_exclude(_: *mut exclude, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int);
    #[link_name = "rboxc_diffutils_add_exclude_file"]
    fn add_exclude_file(
        _: Option<
            unsafe extern "C" fn(
                *mut exclude,
                *const ::core::ffi::c_char,
                ::core::ffi::c_int,
            ) -> (),
        >,
        _: *mut exclude,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_exit_failure"]
    static mut exit_failure: ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_c_file_type"]
    fn c_file_type(_: *const stat) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_file_name_concat"]
    fn file_name_concat(
        dir: *const ::core::ffi::c_char,
        base: *const ::core::ffi::c_char,
        base_in_result: *mut *mut ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_diffutils_hard_locale"]
    fn hard_locale(category: ::core::ffi::c_int) -> bool;
    #[link_name = "rboxc_diffutils_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_diffutils_quote_n"]
    fn quote_n(
        n: ::core::ffi::c_int,
        arg: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_quote"]
    fn quote(arg: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_shell_quote_length"]
    fn shell_quote_length(string: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_diffutils_shell_quote_copy"]
    fn shell_quote_copy(
        p: *mut ::core::ffi::c_char,
        string: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_diffutils_version_etc"]
    fn version_etc(
        stream: *mut FILE,
        command_name: *const ::core::ffi::c_char,
        package: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_diffutils_emit_bug_reporting_address"]
    fn emit_bug_reporting_address();
    #[link_name = "rboxc_diffutils_xalloc_die"]
    fn xalloc_die();
    #[link_name = "rboxc_diffutils_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_ximalloc"]
    fn ximalloc(s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xinmalloc"]
    fn xinmalloc(n: idx_t, s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xpalloc"]
    fn xpalloc(
        pa: *mut ::core::ffi::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_diffutils_xstdopen"]
    fn xstdopen();
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
pub type mode_t = __mode_t;
pub type off_t = __off_t;
pub type ssize_t = isize;
pub type ptrdiff_t = isize;
pub type size_t = usize;
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
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type DIR = __dirstream;
pub type intmax_t = ::libc::intmax_t;
pub type idx_t = ptrdiff_t;
pub type word = *mut incomplete;
pub type lin = ptrdiff_t;
pub type __re_long_size_t = ::core::ffi::c_ulong;
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
pub struct changes(pub ::core::ffi::c_uint);
impl changes {
    pub const UNCHANGED: Self = Self(0);
    pub const OLD: Self = Self(1);
    pub const NEW: Self = Self(2);
    pub const CHANGED: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct colors_style(pub ::core::ffi::c_uint);
impl colors_style {
    pub const NEVER: Self = Self(0);
    pub const AUTO: Self = Self(1);
    pub const ALWAYS: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct output_style(pub ::core::ffi::c_uint);
impl output_style {
    pub const OUTPUT_UNSPECIFIED: Self = Self(0);
    pub const OUTPUT_NORMAL: Self = Self(1);
    pub const OUTPUT_CONTEXT: Self = Self(2);
    pub const OUTPUT_UNIFIED: Self = Self(3);
    pub const OUTPUT_ED: Self = Self(4);
    pub const OUTPUT_FORWARD_ED: Self = Self(5);
    pub const OUTPUT_RCS: Self = Self(6);
    pub const OUTPUT_IFDEF: Self = Self(7);
    pub const OUTPUT_SDIFF: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DIFF_white_space(pub ::core::ffi::c_uint);
impl DIFF_white_space {
    pub const IGNORE_NO_WHITE_SPACE: Self = Self(0);
    pub const IGNORE_TAB_EXPANSION: Self = Self(1);
    pub const IGNORE_TRAILING_SPACE: Self = Self(2);
    pub const IGNORE_TAB_EXPANSION_AND_TRAILING_SPACE: Self = Self(3);
    pub const IGNORE_SPACE_CHANGE: Self = Self(4);
    pub const IGNORE_ALL_SPACE: Self = Self(5);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct detype(pub ::core::ffi::c_uint);
impl detype {
    pub const DE_UNKNOWN: Self = Self(0);
    pub const DE_FIFO: Self = Self(1);
    pub const DE_CHR: Self = Self(2);
    pub const DE_DIR: Self = Self(4);
    pub const DE_BLK: Self = Self(6);
    pub const DE_REG: Self = Self(8);
    pub const DE_LNK: Self = Self(10);
    pub const DE_SOCK: Self = Self(12);
    pub const DE_WHT: Self = Self(14);
    pub const DE_OTHER: Self = Self(15);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_data {
    pub desc: ::core::ffi::c_int,
    pub openerr: ::core::ffi::c_int,
    pub err: ::core::ffi::c_int,
    pub name: *const ::core::ffi::c_char,
    pub filetype: *const ::core::ffi::c_char,
    pub stat: stat,
    pub dirstream: *mut DIR,
    pub buffer: *mut word,
    pub bufsize: idx_t,
    pub buffered: idx_t,
    pub linbuf: *mut *const ::core::ffi::c_char,
    pub linbuf_base: lin,
    pub buffered_lines: lin,
    pub valid_lines: lin,
    pub alloc_lines: lin,
    pub prefix_end: *const ::core::ffi::c_char,
    pub prefix_lines: lin,
    pub suffix_begin: *const ::core::ffi::c_char,
    pub equivs: *mut lin,
    pub undiscarded: *mut lin,
    pub realindexes: *mut lin,
    pub nondiscarded_lines: lin,
    pub changed: *mut bool,
    pub missing_newline: bool,
    pub eof: bool,
    pub equiv_max: lin,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_int);
impl C2Rust_Unnamed {
    pub const OPEN_FAILED: Self = Self(-1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_0 {
    pub const NONEXISTENT: Self = Self(-2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_1 {
    pub const UNOPENED: Self = Self(-3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comparison {
    pub file: [file_data; 2],
    pub parent: *const comparison,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const n_num: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const EXIT_TROUBLE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regexp_list {
    pub regexps: *mut ::core::ffi::c_char,
    pub len: idx_t,
    pub size: idx_t,
    pub multiple_regexps: bool,
    pub buf: *mut re_pattern_buffer,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const binary: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const O_PATH_DEFINED: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const O_PATHSEARCH: Self = Self(2097152);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const BINARY_OPTION: Self = Self(128);
    pub const FROM_FILE_OPTION: Self = Self(129);
    pub const HELP_OPTION: Self = Self(130);
    pub const HORIZON_LINES_OPTION: Self = Self(131);
    pub const IGNORE_FILE_NAME_CASE_OPTION: Self = Self(132);
    pub const INHIBIT_HUNK_MERGE_OPTION: Self = Self(133);
    pub const LEFT_COLUMN_OPTION: Self = Self(134);
    pub const LINE_FORMAT_OPTION: Self = Self(135);
    pub const NO_DEREFERENCE_OPTION: Self = Self(136);
    pub const NO_IGNORE_FILE_NAME_CASE_OPTION: Self = Self(137);
    pub const NORMAL_OPTION: Self = Self(138);
    pub const SDIFF_MERGE_ASSIST_OPTION: Self = Self(139);
    pub const STRIP_TRAILING_CR_OPTION: Self = Self(140);
    pub const SUPPRESS_BLANK_EMPTY_OPTION: Self = Self(141);
    pub const SUPPRESS_COMMON_LINES_OPTION: Self = Self(142);
    pub const TABSIZE_OPTION: Self = Self(143);
    pub const TO_FILE_OPTION: Self = Self(144);
    pub const UNCHANGED_LINE_FORMAT_OPTION: Self = Self(145);
    pub const OLD_LINE_FORMAT_OPTION: Self = Self(146);
    pub const NEW_LINE_FORMAT_OPTION: Self = Self(147);
    pub const UNCHANGED_GROUP_FORMAT_OPTION: Self = Self(148);
    pub const OLD_GROUP_FORMAT_OPTION: Self = Self(149);
    pub const NEW_GROUP_FORMAT_OPTION: Self = Self(150);
    pub const CHANGED_GROUP_FORMAT_OPTION: Self = Self(151);
    pub const COLOR_OPTION: Self = Self(152);
    pub const COLOR_PALETTE_OPTION: Self = Self(153);
    pub const NO_DIRECTORY_OPTION: Self = Self(154);
    pub const PRESUME_OUTPUT_TTY_OPTION: Self = Self(155);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const NOFOLLOW_SYMLINK_ERRNO: Self = Self(40);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const ENAMETOOLONG: ::core::ffi::c_int = 36 as ::core::ffi::c_int;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const EOVERFLOW: ::core::ffi::c_int = 75 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EACCES: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const ENOTDIR: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const __LC_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const INTMAX_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const PTRDIFF_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const LC_TIME: ::core::ffi::c_int = __LC_TIME;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const LIN_MAX: ::core::ffi::c_long = PTRDIFF_MAX;
pub const RE_BACKSLASH_ESCAPE_IN_LISTS: ::core::ffi::c_ulong =
    1 as ::core::ffi::c_int as ::core::ffi::c_ulong;
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
pub const RE_CARET_ANCHORS_HERE: ::core::ffi::c_ulong = RE_ICASE << 1 as ::core::ffi::c_int;
pub const RE_CONTEXT_INVALID_DUP: ::core::ffi::c_ulong =
    RE_CARET_ANCHORS_HERE << 1 as ::core::ffi::c_int;
pub const RE_SYNTAX_GREP: ::core::ffi::c_ulong =
    (RE_SYNTAX_POSIX_BASIC | RE_NEWLINE_ALT) & !(RE_CONTEXT_INVALID_DUP | RE_DOT_NOT_NULL);
pub const _RE_SYNTAX_POSIX_COMMON: ::core::ffi::c_ulong =
    RE_CHAR_CLASSES | RE_DOT_NEWLINE | RE_DOT_NOT_NULL | RE_INTERVALS | RE_NO_EMPTY_RANGES;
pub const RE_SYNTAX_POSIX_BASIC: ::core::ffi::c_ulong =
    _RE_SYNTAX_POSIX_COMMON | RE_BK_PLUS_QM | RE_CONTEXT_INVALID_DUP;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn putc_unlocked(
    mut __c: ::core::ffi::c_int,
    mut __stream: *mut FILE,
) -> ::core::ffi::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(__stream, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh0;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(stdout, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh1 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh1;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
#[export_name = "rboxc_diffutils_robust_output_style"]
#[inline]
pub unsafe extern "C" fn robust_output_style(mut s: output_style) -> bool {
    return s.0 != output_style::OUTPUT_ED.0 && s.0 != output_style::OUTPUT_FORWARD_ED.0;
}
#[inline]
unsafe extern "C" fn __gl_setmode(
    mut fd: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return O_BINARY;
}
#[inline]
unsafe extern "C" fn set_binary_mode(
    mut fd: ::core::ffi::c_int,
    mut mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return __gl_setmode(fd, mode);
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: ::core::ffi::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return r#true != 0,
        _ => return r#false != 0,
    };
}
pub const EXCLUDE_WILDCARDS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int;
pub const FNM_CASEFOLD: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut PROGRAM_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"diff\0") };
pub const GUTTER_WIDTH_MINIMUM: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
static mut recursive: bool = false;
static mut function_regexp_list: regexp_list = regexp_list {
    regexps: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    len: 0,
    size: 0,
    multiple_regexps: false,
    buf: ::core::ptr::null_mut::<re_pattern_buffer>(),
};
static mut ignore_regexp_list: regexp_list = regexp_list {
    regexps: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    len: 0,
    size: 0,
    multiple_regexps: false,
    buf: ::core::ptr::null_mut::<re_pattern_buffer>(),
};
static mut new_file: bool = false;
static mut unidirectional_new_file: bool = false;
static mut report_identical_files: bool = false;
static mut no_directory: bool = false;
static mut group_format_option: [[::core::ffi::c_char; 25]; 4] = unsafe {
    [
        ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(
            *b"--unchanged-group-format\0",
        ),
        ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(
            *b"--old-group-format\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(
            *b"--new-group-format\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<[u8; 25], [::core::ffi::c_char; 25]>(
            *b"--changed-group-format\0\0\0",
        ),
    ]
};
static mut line_format_option: [[::core::ffi::c_char; 24]; 3] = unsafe {
    [
        ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(
            *b"--unchanged-line-format\0",
        ),
        ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(
            *b"--old-line-format\0\0\0\0\0\0\0",
        ),
        ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(
            *b"--new-line-format\0\0\0\0\0\0\0",
        ),
    ]
};
static mut shortopts: [::core::ffi::c_char; 57] = unsafe {
    ::core::mem::transmute::<[u8; 57], [::core::ffi::c_char; 57]>(
        *b"0123456789abBcC:dD:eEfF:hHiI:lL:nNpPqrsS:tTuU:vwW:x:X:yZ\0",
    )
};
static mut longopts: [option; 62] = [
    option {
        name: b"binary\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::BINARY_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"brief\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'q' as ::core::ffi::c_int,
    },
    option {
        name: b"changed-group-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::CHANGED_GROUP_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"color\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 2 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::COLOR_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"context\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 2 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'C' as ::core::ffi::c_int,
    },
    option {
        name: b"ed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'e' as ::core::ffi::c_int,
    },
    option {
        name: b"exclude\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'x' as ::core::ffi::c_int,
    },
    option {
        name: b"exclude-from\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'X' as ::core::ffi::c_int,
    },
    option {
        name: b"expand-tabs\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 't' as ::core::ffi::c_int,
    },
    option {
        name: b"forward-ed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'f' as ::core::ffi::c_int,
    },
    option {
        name: b"from-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::FROM_FILE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::HELP_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"horizon-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::HORIZON_LINES_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"ifdef\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'D' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-all-space\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'w' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-blank-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'B' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-case\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'i' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-file-name-case\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::IGNORE_FILE_NAME_CASE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-matching-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'I' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-space-change\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-tab-expansion\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'E' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-trailing-space\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'Z' as ::core::ffi::c_int,
    },
    option {
        name: b"inhibit-hunk-merge\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::INHIBIT_HUNK_MERGE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"initial-tab\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'T' as ::core::ffi::c_int,
    },
    option {
        name: b"label\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'L' as ::core::ffi::c_int,
    },
    option {
        name: b"left-column\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::LEFT_COLUMN_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"line-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::LINE_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"minimal\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"new-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'N' as ::core::ffi::c_int,
    },
    option {
        name: b"new-group-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NEW_GROUP_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"new-line-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NEW_LINE_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"no-dereference\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NO_DEREFERENCE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"no-ignore-file-name-case\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NO_IGNORE_FILE_NAME_CASE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"normal\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NORMAL_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"old-group-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::OLD_GROUP_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"old-line-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::OLD_LINE_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"paginate\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"palette\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::COLOR_PALETTE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"rcs\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"recursive\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"report-identical-files\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"sdiff-merge-assist\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SDIFF_MERGE_ASSIST_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"show-c-function\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"show-function-line\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'F' as ::core::ffi::c_int,
    },
    option {
        name: b"side-by-side\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'y' as ::core::ffi::c_int,
    },
    option {
        name: b"speed-large-files\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'H' as ::core::ffi::c_int,
    },
    option {
        name: b"starting-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'S' as ::core::ffi::c_int,
    },
    option {
        name: b"strip-trailing-cr\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::STRIP_TRAILING_CR_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"suppress-blank-empty\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SUPPRESS_BLANK_EMPTY_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"suppress-common-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::SUPPRESS_COMMON_LINES_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"tabsize\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::TABSIZE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"text\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'a' as ::core::ffi::c_int,
    },
    option {
        name: b"to-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::TO_FILE_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"unchanged-group-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::UNCHANGED_GROUP_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"unchanged-line-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::UNCHANGED_LINE_FORMAT_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"unidirectional-new-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'P' as ::core::ffi::c_int,
    },
    option {
        name: b"unified\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 2 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'U' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"width\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: 1 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'W' as ::core::ffi::c_int,
    },
    option {
        name: b"-no-directory\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::NO_DIRECTORY_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: b"-presume-output-tty\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_7::PRESUME_OUTPUT_TTY_OPTION.0 as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn option_list(
    mut optionvec: *mut *mut ::core::ffi::c_char,
    mut count: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut size: idx_t = 1 as idx_t;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < count {
        let mut optsize: size_t =
            (1 as size_t).wrapping_add(shell_quote_length(*optionvec.offset(i as isize)));
        let (c2rust_result, c2rust_overflowed) = (size as i128).overflowing_add(optsize as i128);
        let c2rust_result_narrow = c2rust_result as idx_t;
        *&raw mut size = c2rust_result_narrow;
        if c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result {
            xalloc_die();
        }
        i += 1;
    }
    let mut result: *mut ::core::ffi::c_char = ximalloc(size) as *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = result;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < count {
        let c2rust_fresh5 = p;
        p = p.offset(1);
        *c2rust_fresh5 = ' ' as ::core::ffi::c_char;
        p = shell_quote_copy(p, *optionvec.offset(i_0 as isize));
        i_0 += 1;
    }
    *p = '\0' as ::core::ffi::c_char;
    return result;
}
unsafe extern "C" fn exclude_options() -> ::core::ffi::c_int {
    return EXCLUDE_WILDCARDS
        | if ignore_file_name_case as ::core::ffi::c_int != 0 {
            FNM_CASEFOLD
        } else {
            0 as ::core::ffi::c_int
        };
}
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut rboxc_diffutils_stderr: *mut libc::FILE;
}
unsafe extern "C" fn rboxc_diffutils_error_prefix() {
    libc::fprintf(rboxc_diffutils_stderr, b"%s: \0".as_ptr().cast(), program_name);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_diff(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    ::core::ptr::write_volatile(
        &raw mut exit_failure,
        C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int,
    );
    set_program_name(*argv.offset(0isize));
    let prior_error_prefix = error_print_progname;
    if prior_error_prefix.is_none() {
        error_print_progname = Some(rboxc_diffutils_error_prefix);
    }
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    bindtextdomain(
        b"gnulib\0".as_ptr() as *const ::core::ffi::c_char,
        GNULIB_LOCALEDIR.as_ptr(),
    );
    textdomain(PACKAGE.as_ptr());
    c_stack_action(None);
    function_regexp_list.buf = &raw mut function_regexp;
    ignore_regexp_list.buf = &raw mut ignore_regexp;
    re_set_syntax(RE_SYNTAX_GREP | RE_NO_POSIX_BACKTRACKING);
    excluded = new_exclude() as *mut exclude;
    presume_output_tty = r#false != 0;
    xstdopen();
    let mut ocontext: lin = -1 as lin;
    let mut explicit_context: bool = r#false != 0;
    let mut width: intmax_t = 0 as intmax_t;
    let mut show_c_function: bool = r#false != 0;
    let mut from_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut to_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut prev: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = 0;
    loop {
        c = getopt_long(
            argc,
            argv,
            &raw const shortopts as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if 0 as ::core::ffi::c_int > c {
            break;
        }
        match c {
            48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                if !c_isdigit(prev) {
                    ocontext = 0 as lin;
                }
                let (c2rust_result, c2rust_overflowed) =
                    (ocontext as i128).overflowing_mul(10 as ::core::ffi::c_int as i128);
                let c2rust_result_narrow = c2rust_result as lin;
                *&raw mut ocontext = c2rust_result_narrow;
                if c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result || {
                    let (c2rust_result_0, c2rust_overflowed_0) =
                        (ocontext as i128).overflowing_add((c - '0' as ::core::ffi::c_int) as i128);
                    let c2rust_result_narrow_0 = c2rust_result_0 as lin;
                    *&raw mut ocontext = c2rust_result_narrow_0;
                    c2rust_overflowed_0 || c2rust_result_narrow_0 as i128 != c2rust_result_0
                } {
                    ocontext = LIN_MAX as lin;
                }
            }
            97 => {
                text = r#true != 0;
            }
            98 => {
                if ignore_white_space.0 < DIFF_white_space::IGNORE_SPACE_CHANGE.0 {
                    ignore_white_space = DIFF_white_space::IGNORE_SPACE_CHANGE;
                }
            }
            90 => {
                if ignore_white_space.0 < DIFF_white_space::IGNORE_SPACE_CHANGE.0 {
                    ignore_white_space = DIFF_white_space(
                        ignore_white_space.0 | DIFF_white_space::IGNORE_TRAILING_SPACE.0,
                    );
                }
            }
            66 => {
                ignore_blank_lines = r#true != 0;
            }
            67 | 85 => {
                let mut numval: intmax_t = 0;
                if !optarg.is_null() {
                    let mut numend: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    numval = strtoimax(optarg, &raw mut numend, 10 as ::core::ffi::c_int);
                    if *numend as ::core::ffi::c_int != 0 || numval < 0 as intmax_t {
                        try_help(
                            b"invalid context length %s\0".as_ptr() as *const ::core::ffi::c_char,
                            quote(optarg),
                        );
                    }
                } else {
                    numval = 3 as intmax_t;
                }
                specify_style(output_style(
                    (if c == 'U' as ::core::ffi::c_int {
                        output_style::OUTPUT_UNIFIED.0 as ::core::ffi::c_int
                    } else {
                        output_style::OUTPUT_CONTEXT.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ));
                if context < numval as lin {
                    context = (if numval < 9223372036854775807 as intmax_t {
                        numval
                    } else {
                        9223372036854775807 as intmax_t
                    }) as lin;
                }
                explicit_context = r#true != 0;
            }
            99 => {
                specify_style(output_style::OUTPUT_CONTEXT);
                if context < 3 as lin {
                    context = 3 as lin;
                }
            }
            100 => {
                minimal = r#true != 0;
            }
            68 => {
                specify_style(output_style::OUTPUT_IFDEF);
                static mut C_ifdef_group_formats: [::core::ffi::c_char; 104] = unsafe {
                    ::core::mem::transmute::<
                        [u8; 104],
                        [::core::ffi::c_char; 104],
                    >(
                        *b"%=\0#ifndef @\n%<#endif /* ! @ */\n\0#ifdef @\n%>#endif /* @ */\n\0#ifndef @\n%<#else /* @ */\n%>#endif /* @ */\n\0",
                    )
                };
                let mut nats: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
                let mut b: *mut ::core::ffi::c_char = xinmalloc(
                    ::core::mem::size_of::<[::core::ffi::c_char; 104]>()
                        .wrapping_add(1usize)
                        .wrapping_div(nats as usize)
                        .wrapping_add(strlen(optarg)) as idx_t,
                    nats as idx_t,
                ) as *mut ::core::ffi::c_char;
                let mut base: *mut ::core::ffi::c_char = b;
                let mut changes_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while (i as usize) < ::core::mem::size_of::<[::core::ffi::c_char; 104]>() {
                    let mut ch: ::core::ffi::c_char = C_ifdef_group_formats[i as usize];
                    match ch as ::core::ffi::c_int {
                        64 => {
                            b = stpcpy(b, optarg);
                        }
                        0 => {
                            let c2rust_fresh7 = b;
                            b = b.offset(1);
                            *c2rust_fresh7 = ch;
                            let c2rust_fresh8 = changes_0;
                            changes_0 += 1;
                            specify_value(
                                (&raw mut group_format as *mut *const ::core::ffi::c_char)
                                    .offset(c2rust_fresh8 as isize),
                                base,
                                b"-D\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            base = b;
                        }
                        _ => {
                            let c2rust_fresh6 = b;
                            b = b.offset(1);
                            *c2rust_fresh6 = ch;
                        }
                    }
                    i += 1;
                }
            }
            101 => {
                specify_style(output_style::OUTPUT_ED);
            }
            69 => {
                if ignore_white_space.0 < DIFF_white_space::IGNORE_SPACE_CHANGE.0 {
                    ignore_white_space = DIFF_white_space(
                        ignore_white_space.0 | DIFF_white_space::IGNORE_TAB_EXPANSION.0,
                    );
                }
            }
            102 => {
                specify_style(output_style::OUTPUT_FORWARD_ED);
            }
            70 => {
                add_regexp(&raw mut function_regexp_list, optarg);
            }
            72 => {
                speed_large_files = r#true != 0;
            }
            105 => {
                ignore_case = r#true != 0;
            }
            73 => {
                add_regexp(&raw mut ignore_regexp_list, optarg);
            }
            108 => {
                if *(&raw const pr_program as *const ::core::ffi::c_char).offset(0isize) == 0 {
                    try_help(
                        b"pagination not supported on this host\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                }
                paginate = r#true != 0;
                signal(SIGCHLD, SIG_DFL);
            }
            76 => {
                if file_label[0usize].is_null() {
                    file_label[0usize] = optarg;
                } else if file_label[1usize].is_null() {
                    file_label[1usize] = optarg;
                } else {
                    fatal(b"too many file label options\0".as_ptr() as *const ::core::ffi::c_char);
                }
            }
            110 => {
                specify_style(output_style::OUTPUT_RCS);
            }
            78 => {
                new_file = r#true != 0;
            }
            112 => {
                show_c_function = r#true != 0;
                add_regexp(
                    &raw mut function_regexp_list,
                    b"^[[:alpha:]$_]\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            80 => {
                unidirectional_new_file = r#true != 0;
            }
            113 => {
                brief = r#true != 0;
            }
            114 => {
                recursive = r#true != 0;
            }
            115 => {
                report_identical_files = r#true != 0;
            }
            83 => {
                specify_value(
                    &raw mut starting_file,
                    optarg,
                    b"-S\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            116 => {
                expand_tabs = r#true != 0;
            }
            84 => {
                initial_tab = r#true != 0;
            }
            117 => {
                specify_style(output_style::OUTPUT_UNIFIED);
                if context < 3 as lin {
                    context = 3 as lin;
                }
            }
            118 => {
                version_etc(
                    stdout,
                    &raw const PROGRAM_NAME as *const ::core::ffi::c_char,
                    PACKAGE_NAME.as_ptr(),
                    Version,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Paul Eggert\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Mike Haertel\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"David Hayes\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Richard Stallman\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Len Tower\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    nullptr,
                );
                check_stdout();
                return EXIT_SUCCESS;
            }
            119 => {
                ignore_white_space = DIFF_white_space::IGNORE_ALL_SPACE;
            }
            120 => {
                add_exclude(excluded, optarg, exclude_options());
            }
            88 => {
                if add_exclude_file(
                    Some(
                        add_exclude
                            as unsafe extern "C" fn(
                                *mut exclude,
                                *const ::core::ffi::c_char,
                                ::core::ffi::c_int,
                            ) -> (),
                    ),
                    excluded,
                    optarg,
                    exclude_options(),
                    '\n' as ::core::ffi::c_char,
                ) != 0
                {
                    pfatal_with_name(optarg);
                }
            }
            121 => {
                specify_style(output_style::OUTPUT_SDIFF);
            }
            87 => {
                let mut numend_0: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut numval_0: intmax_t =
                    strtoimax(optarg, &raw mut numend_0, 10 as ::core::ffi::c_int);
                if numval_0 <= 0 as intmax_t || *numend_0 as ::core::ffi::c_int != 0 {
                    try_help(
                        b"invalid width %s\0".as_ptr() as *const ::core::ffi::c_char,
                        quote(optarg),
                    );
                }
                if width != numval_0 {
                    if width != 0 {
                        fatal(b"conflicting width options\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    width = numval_0;
                }
            }
            129 => {
                specify_value(
                    &raw mut from_file,
                    optarg,
                    b"--from-file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            130 => {
                usage();
                check_stdout();
                return EXIT_SUCCESS;
            }
            131 => {
                let mut numend_1: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut numval_1: intmax_t =
                    strtoimax(optarg, &raw mut numend_1, 10 as ::core::ffi::c_int);
                if *numend_1 as ::core::ffi::c_int != 0 || numval_1 < 0 as intmax_t {
                    try_help(
                        b"invalid horizon length %s\0".as_ptr() as *const ::core::ffi::c_char,
                        quote(optarg),
                    );
                }
                horizon_lines = if horizon_lines
                    > if numval_1 < 9223372036854775807 as intmax_t {
                        numval_1 as lin
                    } else {
                        9223372036854775807 as lin
                    } {
                    horizon_lines
                } else if numval_1 < 9223372036854775807 as intmax_t {
                    numval_1 as lin
                } else {
                    9223372036854775807 as lin
                };
            }
            132 => {
                ignore_file_name_case = r#true != 0;
            }
            0 | 104 | 128 | 133 => {}
            134 => {
                left_column = r#true != 0;
            }
            135 => {
                specify_style(output_style::OUTPUT_IFDEF);
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while (i_0 as usize)
                    < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
                        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
                {
                    specify_value(
                        (&raw mut line_format as *mut *const ::core::ffi::c_char)
                            .offset(i_0 as isize),
                        optarg,
                        b"--line-format\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    i_0 += 1;
                }
            }
            136 => {
                no_dereference_symlinks = r#true != 0;
            }
            137 => {
                ignore_file_name_case = r#false != 0;
            }
            138 => {
                specify_style(output_style::OUTPUT_NORMAL);
            }
            139 => {
                specify_style(output_style::OUTPUT_SDIFF);
                sdiff_merge_assist = r#true != 0;
            }
            140 => {
                strip_trailing_cr = r#true != 0;
            }
            141 => {
                suppress_blank_empty = r#true != 0;
            }
            142 => {
                suppress_common_lines = r#true != 0;
            }
            143 => {
                let mut numend_2: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut numval_2: intmax_t =
                    strtoimax(optarg, &raw mut numend_2, 10 as ::core::ffi::c_int);
                if !((0 as intmax_t) < numval_2
                    && numval_2 <= INTMAX_MAX as intmax_t - GUTTER_WIDTH_MINIMUM as intmax_t)
                    || *numend_2 as ::core::ffi::c_int != 0
                {
                    try_help(
                        b"invalid tabsize %s\0".as_ptr() as *const ::core::ffi::c_char,
                        quote(optarg),
                    );
                }
                if tabsize != numval_2 {
                    if tabsize != 0 {
                        fatal(
                            b"conflicting tabsize options\0".as_ptr() as *const ::core::ffi::c_char
                        );
                    }
                    tabsize = numval_2;
                }
            }
            144 => {
                specify_value(
                    &raw mut to_file,
                    optarg,
                    b"--to-file\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            145 | 146 | 147 => {
                specify_style(output_style::OUTPUT_IFDEF);
                c -= C2Rust_Unnamed_7::UNCHANGED_LINE_FORMAT_OPTION.0 as ::core::ffi::c_int;
                specify_value(
                    (&raw mut line_format as *mut *const ::core::ffi::c_char).offset(c as isize),
                    optarg,
                    &raw const *(&raw const line_format_option as *const [::core::ffi::c_char; 24])
                        .offset(c as isize) as *const ::core::ffi::c_char,
                );
            }
            148 | 149 | 150 | 151 => {
                specify_style(output_style::OUTPUT_IFDEF);
                c -= C2Rust_Unnamed_7::UNCHANGED_GROUP_FORMAT_OPTION.0 as ::core::ffi::c_int;
                specify_value(
                    (&raw mut group_format as *mut *const ::core::ffi::c_char).offset(c as isize),
                    optarg,
                    &raw const *(&raw const group_format_option as *const [::core::ffi::c_char; 25])
                        .offset(c as isize) as *const ::core::ffi::c_char,
                );
            }
            152 => {
                specify_colors_style(optarg);
            }
            153 => {
                set_color_palette(optarg);
            }
            154 => {
                no_directory = r#true != 0;
            }
            155 => {
                presume_output_tty = r#true != 0;
            }
            _ => {
                try_help(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        prev = c;
    }
    if colors_style_0.0 == colors_style::AUTO.0 {
        let mut t: *const ::core::ffi::c_char =
            getenv(b"TERM\0".as_ptr() as *const ::core::ffi::c_char);
        if !t.is_null()
            && strcmp(t, b"dumb\0".as_ptr() as *const ::core::ffi::c_char)
                == 0 as ::core::ffi::c_int
        {
            colors_style_0 = colors_style::NEVER;
        }
    }
    if output_style_0.0 == output_style::OUTPUT_UNSPECIFIED.0 {
        if show_c_function {
            specify_style(output_style::OUTPUT_CONTEXT);
            if ocontext < 0 as lin {
                context = 3 as lin;
            }
        } else {
            specify_style(output_style::OUTPUT_NORMAL);
        }
    }
    if output_style_0.0 != output_style::OUTPUT_CONTEXT.0
        || hard_locale(LC_TIME) as ::core::ffi::c_int != 0
    {
        time_format = b"%Y-%m-%d %H:%M:%S.%N %z\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        time_format = b"%a %b %e %T %Y\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if 0 as lin <= ocontext
        && (output_style_0.0 == output_style::OUTPUT_CONTEXT.0
            || output_style_0.0 == output_style::OUTPUT_UNIFIED.0)
        && (context < ocontext || ocontext < context && !explicit_context)
    {
        context = ocontext;
    }
    if tabsize == 0 {
        tabsize = 8 as intmax_t;
    }
    if width == 0 {
        width = 130 as intmax_t;
    }
    let mut t_0: intmax_t = if expand_tabs as ::core::ffi::c_int != 0 {
        1 as intmax_t
    } else {
        tabsize
    };
    let mut w: intmax_t = width;
    let mut t_plus_g: intmax_t = t_0 + GUTTER_WIDTH_MINIMUM as intmax_t;
    let mut unaligned_off: intmax_t = (w >> 1 as ::core::ffi::c_int)
        + (t_plus_g >> 1 as ::core::ffi::c_int)
        + (w & t_plus_g & 1 as intmax_t);
    let mut off: intmax_t = unaligned_off - unaligned_off % t_0;
    sdiff_half_width = if 0 as intmax_t
        > if (off - 3 as intmax_t) < w - off {
            off - 3 as intmax_t
        } else {
            w - off
        } {
        0 as intmax_t
    } else if (off - 3 as intmax_t) < w - off {
        off - 3 as intmax_t
    } else {
        w - off
    };
    sdiff_column2_offset = if sdiff_half_width != 0 { off } else { w };
    if horizon_lines < context {
        horizon_lines = context;
    }
    summarize_regexp_list(&raw mut function_regexp_list);
    summarize_regexp_list(&raw mut ignore_regexp_list);
    if output_style_0.0 == output_style::OUTPUT_IFDEF.0 {
        let mut i_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while (i_1 as usize)
            < ::core::mem::size_of::<[*const ::core::ffi::c_char; 3]>()
                .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
        {
            if line_format[i_1 as usize].is_null() {
                line_format[i_1 as usize] = b"%l\n\0".as_ptr() as *const ::core::ffi::c_char;
            }
            i_1 += 1;
        }
        if group_format[changes::OLD.0 as ::core::ffi::c_int as usize].is_null() {
            group_format[changes::OLD.0 as ::core::ffi::c_int as usize] =
                if !group_format[changes::CHANGED.0 as ::core::ffi::c_int as usize].is_null() {
                    group_format[changes::CHANGED.0 as ::core::ffi::c_int as usize]
                } else {
                    b"%<\0".as_ptr() as *const ::core::ffi::c_char
                };
        }
        if group_format[changes::NEW.0 as ::core::ffi::c_int as usize].is_null() {
            group_format[changes::NEW.0 as ::core::ffi::c_int as usize] =
                if !group_format[changes::CHANGED.0 as ::core::ffi::c_int as usize].is_null() {
                    group_format[changes::CHANGED.0 as ::core::ffi::c_int as usize]
                } else {
                    b"%>\0".as_ptr() as *const ::core::ffi::c_char
                };
        }
        if group_format[changes::UNCHANGED.0 as ::core::ffi::c_int as usize].is_null() {
            group_format[changes::UNCHANGED.0 as ::core::ffi::c_int as usize] =
                b"%=\0".as_ptr() as *const ::core::ffi::c_char;
        }
        if group_format[changes::CHANGED.0 as ::core::ffi::c_int as usize].is_null() {
            let mut p: *mut ::core::ffi::c_char = xmalloc(
                strlen(group_format[changes::OLD.0 as ::core::ffi::c_int as usize])
                    .wrapping_add(strlen(
                        group_format[changes::NEW.0 as ::core::ffi::c_int as usize],
                    ))
                    .wrapping_add(1 as size_t),
            ) as *mut ::core::ffi::c_char;
            group_format[changes::CHANGED.0 as ::core::ffi::c_int as usize] = p;
            strcpy(
                stpcpy(
                    p,
                    group_format[changes::OLD.0 as ::core::ffi::c_int as usize],
                ),
                group_format[changes::NEW.0 as ::core::ffi::c_int as usize],
            );
        }
    }
    no_diff_means_no_output = if output_style_0.0 == output_style::OUTPUT_IFDEF.0 {
        (*group_format[changes::UNCHANGED.0 as ::core::ffi::c_int as usize] == 0
            || strcmp(
                group_format[changes::UNCHANGED.0 as ::core::ffi::c_int as usize],
                b"%=\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
                && *line_format[changes::UNCHANGED.0 as ::core::ffi::c_int as usize] == 0)
            as ::core::ffi::c_int
    } else {
        (output_style_0.0 != output_style::OUTPUT_SDIFF.0) as ::core::ffi::c_int
            | suppress_common_lines as ::core::ffi::c_int
    } != 0;
    files_can_be_treated_as_binary = brief as ::core::ffi::c_int
        & C2Rust_Unnamed_4::binary.0 as ::core::ffi::c_int
        & !(ignore_blank_lines as ::core::ffi::c_int
            | ignore_case as ::core::ffi::c_int
            | strip_trailing_cr as ::core::ffi::c_int
            | (!ignore_regexp_list.regexps.is_null() || ignore_white_space.0 != 0)
                as ::core::ffi::c_int)
        != 0;
    switch_string = option_list(
        argv.offset(1 as ::core::ffi::c_int as isize),
        optind - 1 as ::core::ffi::c_int,
    );
    let mut exit_status: ::core::ffi::c_int = EXIT_SUCCESS;
    noparent.file[0usize].desc = AT_FDCWD;
    noparent.file[1usize].desc = AT_FDCWD;
    static mut de_unknowns: [detype; 2] = [detype::DE_UNKNOWN, detype::DE_UNKNOWN];
    if !from_file.is_null() {
        if !to_file.is_null() {
            fatal(b"--from-file and --to-file both specified\0".as_ptr()
                as *const ::core::ffi::c_char);
        } else {
            while optind < argc {
                let mut status: ::core::ffi::c_int = compare_files(
                    &raw mut noparent,
                    &raw const de_unknowns as *const detype,
                    from_file,
                    *argv.offset(optind as isize),
                );
                if exit_status < status {
                    exit_status = status;
                }
                optind += 1;
            }
        }
    } else if !to_file.is_null() {
        while optind < argc {
            let mut status_0: ::core::ffi::c_int = compare_files(
                &raw mut noparent,
                &raw const de_unknowns as *const detype,
                *argv.offset(optind as isize),
                to_file,
            );
            if exit_status < status_0 {
                exit_status = status_0;
            }
            optind += 1;
        }
    } else {
        if argc - optind != 2 as ::core::ffi::c_int {
            if argc - optind < 2 as ::core::ffi::c_int {
                try_help(
                    b"missing operand after %s\0".as_ptr() as *const ::core::ffi::c_char,
                    quote(*argv.offset((argc - 1 as ::core::ffi::c_int) as isize)),
                );
            } else {
                try_help(
                    b"extra operand %s\0".as_ptr() as *const ::core::ffi::c_char,
                    quote(*argv.offset((optind + 2 as ::core::ffi::c_int) as isize)),
                );
            }
        }
        exit_status = compare_files(
            &raw mut noparent,
            &raw const de_unknowns as *const detype,
            *argv.offset(optind as isize),
            *argv.offset((optind + 1 as ::core::ffi::c_int) as isize),
        );
    }
    print_message_queue();
    check_stdout();
    cleanup_signal_handlers();
    return exit_status;
}
unsafe extern "C" fn add_regexp(
    mut reglist: *mut regexp_list,
    mut pattern: *const ::core::ffi::c_char,
) {
    let mut patlen: idx_t = strlen(pattern) as idx_t;
    let mut m: *const ::core::ffi::c_char =
        re_compile_pattern(pattern, patlen as size_t, (*reglist).buf);
    if !m.is_null() {
        if 0 != 0 {
            error(
                C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                squote(0 as ::core::ffi::c_int, pattern),
                m,
            );
            if C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int =
                    C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    squote(0 as ::core::ffi::c_int, pattern),
                    m,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else {
        let mut regexps: *mut ::core::ffi::c_char = (*reglist).regexps;
        let mut len: idx_t = (*reglist).len;
        (*reglist).multiple_regexps = !regexps.is_null();
        let mut multiple_regexps: bool = (*reglist).multiple_regexps;
        (*reglist).len = len
            + (2 as ::core::ffi::c_int * multiple_regexps as ::core::ffi::c_int) as idx_t
            + patlen;
        let mut newlen: idx_t = (*reglist).len;
        let mut size: idx_t = (*reglist).size;
        if size <= newlen {
            (*reglist).regexps = xpalloc(
                regexps as *mut ::core::ffi::c_void,
                &raw mut (*reglist).size,
                newlen - size + 1 as idx_t,
                -1 as ptrdiff_t,
                1 as idx_t,
            ) as *mut ::core::ffi::c_char;
            regexps = (*reglist).regexps;
        }
        if multiple_regexps {
            let c2rust_fresh2 = len;
            len += 1;
            *regexps.offset(c2rust_fresh2 as isize) = '\\' as ::core::ffi::c_char;
            let c2rust_fresh3 = len;
            len += 1;
            *regexps.offset(c2rust_fresh3 as isize) = '|' as ::core::ffi::c_char;
        }
        memcpy(
            regexps.offset(len as isize) as *mut ::core::ffi::c_void,
            pattern as *const ::core::ffi::c_void,
            (patlen + 1 as idx_t) as size_t,
        );
    };
}
unsafe extern "C" fn summarize_regexp_list(mut reglist: *mut regexp_list) {
    if !(*reglist).regexps.is_null() {
        (*(*reglist).buf).fastmap =
            xmalloc(((1 as ::core::ffi::c_int) << CHAR_BIT) as size_t) as *mut ::core::ffi::c_char;
        if (*reglist).multiple_regexps {
            let mut m: *const ::core::ffi::c_char =
                re_compile_pattern((*reglist).regexps, (*reglist).len as size_t, (*reglist).buf);
            if !m.is_null() {
                if 0 != 0 {
                    error(
                        C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        squote(0 as ::core::ffi::c_int, (*reglist).regexps),
                        m,
                    );
                    if C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int =
                            C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            0 as ::core::ffi::c_int,
                            b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            squote(0 as ::core::ffi::c_int, (*reglist).regexps),
                            m,
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
}
unsafe extern "C" fn get_errno() -> ::core::ffi::c_int {
    let mut err: ::core::ffi::c_int = *__errno_location();
    if (0 as ::core::ffi::c_int) < err {
    } else {
        unreachable!();
    };
    return err;
}
unsafe extern "C" fn check_stdout() {
    if ferror_unlocked(stdout) != 0 {
        fatal(b"write failed\0".as_ptr() as *const ::core::ffi::c_char);
    } else if fclose(stdout) != 0 as ::core::ffi::c_int {
        pfatal_with_name(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"standard output\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
}
static mut option_help_msgid: [*const ::core::ffi::c_char; 69] = [
    b"    --normal                  output a normal diff (the default)\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-q, --brief                   report only when files differ\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-s, --report-identical-files  report when two files are the same\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-c, -C NUM, --context[=NUM]   output NUM (default 3) lines of copied context\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-u, -U NUM, --unified[=NUM]   output NUM (default 3) lines of unified context\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-e, --ed                      output an ed script\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-n, --rcs                     output an RCS format diff\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-y, --side-by-side            output in two columns\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-W, --width=NUM               output at most NUM (default 130) print columns\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --left-column             output only the left column of common lines\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --suppress-common-lines   do not output common lines\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-p, --show-c-function         show which C function each change is in\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-F, --show-function-line=RE   show the most recent line matching RE\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --label LABEL             use LABEL instead of file name and timestamp\n                                (can be repeated)\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-t, --expand-tabs             expand tabs to spaces in output\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-T, --initial-tab             make tabs line up by prepending a tab\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --tabsize=NUM             tab stops every NUM (default 8) print columns\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --suppress-blank-empty    suppress space or tab before empty output lines\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-l, --paginate                pass output through 'pr' to paginate it\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-r, --recursive                 recursively compare any subdirectories found\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --no-dereference            don't follow symbolic links\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-N, --new-file                  treat absent files as empty\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --unidirectional-new-file   treat absent first files as empty\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --ignore-file-name-case     ignore case when comparing file names\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --no-ignore-file-name-case  consider case when comparing file names\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-x, --exclude=PAT               exclude files that match PAT\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-X, --exclude-from=FILE         exclude files that match any pattern in FILE\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-S, --starting-file=FILE        start with FILE when comparing directories\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --from-file=FILE1           compare FILE1 to all operands;\n                                  FILE1 can be a directory\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --to-file=FILE2             compare all operands to FILE2;\n                                  FILE2 can be a directory\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-i, --ignore-case               ignore case differences in file contents\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-E, --ignore-tab-expansion      ignore changes due to tab expansion\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-Z, --ignore-trailing-space     ignore white space at line end\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-b, --ignore-space-change       ignore changes in the amount of white space\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-w, --ignore-all-space          ignore all white space\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-B, --ignore-blank-lines        ignore changes where lines are all blank\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"-I, --ignore-matching-lines=RE  ignore changes where all lines match RE\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-a, --text                      treat all files as text\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --strip-trailing-cr         strip trailing carriage return on input\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-D, --ifdef=NAME                output merged file with '#ifdef NAME' diffs\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --GTYPE-group-format=GFMT   format GTYPE input groups with GFMT\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --line-format=LFMT          format all input lines with LFMT\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --LTYPE-line-format=LFMT    format LTYPE input lines with LFMT\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  These format options provide fine-grained control over the output\n    of diff, generalizing -D/--ifdef.\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"  LTYPE is 'old', 'new', or 'unchanged'.  GTYPE is LTYPE or 'changed'.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  GFMT (only) may contain:\n    %<  lines from FILE1\n    %>  lines from FILE2\n    %=  lines common to FILE1 and FILE2\n    %[-][WIDTH][.[PREC]]{doxX}LETTER  printf-style spec for LETTER\n      LETTERs are as follows for new group, lower case for old group:\n        F  first line number\n        L  last line number\n        N  number of lines = L-F+1\n        E  F-1\n        M  L+1\n    %(A=B?T:E)  if A equals B then T else E\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"  LFMT (only) may contain:\n    %L  contents of line\n    %l  contents of line, excluding any trailing newline\n    %[-][WIDTH][.[PREC]]{doxX}n  printf-style spec for input line number\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"  Both GFMT and LFMT may contain:\n    %%  %\n    %c'C'  the single character C\n    %c'\\OOO'  the character with octal code OOO\n    C    the character C (other characters represent themselves)\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"-d, --minimal            try hard to find a smaller set of changes\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --horizon-lines=NUM  keep NUM lines of the common prefix and suffix\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    --speed-large-files  assume large files and many scattered small changes\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --color[=WHEN]       color output; WHEN is 'never', 'always', or 'auto';\n                           plain --color means --color='auto'\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"    --palette=PALETTE    the colors to use when --color is active; PALETTE is\n                           a colon-separated list of terminfo capabilities\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"    --help               display this help and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"-v, --version            output version information and exit\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"FILES are 'FILE1 FILE2' or 'DIR1 DIR2' or 'DIR FILE' or 'FILE DIR'.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"If --from-file or --to-file is given, there are no restrictions on FILE(s).\0"
        .as_ptr() as *const ::core::ffi::c_char,
    b"If a FILE is '-', read standard input.\0".as_ptr() as *const ::core::ffi::c_char,
    b"Exit status is 0 if inputs are the same, 1 if different, 2 if trouble.\0".as_ptr()
        as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn usage() {
    printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... FILES\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        squote(0 as ::core::ffi::c_int, program_name),
    );
    printf(
        b"%s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Compare FILES line by line.\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fputs_unlocked(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Mandatory arguments to long options are mandatory for short options too.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    let mut p: *const *const ::core::ffi::c_char =
        &raw const option_help_msgid as *const *const ::core::ffi::c_char;
    while !(*p).is_null() {
        if **p == 0 {
            putchar_unlocked('\n' as ::core::ffi::c_int);
        } else {
            let mut msg: *const ::core::ffi::c_char =
                dcgettext(::core::ptr::null::<::core::ffi::c_char>(), *p, LC_MESSAGES);
            let mut nl: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            loop {
                nl = strchr(msg, '\n' as ::core::ffi::c_int) as *const ::core::ffi::c_char;
                if nl.is_null() {
                    break;
                }
                fputs_unlocked(b"  \0".as_ptr() as *const ::core::ffi::c_char, stdout);
                if 0 != 0
                    && 0 != 0
                    && (1 as ::core::ffi::c_int as size_t).wrapping_mul(
                        nl.offset(1 as ::core::ffi::c_int as isize).offset_from(msg) as size_t,
                    ) <= 8 as size_t
                    && 1 as ::core::ffi::c_int as size_t != 0 as size_t
                {
                    ({
                        let mut __ptr: *const ::core::ffi::c_char = msg;
                        let mut __stream: *mut FILE = stdout;
                        let mut __cnt: size_t = 0;
                        __cnt = (1 as ::core::ffi::c_int as size_t).wrapping_mul(
                            nl.offset(1 as ::core::ffi::c_int as isize).offset_from(msg) as size_t,
                        );
                        while __cnt > 0 as size_t {
                            let c2rust_fresh4 = __ptr;
                            __ptr = __ptr.offset(1);
                            if putc_unlocked(*c2rust_fresh4 as ::core::ffi::c_int, __stream) == EOF
                            {
                                break;
                            }
                            __cnt = __cnt.wrapping_sub(1);
                        }
                    });
                } else {
                    if 0 != 0 && 1 as ::core::ffi::c_int as size_t == 0 as size_t
                        || 0 != 0
                            && nl.offset(1 as ::core::ffi::c_int as isize).offset_from(msg)
                                as size_t
                                == 0 as size_t
                    {
                    } else {
                        fwrite_unlocked(
                            msg as *const ::core::ffi::c_void,
                            1 as size_t,
                            nl.offset(1 as ::core::ffi::c_int as isize).offset_from(msg) as size_t,
                            stdout,
                        );
                    };
                };
                msg = nl.offset(1 as ::core::ffi::c_int as isize);
            }
            if *msg as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *msg as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            {
                fputs_unlocked(b"  \0".as_ptr() as *const ::core::ffi::c_char, stdout);
            }
            puts(msg);
        }
        p = p.offset(1);
    }
    emit_bug_reporting_address();
}
unsafe extern "C" fn specify_value(
    mut var: *mut *const ::core::ffi::c_char,
    mut value: *const ::core::ffi::c_char,
    mut option: *const ::core::ffi::c_char,
) {
    if !(*var).is_null() && !(strcmp(*var, value) == 0 as ::core::ffi::c_int) {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"conflicting %s option value %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                option,
                quote(value),
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
                        b"conflicting %s option value %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    option,
                    quote(value),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        try_help(
            ::core::ptr::null::<::core::ffi::c_char>(),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    *var = value;
}
unsafe extern "C" fn specify_style(mut style: output_style) {
    if output_style_0.0 != style.0 {
        if output_style_0.0 != output_style::OUTPUT_UNSPECIFIED.0 {
            try_help(
                b"conflicting output style options\0".as_ptr() as *const ::core::ffi::c_char,
                ::core::ptr::null::<::core::ffi::c_char>(),
            );
        }
        output_style_0 = style;
    }
}
unsafe extern "C" fn specify_colors_style(mut value: *const ::core::ffi::c_char) {
    if value.is_null()
        || strcmp(value, b"auto\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        colors_style_0 = colors_style::AUTO;
    } else if strcmp(value, b"always\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        colors_style_0 = colors_style::ALWAYS;
    } else if strcmp(value, b"never\0".as_ptr() as *const ::core::ffi::c_char)
        == 0 as ::core::ffi::c_int
    {
        colors_style_0 = colors_style::NEVER;
    } else {
        try_help(
            b"invalid color %s\0".as_ptr() as *const ::core::ffi::c_char,
            quote(value),
        );
    };
}
unsafe extern "C" fn dir_p(mut pcmp: *const comparison, mut f: ::core::ffi::c_int) -> bool {
    return ((*pcmp).file[f as usize].stat.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
        as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn compare_prepped_files(
    mut parent: *const comparison,
    mut cmp: *mut comparison,
    mut open_flags: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*cmp).file[0usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0
        && (*cmp).file[1usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0
    {
        return EXIT_SUCCESS;
    }
    let mut same_files: bool = (*cmp).file[0usize].desc != C2Rust_Unnamed_0::NONEXISTENT.0
        && (*cmp).file[1usize].desc != C2Rust_Unnamed_0::NONEXISTENT.0
        && (*cmp).file[0usize].filetype == (*cmp).file[1usize].filetype
        && same_file(
            &raw mut (*(&raw mut (*cmp).file as *mut file_data).offset(0isize)).stat,
            &raw mut (*(&raw mut (*cmp).file as *mut file_data).offset(1isize)).stat,
        ) as ::core::ffi::c_int
            != 0;
    if same_files as ::core::ffi::c_int & no_diff_means_no_output as ::core::ffi::c_int != 0 {
        return EXIT_SUCCESS;
    }
    let mut toplevel: bool = parent == &raw mut noparent as *const comparison;
    if dir_p(cmp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int
        & dir_p(cmp, 1 as ::core::ffi::c_int) as ::core::ffi::c_int
        != 0
        || recursive as ::core::ffi::c_int != 0
            && (new_file as ::core::ffi::c_int
                & dir_p(cmp, 1 as ::core::ffi::c_int) as ::core::ffi::c_int
                != 0
                && (*cmp).file[0usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0
                || (new_file as ::core::ffi::c_int | unidirectional_new_file as ::core::ffi::c_int)
                    & dir_p(cmp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    != 0
                    && (*cmp).file[1usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0)
    {
        if output_style_0.0 == output_style::OUTPUT_IFDEF.0 {
            fatal(b"-D option not supported with directories\0".as_ptr()
                as *const ::core::ffi::c_char);
        }
        if recursive as ::core::ffi::c_int | toplevel as ::core::ffi::c_int != 0 {
            return diff_dirs(cmp);
        } else {
            message(
                b"Common subdirectories: %s and %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                squote(0 as ::core::ffi::c_int, (*cmp).file[0usize].name),
                squote(1 as ::core::ffi::c_int, (*cmp).file[1usize].name),
            );
            return EXIT_SUCCESS;
        }
    }
    if (*cmp).file[0usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0
        && new_file as ::core::ffi::c_int | unidirectional_new_file as ::core::ffi::c_int == 0
        || (*cmp).file[1usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0 && !new_file
    {
        let mut existing: bool = (*cmp).file[0usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0;
        let mut dname: *const ::core::ffi::c_char = (*parent).file[existing as usize].name;
        let mut bname: *const ::core::ffi::c_char =
            last_component((*cmp).file[existing as usize].name);
        message(
            b"Only in %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            squote(0 as ::core::ffi::c_int, dname),
            squote(1 as ::core::ffi::c_int, bname),
        );
        return EXIT_FAILURE;
    }
    let mut mode0: mode_t = (*cmp).file[0usize].stat.st_mode;
    let mut mode1: mode_t = (*cmp).file[1usize].stat.st_mode;
    if if toplevel as ::core::ffi::c_int != 0 {
        (!(mode0 & __S_IFMT as mode_t == 0o120000 as mode_t) as ::core::ffi::c_int
            != !(mode1 & __S_IFMT as mode_t == 0o120000 as mode_t) as ::core::ffi::c_int)
            as ::core::ffi::c_int
    } else if mode0 & __S_IFMT as mode_t == 0o100000 as mode_t {
        !(mode1 & __S_IFMT as mode_t == 0o100000 as mode_t) as ::core::ffi::c_int
    } else if mode0 & __S_IFMT as mode_t == 0o120000 as mode_t {
        !(mode1 & __S_IFMT as mode_t == 0o120000 as mode_t) as ::core::ffi::c_int
    } else if mode0 & __S_IFMT as mode_t == 0o20000 as mode_t {
        !(mode1 & __S_IFMT as mode_t == 0o20000 as mode_t) as ::core::ffi::c_int
    } else if mode0 & __S_IFMT as mode_t == 0o60000 as mode_t {
        !(mode1 & __S_IFMT as mode_t == 0o60000 as mode_t) as ::core::ffi::c_int
    } else {
        r#true
    } != 0
    {
        message(
            b"File %s is a %s while file %s is a %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if !file_label[0usize].is_null() {
                file_label[0usize]
            } else {
                squote(0 as ::core::ffi::c_int, (*cmp).file[0usize].name)
            },
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                (*cmp).file[0usize].filetype,
                LC_MESSAGES,
            ),
            if !file_label[1usize].is_null() {
                file_label[1usize]
            } else {
                squote(1 as ::core::ffi::c_int, (*cmp).file[1usize].name)
            },
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                (*cmp).file[1usize].filetype,
                LC_MESSAGES,
            ),
        );
        return EXIT_FAILURE;
    }
    if mode0 & __S_IFMT as mode_t == 0o120000 as mode_t {
        if no_dereference_symlinks as ::core::ffi::c_int != 0 {
        } else {
            unreachable!();
        };
        let mut status: ::core::ffi::c_int = EXIT_SUCCESS;
        let mut link_value: [*mut ::core::ffi::c_char; 2] =
            [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
        link_value[1usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut linkbuf: [[::core::ffi::c_char; 128]; 2] = [[0; 128]; 2];
        let mut f: bool = r#false != 0;
        loop {
            let mut linkfd: ::core::ffi::c_int = (*cmp).file[f as usize].desc;
            let mut dirfd: ::core::ffi::c_int = (*parent).file[f as usize].desc;
            let mut name: *const ::core::ffi::c_char = (*cmp).file[f as usize].name;
            let mut dirarg: ::core::ffi::c_int = if linkfd < 0 as ::core::ffi::c_int {
                dirfd
            } else {
                linkfd
            };
            let mut namearg: *const ::core::ffi::c_char = if linkfd < 0 as ::core::ffi::c_int {
                if dirfd < 0 as ::core::ffi::c_int {
                    name
                } else {
                    last_component(name) as *const ::core::ffi::c_char
                }
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            };
            link_value[f as usize] = careadlinkat(
                dirarg,
                namearg,
                &raw mut *(&raw mut linkbuf as *mut [::core::ffi::c_char; 128]).offset(f as isize)
                    as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 128]>(),
                ::core::ptr::null::<allocator>(),
                Some(
                    readlinkat
                        as unsafe extern "C" fn(
                            ::core::ffi::c_int,
                            *const ::core::ffi::c_char,
                            *mut ::core::ffi::c_char,
                            size_t,
                        ) -> ssize_t,
                ),
            );
            if link_value[f as usize].is_null() {
                perror_with_name((*cmp).file[f as usize].name);
                status = C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                break;
            } else if f {
                status = if strcmp(link_value[0usize], link_value[f as usize])
                    == 0 as ::core::ffi::c_int
                {
                    EXIT_SUCCESS
                } else {
                    EXIT_FAILURE
                };
                break;
            } else {
                f = r#true != 0;
            }
        }
        if status == EXIT_FAILURE {
            message(
                b"Symbolic links %s -> %s and %s -> %s differ\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                quote_n(0 as ::core::ffi::c_int, (*cmp).file[0usize].name),
                quote_n(1 as ::core::ffi::c_int, link_value[0usize]),
                quote_n(2 as ::core::ffi::c_int, (*cmp).file[1usize].name),
                quote_n(3 as ::core::ffi::c_int, link_value[1usize]),
            );
        }
        let mut f_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while f_0 < 2 as ::core::ffi::c_int {
            if link_value[f_0 as usize]
                != &raw mut *(&raw mut linkbuf as *mut [::core::ffi::c_char; 128])
                    .offset(f_0 as isize) as *mut ::core::ffi::c_char
            {
                free(link_value[f_0 as usize] as *mut ::core::ffi::c_void);
            }
            f_0 += 1;
        }
        return status;
    }
    if !toplevel && !(mode0 & __S_IFMT as mode_t == 0o100000 as mode_t) {
        if (*cmp).file[0usize].stat.st_rdev == (*cmp).file[1usize].stat.st_rdev {
            return EXIT_SUCCESS;
        }
        let mut num: [intmax_t; 4] = [
            ((*cmp).file[0usize].stat.st_rdev >> 8 as ::core::ffi::c_int & 0xff as __dev_t)
                as intmax_t,
            ((*cmp).file[0usize].stat.st_rdev & 0xff as __dev_t) as intmax_t,
            ((*cmp).file[1usize].stat.st_rdev >> 8 as ::core::ffi::c_int & 0xff as __dev_t)
                as intmax_t,
            ((*cmp).file[1usize].stat.st_rdev & 0xff as __dev_t) as intmax_t,
        ];
        let mut numbuf: [[::core::ffi::c_char; 21]; 4] = [[0; 21]; 4];
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < C2Rust_Unnamed_2::n_num.0 as ::core::ffi::c_int {
            sprintf(
                &raw mut *(&raw mut numbuf as *mut [::core::ffi::c_char; 21]).offset(i as isize)
                    as *mut ::core::ffi::c_char,
                b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                num[i as usize],
            );
            i += 1;
        }
        message(
            if mode0 & __S_IFMT as mode_t == 0o20000 as mode_t {
                b"Character special files %s (%s, %s) and %s (%s, %s) differ\n\0".as_ptr()
                    as *const ::core::ffi::c_char
            } else {
                b"Block special files %s (%s, %s) and %s (%s, %s) differ\n\0".as_ptr()
                    as *const ::core::ffi::c_char
            },
            quote_n(0 as ::core::ffi::c_int, (*cmp).file[0usize].name),
            &raw mut *(&raw mut numbuf as *mut [::core::ffi::c_char; 21]).offset(0isize)
                as *mut ::core::ffi::c_char,
            &raw mut *(&raw mut numbuf as *mut [::core::ffi::c_char; 21]).offset(1isize)
                as *mut ::core::ffi::c_char,
            quote_n(2 as ::core::ffi::c_int, (*cmp).file[1usize].name),
            &raw mut *(&raw mut numbuf as *mut [::core::ffi::c_char; 21]).offset(2isize)
                as *mut ::core::ffi::c_char,
            &raw mut *(&raw mut numbuf as *mut [::core::ffi::c_char; 21]).offset(3isize)
                as *mut ::core::ffi::c_char,
        );
        return EXIT_FAILURE;
    }
    if files_can_be_treated_as_binary as ::core::ffi::c_int != 0
        && mode0 & __S_IFMT as mode_t == 0o100000 as mode_t
        && mode1 & __S_IFMT as mode_t == 0o100000 as mode_t
        && (*cmp).file[0usize].stat.st_size != (*cmp).file[1usize].stat.st_size
        && 0 as __off_t <= (*cmp).file[0usize].stat.st_size
        && 0 as __off_t <= (*cmp).file[1usize].stat.st_size
    {
        message(
            b"Files %s and %s differ\n\0".as_ptr() as *const ::core::ffi::c_char,
            if !file_label[0usize].is_null() {
                file_label[0usize]
            } else {
                squote(0 as ::core::ffi::c_int, (*cmp).file[0usize].name)
            },
            if !file_label[1usize].is_null() {
                file_label[1usize]
            } else {
                squote(1 as ::core::ffi::c_int, (*cmp).file[1usize].name)
            },
        );
        return EXIT_FAILURE;
    }
    let mut status_0: ::core::ffi::c_int = EXIT_SUCCESS;
    let mut f_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_1 < 2 as ::core::ffi::c_int {
        if (*cmp).file[f_1 as usize].desc == C2Rust_Unnamed_1::UNOPENED.0 {
            if f_1 != 0 && same_files as ::core::ffi::c_int != 0 {
                (*cmp).file[f_1 as usize].desc = (*cmp).file[0usize].desc;
            } else {
                let mut dirfd_0: ::core::ffi::c_int = (*parent).file[f_1 as usize].desc;
                let mut name_0: *const ::core::ffi::c_char = (*cmp).file[f_1 as usize].name;
                let mut nm: *const ::core::ffi::c_char = if dirfd_0 < 0 as ::core::ffi::c_int {
                    name_0
                } else {
                    last_component(name_0) as *const ::core::ffi::c_char
                };
                (*cmp).file[f_1 as usize].desc = openat(dirfd_0, nm, open_flags);
                if (*cmp).file[f_1 as usize].desc < 0 as ::core::ffi::c_int {
                    perror_with_name(name_0);
                    status_0 = C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                }
            }
        } else if (*cmp).file[f_1 as usize].desc == C2Rust_Unnamed::OPEN_FAILED.0 {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    (*cmp).file[f_1 as usize].openerr,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    squote(0 as ::core::ffi::c_int, (*cmp).file[f_1 as usize].name),
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
                        (*cmp).file[f_1 as usize].openerr,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        squote(0 as ::core::ffi::c_int, (*cmp).file[f_1 as usize].name),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            status_0 = C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
        }
        f_1 += 1;
    }
    if status_0 != EXIT_SUCCESS {
        return status_0;
    }
    return diff_2_files(cmp);
}
#[export_name = "rboxc_diffutils_compare_files"]
pub unsafe extern "C" fn compare_files(
    mut parent: *const comparison,
    mut detype_0: *const detype,
    mut name0: *const ::core::ffi::c_char,
    mut name1: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if !(!name0.is_null() && !name1.is_null()
        || unidirectional_new_file as ::core::ffi::c_int != 0 && !name1.is_null()
        || new_file as ::core::ffi::c_int != 0)
    {
        let mut name: *const ::core::ffi::c_char = if !name0.is_null() { name0 } else { name1 };
        let mut dir: *const ::core::ffi::c_char =
            (*parent).file[name0.is_null() as ::core::ffi::c_int as usize].name;
        message(
            b"Only in %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            squote(0 as ::core::ffi::c_int, dir),
            squote(1 as ::core::ffi::c_int, name),
        );
        return EXIT_FAILURE;
    }
    let mut cmp: comparison = comparison {
        file: [
            file_data {
                desc: if !name0.is_null() {
                    C2Rust_Unnamed_1::UNOPENED.0
                } else {
                    C2Rust_Unnamed_0::NONEXISTENT.0
                },
                openerr: 0,
                err: 0,
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                filetype: ::core::ptr::null::<::core::ffi::c_char>(),
                stat: stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_nlink: 0,
                    st_mode: 0,
                    st_uid: 0,
                    st_gid: 0,
                    __pad0: 0,
                    st_rdev: 0,
                    st_size: (if !name0.is_null() {
                        -1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as __off_t,
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
                },
                dirstream: ::core::ptr::null_mut::<DIR>(),
                buffer: ::core::ptr::null_mut::<word>(),
                bufsize: 0,
                buffered: 0,
                linbuf: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                linbuf_base: 0,
                buffered_lines: 0,
                valid_lines: 0,
                alloc_lines: 0,
                prefix_end: ::core::ptr::null::<::core::ffi::c_char>(),
                prefix_lines: 0,
                suffix_begin: ::core::ptr::null::<::core::ffi::c_char>(),
                equivs: ::core::ptr::null_mut::<lin>(),
                undiscarded: ::core::ptr::null_mut::<lin>(),
                realindexes: ::core::ptr::null_mut::<lin>(),
                nondiscarded_lines: 0,
                changed: ::core::ptr::null_mut::<bool>(),
                missing_newline: false,
                eof: false,
                equiv_max: 0,
            },
            file_data {
                desc: if !name1.is_null() {
                    C2Rust_Unnamed_1::UNOPENED.0
                } else {
                    C2Rust_Unnamed_0::NONEXISTENT.0
                },
                openerr: 0,
                err: 0,
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                filetype: ::core::ptr::null::<::core::ffi::c_char>(),
                stat: stat {
                    st_dev: 0,
                    st_ino: 0,
                    st_nlink: 0,
                    st_mode: 0,
                    st_uid: 0,
                    st_gid: 0,
                    __pad0: 0,
                    st_rdev: 0,
                    st_size: (if !name1.is_null() {
                        -1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as __off_t,
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
                },
                dirstream: ::core::ptr::null_mut::<DIR>(),
                buffer: ::core::ptr::null_mut::<word>(),
                bufsize: 0,
                buffered: 0,
                linbuf: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
                linbuf_base: 0,
                buffered_lines: 0,
                valid_lines: 0,
                alloc_lines: 0,
                prefix_end: ::core::ptr::null::<::core::ffi::c_char>(),
                prefix_lines: 0,
                suffix_begin: ::core::ptr::null::<::core::ffi::c_char>(),
                equivs: ::core::ptr::null_mut::<lin>(),
                undiscarded: ::core::ptr::null_mut::<lin>(),
                realindexes: ::core::ptr::null_mut::<lin>(),
                nondiscarded_lines: 0,
                changed: ::core::ptr::null_mut::<bool>(),
                missing_newline: false,
                eof: false,
                equiv_max: 0,
            },
        ],
        parent: parent,
    };
    if name0.is_null() {
        name0 = name1;
    }
    if name1.is_null() {
        name1 = name0;
    }
    let mut free0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut free1: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut toplevel: bool = parent == &raw mut noparent as *const comparison;
    if toplevel {
        free0 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free1 = ::core::ptr::null_mut::<::core::ffi::c_char>();
        cmp.file[0usize].name = name0;
        cmp.file[1usize].name = name1;
    } else {
        free0 = file_name_concat(
            (*parent).file[0usize].name,
            name0,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        cmp.file[0usize].name = free0;
        free1 = file_name_concat(
            (*parent).file[1usize].name,
            name1,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        cmp.file[1usize].name = free1;
    }
    let mut oflags: ::core::ffi::c_int = if C2Rust_Unnamed_4::binary.0 as ::core::ffi::c_int != 0 {
        O_BINARY
    } else {
        0 as ::core::ffi::c_int
    } | O_CLOEXEC
        | if no_dereference_symlinks as ::core::ffi::c_int != 0 {
            O_NOFOLLOW
        } else {
            0 as ::core::ffi::c_int
        };
    let mut f: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f < 2 as ::core::ffi::c_int {
        let mut fd: ::core::ffi::c_int = cmp.file[f as usize].desc;
        if fd == C2Rust_Unnamed_1::UNOPENED.0 {
            if f != 0
                && strcmp(cmp.file[f as usize].name, cmp.file[0usize].name)
                    == 0 as ::core::ffi::c_int
            {
                cmp.file[f as usize].desc = cmp.file[0usize].desc;
                cmp.file[f as usize].filetype = cmp.file[0usize].filetype;
                cmp.file[f as usize].stat = cmp.file[0usize].stat;
            } else {
                let mut parentdesc: ::core::ffi::c_int = (*parent).file[f as usize].desc;
                let mut name_0: *const ::core::ffi::c_char = cmp.file[f as usize].name;
                let mut nm: *const ::core::ffi::c_char = if parentdesc < 0 as ::core::ffi::c_int {
                    name_0
                } else {
                    last_component(name_0) as *const ::core::ffi::c_char
                };
                let mut err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                if strcmp(
                    cmp.file[f as usize].name,
                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                {
                    fd = STDIN_FILENO;
                    if C2Rust_Unnamed_4::binary.0 as ::core::ffi::c_int != 0 && isatty(fd) == 0 {
                        set_binary_mode(fd, O_BINARY);
                    }
                } else if toplevel as ::core::ffi::c_int != 0
                    || (*detype_0.offset(f as isize)).0 == detype::DE_REG.0
                    || (*detype_0.offset(f as isize)).0 == detype::DE_DIR.0
                    || C2Rust_Unnamed_5::O_PATH_DEFINED.0 as ::core::ffi::c_int != 0
                        && (*detype_0.offset(f as isize)).0 == detype::DE_LNK.0
                        && no_dereference_symlinks as ::core::ffi::c_int != 0
                {
                    let mut accmode: ::core::ffi::c_int =
                        if C2Rust_Unnamed_5::O_PATH_DEFINED.0 as ::core::ffi::c_int != 0
                            && !toplevel
                            && (*detype_0.offset(f as isize)).0 == detype::DE_LNK.0
                            && no_dereference_symlinks as ::core::ffi::c_int != 0
                        {
                            C2Rust_Unnamed_6::O_PATHSEARCH.0 as ::core::ffi::c_int
                        } else {
                            O_RDONLY
                        };
                    fd = openat(parentdesc, nm, accmode | oflags);
                    if fd < 0 as ::core::ffi::c_int {
                        err = get_errno();
                        if err == EACCES
                            && toplevel as ::core::ffi::c_int != 0
                            && !ignore_file_name_case
                            && !no_directory
                            && (f == 0 as ::core::ffi::c_int
                                || !dir_p(&raw mut cmp, 0 as ::core::ffi::c_int))
                        {
                            fd = openat(
                                parentdesc,
                                nm,
                                C2Rust_Unnamed_6::O_PATHSEARCH.0 as ::core::ffi::c_int
                                    | O_DIRECTORY
                                    | oflags,
                            );
                            if 0 as ::core::ffi::c_int <= fd {
                                err = 0 as ::core::ffi::c_int;
                            }
                        }
                        if err == C2Rust_Unnamed_8::NOFOLLOW_SYMLINK_ERRNO.0 as ::core::ffi::c_int
                            && (C2Rust_Unnamed_8::NOFOLLOW_SYMLINK_ERRNO.0 as ::core::ffi::c_int
                                != ELOOP
                                || no_dereference_symlinks as ::core::ffi::c_int != 0
                                    && ((*detype_0.offset(f as isize)).0 == detype::DE_UNKNOWN.0
                                        || (*detype_0.offset(f as isize)).0 == detype::DE_LNK.0
                                            && accmode == O_RDONLY))
                        {
                            fd = C2Rust_Unnamed_1::UNOPENED.0;
                            err = 0 as ::core::ffi::c_int;
                        }
                        cmp.file[f as usize].openerr = err;
                    }
                }
                if !(cmp.file[(1 as ::core::ffi::c_int - f) as usize].err != 0
                    || err == ENOENT
                    || err == ENOTDIR
                    || err == ELOOP
                    || err == EOVERFLOW
                    || err == ENAMETOOLONG)
                {
                    if if fd < 0 as ::core::ffi::c_int {
                        fstatat(
                            parentdesc,
                            nm,
                            &raw mut (*(&raw mut cmp.file as *mut file_data).offset(f as isize))
                                .stat,
                            if no_dereference_symlinks as ::core::ffi::c_int != 0 {
                                AT_SYMLINK_NOFOLLOW
                            } else {
                                0 as ::core::ffi::c_int
                            },
                        )
                    } else {
                        fstat(
                            fd,
                            &raw mut (*(&raw mut cmp.file as *mut file_data).offset(f as isize))
                                .stat,
                        )
                    } < 0 as ::core::ffi::c_int
                    {
                        err = get_errno();
                    } else {
                        err = 0 as ::core::ffi::c_int;
                        let mut size: off_t = stat_size(
                            &raw mut (*(&raw mut cmp.file as *mut file_data).offset(f as isize))
                                .stat,
                        );
                        if 0 as off_t <= size && fd == STDIN_FILENO {
                            let mut pos: off_t = lseek(fd, 0 as __off_t, SEEK_CUR);
                            if 0 as off_t <= pos {
                                size = if 0 as off_t > size - pos {
                                    0 as off_t
                                } else {
                                    size - pos
                                };
                            }
                        }
                        cmp.file[f as usize].stat.st_size = size as __off_t;
                        cmp.file[f as usize].filetype = c_file_type(
                            &raw mut (*(&raw mut cmp.file as *mut file_data).offset(f as isize))
                                .stat,
                        );
                    }
                }
                cmp.file[f as usize].desc = fd;
                cmp.file[f as usize].err = err;
            }
        }
        f += 1;
    }
    if toplevel {
        if !no_directory
            && toplevel as ::core::ffi::c_int != 0
            && cmp.file[0usize].err == 0
            && cmp.file[1usize].err == 0
            && dir_p(&raw mut cmp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                != dir_p(&raw mut cmp, 1 as ::core::ffi::c_int) as ::core::ffi::c_int
        {
            let mut fnm_arg: ::core::ffi::c_int =
                dir_p(&raw mut cmp, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
            let mut dir_arg: ::core::ffi::c_int = 1 as ::core::ffi::c_int - fnm_arg;
            if cmp.file[fnm_arg as usize].desc == STDIN_FILENO {
                fatal(b"cannot compare '-' to a directory\0".as_ptr() as *const ::core::ffi::c_char);
            }
            let mut fnm: *const ::core::ffi::c_char = cmp.file[fnm_arg as usize].name;
            let mut dir_detype: detype = detype::DE_UNKNOWN;
            free0 = find_dir_file_pathname(
                (&raw mut cmp.file as *mut file_data).offset(dir_arg as isize),
                last_component(fnm),
                &raw mut dir_detype,
            );
            cmp.file[dir_arg as usize].name = free0;
            let mut filename: *const ::core::ffi::c_char = cmp.file[dir_arg as usize].name;
            let mut dirfd: ::core::ffi::c_int = cmp.file[dir_arg as usize].desc;
            if dirfd < 0 as ::core::ffi::c_int {
                dirfd = AT_FDCWD;
            }
            let mut atname: *const ::core::ffi::c_char = if dirfd < 0 as ::core::ffi::c_int {
                filename
            } else {
                last_component(filename) as *const ::core::ffi::c_char
            };
            cmp.file[dir_arg as usize].desc = C2Rust_Unnamed_1::UNOPENED.0;
            noparent.file[dir_arg as usize].desc = dirfd;
            cmp.file[dir_arg as usize].desc = if dir_detype.0 == detype::DE_LNK.0
                && no_dereference_symlinks as ::core::ffi::c_int != 0
            {
                *__errno_location() = ELOOP;
                -1 as ::core::ffi::c_int
            } else {
                openat(dirfd, atname, O_RDONLY | oflags)
            };
            if C2Rust_Unnamed_5::O_PATH_DEFINED.0 as ::core::ffi::c_int != 0
                && cmp.file[dir_arg as usize].desc < 0 as ::core::ffi::c_int
                && (dir_detype.0 == detype::DE_LNK.0 || dir_detype.0 == detype::DE_UNKNOWN.0)
                && no_dereference_symlinks as ::core::ffi::c_int != 0
                && *__errno_location()
                    == C2Rust_Unnamed_8::NOFOLLOW_SYMLINK_ERRNO.0 as ::core::ffi::c_int
            {
                cmp.file[dir_arg as usize].desc = openat(
                    dirfd,
                    atname,
                    C2Rust_Unnamed_6::O_PATHSEARCH.0 as ::core::ffi::c_int | oflags,
                );
            }
            if if cmp.file[dir_arg as usize].desc < 0 as ::core::ffi::c_int {
                (C2Rust_Unnamed_5::O_PATH_DEFINED.0 as ::core::ffi::c_int != 0
                    || !no_dereference_symlinks
                    || *__errno_location()
                        != C2Rust_Unnamed_8::NOFOLLOW_SYMLINK_ERRNO.0 as ::core::ffi::c_int
                    || fstatat(
                        dirfd,
                        atname,
                        &raw mut (*(&raw mut cmp.file as *mut file_data).offset(dir_arg as isize))
                            .stat,
                        AT_SYMLINK_NOFOLLOW,
                    ) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            } else {
                (fstat(
                    cmp.file[dir_arg as usize].desc,
                    &raw mut (*(&raw mut cmp.file as *mut file_data).offset(dir_arg as isize)).stat,
                ) < 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            } != 0
            {
                cmp.file[dir_arg as usize].err = get_errno();
            } else {
                cmp.file[dir_arg as usize].stat.st_size = stat_size(
                    &raw mut (*(&raw mut cmp.file as *mut file_data).offset(dir_arg as isize)).stat,
                ) as __off_t;
                cmp.file[dir_arg as usize].filetype = c_file_type(
                    &raw mut (*(&raw mut cmp.file as *mut file_data).offset(dir_arg as isize)).stat,
                );
            }
        }
        let mut f_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while f_0 < 2 as ::core::ffi::c_int {
            if (new_file as ::core::ffi::c_int != 0
                || f_0 == 0 as ::core::ffi::c_int
                    && unidirectional_new_file as ::core::ffi::c_int != 0)
                && (cmp.file[f_0 as usize].err == ENOENT || cmp.file[f_0 as usize].err == ENOTDIR)
                && !(cmp.file[(1 as ::core::ffi::c_int - f_0) as usize].err == ENOENT
                    || cmp.file[(1 as ::core::ffi::c_int - f_0) as usize].err == ENOTDIR)
            {
                cmp.file[f_0 as usize].desc = C2Rust_Unnamed_0::NONEXISTENT.0;
                cmp.file[f_0 as usize].err = 0 as ::core::ffi::c_int;
            }
            f_0 += 1;
        }
    }
    let mut f_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_1 < 2 as ::core::ffi::c_int {
        if cmp.file[f_1 as usize].desc == C2Rust_Unnamed_0::NONEXISTENT.0 {
            cmp.file[f_1 as usize].filetype =
                cmp.file[(1 as ::core::ffi::c_int - f_1) as usize].filetype;
            cmp.file[f_1 as usize].stat.st_mode = cmp.file
                [(1 as ::core::ffi::c_int - f_1) as usize]
                .stat
                .st_mode;
        }
        f_1 += 1;
    }
    let mut status: ::core::ffi::c_int = EXIT_SUCCESS;
    let mut f_2: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_2 < 2 as ::core::ffi::c_int {
        if cmp.file[f_2 as usize].err != 0 {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    cmp.file[f_2 as usize].err,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    squote(0 as ::core::ffi::c_int, cmp.file[f_2 as usize].name),
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
                        cmp.file[f_2 as usize].err,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        squote(0 as ::core::ffi::c_int, cmp.file[f_2 as usize].name),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            status = C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
        }
        f_2 += 1;
    }
    if status == EXIT_SUCCESS {
        status = compare_prepped_files(parent, &raw mut cmp, O_RDONLY | oflags);
    }
    let mut f_3: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while f_3 < 2 as ::core::ffi::c_int {
        if (f_3 == 0 as ::core::ffi::c_int || cmp.file[f_3 as usize].desc != cmp.file[0usize].desc)
            && if !cmp.file[f_3 as usize].dirstream.is_null() {
                (closedir(cmp.file[f_3 as usize].dirstream) < 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } else {
                (0 as ::core::ffi::c_int <= cmp.file[f_3 as usize].desc
                    && close(cmp.file[f_3 as usize].desc) < 0 as ::core::ffi::c_int)
                    as ::core::ffi::c_int
            } != 0
        {
            perror_with_name(cmp.file[f_3 as usize].name);
            status = C2Rust_Unnamed_3::EXIT_TROUBLE.0 as ::core::ffi::c_int;
        }
        f_3 += 1;
    }
    if status == EXIT_SUCCESS {
        if report_identical_files as ::core::ffi::c_int != 0
            && !dir_p(&raw mut cmp, 0 as ::core::ffi::c_int)
        {
            message(
                b"Files %s and %s are identical\n\0".as_ptr() as *const ::core::ffi::c_char,
                if !file_label[0usize].is_null() {
                    file_label[0usize]
                } else {
                    squote(0 as ::core::ffi::c_int, cmp.file[0usize].name)
                },
                if !file_label[1usize].is_null() {
                    file_label[1usize]
                } else {
                    squote(1 as ::core::ffi::c_int, cmp.file[1usize].name)
                },
            );
        }
    } else if fflush_unlocked(stdout) != 0 as ::core::ffi::c_int {
        pfatal_with_name(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"standard output\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
    free(free0 as *mut ::core::ffi::c_void);
    free(free1 as *mut ::core::ffi::c_void);
    return status;
}
#[export_name = "rboxc_diffutils_outfile"]
pub static mut outfile: *mut FILE = ::core::ptr::null_mut::<FILE>();
#[export_name = "rboxc_diffutils_brief"]
pub static mut brief: bool = false;
#[export_name = "rboxc_diffutils_expand_tabs"]
pub static mut expand_tabs: bool = false;
#[export_name = "rboxc_diffutils_files_can_be_treated_as_binary"]
pub static mut files_can_be_treated_as_binary: bool = false;
#[export_name = "rboxc_diffutils_ignore_blank_lines"]
pub static mut ignore_blank_lines: bool = false;
#[export_name = "rboxc_diffutils_ignore_case"]
pub static mut ignore_case: bool = false;
#[export_name = "rboxc_diffutils_ignore_file_name_case"]
pub static mut ignore_file_name_case: bool = false;
#[export_name = "rboxc_diffutils_initial_tab"]
pub static mut initial_tab: bool = false;
#[export_name = "rboxc_diffutils_left_column"]
pub static mut left_column: bool = false;
#[export_name = "rboxc_diffutils_minimal"]
pub static mut minimal: bool = false;
#[export_name = "rboxc_diffutils_no_dereference_symlinks"]
pub static mut no_dereference_symlinks: bool = false;
#[export_name = "rboxc_diffutils_no_diff_means_no_output"]
pub static mut no_diff_means_no_output: bool = false;
#[export_name = "rboxc_diffutils_paginate"]
pub static mut paginate: bool = false;
#[export_name = "rboxc_diffutils_presume_output_tty"]
pub static mut presume_output_tty: bool = false;
#[export_name = "rboxc_diffutils_sdiff_merge_assist"]
pub static mut sdiff_merge_assist: bool = false;
#[export_name = "rboxc_diffutils_speed_large_files"]
pub static mut speed_large_files: bool = false;
#[export_name = "rboxc_diffutils_strip_trailing_cr"]
pub static mut strip_trailing_cr: bool = false;
#[export_name = "rboxc_diffutils_suppress_blank_empty"]
pub static mut suppress_blank_empty: bool = false;
#[export_name = "rboxc_diffutils_suppress_common_lines"]
pub static mut suppress_common_lines: bool = false;
#[export_name = "rboxc_diffutils_text"]
pub static mut text: bool = false;
#[export_name = "rboxc_diffutils_file_label"]
pub static mut file_label: [*mut ::core::ffi::c_char; 2] =
    [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
#[export_name = "rboxc_diffutils_switch_string"]
pub static mut switch_string: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_diffutils_group_format"]
pub static mut group_format: [*const ::core::ffi::c_char; 4] =
    [::core::ptr::null::<::core::ffi::c_char>(); 4];
#[export_name = "rboxc_diffutils_line_format"]
pub static mut line_format: [*const ::core::ffi::c_char; 3] =
    [::core::ptr::null::<::core::ffi::c_char>(); 3];
#[export_name = "rboxc_diffutils_starting_file"]
pub static mut starting_file: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_diffutils_time_format"]
pub static mut time_format: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_diffutils_ignore_white_space"]
pub static mut ignore_white_space: DIFF_white_space = DIFF_white_space::IGNORE_NO_WHITE_SPACE;
#[export_name = "rboxc_diffutils_colors_style"]
pub static mut colors_style_0: colors_style = colors_style::NEVER;
#[export_name = "rboxc_diffutils_output_style"]
pub static mut output_style_0: output_style = output_style::OUTPUT_UNSPECIFIED;
#[export_name = "rboxc_diffutils_sdiff_column2_offset"]
pub static mut sdiff_column2_offset: intmax_t = 0;
#[export_name = "rboxc_diffutils_sdiff_half_width"]
pub static mut sdiff_half_width: intmax_t = 0;
#[export_name = "rboxc_diffutils_tabsize"]
pub static mut tabsize: intmax_t = 0;
#[export_name = "rboxc_diffutils_context"]
pub static mut context: lin = 0;
#[export_name = "rboxc_diffutils_horizon_lines"]
pub static mut horizon_lines: lin = 0;
#[export_name = "rboxc_diffutils_curr"]
pub static mut curr: comparison = comparison {
    file: [file_data {
        desc: 0,
        openerr: 0,
        err: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        filetype: ::core::ptr::null::<::core::ffi::c_char>(),
        stat: stat {
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
        },
        dirstream: ::core::ptr::null_mut::<DIR>(),
        buffer: ::core::ptr::null_mut::<word>(),
        bufsize: 0,
        buffered: 0,
        linbuf: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        linbuf_base: 0,
        buffered_lines: 0,
        valid_lines: 0,
        alloc_lines: 0,
        prefix_end: ::core::ptr::null::<::core::ffi::c_char>(),
        prefix_lines: 0,
        suffix_begin: ::core::ptr::null::<::core::ffi::c_char>(),
        equivs: ::core::ptr::null_mut::<lin>(),
        undiscarded: ::core::ptr::null_mut::<lin>(),
        realindexes: ::core::ptr::null_mut::<lin>(),
        nondiscarded_lines: 0,
        changed: ::core::ptr::null_mut::<bool>(),
        missing_newline: false,
        eof: false,
        equiv_max: 0,
    }; 2],
    parent: ::core::ptr::null::<comparison>(),
};
#[export_name = "rboxc_diffutils_noparent"]
pub static mut noparent: comparison = comparison {
    file: [file_data {
        desc: 0,
        openerr: 0,
        err: 0,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        filetype: ::core::ptr::null::<::core::ffi::c_char>(),
        stat: stat {
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
        },
        dirstream: ::core::ptr::null_mut::<DIR>(),
        buffer: ::core::ptr::null_mut::<word>(),
        bufsize: 0,
        buffered: 0,
        linbuf: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        linbuf_base: 0,
        buffered_lines: 0,
        valid_lines: 0,
        alloc_lines: 0,
        prefix_end: ::core::ptr::null::<::core::ffi::c_char>(),
        prefix_lines: 0,
        suffix_begin: ::core::ptr::null::<::core::ffi::c_char>(),
        equivs: ::core::ptr::null_mut::<lin>(),
        undiscarded: ::core::ptr::null_mut::<lin>(),
        realindexes: ::core::ptr::null_mut::<lin>(),
        nondiscarded_lines: 0,
        changed: ::core::ptr::null_mut::<bool>(),
        missing_newline: false,
        eof: false,
        equiv_max: 0,
    }; 2],
    parent: ::core::ptr::null::<comparison>(),
};
#[export_name = "rboxc_diffutils_excluded"]
pub static mut excluded: *mut exclude = ::core::ptr::null_mut::<exclude>();
#[export_name = "rboxc_diffutils_function_regexp"]
pub static mut function_regexp: re_pattern_buffer = re_pattern_buffer {
    buffer: ::core::ptr::null_mut::<re_dfa_t>(),
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    translate: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
#[export_name = "rboxc_diffutils_ignore_regexp"]
pub static mut ignore_regexp: re_pattern_buffer = re_pattern_buffer {
    buffer: ::core::ptr::null_mut::<re_dfa_t>(),
    allocated: 0,
    used: 0,
    syntax: 0,
    fastmap: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    translate: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
    re_nsub: 0,
    can_be_null_regs_allocated_fastmap_accurate_no_sub_not_bol_not_eol_newline_anchor: [0; 1],
    c2rust_padding: [0; 7],
};
pub const nullptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const GNULIB_LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/diffutils/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"diffutils\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"GNU diffutils\0") };
