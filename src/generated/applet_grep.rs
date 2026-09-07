// Generated from pinned GNU Grep 3.12 by scripts/translate-grep.py.
// Source SHA-256: 22ff6b93485baa033ebd3469ded5695d49b5c9aba89dcdb41cc67237a7ad94a0
/* grep.c - main driver file for grep.
   Copyright (C) 1992, 1997-2002, 2004-2025 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct exclude { _opaque: [u8; 0] }
#[repr(C)]
pub struct __dirstream { _opaque: [u8; 0] }
#[repr(C)]
pub struct cycle_check_state { _opaque: [u8; 0] }
#[repr(C)]
pub struct hash_table { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstatat(
        __fd: ::core::ffi::c_int,
        __file: *const ::core::ffi::c_char,
        __buf: *mut stat,
        __flag: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn c32rtomb(__s: *mut ::core::ffi::c_char, __c32: char32_t, __ps: *mut mbstate_t) -> size_t;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn rawmemchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn memrchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
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
    fn mempcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_grep_rpl_mbrlen"]
    fn rpl_mbrlen(s: *const ::core::ffi::c_char, n: size_t, ps: *mut mbstate_t) -> size_t;
    #[link_name = "rboxc_grep_rpl_mbrtoc32"]
    fn rpl_mbrtoc32(
        pc: *mut char32_t,
        s: *const ::core::ffi::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush_unlocked(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn fputs_unlocked(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fread_unlocked(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn fwrite_unlocked(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn clearerr_unlocked(__stream: *mut FILE);
    fn __uflow(_: *mut FILE) -> ::core::ffi::c_int;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn splice(
        __fdin: ::core::ffi::c_int,
        __offin: *mut __off64_t,
        __fdout: ::core::ffi::c_int,
        __offout: *mut __off64_t,
        __len: size_t,
        __flags: ::core::ffi::c_uint,
    ) -> __ssize_t;
    fn lseek(__fd: ::core::ffi::c_int, __offset: __off_t, __whence: ::core::ffi::c_int) -> __off_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_grep_getprogname"]
    fn getprogname() -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_grep_strip_trailing_slashes"]
    fn strip_trailing_slashes(file: *mut ::core::ffi::c_char) -> bool;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
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
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_grep_argmatch_die"]
    static mut argmatch_die: argmatch_exit_fn;
    #[link_name = "rboxc_grep___xargmatch_internal"]
    fn __xargmatch_internal(
        context: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
        arglist: *const *const ::core::ffi::c_char,
        vallist: *const ::core::ffi::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    #[link_name = "rboxc_grep_c_stack_action"]
    fn c_stack_action(
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_grep_should_colorize"]
    fn should_colorize() -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_init_colorize"]
    fn init_colorize();
    #[link_name = "rboxc_grep_print_start_colorize"]
    fn print_start_colorize(
        sgr_start_0: *const ::core::ffi::c_char,
        sgr_seq: *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_grep_print_end_colorize"]
    fn print_end_colorize(sgr_end_0: *const ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_grep_new_exclude"]
    fn new_exclude() -> *mut exclude;
    #[link_name = "rboxc_grep_add_exclude"]
    fn add_exclude(_: *mut exclude, _: *const ::core::ffi::c_char, _: ::core::ffi::c_int);
    #[link_name = "rboxc_grep_add_exclude_file"]
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
    #[link_name = "rboxc_grep_excluded_file_name"]
    fn excluded_file_name(_: *const exclude, _: *const ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_grep_exit_failure"]
    static mut exit_failure: ::core::ffi::c_int;
    #[link_name = "rboxc_grep_openat_safer"]
    fn openat_safer(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_rpl_fts_close"]
    fn rpl_fts_close(_: *mut FTS) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_rpl_fts_open"]
    fn rpl_fts_open(
        _: *const *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: Option<
            unsafe extern "C" fn(*mut *const FTSENT, *mut *const FTSENT) -> ::core::ffi::c_int,
        >,
    ) -> *mut FTS;
    #[link_name = "rboxc_grep_rpl_fts_read"]
    fn rpl_fts_read(_: *mut FTS) -> *mut FTSENT;
    #[link_name = "rboxc_grep_rpl_fts_set"]
    fn rpl_fts_set(_: *mut FTS, _: *mut FTSENT, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_hash_free"]
    fn hash_free(table: *mut Hash_table);
    #[link_name = "rboxc_grep_hash_initialize"]
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    #[link_name = "rboxc_grep_hash_insert_if_absent"]
    fn hash_insert_if_absent(
        table: *mut Hash_table,
        entry: *const ::core::ffi::c_void,
        matched_ent: *mut *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_safe_read"]
    fn safe_read(fd: ::core::ffi::c_int, buf: *mut ::core::ffi::c_void, count: idx_t) -> ptrdiff_t;
    #[link_name = "rboxc_grep_xalloc_die"]
    fn xalloc_die();
    #[link_name = "rboxc_grep_ximalloc"]
    fn ximalloc(s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_grep_xpalloc"]
    fn xpalloc(
        pa: *mut ::core::ffi::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_grep_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_grep_xnmalloc"]
    fn xnmalloc(n: size_t, s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_grep_init_localeinfo"]
    fn init_localeinfo(_: *mut localeinfo);
    #[link_name = "rboxc_grep_case_folded_counterparts"]
    fn case_folded_counterparts(_: wint_t, _: *mut char32_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_wordinit"]
    fn wordinit();
    #[link_name = "rboxc_grep_GEAcompile"]
    fn GEAcompile(
        _: *mut ::core::ffi::c_char,
        _: idx_t,
        _: reg_syntax_t,
        _: bool,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_grep_EGexecute"]
    fn EGexecute(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: idx_t,
        _: *mut idx_t,
        _: *const ::core::ffi::c_char,
    ) -> ptrdiff_t;
    #[link_name = "rboxc_grep_Fcompile"]
    fn Fcompile(
        _: *mut ::core::ffi::c_char,
        _: idx_t,
        _: reg_syntax_t,
        _: bool,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_grep_Fexecute"]
    fn Fexecute(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: idx_t,
        _: *mut idx_t,
        _: *const ::core::ffi::c_char,
    ) -> ptrdiff_t;
    #[link_name = "rboxc_grep_Pcompile"]
    fn Pcompile(
        _: *mut ::core::ffi::c_char,
        _: idx_t,
        _: reg_syntax_t,
        _: bool,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_grep_Pexecute"]
    fn Pexecute(
        _: *mut ::core::ffi::c_void,
        _: *const ::core::ffi::c_char,
        _: idx_t,
        _: *mut idx_t,
        _: *const ::core::ffi::c_char,
    ) -> ptrdiff_t;
    #[link_name = "rboxc_grep_Pprint_version"]
    fn Pprint_version();
    #[link_name = "rboxc_grep_c_strcasecmp"]
    fn c_strcasecmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_grep_version_etc"]
    fn version_etc(
        stream: *mut FILE,
        command_name: *const ::core::ffi::c_char,
        package: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_grep_emit_bug_reporting_address"]
    fn emit_bug_reporting_address();
    #[link_name = "rboxc_grep_xstrtoimax"]
    fn xstrtoimax(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut intmax_t,
        _: *const ::core::ffi::c_char,
    ) -> strtol_error;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uint_least32_t = __uint32_t;
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
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type dev_t = __dev_t;
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
pub type mbstate_t = __mbstate_t;
pub type char32_t = __uint_least32_t;
pub type uint_fast64_t = ::core::ffi::c_ulong;
pub type intptr_t = isize;
pub type uintptr_t = usize;
pub type intmax_t = ::libc::intmax_t;
pub type uintmax_t = ::libc::uintmax_t;
pub type __gnuc_va_list = __builtin_va_list;
pub type va_list = __gnuc_va_list;
pub type wint_t = ::core::ffi::c_uint;
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const _SC_ARG_MAX: Self = Self(0);
    pub const _SC_CHILD_MAX: Self = Self(1);
    pub const _SC_CLK_TCK: Self = Self(2);
    pub const _SC_NGROUPS_MAX: Self = Self(3);
    pub const _SC_OPEN_MAX: Self = Self(4);
    pub const _SC_STREAM_MAX: Self = Self(5);
    pub const _SC_TZNAME_MAX: Self = Self(6);
    pub const _SC_JOB_CONTROL: Self = Self(7);
    pub const _SC_SAVED_IDS: Self = Self(8);
    pub const _SC_REALTIME_SIGNALS: Self = Self(9);
    pub const _SC_PRIORITY_SCHEDULING: Self = Self(10);
    pub const _SC_TIMERS: Self = Self(11);
    pub const _SC_ASYNCHRONOUS_IO: Self = Self(12);
    pub const _SC_PRIORITIZED_IO: Self = Self(13);
    pub const _SC_SYNCHRONIZED_IO: Self = Self(14);
    pub const _SC_FSYNC: Self = Self(15);
    pub const _SC_MAPPED_FILES: Self = Self(16);
    pub const _SC_MEMLOCK: Self = Self(17);
    pub const _SC_MEMLOCK_RANGE: Self = Self(18);
    pub const _SC_MEMORY_PROTECTION: Self = Self(19);
    pub const _SC_MESSAGE_PASSING: Self = Self(20);
    pub const _SC_SEMAPHORES: Self = Self(21);
    pub const _SC_SHARED_MEMORY_OBJECTS: Self = Self(22);
    pub const _SC_AIO_LISTIO_MAX: Self = Self(23);
    pub const _SC_AIO_MAX: Self = Self(24);
    pub const _SC_AIO_PRIO_DELTA_MAX: Self = Self(25);
    pub const _SC_DELAYTIMER_MAX: Self = Self(26);
    pub const _SC_MQ_OPEN_MAX: Self = Self(27);
    pub const _SC_MQ_PRIO_MAX: Self = Self(28);
    pub const _SC_VERSION: Self = Self(29);
    pub const _SC_PAGESIZE: Self = Self(30);
    pub const _SC_RTSIG_MAX: Self = Self(31);
    pub const _SC_SEM_NSEMS_MAX: Self = Self(32);
    pub const _SC_SEM_VALUE_MAX: Self = Self(33);
    pub const _SC_SIGQUEUE_MAX: Self = Self(34);
    pub const _SC_TIMER_MAX: Self = Self(35);
    pub const _SC_BC_BASE_MAX: Self = Self(36);
    pub const _SC_BC_DIM_MAX: Self = Self(37);
    pub const _SC_BC_SCALE_MAX: Self = Self(38);
    pub const _SC_BC_STRING_MAX: Self = Self(39);
    pub const _SC_COLL_WEIGHTS_MAX: Self = Self(40);
    pub const _SC_EQUIV_CLASS_MAX: Self = Self(41);
    pub const _SC_EXPR_NEST_MAX: Self = Self(42);
    pub const _SC_LINE_MAX: Self = Self(43);
    pub const _SC_RE_DUP_MAX: Self = Self(44);
    pub const _SC_CHARCLASS_NAME_MAX: Self = Self(45);
    pub const _SC_2_VERSION: Self = Self(46);
    pub const _SC_2_C_BIND: Self = Self(47);
    pub const _SC_2_C_DEV: Self = Self(48);
    pub const _SC_2_FORT_DEV: Self = Self(49);
    pub const _SC_2_FORT_RUN: Self = Self(50);
    pub const _SC_2_SW_DEV: Self = Self(51);
    pub const _SC_2_LOCALEDEF: Self = Self(52);
    pub const _SC_PII: Self = Self(53);
    pub const _SC_PII_XTI: Self = Self(54);
    pub const _SC_PII_SOCKET: Self = Self(55);
    pub const _SC_PII_INTERNET: Self = Self(56);
    pub const _SC_PII_OSI: Self = Self(57);
    pub const _SC_POLL: Self = Self(58);
    pub const _SC_SELECT: Self = Self(59);
    pub const _SC_UIO_MAXIOV: Self = Self(60);
    pub const _SC_IOV_MAX: Self = Self(60);
    pub const _SC_PII_INTERNET_STREAM: Self = Self(61);
    pub const _SC_PII_INTERNET_DGRAM: Self = Self(62);
    pub const _SC_PII_OSI_COTS: Self = Self(63);
    pub const _SC_PII_OSI_CLTS: Self = Self(64);
    pub const _SC_PII_OSI_M: Self = Self(65);
    pub const _SC_T_IOV_MAX: Self = Self(66);
    pub const _SC_THREADS: Self = Self(67);
    pub const _SC_THREAD_SAFE_FUNCTIONS: Self = Self(68);
    pub const _SC_GETGR_R_SIZE_MAX: Self = Self(69);
    pub const _SC_GETPW_R_SIZE_MAX: Self = Self(70);
    pub const _SC_LOGIN_NAME_MAX: Self = Self(71);
    pub const _SC_TTY_NAME_MAX: Self = Self(72);
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: Self = Self(73);
    pub const _SC_THREAD_KEYS_MAX: Self = Self(74);
    pub const _SC_THREAD_STACK_MIN: Self = Self(75);
    pub const _SC_THREAD_THREADS_MAX: Self = Self(76);
    pub const _SC_THREAD_ATTR_STACKADDR: Self = Self(77);
    pub const _SC_THREAD_ATTR_STACKSIZE: Self = Self(78);
    pub const _SC_THREAD_PRIORITY_SCHEDULING: Self = Self(79);
    pub const _SC_THREAD_PRIO_INHERIT: Self = Self(80);
    pub const _SC_THREAD_PRIO_PROTECT: Self = Self(81);
    pub const _SC_THREAD_PROCESS_SHARED: Self = Self(82);
    pub const _SC_NPROCESSORS_CONF: Self = Self(83);
    pub const _SC_NPROCESSORS_ONLN: Self = Self(84);
    pub const _SC_PHYS_PAGES: Self = Self(85);
    pub const _SC_AVPHYS_PAGES: Self = Self(86);
    pub const _SC_ATEXIT_MAX: Self = Self(87);
    pub const _SC_PASS_MAX: Self = Self(88);
    pub const _SC_XOPEN_VERSION: Self = Self(89);
    pub const _SC_XOPEN_XCU_VERSION: Self = Self(90);
    pub const _SC_XOPEN_UNIX: Self = Self(91);
    pub const _SC_XOPEN_CRYPT: Self = Self(92);
    pub const _SC_XOPEN_ENH_I18N: Self = Self(93);
    pub const _SC_XOPEN_SHM: Self = Self(94);
    pub const _SC_2_CHAR_TERM: Self = Self(95);
    pub const _SC_2_C_VERSION: Self = Self(96);
    pub const _SC_2_UPE: Self = Self(97);
    pub const _SC_XOPEN_XPG2: Self = Self(98);
    pub const _SC_XOPEN_XPG3: Self = Self(99);
    pub const _SC_XOPEN_XPG4: Self = Self(100);
    pub const _SC_CHAR_BIT: Self = Self(101);
    pub const _SC_CHAR_MAX: Self = Self(102);
    pub const _SC_CHAR_MIN: Self = Self(103);
    pub const _SC_INT_MAX: Self = Self(104);
    pub const _SC_INT_MIN: Self = Self(105);
    pub const _SC_LONG_BIT: Self = Self(106);
    pub const _SC_WORD_BIT: Self = Self(107);
    pub const _SC_MB_LEN_MAX: Self = Self(108);
    pub const _SC_NZERO: Self = Self(109);
    pub const _SC_SSIZE_MAX: Self = Self(110);
    pub const _SC_SCHAR_MAX: Self = Self(111);
    pub const _SC_SCHAR_MIN: Self = Self(112);
    pub const _SC_SHRT_MAX: Self = Self(113);
    pub const _SC_SHRT_MIN: Self = Self(114);
    pub const _SC_UCHAR_MAX: Self = Self(115);
    pub const _SC_UINT_MAX: Self = Self(116);
    pub const _SC_ULONG_MAX: Self = Self(117);
    pub const _SC_USHRT_MAX: Self = Self(118);
    pub const _SC_NL_ARGMAX: Self = Self(119);
    pub const _SC_NL_LANGMAX: Self = Self(120);
    pub const _SC_NL_MSGMAX: Self = Self(121);
    pub const _SC_NL_NMAX: Self = Self(122);
    pub const _SC_NL_SETMAX: Self = Self(123);
    pub const _SC_NL_TEXTMAX: Self = Self(124);
    pub const _SC_XBS5_ILP32_OFF32: Self = Self(125);
    pub const _SC_XBS5_ILP32_OFFBIG: Self = Self(126);
    pub const _SC_XBS5_LP64_OFF64: Self = Self(127);
    pub const _SC_XBS5_LPBIG_OFFBIG: Self = Self(128);
    pub const _SC_XOPEN_LEGACY: Self = Self(129);
    pub const _SC_XOPEN_REALTIME: Self = Self(130);
    pub const _SC_XOPEN_REALTIME_THREADS: Self = Self(131);
    pub const _SC_ADVISORY_INFO: Self = Self(132);
    pub const _SC_BARRIERS: Self = Self(133);
    pub const _SC_BASE: Self = Self(134);
    pub const _SC_C_LANG_SUPPORT: Self = Self(135);
    pub const _SC_C_LANG_SUPPORT_R: Self = Self(136);
    pub const _SC_CLOCK_SELECTION: Self = Self(137);
    pub const _SC_CPUTIME: Self = Self(138);
    pub const _SC_THREAD_CPUTIME: Self = Self(139);
    pub const _SC_DEVICE_IO: Self = Self(140);
    pub const _SC_DEVICE_SPECIFIC: Self = Self(141);
    pub const _SC_DEVICE_SPECIFIC_R: Self = Self(142);
    pub const _SC_FD_MGMT: Self = Self(143);
    pub const _SC_FIFO: Self = Self(144);
    pub const _SC_PIPE: Self = Self(145);
    pub const _SC_FILE_ATTRIBUTES: Self = Self(146);
    pub const _SC_FILE_LOCKING: Self = Self(147);
    pub const _SC_FILE_SYSTEM: Self = Self(148);
    pub const _SC_MONOTONIC_CLOCK: Self = Self(149);
    pub const _SC_MULTI_PROCESS: Self = Self(150);
    pub const _SC_SINGLE_PROCESS: Self = Self(151);
    pub const _SC_NETWORKING: Self = Self(152);
    pub const _SC_READER_WRITER_LOCKS: Self = Self(153);
    pub const _SC_SPIN_LOCKS: Self = Self(154);
    pub const _SC_REGEXP: Self = Self(155);
    pub const _SC_REGEX_VERSION: Self = Self(156);
    pub const _SC_SHELL: Self = Self(157);
    pub const _SC_SIGNALS: Self = Self(158);
    pub const _SC_SPAWN: Self = Self(159);
    pub const _SC_SPORADIC_SERVER: Self = Self(160);
    pub const _SC_THREAD_SPORADIC_SERVER: Self = Self(161);
    pub const _SC_SYSTEM_DATABASE: Self = Self(162);
    pub const _SC_SYSTEM_DATABASE_R: Self = Self(163);
    pub const _SC_TIMEOUTS: Self = Self(164);
    pub const _SC_TYPED_MEMORY_OBJECTS: Self = Self(165);
    pub const _SC_USER_GROUPS: Self = Self(166);
    pub const _SC_USER_GROUPS_R: Self = Self(167);
    pub const _SC_2_PBS: Self = Self(168);
    pub const _SC_2_PBS_ACCOUNTING: Self = Self(169);
    pub const _SC_2_PBS_LOCATE: Self = Self(170);
    pub const _SC_2_PBS_MESSAGE: Self = Self(171);
    pub const _SC_2_PBS_TRACK: Self = Self(172);
    pub const _SC_SYMLOOP_MAX: Self = Self(173);
    pub const _SC_STREAMS: Self = Self(174);
    pub const _SC_2_PBS_CHECKPOINT: Self = Self(175);
    pub const _SC_V6_ILP32_OFF32: Self = Self(176);
    pub const _SC_V6_ILP32_OFFBIG: Self = Self(177);
    pub const _SC_V6_LP64_OFF64: Self = Self(178);
    pub const _SC_V6_LPBIG_OFFBIG: Self = Self(179);
    pub const _SC_HOST_NAME_MAX: Self = Self(180);
    pub const _SC_TRACE: Self = Self(181);
    pub const _SC_TRACE_EVENT_FILTER: Self = Self(182);
    pub const _SC_TRACE_INHERIT: Self = Self(183);
    pub const _SC_TRACE_LOG: Self = Self(184);
    pub const _SC_LEVEL1_ICACHE_SIZE: Self = Self(185);
    pub const _SC_LEVEL1_ICACHE_ASSOC: Self = Self(186);
    pub const _SC_LEVEL1_ICACHE_LINESIZE: Self = Self(187);
    pub const _SC_LEVEL1_DCACHE_SIZE: Self = Self(188);
    pub const _SC_LEVEL1_DCACHE_ASSOC: Self = Self(189);
    pub const _SC_LEVEL1_DCACHE_LINESIZE: Self = Self(190);
    pub const _SC_LEVEL2_CACHE_SIZE: Self = Self(191);
    pub const _SC_LEVEL2_CACHE_ASSOC: Self = Self(192);
    pub const _SC_LEVEL2_CACHE_LINESIZE: Self = Self(193);
    pub const _SC_LEVEL3_CACHE_SIZE: Self = Self(194);
    pub const _SC_LEVEL3_CACHE_ASSOC: Self = Self(195);
    pub const _SC_LEVEL3_CACHE_LINESIZE: Self = Self(196);
    pub const _SC_LEVEL4_CACHE_SIZE: Self = Self(197);
    pub const _SC_LEVEL4_CACHE_ASSOC: Self = Self(198);
    pub const _SC_LEVEL4_CACHE_LINESIZE: Self = Self(199);
    pub const _SC_IPV6: Self = Self(235);
    pub const _SC_RAW_SOCKETS: Self = Self(236);
    pub const _SC_V7_ILP32_OFF32: Self = Self(237);
    pub const _SC_V7_ILP32_OFFBIG: Self = Self(238);
    pub const _SC_V7_LP64_OFF64: Self = Self(239);
    pub const _SC_V7_LPBIG_OFFBIG: Self = Self(240);
    pub const _SC_SS_REPL_MAX: Self = Self(241);
    pub const _SC_TRACE_EVENT_NAME_MAX: Self = Self(242);
    pub const _SC_TRACE_NAME_MAX: Self = Self(243);
    pub const _SC_TRACE_SYS_MAX: Self = Self(244);
    pub const _SC_TRACE_USER_EVENT_MAX: Self = Self(245);
    pub const _SC_XOPEN_STREAMS: Self = Self(246);
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: Self = Self(247);
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: Self = Self(248);
    pub const _SC_MINSIGSTKSZ: Self = Self(249);
    pub const _SC_SIGSTKSZ: Self = Self(250);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const EXIT_TROUBLE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const NCHAR: Self = Self(256);
}
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
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
    pub fts_cycle: C2Rust_Unnamed_3,
    pub fts_fd_ring: I_ring,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct patloc {
    pub lineno: idx_t,
    pub filename: *const ::core::ffi::c_char,
    pub fileline: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: ::core::ffi::c_float,
    pub shrink_factor: ::core::ffi::c_float,
    pub growth_threshold: ::core::ffi::c_float,
    pub growth_factor: ::core::ffi::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_tuning = hash_tuning;
pub type Hash_table = hash_table;
pub type Hash_hasher = Option<unsafe extern "C" fn(*const ::core::ffi::c_void, size_t) -> size_t>;
pub type Hash_comparator =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_void) -> bool>;
pub type Hash_data_freer = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type reg_syntax_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct localeinfo {
    pub multibyte: bool,
    pub simple: bool,
    pub using_utf8: bool,
    pub sbclen: [::core::ffi::c_schar; 256],
    pub sbctowc: [wint_t; 256],
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const SEP_CHAR_SELECTED: Self = Self(58);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const SEP_CHAR_REJECTED: Self = Self(45);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct color_cap {
    pub name: *const ::core::ffi::c_char,
    pub var: *mut *const ::core::ffi::c_char,
    pub fct: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_6(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_6 {
    pub const BINARY_FILES_OPTION: Self = Self(128);
    pub const COLOR_OPTION: Self = Self(129);
    pub const EXCLUDE_DIRECTORY_OPTION: Self = Self(130);
    pub const EXCLUDE_OPTION: Self = Self(131);
    pub const EXCLUDE_FROM_OPTION: Self = Self(132);
    pub const GROUP_SEPARATOR_OPTION: Self = Self(133);
    pub const INCLUDE_OPTION: Self = Self(134);
    pub const LINE_BUFFERED_OPTION: Self = Self(135);
    pub const LABEL_OPTION: Self = Self(136);
    pub const NO_IGNORE_CASE_OPTION: Self = Self(137);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct directories_type(pub ::core::ffi::c_uint);
impl directories_type {
    pub const READ_DIRECTORIES: Self = Self(2);
    pub const RECURSE_DIRECTORIES: Self = Self(3);
    pub const SKIP_DIRECTORIES: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_7(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_7 {
    pub const basic_fts_options: Self = Self(776);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_8(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_8 {
    pub const READ_COMMAND_LINE_DEVICES: Self = Self(0);
    pub const READ_DEVICES: Self = Self(1);
    pub const SKIP_DEVICES: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_9(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_9 {
    pub const LISTFILES_NONE: Self = Self(0);
    pub const LISTFILES_MATCHING: Self = Self(1);
    pub const LISTFILES_NONMATCHING: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_10(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_10 {
    pub const BINARY_BINARY_FILES: Self = Self(0);
    pub const TEXT_BINARY_FILES: Self = Self(1);
    pub const WITHOUT_MATCH_BINARY_FILES: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub _gl_dummy: ::core::ffi::c_int,
}
pub type uword = uintmax_t;
pub type execute_fp_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const ::core::ffi::c_char,
        idx_t,
        *mut idx_t,
        *const ::core::ffi::c_char,
    ) -> ptrdiff_t,
>;
pub type compile_fp_t = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_char,
        idx_t,
        reg_syntax_t,
        bool,
    ) -> *mut ::core::ffi::c_void,
>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_13 {
    pub const uword_size: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_15(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_15 {
    pub const GOOD_READSIZE_MIN: Self = Self(98304);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub name: [::core::ffi::c_char; 12],
    pub syntax: ::core::ffi::c_int,
    pub compile: compile_fp_t,
    pub execute: execute_fp_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_17(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_17 {
    pub const E_MATCHER_INDEX: Self = Self(1);
    pub const F_MATCHER_INDEX: Self = Self(2);
    pub const G_MATCHER_INDEX: Self = Self(0);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub _gl_dummy: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub _gl_dummy: ::core::ffi::c_int,
}
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const INTMAX_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINTMAX_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PTRDIFF_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const WEOF: ::core::ffi::c_uint = 0xffffffff as ::core::ffi::c_uint;
pub const _GL_MBSTATE_ZERO_SIZE: usize = ::core::mem::size_of::<mbstate_t>();
#[inline]
unsafe extern "C" fn mbszero(mut ps: *mut mbstate_t) {
    memset(
        ps as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        _GL_MBSTATE_ZERO_SIZE,
    );
}
pub const MB_LEN_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const SEEK_DATA: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SEEK_HOLE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn getc_unlocked(mut __fp: *mut FILE) -> ::core::ffi::c_int {
    return if ((*__fp)._IO_read_ptr >= (*__fp)._IO_read_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __uflow(__fp)
    } else {
        let c2rust_fresh0 = (*__fp)._IO_read_ptr;
        (*__fp)._IO_read_ptr = (*__fp)._IO_read_ptr.offset(1);
        *(c2rust_fresh0 as *mut ::core::ffi::c_uchar) as ::core::ffi::c_int
    };
}
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
        let c2rust_fresh1 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh1;
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
        let c2rust_fresh2 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh2;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NOCTTY: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const SPLICE_F_MOVE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_CUR: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const LOCALEDIR: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"/root/rboxc/build/oracle/grep/share/locale\0",
    )
};
pub const ENXIO: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const EINVAL: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const ESPIPE: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const EMLINK: ::core::ffi::c_int = 31 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -128 as ::core::ffi::c_int && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[inline]
unsafe extern "C" fn to_uchar(mut ch: ::core::ffi::c_char) -> ::core::ffi::c_uchar {
    return ch as ::core::ffi::c_uchar;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: ::core::ffi::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return r#true != 0,
        _ => return r#false != 0,
    };
}
pub const EXCLUDE_ANCHORED: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int;
pub const EXCLUDE_INCLUDE: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 29 as ::core::ffi::c_int;
pub const EXCLUDE_WILDCARDS: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 28 as ::core::ffi::c_int;
pub const FTS_COMFOLLOW: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FTS_LOGICAL: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const FTS_PHYSICAL: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const FTS_ROOTLEVEL: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FTS_D: ::core::ffi::c_int = 1;
pub const FTS_DC: ::core::ffi::c_int = 2;
pub const FTS_DEFAULT: ::core::ffi::c_int = 3;
pub const FTS_DNR: ::core::ffi::c_int = 4;
pub const FTS_DP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const FTS_ERR: ::core::ffi::c_int = 7;
pub const FTS_F: ::core::ffi::c_int = 8;
pub const FTS_NS: ::core::ffi::c_int = 10;
pub const FTS_NSOK: ::core::ffi::c_int = 11;
pub const FTS_SL: ::core::ffi::c_int = 12;
pub const FTS_SLNONE: ::core::ffi::c_int = 13;
pub const FTS_W: ::core::ffi::c_int = 14;
pub const FTS_SKIP: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const optional_argument: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const IDX_MAX: ::core::ffi::c_long = PTRDIFF_MAX;
pub const GNULIB_LOCALEDIR: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"/root/rboxc/build/oracle/grep/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"grep\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GNU grep\0") };
pub const VERSION: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"3.12\0") };
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
pub const RE_SYNTAX_AWK: ::core::ffi::c_ulong = RE_BACKSLASH_ESCAPE_IN_LISTS
    | RE_DOT_NOT_NULL
    | RE_NO_BK_PARENS
    | RE_NO_BK_REFS
    | RE_NO_BK_VBAR
    | RE_NO_EMPTY_RANGES
    | RE_DOT_NEWLINE
    | RE_CONTEXT_INDEP_ANCHORS
    | RE_CHAR_CLASSES
    | RE_UNMATCHED_RIGHT_PAREN_ORD
    | RE_NO_GNU_OPS;
pub const RE_SYNTAX_GNU_AWK: ::core::ffi::c_ulong =
    (RE_SYNTAX_POSIX_EXTENDED | RE_BACKSLASH_ESCAPE_IN_LISTS | RE_INVALID_INTERVAL_ORD)
        & !(RE_DOT_NOT_NULL | RE_CONTEXT_INDEP_OPS | RE_CONTEXT_INVALID_OPS);
pub const RE_SYNTAX_POSIX_AWK: ::core::ffi::c_ulong = RE_SYNTAX_POSIX_EXTENDED
    | RE_BACKSLASH_ESCAPE_IN_LISTS
    | RE_INTERVALS
    | RE_NO_GNU_OPS
    | RE_INVALID_INTERVAL_ORD;
pub const RE_SYNTAX_GREP: ::core::ffi::c_ulong =
    (RE_SYNTAX_POSIX_BASIC | RE_NEWLINE_ALT) & !(RE_CONTEXT_INVALID_DUP | RE_DOT_NOT_NULL);
pub const RE_SYNTAX_EGREP: ::core::ffi::c_ulong =
    (RE_SYNTAX_POSIX_EXTENDED | RE_INVALID_INTERVAL_ORD | RE_NEWLINE_ALT)
        & !(RE_CONTEXT_INVALID_OPS | RE_DOT_NOT_NULL);
pub const _RE_SYNTAX_POSIX_COMMON: ::core::ffi::c_ulong =
    RE_CHAR_CLASSES | RE_DOT_NEWLINE | RE_DOT_NOT_NULL | RE_INTERVALS | RE_NO_EMPTY_RANGES;
pub const RE_SYNTAX_POSIX_BASIC: ::core::ffi::c_ulong =
    _RE_SYNTAX_POSIX_COMMON | RE_BK_PLUS_QM | RE_CONTEXT_INVALID_DUP;
pub const RE_SYNTAX_POSIX_EXTENDED: ::core::ffi::c_ulong = _RE_SYNTAX_POSIX_COMMON
    | RE_CONTEXT_INDEP_ANCHORS
    | RE_CONTEXT_INDEP_OPS
    | RE_NO_BK_BRACES
    | RE_NO_BK_PARENS
    | RE_NO_BK_VBAR
    | RE_CONTEXT_INVALID_OPS
    | RE_UNMATCHED_RIGHT_PAREN_ORD;
#[inline]
unsafe extern "C" fn imbrlen(
    mut s: *const ::core::ffi::c_char,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: size_t = rpl_mbrlen(s, n as size_t, mbs);
    if len <= MB_LEN_MAX as size_t {
        return len as ptrdiff_t;
    }
    let mut neglen: ptrdiff_t = len.wrapping_neg() as ptrdiff_t;
    return -neglen;
}
#[inline]
unsafe extern "C" fn mb_clen(
    mut s: *const ::core::ffi::c_char,
    mut n: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut len: ::core::ffi::c_schar = localeinfo.sbclen[to_uchar(*s) as usize];
    return if len as ::core::ffi::c_int == -2 as ::core::ffi::c_int {
        imbrlen(s, n, mbs)
    } else {
        len as ptrdiff_t
    };
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
unsafe extern "C" fn xset_binary_mode_error() {}
#[inline]
unsafe extern "C" fn xset_binary_mode(mut fd: ::core::ffi::c_int, mut mode: ::core::ffi::c_int) {
    if set_binary_mode(fd, mode) < 0 as ::core::ffi::c_int {
        xset_binary_mode_error();
    }
}
static mut SEP_STR_GROUP: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"--\0") };
static mut out_stat: stat = stat {
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
static mut show_help: ::core::ffi::c_int = 0;
static mut show_version: bool = false;
static mut suppress_errors: bool = false;
static mut color_option: ::core::ffi::c_int = 0;
static mut only_matching: bool = false;
static mut align_tabs: bool = false;
static mut offset_width: ::core::ffi::c_int = 0;
static mut patloc: *mut patloc = ::core::ptr::null_mut::<patloc>();
static mut patlocs_used: idx_t = 0;
static mut patlocs_allocated: idx_t = 0;
static mut pattern_array: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut n_patterns: idx_t = 0;
static mut pattern_table: *mut Hash_table = ::core::ptr::null_mut::<Hash_table>();
unsafe extern "C" fn hash_pattern(
    mut pat: *const ::core::ffi::c_void,
    mut n_buckets: size_t,
) -> size_t {
    let mut h15: uint_fast64_t = 5381 as uint_fast64_t;
    let mut h32: uint_fast64_t = 3657500101 as uint_fast64_t;
    let mut h64: uint_fast64_t = 4123221751654370051 as ::core::ffi::c_long as uint_fast64_t;
    let mut h: size_t = if h64 <= SIZE_MAX {
        h64 as size_t
    } else if h32 <= SIZE_MAX {
        h32 as size_t
    } else {
        h15 as size_t
    };
    let mut pat_offset: intptr_t =
        pat.expose_provenance() as intptr_t - 1 as ::core::ffi::c_int as intptr_t;
    let mut s: *const ::core::ffi::c_uchar =
        (pattern_array as *const ::core::ffi::c_uchar).offset(pat_offset as isize);
    while *s as ::core::ffi::c_int != '\n' as ::core::ffi::c_int {
        h = h.wrapping_mul(33 as size_t) ^ *s as size_t;
        s = s.offset(1);
    }
    return h.wrapping_rem(n_buckets);
}
unsafe extern "C" fn compare_patterns(
    mut a: *const ::core::ffi::c_void,
    mut b: *const ::core::ffi::c_void,
) -> bool {
    let mut a_offset: intptr_t = a.expose_provenance() as intptr_t - 1 as ::core::ffi::c_int as intptr_t;
    let mut b_offset: intptr_t = b.expose_provenance() as intptr_t - 1 as ::core::ffi::c_int as intptr_t;
    let mut p: *const ::core::ffi::c_char = pattern_array.offset(a_offset as isize);
    let mut q: *const ::core::ffi::c_char = pattern_array.offset(b_offset as isize);
    while *p as ::core::ffi::c_int == *q as ::core::ffi::c_int {
        if *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
            return r#true != 0;
        }
        p = p.offset(1);
        q = q.offset(1);
    }
    return r#false != 0;
}
unsafe extern "C" fn update_patterns(
    mut keys: *mut ::core::ffi::c_char,
    mut dupfree_size: idx_t,
    mut size: idx_t,
    mut filename_0: *const ::core::ffi::c_char,
) -> idx_t {
    let mut dst: *mut ::core::ffi::c_char = keys.offset(dupfree_size as isize);
    let mut fileline: idx_t = 1 as idx_t;
    let mut prev_inserted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut srclim: *const ::core::ffi::c_char = keys.offset(size as isize);
    let mut patsize: idx_t = 0;
    let mut src: *const ::core::ffi::c_char = keys.offset(dupfree_size as isize);
    while src < srclim {
        let mut patend: *const ::core::ffi::c_char = rawmemchr(
            src as *const ::core::ffi::c_void,
            '\n' as ::core::ffi::c_int,
        ) as *const ::core::ffi::c_char;
        patsize = patend
            .offset(1 as ::core::ffi::c_int as isize)
            .offset_from(src) as idx_t;
        memmove(
            dst as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            patsize as size_t,
        );
        let mut dst_offset_1: intptr_t =
            dst.offset_from(keys) as intptr_t + 1 as ::core::ffi::c_int as intptr_t;
        let mut inserted: ::core::ffi::c_int = hash_insert_if_absent(
            pattern_table,
            ::core::ptr::with_exposed_provenance_mut::<::core::ffi::c_void>(dst_offset_1 as usize),
            ::core::ptr::null_mut::<*const ::core::ffi::c_void>(),
        );
        if inserted != 0 {
            if inserted < 0 as ::core::ffi::c_int {
                xalloc_die();
            }
            dst = dst.offset(patsize as isize);
            if prev_inserted == 0 {
                if patlocs_used == patlocs_allocated {
                    patloc = xpalloc(
                        patloc as *mut ::core::ffi::c_void,
                        &raw mut patlocs_allocated,
                        1 as idx_t,
                        -1 as ptrdiff_t,
                        ::core::mem::size_of::<patloc>() as idx_t,
                    ) as *mut patloc;
                }
                let c2rust_fresh5 = patlocs_used;
                patlocs_used += 1;
                *patloc.offset(c2rust_fresh5 as isize) = patloc {
                    lineno: n_patterns,
                    filename: filename_0,
                    fileline: fileline,
                };
            }
            n_patterns += 1;
        }
        prev_inserted = inserted;
        fileline += 1;
        src = src.offset(patsize as isize);
    }
    return dst.offset_from(keys);
}
#[export_name = "rboxc_grep_pattern_file_name"]
pub unsafe extern "C" fn pattern_file_name(
    mut lineno: idx_t,
    mut new_lineno: *mut idx_t,
) -> *const ::core::ffi::c_char {
    let mut i: idx_t = 0;
    i = 1 as idx_t;
    while i < patlocs_used {
        if lineno < (*patloc.offset(i as isize)).lineno {
            break;
        }
        i += 1;
    }
    *new_lineno = lineno - (*patloc.offset((i - 1 as idx_t) as isize)).lineno
        + (*patloc.offset((i - 1 as idx_t) as isize)).fileline;
    return (*patloc.offset((i - 1 as idx_t) as isize)).filename;
}
unsafe extern "C" fn clear_asan_poison() {}
unsafe extern "C" fn asan_poison(mut addr: *const ::core::ffi::c_void, mut size: idx_t) {}
static mut group_separator: *const ::core::ffi::c_char =
    unsafe { &raw const SEP_STR_GROUP as *const ::core::ffi::c_char };
static mut selected_match_color: *const ::core::ffi::c_char =
    b"01;31\0".as_ptr() as *const ::core::ffi::c_char;
static mut context_match_color: *const ::core::ffi::c_char =
    b"01;31\0".as_ptr() as *const ::core::ffi::c_char;
static mut filename_color: *const ::core::ffi::c_char =
    b"35\0".as_ptr() as *const ::core::ffi::c_char;
static mut line_num_color: *const ::core::ffi::c_char =
    b"32\0".as_ptr() as *const ::core::ffi::c_char;
static mut byte_num_color: *const ::core::ffi::c_char =
    b"32\0".as_ptr() as *const ::core::ffi::c_char;
static mut sep_color: *const ::core::ffi::c_char = b"36\0".as_ptr() as *const ::core::ffi::c_char;
static mut selected_line_color: *const ::core::ffi::c_char =
    b"\0".as_ptr() as *const ::core::ffi::c_char;
static mut context_line_color: *const ::core::ffi::c_char =
    b"\0".as_ptr() as *const ::core::ffi::c_char;
static mut sgr_start: *const ::core::ffi::c_char =
    b"\x1B[%sm\x1B[K\0".as_ptr() as *const ::core::ffi::c_char;
static mut sgr_end: *const ::core::ffi::c_char =
    b"\x1B[m\x1B[K\0".as_ptr() as *const ::core::ffi::c_char;
unsafe extern "C" fn pr_sgr_start(mut s: *const ::core::ffi::c_char) {
    if *s != 0 {
        print_start_colorize(sgr_start, s);
    }
}
unsafe extern "C" fn pr_sgr_end(mut s: *const ::core::ffi::c_char) {
    if *s != 0 {
        print_end_colorize(sgr_end);
    }
}
unsafe extern "C" fn pr_sgr_start_if(mut s: *const ::core::ffi::c_char) {
    if color_option != 0 {
        pr_sgr_start(s);
    }
}
unsafe extern "C" fn pr_sgr_end_if(mut s: *const ::core::ffi::c_char) {
    if color_option != 0 {
        pr_sgr_end(s);
    }
}
unsafe extern "C" fn color_cap_mt_fct() {
    context_match_color = selected_match_color;
}
unsafe extern "C" fn color_cap_rv_fct() {
    color_option = -1 as ::core::ffi::c_int;
}
unsafe extern "C" fn color_cap_ne_fct() {
    sgr_start = b"\x1B[%sm\0".as_ptr() as *const ::core::ffi::c_char;
    sgr_end = b"\x1B[m\0".as_ptr() as *const ::core::ffi::c_char;
}
static mut color_dict: [color_cap; 12] = unsafe {
    [
        color_cap {
            name: b"mt\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const selected_match_color as *mut *const ::core::ffi::c_char,
            fct: Some(color_cap_mt_fct as unsafe extern "C" fn() -> ()),
        },
        color_cap {
            name: b"ms\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const selected_match_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"mc\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const context_match_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"fn\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const filename_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"ln\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const line_num_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"bn\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const byte_num_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"se\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const sep_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"sl\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const selected_line_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"cx\0".as_ptr() as *const ::core::ffi::c_char,
            var: &raw const context_line_color as *mut *const ::core::ffi::c_char,
            fct: None,
        },
        color_cap {
            name: b"rv\0".as_ptr() as *const ::core::ffi::c_char,
            var: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            fct: Some(color_cap_rv_fct as unsafe extern "C" fn() -> ()),
        },
        color_cap {
            name: b"ne\0".as_ptr() as *const ::core::ffi::c_char,
            var: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            fct: Some(color_cap_ne_fct as unsafe extern "C" fn() -> ()),
        },
        color_cap {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            var: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
            fct: None,
        },
    ]
};
static mut stdout_errno: ::core::ffi::c_int = 0;
unsafe extern "C" fn putchar_errno(mut c: ::core::ffi::c_int) {
    if putchar_unlocked(c) < 0 as ::core::ffi::c_int {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fputs_errno(mut s: *const ::core::ffi::c_char) {
    if fputs_unlocked(s, stdout) < 0 as ::core::ffi::c_int {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn printf_errno(mut format: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut ap: ::core::ffi::VaList;
    ap = c2rust_args.clone();
    if vfprintf(stdout, format, ap) < 0 as ::core::ffi::c_int {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fwrite_errno(
    mut ptr: *const ::core::ffi::c_void,
    mut size: idx_t,
    mut nmemb: idx_t,
) {
    if if 0 != 0
        && 0 != 0
        && (size as size_t).wrapping_mul(nmemb as size_t) <= 8 as size_t
        && size as size_t != 0 as size_t
    {
        ({
            let mut __ptr: *const ::core::ffi::c_char = ptr as *const ::core::ffi::c_char;
            let mut __stream: *mut FILE = stdout;
            let mut __cnt: size_t = 0;
            __cnt = (size as size_t).wrapping_mul(nmemb as size_t);
            while __cnt > 0 as size_t {
                let c2rust_fresh6 = __ptr;
                __ptr = __ptr.offset(1);
                if putc_unlocked(*c2rust_fresh6 as ::core::ffi::c_int, __stream) == EOF {
                    break;
                }
                __cnt = __cnt.wrapping_sub(1);
            }
            (size as size_t)
                .wrapping_mul(nmemb as size_t)
                .wrapping_sub(__cnt)
                .wrapping_div(size as size_t)
        })
    } else if 0 != 0 && size as size_t == 0 as size_t || 0 != 0 && nmemb as size_t == 0 as size_t {
        0 as ::core::ffi::c_int as size_t
    } else {
        fwrite_unlocked(ptr, size as size_t, nmemb as size_t, stdout)
    } != nmemb as size_t
    {
        stdout_errno = *__errno_location();
    }
}
unsafe extern "C" fn fflush_errno() {
    if fflush_unlocked(stdout) != 0 as ::core::ffi::c_int {
        stdout_errno = *__errno_location();
    }
}
static mut excluded_patterns: [*mut exclude; 2] = [::core::ptr::null_mut::<exclude>(); 2];
static mut excluded_directory_patterns: [*mut exclude; 2] = [::core::ptr::null_mut::<exclude>(); 2];
static mut short_options: [::core::ffi::c_char; 58] = unsafe {
    ::core::mem::transmute::<[u8; 58], [::core::ffi::c_char; 58]>(
        *b"0123456789A:B:C:D:EFGHIPTUVX:abcd:e:f:hiLlm:noqRrsuvwxyZz\0",
    )
};
static mut long_options: [option; 50] = unsafe {
    [
        option {
            name: b"basic-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'G' as ::core::ffi::c_int,
        },
        option {
            name: b"extended-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'E' as ::core::ffi::c_int,
        },
        option {
            name: b"fixed-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'F' as ::core::ffi::c_int,
        },
        option {
            name: b"fixed-strings\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'F' as ::core::ffi::c_int,
        },
        option {
            name: b"perl-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'P' as ::core::ffi::c_int,
        },
        option {
            name: b"after-context\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'A' as ::core::ffi::c_int,
        },
        option {
            name: b"before-context\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'B' as ::core::ffi::c_int,
        },
        option {
            name: b"binary-files\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::BINARY_FILES_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"byte-offset\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'b' as ::core::ffi::c_int,
        },
        option {
            name: b"context\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'C' as ::core::ffi::c_int,
        },
        option {
            name: b"color\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::COLOR_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"colour\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::COLOR_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"count\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'c' as ::core::ffi::c_int,
        },
        option {
            name: b"devices\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'D' as ::core::ffi::c_int,
        },
        option {
            name: b"directories\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'd' as ::core::ffi::c_int,
        },
        option {
            name: b"exclude\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::EXCLUDE_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"exclude-from\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::EXCLUDE_FROM_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"exclude-dir\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::EXCLUDE_DIRECTORY_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'f' as ::core::ffi::c_int,
        },
        option {
            name: b"files-with-matches\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'l' as ::core::ffi::c_int,
        },
        option {
            name: b"files-without-match\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'L' as ::core::ffi::c_int,
        },
        option {
            name: b"group-separator\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::GROUP_SEPARATOR_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: &raw const show_help as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"include\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::INCLUDE_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"ignore-case\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'i' as ::core::ffi::c_int,
        },
        option {
            name: b"no-ignore-case\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::NO_IGNORE_CASE_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"initial-tab\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'T' as ::core::ffi::c_int,
        },
        option {
            name: b"label\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::LABEL_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"line-buffered\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::LINE_BUFFERED_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"line-number\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'n' as ::core::ffi::c_int,
        },
        option {
            name: b"line-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'x' as ::core::ffi::c_int,
        },
        option {
            name: b"max-count\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'm' as ::core::ffi::c_int,
        },
        option {
            name: b"no-filename\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"no-group-separator\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_6::GROUP_SEPARATOR_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"no-messages\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 's' as ::core::ffi::c_int,
        },
        option {
            name: b"null\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'Z' as ::core::ffi::c_int,
        },
        option {
            name: b"null-data\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'z' as ::core::ffi::c_int,
        },
        option {
            name: b"only-matching\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'o' as ::core::ffi::c_int,
        },
        option {
            name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'q' as ::core::ffi::c_int,
        },
        option {
            name: b"recursive\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'r' as ::core::ffi::c_int,
        },
        option {
            name: b"dereference-recursive\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'R' as ::core::ffi::c_int,
        },
        option {
            name: b"regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'e' as ::core::ffi::c_int,
        },
        option {
            name: b"invert-match\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as ::core::ffi::c_int,
        },
        option {
            name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'q' as ::core::ffi::c_int,
        },
        option {
            name: b"text\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'a' as ::core::ffi::c_int,
        },
        option {
            name: b"binary\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'U' as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'V' as ::core::ffi::c_int,
        },
        option {
            name: b"with-filename\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'H' as ::core::ffi::c_int,
        },
        option {
            name: b"word-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'w' as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ]
};
#[export_name = "rboxc_grep_match_icase"]
pub static mut match_icase: bool = false;
#[export_name = "rboxc_grep_match_words"]
pub static mut match_words: bool = false;
#[export_name = "rboxc_grep_match_lines"]
pub static mut match_lines: bool = false;
#[export_name = "rboxc_grep_eolbyte"]
pub static mut eolbyte: ::core::ffi::c_char = 0;
static mut filename: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut omit_dot_slash: bool = false;
static mut errseen: bool = false;
static mut encoding_error_output: bool = false;
static mut directories_args: [*const ::core::ffi::c_char; 4] = [
    b"read\0".as_ptr() as *const ::core::ffi::c_char,
    b"recurse\0".as_ptr() as *const ::core::ffi::c_char,
    b"skip\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut directories_types: [directories_type; 3] = [
    directories_type::READ_DIRECTORIES,
    directories_type::RECURSE_DIRECTORIES,
    directories_type::SKIP_DIRECTORIES,
];
static mut directories: directories_type = directories_type::READ_DIRECTORIES;
static mut fts_options: ::core::ffi::c_int =
    C2Rust_Unnamed_7::basic_fts_options.0 as ::core::ffi::c_int | FTS_COMFOLLOW | FTS_PHYSICAL;
static mut devices: C2Rust_Unnamed_8 = C2Rust_Unnamed_8::READ_COMMAND_LINE_DEVICES;
unsafe extern "C" fn is_device_mode(mut m: mode_t) -> bool {
    return m & __S_IFMT as mode_t == 0o20000 as mode_t
        || m & __S_IFMT as mode_t == 0o60000 as mode_t
        || m & __S_IFMT as mode_t == 0o140000 as mode_t
        || m & __S_IFMT as mode_t == 0o10000 as mode_t;
}
unsafe extern "C" fn skip_devices(mut command_line: bool) -> bool {
    return devices.0 == C2Rust_Unnamed_8::SKIP_DEVICES.0
        || (devices.0 == C2Rust_Unnamed_8::READ_COMMAND_LINE_DEVICES.0) as ::core::ffi::c_int
            & !command_line as ::core::ffi::c_int
            != 0;
}
unsafe extern "C" fn usable_st_size(mut st: *const stat) -> bool {
    return (*st).st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
        || (*st).st_mode.wrapping_sub((*st).st_mode) != 0
        || false;
}
static mut seek_failed: bool = false;
static mut seek_data_failed: bool = false;
static mut execute: execute_fp_t = None;
static mut compiled_pattern: *mut ::core::ffi::c_void =
    ::core::ptr::null_mut::<::core::ffi::c_void>();
#[export_name = "rboxc_grep_input_filename"]
pub unsafe extern "C" fn input_filename() -> *const ::core::ffi::c_char {
    if filename.is_null() {
        filename = dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"(standard input)\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        );
    }
    return filename;
}
unsafe extern "C" fn suppressible_error(mut errnum: ::core::ffi::c_int) {
    if !suppress_errors {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                errnum,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                input_filename(),
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
                    errnum,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    input_filename(),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    errseen = r#true != 0;
}
unsafe extern "C" fn clean_up_stdout() {
    if stdout_errno == 0 {
        close_stdout();
    }
}
static mut uword_max: uword = UINTMAX_MAX as uword;
#[export_name = "rboxc_grep_localeinfo"]
pub static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
static mut unibyte_mask: uword = 0;
unsafe extern "C" fn initialize_unibyte_mask() {
    let mut mask: ::core::ffi::c_uchar = 0 as ::core::ffi::c_uchar;
    let mut ms1b: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i <= UCHAR_MAX {
        if (localeinfo.sbclen[i as usize] as ::core::ffi::c_int != 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            & (mask as ::core::ffi::c_int & i == 0) as ::core::ffi::c_int
            != 0
        {
            while ms1b * 2 as ::core::ffi::c_int <= i {
                ms1b *= 2 as ::core::ffi::c_int;
            }
            mask = (mask as ::core::ffi::c_int | ms1b) as ::core::ffi::c_uchar;
        }
        i += 1;
    }
    unibyte_mask = uword_max
        .wrapping_div(UCHAR_MAX as uword)
        .wrapping_mul(mask as uword);
}
unsafe extern "C" fn skip_easy_bytes(
    mut buf: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut s: *const uword = ::core::ptr::null::<uword>();
    p = buf;
    while (p.expose_provenance() as uintptr_t)
        .wrapping_rem(C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as uintptr_t)
        != 0 as uintptr_t
    {
        if to_uchar(*p) as uword & unibyte_mask != 0 {
            return p;
        }
        p = p.offset(1);
    }
    s = p as *const uword;
    while *s & unibyte_mask == 0 {
        s = s.offset(1);
    }
    p = s as *const ::core::ffi::c_char;
    while to_uchar(*p) as uword & unibyte_mask == 0 {
        p = p.offset(1);
    }
    return p;
}
unsafe extern "C" fn buf_has_encoding_errors(
    mut buf: *mut ::core::ffi::c_char,
    mut size: idx_t,
) -> bool {
    if unibyte_mask == 0 {
        return r#false != 0;
    }
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2Rust_Unnamed { __wch: 0 },
    };
    mbszero(&raw mut mbs);
    let mut clen: ptrdiff_t = 0;
    *buf.offset(size as isize) = -1 as ::core::ffi::c_char;
    let mut p: *const ::core::ffi::c_char = buf;
    loop {
        p = skip_easy_bytes(p);
        if p >= buf.offset(size as isize) as *const ::core::ffi::c_char {
            break;
        }
        clen = imbrlen(p, buf.offset(size as isize).offset_from(p), &raw mut mbs);
        if clen < 0 as ptrdiff_t {
            return r#true != 0;
        }
        p = p.offset(clen as isize);
    }
    return r#false != 0;
}
unsafe extern "C" fn buf_has_nulls(mut buf: *mut ::core::ffi::c_char, mut size: idx_t) -> bool {
    *buf.offset(size as isize) = 0 as ::core::ffi::c_char;
    return strlen(buf) != size as size_t;
}
unsafe extern "C" fn file_must_have_nulls(
    mut size: idx_t,
    mut fd: ::core::ffi::c_int,
    mut st: *const stat,
) -> bool {
    if SEEK_HOLE != SEEK_SET
        && !seek_failed
        && usable_st_size(st) as ::core::ffi::c_int != 0
        && size < (*st).st_size as idx_t
    {
        let mut cur: off_t = size as off_t;
        if O_BINARY != 0 || fd == STDIN_FILENO {
            cur = lseek(fd, 0 as __off_t, SEEK_CUR) as off_t;
            if cur < 0 as off_t {
                return r#false != 0;
            }
        }
        let mut hole_start: off_t = lseek(fd, cur, SEEK_HOLE);
        if 0 as off_t <= hole_start {
            if lseek(fd, cur, SEEK_SET) < 0 as __off_t {
                suppressible_error(*__errno_location());
            }
            if hole_start < (*st).st_size {
                return r#true != 0;
            }
        }
    }
    return r#false != 0;
}
unsafe extern "C" fn context_length_arg(
    mut str: *const ::core::ffi::c_char,
    mut out: *mut intmax_t,
) {
    's_23: {
        match xstrtoimax(
            str,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
            out,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        ) {
            strtol_error::LONGINT_OK | strtol_error::LONGINT_OVERFLOW => {
                if 0 as intmax_t <= *out {
                    break 's_23;
                }
            }
            _ => {}
        }
        if ::core::mem::size_of::<C2Rust_Unnamed_14>() != 0 {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    str,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"invalid context length argument\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        str,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"invalid context length argument\0".as_ptr()
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
            if false {
            } else {
                unreachable!();
            };
        } else {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    str,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"invalid context length argument\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        str,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"invalid context length argument\0".as_ptr()
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
            if false {
            } else {
                unreachable!();
            };
        };
    };
}
unsafe extern "C" fn exclude_options(mut command_line: bool) -> ::core::ffi::c_int {
    return EXCLUDE_WILDCARDS
        | if command_line as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            EXCLUDE_ANCHORED
        };
}
unsafe extern "C" fn skipped_file(
    mut name: *const ::core::ffi::c_char,
    mut command_line: bool,
    mut is_dir: bool,
) -> bool {
    let mut pats: *mut *mut exclude = ::core::ptr::null_mut::<*mut exclude>();
    if !is_dir {
        pats = &raw mut excluded_patterns as *mut *mut exclude;
    } else if directories.0 == directories_type::SKIP_DIRECTORIES.0 {
        return r#true != 0;
    } else if command_line as ::core::ffi::c_int != 0 && omit_dot_slash as ::core::ffi::c_int != 0 {
        return r#false != 0;
    } else {
        pats = &raw mut excluded_directory_patterns as *mut *mut exclude;
    }
    return !(*pats.offset(command_line as isize)).is_null()
        && excluded_file_name(*pats.offset(command_line as isize), name) as ::core::ffi::c_int
            != 0;
}
static mut buffer: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut bufalloc: idx_t = 0;
static mut bufdesc: ::core::ffi::c_int = 0;
static mut bufbeg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut buflim: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut pagesize: idx_t = 0;
static mut good_readsize: idx_t = 0;
static mut bufoffset: off_t = 0;
static mut after_last_match: off_t = 0;
static mut skip_nuls: bool = false;
static mut skip_empty_lines: bool = false;
static mut totalnl: intmax_t = 0;
unsafe extern "C" fn add_count(mut a: intmax_t, mut b: idx_t) -> intmax_t {
    let mut sum: intmax_t = 0;
    let (c2rust_result, c2rust_overflowed) = (a as i128).overflowing_add(b as i128);
    let c2rust_result_narrow = c2rust_result as intmax_t;
    *&raw mut sum = c2rust_result_narrow;
    if c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result {
        if ::core::mem::size_of::<C2Rust_Unnamed_12>() != 0 {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"input is too large to count\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"input is too large to count\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            if false {
            } else {
                unreachable!();
            };
        } else {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"input is too large to count\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"input is too large to count\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            if false {
            } else {
                unreachable!();
            };
        };
    }
    return sum;
}
unsafe extern "C" fn all_zeros(mut buf: *const ::core::ffi::c_char, mut size: idx_t) -> bool {
    let mut p: *const ::core::ffi::c_char = buf;
    while p < buf.offset(size as isize) {
        if *p != 0 {
            return r#false != 0;
        }
        p = p.offset(1);
    }
    return r#true != 0;
}
unsafe extern "C" fn reset(mut fd: ::core::ffi::c_int, mut st: *const stat) -> bool {
    buflim = if (buffer
        .offset(1 as ::core::ffi::c_int as isize)
        .expose_provenance() as uintptr_t)
        .wrapping_rem(pagesize as uintptr_t)
        == 0 as uintptr_t
    {
        buffer.offset(1 as ::core::ffi::c_int as isize)
    } else {
        buffer.offset(1 as ::core::ffi::c_int as isize).offset(
            (pagesize as uintptr_t).wrapping_sub(
                (buffer
                    .offset(1 as ::core::ffi::c_int as isize)
                    .expose_provenance() as uintptr_t)
                    .wrapping_rem(pagesize as uintptr_t),
            ) as isize,
        )
    };
    bufbeg = buflim;
    *bufbeg.offset(-1isize) = eolbyte;
    bufdesc = fd;
    bufoffset = (if fd == STDIN_FILENO {
        lseek(fd, 0 as __off_t, SEEK_CUR)
    } else {
        0 as __off_t
    }) as off_t;
    seek_failed = bufoffset < 0 as off_t;
    seek_data_failed = seek_failed;
    if seek_failed {
        if *__errno_location() != ESPIPE {
            suppressible_error(*__errno_location());
            return r#false != 0;
        }
        bufoffset = 0 as off_t;
    }
    return r#true != 0;
}
unsafe extern "C" fn fillbuf(mut save: idx_t, mut st: *const stat) -> bool {
    let mut readbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut min_after_buflim: idx_t =
        good_readsize + C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as idx_t;
    if min_after_buflim <= buffer.offset(bufalloc as isize).offset_from(buflim) {
        readbuf = buflim;
    } else {
        let mut newbuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut minsize: idx_t = save + good_readsize;
        let mut incr_min: ptrdiff_t = minsize - bufalloc + min_after_buflim;
        if incr_min <= 0 as ptrdiff_t {
            newbuf = buffer;
        } else {
            let mut alloc_max: ptrdiff_t = -1 as ptrdiff_t;
            if usable_st_size(st) {
                let mut to_be_read: off_t = (*st).st_size - bufoffset;
                let mut a: ptrdiff_t = 0;
                if 0 as off_t <= to_be_read && {
                    let (c2rust_result, c2rust_overflowed) =
                        (to_be_read as i128).overflowing_add((save + min_after_buflim) as i128);
                    let c2rust_result_narrow = c2rust_result as ptrdiff_t;
                    *&raw mut a = c2rust_result_narrow;
                    !(c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result)
                } {
                    alloc_max = if a > bufalloc + incr_min {
                        a
                    } else {
                        bufalloc + incr_min
                    };
                }
            }
            newbuf = xpalloc(nullptr, &raw mut bufalloc, incr_min, alloc_max, 1 as idx_t)
                as *mut ::core::ffi::c_char;
        }
        readbuf = if (newbuf
            .offset(1 as ::core::ffi::c_int as isize)
            .offset(save as isize)
            .expose_provenance() as uintptr_t)
            .wrapping_rem(pagesize as uintptr_t)
            == 0 as uintptr_t
        {
            newbuf
                .offset(1 as ::core::ffi::c_int as isize)
                .offset(save as isize)
        } else {
            newbuf
                .offset(1 as ::core::ffi::c_int as isize)
                .offset(save as isize)
                .offset(
                    (pagesize as uintptr_t).wrapping_sub(
                        (newbuf
                            .offset(1 as ::core::ffi::c_int as isize)
                            .offset(save as isize)
                            .expose_provenance() as uintptr_t)
                            .wrapping_rem(pagesize as uintptr_t),
                    ) as isize,
                )
        };
        let mut moved: idx_t = save + 1 as idx_t;
        memmove(
            readbuf.offset(-(moved as isize)) as *mut ::core::ffi::c_void,
            buflim.offset(-(moved as isize)) as *const ::core::ffi::c_void,
            moved as size_t,
        );
        if (0 as ptrdiff_t) < incr_min {
            free(buffer as *mut ::core::ffi::c_void);
            buffer = newbuf;
        }
    }
    bufbeg = readbuf.offset(-(save as isize));
    clear_asan_poison();
    let mut fillsize: ptrdiff_t = 0;
    let mut cc: bool = r#true != 0;
    loop {
        fillsize = safe_read(bufdesc, readbuf as *mut ::core::ffi::c_void, good_readsize);
        if fillsize < 0 as ptrdiff_t {
            fillsize = 0 as ptrdiff_t;
            cc = r#false != 0;
        }
        bufoffset += fillsize as ::core::ffi::c_long;
        if (fillsize == 0 as ptrdiff_t) as ::core::ffi::c_int | !skip_nuls as ::core::ffi::c_int
            != 0
            || !all_zeros(readbuf, fillsize)
        {
            break;
        }
        totalnl = add_count(totalnl, fillsize);
        if SEEK_DATA != SEEK_SET && !seek_data_failed {
            let mut data_start: off_t = lseek(bufdesc, bufoffset, SEEK_DATA);
            if data_start < 0 as off_t
                && *__errno_location() == ENXIO
                && usable_st_size(st) as ::core::ffi::c_int != 0
                && bufoffset < (*st).st_size
            {
                data_start = lseek(bufdesc, 0 as __off_t, SEEK_END) as off_t;
            }
            if data_start < 0 as off_t {
                seek_data_failed = r#true != 0;
            } else {
                totalnl = add_count(totalnl, data_start as idx_t - bufoffset as idx_t);
                bufoffset = data_start;
            }
        }
    }
    buflim = readbuf.offset(fillsize as isize);
    memset(
        buflim as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as size_t,
    );
    asan_poison(
        buflim.offset(C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as isize)
            as *const ::core::ffi::c_void,
        bufalloc
            - buflim.offset_from(buffer)
            - C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as idx_t,
    );
    return cc;
}
static mut binary_files: C2Rust_Unnamed_10 = C2Rust_Unnamed_10::BINARY_BINARY_FILES;
static mut list_files: C2Rust_Unnamed_9 = C2Rust_Unnamed_9::LISTFILES_NONE;
static mut out_file: ::core::ffi::c_int = 0;
static mut filename_mask: ::core::ffi::c_int = 0;
static mut out_quiet: bool = false;
static mut out_invert: bool = false;
static mut out_line: bool = false;
static mut out_byte: bool = false;
static mut out_before: intmax_t = 0;
static mut out_after: intmax_t = 0;
static mut count_matches: bool = false;
static mut max_count: intmax_t = 0;
static mut line_buffered: bool = false;
static mut label: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut totalcc: intmax_t = 0;
static mut lastnl: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut lastout: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut outleft: intmax_t = 0;
static mut pending: intmax_t = 0;
static mut done_on_match: bool = false;
static mut exit_on_match: bool = false;
static mut dev_null_output: bool = false;
static mut binary: bool = false;
unsafe extern "C" fn nlscan(mut lim: *const ::core::ffi::c_char) {
    let mut newlines: idx_t = 0 as idx_t;
    let mut beg: *const ::core::ffi::c_char = lastnl;
    while beg < lim {
        beg = memchr(
            beg as *const ::core::ffi::c_void,
            eolbyte as ::core::ffi::c_int,
            lim.offset_from(beg) as size_t,
        ) as *const ::core::ffi::c_void as *const ::core::ffi::c_char;
        if beg.is_null() {
            break;
        }
        newlines += 1;
        beg = beg.offset(1);
    }
    totalnl = add_count(totalnl, newlines);
    lastnl = lim;
}
unsafe extern "C" fn print_filename() {
    pr_sgr_start_if(filename_color);
    fputs_errno(input_filename());
    pr_sgr_end_if(filename_color);
}
unsafe extern "C" fn print_sep(mut sep: ::core::ffi::c_char) {
    pr_sgr_start_if(sep_color);
    putchar_errno(sep as ::core::ffi::c_int);
    pr_sgr_end_if(sep_color);
}
unsafe extern "C" fn print_offset(mut pos: intmax_t, mut color: *const ::core::ffi::c_char) {
    pr_sgr_start_if(color);
    printf_errno(
        b"%*ld\0".as_ptr() as *const ::core::ffi::c_char,
        offset_width,
        pos,
    );
    pr_sgr_end_if(color);
}
unsafe extern "C" fn print_line_head(
    mut beg: *mut ::core::ffi::c_char,
    mut len: idx_t,
    mut lim: *const ::core::ffi::c_char,
    mut sep: ::core::ffi::c_char,
) -> bool {
    if binary_files.0 != C2Rust_Unnamed_10::TEXT_BINARY_FILES.0 {
        let mut ch: ::core::ffi::c_char = *beg.offset(len as isize);
        let mut encoding_errors: bool = buf_has_encoding_errors(beg, len);
        *beg.offset(len as isize) = ch;
        if encoding_errors {
            encoding_error_output = r#true != 0;
            return r#false != 0;
        }
    }
    if out_file != 0 {
        print_filename();
        if filename_mask != 0 {
            print_sep(sep);
        } else {
            putchar_errno(0 as ::core::ffi::c_int);
        }
    }
    if out_line {
        if lastnl < lim {
            nlscan(beg);
            totalnl = add_count(totalnl, 1 as idx_t);
            lastnl = lim;
        }
        print_offset(totalnl, line_num_color);
        print_sep(sep);
    }
    if out_byte {
        let mut pos: intmax_t = add_count(totalcc, beg.offset_from(bufbeg));
        print_offset(pos, byte_num_color);
        print_sep(sep);
    }
    if align_tabs as ::core::ffi::c_int != 0
        && out_file | out_line as ::core::ffi::c_int | out_byte as ::core::ffi::c_int != 0
        && len != 0 as idx_t
    {
        putchar_errno('\t' as ::core::ffi::c_int);
    }
    return r#true != 0;
}
unsafe extern "C" fn print_line_middle(
    mut beg: *mut ::core::ffi::c_char,
    mut lim: *mut ::core::ffi::c_char,
    mut line_color: *const ::core::ffi::c_char,
    mut match_color: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut match_size: idx_t = 0;
    let mut match_offset: ptrdiff_t = 0;
    let mut cur: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut mid: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut b: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    cur = beg;
    while cur < lim && {
        match_offset = execute.expect("non-null function pointer")(
            compiled_pattern,
            beg,
            lim.offset_from(beg),
            &raw mut match_size,
            cur,
        );
        0 as ptrdiff_t <= match_offset
    } {
        b = beg.offset(match_offset as isize);
        if b == lim {
            break;
        }
        if match_size == 0 as idx_t {
            match_size = 1 as idx_t;
            if mid.is_null() {
                mid = cur;
            }
        } else {
            if only_matching {
                let mut sep: ::core::ffi::c_char = (if out_invert as ::core::ffi::c_int != 0 {
                    C2Rust_Unnamed_5::SEP_CHAR_REJECTED.0 as ::core::ffi::c_int
                } else {
                    C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int
                }) as ::core::ffi::c_char;
                if !print_line_head(b, match_size, lim, sep) {
                    return ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            } else {
                pr_sgr_start(line_color);
                if !mid.is_null() {
                    cur = mid;
                    mid = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                fwrite_errno(
                    cur as *const ::core::ffi::c_void,
                    1 as idx_t,
                    b.offset_from(cur),
                );
            }
            pr_sgr_start_if(match_color);
            fwrite_errno(b as *const ::core::ffi::c_void, 1 as idx_t, match_size);
            pr_sgr_end_if(match_color);
            if only_matching {
                putchar_errno(eolbyte as ::core::ffi::c_int);
            }
        }
        cur = b.offset(match_size as isize);
    }
    if only_matching {
        cur = lim;
    } else if !mid.is_null() {
        cur = mid;
    }
    return cur;
}
unsafe extern "C" fn print_line_tail(
    mut beg: *mut ::core::ffi::c_char,
    mut lim: *const ::core::ffi::c_char,
    mut line_color: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut eol_size: idx_t = 0;
    let mut tail_size: idx_t = 0;
    eol_size = (lim > beg as *const ::core::ffi::c_char
        && *lim.offset(-1isize) as ::core::ffi::c_int == eolbyte as ::core::ffi::c_int)
        as ::core::ffi::c_int as idx_t;
    eol_size += (lim.offset(-(eol_size as isize)) > beg as *const ::core::ffi::c_char
        && *lim.offset(-(1 as idx_t + eol_size) as isize) as ::core::ffi::c_int
            == '\r' as ::core::ffi::c_int) as ::core::ffi::c_int as idx_t;
    tail_size = lim.offset(-(eol_size as isize)).offset_from(beg) as idx_t;
    if tail_size > 0 as idx_t {
        pr_sgr_start(line_color);
        fwrite_errno(beg as *const ::core::ffi::c_void, 1 as idx_t, tail_size);
        beg = beg.offset(tail_size as isize);
        pr_sgr_end(line_color);
    }
    return beg;
}
unsafe extern "C" fn prline(
    mut beg: *mut ::core::ffi::c_char,
    mut lim: *mut ::core::ffi::c_char,
    mut sep: ::core::ffi::c_char,
) {
    let mut matching: bool = false;
    let mut line_color: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut match_color: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    if !only_matching {
        if !print_line_head(beg, lim.offset_from(beg) - 1 as idx_t, lim, sep) {
            return;
        }
    }
    matching = (sep as ::core::ffi::c_int
        == C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int)
        as ::core::ffi::c_int
        ^ out_invert as ::core::ffi::c_int
        != 0;
    if color_option != 0 {
        line_color = if (sep as ::core::ffi::c_int
            == C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int)
            as ::core::ffi::c_int
            ^ (out_invert as ::core::ffi::c_int != 0 && color_option < 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int
            != 0
        {
            selected_line_color
        } else {
            context_line_color
        };
        match_color = if sep as ::core::ffi::c_int
            == C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int
        {
            selected_match_color
        } else {
            context_match_color
        };
    } else {
        match_color = ::core::ptr::null::<::core::ffi::c_char>();
        line_color = match_color;
    }
    if only_matching as ::core::ffi::c_int != 0 && matching as ::core::ffi::c_int != 0
        || color_option != 0
            && (*line_color as ::core::ffi::c_int != 0 || *match_color as ::core::ffi::c_int != 0)
    {
        if matching as ::core::ffi::c_int != 0
            && (only_matching as ::core::ffi::c_int != 0 || *match_color as ::core::ffi::c_int != 0)
        {
            beg = print_line_middle(beg, lim, line_color, match_color);
            if beg.is_null() {
                return;
            }
        }
        if !only_matching && *line_color as ::core::ffi::c_int != 0 {
            beg = print_line_tail(beg, lim, line_color);
        }
    }
    if !only_matching && lim > beg {
        fwrite_errno(
            beg as *const ::core::ffi::c_void,
            1 as idx_t,
            lim.offset_from(beg),
        );
    }
    if line_buffered {
        fflush_errno();
    }
    if stdout_errno != 0 {
        if ::core::mem::size_of::<C2Rust_Unnamed_11>() != 0 {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    stdout_errno,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        stdout_errno,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            if false {
            } else {
                unreachable!();
            };
        } else {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                    stdout_errno,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        stdout_errno,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            if false {
            } else {
                unreachable!();
            };
        };
    }
    lastout = lim;
}
unsafe extern "C" fn prpending(mut lim: *const ::core::ffi::c_char) {
    if lastout.is_null() {
        lastout = bufbeg;
    }
    while (0 as intmax_t) < pending && lastout < lim as *mut ::core::ffi::c_char {
        let mut nl: *mut ::core::ffi::c_char = rawmemchr(
            lastout as *const ::core::ffi::c_void,
            eolbyte as ::core::ffi::c_int,
        ) as *mut ::core::ffi::c_char;
        prline(
            lastout,
            nl.offset(1 as ::core::ffi::c_int as isize),
            C2Rust_Unnamed_5::SEP_CHAR_REJECTED.0 as ::core::ffi::c_int as ::core::ffi::c_char,
        );
        pending -= 1;
    }
}
unsafe extern "C" fn prtext(mut beg: *mut ::core::ffi::c_char, mut lim: *mut ::core::ffi::c_char) {
    static mut used: bool = false;
    let mut eol: ::core::ffi::c_char = eolbyte;
    if !out_quiet && pending > 0 as intmax_t {
        prpending(beg);
    }
    let mut p: *mut ::core::ffi::c_char = beg;
    if !out_quiet {
        let mut bp: *const ::core::ffi::c_char = if !lastout.is_null() { lastout } else { bufbeg };
        let mut i: intmax_t = 0;
        i = 0 as intmax_t;
        while i < out_before {
            if p > bp as *mut ::core::ffi::c_char {
                loop {
                    p = p.offset(-1);
                    if *p.offset(-1isize) as ::core::ffi::c_int == eol as ::core::ffi::c_int {
                        break;
                    }
                }
            }
            i += 1;
        }
        if (0 as intmax_t <= out_before || 0 as intmax_t <= out_after)
            && used as ::core::ffi::c_int != 0
            && p != lastout
            && !group_separator.is_null()
        {
            pr_sgr_start_if(sep_color);
            fputs_errno(group_separator);
            pr_sgr_end_if(sep_color);
            putchar_errno('\n' as ::core::ffi::c_int);
        }
        while p < beg {
            let mut nl: *mut ::core::ffi::c_char =
                rawmemchr(p as *const ::core::ffi::c_void, eol as ::core::ffi::c_int)
                    as *mut ::core::ffi::c_char;
            nl = nl.offset(1);
            prline(
                p,
                nl,
                C2Rust_Unnamed_5::SEP_CHAR_REJECTED.0 as ::core::ffi::c_int as ::core::ffi::c_char,
            );
            p = nl;
        }
    }
    let mut n: intmax_t = 0;
    if out_invert {
        n = 0 as intmax_t;
        while p < lim && n < outleft {
            let mut nl_0: *mut ::core::ffi::c_char =
                rawmemchr(p as *const ::core::ffi::c_void, eol as ::core::ffi::c_int)
                    as *mut ::core::ffi::c_char;
            nl_0 = nl_0.offset(1);
            if !out_quiet {
                prline(
                    p,
                    nl_0,
                    C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int
                        as ::core::ffi::c_char,
                );
            }
            p = nl_0;
            n += 1;
        }
    } else {
        if !out_quiet {
            prline(
                beg,
                lim,
                C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int as ::core::ffi::c_char,
            );
        }
        n = 1 as intmax_t;
        p = lim;
    }
    after_last_match = (bufoffset as isize - buflim.offset_from(p)) as off_t;
    pending = if out_quiet as ::core::ffi::c_int != 0 {
        0 as intmax_t
    } else if 0 as intmax_t > out_after {
        0 as intmax_t
    } else {
        out_after
    };
    used = r#true != 0;
    outleft -= n;
}
unsafe extern "C" fn zap_nuls(
    mut p: *mut ::core::ffi::c_char,
    mut lim: *mut ::core::ffi::c_char,
    mut eol: ::core::ffi::c_char,
) {
    if eol != 0 {
        loop {
            *lim = '\0' as ::core::ffi::c_char;
            p = p.offset(strlen(p) as isize);
            *lim = eol;
            if p == lim {
                break;
            }
            loop {
                let c2rust_fresh8 = p;
                p = p.offset(1);
                *c2rust_fresh8 = eol;
                if *p != 0 {
                    break;
                }
            }
        }
    }
}
unsafe extern "C" fn grepbuf(
    mut beg: *mut ::core::ffi::c_char,
    mut lim: *const ::core::ffi::c_char,
) -> intmax_t {
    let mut outleft0: intmax_t = outleft;
    let mut endp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut p: *mut ::core::ffi::c_char = beg;
    while p < lim as *mut ::core::ffi::c_char {
        let mut match_size: idx_t = 0;
        let mut match_offset: ptrdiff_t = execute.expect("non-null function pointer")(
            compiled_pattern,
            p,
            lim.offset_from(p),
            &raw mut match_size,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if match_offset < 0 as ptrdiff_t {
            if !out_invert {
                break;
            }
            match_offset = lim.offset_from(p) as ptrdiff_t;
            match_size = 0 as idx_t;
        }
        let mut b: *mut ::core::ffi::c_char = p.offset(match_offset as isize);
        endp = b.offset(match_size as isize);
        if !out_invert && b == lim as *mut ::core::ffi::c_char {
            break;
        }
        if !out_invert || p < b {
            if list_files.0 != C2Rust_Unnamed_9::LISTFILES_NONE.0 {
                return 1 as intmax_t;
            }
            let mut prbeg: *mut ::core::ffi::c_char = if out_invert as ::core::ffi::c_int != 0 {
                p
            } else {
                b
            };
            let mut prend: *mut ::core::ffi::c_char = if out_invert as ::core::ffi::c_int != 0 {
                b
            } else {
                endp
            };
            prtext(prbeg, prend);
            if outleft == 0 || done_on_match as ::core::ffi::c_int != 0 {
                if exit_on_match {
                    stdout_errno = -1 as ::core::ffi::c_int;
                    exit(EXIT_SUCCESS);
                }
                break;
            }
        }
        p = endp;
    }
    return outleft0 - outleft;
}
unsafe extern "C" fn grep(
    mut fd: ::core::ffi::c_int,
    mut st: *const stat,
    mut ineof: *mut bool,
) -> intmax_t {
    let mut nlines: intmax_t = 0;
    let mut i: intmax_t = 0;
    let mut residue: idx_t = 0;
    let mut save: idx_t = 0;
    let mut eol: ::core::ffi::c_char = eolbyte;
    let mut nul_zapper: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
    let mut done_on_match_0: bool = done_on_match;
    let mut out_quiet_0: bool = out_quiet;
    let mut nlines_first_null: intmax_t = -1 as intmax_t;
    if !reset(fd, st) {
        return 0 as intmax_t;
    }
    totalcc = 0 as intmax_t;
    lastout = ::core::ptr::null_mut::<::core::ffi::c_char>();
    totalnl = 0 as intmax_t;
    outleft = max_count;
    after_last_match = 0 as off_t;
    pending = 0 as intmax_t;
    skip_nuls = skip_empty_lines as ::core::ffi::c_int != 0 && eol == 0;
    encoding_error_output = r#false != 0;
    nlines = 0 as intmax_t;
    residue = 0 as idx_t;
    save = 0 as idx_t;
    if !fillbuf(save, st) {
        suppressible_error(*__errno_location());
        return 0 as intmax_t;
    }
    offset_width = 0 as ::core::ffi::c_int;
    if align_tabs {
        let mut num: intmax_t = if usable_st_size(st) as ::core::ffi::c_int != 0 {
            (*st).st_size as intmax_t
        } else {
            INTMAX_MAX as intmax_t
        };
        num += (out_line as ::core::ffi::c_int != 0 && num < INTMAX_MAX as intmax_t)
            as ::core::ffi::c_int as intmax_t;
        loop {
            offset_width += 1;
            num /= 10 as intmax_t;
            if num == 0 as intmax_t {
                break;
            }
        }
    }
    let mut firsttime: bool = r#true != 0;
    '_finish_grep: {
        loop {
            if nlines_first_null < 0 as intmax_t
                && eol as ::core::ffi::c_int != 0
                && binary_files.0 != C2Rust_Unnamed_10::TEXT_BINARY_FILES.0
                && (buf_has_nulls(bufbeg, buflim.offset_from(bufbeg)) as ::core::ffi::c_int != 0
                    || firsttime as ::core::ffi::c_int != 0
                        && file_must_have_nulls(buflim.offset_from(bufbeg), fd, st)
                            as ::core::ffi::c_int
                            != 0)
            {
                if binary_files.0 == C2Rust_Unnamed_10::WITHOUT_MATCH_BINARY_FILES.0 {
                    return 0 as intmax_t;
                }
                if !count_matches {
                    out_quiet = r#true != 0;
                    if max_count == INTMAX_MAX as intmax_t {
                        done_on_match = r#true != 0;
                    }
                }
                nlines_first_null = nlines;
                nul_zapper = eol;
                skip_nuls = skip_empty_lines;
            }
            lastnl = bufbeg;
            if !lastout.is_null() {
                lastout = bufbeg;
            }
            let mut beg: *mut ::core::ffi::c_char = bufbeg.offset(save as isize);
            if beg == buflim {
                *ineof = r#true != 0;
                break;
            } else {
                zap_nuls(beg, buflim, nul_zapper);
                let mut last_eol: *mut ::core::ffi::c_char = memrchr(
                    beg as *const ::core::ffi::c_void,
                    eol as ::core::ffi::c_int,
                    buflim.offset_from(beg) as size_t,
                )
                    as *mut ::core::ffi::c_char;
                beg = beg.offset(-(residue as isize));
                let mut lim: *mut ::core::ffi::c_char = if !last_eol.is_null() {
                    last_eol.offset(1 as ::core::ffi::c_int as isize)
                } else {
                    beg
                };
                residue = buflim.offset_from(lim) as idx_t;
                if beg < lim {
                    if outleft != 0 {
                        nlines += grepbuf(beg, lim);
                    }
                    if pending != 0 {
                        prpending(lim);
                    }
                    if outleft == 0 && pending == 0
                        || done_on_match as ::core::ffi::c_int != 0
                            && if 0 as intmax_t > nlines_first_null {
                                0 as intmax_t
                            } else {
                                nlines_first_null
                            } < nlines
                    {
                        break '_finish_grep;
                    }
                }
                i = 0 as intmax_t;
                beg = lim;
                while i < out_before && beg > bufbeg && beg != lastout {
                    i += 1;
                    loop {
                        beg = beg.offset(-1);
                        if *beg.offset(-1isize) as ::core::ffi::c_int == eol as ::core::ffi::c_int {
                            break;
                        }
                    }
                }
                if beg != lastout {
                    lastout = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                save = lim.offset(residue as isize).offset_from(beg) as idx_t;
                if out_byte {
                    totalcc = add_count(totalcc, buflim.offset_from(bufbeg) - save);
                }
                if out_line {
                    nlscan(beg);
                }
                if !fillbuf(save, st) {
                    suppressible_error(*__errno_location());
                    break '_finish_grep;
                } else {
                    firsttime = r#false != 0;
                }
            }
        }
        if residue != 0 {
            let c2rust_fresh7 = buflim;
            buflim = buflim.offset(1);
            *c2rust_fresh7 = eol;
            if outleft != 0 {
                nlines += grepbuf(
                    bufbeg.offset(save as isize).offset(-(residue as isize)),
                    buflim,
                );
            }
            if pending != 0 {
                prpending(buflim);
            }
        }
    }
    done_on_match = done_on_match_0;
    out_quiet = out_quiet_0;
    if binary_files.0 == C2Rust_Unnamed_10::BINARY_BINARY_FILES.0
        && !out_quiet
        && (encoding_error_output as ::core::ffi::c_int != 0
            || 0 as intmax_t <= nlines_first_null && nlines_first_null < nlines)
    {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"%s: binary file matches\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                input_filename(),
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
                        b"%s: binary file matches\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    input_filename(),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return nlines;
}
unsafe extern "C" fn grepdirent(
    mut fts: *mut FTS,
    mut ent: *mut FTSENT,
    mut command_line: bool,
) -> bool {
    let mut follow: bool = false;
    command_line = command_line as ::core::ffi::c_int
        & ((*ent).fts_level == FTS_ROOTLEVEL as ptrdiff_t) as ::core::ffi::c_int
        != 0;
    if (*ent).fts_info as ::core::ffi::c_int == FTS_DP {
        return r#true != 0;
    }
    if !command_line
        && skipped_file(
            &raw mut (*ent).fts_name as *mut ::core::ffi::c_char,
            r#false != 0,
            (*ent).fts_info as ::core::ffi::c_int == FTS_D
                || (*ent).fts_info as ::core::ffi::c_int == FTS_DC
                || (*ent).fts_info as ::core::ffi::c_int == FTS_DNR,
        ) as ::core::ffi::c_int
            != 0
    {
        rpl_fts_set(fts, ent, FTS_SKIP);
        return r#true != 0;
    }
    filename = (*ent).fts_path;
    if omit_dot_slash as ::core::ffi::c_int != 0
        && *filename.offset(1isize) as ::core::ffi::c_int != 0
    {
        filename = filename.offset(2 as ::core::ffi::c_int as isize);
    }
    follow = (*fts).fts_options & FTS_LOGICAL != 0
        || (*fts).fts_options & FTS_COMFOLLOW != 0 && command_line as ::core::ffi::c_int != 0;
    match (*ent).fts_info as ::core::ffi::c_int {
        FTS_D => {
            if directories.0 == directories_type::RECURSE_DIRECTORIES.0 {
                return r#true != 0;
            }
            rpl_fts_set(fts, ent, FTS_SKIP);
        }
        FTS_DC => {
            if !suppress_errors {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s: warning: recursive directory loop\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        filename,
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
                                b"%s: warning: recursive directory loop\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            filename,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            return r#true != 0;
        }
        FTS_DNR | FTS_ERR | FTS_NS => {
            suppressible_error((*ent).fts_errno);
            return r#true != 0;
        }
        FTS_DEFAULT | FTS_NSOK => {
            if skip_devices(command_line) {
                let mut st: *mut stat = &raw mut (*ent).fts_statp as *mut stat;
                let mut st1: stat = stat {
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
                if (*st).st_mode == 0 {
                    let mut flag: ::core::ffi::c_int = if follow as ::core::ffi::c_int != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        AT_SYMLINK_NOFOLLOW
                    };
                    if fstatat((*fts).fts_cwd_fd, (*ent).fts_accpath, &raw mut st1, flag)
                        != 0 as ::core::ffi::c_int
                    {
                        suppressible_error(*__errno_location());
                        return r#true != 0;
                    }
                    st = &raw mut st1;
                }
                if is_device_mode((*st).st_mode) {
                    return r#true != 0;
                }
            }
        }
        FTS_F | FTS_SLNONE => {}
        FTS_SL | FTS_W => return r#true != 0,
        _ => {
            abort();
        }
    }
    return grepfile((*fts).fts_cwd_fd, (*ent).fts_accpath, follow, command_line);
}
unsafe extern "C" fn open_symlink_nofollow_error(mut err: ::core::ffi::c_int) -> bool {
    if err == ELOOP || err == EMLINK {
        return r#true != 0;
    }
    return r#false != 0;
}
unsafe extern "C" fn grepfile(
    mut dirdesc: ::core::ffi::c_int,
    mut name: *const ::core::ffi::c_char,
    mut follow: bool,
    mut command_line: bool,
) -> bool {
    let mut oflag: ::core::ffi::c_int = O_RDONLY
        | O_NOCTTY
        | if binary as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }
        | if follow as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            O_NOFOLLOW
        }
        | if skip_devices(command_line) as ::core::ffi::c_int != 0 {
            O_NONBLOCK
        } else {
            0 as ::core::ffi::c_int
        };
    let mut desc: ::core::ffi::c_int = openat_safer(dirdesc, name, oflag);
    if desc < 0 as ::core::ffi::c_int {
        if follow as ::core::ffi::c_int != 0 || !open_symlink_nofollow_error(*__errno_location()) {
            suppressible_error(*__errno_location());
        }
        return r#true != 0;
    }
    return grepdesc(desc, command_line);
}
unsafe extern "C" fn drain_input(mut fd: ::core::ffi::c_int, mut st: *const stat) -> bool {
    let mut nbytes: ssize_t = 0;
    if (*st).st_mode & __S_IFMT as __mode_t == 0o10000 as __mode_t
        && dev_null_output as ::core::ffi::c_int != 0
    {
        nbytes = splice(
            fd,
            ::core::ptr::null_mut::<__off64_t>(),
            STDOUT_FILENO,
            ::core::ptr::null_mut::<__off64_t>(),
            good_readsize as size_t,
            SPLICE_F_MOVE as ::core::ffi::c_uint,
        ) as ssize_t;
        if 0 as ssize_t <= nbytes || *__errno_location() != EINVAL {
            while (0 as ssize_t) < nbytes {
                nbytes = splice(
                    fd,
                    ::core::ptr::null_mut::<__off64_t>(),
                    STDOUT_FILENO,
                    ::core::ptr::null_mut::<__off64_t>(),
                    good_readsize as size_t,
                    SPLICE_F_MOVE as ::core::ffi::c_uint,
                ) as ssize_t;
            }
            return nbytes == 0 as ssize_t;
        }
    }
    loop {
        nbytes = safe_read(fd, buffer as *mut ::core::ffi::c_void, bufalloc) as ssize_t;
        if nbytes == 0 {
            break;
        }
        if nbytes < 0 as ssize_t {
            return r#false != 0;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn finalize_input(
    mut fd: ::core::ffi::c_int,
    mut st: *const stat,
    mut ineof: bool,
) {
    if fd == STDIN_FILENO
        && if outleft != 0 {
            (!ineof
                && (seek_failed as ::core::ffi::c_int != 0
                    || lseek(fd, 0 as __off_t, SEEK_END) < 0 as __off_t
                        && *__errno_location() != EINVAL)
                && !drain_input(fd, st)) as ::core::ffi::c_int
        } else {
            (bufoffset != after_last_match
                && !seek_failed
                && lseek(fd, after_last_match, SEEK_SET) < 0 as __off_t)
                as ::core::ffi::c_int
        } != 0
    {
        suppressible_error(*__errno_location());
    }
}
unsafe extern "C" fn grepdesc(mut desc: ::core::ffi::c_int, mut command_line: bool) -> bool {
    let mut count: intmax_t = 0;
    let mut status: bool = r#true != 0;
    let mut ineof: bool = r#false != 0;
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
    if fstat(desc, &raw mut st) != 0 as ::core::ffi::c_int {
        suppressible_error(*__errno_location());
    } else if !(desc != STDIN_FILENO
        && skip_devices(command_line) as ::core::ffi::c_int != 0
        && is_device_mode(st.st_mode) as ::core::ffi::c_int != 0)
    {
        if !(desc != STDIN_FILENO
            && command_line as ::core::ffi::c_int != 0
            && skipped_file(
                filename,
                r#true != 0,
                (st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int,
            ) as ::core::ffi::c_int
                != 0)
        {
            if out_file < 0 as ::core::ffi::c_int {
                out_file = (st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t)
                    as ::core::ffi::c_int;
            }
            if desc != STDIN_FILENO
                && directories.0 == directories_type::RECURSE_DIRECTORIES.0
                && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
            {
                let mut fts: *mut FTS = ::core::ptr::null_mut::<FTS>();
                let mut ent: *mut FTSENT = ::core::ptr::null_mut::<FTSENT>();
                let mut opts: ::core::ffi::c_int = fts_options
                    & !if command_line as ::core::ffi::c_int != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        FTS_COMFOLLOW
                    };
                let mut fts_arg: [*mut ::core::ffi::c_char; 2] =
                    [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
                if close(desc) != 0 as ::core::ffi::c_int {
                    suppressible_error(*__errno_location());
                }
                fts_arg[0usize] = filename as *mut ::core::ffi::c_char;
                fts_arg[1usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
                fts = rpl_fts_open(
                    &raw mut fts_arg as *mut *mut ::core::ffi::c_char,
                    opts,
                    None,
                );
                if fts.is_null() {
                    xalloc_die();
                }
                loop {
                    ent = rpl_fts_read(fts);
                    if ent.is_null() {
                        break;
                    }
                    status = status as ::core::ffi::c_int
                        & grepdirent(fts, ent, command_line) as ::core::ffi::c_int
                        != 0;
                }
                if *__errno_location() != 0 {
                    suppressible_error(*__errno_location());
                }
                if rpl_fts_close(fts) != 0 as ::core::ffi::c_int {
                    suppressible_error(*__errno_location());
                }
                return status;
            }
            if !(desc != STDIN_FILENO
                && (directories.0 == directories_type::SKIP_DIRECTORIES.0
                    && st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t
                    || (devices.0 == C2Rust_Unnamed_8::SKIP_DEVICES.0
                        || devices.0 == C2Rust_Unnamed_8::READ_COMMAND_LINE_DEVICES.0
                            && !command_line)
                        && is_device_mode(st.st_mode) as ::core::ffi::c_int != 0))
            {
                if !out_quiet
                    && list_files.0 == C2Rust_Unnamed_9::LISTFILES_NONE.0
                    && (1 as intmax_t) < max_count
                    && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
                    && st.st_dev ^ out_stat.st_dev | st.st_ino ^ out_stat.st_ino == 0
                {
                    if !suppress_errors {
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"%s: input file is also the output\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                input_filename(),
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
                                        b"%s: input file is also the output\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                    input_filename(),
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                    }
                    errseen = r#true != 0;
                } else {
                    count = grep(desc, &raw mut st, &raw mut ineof);
                    if count_matches {
                        if out_file != 0 {
                            print_filename();
                            if filename_mask != 0 {
                                print_sep(
                                    C2Rust_Unnamed_4::SEP_CHAR_SELECTED.0 as ::core::ffi::c_int
                                        as ::core::ffi::c_char,
                                );
                            } else {
                                putchar_errno(0 as ::core::ffi::c_int);
                            }
                        }
                        printf_errno(b"%ld\n\0".as_ptr() as *const ::core::ffi::c_char, count);
                        if line_buffered {
                            fflush_errno();
                        }
                    }
                    status = count == 0;
                    if list_files.0 == C2Rust_Unnamed_9::LISTFILES_NONE.0 {
                        finalize_input(desc, &raw mut st, ineof);
                    } else if list_files.0
                        == (if status as ::core::ffi::c_int != 0 {
                            C2Rust_Unnamed_9::LISTFILES_NONMATCHING.0 as ::core::ffi::c_int
                        } else {
                            C2Rust_Unnamed_9::LISTFILES_MATCHING.0 as ::core::ffi::c_int
                        }) as ::core::ffi::c_uint
                    {
                        print_filename();
                        putchar_errno('\n' as ::core::ffi::c_int & filename_mask);
                        if line_buffered {
                            fflush_errno();
                        }
                    }
                }
            }
        }
    }
    if desc != STDIN_FILENO && close(desc) != 0 as ::core::ffi::c_int {
        suppressible_error(*__errno_location());
    }
    return status;
}
unsafe extern "C" fn grep_command_line_arg(mut arg: *const ::core::ffi::c_char) -> bool {
    if strcmp(arg, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        filename = label;
        if binary {
            xset_binary_mode(STDIN_FILENO, O_BINARY);
        }
        return grepdesc(STDIN_FILENO, r#true != 0);
    } else {
        filename = arg;
        return grepfile(AT_FDCWD, arg, r#true != 0, r#true != 0);
    };
}
#[export_name = "rboxc_grep_usage"]
pub unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) {
    if status != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Usage: %s [OPTION]... PATTERNS [FILE]...\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            getprogname(),
        );
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            getprogname(),
        );
    } else {
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Usage: %s [OPTION]... PATTERNS [FILE]...\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            getprogname(),
        );
        printf(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Search for PATTERNS in each FILE.\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Example: %s -i 'hello world' menu.h main.c\nPATTERNS can contain multiple patterns separated by newlines.\n\nPattern selection and interpretation:\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            getprogname(),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"  -E, --extended-regexp     PATTERNS are extended regular expressions\n  -F, --fixed-strings       PATTERNS are strings\n  -G, --basic-regexp        PATTERNS are basic regular expressions\n  -P, --perl-regexp         PATTERNS are Perl regular expressions\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"  -e, --regexp=PATTERNS     use PATTERNS for matching\n  -f, --file=FILE           take PATTERNS from FILE\n  -i, --ignore-case         ignore case distinctions in patterns and data\n      --no-ignore-case      do not ignore case distinctions (default)\n  -w, --word-regexp         match only whole words\n  -x, --line-regexp         match only whole lines\n  -z, --null-data           a data line ends in 0 byte, not newline\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"\nMiscellaneous:\n  -s, --no-messages         suppress error messages\n  -v, --invert-match        select non-matching lines\n  -V, --version             display version information and exit\n      --help                display this help text and exit\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"\nOutput control:\n  -m, --max-count=NUM       stop after NUM selected lines\n  -b, --byte-offset         print the byte offset with output lines\n  -n, --line-number         print line number with output lines\n      --line-buffered       flush output on every line\n  -H, --with-filename       print file name with output lines\n  -h, --no-filename         suppress the file name prefix on output\n      --label=LABEL         use LABEL as the standard input file name prefix\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"  -o, --only-matching       show only nonempty parts of lines that match\n  -q, --quiet, --silent     suppress all normal output\n      --binary-files=TYPE   assume that binary files are TYPE;\n                            TYPE is 'binary', 'text', or 'without-match'\n  -a, --text                equivalent to --binary-files=text\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"  -I                        equivalent to --binary-files=without-match\n  -d, --directories=ACTION  how to handle directories;\n                            ACTION is 'read', 'recurse', or 'skip'\n  -D, --devices=ACTION      how to handle devices, FIFOs and sockets;\n                            ACTION is 'read' or 'skip'\n  -r, --recursive           like --directories=recurse\n  -R, --dereference-recursive  likewise, but follow all symlinks\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"      --include=GLOB        search only files that match GLOB (a file pattern)\n      --exclude=GLOB        skip files that match GLOB\n      --exclude-from=FILE   skip files that match any file pattern from FILE\n      --exclude-dir=GLOB    skip directories that match GLOB\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"  -L, --files-without-match  print only names of FILEs with no selected lines\n  -l, --files-with-matches  print only names of FILEs with selected lines\n  -c, --count               print only a count of selected lines per FILE\n  -T, --initial-tab         make tabs line up (if needed)\n  -Z, --null                print 0 byte after FILE name\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"\nContext control:\n  -B, --before-context=NUM  print NUM lines of leading context\n  -A, --after-context=NUM   print NUM lines of trailing context\n  -C, --context=NUM         print NUM lines of output context\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"  -NUM                      same as --context=NUM\n      --group-separator=SEP  print SEP on line between matches with context\n      --no-group-separator  do not print separator for matches with context\n      --color[=WHEN],\n      --colour[=WHEN]       use markers to highlight the matching strings;\n                            WHEN is 'always', 'never', or 'auto'\n  -U, --binary              do not strip CR characters at EOL (MSDOS/Windows)\n\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"When FILE is '-', read standard input.  If no FILE is given, read standard\ninput, but with -r, recursively search the working directory instead.  With\nfewer than two FILEs, assume -h.  Exit status is 0 if any line is selected,\n1 otherwise; if any error occurs and -q is not given, the exit status is 2.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        emit_bug_reporting_address();
    }
    exit(status);
}
static mut matchers: [C2Rust_Unnamed_16; 7] = unsafe {
    [
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"grep\0\0\0\0\0\0\0\0",
            ),
            syntax: RE_SYNTAX_GREP as ::core::ffi::c_int,
            compile: Some(
                GEAcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                EGexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"egrep\0\0\0\0\0\0\0",
            ),
            syntax: RE_SYNTAX_EGREP as ::core::ffi::c_int,
            compile: Some(
                GEAcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                EGexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"fgrep\0\0\0\0\0\0\0",
            ),
            syntax: 0 as ::core::ffi::c_int,
            compile: Some(
                Fcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                Fexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"awk\0\0\0\0\0\0\0\0\0",
            ),
            syntax: RE_SYNTAX_AWK as ::core::ffi::c_int,
            compile: Some(
                GEAcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                EGexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"gawk\0\0\0\0\0\0\0\0",
            ),
            syntax: RE_SYNTAX_GNU_AWK as ::core::ffi::c_int,
            compile: Some(
                GEAcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                EGexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"posixawk\0\0\0\0",
            ),
            syntax: RE_SYNTAX_POSIX_AWK as ::core::ffi::c_int,
            compile: Some(
                GEAcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                EGexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
        C2Rust_Unnamed_16 {
            name: ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(
                *b"perl\0\0\0\0\0\0\0\0",
            ),
            syntax: 0 as ::core::ffi::c_int,
            compile: Some(
                Pcompile
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_char,
                        idx_t,
                        reg_syntax_t,
                        bool,
                    ) -> *mut ::core::ffi::c_void,
            ),
            execute: Some(
                Pexecute
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                        idx_t,
                        *mut idx_t,
                        *const ::core::ffi::c_char,
                    ) -> ptrdiff_t,
            ),
        },
    ]
};
unsafe extern "C" fn setmatcher(
    mut m: *const ::core::ffi::c_char,
    mut matcher: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (i as usize)
        < ::core::mem::size_of::<[C2Rust_Unnamed_16; 7]>()
            .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_16>())
    {
        if strcmp(
            m,
            &raw const (*(&raw const matchers as *const C2Rust_Unnamed_16).offset(i as isize)).name
                as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            if 0 as ::core::ffi::c_int <= matcher && matcher != i {
                if ::core::mem::size_of::<C2Rust_Unnamed_19>() != 0 {
                    if 0 != 0 {
                        error(
                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"conflicting matchers specified\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                        );
                        if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int =
                                C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"conflicting matchers specified\0".as_ptr()
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
                    if false {
                    } else {
                        unreachable!();
                    };
                } else {
                    if 0 != 0 {
                        error(
                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"conflicting matchers specified\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                        );
                        if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                        {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int =
                                C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"conflicting matchers specified\0".as_ptr()
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
                    if false {
                    } else {
                        unreachable!();
                    };
                };
            }
            return i;
        }
        i += 1;
    }
    if ::core::mem::size_of::<C2Rust_Unnamed_18>() != 0 {
        if 0 != 0 {
            error(
                C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"invalid matcher %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                m,
            );
            if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int =
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"invalid matcher %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    m,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        if false {
        } else {
            unreachable!();
        };
    } else {
        if 0 != 0 {
            error(
                C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"invalid matcher %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                m,
            );
            if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int =
                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"invalid matcher %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    m,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        if false {
        } else {
            unreachable!();
        };
    };
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn get_nondigit_option(
    mut argc: ::core::ffi::c_int,
    mut argv: *const *mut ::core::ffi::c_char,
    mut default_context: *mut intmax_t,
) -> ::core::ffi::c_int {
    static mut prev_digit_optind: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut this_digit_optind: ::core::ffi::c_int = 0;
    let mut was_digit: bool = false;
    let mut buf: [::core::ffi::c_char; 25] = [0; 25];
    let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    let mut opt: ::core::ffi::c_int = 0;
    was_digit = r#false != 0;
    this_digit_optind = optind;
    loop {
        opt = getopt_long(
            argc,
            argv as *mut *mut ::core::ffi::c_char,
            &raw const short_options as *const ::core::ffi::c_char,
            &raw const long_options as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if !c_isdigit(opt) {
            break;
        }
        if prev_digit_optind != this_digit_optind || !was_digit {
            p = &raw mut buf as *mut ::core::ffi::c_char;
        } else {
            p = p.offset(
                -((buf[0usize] as ::core::ffi::c_int == '0' as ::core::ffi::c_int)
                    as ::core::ffi::c_int as isize),
            );
        }
        if p == (&raw mut buf as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<[::core::ffi::c_char; 25]>() as isize)
            .offset(-(4 as ::core::ffi::c_int as isize))
        {
            strcpy(p, b"...\0".as_ptr() as *const ::core::ffi::c_char);
            p = p.offset(3 as ::core::ffi::c_int as isize);
            break;
        } else {
            let c2rust_fresh9 = p;
            p = p.offset(1);
            *c2rust_fresh9 = opt as ::core::ffi::c_char;
            was_digit = r#true != 0;
            prev_digit_optind = this_digit_optind;
            this_digit_optind = optind;
        }
    }
    if p != &raw mut buf as *mut ::core::ffi::c_char {
        *p = '\0' as ::core::ffi::c_char;
        context_length_arg(&raw mut buf as *mut ::core::ffi::c_char, default_context);
    }
    return opt;
}
unsafe extern "C" fn parse_grep_colors() {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut q: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    p = getenv(b"GREP_COLORS\0".as_ptr() as *const ::core::ffi::c_char);
    if p.is_null() || *p as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        return;
    }
    q = xstrdup(p);
    name = q;
    val = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        if *q as ::core::ffi::c_int == ':' as ::core::ffi::c_int
            || *q as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            let mut c: ::core::ffi::c_char = *q;
            let mut cap: *const color_cap = ::core::ptr::null::<color_cap>();
            let c2rust_fresh10 = q;
            q = q.offset(1);
            *c2rust_fresh10 = '\0' as ::core::ffi::c_char;
            cap = &raw const color_dict as *const color_cap;
            while !(*cap).name.is_null() {
                if strcmp((*cap).name, name) == 0 as ::core::ffi::c_int {
                    break;
                }
                cap = cap.offset(1);
            }
            if !(*cap).var.is_null() && !val.is_null() {
                *(*cap).var = val;
            }
            if (*cap).fct.is_some() {
                (*cap).fct.expect("non-null function pointer")();
            }
            if c as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                return;
            }
            name = q;
            val = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else if *q as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
            if q == name || !val.is_null() {
                return;
            }
            let c2rust_fresh11 = q;
            q = q.offset(1);
            *c2rust_fresh11 = '\0' as ::core::ffi::c_char;
            val = q;
        } else if val.is_null() {
            q = q.offset(1);
        } else if *q as ::core::ffi::c_int == ';' as ::core::ffi::c_int
            || c_isdigit(*q as ::core::ffi::c_int) as ::core::ffi::c_int != 0
        {
            q = q.offset(1);
        } else {
            return;
        }
    }
}
unsafe extern "C" fn contains_encoding_error(
    mut pat: *const ::core::ffi::c_char,
    mut patlen: idx_t,
) -> bool {
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2Rust_Unnamed { __wch: 0 },
    };
    mbszero(&raw mut mbs);
    let mut charlen: ptrdiff_t = 0;
    let mut i: idx_t = 0 as idx_t;
    while i < patlen {
        charlen = mb_clen(pat.offset(i as isize), patlen - i, &raw mut mbs);
        if charlen < 0 as ptrdiff_t {
            return r#true != 0;
        }
        i += charlen;
    }
    return r#false != 0;
}
static mut ok_fold: [::core::ffi::c_schar; 256] = [0; 256];
unsafe extern "C" fn setup_ok_fold() {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < C2Rust_Unnamed_2::NCHAR.0 as ::core::ffi::c_int {
        let mut wi: wint_t = localeinfo.sbctowc[i as usize];
        if wi != WEOF {
            let mut ok: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            let mut folded: [char32_t; 32] = [0; 32];
            let mut n: ::core::ffi::c_int =
                case_folded_counterparts(wi, &raw mut folded as *mut char32_t);
            loop {
                n -= 1;
                if 0 as ::core::ffi::c_int > n {
                    break;
                }
                let mut buf: [::core::ffi::c_char; 16] = [0; 16];
                let mut s: mbstate_t = mbstate_t {
                    __count: 0,
                    __value: C2Rust_Unnamed { __wch: 0 },
                };
                mbszero(&raw mut s);
                if c32rtomb(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    folded[n as usize],
                    &raw mut s,
                ) == 1 as size_t
                {
                    continue;
                }
                ok = -1 as ::core::ffi::c_int;
                break;
            }
            ok_fold[i as usize] = ok as ::core::ffi::c_schar;
        }
        i += 1;
    }
}
unsafe extern "C" fn fgrep_icase_charlen(
    mut pat: *const ::core::ffi::c_char,
    mut patlen: idx_t,
    mut mbs: *mut mbstate_t,
) -> ptrdiff_t {
    let mut pat0: ::core::ffi::c_uchar = *pat.offset(0isize) as ::core::ffi::c_uchar;
    if localeinfo.sbctowc[pat0 as usize] != WEOF {
        return ok_fold[pat0 as usize] as ptrdiff_t;
    }
    let mut wc: char32_t = 0;
    let mut wn: size_t = rpl_mbrtoc32(&raw mut wc, pat, patlen as size_t, mbs);
    if (MB_LEN_MAX as size_t) < wn {
        return -1 as ptrdiff_t;
    }
    let mut folded: [char32_t; 32] = [0; 32];
    if case_folded_counterparts(wc as wint_t, &raw mut folded as *mut char32_t) != 0 {
        return -1 as ptrdiff_t;
    }
    let mut i: idx_t = wn as idx_t;
    loop {
        i -= 1;
        if (0 as idx_t) >= i {
            break;
        }
        let mut c: ::core::ffi::c_uchar = *pat.offset(i as isize) as ::core::ffi::c_uchar;
        if ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_uchar>() > 1usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = c as ::core::ffi::c_int;
                    __res = (if __c < -128 as ::core::ffi::c_int || __c > 255 as ::core::ffi::c_int
                    {
                        __c as __int32_t
                    } else {
                        *(*__ctype_toupper_loc()).offset(__c as isize)
                    }) as ::core::ffi::c_int;
                } else {
                    __res = toupper(c as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc()).offset(c as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        }) != c as ::core::ffi::c_int
        {
            return -1 as ptrdiff_t;
        }
    }
    return wn as ptrdiff_t;
}
unsafe extern "C" fn fgrep_icase_available(
    mut pat: *const ::core::ffi::c_char,
    mut patlen: idx_t,
) -> bool {
    let mut mbs: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2Rust_Unnamed { __wch: 0 },
    };
    mbszero(&raw mut mbs);
    let mut i: idx_t = 0 as idx_t;
    while i < patlen {
        let mut n: ::core::ffi::c_int =
            fgrep_icase_charlen(pat.offset(i as isize), patlen - i, &raw mut mbs)
                as ::core::ffi::c_int;
        if n < 0 as ::core::ffi::c_int {
            return r#false != 0;
        }
        i += n as idx_t;
    }
    return r#true != 0;
}
#[export_name = "rboxc_grep_fgrep_to_grep_pattern"]
pub unsafe extern "C" fn fgrep_to_grep_pattern(
    mut keys_p: *mut *mut ::core::ffi::c_char,
    mut len_p: *mut idx_t,
) {
    let mut len: idx_t = *len_p;
    let mut keys: *mut ::core::ffi::c_char = *keys_p;
    let mut mb_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2Rust_Unnamed { __wch: 0 },
    };
    mbszero(&raw mut mb_state);
    let mut new_keys: *mut ::core::ffi::c_char =
        xnmalloc((len + 1 as idx_t) as size_t, 2 as size_t) as *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = new_keys;
    let mut n: ptrdiff_t = 0;
    while len != 0 {
        n = mb_clen(keys, len, &raw mut mb_state);
        's_84: {
            'c_12247: {
                match n {
                    -2 => {
                        n = len as ptrdiff_t;
                        break 'c_12247;
                    }
                    -1 => {
                        memset(
                            &raw mut mb_state as *mut ::core::ffi::c_void,
                            0 as ::core::ffi::c_int,
                            ::core::mem::size_of::<mbstate_t>(),
                        );
                        n = 1 as ptrdiff_t;
                    }
                    1 => {}
                    _ => {
                        break 'c_12247;
                    }
                }
                match *keys as ::core::ffi::c_int {
                    36 | 42 | 46 | 91 | 92 | 94 => {
                        let c2rust_fresh3 = p;
                        p = p.offset(1);
                        *c2rust_fresh3 = '\\' as ::core::ffi::c_char;
                    }
                    _ => {}
                }
                let c2rust_fresh4 = p;
                p = p.offset(1);
                *c2rust_fresh4 = *keys;
                break 's_84;
            }
            p = mempcpy(
                p as *mut ::core::ffi::c_void,
                keys as *const ::core::ffi::c_void,
                n as size_t,
            ) as *mut ::core::ffi::c_char;
        }
        keys = keys.offset(n as isize);
        len -= n;
    }
    *p = '\n' as ::core::ffi::c_char;
    free(*keys_p as *mut ::core::ffi::c_void);
    *keys_p = new_keys;
    *len_p = p.offset_from(new_keys) as idx_t;
}
unsafe extern "C" fn try_fgrep_pattern(
    mut matcher: ::core::ffi::c_int,
    mut keys: *mut ::core::ffi::c_char,
    mut len_p: *mut idx_t,
) -> ::core::ffi::c_int {
    let mut result: ::core::ffi::c_int = matcher;
    let mut len: idx_t = *len_p;
    let mut new_keys: *mut ::core::ffi::c_char =
        ximalloc(len + 1 as idx_t) as *mut ::core::ffi::c_char;
    let mut p: *mut ::core::ffi::c_char = new_keys;
    let mut q: *const ::core::ffi::c_char = keys;
    let mut mb_state: mbstate_t = mbstate_t {
        __count: 0,
        __value: C2Rust_Unnamed { __wch: 0 },
    };
    mbszero(&raw mut mb_state);
    '_fail: {
        while len != 0 as idx_t {
            match *q as ::core::ffi::c_int {
                36 | 42 | 46 | 91 | 94 => {
                    break '_fail;
                }
                40 | 43 | 63 | 123 | 124 => {
                    if matcher != C2Rust_Unnamed_17::G_MATCHER_INDEX.0 as ::core::ffi::c_int {
                        break '_fail;
                    }
                }
                92 => {
                    if (1 as idx_t) < len {
                        match *q.offset(1isize) as ::core::ffi::c_int {
                            10 | 66 | 83 | 87 | 39 | 60 | 98 | 115 | 119 | 96 | 62 | 49 | 50
                            | 51 | 52 | 53 | 54 | 55 | 56 | 57 => {
                                break '_fail;
                            }
                            40 | 43 | 63 | 123 | 124 | 41 => {
                                if matcher
                                    == C2Rust_Unnamed_17::G_MATCHER_INDEX.0 as ::core::ffi::c_int
                                {
                                    break '_fail;
                                }
                            }
                            _ => {}
                        }
                        q = q.offset(1);
                        len -= 1;
                    }
                }
                _ => {}
            }
            let mut clen: ptrdiff_t = if match_icase as ::core::ffi::c_int != 0 {
                fgrep_icase_charlen(q, len, &raw mut mb_state)
            } else {
                mb_clen(q, len, &raw mut mb_state)
            };
            if clen < 0 as ptrdiff_t {
                break '_fail;
            }
            p = mempcpy(
                p as *mut ::core::ffi::c_void,
                q as *const ::core::ffi::c_void,
                clen as size_t,
            ) as *mut ::core::ffi::c_char;
            q = q.offset(clen as isize);
            len -= clen;
        }
        if *len_p != p.offset_from(new_keys) {
            *len_p = p.offset_from(new_keys) as idx_t;
            let mut keys_end: *mut ::core::ffi::c_char = mempcpy(
                keys as *mut ::core::ffi::c_void,
                new_keys as *const ::core::ffi::c_void,
                p.offset_from(new_keys) as size_t,
            ) as *mut ::core::ffi::c_char;
            *keys_end = '\n' as ::core::ffi::c_char;
        }
        result = C2Rust_Unnamed_17::F_MATCHER_INDEX.0 as ::core::ffi::c_int;
    }
    free(new_keys as *mut ::core::ffi::c_void);
    return result;
}
// SPDX-License-Identifier: GPL-3.0-or-later
// GNU grep uses glibc invocation names instead of Gnulib set_program_name.
extern "C" {
    static mut program_invocation_name: *mut ::core::ffi::c_char;
    static mut program_invocation_short_name: *mut ::core::ffi::c_char;
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut rboxc_grep_stderr: *mut libc::FILE;
}
unsafe extern "C" fn rboxc_grep_error_prefix() {
    libc::fprintf(rboxc_grep_stderr, b"%s: \0".as_ptr().cast(), program_invocation_name);
}
unsafe fn rboxc_grep_set_invocation(name: *mut ::core::ffi::c_char) {
    program_invocation_name = name;
    let slash = libc::strrchr(name, b'/' as ::core::ffi::c_int);
    program_invocation_short_name = if slash.is_null() { name } else { slash.add(1) };
    let prior = error_print_progname;
    if prior.is_none() {
        error_print_progname = Some(rboxc_grep_error_prefix);
    }
}

#[no_mangle]
pub unsafe extern "C" fn single_binary_main_grep(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rboxc_grep_set_invocation(*argv);
    let mut keys: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut keycc: idx_t = 0 as idx_t;
    let mut keyalloc: idx_t = 0 as idx_t;
    let mut matcher: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
    let mut opt: ::core::ffi::c_int = 0;
    let mut prev_optind: ::core::ffi::c_int = 0;
    let mut last_recursive: ::core::ffi::c_int = 0;
    let mut default_context: intmax_t = 0;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    ::core::ptr::write_volatile(
        &raw mut exit_failure,
        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
    );
    let mut filename_option: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    eolbyte = '\n' as ::core::ffi::c_char;
    filename_mask = !(0 as ::core::ffi::c_int);
    max_count = INTMAX_MAX as intmax_t;
    out_before = -1 as intmax_t;
    out_after = out_before;
    default_context = -1 as intmax_t;
    only_matching = r#false != 0;
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    bindtextdomain(
        b"gnulib\0".as_ptr() as *const ::core::ffi::c_char,
        GNULIB_LOCALEDIR.as_ptr(),
    );
    textdomain(PACKAGE.as_ptr());
    init_localeinfo(&raw mut localeinfo);
    atexit(Some(clean_up_stdout as unsafe extern "C" fn() -> ()));
    c_stack_action(None);
    last_recursive = 0 as ::core::ffi::c_int;
    pattern_table = hash_initialize(
        0 as size_t,
        ::core::ptr::null::<Hash_tuning>(),
        Some(hash_pattern as unsafe extern "C" fn(*const ::core::ffi::c_void, size_t) -> size_t),
        Some(
            compare_patterns
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> bool,
        ),
        None,
    );
    if pattern_table.is_null() {
        xalloc_die();
    }
    loop {
        prev_optind = optind;
        opt = get_nondigit_option(argc, argv, &raw mut default_context);
        if opt == -1 as ::core::ffi::c_int {
            break;
        }
        's_743: {
            match opt {
                65 => {
                    context_length_arg(optarg, &raw mut out_after);
                    break 's_743;
                }
                66 => {
                    context_length_arg(optarg, &raw mut out_before);
                    break 's_743;
                }
                67 => {
                    context_length_arg(optarg, &raw mut default_context);
                    break 's_743;
                }
                68 => {
                    if strcmp(optarg, b"read\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        devices = C2Rust_Unnamed_8::READ_DEVICES;
                    } else if strcmp(optarg, b"skip\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        devices = C2Rust_Unnamed_8::SKIP_DEVICES;
                    } else {
                        if ::core::mem::size_of::<C2Rust_Unnamed_25>() != 0 {
                            if 0 != 0 {
                                error(
                                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"unknown devices method\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                );
                                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int =
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"unknown devices method\0".as_ptr()
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
                            if false {
                            } else {
                                unreachable!();
                            };
                        } else {
                            if 0 != 0 {
                                error(
                                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"unknown devices method\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                );
                                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int =
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"unknown devices method\0".as_ptr()
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
                            if false {
                            } else {
                                unreachable!();
                            };
                        };
                    }
                    break 's_743;
                }
                69 => {
                    matcher =
                        setmatcher(b"egrep\0".as_ptr() as *const ::core::ffi::c_char, matcher);
                    break 's_743;
                }
                70 => {
                    matcher =
                        setmatcher(b"fgrep\0".as_ptr() as *const ::core::ffi::c_char, matcher);
                    break 's_743;
                }
                80 => {
                    matcher = setmatcher(b"perl\0".as_ptr() as *const ::core::ffi::c_char, matcher);
                    break 's_743;
                }
                71 => {
                    matcher = setmatcher(b"grep\0".as_ptr() as *const ::core::ffi::c_char, matcher);
                    break 's_743;
                }
                88 => {
                    matcher = setmatcher(optarg, matcher);
                    break 's_743;
                }
                72 => {
                    filename_option = 1 as ::core::ffi::c_int;
                    break 's_743;
                }
                73 => {
                    binary_files = C2Rust_Unnamed_10::WITHOUT_MATCH_BINARY_FILES;
                    break 's_743;
                }
                84 => {
                    align_tabs = r#true != 0;
                    break 's_743;
                }
                86 => {
                    show_version = r#true != 0;
                    break 's_743;
                }
                97 => {
                    binary_files = C2Rust_Unnamed_10::TEXT_BINARY_FILES;
                    break 's_743;
                }
                98 => {
                    out_byte = r#true != 0;
                    break 's_743;
                }
                99 => {
                    count_matches = r#true != 0;
                    break 's_743;
                }
                100 => {
                    directories = directories_types[__xargmatch_internal(
                        b"--directories\0".as_ptr() as *const ::core::ffi::c_char,
                        optarg,
                        &raw const directories_args as *const *const ::core::ffi::c_char,
                        &raw const directories_types as *const directories_type
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<directories_type>(),
                        argmatch_die,
                        r#true != 0,
                    ) as usize];
                    if directories.0 == directories_type::RECURSE_DIRECTORIES.0 {
                        last_recursive = prev_optind;
                    }
                    break 's_743;
                }
                101 => {
                    let mut cc: idx_t = strlen(optarg) as idx_t;
                    let mut shortage: ptrdiff_t = keycc - keyalloc + cc + 1 as ptrdiff_t;
                    if (0 as ptrdiff_t) < shortage {
                        keys = xpalloc(
                            keys as *mut ::core::ffi::c_void,
                            &raw mut keyalloc,
                            shortage,
                            -1 as ptrdiff_t,
                            1 as idx_t,
                        ) as *mut ::core::ffi::c_char;
                        pattern_array = keys;
                    }
                    let mut keyend: *mut ::core::ffi::c_char = mempcpy(
                        keys.offset(keycc as isize) as *mut ::core::ffi::c_void,
                        optarg as *const ::core::ffi::c_void,
                        cc as size_t,
                    )
                        as *mut ::core::ffi::c_char;
                    *keyend = '\n' as ::core::ffi::c_char;
                    keycc = update_patterns(
                        keys,
                        keycc,
                        keycc + cc + 1 as idx_t,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    break 's_743;
                }
                102 => {
                    if strcmp(optarg, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        if binary {
                            xset_binary_mode(STDIN_FILENO, O_BINARY);
                        }
                        fp = stdin;
                    } else {
                        fp = fopen(
                            optarg,
                            if binary as ::core::ffi::c_int != 0 {
                                b"rb\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"r\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        ) as *mut FILE;
                        if fp.is_null() {
                            if ::core::mem::size_of::<C2Rust_Unnamed_24>() != 0 {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        *__errno_location(),
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        optarg,
                                    );
                                    if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            *__errno_location(),
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            optarg,
                                        );
                                        if __errstatus != 0 as ::core::ffi::c_int {
                                            unreachable!();
                                        } else {
                                        };
                                    });
                                };
                                if false {
                                } else {
                                    unreachable!();
                                };
                            } else {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        *__errno_location(),
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        optarg,
                                    );
                                    if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            *__errno_location(),
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            optarg,
                                        );
                                        if __errstatus != 0 as ::core::ffi::c_int {
                                            unreachable!();
                                        } else {
                                        };
                                    });
                                };
                                if false {
                                } else {
                                    unreachable!();
                                };
                            };
                        }
                    }
                    let mut newkeycc: idx_t = keycc;
                    let mut cc_0: idx_t = 0;
                    loop {
                        let mut shortage_0: ptrdiff_t = newkeycc - keyalloc + 2 as ptrdiff_t;
                        if (0 as ptrdiff_t) < shortage_0 {
                            keys = xpalloc(
                                keys as *mut ::core::ffi::c_void,
                                &raw mut keyalloc,
                                shortage_0,
                                -1 as ptrdiff_t,
                                1 as idx_t,
                            ) as *mut ::core::ffi::c_char;
                            pattern_array = keys;
                        }
                        cc_0 = (if 0 != 0
                            && 0 != 0
                            && (1 as ::core::ffi::c_int as size_t)
                                .wrapping_mul((keyalloc - (newkeycc + 1 as idx_t)) as size_t)
                                <= 8 as size_t
                            && 1 as ::core::ffi::c_int as size_t != 0 as size_t
                        {
                            ({
                                let mut __ptr: *mut ::core::ffi::c_char =
                                    keys.offset(newkeycc as isize);
                                let mut __stream: *mut FILE = fp;
                                let mut __cnt: size_t = 0;
                                __cnt = (1 as ::core::ffi::c_int as size_t)
                                    .wrapping_mul((keyalloc - (newkeycc + 1 as idx_t)) as size_t);
                                while __cnt > 0 as size_t {
                                    let mut __c: ::core::ffi::c_int = getc_unlocked(__stream);
                                    if __c == EOF {
                                        break;
                                    }
                                    let c2rust_fresh12 = __ptr;
                                    __ptr = __ptr.offset(1);
                                    *c2rust_fresh12 = __c as ::core::ffi::c_char;
                                    __cnt = __cnt.wrapping_sub(1);
                                }
                                (1 as ::core::ffi::c_int as size_t)
                                    .wrapping_mul((keyalloc - (newkeycc + 1 as idx_t)) as size_t)
                                    .wrapping_sub(__cnt)
                                    .wrapping_div(1 as ::core::ffi::c_int as size_t)
                            })
                        } else if 0 != 0 && 1 as ::core::ffi::c_int as size_t == 0 as size_t
                            || 0 != 0
                                && (keyalloc - (newkeycc + 1 as idx_t)) as size_t == 0 as size_t
                        {
                            0 as ::core::ffi::c_int as size_t
                        } else {
                            fread_unlocked(
                                keys.offset(newkeycc as isize) as *mut ::core::ffi::c_void,
                                1 as size_t,
                                (keyalloc - (newkeycc + 1 as idx_t)) as size_t,
                                fp,
                            )
                        }) as idx_t;
                        if cc_0 == 0 as idx_t {
                            break;
                        }
                        newkeycc += cc_0;
                    }
                    let mut err: ::core::ffi::c_int = *__errno_location();
                    if ferror_unlocked(fp) == 0 {
                        err = 0 as ::core::ffi::c_int;
                        if fp == stdin {
                            clearerr_unlocked(fp);
                        } else if fclose(fp) != 0 as ::core::ffi::c_int {
                            err = *__errno_location();
                        }
                    }
                    if err != 0 {
                        if ::core::mem::size_of::<C2Rust_Unnamed_23>() != 0 {
                            if 0 != 0 {
                                error(
                                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                    err,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    optarg,
                                );
                                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int =
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        err,
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        optarg,
                                    );
                                    if __errstatus != 0 as ::core::ffi::c_int {
                                        unreachable!();
                                    } else {
                                    };
                                });
                            };
                            if false {
                            } else {
                                unreachable!();
                            };
                        } else {
                            if 0 != 0 {
                                error(
                                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                    err,
                                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                    optarg,
                                );
                                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int =
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        err,
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        optarg,
                                    );
                                    if __errstatus != 0 as ::core::ffi::c_int {
                                        unreachable!();
                                    } else {
                                    };
                                });
                            };
                            if false {
                            } else {
                                unreachable!();
                            };
                        };
                    }
                    if newkeycc != keycc
                        && *keys.offset((newkeycc - 1 as idx_t) as isize) as ::core::ffi::c_int
                            != '\n' as ::core::ffi::c_int
                    {
                        let c2rust_fresh13 = newkeycc;
                        newkeycc += 1;
                        *keys.offset(c2rust_fresh13 as isize) = '\n' as ::core::ffi::c_char;
                    }
                    keycc = update_patterns(keys, keycc, newkeycc, optarg);
                    break 's_743;
                }
                104 => {
                    filename_option = -1 as ::core::ffi::c_int;
                    break 's_743;
                }
                105 | 121 => {
                    match_icase = r#true != 0;
                    break 's_743;
                }
                137 => {
                    match_icase = r#false != 0;
                    break 's_743;
                }
                76 => {
                    list_files = C2Rust_Unnamed_9::LISTFILES_NONMATCHING;
                    break 's_743;
                }
                108 => {
                    list_files = C2Rust_Unnamed_9::LISTFILES_MATCHING;
                    break 's_743;
                }
                109 => {
                    match xstrtoimax(
                        optarg,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                        &raw mut max_count,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    ) {
                        strtol_error::LONGINT_OK | strtol_error::LONGINT_OVERFLOW => {}
                        _ => {
                            if ::core::mem::size_of::<C2Rust_Unnamed_22>() != 0 {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"invalid max count\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            5 as ::core::ffi::c_int,
                                        ),
                                    );
                                    if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            0 as ::core::ffi::c_int,
                                            dcgettext(
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                b"invalid max count\0".as_ptr()
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
                                if false {
                                } else {
                                    unreachable!();
                                };
                            } else {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"invalid max count\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            5 as ::core::ffi::c_int,
                                        ),
                                    );
                                    if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            0 as ::core::ffi::c_int,
                                            dcgettext(
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                b"invalid max count\0".as_ptr()
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
                                if false {
                                } else {
                                    unreachable!();
                                };
                            };
                        }
                    }
                    break 's_743;
                }
                110 => {
                    out_line = r#true != 0;
                    break 's_743;
                }
                111 => {
                    only_matching = r#true != 0;
                    break 's_743;
                }
                113 => {
                    exit_on_match = r#true != 0;
                    break 's_743;
                }
                82 => {
                    fts_options =
                        C2Rust_Unnamed_7::basic_fts_options.0 as ::core::ffi::c_int | FTS_LOGICAL;
                }
                114 => {}
                115 => {
                    suppress_errors = r#true != 0;
                    break 's_743;
                }
                118 => {
                    out_invert = r#true != 0;
                    break 's_743;
                }
                119 => {
                    wordinit();
                    match_words = r#true != 0;
                    break 's_743;
                }
                120 => {
                    match_lines = r#true != 0;
                    break 's_743;
                }
                90 => {
                    filename_mask = 0 as ::core::ffi::c_int;
                    break 's_743;
                }
                122 => {
                    eolbyte = '\0' as ::core::ffi::c_char;
                    break 's_743;
                }
                128 => {
                    if strcmp(optarg, b"binary\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        binary_files = C2Rust_Unnamed_10::BINARY_BINARY_FILES;
                    } else if strcmp(optarg, b"text\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                    {
                        binary_files = C2Rust_Unnamed_10::TEXT_BINARY_FILES;
                    } else if strcmp(
                        optarg,
                        b"without-match\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0 as ::core::ffi::c_int
                    {
                        binary_files = C2Rust_Unnamed_10::WITHOUT_MATCH_BINARY_FILES;
                    } else {
                        if ::core::mem::size_of::<C2Rust_Unnamed_21>() != 0 {
                            if 0 != 0 {
                                error(
                                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"unknown binary-files type\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                );
                                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int =
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"unknown binary-files type\0".as_ptr()
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
                            if false {
                            } else {
                                unreachable!();
                            };
                        } else {
                            if 0 != 0 {
                                error(
                                    C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"unknown binary-files type\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                );
                                if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                                {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int =
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"unknown binary-files type\0".as_ptr()
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
                            if false {
                            } else {
                                unreachable!();
                            };
                        };
                    }
                    break 's_743;
                }
                129 => {
                    if !optarg.is_null() {
                        if c_strcasecmp(optarg, b"always\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0
                            || c_strcasecmp(optarg, b"yes\0".as_ptr() as *const ::core::ffi::c_char)
                                == 0
                            || c_strcasecmp(
                                optarg,
                                b"force\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0
                        {
                            color_option = 1 as ::core::ffi::c_int;
                        } else if c_strcasecmp(
                            optarg,
                            b"never\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0
                            || c_strcasecmp(optarg, b"no\0".as_ptr() as *const ::core::ffi::c_char)
                                == 0
                            || c_strcasecmp(
                                optarg,
                                b"none\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0
                        {
                            color_option = 0 as ::core::ffi::c_int;
                        } else if c_strcasecmp(
                            optarg,
                            b"auto\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0
                            || c_strcasecmp(optarg, b"tty\0".as_ptr() as *const ::core::ffi::c_char)
                                == 0
                            || c_strcasecmp(
                                optarg,
                                b"if-tty\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0
                        {
                            color_option = 2 as ::core::ffi::c_int;
                        } else {
                            show_help = 1 as ::core::ffi::c_int;
                        }
                    } else {
                        color_option = 2 as ::core::ffi::c_int;
                    }
                    break 's_743;
                }
                131 | 134 => {
                    let mut cmd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while cmd < 2 as ::core::ffi::c_int {
                        if excluded_patterns[cmd as usize].is_null() {
                            excluded_patterns[cmd as usize] = new_exclude();
                        }
                        add_exclude(
                            excluded_patterns[cmd as usize],
                            optarg,
                            if opt == C2Rust_Unnamed_6::INCLUDE_OPTION.0 as ::core::ffi::c_int {
                                EXCLUDE_INCLUDE
                            } else {
                                0 as ::core::ffi::c_int
                            } | exclude_options(cmd != 0),
                        );
                        cmd += 1;
                    }
                    break 's_743;
                }
                132 => {
                    let mut cmd_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while cmd_0 < 2 as ::core::ffi::c_int {
                        if excluded_patterns[cmd_0 as usize].is_null() {
                            excluded_patterns[cmd_0 as usize] = new_exclude();
                        }
                        if add_exclude_file(
                            Some(
                                add_exclude
                                    as unsafe extern "C" fn(
                                        *mut exclude,
                                        *const ::core::ffi::c_char,
                                        ::core::ffi::c_int,
                                    )
                                        -> (),
                            ),
                            excluded_patterns[cmd_0 as usize],
                            optarg,
                            exclude_options(cmd_0 != 0),
                            '\n' as ::core::ffi::c_char,
                        ) != 0 as ::core::ffi::c_int
                        {
                            if ::core::mem::size_of::<C2Rust_Unnamed_20>() != 0 {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        *__errno_location(),
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        optarg,
                                    );
                                    if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            *__errno_location(),
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            optarg,
                                        );
                                        if __errstatus != 0 as ::core::ffi::c_int {
                                            unreachable!();
                                        } else {
                                        };
                                    });
                                };
                                if false {
                                } else {
                                    unreachable!();
                                };
                            } else {
                                if 0 != 0 {
                                    error(
                                        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int,
                                        *__errno_location(),
                                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                        optarg,
                                    );
                                    if C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                    {
                                        unreachable!();
                                    } else {
                                    };
                                } else {
                                    ({
                                        let __errstatus: ::core::ffi::c_int =
                                            C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int;
                                        error(
                                            __errstatus,
                                            *__errno_location(),
                                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                                            optarg,
                                        );
                                        if __errstatus != 0 as ::core::ffi::c_int {
                                            unreachable!();
                                        } else {
                                        };
                                    });
                                };
                                if false {
                                } else {
                                    unreachable!();
                                };
                            };
                        }
                        cmd_0 += 1;
                    }
                    break 's_743;
                }
                130 => {
                    strip_trailing_slashes(optarg);
                    let mut cmd_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while cmd_1 < 2 as ::core::ffi::c_int {
                        if excluded_directory_patterns[cmd_1 as usize].is_null() {
                            excluded_directory_patterns[cmd_1 as usize] = new_exclude();
                        }
                        add_exclude(
                            excluded_directory_patterns[cmd_1 as usize],
                            optarg,
                            exclude_options(cmd_1 != 0),
                        );
                        cmd_1 += 1;
                    }
                    break 's_743;
                }
                133 => {
                    group_separator = optarg;
                    break 's_743;
                }
                135 => {
                    line_buffered = r#true != 0;
                    break 's_743;
                }
                136 => {
                    label = optarg;
                    break 's_743;
                }
                85 | 0 => {
                    break 's_743;
                }
                _ => {
                    usage(C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int);
                    break 's_743;
                }
            }
            directories = directories_type::RECURSE_DIRECTORIES;
            last_recursive = prev_optind;
        }
    }
    if show_version {
        version_etc(
            stdout,
            getprogname(),
            PACKAGE_NAME.as_ptr(),
            VERSION.as_ptr(),
            nullptr as *mut ::core::ffi::c_char,
        );
        puts(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Written by Mike Haertel and others; see\n<https://git.savannah.gnu.org/cgit/grep.git/tree/AUTHORS>.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        Pprint_version();
        return EXIT_SUCCESS;
    }
    if show_help != 0 {
        usage(EXIT_SUCCESS);
    }
    if !keys.is_null() {
        if keycc == 0 as idx_t {
            out_invert = out_invert as ::core::ffi::c_int ^ r#true != 0;
            match_words = r#false != 0;
            match_lines = match_words;
            let c2rust_fresh14 = keycc;
            keycc += 1;
            *keys.offset(c2rust_fresh14 as isize) = '\n' as ::core::ffi::c_char;
        }
    } else if optind < argc {
        let c2rust_fresh15 = optind;
        optind += 1;
        let mut pat: *const ::core::ffi::c_char = *argv.offset(c2rust_fresh15 as isize);
        let mut skip_bs: bool = matcher
            != C2Rust_Unnamed_17::F_MATCHER_INDEX.0 as ::core::ffi::c_int
            && *pat.offset(0isize) as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
            && *pat.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int;
        keys = xstrdup(pat.offset(skip_bs as ::core::ffi::c_int as isize));
        pattern_array = keys;
        let mut patlen: idx_t = strlen(keys) as idx_t;
        *keys.offset(patlen as isize) = '\n' as ::core::ffi::c_char;
        keycc = update_patterns(
            keys,
            0 as idx_t,
            patlen + 1 as idx_t,
            b"\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        usage(C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int);
    }
    keycc -= 1;
    hash_free(pattern_table);
    let mut possibly_tty: bool = r#false != 0;
    let mut tmp_stat: stat = stat {
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
    if !exit_on_match && fstat(STDOUT_FILENO, &raw mut tmp_stat) == 0 as ::core::ffi::c_int {
        if tmp_stat.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t {
            out_stat = tmp_stat;
        } else if tmp_stat.st_mode & __S_IFMT as __mode_t == 0o20000 as __mode_t {
            let mut null_stat: stat = stat {
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
            if stat(
                b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut null_stat,
            ) == 0 as ::core::ffi::c_int
                && tmp_stat.st_dev ^ null_stat.st_dev | tmp_stat.st_ino ^ null_stat.st_ino == 0
            {
                dev_null_output = r#true != 0;
            } else {
                possibly_tty = r#true != 0;
            }
        }
    }
    if exit_on_match as ::core::ffi::c_int | dev_null_output as ::core::ffi::c_int != 0 {
        list_files = C2Rust_Unnamed_9::LISTFILES_NONE;
    }
    if exit_on_match as ::core::ffi::c_int | dev_null_output as ::core::ffi::c_int != 0
        || list_files.0 != C2Rust_Unnamed_9::LISTFILES_NONE.0
    {
        count_matches = r#false != 0;
        if max_count == INTMAX_MAX as intmax_t {
            done_on_match = r#true != 0;
        }
    }
    out_quiet = count_matches as ::core::ffi::c_int
        | done_on_match as ::core::ffi::c_int
        | exit_on_match as ::core::ffi::c_int
        != 0;
    if out_after < 0 as intmax_t {
        out_after = default_context;
    }
    if out_before < 0 as intmax_t {
        out_before = default_context;
    }
    if (max_count == 0 as intmax_t
        || keycc == 0 as idx_t
            && out_invert as ::core::ffi::c_int != 0
            && !match_lines
            && !match_words)
        && list_files.0 != C2Rust_Unnamed_9::LISTFILES_NONMATCHING.0
    {
        return EXIT_FAILURE;
    }
    if color_option == 2 as ::core::ffi::c_int {
        color_option = (possibly_tty as ::core::ffi::c_int != 0
            && should_colorize() != 0
            && isatty(STDOUT_FILENO) != 0) as ::core::ffi::c_int;
    }
    init_colorize();
    if color_option != 0 {
        let mut userval: *mut ::core::ffi::c_char =
            getenv(b"GREP_COLOR\0".as_ptr() as *const ::core::ffi::c_char);
        if !userval.is_null() && *userval as ::core::ffi::c_int != 0 {
            let mut q: *mut ::core::ffi::c_char = userval;
            while *q as ::core::ffi::c_int == ';' as ::core::ffi::c_int
                || c_isdigit(*q as ::core::ffi::c_int) as ::core::ffi::c_int != 0
            {
                if *q.offset(1isize) == 0 {
                    context_match_color = userval;
                    selected_match_color = context_match_color;
                    break;
                } else {
                    q = q.offset(1);
                }
            }
        }
        parse_grep_colors();
        if selected_match_color == userval as *const ::core::ffi::c_char
            || context_match_color == userval as *const ::core::ffi::c_char
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    userval,
                    userval,
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
                            b"warning: GREP_COLOR='%s' is deprecated; use GREP_COLORS='mt=%s'\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        userval,
                        userval,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    initialize_unibyte_mask();
    if matcher < 0 as ::core::ffi::c_int {
        matcher = C2Rust_Unnamed_17::G_MATCHER_INDEX.0 as ::core::ffi::c_int;
    }
    if matcher == C2Rust_Unnamed_17::F_MATCHER_INDEX.0 as ::core::ffi::c_int
        || matcher == C2Rust_Unnamed_17::E_MATCHER_INDEX.0 as ::core::ffi::c_int
        || matcher == C2Rust_Unnamed_17::G_MATCHER_INDEX.0 as ::core::ffi::c_int
    {
        if match_icase {
            setup_ok_fold();
        }
        if matcher == C2Rust_Unnamed_17::F_MATCHER_INDEX.0 as ::core::ffi::c_int {
            if if !localeinfo.multibyte {
                (n_patterns == 1 as idx_t && match_words as ::core::ffi::c_int != 0)
                    as ::core::ffi::c_int
            } else {
                (contains_encoding_error(keys, keycc) as ::core::ffi::c_int != 0
                    || match_icase as ::core::ffi::c_int != 0
                        && !fgrep_icase_available(keys, keycc))
                    as ::core::ffi::c_int
            } != 0
            {
                fgrep_to_grep_pattern(&raw mut pattern_array, &raw mut keycc);
                keys = pattern_array;
                matcher = C2Rust_Unnamed_17::G_MATCHER_INDEX.0 as ::core::ffi::c_int;
            }
        } else if (1 as idx_t) < n_patterns {
            matcher = try_fgrep_pattern(matcher, keys, &raw mut keycc);
        }
    }
    execute = matchers[matcher as usize].execute;
    compiled_pattern = matchers[matcher as usize]
        .compile
        .expect("non-null function pointer")(
        keys,
        keycc,
        matchers[matcher as usize].syntax as reg_syntax_t,
        only_matching as ::core::ffi::c_int | color_option != 0,
    );
    let mut eolbytes: [::core::ffi::c_char; 3] =
        [0 as ::core::ffi::c_char, eolbyte, 0 as ::core::ffi::c_char];
    let mut match_size: idx_t = 0;
    skip_empty_lines = (execute.expect("non-null function pointer")(
        compiled_pattern,
        (&raw mut eolbytes as *mut ::core::ffi::c_char).offset(1 as ::core::ffi::c_int as isize),
        1 as idx_t,
        &raw mut match_size,
        ::core::ptr::null::<::core::ffi::c_char>(),
    ) == 0) as ::core::ffi::c_int
        == out_invert as ::core::ffi::c_int;
    let mut num_operands: ::core::ffi::c_int = argc - optind;
    out_file =
        if filename_option == 0 as ::core::ffi::c_int && num_operands <= 1 as ::core::ffi::c_int {
            -((directories.0 == directories_type::RECURSE_DIRECTORIES.0) as ::core::ffi::c_int)
        } else {
            (0 as ::core::ffi::c_int <= filename_option) as ::core::ffi::c_int
        };
    if binary {
        xset_binary_mode(STDOUT_FILENO, O_BINARY);
    }
    let mut psize: ::core::ffi::c_long =
        sysconf(C2Rust_Unnamed_0::_SC_PAGESIZE.0 as ::core::ffi::c_int);
    if !((0 as ::core::ffi::c_long) < psize
        && psize
            <= (IDX_MAX
                - C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as ::core::ffi::c_long)
                / 2 as ::core::ffi::c_long)
    {
        abort();
    }
    pagesize = psize as idx_t;
    good_readsize = (if (C2Rust_Unnamed_15::GOOD_READSIZE_MIN.0 as ::core::ffi::c_int as uintptr_t)
        .wrapping_rem(pagesize as uintptr_t)
        == 0 as uintptr_t
    {
        C2Rust_Unnamed_15::GOOD_READSIZE_MIN.0 as ::core::ffi::c_int as uintptr_t
    } else {
        (C2Rust_Unnamed_15::GOOD_READSIZE_MIN.0 as ::core::ffi::c_int as uintptr_t).wrapping_add(
            (pagesize as uintptr_t).wrapping_sub(
                (C2Rust_Unnamed_15::GOOD_READSIZE_MIN.0 as ::core::ffi::c_int as uintptr_t)
                    .wrapping_rem(pagesize as uintptr_t),
            ),
        )
    }) as idx_t;
    bufalloc =
        good_readsize + pagesize + C2Rust_Unnamed_13::uword_size.0 as ::core::ffi::c_int as idx_t;
    buffer = ximalloc(bufalloc) as *mut ::core::ffi::c_char;
    if fts_options & FTS_LOGICAL != 0 && devices.0 == C2Rust_Unnamed_8::READ_COMMAND_LINE_DEVICES.0
    {
        devices = C2Rust_Unnamed_8::READ_DEVICES;
    }
    let mut files: *const *mut ::core::ffi::c_char =
        ::core::ptr::null::<*mut ::core::ffi::c_char>();
    if (0 as ::core::ffi::c_int) < num_operands {
        files = argv.offset(optind as isize);
    } else if directories.0 == directories_type::RECURSE_DIRECTORIES.0
        && (0 as ::core::ffi::c_int) < last_recursive
    {
        static mut cwd_only: [*mut ::core::ffi::c_char; 2] = [
            b".\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        files = &raw const cwd_only as *const *mut ::core::ffi::c_char;
        omit_dot_slash = r#true != 0;
    } else {
        static mut stdin_only: [*mut ::core::ffi::c_char; 2] = [
            b"-\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ];
        files = &raw const stdin_only as *const *mut ::core::ffi::c_char;
    }
    let mut status: bool = r#true != 0;
    loop {
        let c2rust_fresh16 = files;
        files = files.offset(1);
        status = status as ::core::ffi::c_int
            & grep_command_line_arg(*c2rust_fresh16) as ::core::ffi::c_int
            != 0;
        if (*files).is_null() {
            break;
        }
    }
    return if errseen as ::core::ffi::c_int != 0 {
        C2Rust_Unnamed_1::EXIT_TROUBLE.0 as ::core::ffi::c_int
    } else {
        status as ::core::ffi::c_int
    };
}
pub const nullptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

// SPDX-License-Identifier: GPL-3.0-or-later
// Translate the pinned egrep.sh warning and option insertion into internal
// Rust dispatch. The GNU shell aliases themselves do not implement matching.
static mut RBOXC_GREP_ALIAS_ARGV: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut();
unsafe extern "C" fn rboxc_grep_release_alias() {
    let saved_errno = *libc::__errno_location();
    libc::free(RBOXC_GREP_ALIAS_ARGV.cast());
    RBOXC_GREP_ALIAS_ARGV = ::core::ptr::null_mut();
    *libc::__errno_location() = saved_errno;
}
unsafe fn rboxc_grep_alias(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
    option: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let name = *argv;
    let slash = libc::strrchr(name, b'/' as ::core::ffi::c_int);
    let command = if slash.is_null() { name } else { slash.add(1) };
    libc::fprintf(rboxc_grep_stderr,
        b"%s: warning: %s is obsolescent; using grep %s\n\0".as_ptr().cast(),
        command, command, option);
    let Some(count) = argc.checked_add(1) else { return 2; };
    let adjusted = xnmalloc((count as usize + 1) as size_t,
        ::core::mem::size_of::<*mut ::core::ffi::c_char>() as size_t)
        .cast::<*mut ::core::ffi::c_char>();
    RBOXC_GREP_ALIAS_ARGV = adjusted;
    atexit(Some(rboxc_grep_release_alias));
    *adjusted = b"grep\0".as_ptr().cast_mut().cast();
    *adjusted.add(1) = option;
    for index in 1..argc as usize {
        *adjusted.add(index + 1) = *argv.add(index);
    }
    *adjusted.add(count as usize) = ::core::ptr::null_mut();
    let status = single_binary_main_grep(count, adjusted);
    rboxc_grep_release_alias();
    status
}
pub unsafe extern "C" fn single_binary_main_egrep(argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_grep_alias(argc, argv, b"-E\0".as_ptr().cast_mut().cast())
}
pub unsafe extern "C" fn single_binary_main_fgrep(argc: ::core::ffi::c_int,
    argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_grep_alias(argc, argv, b"-F\0".as_ptr().cast_mut().cast())
}

