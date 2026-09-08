// Generated from pinned GNU patch 2.8 by scripts/translate-entry-provider.py.
// Source SHA-256: ca20b87c33247159560d896283c7ac506f71304bdc3249d9826c8bfb92417106
/* patch - a program to apply diffs to original files */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
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
    #[link_name = "rboxc_patch_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_patch_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn chdir(__path: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn setbuf(__stream: *mut FILE, __buf: *mut ::core::ffi::c_char);
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn free(_: *mut ::core::ffi::c_void);
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_merge_hunk"]
    fn merge_hunk(hunk: intmax_t, _: *mut outstate, r#where: idx_t, _: *mut bool) -> bool;
    #[link_name = "rboxc_patch_argmatch"]
    fn argmatch(
        arg: *const ::core::ffi::c_char,
        arglist: *const *const ::core::ffi::c_char,
        vallist: *const ::core::ffi::c_void,
        valsize: size_t,
    ) -> ptrdiff_t;
    #[link_name = "rboxc_patch_argmatch_invalid"]
    fn argmatch_invalid(
        context: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
        problem: ptrdiff_t,
    );
    #[link_name = "rboxc_patch_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_patch_exit_failure"]
    static mut exit_failure: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_input_lines"]
    static mut input_lines: idx_t;
    #[link_name = "rboxc_patch_ifetch"]
    fn ifetch(_: idx_t) -> iline;
    #[link_name = "rboxc_patch_get_input_file"]
    fn get_input_file(
        _: *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: mode_t,
    ) -> bool;
    #[link_name = "rboxc_patch_re_input"]
    fn re_input();
    #[link_name = "rboxc_patch_scan_input"]
    fn scan_input(_: *mut ::core::ffi::c_char, _: mode_t, _: ::core::ffi::c_int);
    #[link_name = "rboxc_patch_pch_end"]
    fn pch_end() -> idx_t;
    #[link_name = "rboxc_patch_pch_first"]
    fn pch_first() -> idx_t;
    #[link_name = "rboxc_patch_pch_hunk_beg"]
    fn pch_hunk_beg() -> idx_t;
    #[link_name = "rboxc_patch_pch_c_function"]
    fn pch_c_function() -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_patch_pch_git_diff"]
    fn pch_git_diff() -> bool;
    #[link_name = "rboxc_patch_pch_timestr"]
    fn pch_timestr(which: bool) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_patch_pch_mode"]
    fn pch_mode(which: bool) -> mode_t;
    #[link_name = "rboxc_patch_pch_newfirst"]
    fn pch_newfirst() -> idx_t;
    #[link_name = "rboxc_patch_pch_prefix_context"]
    fn pch_prefix_context() -> idx_t;
    #[link_name = "rboxc_patch_pch_ptrn_lines"]
    fn pch_ptrn_lines() -> idx_t;
    #[link_name = "rboxc_patch_pch_repl_lines"]
    fn pch_repl_lines() -> idx_t;
    #[link_name = "rboxc_patch_pch_suffix_context"]
    fn pch_suffix_context() -> idx_t;
    #[link_name = "rboxc_patch_pch_swap"]
    fn pch_swap();
    #[link_name = "rboxc_patch_pch_write_line"]
    fn pch_write_line(_: idx_t, _: *mut FILE) -> bool;
    #[link_name = "rboxc_patch_there_is_another_patch"]
    fn there_is_another_patch(_: bool, _: *mut mode_t) -> bool;
    #[link_name = "rboxc_patch_pfetch"]
    fn pfetch(_: idx_t) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_patch_pch_char"]
    fn pch_char(_: idx_t) -> ::core::ffi::c_char;
    #[link_name = "rboxc_patch_another_hunk"]
    fn another_hunk(_: diff, _: bool) -> bool;
    #[link_name = "rboxc_patch_pch_says_nonexistent"]
    fn pch_says_nonexistent(_: bool) -> ::core::ffi::c_char;
    #[link_name = "rboxc_patch_pch_line_len"]
    fn pch_line_len(_: idx_t) -> idx_t;
    #[link_name = "rboxc_patch_pch_name"]
    fn pch_name(_: nametype) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_patch_pch_copy"]
    fn pch_copy() -> bool;
    #[link_name = "rboxc_patch_pch_rename"]
    fn pch_rename() -> bool;
    #[link_name = "rboxc_patch_do_ed_script"]
    fn do_ed_script(_: *mut ::core::ffi::c_char, _: *mut outfile, _: *mut FILE);
    #[link_name = "rboxc_patch_open_patch_file"]
    fn open_patch_file(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_patch_re_patch"]
    fn re_patch();
    #[link_name = "rboxc_patch_pch_normalize"]
    fn pch_normalize(_: diff);
    #[link_name = "rboxc_patch_p_timestamp"]
    static mut p_timestamp: [timespec; 2];
    #[link_name = "rboxc_patch_quoting_style_args"]
    static quoting_style_args: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_patch_set_quoting_style"]
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    #[link_name = "rboxc_patch_quotearg"]
    fn quotearg(arg: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_patch_simple_backup_suffix"]
    static mut simple_backup_suffix: *const ::core::ffi::c_char;
    #[link_name = "rboxc_patch_find_backup_file_name"]
    fn find_backup_file_name(
        _: ::core::ffi::c_int,
        _: *const ::core::ffi::c_char,
        _: backup_type,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_patch_get_version"]
    fn get_version(
        context: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
    ) -> backup_type;
    #[link_name = "rboxc_patch_backup_type"]
    static mut backup_type_0: backup_type;
    #[link_name = "rboxc_patch_volatilize"]
    fn volatilize(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_patch_ok_to_reverse"]
    fn ok_to_reverse(_: *const ::core::ffi::c_char, ...) -> bool;
    #[link_name = "rboxc_patch_say"]
    fn say(_: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_patch_fatal"]
    fn fatal(_: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_patch_pfatal"]
    fn pfatal(_: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_patch_create_file"]
    fn create_file(
        _: *mut outfile,
        _: ::core::ffi::c_int,
        _: mode_t,
        _: bool,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_Fclose"]
    fn Fclose(_: *mut FILE);
    #[link_name = "rboxc_patch_Fflush"]
    fn Fflush(_: *mut FILE);
    #[link_name = "rboxc_patch_Fprintf"]
    fn Fprintf(_: *mut FILE, _: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_patch_Fputc"]
    fn Fputc(_: ::core::ffi::c_int, _: *mut FILE);
    #[link_name = "rboxc_patch_Fputs"]
    fn Fputs(_: *const ::core::ffi::c_char, _: *mut FILE);
    #[link_name = "rboxc_patch_Fwrite"]
    fn Fwrite(_: *const ::core::ffi::c_void, _: size_t, _: size_t, _: *mut FILE);
    #[link_name = "rboxc_patch_copy_file"]
    fn copy_file(
        _: *mut ::core::ffi::c_char,
        _: *const stat,
        _: *mut outfile,
        _: *mut stat,
        _: ::core::ffi::c_int,
        _: mode_t,
        _: file_attributes,
        _: bool,
    );
    #[link_name = "rboxc_patch_append_to_file"]
    fn append_to_file(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_patch_init_signals"]
    fn init_signals();
    #[link_name = "rboxc_patch_defer_signals"]
    fn defer_signals();
    #[link_name = "rboxc_patch_undefer_signals"]
    fn undefer_signals();
    #[link_name = "rboxc_patch_init_backup_hash_table"]
    fn init_backup_hash_table();
    #[link_name = "rboxc_patch_init_time"]
    fn init_time();
    #[link_name = "rboxc_patch_create_backup"]
    fn create_backup(_: *mut ::core::ffi::c_char, _: *const stat, _: bool);
    #[link_name = "rboxc_patch_move_file"]
    fn move_file(_: *mut outfile, _: *const stat, _: *mut ::core::ffi::c_char, _: mode_t, _: bool);
    #[link_name = "rboxc_patch_read_fatal"]
    fn read_fatal();
    #[link_name = "rboxc_patch_removedirs"]
    fn removedirs(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_patch_write_fatal"]
    fn write_fatal();
    #[link_name = "rboxc_patch_putline"]
    fn putline(_: *mut FILE, ...);
    #[link_name = "rboxc_patch_insert_file_id"]
    fn insert_file_id(_: *const stat, _: file_id_type);
    #[link_name = "rboxc_patch_lookup_file_id"]
    fn lookup_file_id(_: *const stat) -> file_id_type;
    #[link_name = "rboxc_patch_set_queued_output"]
    fn set_queued_output(_: *const stat, _: bool);
    #[link_name = "rboxc_patch_has_queued_output"]
    fn has_queued_output(_: *const stat) -> bool;
    #[link_name = "rboxc_patch_stat_file"]
    fn stat_file(_: *mut ::core::ffi::c_char, _: *mut stat) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_set_file_attributes"]
    fn set_file_attributes(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: file_attributes,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *const stat,
        _: mode_t,
        _: *mut timespec,
    );
    #[link_name = "rboxc_patch_make_tempfile"]
    fn make_tempfile(
        _: *mut outfile,
        _: ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: mode_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_version"]
    fn version();
    #[link_name = "rboxc_patch_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_patch_ximalloc"]
    fn ximalloc(s: idx_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_patch_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_patch_xstdopen"]
    fn xstdopen();
    #[link_name = "rboxc_patch_unsafe"]
    static mut r#unsafe: bool;
    #[link_name = "rboxc_patch_safe_open"]
    fn safe_open(
        pathname: *mut ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        mode: mode_t,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_safe_unlink"]
    fn safe_unlink(pathname: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_patch_safe_access"]
    fn safe_access(
        pathname: *mut ::core::ffi::c_char,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}
pub type ptrdiff_t = isize;
pub type size_t = usize;
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
pub type intmax_t = ::libc::intmax_t;
pub type idx_t = ptrdiff_t;
pub type mode_t = __mode_t;
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
pub struct outfile {
    pub name: *mut ::core::ffi::c_char,
    pub exists: *mut ::core::ffi::c_char,
    pub alloc: *mut ::core::ffi::c_char,
    pub temporary: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct verbosity(pub ::core::ffi::c_uint);
impl verbosity {
    pub const DEFAULT_VERBOSITY: Self = Self(0);
    pub const SILENT: Self = Self(1);
    pub const VERBOSE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct diff(pub ::core::ffi::c_uint);
impl diff {
    pub const NO_DIFF: Self = Self(0);
    pub const CONTEXT_DIFF: Self = Self(1);
    pub const NORMAL_DIFF: Self = Self(2);
    pub const ED_DIFF: Self = Self(3);
    pub const NEW_CONTEXT_DIFF: Self = Self(4);
    pub const UNI_DIFF: Self = Self(5);
    pub const GIT_BINARY_DIFF: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_to_output {
    pub from: outfile,
    pub from_st: stat,
    pub to: *mut ::core::ffi::c_char,
    pub mode: mode_t,
    pub backup: bool,
    pub next: *mut file_to_output,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct outstate {
    pub ofp: *mut FILE,
    pub after_newline: bool,
    pub zero_output: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iline {
    pub ptr: *const ::core::ffi::c_char,
    pub size: idx_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct conflict_style(pub ::core::ffi::c_uint);
impl conflict_style {
    pub const MERGE_MERGE: Self = Self(0);
    pub const MERGE_DIFF3: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nametype(pub ::core::ffi::c_uint);
impl nametype {
    pub const OLD: Self = Self(0);
    pub const NEW: Self = Self(1);
    pub const INDEX: Self = Self(2);
    pub const NONE: Self = Self(3);
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
pub struct backup_type(pub ::core::ffi::c_uint);
impl backup_type {
    pub const no_backups: Self = Self(0);
    pub const simple_backups: Self = Self(1);
    pub const numbered_existing_backups: Self = Self(2);
    pub const numbered_backups: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct file_id_type(pub ::core::ffi::c_uint);
impl file_id_type {
    pub const UNKNOWN: Self = Self(0);
    pub const CREATED: Self = Self(1);
    pub const DELETE_LATER: Self = Self(2);
    pub const OVERWRITTEN: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct file_attributes(pub ::core::ffi::c_uint);
impl file_attributes {
    pub const FA_TIMES: Self = Self(1);
    pub const FA_IDS: Self = Self(2);
    pub const FA_MODE: Self = Self(4);
    pub const FA_XATTRS: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const EXIT_TROUBLE: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const OUTSIDE: Self = Self(0);
    pub const IN_IFNDEF: Self = Self(1);
    pub const IN_IFDEF: Self = Self(2);
    pub const IN_ELSE: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const RO_IGNORE: Self = Self(0);
    pub const RO_WARN: Self = Self(1);
    pub const RO_FAIL: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_to_delete {
    pub name: *mut ::core::ffi::c_char,
    pub st: stat,
    pub backup: bool,
    pub next: *mut file_to_delete,
}
#[inline]
unsafe extern "C" fn c_isblank(mut c: ::core::ffi::c_int) -> bool {
    return c == ' ' as ::core::ffi::c_int || c == '\t' as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn c_isdigit(mut c: ::core::ffi::c_int) -> bool {
    match c {
        48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 | 56 | 57 => return r#true != 0,
        _ => return r#false != 0,
    };
}
pub const CHAR_MAX: ::core::ffi::c_int = __SCHAR_MAX__;
pub const INTMAX_MIN: ::core::ffi::c_long =
    -9223372036854775807 as ::core::ffi::c_long - 1 as ::core::ffi::c_long;
pub const INTMAX_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const ELOOP: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STDERR_FILENO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const W_OK: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EXDEV: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __S_IFREG: ::core::ffi::c_int = 0o100000 as ::core::ffi::c_int;
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const AT_FDCWD: ::core::ffi::c_int = -100 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const S_IFMT: ::core::ffi::c_int = __S_IFMT;
pub const S_IFREG: ::core::ffi::c_int = __S_IFREG;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRWXG: ::core::ffi::c_int = S_IRWXU >> 3 as ::core::ffi::c_int;
pub const S_IRWXO: ::core::ffi::c_int = S_IRWXG >> 3 as ::core::ffi::c_int;
pub const S_IRWXUGO: ::core::ffi::c_int = S_IRWXU | S_IRWXG | S_IRWXO;
pub const binary_transput: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const optional_argument: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> ::core::ffi::c_int {
    return 2 as ::core::ffi::c_int
        * ((a.tv_sec > b.tv_sec) as ::core::ffi::c_int
            - (a.tv_sec < b.tv_sec) as ::core::ffi::c_int)
        + ((a.tv_nsec > b.tv_nsec) as ::core::ffi::c_int
            - (a.tv_nsec < b.tv_nsec) as ::core::ffi::c_int);
}
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn devolatilize(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    return s as *mut ::core::ffi::c_char;
}
#[export_name = "rboxc_patch_batch"]
pub static mut batch: bool = false;
#[export_name = "rboxc_patch_canonicalize_ws"]
pub static mut canonicalize_ws: bool = false;
#[export_name = "rboxc_patch_dry_run"]
pub static mut dry_run: bool = false;
#[export_name = "rboxc_patch_follow_symlinks"]
pub static mut follow_symlinks: bool = false;
#[export_name = "rboxc_patch_force"]
pub static mut force: bool = false;
#[export_name = "rboxc_patch_no_strip_trailing_cr"]
pub static mut no_strip_trailing_cr: bool = false;
#[export_name = "rboxc_patch_noreverse_flag"]
pub static mut noreverse_flag: bool = false;
#[export_name = "rboxc_patch_posixly_correct"]
pub static mut posixly_correct: bool = false;
#[export_name = "rboxc_patch_reverse_flag"]
pub static mut reverse_flag: bool = false;
#[export_name = "rboxc_patch_set_time"]
pub static mut set_time: bool = false;
#[export_name = "rboxc_patch_set_utc"]
pub static mut set_utc: bool = false;
#[export_name = "rboxc_patch_skip_rest_of_patch"]
pub static mut skip_rest_of_patch: bool = false;
#[export_name = "rboxc_patch_inname"]
pub static mut inname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_patch_outfile"]
pub static mut outfile: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_patch_revision"]
pub static mut revision: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_patch_origbase"]
pub static mut origbase: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_patch_origprae"]
pub static mut origprae: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_patch_origsuff"]
pub static mut origsuff: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_patch_conflict_style"]
pub static mut conflict_style_0: conflict_style = conflict_style::MERGE_MERGE;
#[export_name = "rboxc_patch_diff_type"]
pub static mut diff_type: diff = diff::NO_DIFF;
#[export_name = "rboxc_patch_verbosity"]
pub static mut verbosity_0: verbosity = verbosity::DEFAULT_VERBOSITY;
#[export_name = "rboxc_patch_last_frozen_line"]
pub static mut last_frozen_line: idx_t = 0;
#[export_name = "rboxc_patch_inerrno"]
pub static mut inerrno: ::core::ffi::c_int = 0;
#[export_name = "rboxc_patch_patch_get"]
pub static mut patch_get: intmax_t = 0;
#[export_name = "rboxc_patch_strippath"]
pub static mut strippath: intmax_t = 0;
#[export_name = "rboxc_patch_in_offset"]
pub static mut in_offset: ptrdiff_t = 0;
#[export_name = "rboxc_patch_out_offset"]
pub static mut out_offset: ptrdiff_t = 0;
#[export_name = "rboxc_patch_invc"]
pub static mut invc: ::core::ffi::c_schar = 0;
#[export_name = "rboxc_patch_instat"]
pub static mut instat: stat = stat {
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
#[export_name = "rboxc_patch_tmped"]
pub static mut tmped: outfile = outfile {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    temporary: r#true != 0,
};
#[export_name = "rboxc_patch_tmppat"]
pub static mut tmppat: outfile = outfile {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    temporary: r#true != 0,
};
#[export_name = "rboxc_patch_debug"]
pub static mut debug: ::core::ffi::c_ushort = 0;
static mut merge: bool = false;
static mut reject_format: diff = diff::NO_DIFF;
static mut make_backups: bool = false;
static mut backup_if_mismatch: bool = false;
static mut backup_if_mismatch_specified: bool = false;
static mut version_control: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut version_control_context: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
static mut remove_empty_files: bool = false;
static mut explicit_inname: bool = false;
static mut read_only_behavior: C2Rust_Unnamed_1 = C2Rust_Unnamed_1::RO_WARN;
static mut reverse_flag_specified: bool = false;
static mut do_defines: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut if_defined: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"\n#ifdef \0") };
static mut not_defined: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"\n#ifndef \0") };
static mut else_defined: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"\n#else\n\0") };
static mut end_defined: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"\n#endif\n\0") };
static mut rejfp: *mut FILE = ::core::ptr::null_mut::<FILE>();
static mut patchname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut outrej: outfile = outfile {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    temporary: false,
};
static mut tmpout: outfile = outfile {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    temporary: r#true != 0,
};
static mut tmprej: outfile = outfile {
    name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    temporary: r#true != 0,
};
static mut maxfuzz: intmax_t = 2 as intmax_t;
static mut serrbuf: [::core::ffi::c_char; 8192] = [0; 8192];
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_patch(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut somefailed: bool = r#false != 0;
    let mut outstate: outstate = outstate {
        ofp: ::core::ptr::null_mut::<FILE>(),
        after_newline: false,
        zero_output: false,
    };
    let mut tmpoutst: stat = stat {
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
    let mut skip_reject_file: bool = r#false != 0;
    let mut apply_empty_patch: bool = r#false != 0;
    let mut file_type: mode_t = 0;
    let mut have_git_diff: bool = r#false != 0;
    ::core::ptr::write_volatile(
        &raw mut exit_failure,
        C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int,
    );
    set_program_name(*argv.offset(0isize));
    init_time();
    xstdopen();
    setbuf(stderr, &raw mut serrbuf as *mut ::core::ffi::c_char);
    atexit(Some(close_stdout as unsafe extern "C" fn() -> ()));
    strippath = -1 as intmax_t;
    val = getenv(b"QUOTING_STYLE\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: ::core::ffi::c_int = (if !val.is_null() {
        argmatch(
            val,
            &raw const quoting_style_args as *const *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_void>(),
            0 as size_t,
        )
    } else {
        -1 as ptrdiff_t
    }) as ::core::ffi::c_int;
    set_quoting_style(
        ::core::ptr::null_mut::<quoting_options>(),
        quoting_style(
            (if i < 0 as ::core::ffi::c_int {
                quoting_style::shell_quoting_style.0 as ::core::ffi::c_int
            } else {
                i
            }) as ::core::ffi::c_uint,
        ),
    );
    posixly_correct =
        !getenv(b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char).is_null();
    val = getenv(b"PATCH_GET\0".as_ptr() as *const ::core::ffi::c_char);
    patch_get = if !val.is_null() {
        numeric_string(
            val,
            r#true != 0,
            b"PATCH_GET value\0".as_ptr() as *const ::core::ffi::c_char,
        )
    } else {
        0 as intmax_t
    };
    val = getenv(b"SIMPLE_BACKUP_SUFFIX\0".as_ptr() as *const ::core::ffi::c_char);
    simple_backup_suffix = if !val.is_null() && *val as ::core::ffi::c_int != 0 {
        val
    } else {
        b".orig\0".as_ptr() as *const ::core::ffi::c_char
    };
    version_control = getenv(b"PATCH_VERSION_CONTROL\0".as_ptr() as *const ::core::ffi::c_char);
    if !version_control.is_null() {
        version_control_context =
            b"$PATCH_VERSION_CONTROL\0".as_ptr() as *const ::core::ffi::c_char;
    } else {
        version_control = getenv(b"VERSION_CONTROL\0".as_ptr() as *const ::core::ffi::c_char);
        if !version_control.is_null() {
            version_control_context = b"$VERSION_CONTROL\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    init_backup_hash_table();
    get_some_switches(argc, argv);
    if set_utc as ::core::ffi::c_int != 0
        && setenv(
            b"TZ\0".as_ptr() as *const ::core::ffi::c_char,
            b"UTC0\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
    {
        pfatal(b"setenv\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if !backup_if_mismatch_specified {
        backup_if_mismatch = !posixly_correct;
    }
    if make_backups as ::core::ffi::c_int | backup_if_mismatch as ::core::ffi::c_int != 0 {
        backup_type_0 = get_version(version_control_context, version_control);
    }
    init_output(&raw mut outstate);
    if !outfile.is_null() {
        outstate.ofp = open_outfile(outfile);
    }
    init_signals();
    if !inname.is_null() {
        r#unsafe = r#true != 0;
    }
    if !inname.is_null() && !outfile.is_null() {
        apply_empty_patch = r#true != 0;
        file_type = S_IFREG as mode_t;
        inerrno = -1 as ::core::ffi::c_int;
    }
    open_patch_file(patchname);
    while there_is_another_patch(
        !(!inname.is_null() || posixly_correct as ::core::ffi::c_int != 0),
        &raw mut file_type,
    ) as ::core::ffi::c_int
        != 0
        || apply_empty_patch as ::core::ffi::c_int != 0
    {
        let mut hunk: intmax_t = 0 as intmax_t;
        let mut failed: intmax_t = 0 as intmax_t;
        let mut mismatch: bool = r#false != 0;
        let mut outname: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if skip_rest_of_patch {
            somefailed = r#true != 0;
        }
        if have_git_diff as ::core::ffi::c_int != pch_git_diff() as ::core::ffi::c_int {
            if have_git_diff {
                output_files(::core::ptr::null::<stat>(), 0 as ::core::ffi::c_int);
                inerrno = -1 as ::core::ffi::c_int;
            }
            have_git_diff = !have_git_diff;
        }
        if !rejfp.is_null() {
            Fclose(rejfp);
            rejfp = ::core::ptr::null_mut::<FILE>();
        }
        defer_signals();
        perfile_cleanup_remove();
        undefer_signals();
        perfile_cleanup_free();
        if !skip_rest_of_patch && file_type == 0 {
            let mut old_mode: ::core::ffi::c_uint =
                pch_mode(reverse_flag) & S_IFMT as ::core::ffi::c_uint;
            let mut new_mode: ::core::ffi::c_uint =
                pch_mode(!reverse_flag) & S_IFMT as ::core::ffi::c_uint;
            say(
                b"File %s: can't change file type from %#o to %#o.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                quotearg(inname),
                old_mode,
                new_mode,
            );
            skip_rest_of_patch = r#true != 0;
            somefailed = r#true != 0;
        }
        if !skip_rest_of_patch {
            if !outfile.is_null() {
                outname = outfile;
            } else if pch_copy() as ::core::ffi::c_int != 0
                || pch_rename() as ::core::ffi::c_int != 0
            {
                outname = pch_name(nametype(
                    !reverse_flag as ::core::ffi::c_int as ::core::ffi::c_uint,
                ));
            } else {
                outname = inname;
            }
        }
        if pch_git_diff() as ::core::ffi::c_int != 0 && !skip_rest_of_patch {
            let mut outstat: stat = stat {
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
            let mut outerrno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if strcmp(inname, outname) == 0 {
                if inerrno == -1 as ::core::ffi::c_int {
                    inerrno = stat_file(inname, &raw mut instat);
                }
                outstat = instat;
                outerrno = inerrno;
            } else {
                outerrno = stat_file(outname, &raw mut outstat);
            }
            if outerrno == 0 {
                if has_queued_output(&raw mut outstat) {
                    output_files(&raw mut outstat, 0 as ::core::ffi::c_int);
                    outerrno = stat_file(outname, &raw mut outstat);
                    inerrno = -1 as ::core::ffi::c_int;
                }
                if outerrno == 0 {
                    set_queued_output(&raw mut outstat, r#true != 0);
                }
            }
        }
        if !skip_rest_of_patch {
            if !get_input_file(inname, outname, file_type) {
                skip_rest_of_patch = r#true != 0;
                somefailed = r#true != 0;
            }
        }
        if read_only_behavior.0 != C2Rust_Unnamed_1::RO_IGNORE.0
            && inerrno == 0
            && !(instat.st_mode & __S_IFMT as __mode_t == 0o120000 as __mode_t)
            && safe_access(inname, W_OK) != 0 as ::core::ffi::c_int
        {
            say(
                b"File %s is read-only; \0".as_ptr() as *const ::core::ffi::c_char,
                quotearg(inname),
            );
            if read_only_behavior.0 == C2Rust_Unnamed_1::RO_WARN.0 {
                say(b"trying to patch anyway\n\0".as_ptr() as *const ::core::ffi::c_char);
            } else {
                say(b"refusing to patch\n\0".as_ptr() as *const ::core::ffi::c_char);
                skip_rest_of_patch = r#true != 0;
                somefailed = r#true != 0;
            }
        }
        tmpoutst.st_size = -1 as __off_t;
        let mut outfd: ::core::ffi::c_int = make_tempfile(
            &raw mut tmpout,
            'o' as ::core::ffi::c_char,
            outname,
            O_WRONLY | binary_transput,
            instat.st_mode & S_IRWXUGO as mode_t,
        );
        if outfd < 0 as ::core::ffi::c_int {
            if diff_type.0 == diff::ED_DIFF.0
                || !(*__errno_location() == ELOOP || *__errno_location() == EXDEV)
            {
                pfatal(
                    b"Can't create temporary file %s\0".as_ptr() as *const ::core::ffi::c_char,
                    tmpout.name,
                );
            }
            say(
                b"Invalid file name %s -- skipping patch\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                quotearg(outname),
            );
            skip_rest_of_patch = r#true != 0;
            skip_reject_file = r#true != 0;
            somefailed = r#true != 0;
        }
        if outfile.is_null() {
            init_output(&raw mut outstate);
        }
        let mut ifd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
        if diff_type.0 == diff::ED_DIFF.0 {
            outstate.zero_output = r#false != 0;
            somefailed =
                somefailed as ::core::ffi::c_int | skip_rest_of_patch as ::core::ffi::c_int != 0;
            do_ed_script(inname, &raw mut tmpout, outstate.ofp);
            if !dry_run && outfile.is_null() && !skip_rest_of_patch {
                if fstat(outfd, &raw mut tmpoutst) != 0 as ::core::ffi::c_int {
                    pfatal(b"%s\0".as_ptr() as *const ::core::ffi::c_char, tmpout.name);
                }
                outstate.zero_output = tmpoutst.st_size == 0 as __off_t;
            }
        } else {
            let mut apply_anyway: bool = merge;
            if !skip_rest_of_patch && diff_type.0 == diff::GIT_BINARY_DIFF.0 {
                say(
                    b"File %s: git binary diffs are not supported.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    quotearg(outname),
                );
                skip_rest_of_patch = r#true != 0;
                somefailed = r#true != 0;
            }
            if !skip_rest_of_patch && outfile.is_null() {
                outstate.ofp = fdopen(
                    outfd,
                    if binary_transput != 0 {
                        b"wb\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"w\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                if outstate.ofp.is_null() {
                    pfatal(b"%s\0".as_ptr() as *const ::core::ffi::c_char, tmpout.name);
                }
            } else {
                outstate.after_newline = r#true != 0;
            }
            if !skip_rest_of_patch {
                if file_type & __S_IFMT as mode_t == 0o100000 as mode_t
                    && instat.st_size != 0 as __off_t
                {
                    let mut oflags: ::core::ffi::c_int = O_RDONLY
                        | binary_transput
                        | if follow_symlinks as ::core::ffi::c_int != 0 {
                            0 as ::core::ffi::c_int
                        } else {
                            O_NOFOLLOW
                        };
                    ifd = safe_open(inname, oflags, 0 as mode_t);
                    if ifd < 0 as ::core::ffi::c_int {
                        pfatal(
                            b"Can't open file %s\0".as_ptr() as *const ::core::ffi::c_char,
                            quotearg(inname),
                        );
                    }
                }
                scan_input(inname, file_type, ifd);
                if verbosity_0.0 != verbosity::SILENT.0 {
                    let mut renamed: bool = strcmp(inname, outname) != 0;
                    let mut skip_rename: bool = !renamed && pch_rename() as ::core::ffi::c_int != 0;
                    say(
                        b"%s %s %s%c\0".as_ptr() as *const ::core::ffi::c_char,
                        if dry_run as ::core::ffi::c_int != 0 {
                            b"checking\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"patching\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        if file_type & __S_IFMT as mode_t == 0o120000 as mode_t {
                            b"symbolic link\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"file\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        quotearg(outname),
                        if renamed as ::core::ffi::c_int != 0
                            || skip_rename as ::core::ffi::c_int != 0
                        {
                            ' ' as ::core::ffi::c_int
                        } else {
                            '\n' as ::core::ffi::c_int
                        },
                    );
                    if renamed as ::core::ffi::c_int != 0 || skip_rename as ::core::ffi::c_int != 0
                    {
                        say(
                            b"(%s%s from %s)\n\0".as_ptr() as *const ::core::ffi::c_char,
                            if skip_rename as ::core::ffi::c_int != 0 {
                                b"already \0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            if pch_copy() as ::core::ffi::c_int != 0 {
                                b"copied\0".as_ptr() as *const ::core::ffi::c_char
                            } else if pch_rename() as ::core::ffi::c_int != 0 {
                                b"renamed\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"read\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            if !skip_rename {
                                inname
                            } else {
                                pch_name(nametype(
                                    (strcmp(inname, pch_name(nametype::OLD)) == 0)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint,
                                ))
                            },
                        );
                    }
                }
            }
            while another_hunk(diff_type, reverse_flag) {
                let mut r#where: idx_t = 0 as idx_t;
                let mut newwhere: idx_t = 0;
                let mut fuzz: idx_t = 0 as idx_t;
                let mut mymaxfuzz: idx_t = 0;
                if merge {
                    mymaxfuzz = 0 as idx_t;
                } else {
                    let mut prefix_context: idx_t = pch_prefix_context();
                    let mut suffix_context: idx_t = pch_suffix_context();
                    let mut context: idx_t = if prefix_context > suffix_context {
                        prefix_context
                    } else {
                        suffix_context
                    };
                    mymaxfuzz = (if maxfuzz < context as intmax_t {
                        maxfuzz
                    } else {
                        context as intmax_t
                    }) as idx_t;
                }
                hunk += 1;
                if !skip_rest_of_patch {
                    let mut incr_fuzz: bool = false;
                    loop {
                        incr_fuzz = r#true != 0;
                        r#where = locate_hunk(fuzz);
                        if r#where == 0 || fuzz != 0 || in_offset != 0 {
                            mismatch = r#true != 0;
                        }
                        if hunk == 1 as intmax_t
                            && r#where == 0
                            && force as ::core::ffi::c_int | apply_anyway as ::core::ffi::c_int == 0
                            && reverse_flag as ::core::ffi::c_int
                                == reverse_flag_specified as ::core::ffi::c_int
                        {
                            pch_swap();
                            r#where = locate_hunk(fuzz);
                            if r#where != 0
                                && ok_to_reverse(
                                    b"%s patch detected!\0".as_ptr() as *const ::core::ffi::c_char,
                                    if reverse_flag as ::core::ffi::c_int != 0 {
                                        b"Unreversed\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"Reversed (or previously applied)\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                    },
                                ) as ::core::ffi::c_int
                                    != 0
                            {
                                reverse_flag = !reverse_flag;
                            } else {
                                pch_swap();
                                if r#where != 0 {
                                    apply_anyway = r#true != 0;
                                    incr_fuzz = r#false != 0;
                                    r#where = 0 as idx_t;
                                }
                            }
                        }
                        if !(!skip_rest_of_patch && r#where == 0 && {
                            fuzz += incr_fuzz as idx_t;
                            fuzz <= mymaxfuzz
                        }) {
                            break;
                        }
                    }
                }
                newwhere = if r#where != 0 { r#where } else { pch_first() } + out_offset;
                if skip_rest_of_patch as ::core::ffi::c_int != 0
                    || merge as ::core::ffi::c_int != 0
                        && !merge_hunk(hunk, &raw mut outstate, r#where, &raw mut somefailed)
                    || !merge
                        && (r#where == 1 as idx_t
                            && pch_says_nonexistent(reverse_flag) as ::core::ffi::c_int
                                == 2 as ::core::ffi::c_int
                            && instat.st_size != 0
                            || r#where == 0
                            || !apply_hunk(&raw mut outstate, r#where))
                {
                    if !skip_reject_file {
                        abort_hunk(outname, failed == 0, reverse_flag);
                    }
                    failed += 1;
                    if verbosity_0.0 == verbosity::VERBOSE.0
                        || !skip_rest_of_patch && verbosity_0.0 != verbosity::SILENT.0
                    {
                        say(
                            b"Hunk #%jd %s at %td%s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                            hunk,
                            if skip_rest_of_patch as ::core::ffi::c_int != 0 {
                                b"ignored\0".as_ptr() as *const ::core::ffi::c_char
                            } else {
                                b"FAILED\0".as_ptr() as *const ::core::ffi::c_char
                            },
                            newwhere,
                            if !skip_rest_of_patch
                                && check_line_endings(newwhere) as ::core::ffi::c_int != 0
                            {
                                b" (different line endings)\0".as_ptr()
                                    as *const ::core::ffi::c_char
                            } else {
                                b"\0".as_ptr() as *const ::core::ffi::c_char
                            },
                        );
                    }
                } else if !merge
                    && (verbosity_0.0 == verbosity::VERBOSE.0
                        || verbosity_0.0 != verbosity::SILENT.0 && (fuzz != 0 || in_offset != 0))
                {
                    say(
                        b"Hunk #%jd succeeded at %td\0".as_ptr() as *const ::core::ffi::c_char,
                        hunk,
                        newwhere,
                    );
                    if fuzz != 0 {
                        say(
                            b" with fuzz %td\0".as_ptr() as *const ::core::ffi::c_char,
                            fuzz,
                        );
                    }
                    if in_offset != 0 {
                        say(
                            b" (offset %td line%s)\0".as_ptr() as *const ::core::ffi::c_char,
                            in_offset,
                            (b"s\0".as_ptr() as *const ::core::ffi::c_char).offset(
                                (in_offset == 1 as ptrdiff_t) as ::core::ffi::c_int as isize,
                            ),
                        );
                    }
                    say(b".\n\0".as_ptr() as *const ::core::ffi::c_char);
                }
            }
            if !skip_rest_of_patch {
                if !spew_output(&raw mut outstate, &raw mut tmpoutst) {
                    say(b"Skipping patch.\n\0".as_ptr() as *const ::core::ffi::c_char);
                    skip_rest_of_patch = r#true != 0;
                }
            }
        }
        let mut replace_file: bool = r#false != 0;
        let mut backup: bool = false;
        let mut mode: mode_t = 0;
        if !skip_rest_of_patch && outfile.is_null() {
            backup = make_backups as ::core::ffi::c_int != 0
                || backup_if_mismatch as ::core::ffi::c_int != 0
                    && mismatch as intmax_t | failed != 0;
            if outstate.zero_output as ::core::ffi::c_int != 0
                && (remove_empty_files as ::core::ffi::c_int != 0
                    || pch_says_nonexistent(!reverse_flag) as ::core::ffi::c_int
                        == 2 as ::core::ffi::c_int
                        && !posixly_correct
                    || file_type & __S_IFMT as mode_t == 0o120000 as mode_t)
            {
                if !dry_run {
                    output_file(
                        ::core::ptr::null_mut::<outfile>(),
                        ::core::ptr::null::<stat>(),
                        outname,
                        if inname == outname {
                            &raw mut instat
                        } else {
                            ::core::ptr::null_mut::<stat>()
                        },
                        file_type | 0 as mode_t,
                        backup,
                    );
                }
            } else {
                if !outstate.zero_output
                    && pch_says_nonexistent(!reverse_flag) as ::core::ffi::c_int
                        == 2 as ::core::ffi::c_int
                    && (remove_empty_files as ::core::ffi::c_int != 0 || !posixly_correct)
                    && !(merge as ::core::ffi::c_int != 0 && somefailed as ::core::ffi::c_int != 0)
                {
                    mismatch = r#true != 0;
                    somefailed = r#true != 0;
                    if verbosity_0.0 != verbosity::SILENT.0 {
                        say(
                            b"Not deleting file %s as content differs from patch\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            quotearg(outname),
                        );
                    }
                }
                if !dry_run {
                    let mut old_mode_0: mode_t = pch_mode(reverse_flag);
                    let mut new_mode_0: mode_t = pch_mode(!reverse_flag);
                    let mut set_mode: bool = new_mode_0 != 0 && old_mode_0 != new_mode_0;
                    if failed < hunk
                        || diff_type.0 == diff::ED_DIFF.0
                        || set_mode as ::core::ffi::c_int != 0
                        || pch_copy() as ::core::ffi::c_int != 0
                        || pch_rename() as ::core::ffi::c_int != 0
                    {
                        let mut attr: file_attributes =
                            file_attributes(0 as ::core::ffi::c_int as ::core::ffi::c_uint);
                        let mut new_time: timespec =
                            p_timestamp[!reverse_flag as ::core::ffi::c_int as usize];
                        mode = (file_type
                            | if set_mode as ::core::ffi::c_int != 0 {
                                new_mode_0
                            } else {
                                instat.st_mode
                            } & S_IRWXUGO as __mode_t) as mode_t;
                        if set_time as ::core::ffi::c_int | set_utc as ::core::ffi::c_int != 0
                            && 0 as __syscall_slong_t <= new_time.tv_nsec
                        {
                            let mut old_time: timespec = p_timestamp[reverse_flag as usize];
                            if !force
                                && inerrno == 0
                                && pch_says_nonexistent(reverse_flag) as ::core::ffi::c_int
                                    != 2 as ::core::ffi::c_int
                                && 0 as __syscall_slong_t <= old_time.tv_nsec
                                && timespec_cmp(old_time, get_stat_mtime(&raw mut instat)) != 0
                            {
                                say(
                                    b"Not setting time of file %s (time mismatch)\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    quotearg(outname),
                                );
                            } else if !force && mismatch as intmax_t | failed != 0 {
                                say(
                                    b"Not setting time of file %s (contents mismatch)\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    quotearg(outname),
                                );
                            } else {
                                attr = file_attributes(attr.0 | file_attributes::FA_TIMES.0);
                            }
                        }
                        if inerrno != 0 {
                            if set_mode {
                                attr = file_attributes(attr.0 | file_attributes::FA_MODE.0);
                            }
                            set_file_attributes(
                                tmpout.name,
                                outfd,
                                attr,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                -1 as ::core::ffi::c_int,
                                ::core::ptr::null::<stat>(),
                                mode,
                                &raw mut new_time,
                            );
                        } else {
                            attr = file_attributes(
                                attr.0
                                    | (file_attributes::FA_IDS.0 as ::core::ffi::c_int
                                        | file_attributes::FA_MODE.0 as ::core::ffi::c_int
                                        | file_attributes::FA_XATTRS.0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_uint,
                            );
                            set_file_attributes(
                                tmpout.name,
                                outfd,
                                attr,
                                inname,
                                ifd,
                                &raw mut instat,
                                mode,
                                &raw mut new_time,
                            );
                        }
                        replace_file = r#true != 0;
                    } else if backup {
                        let mut outstat_0: stat = stat {
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
                        if stat_file(outname, &raw mut outstat_0) != 0 as ::core::ffi::c_int {
                            say(
                                b"Cannot stat file %s, skipping backup\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                outname,
                            );
                        } else {
                            let mut c2rust_lvalue: outfile = outfile {
                                name: outname,
                                exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                temporary: false,
                            };
                            output_file(
                                &raw mut c2rust_lvalue,
                                &raw mut outstat_0,
                                ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                ::core::ptr::null::<stat>(),
                                file_type | 0 as mode_t,
                                r#true != 0,
                            );
                        }
                    }
                }
            }
        }
        if 0 as ::core::ffi::c_int <= ifd && close(ifd) < 0 as ::core::ffi::c_int {
            read_fatal();
        }
        if outfile.is_null() {
            if !outstate.ofp.is_null() {
                Fclose(outstate.ofp);
                outstate.ofp = ::core::ptr::null_mut::<FILE>();
            } else if 0 as ::core::ffi::c_int <= outfd && close(outfd) < 0 as ::core::ffi::c_int {
                write_fatal();
            }
        }
        if replace_file {
            output_file(
                &raw mut tmpout,
                &raw mut tmpoutst,
                outname,
                ::core::ptr::null::<stat>(),
                mode,
                backup,
            );
            if pch_rename() {
                output_file(
                    ::core::ptr::null_mut::<outfile>(),
                    ::core::ptr::null::<stat>(),
                    inname,
                    &raw mut instat,
                    mode,
                    backup,
                );
            }
        }
        if diff_type.0 != diff::ED_DIFF.0 {
            let mut rejst: stat = stat {
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
            if failed != 0 && !skip_reject_file {
                Fflush(rejfp);
                if fstat(fileno(rejfp), &raw mut rejst) < 0 as ::core::ffi::c_int {
                    write_fatal();
                }
                Fclose(rejfp);
                rejfp = ::core::ptr::null_mut::<FILE>();
                somefailed = r#true != 0;
                say(
                    b"%jd out of %jd hunk%s %s\0".as_ptr() as *const ::core::ffi::c_char,
                    failed,
                    hunk,
                    (b"s\0".as_ptr() as *const ::core::ffi::c_char)
                        .offset((hunk == 1 as intmax_t) as ::core::ffi::c_int as isize),
                    if skip_rest_of_patch as ::core::ffi::c_int != 0 {
                        b"ignored\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"FAILED\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                let mut rejname: *mut ::core::ffi::c_char = outrej.name;
                if !outname.is_null()
                    && (rejname.is_null()
                        || strcmp(rejname, b"-\0".as_ptr() as *const ::core::ffi::c_char)
                            != 0 as ::core::ffi::c_int)
                {
                    let mut rej: *mut ::core::ffi::c_char = rejname;
                    if rejname.is_null() {
                        let mut s: *const ::core::ffi::c_char = simple_backup_suffix;
                        simple_backup_suffix = b".rej\0".as_ptr() as *const ::core::ffi::c_char;
                        rej = find_backup_file_name(AT_FDCWD, outname, backup_type::simple_backups);
                        let mut len: idx_t = strlen(rej) as idx_t;
                        if *rej.offset((len - 1 as idx_t) as isize) as ::core::ffi::c_int
                            == '~' as ::core::ffi::c_int
                        {
                            *rej.offset((len - 1 as idx_t) as isize) = '#' as ::core::ffi::c_char;
                        }
                        simple_backup_suffix = s;
                    }
                    if !dry_run {
                        say(
                            b" -- saving rejects to file %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            quotearg(rej),
                        );
                        if !rejname.is_null() {
                            if ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(
                                &raw const outrej.exists,
                            )
                            .is_null()
                            {
                                copy_file(
                                    tmprej.name,
                                    ::core::ptr::null::<stat>(),
                                    &raw mut outrej,
                                    ::core::ptr::null_mut::<stat>(),
                                    0 as ::core::ffi::c_int,
                                    (S_IFREG | 0o666 as ::core::ffi::c_int) as mode_t,
                                    file_attributes(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
                                    r#true != 0,
                                );
                            } else {
                                append_to_file(tmprej.name, rejname);
                            }
                        } else {
                            let mut oldst: stat = stat {
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
                            let mut olderrno: ::core::ffi::c_int = 0;
                            olderrno = stat_file(rej, &raw mut oldst);
                            if olderrno != 0 && olderrno != ENOENT {
                                write_fatal();
                            }
                            if olderrno == 0
                                && lookup_file_id(&raw mut oldst).0 == file_id_type::CREATED.0
                            {
                                append_to_file(tmprej.name, rej);
                            } else {
                                move_file(
                                    &raw mut tmprej,
                                    &raw mut rejst,
                                    rej,
                                    (S_IFREG | 0o666 as ::core::ffi::c_int) as mode_t,
                                    r#false != 0,
                                );
                            }
                        }
                    } else {
                        say(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    if rejname.is_null() {
                        free(rej as *mut ::core::ffi::c_void);
                    }
                } else {
                    say(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                }
            }
        }
        reinitialize_almost_everything();
        skip_reject_file = r#false != 0;
        apply_empty_patch = r#false != 0;
    }
    if !outstate.ofp.is_null() {
        Fclose(outstate.ofp);
    }
    defer_signals();
    cleanup_remove();
    undefer_signals();
    output_files(::core::ptr::null::<stat>(), 1 as ::core::ffi::c_int);
    delete_files();
    return if somefailed as ::core::ffi::c_int != 0 {
        EXIT_FAILURE
    } else {
        EXIT_SUCCESS
    };
}
unsafe extern "C" fn reinitialize_almost_everything() {
    re_patch();
    re_input();
    input_lines = 0 as idx_t;
    last_frozen_line = 0 as idx_t;
    if !inname.is_null() && !explicit_inname {
        free(inname as *mut ::core::ffi::c_void);
        inname = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    in_offset = 0 as ptrdiff_t;
    out_offset = 0 as ptrdiff_t;
    diff_type = diff::NO_DIFF;
    if !revision.is_null() {
        free(revision as *mut ::core::ffi::c_void);
        revision = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    reverse_flag = reverse_flag_specified;
    skip_rest_of_patch = r#false != 0;
}
static mut shortopts: [::core::ffi::c_char; 42] = unsafe {
    ::core::mem::transmute::<[u8; 42], [::core::ffi::c_char; 42]>(
        *b"bB:cd:D:eEfF:g:i:lnNo:p:r:RstTuvV:x:Y:z:Z\0",
    )
};
static mut longopts: [option; 42] = [
    option {
        name: b"backup\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'b' as ::core::ffi::c_int,
    },
    option {
        name: b"prefix\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'B' as ::core::ffi::c_int,
    },
    option {
        name: b"context\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'c' as ::core::ffi::c_int,
    },
    option {
        name: b"directory\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"ifdef\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'D' as ::core::ffi::c_int,
    },
    option {
        name: b"ed\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'e' as ::core::ffi::c_int,
    },
    option {
        name: b"remove-empty-files\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'E' as ::core::ffi::c_int,
    },
    option {
        name: b"force\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'f' as ::core::ffi::c_int,
    },
    option {
        name: b"fuzz\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'F' as ::core::ffi::c_int,
    },
    option {
        name: b"get\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'g' as ::core::ffi::c_int,
    },
    option {
        name: b"input\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'i' as ::core::ffi::c_int,
    },
    option {
        name: b"ignore-whitespace\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"merge\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'm' as ::core::ffi::c_int,
    },
    option {
        name: b"normal\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"forward\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'N' as ::core::ffi::c_int,
    },
    option {
        name: b"output\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'o' as ::core::ffi::c_int,
    },
    option {
        name: b"strip\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"reject-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"reverse\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'R' as ::core::ffi::c_int,
    },
    option {
        name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"batch\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 't' as ::core::ffi::c_int,
    },
    option {
        name: b"set-time\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'T' as ::core::ffi::c_int,
    },
    option {
        name: b"unified\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'u' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"version-control\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'V' as ::core::ffi::c_int,
    },
    option {
        name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'x' as ::core::ffi::c_int,
    },
    option {
        name: b"basename-prefix\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'Y' as ::core::ffi::c_int,
    },
    option {
        name: b"suffix\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'z' as ::core::ffi::c_int,
    },
    option {
        name: b"set-utc\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'Z' as ::core::ffi::c_int,
    },
    option {
        name: b"dry-run\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 1 as ::core::ffi::c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 2 as ::core::ffi::c_int,
    },
    option {
        name: b"binary\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 3 as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 4 as ::core::ffi::c_int,
    },
    option {
        name: b"backup-if-mismatch\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 5 as ::core::ffi::c_int,
    },
    option {
        name: b"no-backup-if-mismatch\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 6 as ::core::ffi::c_int,
    },
    option {
        name: b"posix\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 7 as ::core::ffi::c_int,
    },
    option {
        name: b"quoting-style\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 8 as ::core::ffi::c_int,
    },
    option {
        name: b"reject-format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 9 as ::core::ffi::c_int,
    },
    option {
        name: b"read-only\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 10 as ::core::ffi::c_int,
    },
    option {
        name: b"follow-symlinks\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: CHAR_MAX + 11 as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
static mut option_help: [*const ::core::ffi::c_char; 66] = [
    b"Input options:\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -p NUM  --strip=NUM  Strip NUM leading components from file names.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -F LINES  --fuzz LINES  Set the fuzz factor to LINES for inexact matching.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -l  --ignore-whitespace  Ignore white space changes between patch and input.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -c  --context  Interpret the patch as a context difference.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -e  --ed  Interpret the patch as an ed script.\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -n  --normal  Interpret the patch as a normal difference.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -u  --unified  Interpret the patch as a unified difference.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -N  --forward  Ignore patches that appear to be reversed or already applied.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -R  --reverse  Assume patches were created with old and new files swapped.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -i PATCHFILE  --input=PATCHFILE  Read patch from PATCHFILE instead of stdin.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"Output options:\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -o FILE  --output=FILE  Output patched files to FILE.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -r FILE  --reject-file=FILE  Output rejects to FILE.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -D NAME  --ifdef=NAME  Make merged if-then-else output using NAME.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --merge  Merge using conflict markers instead of creating reject files.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -E  --remove-empty-files  Remove output files that are empty after patching.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -Z  --set-utc  Set times of patched files, assuming diff uses UTC (GMT).\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -T  --set-time  Likewise, assuming local time.\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  --quoting-style=WORD   output file names using quoting style WORD.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    Valid WORDs are: literal, shell, shell-always, c, escape.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"    Default is taken from QUOTING_STYLE env variable, or 'shell' if unset.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"Backup and version control options:\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -b  --backup  Back up the original contents of each file.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --backup-if-mismatch  Back up if the patch does not match exactly.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --no-backup-if-mismatch  Back up mismatches only if otherwise requested.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -V STYLE  --version-control=STYLE  Use STYLE version control.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\tSTYLE is either 'simple', 'numbered', or 'existing'.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -B PREFIX  --prefix=PREFIX  Prepend PREFIX to backup file names.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -Y PREFIX  --basename-prefix=PREFIX  Prepend PREFIX to backup file basenames.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -z SUFFIX  --suffix=SUFFIX  Append SUFFIX to backup file names.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -g NUM  --get=NUM  Get files from RCS etc. if positive; ask if negative.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"Miscellaneous options:\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -t  --batch  Ask no questions; skip bad-Prereq patches; assume reversed.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -f  --force  Like -t, but ignore bad-Prereq patches, and assume unreversed.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  -s  --quiet  --silent  Work silently unless an error occurs.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --verbose  Output extra information about the work being done.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --dry-run  Do not actually change any files; just print what would happen.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --posix  Conform to the POSIX standard.\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -d DIR  --directory=DIR  Change the working directory to DIR first.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --reject-format=FORMAT  Create 'context' or 'unified' rejects.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"  --binary  Read and write data in binary mode.\0".as_ptr() as *const ::core::ffi::c_char,
    b"  --read-only=BEHAVIOR  How to handle read-only input files: 'ignore' that they\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"                        are read-only, 'warn' (default), or 'fail'.\0".as_ptr()
        as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"  -v  --version  Output version info.\0".as_ptr() as *const ::core::ffi::c_char,
    b"  --help  Output this help.\0".as_ptr() as *const ::core::ffi::c_char,
    b"\0".as_ptr() as *const ::core::ffi::c_char,
    b"Report bugs to <bug-patch@gnu.org>.\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
unsafe extern "C" fn usage(mut stream: *mut FILE, mut status: ::core::ffi::c_int) {
    let mut p: *const *const ::core::ffi::c_char =
        ::core::ptr::null::<*const ::core::ffi::c_char>();
    if status != EXIT_SUCCESS {
        Fprintf(
            stream,
            b"%s: Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
            program_name,
        );
    } else {
        Fprintf(
            stream,
            b"Usage: %s [OPTION]... [ORIGFILE [PATCHFILE]]\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            program_name,
        );
        p = &raw const option_help as *const *const ::core::ffi::c_char;
        while !(*p).is_null() {
            Fprintf(stream, b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, *p);
            p = p.offset(1);
        }
    }
    exit(status);
}
unsafe extern "C" fn backup_file_name_option(
    mut option_type: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    if *optarg == 0 {
        fatal(
            b"backup %s is empty\0".as_ptr() as *const ::core::ffi::c_char,
            option_type,
        );
    }
    return xstrdup(optarg);
}
unsafe extern "C" fn get_some_switches(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut optc: ::core::ffi::c_int = 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            &raw const shortopts as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if 0 as ::core::ffi::c_int > optc {
            break;
        }
        's_434: {
            '_case_z: {
                match optc {
                    98 => {
                        make_backups = r#true != 0;
                        if argc - optind == 3 as ::core::ffi::c_int
                            && strcmp(
                                *argv.offset((optind - 1 as ::core::ffi::c_int) as isize),
                                b"-b\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            && !(*(*argv.offset((optind + 0 as ::core::ffi::c_int) as isize))
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == '-' as ::core::ffi::c_int
                                && *(*argv.offset((optind + 0 as ::core::ffi::c_int) as isize))
                                    .offset(1isize)
                                    as ::core::ffi::c_int
                                    != 0)
                            && !(*(*argv.offset((optind + 1 as ::core::ffi::c_int) as isize))
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == '-' as ::core::ffi::c_int
                                && *(*argv.offset((optind + 1 as ::core::ffi::c_int) as isize))
                                    .offset(1isize)
                                    as ::core::ffi::c_int
                                    != 0)
                            && !(*(*argv.offset((optind + 2 as ::core::ffi::c_int) as isize))
                                .offset(0isize)
                                as ::core::ffi::c_int
                                == '-' as ::core::ffi::c_int
                                && *(*argv.offset((optind + 2 as ::core::ffi::c_int) as isize))
                                    .offset(1isize)
                                    as ::core::ffi::c_int
                                    != 0)
                        {
                            let c2rust_fresh2 = optind;
                            optind += 1;
                            optarg = *argv.offset(c2rust_fresh2 as isize);
                            if verbosity_0.0 != verbosity::SILENT.0 {
                                say(
                                    b"warning: the '-b %s' option is obsolete; use '-b -z %s' instead\n\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    optarg,
                                    optarg,
                                );
                            }
                            break '_case_z;
                        } else {
                            break 's_434;
                        }
                    }
                    66 => {
                        origprae = backup_file_name_option(
                            b"prefix\0".as_ptr() as *const ::core::ffi::c_char
                        );
                        break 's_434;
                    }
                    99 => {
                        diff_type = diff::CONTEXT_DIFF;
                        break 's_434;
                    }
                    100 => {
                        if chdir(optarg) < 0 as ::core::ffi::c_int {
                            pfatal(
                                b"Can't change to directory %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                quotearg(optarg),
                            );
                        }
                        break 's_434;
                    }
                    68 => {
                        do_defines = xstrdup(optarg);
                        break 's_434;
                    }
                    101 => {
                        diff_type = diff::ED_DIFF;
                        break 's_434;
                    }
                    69 => {
                        remove_empty_files = r#true != 0;
                        break 's_434;
                    }
                    102 => {
                        force = r#true != 0;
                        break 's_434;
                    }
                    70 => {
                        maxfuzz = numeric_string(
                            optarg,
                            r#false != 0,
                            b"fuzz factor\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        break 's_434;
                    }
                    103 => {
                        patch_get = numeric_string(
                            optarg,
                            r#true != 0,
                            b"get option value\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        break 's_434;
                    }
                    105 => {
                        patchname = xstrdup(optarg);
                        break 's_434;
                    }
                    108 => {
                        canonicalize_ws = r#true != 0;
                        break 's_434;
                    }
                    109 => {
                        merge = r#true != 0;
                        if !optarg.is_null() {
                            if strcmp(optarg, b"merge\0".as_ptr() as *const ::core::ffi::c_char)
                                == 0
                            {
                                conflict_style_0 = conflict_style::MERGE_MERGE;
                            } else if strcmp(
                                optarg,
                                b"diff3\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0
                            {
                                conflict_style_0 = conflict_style::MERGE_DIFF3;
                            } else {
                                usage(stderr, C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
                            }
                        } else {
                            conflict_style_0 = conflict_style::MERGE_MERGE;
                        }
                        break 's_434;
                    }
                    110 => {
                        diff_type = diff::NORMAL_DIFF;
                        break 's_434;
                    }
                    78 => {
                        noreverse_flag = r#true != 0;
                        break 's_434;
                    }
                    111 => {
                        outfile = xstrdup(optarg);
                        break 's_434;
                    }
                    112 => {
                        strippath = numeric_string(
                            optarg,
                            r#false != 0,
                            b"strip count\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        break 's_434;
                    }
                    114 => {
                        outrej.name = xstrdup(optarg);
                        break 's_434;
                    }
                    82 => {
                        reverse_flag = r#true != 0;
                        reverse_flag_specified = r#true != 0;
                        break 's_434;
                    }
                    115 => {
                        verbosity_0 = verbosity::SILENT;
                        break 's_434;
                    }
                    116 => {
                        batch = r#true != 0;
                        break 's_434;
                    }
                    84 => {
                        set_time = r#true != 0;
                        break 's_434;
                    }
                    117 => {
                        diff_type = diff::UNI_DIFF;
                        break 's_434;
                    }
                    118 => {
                        version();
                        exit(EXIT_SUCCESS);
                    }
                    86 => {
                        version_control = optarg;
                        version_control_context = b"--version-control or -V option\0".as_ptr()
                            as *const ::core::ffi::c_char;
                        break 's_434;
                    }
                    120 => {
                        debug = numeric_string(
                            optarg,
                            r#true != 0,
                            b"debugging option\0".as_ptr() as *const ::core::ffi::c_char,
                        ) as ::core::ffi::c_ushort;
                        break 's_434;
                    }
                    89 => {
                        origbase = backup_file_name_option(
                            b"basename prefix\0".as_ptr() as *const ::core::ffi::c_char
                        );
                        break 's_434;
                    }
                    122 => {
                        break '_case_z;
                    }
                    90 => {
                        set_utc = r#true != 0;
                        break 's_434;
                    }
                    128 => {
                        dry_run = r#true != 0;
                        break 's_434;
                    }
                    129 => {
                        verbosity_0 = verbosity::VERBOSE;
                        break 's_434;
                    }
                    130 => {
                        no_strip_trailing_cr = r#true != 0;
                        break 's_434;
                    }
                    131 => {
                        usage(stdout, EXIT_SUCCESS);
                    }
                    132 => {}
                    133 => {
                        backup_if_mismatch = r#false != 0;
                        backup_if_mismatch_specified = r#true != 0;
                        break 's_434;
                    }
                    134 => {
                        posixly_correct = r#true != 0;
                        break 's_434;
                    }
                    135 => {
                        let mut i: ::core::ffi::c_int = argmatch(
                            optarg,
                            &raw const quoting_style_args as *const *const ::core::ffi::c_char,
                            ::core::ptr::null::<::core::ffi::c_void>(),
                            0 as size_t,
                        )
                            as ::core::ffi::c_int;
                        if i < 0 as ::core::ffi::c_int {
                            argmatch_invalid(
                                b"quoting style\0".as_ptr() as *const ::core::ffi::c_char,
                                optarg,
                                i as ptrdiff_t,
                            );
                            usage(stderr, C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
                        }
                        set_quoting_style(
                            ::core::ptr::null_mut::<quoting_options>(),
                            quoting_style(i as ::core::ffi::c_uint),
                        );
                        break 's_434;
                    }
                    136 => {
                        if strcmp(optarg, b"context\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                        {
                            reject_format = diff::NEW_CONTEXT_DIFF;
                        } else if strcmp(
                            optarg,
                            b"unified\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                        {
                            reject_format = diff::UNI_DIFF;
                        } else {
                            usage(stderr, C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
                        }
                        break 's_434;
                    }
                    137 => {
                        if strcmp(optarg, b"ignore\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                        {
                            read_only_behavior = C2Rust_Unnamed_1::RO_IGNORE;
                        } else if strcmp(optarg, b"warn\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                        {
                            read_only_behavior = C2Rust_Unnamed_1::RO_WARN;
                        } else if strcmp(optarg, b"fail\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                        {
                            read_only_behavior = C2Rust_Unnamed_1::RO_FAIL;
                        } else {
                            usage(stderr, C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
                        }
                        break 's_434;
                    }
                    138 => {
                        follow_symlinks = r#true != 0;
                        break 's_434;
                    }
                    _ => {
                        usage(stderr, C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
                        break 's_434;
                    }
                }
                backup_if_mismatch = r#true != 0;
                backup_if_mismatch_specified = r#true != 0;
                break 's_434;
            }
            origsuff = backup_file_name_option(b"suffix\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    if optind < argc {
        let c2rust_fresh3 = optind;
        optind += 1;
        inname = xstrdup(*argv.offset(c2rust_fresh3 as isize));
        explicit_inname = r#true != 0;
        invc = -1 as ::core::ffi::c_schar;
        if optind < argc {
            let c2rust_fresh4 = optind;
            optind += 1;
            patchname = xstrdup(*argv.offset(c2rust_fresh4 as isize));
            if optind < argc {
                Fprintf(
                    stderr,
                    b"%s: %s: extra operand\n\0".as_ptr() as *const ::core::ffi::c_char,
                    program_name,
                    quotearg(*argv.offset(optind as isize)),
                );
                usage(stderr, C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
            }
        }
    }
}
unsafe extern "C" fn numeric_string(
    mut string: *const ::core::ffi::c_char,
    mut negative_allowed: bool,
    mut argtype_msgid: *const ::core::ffi::c_char,
) -> intmax_t {
    let mut value: intmax_t = 0 as intmax_t;
    let mut p: *const ::core::ffi::c_char = string;
    let mut negative: bool = *p as ::core::ffi::c_int == '-' as ::core::ffi::c_int;
    let mut overflow: bool = r#false != 0;
    p = p.offset(
        (negative as ::core::ffi::c_int != 0
            || *p as ::core::ffi::c_int == '+' as ::core::ffi::c_int) as ::core::ffi::c_int
            as isize,
    );
    loop {
        if !c_isdigit(*p as ::core::ffi::c_int) {
            fatal(
                b"%s %s is not a number\0".as_ptr() as *const ::core::ffi::c_char,
                argtype_msgid,
                quotearg(string),
            );
        }
        let (c2rust_result, c2rust_overflowed) =
            (value as i128).overflowing_mul(10 as ::core::ffi::c_int as i128);
        let c2rust_result_narrow = c2rust_result as intmax_t;
        *&raw mut value = c2rust_result_narrow;
        overflow = overflow as ::core::ffi::c_int
            | (c2rust_overflowed || c2rust_result_narrow as i128 != c2rust_result)
                as ::core::ffi::c_int
            != 0;
        let (c2rust_result_0, c2rust_overflowed_0) = (value as i128).overflowing_add(
            (if negative as ::core::ffi::c_int != 0 {
                '0' as ::core::ffi::c_int - *p as ::core::ffi::c_int
            } else {
                *p as ::core::ffi::c_int - '0' as ::core::ffi::c_int
            }) as i128,
        );
        let c2rust_result_narrow_0 = c2rust_result_0 as intmax_t;
        *&raw mut value = c2rust_result_narrow_0;
        overflow = overflow as ::core::ffi::c_int
            | (c2rust_overflowed_0 || c2rust_result_narrow_0 as i128 != c2rust_result_0)
                as ::core::ffi::c_int
            != 0;
        p = p.offset(1);
        if *p == 0 {
            break;
        }
    }
    if value < 0 as intmax_t && !negative_allowed {
        fatal(
            b"%s %s is negative\0".as_ptr() as *const ::core::ffi::c_char,
            argtype_msgid,
            quotearg(string),
        );
    }
    return if !overflow {
        value
    } else if negative as ::core::ffi::c_int != 0 {
        INTMAX_MIN as intmax_t
    } else {
        INTMAX_MAX as intmax_t
    };
}
unsafe extern "C" fn locate_hunk(mut fuzz: idx_t) -> idx_t {
    let mut first_guess: idx_t = pch_first() + in_offset;
    let mut pat_lines: idx_t = pch_ptrn_lines();
    let mut prefix_context: idx_t = pch_prefix_context();
    let mut suffix_context: idx_t = pch_suffix_context();
    let mut context: idx_t = if prefix_context > suffix_context {
        prefix_context
    } else {
        suffix_context
    };
    let mut prefix_fuzz: ptrdiff_t = fuzz + prefix_context - context;
    let mut suffix_fuzz: ptrdiff_t = fuzz + suffix_context - context;
    let mut max_where: idx_t = input_lines - (pat_lines - suffix_fuzz) + 1 as idx_t;
    let mut min_where: idx_t = last_frozen_line + 1 as idx_t;
    let mut max_pos_offset: ptrdiff_t = max_where - first_guess;
    let mut max_neg_offset: ptrdiff_t = first_guess - min_where;
    let mut max_offset: ptrdiff_t = if max_pos_offset > max_neg_offset {
        max_pos_offset
    } else {
        max_neg_offset
    };
    let mut min_offset: ptrdiff_t = 0;
    if pat_lines == 0 {
        return first_guess;
    }
    if first_guess <= max_neg_offset {
        max_neg_offset = (first_guess - 1 as idx_t) as ptrdiff_t;
    }
    if prefix_fuzz < 0 as ptrdiff_t && pch_first() <= 1 as idx_t {
        if suffix_fuzz < 0 as ptrdiff_t {
            if pat_lines != input_lines || prefix_context < last_frozen_line {
                return 0 as idx_t;
            }
        }
        let mut offset: ptrdiff_t = 1 as ptrdiff_t - first_guess;
        if last_frozen_line <= prefix_context
            && offset <= max_pos_offset
            && patch_match(first_guess, offset, 0 as idx_t, suffix_fuzz) as ::core::ffi::c_int != 0
        {
            in_offset += offset;
            return first_guess + offset;
        } else {
            return 0 as idx_t;
        }
    } else if prefix_fuzz < 0 as ptrdiff_t {
        prefix_fuzz = 0 as ptrdiff_t;
    }
    if suffix_fuzz < 0 as ptrdiff_t {
        let mut offset_0: ptrdiff_t = first_guess - (input_lines - pat_lines + 1 as ptrdiff_t);
        if offset_0 <= max_neg_offset
            && patch_match(first_guess, -offset_0, prefix_fuzz, 0 as idx_t) as ::core::ffi::c_int
                != 0
        {
            in_offset -= offset_0;
            return first_guess - offset_0;
        } else {
            return 0 as idx_t;
        }
    }
    min_offset = (if max_pos_offset < 0 as ptrdiff_t {
        first_guess - max_where
    } else if max_neg_offset < 0 as ptrdiff_t {
        first_guess - min_where
    } else {
        0 as idx_t
    }) as ptrdiff_t;
    let mut offset_1: ptrdiff_t = min_offset;
    while offset_1 <= max_offset {
        if offset_1 <= max_pos_offset
            && patch_match(first_guess, offset_1, prefix_fuzz, suffix_fuzz) as ::core::ffi::c_int
                != 0
        {
            if debug as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                say(
                    b"Offset changing from %td to %td\n\0".as_ptr() as *const ::core::ffi::c_char,
                    in_offset,
                    in_offset + offset_1,
                );
            }
            in_offset += offset_1;
            return first_guess + offset_1;
        }
        if offset_1 <= max_neg_offset
            && patch_match(first_guess, -offset_1, prefix_fuzz, suffix_fuzz) as ::core::ffi::c_int
                != 0
        {
            if debug as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                say(
                    b"Offset changing from %td to %td\n\0".as_ptr() as *const ::core::ffi::c_char,
                    in_offset,
                    in_offset - offset_1,
                );
            }
            in_offset -= offset_1;
            return first_guess - offset_1;
        }
        offset_1 += 1;
    }
    return 0 as idx_t;
}
unsafe extern "C" fn mangled_patch(mut old: idx_t, mut new: idx_t) {
    if debug as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
        say(
            b"oldchar = '%c', newchar = '%c'\n\0".as_ptr() as *const ::core::ffi::c_char,
            pch_char(old) as ::core::ffi::c_int,
            pch_char(new) as ::core::ffi::c_int,
        );
    }
    fatal(
        b"Out-of-sync patch, lines %td,%td -- mangled text or line numbers, maybe?\0".as_ptr()
            as *const ::core::ffi::c_char,
        pch_hunk_beg() + old,
        pch_hunk_beg() + new,
    );
}
unsafe extern "C" fn print_unidiff_range(mut fp: *mut FILE, mut start: idx_t, mut count: idx_t) {
    match count {
        0 => {
            Fprintf(
                fp,
                b"%td,0\0".as_ptr() as *const ::core::ffi::c_char,
                start - 1 as idx_t,
            );
        }
        1 => {
            Fprintf(fp, b"%td\0".as_ptr() as *const ::core::ffi::c_char, start);
        }
        _ => {
            Fprintf(
                fp,
                b"%td,%td\0".as_ptr() as *const ::core::ffi::c_char,
                start,
                count,
            );
        }
    };
}
unsafe extern "C" fn print_header_line(
    mut fp: *mut FILE,
    mut tag: *const ::core::ffi::c_char,
    mut reverse: bool,
) {
    let mut name: *const ::core::ffi::c_char = pch_name(nametype(reverse as ::core::ffi::c_uint));
    let mut timestr: *const ::core::ffi::c_char = pch_timestr(reverse);
    putline(
        fp,
        tag,
        if !name.is_null() {
            name
        } else {
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char
        },
        timestr,
        nullptr,
    );
}
unsafe extern "C" fn abort_hunk_unified(mut header: bool, mut reverse: bool) {
    let mut old: idx_t = 1 as idx_t;
    let mut lastline: idx_t = pch_ptrn_lines();
    let mut new: idx_t = lastline + 1 as idx_t;
    let mut c_function: *const ::core::ffi::c_char = pch_c_function();
    if header {
        if !pch_name(nametype::INDEX).is_null() {
            putline(
                rejfp,
                b"Index: \0".as_ptr() as *const ::core::ffi::c_char,
                pch_name(nametype::INDEX),
                nullptr,
            );
        }
        print_header_line(
            rejfp,
            b"--- \0".as_ptr() as *const ::core::ffi::c_char,
            reverse,
        );
        print_header_line(
            rejfp,
            b"+++ \0".as_ptr() as *const ::core::ffi::c_char,
            !reverse,
        );
    }
    Fputs(b"@@ -\0".as_ptr() as *const ::core::ffi::c_char, rejfp);
    print_unidiff_range(rejfp, pch_first() + out_offset, lastline);
    Fputs(b" +\0".as_ptr() as *const ::core::ffi::c_char, rejfp);
    print_unidiff_range(rejfp, pch_newfirst() + out_offset, pch_repl_lines());
    putline(
        rejfp,
        b" @@\0".as_ptr() as *const ::core::ffi::c_char,
        c_function,
        nullptr,
    );
    while pch_char(new) as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        || pch_char(new) as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
    {
        new += 1;
    }
    if diff_type.0 != diff::UNI_DIFF.0 {
        pch_normalize(diff::UNI_DIFF);
    }
    loop {
        while pch_char(old) as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
            Fputc('-' as ::core::ffi::c_int, rejfp);
            pch_write_line(old, rejfp);
            old += 1;
        }
        while pch_char(new) as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
            Fputc('+' as ::core::ffi::c_int, rejfp);
            pch_write_line(new, rejfp);
            new += 1;
        }
        if old > lastline {
            break;
        }
        if pch_char(new) as ::core::ffi::c_int != pch_char(old) as ::core::ffi::c_int {
            mangled_patch(old, new);
        }
        Fputc(' ' as ::core::ffi::c_int, rejfp);
        pch_write_line(old, rejfp);
        old += 1;
        new += 1;
    }
    if pch_char(new) as ::core::ffi::c_int != '^' as ::core::ffi::c_int {
        mangled_patch(old, new);
    }
}
unsafe extern "C" fn abort_hunk_context(mut header: bool, mut reverse: bool) {
    let mut pat_end: idx_t = pch_end();
    let mut oldfirst: idx_t = pch_first() + out_offset;
    let mut newfirst: idx_t = pch_newfirst() + out_offset;
    let mut oldlast: idx_t = oldfirst + pch_ptrn_lines() - 1 as idx_t;
    let mut newlast: idx_t = newfirst + pch_repl_lines() - 1 as idx_t;
    let mut stars: *const ::core::ffi::c_char = if diff_type.0 < diff::NEW_CONTEXT_DIFF.0 {
        b"\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b" ****\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut minuses: *const ::core::ffi::c_char = if diff_type.0 < diff::NEW_CONTEXT_DIFF.0 {
        b" -----\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b" ----\0".as_ptr() as *const ::core::ffi::c_char
    };
    let mut c_function: *const ::core::ffi::c_char = pch_c_function();
    if diff_type.0 == diff::UNI_DIFF.0 {
        pch_normalize(diff::NEW_CONTEXT_DIFF);
    }
    if header {
        if !pch_name(nametype::INDEX).is_null() {
            putline(
                rejfp,
                b"Index: \0".as_ptr() as *const ::core::ffi::c_char,
                pch_name(nametype::INDEX),
                nullptr,
            );
        }
        print_header_line(
            rejfp,
            b"*** \0".as_ptr() as *const ::core::ffi::c_char,
            reverse,
        );
        print_header_line(
            rejfp,
            b"--- \0".as_ptr() as *const ::core::ffi::c_char,
            !reverse,
        );
    }
    putline(
        rejfp,
        b"***************\0".as_ptr() as *const ::core::ffi::c_char,
        c_function,
        nullptr,
    );
    let mut i: idx_t = 0 as idx_t;
    while i <= pat_end {
        's_140: {
            match pch_char(i) as ::core::ffi::c_int {
                42 => {
                    if oldlast < oldfirst {
                        Fprintf(
                            rejfp,
                            b"*** 0%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            stars,
                        );
                    } else if oldlast == oldfirst {
                        Fprintf(
                            rejfp,
                            b"*** %td%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            oldfirst,
                            stars,
                        );
                    } else {
                        Fprintf(
                            rejfp,
                            b"*** %td,%td%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            oldfirst,
                            oldlast,
                            stars,
                        );
                    }
                    break 's_140;
                }
                61 => {
                    if newlast < newfirst {
                        Fprintf(
                            rejfp,
                            b"--- 0%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            minuses,
                        );
                    } else if newlast == newfirst {
                        Fprintf(
                            rejfp,
                            b"--- %td%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            newfirst,
                            minuses,
                        );
                    } else {
                        Fprintf(
                            rejfp,
                            b"--- %td,%td%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                            newfirst,
                            newlast,
                            minuses,
                        );
                    }
                    break 's_140;
                }
                32 | 45 | 43 | 33 => {
                    Fprintf(
                        rejfp,
                        b"%c \0".as_ptr() as *const ::core::ffi::c_char,
                        pch_char(i) as ::core::ffi::c_int,
                    );
                }
                10 => {}
                _ => {
                    fatal(b"fatal internal error in abort_hunk_context\0".as_ptr()
                        as *const ::core::ffi::c_char);
                    break 's_140;
                }
            }
            pch_write_line(i, rejfp);
        }
        if ferror(rejfp) != 0 {
            write_fatal();
        }
        i += 1;
    }
}
unsafe extern "C" fn abort_hunk(
    mut outname: *const ::core::ffi::c_char,
    mut header: bool,
    mut reverse: bool,
) {
    if ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const tmprej.exists).is_null() {
        init_reject(outname);
    }
    if reject_format.0 == diff::UNI_DIFF.0
        || reject_format.0 == diff::NO_DIFF.0 && diff_type.0 == diff::UNI_DIFF.0
    {
        abort_hunk_unified(header, reverse);
    } else {
        abort_hunk_context(header, reverse);
    };
}
unsafe extern "C" fn apply_hunk(mut outstate: *mut outstate, mut r#where: idx_t) -> bool {
    let mut old: idx_t = 1 as idx_t;
    let mut lastline: idx_t = pch_ptrn_lines();
    let mut new: idx_t = lastline + 1 as idx_t;
    let mut def_state: C2Rust_Unnamed_0 = C2Rust_Unnamed_0::OUTSIDE;
    let mut R_do_defines: *const ::core::ffi::c_char = do_defines;
    let mut pat_end: idx_t = pch_end();
    let mut fp: *mut FILE = (*outstate).ofp;
    r#where -= 1;
    while pch_char(new) as ::core::ffi::c_int == '=' as ::core::ffi::c_int
        || pch_char(new) as ::core::ffi::c_int == '\n' as ::core::ffi::c_int
    {
        new += 1;
    }
    while old <= lastline {
        if pch_char(old) as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
            '_c2rust_label: {
                if (*outstate).after_newline {
                } else {
                    __assert_fail(
                        b"outstate->after_newline\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/patch-2.8/src/patch.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1426 as ::core::ffi::c_uint,
                        b"_Bool apply_hunk(struct outstate *, idx_t)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            if !copy_till(outstate, r#where + old - 1 as idx_t) {
                return r#false != 0;
            }
            if !R_do_defines.is_null() {
                if def_state.0 == C2Rust_Unnamed_0::OUTSIDE.0 {
                    putline(
                        fp,
                        (&raw const not_defined as *const ::core::ffi::c_char)
                            .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                        R_do_defines,
                        nullptr,
                    );
                    def_state = C2Rust_Unnamed_0::IN_IFNDEF;
                } else if def_state.0 == C2Rust_Unnamed_0::IN_IFDEF.0 {
                    Fputs(
                        (&raw const else_defined as *const ::core::ffi::c_char)
                            .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                        fp,
                    );
                    def_state = C2Rust_Unnamed_0::IN_ELSE;
                }
                if ferror(fp) != 0 {
                    write_fatal();
                }
                (*outstate).after_newline = pch_write_line(old, fp);
                (*outstate).zero_output = r#false != 0;
            }
            last_frozen_line += 1;
            old += 1;
        } else {
            if new > pat_end {
                break;
            }
            if pch_char(new) as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
                if !copy_till(outstate, r#where + old - 1 as idx_t) {
                    return r#false != 0;
                }
                if !R_do_defines.is_null() {
                    if def_state.0 == C2Rust_Unnamed_0::IN_IFNDEF.0 {
                        Fputs(
                            (&raw const else_defined as *const ::core::ffi::c_char)
                                .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                            fp,
                        );
                        def_state = C2Rust_Unnamed_0::IN_ELSE;
                    } else if def_state.0 == C2Rust_Unnamed_0::OUTSIDE.0 {
                        putline(
                            fp,
                            (&raw const if_defined as *const ::core::ffi::c_char)
                                .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                            R_do_defines,
                            nullptr,
                        );
                        def_state = C2Rust_Unnamed_0::IN_IFDEF;
                    }
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                }
                (*outstate).after_newline = pch_write_line(new, fp);
                (*outstate).zero_output = r#false != 0;
                new += 1;
            } else if pch_char(new) as ::core::ffi::c_int != pch_char(old) as ::core::ffi::c_int {
                mangled_patch(old, new);
            } else if pch_char(new) as ::core::ffi::c_int == '!' as ::core::ffi::c_int {
                '_c2rust_label_0: {
                    if (*outstate).after_newline {
                    } else {
                        __assert_fail(
                            b"outstate->after_newline\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/opt/src/patch-2.8/src/patch.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1473 as ::core::ffi::c_uint,
                            b"_Bool apply_hunk(struct outstate *, idx_t)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                if !copy_till(outstate, r#where + old - 1 as idx_t) {
                    return r#false != 0;
                }
                '_c2rust_label_1: {
                    if (*outstate).after_newline {
                    } else {
                        __assert_fail(
                            b"outstate->after_newline\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/opt/src/patch-2.8/src/patch.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1476 as ::core::ffi::c_uint,
                            b"_Bool apply_hunk(struct outstate *, idx_t)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                if !R_do_defines.is_null() {
                    putline(
                        fp,
                        (&raw const not_defined as *const ::core::ffi::c_char)
                            .offset(1 as ::core::ffi::c_int as isize),
                        R_do_defines,
                        nullptr,
                    );
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                    def_state = C2Rust_Unnamed_0::IN_IFNDEF;
                }
                loop {
                    if !R_do_defines.is_null() {
                        (*outstate).after_newline = pch_write_line(old, fp);
                    }
                    last_frozen_line += 1;
                    old += 1;
                    if pch_char(old) as ::core::ffi::c_int != '!' as ::core::ffi::c_int {
                        break;
                    }
                }
                if !R_do_defines.is_null() {
                    Fputs(
                        (&raw const else_defined as *const ::core::ffi::c_char)
                            .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                        fp,
                    );
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                    def_state = C2Rust_Unnamed_0::IN_ELSE;
                }
                loop {
                    (*outstate).after_newline = pch_write_line(new, fp);
                    new += 1;
                    if pch_char(new) as ::core::ffi::c_int != '!' as ::core::ffi::c_int {
                        break;
                    }
                }
                (*outstate).zero_output = r#false != 0;
            } else {
                '_c2rust_label_2: {
                    if pch_char(new) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int {
                    } else {
                        __assert_fail(
                            b"pch_char(new) == ' '\0".as_ptr() as *const ::core::ffi::c_char,
                            b"/opt/src/patch-2.8/src/patch.c\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            1510 as ::core::ffi::c_uint,
                            b"_Bool apply_hunk(struct outstate *, idx_t)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                    }
                };
                old += 1;
                new += 1;
                if !R_do_defines.is_null() && def_state.0 != C2Rust_Unnamed_0::OUTSIDE.0 {
                    Fputs(
                        (&raw const end_defined as *const ::core::ffi::c_char)
                            .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                        fp,
                    );
                    if ferror(fp) != 0 {
                        write_fatal();
                    }
                    (*outstate).after_newline = r#true != 0;
                    def_state = C2Rust_Unnamed_0::OUTSIDE;
                }
            }
        }
    }
    if new <= pat_end && pch_char(new) as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
        if !copy_till(outstate, r#where + old - 1 as idx_t) {
            return r#false != 0;
        }
        if !R_do_defines.is_null() {
            if def_state.0 == C2Rust_Unnamed_0::OUTSIDE.0 {
                putline(
                    fp,
                    (&raw const if_defined as *const ::core::ffi::c_char)
                        .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                    R_do_defines,
                    nullptr,
                );
                def_state = C2Rust_Unnamed_0::IN_IFDEF;
            } else if def_state.0 == C2Rust_Unnamed_0::IN_IFNDEF.0 {
                Fputs(
                    (&raw const else_defined as *const ::core::ffi::c_char)
                        .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
                    fp,
                );
                def_state = C2Rust_Unnamed_0::IN_ELSE;
            }
            if ferror(fp) != 0 {
                write_fatal();
            }
            (*outstate).zero_output = r#false != 0;
        }
        loop {
            if !(*outstate).after_newline {
                Fputc('\n' as ::core::ffi::c_int, fp);
            }
            (*outstate).after_newline = pch_write_line(new, fp);
            (*outstate).zero_output = r#false != 0;
            new += 1;
            if !(new <= pat_end && pch_char(new) as ::core::ffi::c_int == '+' as ::core::ffi::c_int)
            {
                break;
            }
        }
    }
    if !R_do_defines.is_null() && def_state.0 != C2Rust_Unnamed_0::OUTSIDE.0 {
        Fputs(
            (&raw const end_defined as *const ::core::ffi::c_char)
                .offset((*outstate).after_newline as ::core::ffi::c_int as isize),
            fp,
        );
        if ferror(fp) != 0 {
            write_fatal();
        }
        (*outstate).after_newline = r#true != 0;
    }
    out_offset += (pch_repl_lines() - pch_ptrn_lines()) as ptrdiff_t;
    return r#true != 0;
}
unsafe extern "C" fn create_output_file(
    mut out: *mut outfile,
    mut open_flags: ::core::ffi::c_int,
) -> *mut FILE {
    let mut fd: ::core::ffi::c_int = create_file(
        out,
        O_WRONLY | binary_transput | open_flags,
        instat.st_mode,
        r#true != 0,
    );
    let mut f: *mut FILE = fdopen(
        fd,
        if binary_transput != 0 {
            b"wb\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"w\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if f.is_null() {
        pfatal(
            b"Can't create file %s\0".as_ptr() as *const ::core::ffi::c_char,
            quotearg((*out).name),
        );
    }
    return f;
}
unsafe extern "C" fn init_output(mut outstate: *mut outstate) {
    (*outstate).ofp = ::core::ptr::null_mut::<FILE>();
    (*outstate).after_newline = r#true != 0;
    (*outstate).zero_output = r#true != 0;
}
unsafe extern "C" fn open_outfile(mut name: *mut ::core::ffi::c_char) -> *mut FILE {
    if strcmp(name, b"-\0".as_ptr() as *const ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        let mut c2rust_lvalue: outfile = outfile {
            name: name,
            exists: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            alloc: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            temporary: false,
        };
        return create_output_file(&raw mut c2rust_lvalue, 0 as ::core::ffi::c_int);
    } else {
        let mut stdout_dup: ::core::ffi::c_int = dup(STDOUT_FILENO);
        if stdout_dup < 0 as ::core::ffi::c_int {
            pfatal(b"Failed to duplicate standard output\0".as_ptr() as *const ::core::ffi::c_char);
        }
        let mut ofp: *mut FILE = fdopen(stdout_dup, b"a\0".as_ptr() as *const ::core::ffi::c_char);
        if ofp.is_null() {
            pfatal(b"Failed to duplicate standard output\0".as_ptr() as *const ::core::ffi::c_char);
        }
        if dup2(STDERR_FILENO, STDOUT_FILENO) < 0 as ::core::ffi::c_int {
            pfatal(b"Failed to redirect messages to standard error\0".as_ptr()
                as *const ::core::ffi::c_char);
        }
        return ofp;
    };
}
unsafe extern "C" fn init_reject(mut outname: *const ::core::ffi::c_char) {
    let mut fd: ::core::ffi::c_int = 0;
    fd = make_tempfile(
        &raw mut tmprej,
        'r' as ::core::ffi::c_char,
        outname,
        O_WRONLY | binary_transput,
        0o666 as mode_t,
    );
    if fd < 0 as ::core::ffi::c_int {
        pfatal(
            b"Can't create temporary file %s\0".as_ptr() as *const ::core::ffi::c_char,
            tmprej.name,
        );
    }
    rejfp = fdopen(
        fd,
        if binary_transput != 0 {
            b"wb\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"w\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if rejfp.is_null() {
        pfatal(
            b"Can't open stream for file %s\0".as_ptr() as *const ::core::ffi::c_char,
            quotearg(tmprej.name),
        );
    }
}
#[export_name = "rboxc_patch_copy_till"]
pub unsafe extern "C" fn copy_till(mut outstate: *mut outstate, mut lastline: idx_t) -> bool {
    let mut R_last_frozen_line: idx_t = last_frozen_line;
    let mut fp: *mut FILE = (*outstate).ofp;
    if R_last_frozen_line > lastline {
        say(b"misordered hunks! output would be garbled\n\0".as_ptr() as *const ::core::ffi::c_char);
        return r#false != 0;
    }
    while R_last_frozen_line < lastline {
        R_last_frozen_line += 1;
        let mut line: iline = ifetch(R_last_frozen_line);
        if line.size != 0 {
            if !(*outstate).after_newline {
                Fputc('\n' as ::core::ffi::c_int, fp);
            }
            Fwrite(
                line.ptr as *const ::core::ffi::c_void,
                1 as size_t,
                line.size as size_t,
                fp,
            );
            (*outstate).after_newline = *line.ptr.offset((line.size - 1 as idx_t) as isize)
                as ::core::ffi::c_int
                == '\n' as ::core::ffi::c_int;
            (*outstate).zero_output = r#false != 0;
        }
    }
    last_frozen_line = R_last_frozen_line;
    return r#true != 0;
}
unsafe extern "C" fn spew_output(mut outstate: *mut outstate, mut st: *mut stat) -> bool {
    if debug as ::core::ffi::c_int & 256 as ::core::ffi::c_int != 0 {
        say(
            b"il=%td lfl=%td\n\0".as_ptr() as *const ::core::ffi::c_char,
            input_lines,
            last_frozen_line,
        );
    }
    if last_frozen_line < input_lines {
        if !copy_till(outstate, input_lines) {
            return r#false != 0;
        }
    }
    if !(*outstate).ofp.is_null() && outfile.is_null() {
        Fflush((*outstate).ofp);
        if fstat(fileno((*outstate).ofp), st) < 0 as ::core::ffi::c_int {
            write_fatal();
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn patch_match(
    mut base: idx_t,
    mut offset: ptrdiff_t,
    mut prefix_fuzz: idx_t,
    mut suffix_fuzz: idx_t,
) -> bool {
    let mut pat_lines: idx_t = pch_ptrn_lines() - suffix_fuzz;
    let mut pline: idx_t = 1 as idx_t + prefix_fuzz;
    while pline <= pat_lines {
        let mut line: iline = ifetch(pline - 1 as idx_t + base + offset);
        if canonicalize_ws {
            if !similar(line.ptr, line.size, pfetch(pline), pch_line_len(pline)) {
                return r#false != 0;
            }
        } else if line.size != pch_line_len(pline)
            || memcmp(
                line.ptr as *const ::core::ffi::c_void,
                pfetch(pline) as *const ::core::ffi::c_void,
                line.size as size_t,
            ) != 0 as ::core::ffi::c_int
        {
            return r#false != 0;
        }
        pline += 1;
    }
    return r#true != 0;
}
unsafe extern "C" fn check_line_endings(mut r#where: idx_t) -> bool {
    let mut p: *const ::core::ffi::c_char = pfetch(1 as idx_t);
    let mut size: idx_t = pch_line_len(1 as idx_t);
    if size == 0 {
        return r#false != 0;
    }
    let mut patch_crlf: bool = 2 as idx_t <= size
        && *p.offset((size - 2 as idx_t) as isize) as ::core::ffi::c_int
            == '\r' as ::core::ffi::c_int
        && *p.offset((size - 1 as idx_t) as isize) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int;
    if input_lines == 0 {
        return r#false != 0;
    }
    if r#where > input_lines {
        r#where = input_lines;
    }
    let mut line: iline = ifetch(r#where);
    if line.size == 0 {
        return r#false != 0;
    }
    let mut input_crlf: bool = 2 as idx_t <= line.size
        && *line.ptr.offset((line.size - 2 as idx_t) as isize) as ::core::ffi::c_int
            == '\r' as ::core::ffi::c_int
        && *line.ptr.offset((line.size - 1 as idx_t) as isize) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int;
    return patch_crlf as ::core::ffi::c_int != input_crlf as ::core::ffi::c_int;
}
#[export_name = "rboxc_patch_similar"]
pub unsafe extern "C" fn similar(
    mut a: *const ::core::ffi::c_char,
    mut alen: idx_t,
    mut b: *const ::core::ffi::c_char,
    mut blen: idx_t,
) -> bool {
    alen -= (alen != 0
        && *a.offset((alen - 1 as idx_t) as isize) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int) as ::core::ffi::c_int as idx_t;
    blen -= (blen != 0
        && *b.offset((blen - 1 as idx_t) as isize) as ::core::ffi::c_int
            == '\n' as ::core::ffi::c_int) as ::core::ffi::c_int as idx_t;
    loop {
        if blen == 0 || c_isblank(*b as ::core::ffi::c_int) as ::core::ffi::c_int != 0 {
            while blen != 0 && c_isblank(*b as ::core::ffi::c_int) as ::core::ffi::c_int != 0 {
                b = b.offset(1);
                blen -= 1;
            }
            if alen != 0 {
                if !c_isblank(*a as ::core::ffi::c_int) {
                    return r#false != 0;
                }
                loop {
                    a = a.offset(1);
                    alen -= 1;
                    if !(alen != 0
                        && c_isblank(*a as ::core::ffi::c_int) as ::core::ffi::c_int != 0)
                    {
                        break;
                    }
                }
            }
            if alen == 0 || blen == 0 {
                return alen == blen;
            }
        } else if alen == 0 || {
            let c2rust_fresh0 = a;
            a = a.offset(1);
            let c2rust_fresh1 = b;
            b = b.offset(1);
            *c2rust_fresh0 as ::core::ffi::c_int != *c2rust_fresh1 as ::core::ffi::c_int
        } {
            return r#false != 0;
        } else {
            alen -= 1;
            blen -= 1;
        }
    }
}
static mut files_to_delete: *mut file_to_delete = ::core::ptr::null_mut::<file_to_delete>();
static mut files_to_delete_tail: *mut *mut file_to_delete =
    unsafe { &raw const files_to_delete as *mut *mut file_to_delete };
unsafe extern "C" fn delete_file_later(
    mut name: *mut ::core::ffi::c_char,
    mut st: *const stat,
    mut backup: bool,
) {
    let mut file_to_delete: *mut file_to_delete = ::core::ptr::null_mut::<file_to_delete>();
    let mut st_tmp: stat = stat {
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
    if st.is_null() {
        if stat_file(name, &raw mut st_tmp) != 0 as ::core::ffi::c_int {
            pfatal(
                b"Can't get file attributes of %s %s\0".as_ptr() as *const ::core::ffi::c_char,
                b"file\0".as_ptr() as *const ::core::ffi::c_char,
                name,
            );
        }
        st = &raw mut st_tmp;
    }
    file_to_delete = xmalloc(::core::mem::size_of::<file_to_delete>()) as *mut file_to_delete;
    (*file_to_delete).name = xstrdup(name);
    (*file_to_delete).st = *st;
    (*file_to_delete).backup = backup;
    (*file_to_delete).next = ::core::ptr::null_mut::<file_to_delete>();
    *files_to_delete_tail = file_to_delete;
    files_to_delete_tail = &raw mut (*file_to_delete).next;
    insert_file_id(st, file_id_type::DELETE_LATER);
}
unsafe extern "C" fn delete_files() {
    let mut next: *mut file_to_delete = ::core::ptr::null_mut::<file_to_delete>();
    let mut f: *mut file_to_delete = files_to_delete;
    while !f.is_null() {
        if lookup_file_id(&raw mut (*f).st).0 == file_id_type::DELETE_LATER.0 {
            let mut mode: mode_t = (*f).st.st_mode;
            if verbosity_0.0 == verbosity::VERBOSE.0 {
                say(
                    b"Removing %s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    if mode & __S_IFMT as mode_t == 0o120000 as mode_t {
                        b"symbolic link\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"file\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    quotearg((*f).name),
                );
            }
            move_file(
                ::core::ptr::null_mut::<outfile>(),
                ::core::ptr::null::<stat>(),
                (*f).name,
                mode,
                (*f).backup,
            );
            removedirs((*f).name);
        }
        next = (*f).next;
        f = next;
    }
}
static mut files_to_output: *mut file_to_output = ::core::ptr::null_mut::<file_to_output>();
static mut files_to_output_tail: *mut *mut file_to_output =
    unsafe { &raw const files_to_output as *mut *mut file_to_output };
unsafe extern "C" fn output_file_later(
    mut from: *mut outfile,
    mut from_st: *const stat,
    mut to: *const ::core::ffi::c_char,
    mut mode: mode_t,
    mut backup: bool,
) {
    let mut tosize: idx_t = (if !to.is_null() {
        strlen(to).wrapping_add(1 as size_t)
    } else {
        0 as size_t
    }) as idx_t;
    let mut f: *mut file_to_output =
        ximalloc(::core::mem::size_of::<file_to_output>().wrapping_add(tosize as usize) as idx_t)
            as *mut file_to_output;
    let mut alloc: *mut ::core::ffi::c_char = (*from).alloc;
    (*f).from.alloc = if !alloc.is_null() {
        alloc
    } else {
        xstrdup((*from).name)
    };
    (*f).from.name = (*f).from.alloc;
    if !alloc.is_null() {
        (*from).alloc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    ::core::ptr::write_volatile(
        &raw mut (*f).from.exists,
        if !alloc.is_null() {
            ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const (*from).exists)
        } else {
            volatilize((*f).from.alloc)
        },
    );
    (*f).from.temporary = (*from).temporary;
    (*f).from_st = *from_st;
    ::core::ptr::write_volatile(
        &raw mut (*f).to,
        (if !to.is_null() {
            memcpy(
                f.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                to as *const ::core::ffi::c_void,
                tosize as size_t,
            )
        } else {
            nullptr
        }) as *mut ::core::ffi::c_char,
    );
    (*f).mode = mode;
    (*f).backup = backup;
    ::core::ptr::write_volatile(
        &raw mut (*f).next,
        ::core::ptr::null_mut::<file_to_output>(),
    );
    defer_signals();
    ::core::ptr::write_volatile(files_to_output_tail, f);
    ::core::ptr::write_volatile(
        &raw mut (*from).exists,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
    );
    undefer_signals();
    files_to_output_tail = &raw mut (*f).next;
}
unsafe extern "C" fn output_file_now(
    mut from: *mut outfile,
    mut from_st: *const stat,
    mut to: *mut ::core::ffi::c_char,
    mut mode: mode_t,
    mut backup: bool,
) {
    if to.is_null() {
        if backup {
            create_backup((*from).name, from_st, r#true != 0);
        }
    } else {
        '_c2rust_label: {
            if 0 as __off_t <= (*from_st).st_size {
            } else {
                __assert_fail(
                    b"0 <= from_st->st_size\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/patch-2.8/src/patch.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    1862 as ::core::ffi::c_uint,
                    b"void output_file_now(struct outfile *, const struct stat *, char *, mode_t, _Bool)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        move_file(from, from_st, to, mode, backup);
    };
}
unsafe extern "C" fn output_file(
    mut from: *mut outfile,
    mut from_st: *const stat,
    mut to: *mut ::core::ffi::c_char,
    mut to_st: *const stat,
    mut mode: mode_t,
    mut backup: bool,
) {
    if from.is_null() {
        delete_file_later(to, to_st, backup);
    } else if pch_git_diff() as ::core::ffi::c_int != 0
        && pch_says_nonexistent(reverse_flag) as ::core::ffi::c_int != 2 as ::core::ffi::c_int
    {
        output_file_later(from, from_st, to, mode, backup);
    } else {
        output_file_now(from, from_st, to, mode, backup);
    };
}
unsafe extern "C" fn output_files(mut st: *const stat, mut exiting: ::core::ffi::c_int) {
    let mut next: *mut file_to_output = ::core::ptr::null_mut::<file_to_output>();
    let mut f: *mut file_to_output =
        ::core::ptr::read_volatile::<*mut file_to_output>(&raw const files_to_output);
    while !f.is_null() {
        let mut to: *mut ::core::ffi::c_char =
            ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const (*f).to);
        next = ::core::ptr::read_volatile::<*mut file_to_output>(&raw const (*f).next);
        let mut early_return: bool = false;
        if exiting < 0 as ::core::ffi::c_int {
            if !to.is_null() {
                let mut exists: *mut ::core::ffi::c_char =
                    ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(
                        &raw const (*f).from.exists,
                    );
                if !exists.is_null() {
                    unlink(devolatilize(exists));
                }
            }
            early_return = r#false != 0;
        } else {
            let mut from_st: *const stat = &raw mut (*f).from_st;
            output_file_now(&raw mut (*f).from, from_st, to, (*f).mode, (*f).backup);
            defer_signals();
            if !to.is_null() {
                let mut exists_0: *mut ::core::ffi::c_char =
                    ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(
                        &raw const (*f).from.exists,
                    );
                if !exists_0.is_null() {
                    safe_unlink(devolatilize(exists_0));
                }
            }
            ::core::ptr::write_volatile(&raw mut files_to_output, next);
            undefer_signals();
            early_return = !st.is_null()
                && (*st).st_dev == (*from_st).st_dev
                && (*st).st_ino == (*from_st).st_ino;
        }
        if exiting == 0 {
            free((*f).from.alloc as *mut ::core::ffi::c_void);
            free(f as *mut ::core::ffi::c_void);
        }
        if next.is_null() {
            files_to_output_tail = &raw mut files_to_output;
        }
        if early_return {
            return;
        }
        f = next;
    }
}
#[export_name = "rboxc_patch_fatal_cleanup"]
pub unsafe extern "C" fn fatal_cleanup() {
    cleanup_remove();
    output_files(::core::ptr::null::<stat>(), -1 as ::core::ffi::c_int);
}
#[export_name = "rboxc_patch_fatal_exit"]
pub unsafe extern "C" fn fatal_exit() {
    defer_signals();
    cleanup_remove();
    undefer_signals();
    output_files(::core::ptr::null::<stat>(), 1 as ::core::ffi::c_int);
    exit(C2Rust_Unnamed::EXIT_TROUBLE.0 as ::core::ffi::c_int);
}
unsafe extern "C" fn remove_if_needed(mut tmp: *mut outfile) {
    let mut exists: *mut ::core::ffi::c_char =
        ::core::ptr::read_volatile::<*mut ::core::ffi::c_char>(&raw const (*tmp).exists);
    if !exists.is_null() {
        safe_unlink(devolatilize(exists));
        ::core::ptr::write_volatile(
            &raw mut (*tmp).exists,
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
        );
    }
}
unsafe extern "C" fn perfile_cleanup_remove() {
    remove_if_needed(&raw mut tmped);
    remove_if_needed(&raw mut tmpout);
    remove_if_needed(&raw mut tmprej);
}
unsafe extern "C" fn cleanup_remove() {
    remove_if_needed(&raw mut tmppat);
    perfile_cleanup_remove();
}
unsafe extern "C" fn free_outfile_name(mut f: *mut outfile) {
    free((*f).alloc as *mut ::core::ffi::c_void);
    (*f).alloc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    (*f).name = (*f).alloc;
}
unsafe extern "C" fn perfile_cleanup_free() {
    free_outfile_name(&raw mut tmped);
    free_outfile_name(&raw mut tmpout);
    free_outfile_name(&raw mut tmprej);
}
pub const nullptr: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
