// Generated from pinned GNU Tar 1.35 by scripts/translate-tar.py.
// Source SHA-256: cc5fad63d3ae5b7610cb3a30a670edb2e27baacd2d7668703e464a69577c86d5
/* A tar (tape archiver) program.

   Copyright 1988-2023 Free Software Foundation, Inc.

   Written by John Gilmore, starting 1985-08-25.

   This program is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3, or (at your option) any later
   version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General
   Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program.  If not, see <http://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct __dirstream { _opaque: [u8; 0] }
#[repr(C)]
pub struct exclist { _opaque: [u8; 0] }
#[repr(C)]
pub struct exclude { _opaque: [u8; 0] }
#[repr(C)]
pub struct mode_change { _opaque: [u8; 0] }
#[repr(C)]
pub struct quoting_options { _opaque: [u8; 0] }
#[repr(C)]
pub struct wordsplit_node { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
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
    fn memchr(
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strcspn(
        __s: *const ::core::ffi::c_char,
        __reject: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_ulong;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn rpmatch(__response: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn __getdelim(
        __lineptr: *mut *mut ::core::ffi::c_char,
        __n: *mut size_t,
        __delimiter: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> __ssize_t;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tar_base_name"]
    fn base_name(file: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    fn closedir(__dirp: *mut DIR) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_tar_x2nrealloc"]
    fn x2nrealloc(
        p: *mut ::core::ffi::c_void,
        pn: *mut size_t,
        s: size_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_tar_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    #[link_name = "rboxc_tar_error_hook"]
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    #[link_name = "rboxc_tar_exit_status"]
    static mut exit_status: ::core::ffi::c_int;
    #[link_name = "rboxc_tar_open_fatal"]
    fn open_fatal(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_stat_error"]
    fn stat_error(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_fatal_exit"]
    fn fatal_exit();
    #[link_name = "rboxc_tar_argp_parse"]
    fn argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_tar_argp_help"]
    fn argp_help(
        __argp: *const argp,
        __stream: *mut FILE,
        __flags: ::core::ffi::c_uint,
        __name: *mut ::core::ffi::c_char,
    );
    #[link_name = "rboxc_tar_argp_error"]
    fn argp_error(__state: *const argp_state, __fmt: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_tar_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_tar_simple_backup_suffix"]
    static mut simple_backup_suffix: *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar_xget_version"]
    fn xget_version(
        context: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
    ) -> backup_type;
    #[link_name = "rboxc_tar_new_exclude"]
    fn new_exclude() -> *mut exclude;
    #[link_name = "rboxc_tar_mode_compile"]
    fn mode_compile(_: *const ::core::ffi::c_char) -> *mut mode_change;
    #[link_name = "rboxc_tar_quote"]
    fn quote(arg: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar__obstack_newchunk"]
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    #[link_name = "rboxc_tar__obstack_free"]
    fn _obstack_free(_: *mut obstack, _: *mut ::core::ffi::c_void);
    #[link_name = "rboxc_tar__obstack_begin"]
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
        _: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_xvasprintf"]
    fn xvasprintf(
        format: *const ::core::ffi::c_char,
        args: ::core::ffi::VaList,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tar_stdlis"]
    static mut stdlis: *mut FILE;
    #[link_name = "rboxc_tar_closeout_volume_number"]
    fn closeout_volume_number();
    #[link_name = "rboxc_tar_compute_duration"]
    fn compute_duration() -> ::core::ffi::c_double;
    #[link_name = "rboxc_tar_init_volume_number"]
    fn init_volume_number();
    #[link_name = "rboxc_tar_print_total_stats"]
    fn print_total_stats();
    #[link_name = "rboxc_tar_set_start_time"]
    fn set_start_time();
    #[link_name = "rboxc_tar_create_archive"]
    fn create_archive();
    #[link_name = "rboxc_tar_check_links"]
    fn check_links();
    #[link_name = "rboxc_tar_diff_archive"]
    fn diff_archive();
    #[link_name = "rboxc_tar_diff_init"]
    fn diff_init();
    #[link_name = "rboxc_tar_extr_init"]
    fn extr_init();
    #[link_name = "rboxc_tar_extract_archive"]
    fn extract_archive();
    #[link_name = "rboxc_tar_extract_finish"]
    fn extract_finish();
    #[link_name = "rboxc_tar_delete_archive_members"]
    fn delete_archive_members();
    #[link_name = "rboxc_tar_show_snapshot_field_ranges"]
    fn show_snapshot_field_ranges();
    #[link_name = "rboxc_tar_current_header"]
    static mut current_header: *mut block;
    #[link_name = "rboxc_tar_current_format"]
    static mut current_format: archive_format;
    #[link_name = "rboxc_tar_tartime"]
    fn tartime(t: timespec, full_time: bool) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar_list_archive"]
    fn list_archive();
    #[link_name = "rboxc_tar_test_archive_label"]
    fn test_archive_label();
    #[link_name = "rboxc_tar_read_and"]
    fn read_and(do_something: Option<unsafe extern "C" fn() -> ()>);
    #[link_name = "rboxc_tar_code_timespec"]
    fn code_timespec(ts: timespec, sbuf: *mut ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar_close_diag"]
    fn close_diag(name: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_filename_args"]
    static mut filename_args: files_count;
    #[link_name = "rboxc_tar_gname_to_gid"]
    fn gname_to_gid(gname: *const ::core::ffi::c_char, pgid: *mut gid_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_uname_to_uid"]
    fn uname_to_uid(uname: *const ::core::ffi::c_char, puid: *mut uid_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_name_init"]
    fn name_init();
    #[link_name = "rboxc_tar_name_add_name"]
    fn name_add_name(name: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_name_term"]
    fn name_term();
    #[link_name = "rboxc_tar_add_starting_file"]
    fn add_starting_file(file_name: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_update_archive"]
    fn update_archive();
    #[link_name = "rboxc_tar_xattrs_mask_add"]
    fn xattrs_mask_add(mask: *const ::core::ffi::c_char, incl: bool);
    #[link_name = "rboxc_tar_xattrs_clear_setup"]
    fn xattrs_clear_setup();
    #[link_name = "rboxc_tar_xheader_destroy"]
    fn xheader_destroy(hdr: *mut xheader);
    #[link_name = "rboxc_tar_xheader_set_option"]
    fn xheader_set_option(string: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_tar_xattr_map_free"]
    fn xattr_map_free(xattr_map: *mut xattr_map);
    #[link_name = "rboxc_tar_set_transform_expr"]
    fn set_transform_expr(expr: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_set_compression_program_by_suffix"]
    fn set_compression_program_by_suffix(
        name: *const ::core::ffi::c_char,
        defprog: *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_tar_strip_compression_suffix"]
    fn strip_compression_suffix(name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tar_checkpoint_compile_action"]
    fn checkpoint_compile_action(str: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_checkpoint_finish_compile"]
    fn checkpoint_finish_compile();
    #[link_name = "rboxc_tar_checkpoint_finish"]
    fn checkpoint_finish();
    #[link_name = "rboxc_tar_checkpoint_flush_actions"]
    fn checkpoint_flush_actions();
    #[link_name = "rboxc_tar_set_warning_option"]
    fn set_warning_option(arg: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_warning_option"]
    static mut warning_option: ::core::ffi::c_int;
    #[link_name = "rboxc_tar_info_free_exclist"]
    fn info_free_exclist(dir: *mut tar_stat_info);
    #[link_name = "rboxc_tar_owner_map_read"]
    fn owner_map_read(name: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_group_map_read"]
    fn group_map_read(file: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_argmatch_die"]
    static mut argmatch_die: argmatch_exit_fn;
    #[link_name = "rboxc_tar___xargmatch_internal"]
    fn __xargmatch_internal(
        context: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
        arglist: *const *const ::core::ffi::c_char,
        vallist: *const ::core::ffi::c_void,
        valsize: size_t,
        exit_fn: argmatch_exit_fn,
        allow_abbreviation: bool,
    ) -> ptrdiff_t;
    #[link_name = "rboxc_tar_close_stdout_set_file_name"]
    fn close_stdout_set_file_name(file: *const ::core::ffi::c_char);
    #[link_name = "rboxc_tar_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_tar_exit_failure"]
    static mut exit_failure: ::core::ffi::c_int;
    #[link_name = "rboxc_tar_parse_datetime"]
    fn parse_datetime(_: *mut timespec, _: *const ::core::ffi::c_char, _: *const timespec) -> bool;
    #[link_name = "rboxc_tar_rmt_command"]
    static mut rmt_command: *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar_force_local_option"]
    static mut force_local_option: bool;
    #[link_name = "rboxc_tar_wordsplit"]
    fn wordsplit(
        s: *const ::core::ffi::c_char,
        ws: *mut wordsplit_t,
        flags: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_wordsplit_free"]
    fn wordsplit_free(ws: *mut wordsplit_t);
    #[link_name = "rboxc_tar_wordsplit_strerror"]
    fn wordsplit_strerror(ws: *mut wordsplit_t) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_tar_quoting_style_args"]
    static quoting_style_args: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_tar_set_quoting_style"]
    fn set_quoting_style(o: *mut quoting_options, s: quoting_style);
    #[link_name = "rboxc_tar_set_char_quoting"]
    fn set_char_quoting(
        o: *mut quoting_options,
        c: ::core::ffi::c_char,
        i: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_quotearg_colon"]
    fn quotearg_colon(arg: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_tar_xstrtoumax"]
    fn xstrtoumax(
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut uintmax_t,
        _: *const ::core::ffi::c_char,
    ) -> strtol_error;
    #[link_name = "rboxc_tar_stdopen"]
    fn stdopen() -> ::core::ffi::c_int;
    #[link_name = "rboxc_tar_names_argp"]
    static mut names_argp: argp;
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
pub type __ssize_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type dev_t = __dev_t;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type ptrdiff_t = isize;
pub type error_t = ::core::ffi::c_int;
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
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut ::core::ffi::c_char,
    pub next_free: *mut ::core::ffi::c_char,
    pub chunk_limit: *mut ::core::ffi::c_char,
    pub temp: C2Rust_Unnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2Rust_Unnamed_1,
    pub freefun: C2Rust_Unnamed_0,
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
pub union C2Rust_Unnamed_0 {
    pub plain: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    pub extra:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub plain: Option<unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void>,
    pub extra:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> *mut ::core::ffi::c_void>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub i: size_t,
    pub p: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut ::core::ffi::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [::core::ffi::c_char; 0],
}
pub type uintmax_t = ::libc::uintmax_t;
pub type DIR = __dirstream;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct savedir_option(pub ::core::ffi::c_uint);
impl savedir_option {
    pub const SAVEDIR_SORT_NONE: Self = Self(0);
    pub const SAVEDIR_SORT_NAME: Self = Self(1);
    pub const SAVEDIR_SORT_INODE: Self = Self(2);
    pub const SAVEDIR_SORT_FASTREAD: Self = Self(2);
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
pub struct posix_header {
    pub name: [::core::ffi::c_char; 100],
    pub mode: [::core::ffi::c_char; 8],
    pub uid: [::core::ffi::c_char; 8],
    pub gid: [::core::ffi::c_char; 8],
    pub size: [::core::ffi::c_char; 12],
    pub mtime: [::core::ffi::c_char; 12],
    pub chksum: [::core::ffi::c_char; 8],
    pub typeflag: ::core::ffi::c_char,
    pub linkname: [::core::ffi::c_char; 100],
    pub magic: [::core::ffi::c_char; 6],
    pub version: [::core::ffi::c_char; 2],
    pub uname: [::core::ffi::c_char; 32],
    pub gname: [::core::ffi::c_char; 32],
    pub devmajor: [::core::ffi::c_char; 8],
    pub devminor: [::core::ffi::c_char; 8],
    pub prefix: [::core::ffi::c_char; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse {
    pub offset: [::core::ffi::c_char; 12],
    pub numbytes: [::core::ffi::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_header {
    pub sp: [sparse; 21],
    pub isextended: ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldgnu_header {
    pub unused_pad1: [::core::ffi::c_char; 345],
    pub atime: [::core::ffi::c_char; 12],
    pub ctime: [::core::ffi::c_char; 12],
    pub offset: [::core::ffi::c_char; 12],
    pub longnames: [::core::ffi::c_char; 4],
    pub unused_pad2: ::core::ffi::c_char,
    pub sp: [sparse; 4],
    pub isextended: ::core::ffi::c_char,
    pub realsize: [::core::ffi::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_header {
    pub name: [::core::ffi::c_char; 100],
    pub mode: [::core::ffi::c_char; 8],
    pub uid: [::core::ffi::c_char; 8],
    pub gid: [::core::ffi::c_char; 8],
    pub size: [::core::ffi::c_char; 12],
    pub mtime: [::core::ffi::c_char; 12],
    pub chksum: [::core::ffi::c_char; 8],
    pub typeflag: ::core::ffi::c_char,
    pub linkname: [::core::ffi::c_char; 100],
    pub magic: [::core::ffi::c_char; 6],
    pub version: [::core::ffi::c_char; 2],
    pub uname: [::core::ffi::c_char; 32],
    pub gname: [::core::ffi::c_char; 32],
    pub devmajor: [::core::ffi::c_char; 8],
    pub devminor: [::core::ffi::c_char; 8],
    pub prefix: [::core::ffi::c_char; 131],
    pub atime: [::core::ffi::c_char; 12],
    pub ctime: [::core::ffi::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_in_header {
    pub fill: [::core::ffi::c_char; 345],
    pub prefix: [::core::ffi::c_char; 1],
    pub fill2: ::core::ffi::c_char,
    pub fill3: [::core::ffi::c_char; 8],
    pub isextended: ::core::ffi::c_char,
    pub sp: [sparse; 4],
    pub realsize: [::core::ffi::c_char; 12],
    pub offset: [::core::ffi::c_char; 12],
    pub atime: [::core::ffi::c_char; 12],
    pub ctime: [::core::ffi::c_char; 12],
    pub mfill: [::core::ffi::c_char; 8],
    pub xmagic: [::core::ffi::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_ext_header {
    pub sp: [sparse; 21],
    pub isextended: ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct archive_format(pub ::core::ffi::c_uint);
impl archive_format {
    pub const DEFAULT_FORMAT: Self = Self(0);
    pub const V7_FORMAT: Self = Self(1);
    pub const OLDGNU_FORMAT: Self = Self(2);
    pub const USTAR_FORMAT: Self = Self(3);
    pub const POSIX_FORMAT: Self = Self(4);
    pub const STAR_FORMAT: Self = Self(5);
    pub const GNU_FORMAT: Self = Self(6);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sp_array {
    pub offset: off_t,
    pub numbytes: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xheader {
    pub stk: *mut obstack,
    pub size: size_t,
    pub buffer: *mut ::core::ffi::c_char,
    pub string_length: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_array {
    pub xkey: *mut ::core::ffi::c_char,
    pub xval_ptr: *mut ::core::ffi::c_char,
    pub xval_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_map {
    pub xm_map: *mut xattr_array,
    pub xm_size: size_t,
    pub xm_max: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_stat_info {
    pub orig_file_name: *mut ::core::ffi::c_char,
    pub file_name: *mut ::core::ffi::c_char,
    pub had_trailing_slash: bool,
    pub link_name: *mut ::core::ffi::c_char,
    pub uname: *mut ::core::ffi::c_char,
    pub gname: *mut ::core::ffi::c_char,
    pub cntx_name: *mut ::core::ffi::c_char,
    pub acls_a_ptr: *mut ::core::ffi::c_char,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut ::core::ffi::c_char,
    pub acls_d_len: size_t,
    pub stat: stat,
    pub atime: timespec,
    pub mtime: timespec,
    pub ctime: timespec,
    pub archive_file_size: off_t,
    pub is_sparse: bool,
    pub sparse_major: ::core::ffi::c_uint,
    pub sparse_minor: ::core::ffi::c_uint,
    pub sparse_map_avail: size_t,
    pub sparse_map_size: size_t,
    pub sparse_map: *mut sp_array,
    pub real_size: off_t,
    pub real_size_set: bool,
    pub sparse_name_done: bool,
    pub xattr_map: xattr_map,
    pub xhdr: xheader,
    pub is_dumpdir: bool,
    pub skipped: bool,
    pub dumpdir: *mut ::core::ffi::c_char,
    pub parent: *mut tar_stat_info,
    pub dirstream: *mut DIR,
    pub fd: ::core::ffi::c_int,
    pub exclude_list: *mut exclist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union block {
    pub buffer: [::core::ffi::c_char; 512],
    pub header: posix_header,
    pub star_header: star_header,
    pub oldgnu_header: oldgnu_header,
    pub sparse_header: sparse_header,
    pub star_in_header: star_in_header,
    pub star_ext_header: star_ext_header,
}
pub type tarlong = ::core::ffi::c_double;
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
pub struct subcommand(pub ::core::ffi::c_uint);
impl subcommand {
    pub const UNKNOWN_SUBCOMMAND: Self = Self(0);
    pub const APPEND_SUBCOMMAND: Self = Self(1);
    pub const CAT_SUBCOMMAND: Self = Self(2);
    pub const CREATE_SUBCOMMAND: Self = Self(3);
    pub const DELETE_SUBCOMMAND: Self = Self(4);
    pub const DIFF_SUBCOMMAND: Self = Self(5);
    pub const EXTRACT_SUBCOMMAND: Self = Self(6);
    pub const LIST_SUBCOMMAND: Self = Self(7);
    pub const UPDATE_SUBCOMMAND: Self = Self(8);
    pub const TEST_LABEL_SUBCOMMAND: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct atime_preserve(pub ::core::ffi::c_uint);
impl atime_preserve {
    pub const no_atime_preserve: Self = Self(0);
    pub const replace_atime_preserve: Self = Self(1);
    pub const system_atime_preserve: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct old_files(pub ::core::ffi::c_uint);
impl old_files {
    pub const DEFAULT_OLD_FILES: Self = Self(0);
    pub const NO_OVERWRITE_DIR_OLD_FILES: Self = Self(1);
    pub const OVERWRITE_OLD_FILES: Self = Self(2);
    pub const UNLINK_FIRST_OLD_FILES: Self = Self(3);
    pub const KEEP_OLD_FILES: Self = Self(4);
    pub const SKIP_OLD_FILES: Self = Self(5);
    pub const KEEP_NEWER_FILES: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct set_mtime_option_mode(pub ::core::ffi::c_uint);
impl set_mtime_option_mode {
    pub const USE_FILE_MTIME: Self = Self(0);
    pub const FORCE_MTIME: Self = Self(1);
    pub const CLAMP_MTIME: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct hole_detection_method(pub ::core::ffi::c_uint);
impl hole_detection_method {
    pub const HOLE_DETECTION_DEFAULT: Self = Self(0);
    pub const HOLE_DETECTION_RAW: Self = Self(1);
    pub const HOLE_DETECTION_SEEK: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct files_count(pub ::core::ffi::c_uint);
impl files_count {
    pub const FILES_NONE: Self = Self(0);
    pub const FILES_ONE: Self = Self(1);
    pub const FILES_MANY: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fmttab {
    pub name: *const ::core::ffi::c_char,
    pub fmt: archive_format,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_locus {
    pub source: option_source,
    pub name: *const ::core::ffi::c_char,
    pub line: size_t,
    pub prev: *mut option_locus,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct option_source(pub ::core::ffi::c_uint);
impl option_source {
    pub const OPTS_ENVIRON: Self = Self(0);
    pub const OPTS_COMMAND_LINE: Self = Self(1);
    pub const OPTS_FILE: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_args {
    pub loc: *mut option_locus,
    pub textual_date: *mut textual_date,
    pub o_option: bool,
    pub pax_option: bool,
    pub compress_autodetect: bool,
    pub backup_suffix_string: *const ::core::ffi::c_char,
    pub version_control_string: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textual_date {
    pub next: *mut textual_date,
    pub ts: timespec,
    pub rpl_option: *const ::core::ffi::c_char,
    pub date: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigtab {
    pub name: *const ::core::ffi::c_char,
    pub signo: ::core::ffi::c_int,
}
pub type argmatch_exit_fn = Option<unsafe extern "C" fn() -> ()>;
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
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut ::core::ffi::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: ::core::ffi::c_uint,
    pub ws_options: ::core::ffi::c_uint,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const ::core::ffi::c_char,
    pub ws_comment: *const ::core::ffi::c_char,
    pub ws_escape: [*const ::core::ffi::c_char; 2],
    pub ws_alloc_die: Option<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>,
    pub ws_debug: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()>,
    pub ws_env: *mut *const ::core::ffi::c_char,
    pub ws_envbuf: *mut *mut ::core::ffi::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            size_t,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub ws_closure: *mut ::core::ffi::c_void,
    pub ws_command: Option<
        unsafe extern "C" fn(
            *mut *mut ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            size_t,
            *mut *mut ::core::ffi::c_char,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub ws_input: *const ::core::ffi::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: ::core::ffi::c_int,
    pub ws_usererr: *mut ::core::ffi::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: ::core::ffi::c_int,
}
pub type wordsplit_t = wordsplit;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const ACLS_OPTION: Self = Self(128);
    pub const ATIME_PRESERVE_OPTION: Self = Self(129);
    pub const BACKUP_OPTION: Self = Self(130);
    pub const CHECK_DEVICE_OPTION: Self = Self(131);
    pub const CHECKPOINT_OPTION: Self = Self(132);
    pub const CHECKPOINT_ACTION_OPTION: Self = Self(133);
    pub const CLAMP_MTIME_OPTION: Self = Self(134);
    pub const DELAY_DIRECTORY_RESTORE_OPTION: Self = Self(135);
    pub const HARD_DEREFERENCE_OPTION: Self = Self(136);
    pub const DELETE_OPTION: Self = Self(137);
    pub const FORCE_LOCAL_OPTION: Self = Self(138);
    pub const FULL_TIME_OPTION: Self = Self(139);
    pub const GROUP_OPTION: Self = Self(140);
    pub const GROUP_MAP_OPTION: Self = Self(141);
    pub const IGNORE_COMMAND_ERROR_OPTION: Self = Self(142);
    pub const IGNORE_FAILED_READ_OPTION: Self = Self(143);
    pub const INDEX_FILE_OPTION: Self = Self(144);
    pub const KEEP_DIRECTORY_SYMLINK_OPTION: Self = Self(145);
    pub const KEEP_NEWER_FILES_OPTION: Self = Self(146);
    pub const LEVEL_OPTION: Self = Self(147);
    pub const LZIP_OPTION: Self = Self(148);
    pub const LZMA_OPTION: Self = Self(149);
    pub const LZOP_OPTION: Self = Self(150);
    pub const MODE_OPTION: Self = Self(151);
    pub const MTIME_OPTION: Self = Self(152);
    pub const NEWER_MTIME_OPTION: Self = Self(153);
    pub const NO_ACLS_OPTION: Self = Self(154);
    pub const NO_AUTO_COMPRESS_OPTION: Self = Self(155);
    pub const NO_CHECK_DEVICE_OPTION: Self = Self(156);
    pub const NO_DELAY_DIRECTORY_RESTORE_OPTION: Self = Self(157);
    pub const NO_IGNORE_COMMAND_ERROR_OPTION: Self = Self(158);
    pub const NO_OVERWRITE_DIR_OPTION: Self = Self(159);
    pub const NO_QUOTE_CHARS_OPTION: Self = Self(160);
    pub const NO_SAME_OWNER_OPTION: Self = Self(161);
    pub const NO_SAME_PERMISSIONS_OPTION: Self = Self(162);
    pub const NO_SEEK_OPTION: Self = Self(163);
    pub const NO_SELINUX_CONTEXT_OPTION: Self = Self(164);
    pub const NO_XATTR_OPTION: Self = Self(165);
    pub const NUMERIC_OWNER_OPTION: Self = Self(166);
    pub const OCCURRENCE_OPTION: Self = Self(167);
    pub const OLD_ARCHIVE_OPTION: Self = Self(168);
    pub const ONE_FILE_SYSTEM_OPTION: Self = Self(169);
    pub const ONE_TOP_LEVEL_OPTION: Self = Self(170);
    pub const OVERWRITE_DIR_OPTION: Self = Self(171);
    pub const OVERWRITE_OPTION: Self = Self(172);
    pub const OWNER_OPTION: Self = Self(173);
    pub const OWNER_MAP_OPTION: Self = Self(174);
    pub const PAX_OPTION: Self = Self(175);
    pub const POSIX_OPTION: Self = Self(176);
    pub const QUOTE_CHARS_OPTION: Self = Self(177);
    pub const QUOTING_STYLE_OPTION: Self = Self(178);
    pub const RECORD_SIZE_OPTION: Self = Self(179);
    pub const RECURSIVE_UNLINK_OPTION: Self = Self(180);
    pub const REMOVE_FILES_OPTION: Self = Self(181);
    pub const RESTRICT_OPTION: Self = Self(182);
    pub const RMT_COMMAND_OPTION: Self = Self(183);
    pub const RSH_COMMAND_OPTION: Self = Self(184);
    pub const SAME_OWNER_OPTION: Self = Self(185);
    pub const SELINUX_CONTEXT_OPTION: Self = Self(186);
    pub const SHOW_DEFAULTS_OPTION: Self = Self(187);
    pub const SHOW_OMITTED_DIRS_OPTION: Self = Self(188);
    pub const SHOW_SNAPSHOT_FIELD_RANGES_OPTION: Self = Self(189);
    pub const SHOW_TRANSFORMED_NAMES_OPTION: Self = Self(190);
    pub const SKIP_OLD_FILES_OPTION: Self = Self(191);
    pub const SORT_OPTION: Self = Self(192);
    pub const HOLE_DETECTION_OPTION: Self = Self(193);
    pub const SPARSE_VERSION_OPTION: Self = Self(194);
    pub const STRIP_COMPONENTS_OPTION: Self = Self(195);
    pub const SUFFIX_OPTION: Self = Self(196);
    pub const TEST_LABEL_OPTION: Self = Self(197);
    pub const TOTALS_OPTION: Self = Self(198);
    pub const TO_COMMAND_OPTION: Self = Self(199);
    pub const TRANSFORM_OPTION: Self = Self(200);
    pub const UTC_OPTION: Self = Self(201);
    pub const VOLNO_FILE_OPTION: Self = Self(202);
    pub const WARNING_OPTION: Self = Self(203);
    pub const XATTR_OPTION: Self = Self(204);
    pub const XATTR_EXCLUDE: Self = Self(205);
    pub const XATTR_INCLUDE: Self = Self(206);
    pub const ZSTD_OPTION: Self = Self(207);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const GRH_COMMAND: Self = Self(0);
    pub const GRID_COMMAND: Self = Self(1);
    pub const GRH_MODIFIER: Self = Self(2);
    pub const GRID_MODIFIER: Self = Self(3);
    pub const GRID_FILE_NAME: Self = Self(4);
    pub const GRH_OVERWRITE: Self = Self(5);
    pub const GRID_OVERWRITE: Self = Self(6);
    pub const GRH_OUTPUT: Self = Self(7);
    pub const GRID_OUTPUT: Self = Self(8);
    pub const GRH_FATTR: Self = Self(9);
    pub const GRID_FATTR: Self = Self(10);
    pub const GRH_XATTR: Self = Self(11);
    pub const GRID_XATTR: Self = Self(12);
    pub const GRH_DEVICE: Self = Self(13);
    pub const GRID_DEVICE: Self = Self(14);
    pub const GRH_BLOCKING: Self = Self(15);
    pub const GRID_BLOCKING: Self = Self(16);
    pub const GRH_FORMAT: Self = Self(17);
    pub const GRID_FORMAT: Self = Self(18);
    pub const GRDOC_FORMAT: Self = Self(19);
    pub const GRID_FORMAT_OPT: Self = Self(20);
    pub const GRH_COMPRESS: Self = Self(21);
    pub const GRID_COMPRESS: Self = Self(22);
    pub const GRH_FILE: Self = Self(23);
    pub const GRID_FILE: Self = Self(24);
    pub const GRH_NAME_XFORM: Self = Self(25);
    pub const GRID_NAME_XFORM: Self = Self(26);
    pub const GRH_INFORMATIVE: Self = Self(27);
    pub const GRID_INFORMATIVE: Self = Self(28);
    pub const GRH_COMPAT: Self = Self(29);
    pub const GRID_COMPAT: Self = Self(30);
    pub const GRH_OTHER: Self = Self(31);
    pub const GRID_OTHER: Self = Self(32);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct option_class_0(pub ::core::ffi::c_uint);
impl option_class_0 {
    pub const OC_COMPRESS: Self = Self(0);
    pub const OC_OCCURRENCE: Self = Self(1);
    pub const OC_LISTED_INCREMENTAL: Self = Self(2);
    pub const OC_NEWER: Self = Self(3);
    pub const OC_VERIFY: Self = Self(4);
    pub const OC_STARTING_FILE: Self = Self(5);
    pub const OC_SAME_ORDER: Self = Self(6);
    pub const OC_ONE_TOP_LEVEL: Self = Self(7);
    pub const OC_ABSOLUTE_NAMES: Self = Self(8);
    pub const OC_OLD_FILES: Self = Self(9);
    pub const OC_MAX: Self = Self(10);
}
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_NOCTTY: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const __O_DIRECTORY: ::core::ffi::c_int = 0o200000 as ::core::ffi::c_int;
pub const __O_NOFOLLOW: ::core::ffi::c_int = 0o400000 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const __O_NOATIME: ::core::ffi::c_int = 0o1000000 as ::core::ffi::c_int;
pub const O_DIRECTORY: ::core::ffi::c_int = __O_DIRECTORY;
pub const O_NOFOLLOW: ::core::ffi::c_int = __O_NOFOLLOW;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
pub const O_NOATIME: ::core::ffi::c_int = __O_NOATIME;
pub const AT_SYMLINK_NOFOLLOW: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
pub const O_SEARCH: ::core::ffi::c_int = O_RDONLY;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGUSR2: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn fputc_unlocked(
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
unsafe extern "C" fn getline(
    mut __lineptr: *mut *mut ::core::ffi::c_char,
    mut __n: *mut size_t,
    mut __stream: *mut FILE,
) -> __ssize_t {
    return __getdelim(__lineptr, __n, '\n' as ::core::ffi::c_int, __stream);
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const CHAR_BIT: ::core::ffi::c_int = __CHAR_BIT__;
pub const UINTMAX_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const LOCALEDIR: [::core::ffi::c_char; 42] = unsafe {
    ::core::mem::transmute::<[u8; 42], [::core::ffi::c_char; 42]>(
        *b"/root/rboxc/build/oracle/tar/share/locale\0",
    )
};
pub const DEFAULT_RMT_COMMAND: [::core::ffi::c_char; 41] = unsafe {
    ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
        *b"/root/rboxc/build/oracle/tar/libexec/rmt\0",
    )
};
pub const PAXEXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PAXEXIT_FAILURE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BZIP2_PROGRAM: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"bzip2\0") };
pub const COMPRESS_PROGRAM: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"compress\0") };
pub const DEFAULT_ARCHIVE: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"-\0") };
pub const DEFAULT_BLOCKING: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const GZIP_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"gzip\0") };
pub const LZIP_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"lzip\0") };
pub const LZMA_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"lzma\0") };
pub const LZOP_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"lzop\0") };
pub const PACKAGE: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"tar\0") };
pub const XZ_PROGRAM: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"xz\0") };
pub const ZSTD_PROGRAM: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"zstd\0") };
pub const FNM_LEADING_DIR: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const OPTION_ARG_OPTIONAL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const OPTION_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPTION_ALIAS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const OPTION_DOC: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const OPTION_NO_TRANS: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_KEY_ARG: ::core::ffi::c_int = 0;
pub const ARGP_KEY_INIT: ::core::ffi::c_int = 16777219;
pub const ARGP_KEY_ERROR: ::core::ffi::c_int = 16777221;
pub const ARGP_KEY_HELP_EXTRA: ::core::ffi::c_int = 33554436;
pub const ARGP_NO_ERRS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const ARGP_IN_ORDER: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const ARGP_NO_EXIT: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const ARGP_HELP_SEE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const BLOCKSIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const TAREXIT_SUCCESS: ::core::ffi::c_int = PAXEXIT_SUCCESS;
pub const TAREXIT_FAILURE: ::core::ffi::c_int = PAXEXIT_FAILURE;
#[inline]
unsafe extern "C" fn get_stat_mtime(mut st: *const stat) -> timespec {
    return (*st).st_mtim;
}
#[inline]
unsafe extern "C" fn timespec_cmp(mut a: timespec, mut b: timespec) -> ::core::ffi::c_int {
    return 2 as ::core::ffi::c_int
        * ((a.tv_sec > b.tv_sec) as ::core::ffi::c_int
            - (a.tv_sec < b.tv_sec) as ::core::ffi::c_int)
        + ((a.tv_nsec > b.tv_nsec) as ::core::ffi::c_int
            - (a.tv_nsec < b.tv_nsec) as ::core::ffi::c_int);
}
#[export_name = "rboxc_tar_subcommand_option"]
pub static mut subcommand_option: subcommand = subcommand::UNKNOWN_SUBCOMMAND;
#[export_name = "rboxc_tar_archive_format"]
pub static mut archive_format_0: archive_format = archive_format::DEFAULT_FORMAT;
#[export_name = "rboxc_tar_blocking_factor"]
pub static mut blocking_factor: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_record_size"]
pub static mut record_size: size_t = 0;
#[export_name = "rboxc_tar_absolute_names_option"]
pub static mut absolute_names_option: bool = false;
#[export_name = "rboxc_tar_utc_option"]
pub static mut utc_option: bool = false;
#[export_name = "rboxc_tar_full_time_option"]
pub static mut full_time_option: bool = false;
#[export_name = "rboxc_tar_after_date_option"]
pub static mut after_date_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_atime_preserve_option"]
pub static mut atime_preserve_option: atime_preserve = atime_preserve::no_atime_preserve;
#[export_name = "rboxc_tar_backup_option"]
pub static mut backup_option: bool = false;
#[export_name = "rboxc_tar_backup_type"]
pub static mut backup_type_0: backup_type = backup_type::no_backups;
#[export_name = "rboxc_tar_block_number_option"]
pub static mut block_number_option: bool = false;
#[export_name = "rboxc_tar_checkpoint_option"]
pub static mut checkpoint_option: ::core::ffi::c_uint = 0;
pub const DEFAULT_CHECKPOINT: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[export_name = "rboxc_tar_use_compress_program_option"]
pub static mut use_compress_program_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_dereference_option"]
pub static mut dereference_option: bool = false;
#[export_name = "rboxc_tar_hard_dereference_option"]
pub static mut hard_dereference_option: bool = false;
#[export_name = "rboxc_tar_excluded"]
pub static mut excluded: *mut exclude = ::core::ptr::null_mut::<exclude>();
#[export_name = "rboxc_tar_group_name_option"]
pub static mut group_name_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_group_option"]
pub static mut group_option: gid_t = 0;
#[export_name = "rboxc_tar_ignore_failed_read_option"]
pub static mut ignore_failed_read_option: bool = false;
#[export_name = "rboxc_tar_ignore_zeros_option"]
pub static mut ignore_zeros_option: bool = false;
#[export_name = "rboxc_tar_incremental_option"]
pub static mut incremental_option: bool = false;
#[export_name = "rboxc_tar_info_script_option"]
pub static mut info_script_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_interactive_option"]
pub static mut interactive_option: bool = false;
#[export_name = "rboxc_tar_occurrence_option"]
pub static mut occurrence_option: uintmax_t = 0;
#[export_name = "rboxc_tar_old_files_option"]
pub static mut old_files_option: old_files = old_files::DEFAULT_OLD_FILES;
#[export_name = "rboxc_tar_keep_directory_symlink_option"]
pub static mut keep_directory_symlink_option: bool = false;
#[export_name = "rboxc_tar_listed_incremental_option"]
pub static mut listed_incremental_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_incremental_level"]
pub static mut incremental_level: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_check_device_option"]
pub static mut check_device_option: bool = false;
#[export_name = "rboxc_tar_mode_option"]
pub static mut mode_option: *mut mode_change = ::core::ptr::null_mut::<mode_change>();
#[export_name = "rboxc_tar_initial_umask"]
pub static mut initial_umask: mode_t = 0;
#[export_name = "rboxc_tar_multi_volume_option"]
pub static mut multi_volume_option: bool = false;
#[export_name = "rboxc_tar_newer_mtime_option"]
pub static mut newer_mtime_option: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
#[export_name = "rboxc_tar_set_mtime_option"]
pub static mut set_mtime_option: set_mtime_option_mode = set_mtime_option_mode::USE_FILE_MTIME;
#[export_name = "rboxc_tar_mtime_option"]
pub static mut mtime_option: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
#[export_name = "rboxc_tar_recursion_option"]
pub static mut recursion_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_numeric_owner_option"]
pub static mut numeric_owner_option: bool = false;
#[export_name = "rboxc_tar_one_file_system_option"]
pub static mut one_file_system_option: bool = false;
#[export_name = "rboxc_tar_one_top_level_option"]
pub static mut one_top_level_option: bool = false;
#[export_name = "rboxc_tar_one_top_level_dir"]
pub static mut one_top_level_dir: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_owner_name_option"]
pub static mut owner_name_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_owner_option"]
pub static mut owner_option: uid_t = 0;
#[export_name = "rboxc_tar_recursive_unlink_option"]
pub static mut recursive_unlink_option: bool = false;
#[export_name = "rboxc_tar_read_full_records_option"]
pub static mut read_full_records_option: bool = false;
#[export_name = "rboxc_tar_remove_files_option"]
pub static mut remove_files_option: bool = false;
#[export_name = "rboxc_tar_rsh_command_option"]
pub static mut rsh_command_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_same_order_option"]
pub static mut same_order_option: bool = false;
#[export_name = "rboxc_tar_same_owner_option"]
pub static mut same_owner_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_same_permissions_option"]
pub static mut same_permissions_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_selinux_context_option"]
pub static mut selinux_context_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_acls_option"]
pub static mut acls_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_xattrs_option"]
pub static mut xattrs_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_strip_name_components"]
pub static mut strip_name_components: size_t = 0;
#[export_name = "rboxc_tar_show_omitted_dirs_option"]
pub static mut show_omitted_dirs_option: bool = false;
#[export_name = "rboxc_tar_sparse_option"]
pub static mut sparse_option: bool = false;
#[export_name = "rboxc_tar_tar_sparse_major"]
pub static mut tar_sparse_major: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_tar_tar_sparse_minor"]
pub static mut tar_sparse_minor: ::core::ffi::c_uint = 0;
#[export_name = "rboxc_tar_hole_detection"]
pub static mut hole_detection: hole_detection_method =
    hole_detection_method::HOLE_DETECTION_DEFAULT;
#[export_name = "rboxc_tar_starting_file_option"]
pub static mut starting_file_option: bool = false;
#[export_name = "rboxc_tar_tape_length_option"]
pub static mut tape_length_option: tarlong = 0.;
#[export_name = "rboxc_tar_to_stdout_option"]
pub static mut to_stdout_option: bool = false;
#[export_name = "rboxc_tar_totals_option"]
pub static mut totals_option: bool = false;
#[export_name = "rboxc_tar_touch_option"]
pub static mut touch_option: bool = false;
#[export_name = "rboxc_tar_to_command_option"]
pub static mut to_command_option: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_ignore_command_error_option"]
pub static mut ignore_command_error_option: bool = false;
#[export_name = "rboxc_tar_restrict_option"]
pub static mut restrict_option: bool = false;
#[export_name = "rboxc_tar_verbose_option"]
pub static mut verbose_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_verify_option"]
pub static mut verify_option: bool = false;
#[export_name = "rboxc_tar_volno_file_option"]
pub static mut volno_file_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_volume_label_option"]
pub static mut volume_label_option: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_posixly_correct"]
pub static mut posixly_correct: bool = false;
#[export_name = "rboxc_tar_archive"]
pub static mut archive: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_dev_null_output"]
pub static mut dev_null_output: bool = false;
#[export_name = "rboxc_tar_start_time"]
pub static mut start_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
#[export_name = "rboxc_tar_volume_start_time"]
pub static mut volume_start_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
#[export_name = "rboxc_tar_last_stat_time"]
pub static mut last_stat_time: timespec = timespec {
    tv_sec: 0,
    tv_nsec: 0,
};
#[export_name = "rboxc_tar_current_stat_info"]
pub static mut current_stat_info: tar_stat_info = tar_stat_info {
    orig_file_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    file_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    had_trailing_slash: false,
    link_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    uname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    gname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    cntx_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    acls_a_ptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    acls_a_len: 0,
    acls_d_ptr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    acls_d_len: 0,
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
    atime: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    mtime: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    ctime: timespec {
        tv_sec: 0,
        tv_nsec: 0,
    },
    archive_file_size: 0,
    is_sparse: false,
    sparse_major: 0,
    sparse_minor: 0,
    sparse_map_avail: 0,
    sparse_map_size: 0,
    sparse_map: ::core::ptr::null_mut::<sp_array>(),
    real_size: 0,
    real_size_set: false,
    sparse_name_done: false,
    xattr_map: xattr_map {
        xm_map: ::core::ptr::null_mut::<xattr_array>(),
        xm_size: 0,
        xm_max: 0,
    },
    xhdr: xheader {
        stk: ::core::ptr::null_mut::<obstack>(),
        size: 0,
        buffer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        string_length: 0,
    },
    is_dumpdir: false,
    skipped: false,
    dumpdir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    parent: ::core::ptr::null_mut::<tar_stat_info>(),
    dirstream: ::core::ptr::null_mut::<DIR>(),
    fd: 0,
    exclude_list: ::core::ptr::null_mut::<exclist>(),
};
#[export_name = "rboxc_tar_archive_name_array"]
pub static mut archive_name_array: *mut *const ::core::ffi::c_char =
    ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
#[export_name = "rboxc_tar_archive_names"]
pub static mut archive_names: size_t = 0;
#[export_name = "rboxc_tar_archive_name_cursor"]
pub static mut archive_name_cursor: *mut *const ::core::ffi::c_char =
    ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
#[export_name = "rboxc_tar_index_file_name"]
pub static mut index_file_name: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_archive_stat"]
pub static mut archive_stat: stat = stat {
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
#[export_name = "rboxc_tar_open_read_flags"]
pub static mut open_read_flags: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_open_searchdir_flags"]
pub static mut open_searchdir_flags: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_fstatat_flags"]
pub static mut fstatat_flags: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_seek_option"]
pub static mut seek_option: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_seekable_archive"]
pub static mut seekable_archive: bool = false;
#[export_name = "rboxc_tar_root_device"]
pub static mut root_device: dev_t = 0;
#[export_name = "rboxc_tar_unquote_option"]
pub static mut unquote_option: bool = false;
#[export_name = "rboxc_tar_savedir_sort_order"]
pub static mut savedir_sort_order: ::core::ffi::c_int = 0;
#[export_name = "rboxc_tar_show_transformed_names_option"]
pub static mut show_transformed_names_option: bool = false;
#[export_name = "rboxc_tar_delay_directory_restore_option"]
pub static mut delay_directory_restore_option: bool = false;
#[inline]
unsafe extern "C" fn name_more_files() -> bool {
    return filename_args.0 != files_count::FILES_NONE.0;
}
pub const WARN_NEW_DIRECTORY: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const WARN_RENAME_DIRECTORY: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const WARN_DECOMPRESS_PROGRAM: ::core::ffi::c_int = 0x80000 as ::core::ffi::c_int;
pub const WARN_EXISTING_FILE: ::core::ffi::c_int = 0x100000 as ::core::ffi::c_int;
pub const WARN_RECORD_SIZE: ::core::ffi::c_int = 0x400000 as ::core::ffi::c_int;
pub const WARN_VERBOSE_WARNINGS: ::core::ffi::c_int = WARN_RENAME_DIRECTORY
    | WARN_NEW_DIRECTORY
    | WARN_DECOMPRESS_PROGRAM
    | WARN_EXISTING_FILE
    | WARN_RECORD_SIZE;
pub const EX_USAGE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const WRDSF_DOOFFS: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const WRDSF_NOCMD: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const WRDSF_NOVAR: ::core::ffi::c_int = 0x40 as ::core::ffi::c_int;
pub const WRDSF_SQUOTE: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const WRDSF_DQUOTE: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const WRDSF_QUOTE: ::core::ffi::c_int = WRDSF_SQUOTE | WRDSF_DQUOTE;
pub const WRDSF_SQUEEZE_DELIMS: ::core::ffi::c_int = 0x800 as ::core::ffi::c_int;
pub const WRDSF_CESCAPES: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const WRDSF_DEFFLAGS: ::core::ffi::c_int =
    WRDSF_NOVAR | WRDSF_NOCMD | WRDSF_QUOTE | WRDSF_SQUEEZE_DELIMS | WRDSF_CESCAPES;
#[inline]
unsafe extern "C" fn priv_set_remove_linkdir() -> ::core::ffi::c_int {
    return -1 as ::core::ffi::c_int;
}
static mut check_links_option: ::core::ffi::c_int = 0;
static mut allocated_archive_names: size_t = 0;
static mut stdin_used_by: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_tar_request_stdin"]
pub unsafe extern "C" fn request_stdin(mut rpl_option: *const ::core::ffi::c_char) {
    if !stdin_used_by.is_null() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Options '%s' and '%s' both want standard input\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                stdin_used_by,
                rpl_option,
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
                        b"Options '%s' and '%s' both want standard input\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    stdin_used_by,
                    rpl_option,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        usage(PAXEXIT_FAILURE);
    }
    stdin_used_by = rpl_option;
}
#[export_name = "rboxc_tar_confirm"]
pub unsafe extern "C" fn confirm(
    mut message_action: *const ::core::ffi::c_char,
    mut message_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut confirm_file: *mut FILE = ::core::ptr::null_mut::<FILE>();
    static mut confirm_file_EOF: ::core::ffi::c_int = 0;
    let mut status: bool = r#false != 0;
    if confirm_file.is_null() {
        if archive == 0 as ::core::ffi::c_int || !stdin_used_by.is_null() {
            confirm_file = fopen(
                TTY_NAME.as_ptr(),
                b"r\0".as_ptr() as *const ::core::ffi::c_char,
            ) as *mut FILE;
            if confirm_file.is_null() {
                open_fatal(TTY_NAME.as_ptr());
            }
        } else {
            request_stdin(b"-w\0".as_ptr() as *const ::core::ffi::c_char);
            confirm_file = stdin;
        }
    }
    fprintf(
        stdlis,
        b"%s %s?\0".as_ptr() as *const ::core::ffi::c_char,
        message_action,
        quote(message_name),
    );
    fflush_unlocked(stdlis);
    if confirm_file_EOF == 0 {
        let mut response: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut response_size: size_t = 0 as size_t;
        if getline(&raw mut response, &raw mut response_size, confirm_file) < 0 as __ssize_t {
            confirm_file_EOF = 1 as ::core::ffi::c_int;
        } else {
            status = rpmatch(response) > 0 as ::core::ffi::c_int;
        }
        free(response as *mut ::core::ffi::c_void);
    }
    if confirm_file_EOF != 0 {
        fputc_unlocked('\n' as ::core::ffi::c_int, stdlis);
        fflush_unlocked(stdlis);
    }
    return status as ::core::ffi::c_int;
}
static mut fmttab: [fmttab; 7] = [
    fmttab {
        name: b"v7\0".as_ptr() as *const ::core::ffi::c_char,
        fmt: archive_format::V7_FORMAT,
    },
    fmttab {
        name: b"oldgnu\0".as_ptr() as *const ::core::ffi::c_char,
        fmt: archive_format::OLDGNU_FORMAT,
    },
    fmttab {
        name: b"ustar\0".as_ptr() as *const ::core::ffi::c_char,
        fmt: archive_format::USTAR_FORMAT,
    },
    fmttab {
        name: b"posix\0".as_ptr() as *const ::core::ffi::c_char,
        fmt: archive_format::POSIX_FORMAT,
    },
    fmttab {
        name: b"gnu\0".as_ptr() as *const ::core::ffi::c_char,
        fmt: archive_format::GNU_FORMAT,
    },
    fmttab {
        name: b"pax\0".as_ptr() as *const ::core::ffi::c_char,
        fmt: archive_format::POSIX_FORMAT,
    },
    fmttab {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        fmt: archive_format(0 as ::core::ffi::c_int as ::core::ffi::c_uint),
    },
];
unsafe extern "C" fn set_archive_format(mut name: *const ::core::ffi::c_char) {
    let mut p: *const fmttab = ::core::ptr::null::<fmttab>();
    p = &raw const fmttab as *const fmttab as *const fmttab;
    while strcmp((*p).name, name) != 0 as ::core::ffi::c_int {
        p = p.offset(1);
        if (*p).name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s: Invalid archive format\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_colon(name),
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
                            b"%s: Invalid archive format\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_colon(name),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
    }
    archive_format_0 = (*p).fmt;
}
unsafe extern "C" fn set_xattr_option(mut value: ::core::ffi::c_int) {
    if value == 1 as ::core::ffi::c_int {
        set_archive_format(b"posix\0".as_ptr() as *const ::core::ffi::c_char);
    }
    xattrs_option = value;
}
#[export_name = "rboxc_tar_archive_format_string"]
pub unsafe extern "C" fn archive_format_string(
    mut fmt: archive_format,
) -> *const ::core::ffi::c_char {
    let mut p: *const fmttab = ::core::ptr::null::<fmttab>();
    p = &raw const fmttab as *const fmttab as *const fmttab;
    while !(*p).name.is_null() {
        if (*p).fmt.0 == fmt.0 {
            return (*p).name;
        }
        p = p.offset(1);
    }
    return b"unknown?\0".as_ptr() as *const ::core::ffi::c_char;
}
unsafe extern "C" fn assert_format(mut fmt_mask: ::core::ffi::c_uint) {
    if ((1 as ::core::ffi::c_int) << archive_format_0.0) as ::core::ffi::c_uint & fmt_mask
        == 0 as ::core::ffi::c_uint
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"GNU features wanted on incompatible archive format\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"GNU features wanted on incompatible archive format\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
}
#[export_name = "rboxc_tar_subcommand_string"]
pub unsafe extern "C" fn subcommand_string(mut c: subcommand) -> *const ::core::ffi::c_char {
    match c {
        subcommand::UNKNOWN_SUBCOMMAND => {
            return b"unknown?\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::APPEND_SUBCOMMAND => {
            return b"-r\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::CAT_SUBCOMMAND => {
            return b"-A\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::CREATE_SUBCOMMAND => {
            return b"-c\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::DELETE_SUBCOMMAND => {
            return b"-D\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::DIFF_SUBCOMMAND => {
            return b"-d\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::EXTRACT_SUBCOMMAND => {
            return b"-x\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::LIST_SUBCOMMAND => {
            return b"-t\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::UPDATE_SUBCOMMAND => {
            return b"-u\0".as_ptr() as *const ::core::ffi::c_char;
        }
        subcommand::TEST_LABEL_SUBCOMMAND => {
            return b"--test-label\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    abort();
}
unsafe extern "C" fn tar_list_quoting_styles(
    mut stk: *mut obstack,
    mut prefix: *const ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut prefixlen: size_t = strlen(prefix);
    i = 0 as ::core::ffi::c_int;
    while !(*(&raw const quoting_style_args as *const *const ::core::ffi::c_char)
        .offset(i as isize))
    .is_null()
    {
        let mut __o: *mut obstack = stk;
        let mut __len: size_t = prefixlen;
        if ({
            let mut __o1: *const obstack = __o;
            (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
        }) < __len
        {
            _obstack_newchunk(__o, __len);
        }
        memcpy(
            (*__o).next_free as *mut ::core::ffi::c_void,
            prefix as *const ::core::ffi::c_void,
            __len,
        );
        (*__o).next_free = (*__o).next_free.offset(__len as isize);
        let mut __o_0: *mut obstack = stk;
        let mut __len_0: size_t = strlen(
            *(&raw const quoting_style_args as *const *const ::core::ffi::c_char)
                .offset(i as isize),
        );
        if ({
            let mut __o1: *const obstack = __o_0;
            (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
        }) < __len_0
        {
            _obstack_newchunk(__o_0, __len_0);
        }
        memcpy(
            (*__o_0).next_free as *mut ::core::ffi::c_void,
            *(&raw const quoting_style_args as *const *const ::core::ffi::c_char).offset(i as isize)
                as *const ::core::ffi::c_void,
            __len_0,
        );
        (*__o_0).next_free = (*__o_0).next_free.offset(__len_0 as isize);
        let mut __o_1: *mut obstack = stk;
        if ({
            let mut __o1: *const obstack = __o_1;
            (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
        }) < 1 as size_t
        {
            _obstack_newchunk(__o_1, 1 as size_t);
        }
        let c2rust_fresh3 = (*__o_1).next_free;
        (*__o_1).next_free = (*__o_1).next_free.offset(1);
        *c2rust_fresh3 = '\n' as ::core::ffi::c_char;
        i += 1;
    }
}
unsafe extern "C" fn tar_set_quoting_style(mut arg: *mut ::core::ffi::c_char) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while !(*(&raw const quoting_style_args as *const *const ::core::ffi::c_char)
        .offset(i as isize))
    .is_null()
    {
        if strcmp(
            arg,
            *(&raw const quoting_style_args as *const *const ::core::ffi::c_char)
                .offset(i as isize),
        ) == 0 as ::core::ffi::c_int
        {
            set_quoting_style(
                ::core::ptr::null_mut::<quoting_options>(),
                quoting_style(i as ::core::ffi::c_uint),
            );
            return;
        }
        i += 1;
    }
    if error_hook.is_some() {
        error_hook.expect("non-null function pointer")();
    }
    if 0 != 0 {
        error(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            ),
            arg,
            program_name,
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
                    b"Unknown quoting style '%s'. Try '%s --quoting-style=help' to get a list.\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                arg,
                program_name,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
    fatal_exit();
}
static mut doc: [::core::ffi::c_char; 702] = unsafe {
    ::core::mem::transmute::<
        [u8; 702],
        [::core::ffi::c_char; 702],
    >(
        *b"GNU 'tar' saves many files together into a single tape or disk archive, and can restore individual files from the archive.\n\nExamples:\n  tar -cf archive.tar foo bar  # Create archive.tar from files foo and bar.\n  tar -tvf archive.tar         # List all files in archive.tar verbosely.\n  tar -xf archive.tar          # Extract all files from archive.tar.\n\x0BThe backup suffix is '~', unless set with --suffix or SIMPLE_BACKUP_SUFFIX.\nThe version control may be set with --backup or VERSION_CONTROL, values are:\n\n  none, off       never make backups\n  t, numbered     make numbered backups\n  nil, existing   numbered if numbered backups exist, simple otherwise\n  never, simple   always make simple backups\n\0",
    )
};
static mut options: [argp_option; 169] = [
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Main operation mode:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"list\0".as_ptr() as *const ::core::ffi::c_char,
        key: 't' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"list the contents of an archive\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"extract\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'x' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"extract files from an archive\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"get\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"create\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'c' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"create a new archive\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"diff\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'd' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"find differences between archive and file system\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"compare\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"append\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'r' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"append files to the end of an archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"update\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'u' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"only append files newer than copy in archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"catenate\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'A' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"append tar files to an archive\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"concatenate\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"delete\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::DELETE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"delete from the archive (not on mag tapes!)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"test-label\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::TEST_LABEL_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"test the archive volume label and exit\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMMAND.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"sparse\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'S' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"handle sparse files efficiently\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"hole-detection\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::HOLE_DETECTION_OPTION.0 as ::core::ffi::c_int,
        arg: b"TYPE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"technique to detect holes\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"sparse-version\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SPARSE_VERSION_OPTION.0 as ::core::ffi::c_int,
        arg: b"MAJOR[.MINOR]\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set version of the sparse format to use (implies --sparse)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"incremental\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'G' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"handle old GNU-format incremental backup\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"listed-incremental\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'g' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"handle new GNU-format incremental backup\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"level\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::LEVEL_OPTION.0 as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"dump level for created listed-incremental archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ignore-failed-read\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::IGNORE_FAILED_READ_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not exit with nonzero on unreadable files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"occurrence\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OCCURRENCE_OPTION.0 as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"process only the NUMBERth occurrence of each file in the archive; this option is valid only in conjunction with one of the subcommands --delete, --diff, --extract or --list and when a list of files is given either on the command line or via the -T option; NUMBER defaults to 1\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"seek\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'n' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"archive is seekable\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-seek\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_SEEK_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"archive is not seekable\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-check-device\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_CHECK_DEVICE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not check device numbers when creating incremental archives\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"check-device\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::CHECK_DEVICE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"check device numbers when creating incremental archives (default)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_MODIFIER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Overwrite control:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"verify\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'W' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"attempt to verify the archive after writing it\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"remove-files\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::REMOVE_FILES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"remove files after adding them to the archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"keep-old-files\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'k' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"don't replace existing files when extracting, treat them as errors\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"skip-old-files\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SKIP_OLD_FILES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"don't replace existing files when extracting, silently skip over them\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"keep-newer-files\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::KEEP_NEWER_FILES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"don't replace existing files that are newer than their archive copies\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"overwrite\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OVERWRITE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"overwrite existing files when extracting\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"unlink-first\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'U' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"remove each file prior to extracting over it\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"recursive-unlink\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::RECURSIVE_UNLINK_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"empty hierarchies prior to extracting directory\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-overwrite-dir\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_OVERWRITE_DIR_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"preserve metadata of existing directories\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"overwrite-dir\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OVERWRITE_DIR_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"overwrite metadata of existing directories when extracting (default)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"keep-directory-symlink\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::KEEP_DIRECTORY_SYMLINK_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"preserve existing symlinks to directories when extracting\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"one-top-level\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::ONE_TOP_LEVEL_OPTION.0 as ::core::ffi::c_int,
        arg: b"DIR\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"create a subdirectory to avoid having loose files extracted\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OVERWRITE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Select output stream:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_OUTPUT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'O' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"extract files to standard output\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OUTPUT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"to-command\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::TO_COMMAND_OPTION.0 as ::core::ffi::c_int,
        arg: b"COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"pipe extracted files to another program\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OUTPUT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ignore-command-error\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::IGNORE_COMMAND_ERROR_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"ignore exit codes of children\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OUTPUT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-ignore-command-error\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_IGNORE_COMMAND_ERROR_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"treat non-zero exit codes of children as error\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_OUTPUT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Handling of file attributes:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"owner\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OWNER_OPTION.0 as ::core::ffi::c_int,
        arg: b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"force NAME as owner for added files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"group\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::GROUP_OPTION.0 as ::core::ffi::c_int,
        arg: b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"force NAME as group for added files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"owner-map\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OWNER_MAP_OPTION.0 as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use FILE to map file owner UIDs and names\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"group-map\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::GROUP_MAP_OPTION.0 as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use FILE to map file owner GIDs and names\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"mtime\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::MTIME_OPTION.0 as ::core::ffi::c_int,
        arg: b"DATE-OR-FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set mtime for added files from DATE-OR-FILE\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"clamp-mtime\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::CLAMP_MTIME_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"only set time when the file is more recent than what was given with --mtime\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::MODE_OPTION.0 as ::core::ffi::c_int,
        arg: b"CHANGES\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"force (symbolic) mode CHANGES for added files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"atime-preserve\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::ATIME_PRESERVE_OPTION.0 as ::core::ffi::c_int,
        arg: b"METHOD\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"preserve access times on dumped files, either by restoring the times after reading (METHOD='replace'; default) or by not setting the times in the first place (METHOD='system')\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"touch\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'm' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"don't extract file modified time\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"same-owner\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SAME_OWNER_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"try extracting files with the same ownership as exists in the archive (default for superuser)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-same-owner\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_SAME_OWNER_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"extract files as yourself (default for ordinary users)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"numeric-owner\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NUMERIC_OWNER_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"always use numbers for user/group names\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"preserve-permissions\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"extract information about file permissions (default for superuser)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"same-permissions\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-same-permissions\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_SAME_PERMISSIONS_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"apply the user's umask when extracting permissions from the archive (default for ordinary users)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"preserve-order\0".as_ptr() as *const ::core::ffi::c_char,
        key: 's' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"member arguments are listed in the same order as the files in the archive\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"same-order\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"delay-directory-restore\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::DELAY_DIRECTORY_RESTORE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"delay setting modification times and permissions of extracted directories until the end of extraction\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-delay-directory-restore\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_DELAY_DIRECTORY_RESTORE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"cancel the effect of --delay-directory-restore option\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"sort\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SORT_OPTION.0 as ::core::ffi::c_int,
        arg: b"ORDER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"directory sorting order: none (default), name or inode\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Handling of extended file attributes:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"xattrs\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::XATTR_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Enable extended attributes support\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-xattrs\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_XATTR_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Disable extended attributes support\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"xattrs-include\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::XATTR_INCLUDE.0 as ::core::ffi::c_int,
        arg: b"MASK\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"specify the include pattern for xattr keys\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"xattrs-exclude\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::XATTR_EXCLUDE.0 as ::core::ffi::c_int,
        arg: b"MASK\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"specify the exclude pattern for xattr keys\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"selinux\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SELINUX_CONTEXT_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Enable the SELinux context support\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-selinux\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_SELINUX_CONTEXT_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Disable the SELinux context support\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"acls\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::ACLS_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Enable the POSIX ACLs support\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-acls\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_ACLS_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Disable the POSIX ACLs support\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_XATTR.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Device selection and switching:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: b"ARCHIVE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use archive file or device ARCHIVE\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '0' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '1' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '2' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '3' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '4' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '5' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '6' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '7' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '8' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: '9' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_HIDDEN,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"force-local\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::FORCE_LOCAL_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"archive file is local even if it has a colon\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rmt-command\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::RMT_COMMAND_OPTION.0 as ::core::ffi::c_int,
        arg: b"COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use given rmt COMMAND instead of rmt\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rsh-command\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::RSH_COMMAND_OPTION.0 as ::core::ffi::c_int,
        arg: b"COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use remote COMMAND instead of rsh\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"multi-volume\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'M' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"create/list/extract multi-volume archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"tape-length\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'L' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"change tape after writing NUMBER x 1024 bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"info-script\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'F' as ::core::ffi::c_int,
        arg: b"NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"run script at end of each tape (implies -M)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"new-volume-script\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"volno-file\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::VOLNO_FILE_OPTION.0 as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use/update the volume number in FILE\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_DEVICE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Device blocking:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_BLOCKING.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"blocking-factor\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'b' as ::core::ffi::c_int,
        arg: b"BLOCKS\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"BLOCKS x 512 bytes per record\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_BLOCKING.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"record-size\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::RECORD_SIZE_OPTION.0 as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"NUMBER of bytes per record, multiple of 512\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_BLOCKING.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ignore-zeros\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'i' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"ignore zeroed blocks in archive (means EOF)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_BLOCKING.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"read-full-records\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'B' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"reblock as we read (for 4.2BSD pipes)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_BLOCKING.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Archive format selection:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"format\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'H' as ::core::ffi::c_int,
        arg: b"FORMAT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"create archive of the given format\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"FORMAT is one of the following:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"  v7\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_DOC | OPTION_NO_TRANS,
        doc: b"old V7 tar format\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"  oldgnu\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_DOC | OPTION_NO_TRANS,
        doc: b"GNU format as per tar <= 1.12\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"  gnu\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_DOC | OPTION_NO_TRANS,
        doc: b"GNU tar 1.13.x format\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"  ustar\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_DOC | OPTION_NO_TRANS,
        doc: b"POSIX 1003.1-1988 (ustar) format\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"  pax\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_DOC | OPTION_NO_TRANS,
        doc: b"POSIX 1003.1-2001 (pax) format\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"  posix\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_DOC | OPTION_NO_TRANS,
        doc: b"same as pax\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRDOC_FORMAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"old-archive\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::OLD_ARCHIVE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"same as --format=v7\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FORMAT_OPT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"portability\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_FORMAT_OPT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"posix\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::POSIX_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"same as --format=posix\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FORMAT_OPT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"pax-option\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::PAX_OPTION.0 as ::core::ffi::c_int,
        arg: b"keyword[[:]=value][,keyword[[:]=value]]...\0".as_ptr()
            as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"control pax keywords\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FORMAT_OPT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"label\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'V' as ::core::ffi::c_int,
        arg: b"TEXT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"create archive with volume name TEXT; at list/extract time, use TEXT as a globbing pattern for volume name\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FORMAT_OPT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Compression options:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"auto-compress\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'a' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"use archive suffix to determine the compression program\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-auto-compress\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_AUTO_COMPRESS_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"do not use archive suffix to determine the compression program\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"use-compress-program\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'I' as ::core::ffi::c_int,
        arg: b"PROG\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"filter through PROG (must accept -d)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"bzip2\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'j' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"gzip\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'z' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"gunzip\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"ungzip\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"compress\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'Z' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"uncompress\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"lzip\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::LZIP_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"lzma\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::LZMA_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"lzop\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::LZOP_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"xz\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'J' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"zstd\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::ZSTD_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_COMPRESS.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Local file selection:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"one-file-system\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::ONE_FILE_SYSTEM_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"stay in local file system when creating archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"absolute-names\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'P' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"don't strip leading '/'s from file names\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"dereference\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'h' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"follow symlinks; archive and dump the files they point to\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"hard-dereference\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::HARD_DEREFERENCE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"follow hard links; archive and dump the files they refer to\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"starting-file\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'K' as ::core::ffi::c_int,
        arg: b"MEMBER-NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"begin at member MEMBER-NAME when reading the archive\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"newer\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'N' as ::core::ffi::c_int,
        arg: b"DATE-OR-FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"only store files newer than DATE-OR-FILE\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"after-date\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"newer-mtime\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NEWER_MTIME_OPTION.0 as ::core::ffi::c_int,
        arg: b"DATE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"compare date and time when data changed only\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"backup\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::BACKUP_OPTION.0 as ::core::ffi::c_int,
        arg: b"CONTROL\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"backup before removal, choose version CONTROL\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"suffix\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SUFFIX_OPTION.0 as ::core::ffi::c_int,
        arg: b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"backup before removal, override usual suffix ('~' unless overridden by environment variable SIMPLE_BACKUP_SUFFIX)\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_FILE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"File name transformations:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_NAME_XFORM.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"strip-components\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::STRIP_COMPONENTS_OPTION.0 as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"strip NUMBER leading components from file names on extraction\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_NAME_XFORM.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"transform\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::TRANSFORM_OPTION.0 as ::core::ffi::c_int,
        arg: b"EXPRESSION\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"use sed replace EXPRESSION to transform file names\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_NAME_XFORM.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"xform\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_NAME_XFORM.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Informative output:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"checkpoint\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::CHECKPOINT_OPTION.0 as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"display progress messages every NUMBERth record (default 10)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"checkpoint-action\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::CHECKPOINT_ACTION_OPTION.0 as ::core::ffi::c_int,
        arg: b"ACTION\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"execute ACTION on each checkpoint\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"check-links\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'l' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"print a message if not all links are dumped\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"totals\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::TOTALS_OPTION.0 as ::core::ffi::c_int,
        arg: b"SIGNAL\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_ARG_OPTIONAL,
        doc: b"print total bytes after processing the archive; with an argument - print total bytes when this SIGNAL is delivered; Allowed signals are: SIGHUP, SIGQUIT, SIGINT, SIGUSR1 and SIGUSR2; the names without SIG prefix are also accepted\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"utc\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::UTC_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"print file modification times in UTC\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"full-time\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::FULL_TIME_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"print file time to its full resolution\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"index-file\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::INDEX_FILE_OPTION.0 as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"send verbose output to FILE\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"block-number\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'R' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"show block number within archive with each message\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"show-defaults\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SHOW_DEFAULTS_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"show tar defaults\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"show-snapshot-field-ranges\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SHOW_SNAPSHOT_FIELD_RANGES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"show valid ranges for snapshot-file fields\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"show-omitted-dirs\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SHOW_OMITTED_DIRS_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"when listing or extracting, list each directory that does not match search criteria\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"show-transformed-names\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::SHOW_TRANSFORMED_NAMES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"show file or archive names after transformation\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"show-stored-names\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"quoting-style\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::QUOTING_STYLE_OPTION.0 as ::core::ffi::c_int,
        arg: b"STYLE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"set name quoting style; see below for valid STYLE values\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"quote-chars\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::QUOTE_CHARS_OPTION.0 as ::core::ffi::c_int,
        arg: b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"additionally quote characters from STRING\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-quote-chars\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::NO_QUOTE_CHARS_OPTION.0 as ::core::ffi::c_int,
        arg: b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"disable quoting for characters from STRING\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"interactive\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'w' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"ask for confirmation for every action\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"confirmation\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'v' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"verbosely list files processed\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::WARNING_OPTION.0 as ::core::ffi::c_int,
        arg: b"KEYWORD\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"warning control\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_INFORMATIVE.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Compatibility options:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_COMPAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'o' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"when creating, same as --old-archive; when extracting, same as --no-same-owner\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRID_COMPAT.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Other options:\0".as_ptr() as *const ::core::ffi::c_char,
        group: C2Rust_Unnamed_4::GRH_OTHER.0 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"restrict\0".as_ptr() as *const ::core::ffi::c_char,
        key: C2Rust_Unnamed_3::RESTRICT_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"disable use of some potentially harmful options\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: -1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: 0 as ::core::ffi::c_int,
    },
];
static mut atime_preserve_args: [*const ::core::ffi::c_char; 3] = [
    b"replace\0".as_ptr() as *const ::core::ffi::c_char,
    b"system\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut atime_preserve_types: [atime_preserve; 2] = [
    atime_preserve::replace_atime_preserve,
    atime_preserve::system_atime_preserve,
];
unsafe extern "C" fn easprintf(
    mut format: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> *mut ::core::ffi::c_char {
    let mut args: ::core::ffi::VaList;
    args = c2rust_args.clone();
    let mut result: *mut ::core::ffi::c_char = xvasprintf(format, args);
    let mut err: ::core::ffi::c_int = *__errno_location();
    if result.is_null() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                err,
                b"vasprintf\0".as_ptr() as *const ::core::ffi::c_char,
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
                    err,
                    b"vasprintf\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        fatal_exit();
    }
    return result;
}
unsafe extern "C" fn format_default_settings() -> *mut ::core::ffi::c_char {
    return easprintf(
        b"--format=%s -f%s -b%d --quoting-style=%s --rmt-command=%s\0".as_ptr()
            as *const ::core::ffi::c_char,
        archive_format_string(archive_format::GNU_FORMAT),
        DEFAULT_ARCHIVE.as_ptr(),
        DEFAULT_BLOCKING,
        *(&raw const quoting_style_args as *const *const ::core::ffi::c_char)
            .offset(quoting_style::escape_quoting_style.0 as ::core::ffi::c_int as isize),
        DEFAULT_RMT_COMMAND.as_ptr(),
    );
}
unsafe extern "C" fn option_conflict_error(
    mut a: *const ::core::ffi::c_char,
    mut b: *const ::core::ffi::c_char,
) {
    if error_hook.is_some() {
        error_hook.expect("non-null function pointer")();
    }
    if 0 != 0 {
        error(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"'%s' cannot be used with '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            ),
            a,
            b,
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
                    b"'%s' cannot be used with '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                a,
                b,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
    usage(PAXEXIT_FAILURE);
}
static mut option_class: [*mut option_locus; 10] = [::core::ptr::null_mut::<option_locus>(); 10];
unsafe extern "C" fn optloc_save(
    mut id: ::core::ffi::c_uint,
    mut loc: *mut option_locus,
) -> *mut option_locus {
    let mut optloc: *mut option_locus = ::core::ptr::null_mut::<option_locus>();
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut s: size_t = 0;
    if id as usize
        >= ::core::mem::size_of::<[*mut option_locus; 10]>()
            .wrapping_div(::core::mem::size_of::<*mut option_locus>())
    {
        abort();
    }
    s = ::core::mem::size_of::<option_locus>() as size_t;
    if !(*loc).name.is_null() {
        s = s.wrapping_add(strlen((*loc).name).wrapping_add(1 as size_t));
    }
    optloc = xmalloc(s) as *mut option_locus;
    if !(*loc).name.is_null() {
        p = (optloc as *mut ::core::ffi::c_char)
            .offset(::core::mem::size_of::<option_locus>() as isize);
        strcpy(p, (*loc).name);
        (*optloc).name = p;
    } else {
        (*optloc).name = ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*optloc).source = (*loc).source;
    (*optloc).line = (*loc).line;
    (*optloc).prev = option_class[id as usize];
    option_class[id as usize] = optloc;
    return (*optloc).prev;
}
unsafe extern "C" fn optloc_lookup(mut id: ::core::ffi::c_int) -> *mut option_locus {
    return option_class[id as usize];
}
unsafe extern "C" fn option_set_in_cl(mut id: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mut loc: *mut option_locus = optloc_lookup(id);
    if loc.is_null() {
        return 0 as ::core::ffi::c_int;
    }
    return ((*loc).source.0 == option_source::OPTS_COMMAND_LINE.0) as ::core::ffi::c_int;
}
unsafe extern "C" fn optloc_eq(
    mut a: *mut option_locus,
    mut b: *mut option_locus,
) -> ::core::ffi::c_int {
    if !a.is_null() {
    } else {
        unreachable!();
    };
    if (*a).source.0 != (*b).source.0 {
        return 0 as ::core::ffi::c_int;
    }
    if (*a).source.0 == option_source::OPTS_COMMAND_LINE.0 {
        return 1 as ::core::ffi::c_int;
    }
    return (strcmp((*a).name, (*b).name) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn set_subcommand_option(mut subcommand_0: subcommand) {
    if subcommand_option.0 != subcommand::UNKNOWN_SUBCOMMAND.0
        && subcommand_option.0 != subcommand_0.0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option\0"
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
                        b"You may not specify more than one '-Acdtrux', '--delete' or  '--test-label' option\0"
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
        usage(PAXEXIT_FAILURE);
    }
    subcommand_option = subcommand_0;
}
unsafe extern "C" fn set_use_compress_program_option(
    mut string: *const ::core::ffi::c_char,
    mut loc: *mut option_locus,
) {
    let mut p: *mut option_locus = optloc_save(option_class_0::OC_COMPRESS.0, loc);
    if !use_compress_program_option.is_null()
        && strcmp(use_compress_program_option, string) != 0 as ::core::ffi::c_int
        && (*p).source.0 == option_source::OPTS_COMMAND_LINE.0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Conflicting compression options\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"Conflicting compression options\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        usage(PAXEXIT_FAILURE);
    }
    use_compress_program_option = string;
}
unsafe extern "C" fn sigstat(mut signo: ::core::ffi::c_int) {
    compute_duration();
    print_total_stats();
    signal(
        signo,
        Some(sigstat as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
unsafe extern "C" fn stat_on_signal(mut signo: ::core::ffi::c_int) {
    signal(
        signo,
        Some(sigstat as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
#[export_name = "rboxc_tar_decode_signal"]
pub unsafe extern "C" fn decode_signal(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut sigtab: [sigtab; 5] = [
        sigtab {
            name: b"USR1\0".as_ptr() as *const ::core::ffi::c_char,
            signo: SIGUSR1,
        },
        sigtab {
            name: b"USR2\0".as_ptr() as *const ::core::ffi::c_char,
            signo: SIGUSR2,
        },
        sigtab {
            name: b"HUP\0".as_ptr() as *const ::core::ffi::c_char,
            signo: SIGHUP,
        },
        sigtab {
            name: b"INT\0".as_ptr() as *const ::core::ffi::c_char,
            signo: SIGINT,
        },
        sigtab {
            name: b"QUIT\0".as_ptr() as *const ::core::ffi::c_char,
            signo: SIGQUIT,
        },
    ];
    let mut p: *const sigtab = ::core::ptr::null::<sigtab>();
    let mut s: *const ::core::ffi::c_char = name;
    if strncmp(
        s,
        b"SIG\0".as_ptr() as *const ::core::ffi::c_char,
        3 as size_t,
    ) == 0 as ::core::ffi::c_int
    {
        s = s.offset(3 as ::core::ffi::c_int as isize);
    }
    p = &raw const sigtab as *const sigtab as *const sigtab;
    while p
        < (&raw const sigtab as *const sigtab).offset(
            ::core::mem::size_of::<[sigtab; 5]>().wrapping_div(::core::mem::size_of::<sigtab>())
                as isize,
        )
    {
        if strcmp((*p).name, s) == 0 as ::core::ffi::c_int {
            return (*p).signo;
        }
        p = p.offset(1);
    }
    if error_hook.is_some() {
        error_hook.expect("non-null function pointer")();
    }
    if 0 != 0 {
        error(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Unknown signal name: %s\0".as_ptr() as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            ),
            name,
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
                    b"Unknown signal name: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                name,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
    fatal_exit();
    panic!("Reached end of non-void function without returning");
}
unsafe extern "C" fn set_stat_signal(mut name: *const ::core::ffi::c_char) {
    stat_on_signal(decode_signal(name));
}
unsafe extern "C" fn get_date_or_file(
    mut args: *mut tar_args,
    mut rpl_option: *const ::core::ffi::c_char,
    mut str: *const ::core::ffi::c_char,
    mut ts: *mut timespec,
) -> ::core::ffi::c_int {
    if 0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        || *str as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        || *str as ::core::ffi::c_int == '.' as ::core::ffi::c_int
    {
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
        if stat(str, &raw mut st) != 0 as ::core::ffi::c_int {
            stat_error(str);
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Date sample file not found\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"Date sample file not found\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        *ts = get_stat_mtime(&raw mut st);
    } else if !parse_datetime(ts, str, ::core::ptr::null::<timespec>()) {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Substituting %s for unknown date format %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                tartime(*ts, false),
                quote(str),
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
                        b"Substituting %s for unknown date format %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    tartime(*ts, false),
                    quote(str),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        (*ts).tv_nsec = 0 as __syscall_slong_t;
        return 1 as ::core::ffi::c_int;
    } else {
        let mut p: *mut textual_date =
            xmalloc(::core::mem::size_of::<textual_date>()) as *mut textual_date;
        (*p).ts = *ts;
        (*p).rpl_option = rpl_option;
        (*p).date = xstrdup(str);
        (*p).next = (*args).textual_date as *mut textual_date;
        (*args).textual_date = p as *mut textual_date;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn report_textual_dates(mut args: *mut tar_args) {
    let mut p: *mut textual_date = ::core::ptr::null_mut::<textual_date>();
    p = (*args).textual_date as *mut textual_date;
    while !p.is_null() {
        let mut next: *mut textual_date = (*p).next;
        if verbose_option != 0 {
            let mut treated_as: *const ::core::ffi::c_char = tartime((*p).ts, r#true != 0);
            if strcmp((*p).date, treated_as) != 0 as ::core::ffi::c_int {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Option %s: Treating date '%s' as %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        (*p).rpl_option,
                        (*p).date,
                        treated_as,
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
                                b"Option %s: Treating date '%s' as %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            (*p).rpl_option,
                            (*p).date,
                            treated_as,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        }
        free((*p).date as *mut ::core::ffi::c_void);
        free(p as *mut ::core::ffi::c_void);
        p = next;
    }
}
unsafe extern "C" fn tar_help_filter(
    mut key: ::core::ffi::c_int,
    mut text: *const ::core::ffi::c_char,
    mut input: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_char {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: ::core::ptr::null_mut::<_obstack_chunk>(),
        object_base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        next_free: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        chunk_limit: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        temp: C2Rust_Unnamed_2 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2Rust_Unnamed_1 { plain: None },
        freefun: C2Rust_Unnamed_0 { plain: None },
        extra_arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    match key {
        106 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                BZIP2_PROGRAM.as_ptr(),
            );
        }
        122 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                GZIP_PROGRAM.as_ptr(),
            );
        }
        90 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                COMPRESS_PROGRAM.as_ptr(),
            );
        }
        148 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                LZIP_PROGRAM.as_ptr(),
            );
        }
        149 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                LZMA_PROGRAM.as_ptr(),
            );
        }
        150 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                LZOP_PROGRAM.as_ptr(),
            );
        }
        74 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                XZ_PROGRAM.as_ptr(),
            );
        }
        207 => {
            s = easprintf(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"filter the archive through %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                ZSTD_PROGRAM.as_ptr(),
            );
        }
        ARGP_KEY_HELP_EXTRA => {
            let mut tstr: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
            _obstack_begin(
                &raw mut stk,
                0 as size_t,
                0 as size_t,
                Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
                Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
            );
            tstr = dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Valid arguments for the --quoting-style option are:\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            );
            let mut __o: *mut obstack = &raw mut stk;
            let mut __len: size_t = strlen(tstr);
            if ({
                let mut __o1: *const obstack = __o;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            memcpy(
                (*__o).next_free as *mut ::core::ffi::c_void,
                tstr as *const ::core::ffi::c_void,
                __len,
            );
            (*__o).next_free = (*__o).next_free.offset(__len as isize);
            let mut __o_0: *mut obstack = &raw mut stk;
            let mut __len_0: size_t = 2 as size_t;
            if ({
                let mut __o1: *const obstack = __o_0;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < __len_0
            {
                _obstack_newchunk(__o_0, __len_0);
            }
            memcpy(
                (*__o_0).next_free as *mut ::core::ffi::c_void,
                b"\n\n\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                __len_0,
            );
            (*__o_0).next_free = (*__o_0).next_free.offset(__len_0 as isize);
            tar_list_quoting_styles(&raw mut stk, b"  \0".as_ptr() as *const ::core::ffi::c_char);
            tstr = dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"\n*This* tar defaults to:\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            );
            let mut __o_1: *mut obstack = &raw mut stk;
            let mut __len_1: size_t = strlen(tstr);
            if ({
                let mut __o1: *const obstack = __o_1;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < __len_1
            {
                _obstack_newchunk(__o_1, __len_1);
            }
            memcpy(
                (*__o_1).next_free as *mut ::core::ffi::c_void,
                tstr as *const ::core::ffi::c_void,
                __len_1,
            );
            (*__o_1).next_free = (*__o_1).next_free.offset(__len_1 as isize);
            s = format_default_settings();
            let mut __o_2: *mut obstack = &raw mut stk;
            let mut __len_2: size_t = strlen(s);
            if ({
                let mut __o1: *const obstack = __o_2;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < __len_2
            {
                _obstack_newchunk(__o_2, __len_2);
            }
            memcpy(
                (*__o_2).next_free as *mut ::core::ffi::c_void,
                s as *const ::core::ffi::c_void,
                __len_2,
            );
            (*__o_2).next_free = (*__o_2).next_free.offset(__len_2 as isize);
            free(s.cast());
            let mut __o_3: *mut obstack = &raw mut stk;
            if ({
                let mut __o1: *const obstack = __o_3;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < 1 as size_t
            {
                _obstack_newchunk(__o_3, 1 as size_t);
            }
            let c2rust_fresh1 = (*__o_3).next_free;
            (*__o_3).next_free = (*__o_3).next_free.offset(1);
            *c2rust_fresh1 = '\n' as ::core::ffi::c_char;
            let mut __o_4: *mut obstack = &raw mut stk;
            if ({
                let mut __o1: *const obstack = __o_4;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < 1 as size_t
            {
                _obstack_newchunk(__o_4, 1 as size_t);
            }
            let c2rust_fresh2 = (*__o_4).next_free;
            (*__o_4).next_free = (*__o_4).next_free.offset(1);
            *c2rust_fresh2 = 0 as ::core::ffi::c_char;
            s = xstrdup(
                ({
                    let mut __o1: *mut obstack = &raw mut stk;
                    let mut __value: *mut ::core::ffi::c_void =
                        (*__o1).object_base as *mut ::core::ffi::c_void;
                    if (*__o1).next_free == __value as *mut ::core::ffi::c_char {
                        (*__o1).set_maybe_empty_object(
                            1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                    }
                    (*__o1).next_free = if ::core::mem::size_of::<ptrdiff_t>()
                        < ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                    {
                        (*__o1).object_base
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    }
                    .offset(
                        (((*__o1).next_free.offset_from(
                            if ::core::mem::size_of::<ptrdiff_t>()
                                < ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                            {
                                (*__o1).object_base
                            } else {
                                ::core::ptr::null_mut::<::core::ffi::c_char>()
                            },
                        ) as size_t)
                            .wrapping_add((*__o1).alignment_mask)
                            & !(*__o1).alignment_mask) as isize,
                    );
                    if (*__o1)
                        .next_free
                        .offset_from((*__o1).chunk as *mut ::core::ffi::c_char)
                        as size_t
                        > (*__o1)
                            .chunk_limit
                            .offset_from((*__o1).chunk as *mut ::core::ffi::c_char)
                            as size_t
                    {
                        (*__o1).next_free = (*__o1).chunk_limit;
                    }
                    (*__o1).object_base = (*__o1).next_free;
                    __value
                }) as *const ::core::ffi::c_char,
            );
            let mut __o_5: *mut obstack = &raw mut stk;
            let mut __obj: *mut ::core::ffi::c_void =
                ::core::ptr::null_mut::<::core::ffi::c_void>();
            if __obj > (*__o_5).chunk as *mut ::core::ffi::c_void
                && __obj < (*__o_5).chunk_limit as *mut ::core::ffi::c_void
            {
                (*__o_5).object_base = __obj as *mut ::core::ffi::c_char;
                (*__o_5).next_free = (*__o_5).object_base;
            } else {
                _obstack_free(__o_5, __obj);
            }
        }
        _ => {
            s = text as *mut ::core::ffi::c_char;
        }
    }
    return s;
}
unsafe extern "C" fn expand_pax_option(
    mut targs: *mut tar_args,
    mut arg: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: ::core::ptr::null_mut::<_obstack_chunk>(),
        object_base: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        next_free: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        chunk_limit: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        temp: C2Rust_Unnamed_2 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2Rust_Unnamed_1 { plain: None },
        freefun: C2Rust_Unnamed_0 { plain: None },
        extra_arg: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut res: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    _obstack_begin(
        &raw mut stk,
        0 as size_t,
        0 as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut ::core::ffi::c_void),
        Some(free as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()),
    );
    while *arg != 0 {
        let mut seglen: size_t =
            strcspn(arg, b",\0".as_ptr() as *const ::core::ffi::c_char) as size_t;
        let mut p: *mut ::core::ffi::c_char = memchr(
            arg as *const ::core::ffi::c_void,
            '=' as ::core::ffi::c_int,
            seglen,
        ) as *const ::core::ffi::c_void
            as *mut ::core::ffi::c_char;
        if !p.is_null() {
            let mut len: size_t = (p.offset_from(arg) + 1isize) as size_t;
            let mut __o: *mut obstack = &raw mut stk;
            let mut __len: size_t = len;
            if ({
                let mut __o1: *const obstack = __o;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < __len
            {
                _obstack_newchunk(__o, __len);
            }
            memcpy(
                (*__o).next_free as *mut ::core::ffi::c_void,
                arg as *const ::core::ffi::c_void,
                __len,
            );
            (*__o).next_free = (*__o).next_free.offset(__len as isize);
            len = seglen.wrapping_sub(len);
            p = p.offset(1);
            while *p as ::core::ffi::c_int != 0
                && *(*__ctype_b_loc())
                    .offset(*p as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
            {
                len = len.wrapping_sub(1);
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == '{' as ::core::ffi::c_int
                && *p.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
                    == '}' as ::core::ffi::c_int
            {
                let mut ts: timespec = timespec {
                    tv_sec: 0,
                    tv_nsec: 0,
                };
                let mut tmp: *mut ::core::ffi::c_char = xmalloc(len) as *mut ::core::ffi::c_char;
                memcpy(
                    tmp as *mut ::core::ffi::c_void,
                    p.offset(1 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    len.wrapping_sub(2 as size_t),
                );
                *tmp.offset(len.wrapping_sub(2 as size_t) as isize) = 0 as ::core::ffi::c_char;
                if get_date_or_file(
                    targs,
                    b"--pax-option\0".as_ptr() as *const ::core::ffi::c_char,
                    tmp,
                    &raw mut ts,
                ) == 0 as ::core::ffi::c_int
                {
                    let mut buf: [::core::ffi::c_char; 32] = [0; 32];
                    let mut s: *const ::core::ffi::c_char =
                        code_timespec(ts, &raw mut buf as *mut ::core::ffi::c_char);
                    let mut __o_0: *mut obstack = &raw mut stk;
                    let mut __len_0: size_t = strlen(s);
                    if ({
                        let mut __o1: *const obstack = __o_0;
                        (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
                    }) < __len_0
                    {
                        _obstack_newchunk(__o_0, __len_0);
                    }
                    memcpy(
                        (*__o_0).next_free as *mut ::core::ffi::c_void,
                        s as *const ::core::ffi::c_void,
                        __len_0,
                    );
                    (*__o_0).next_free = (*__o_0).next_free.offset(__len_0 as isize);
                } else {
                    let mut __o_1: *mut obstack = &raw mut stk;
                    let mut __len_1: size_t = len;
                    if ({
                        let mut __o1: *const obstack = __o_1;
                        (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
                    }) < __len_1
                    {
                        _obstack_newchunk(__o_1, __len_1);
                    }
                    memcpy(
                        (*__o_1).next_free as *mut ::core::ffi::c_void,
                        p as *const ::core::ffi::c_void,
                        __len_1,
                    );
                    (*__o_1).next_free = (*__o_1).next_free.offset(__len_1 as isize);
                }
                free(tmp as *mut ::core::ffi::c_void);
            } else {
                let mut __o_2: *mut obstack = &raw mut stk;
                let mut __len_2: size_t = len;
                if ({
                    let mut __o1: *const obstack = __o_2;
                    (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
                }) < __len_2
                {
                    _obstack_newchunk(__o_2, __len_2);
                }
                memcpy(
                    (*__o_2).next_free as *mut ::core::ffi::c_void,
                    p as *const ::core::ffi::c_void,
                    __len_2,
                );
                (*__o_2).next_free = (*__o_2).next_free.offset(__len_2 as isize);
            }
        } else {
            let mut __o_3: *mut obstack = &raw mut stk;
            let mut __len_3: size_t = seglen;
            if ({
                let mut __o1: *const obstack = __o_3;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < __len_3
            {
                _obstack_newchunk(__o_3, __len_3);
            }
            memcpy(
                (*__o_3).next_free as *mut ::core::ffi::c_void,
                arg as *const ::core::ffi::c_void,
                __len_3,
            );
            (*__o_3).next_free = (*__o_3).next_free.offset(__len_3 as isize);
        }
        arg = arg.offset(seglen as isize);
        if *arg != 0 {
            let mut __o_4: *mut obstack = &raw mut stk;
            if ({
                let mut __o1: *const obstack = __o_4;
                (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
            }) < 1 as size_t
            {
                _obstack_newchunk(__o_4, 1 as size_t);
            }
            let c2rust_fresh5 = (*__o_4).next_free;
            (*__o_4).next_free = (*__o_4).next_free.offset(1);
            *c2rust_fresh5 = *arg;
            arg = arg.offset(1);
        }
    }
    let mut __o_5: *mut obstack = &raw mut stk;
    if ({
        let mut __o1: *const obstack = __o_5;
        (*__o1).chunk_limit.offset_from((*__o1).next_free) as size_t
    }) < 1 as size_t
    {
        _obstack_newchunk(__o_5, 1 as size_t);
    }
    let c2rust_fresh6 = (*__o_5).next_free;
    (*__o_5).next_free = (*__o_5).next_free.offset(1);
    *c2rust_fresh6 = 0 as ::core::ffi::c_char;
    res = xstrdup(
        ({
            let mut __o1: *mut obstack = &raw mut stk;
            let mut __value: *mut ::core::ffi::c_void =
                (*__o1).object_base as *mut ::core::ffi::c_void;
            if (*__o1).next_free == __value as *mut ::core::ffi::c_char {
                (*__o1).set_maybe_empty_object(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            (*__o1).next_free = if ::core::mem::size_of::<ptrdiff_t>()
                < ::core::mem::size_of::<*mut ::core::ffi::c_void>()
            {
                (*__o1).object_base
            } else {
                ::core::ptr::null_mut::<::core::ffi::c_char>()
            }
            .offset(
                (((*__o1).next_free.offset_from(
                    if ::core::mem::size_of::<ptrdiff_t>()
                        < ::core::mem::size_of::<*mut ::core::ffi::c_void>()
                    {
                        (*__o1).object_base
                    } else {
                        ::core::ptr::null_mut::<::core::ffi::c_char>()
                    },
                ) as size_t)
                    .wrapping_add((*__o1).alignment_mask)
                    & !(*__o1).alignment_mask) as isize,
            );
            if (*__o1)
                .next_free
                .offset_from((*__o1).chunk as *mut ::core::ffi::c_char) as size_t
                > (*__o1)
                    .chunk_limit
                    .offset_from((*__o1).chunk as *mut ::core::ffi::c_char)
                    as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *const ::core::ffi::c_char,
    );
    let mut __o_6: *mut obstack = &raw mut stk;
    let mut __obj: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if __obj > (*__o_6).chunk as *mut ::core::ffi::c_void
        && __obj < (*__o_6).chunk_limit as *mut ::core::ffi::c_void
    {
        (*__o_6).object_base = __obj as *mut ::core::ffi::c_char;
        (*__o_6).next_free = (*__o_6).object_base;
    } else {
        _obstack_free(__o_6, __obj);
    }
    return res;
}
unsafe extern "C" fn parse_owner_group(
    mut arg: *mut ::core::ffi::c_char,
    mut field_max: uintmax_t,
    mut name_option: *mut *const ::core::ffi::c_char,
) -> uintmax_t {
    let mut u: uintmax_t = UINTMAX_MAX as uintmax_t;
    let mut end: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut invalid_num: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut colon: *mut ::core::ffi::c_char = strchr(arg, ':' as ::core::ffi::c_int);
    if !colon.is_null() {
        let mut num: *const ::core::ffi::c_char = colon.offset(1 as ::core::ffi::c_int as isize);
        *colon = '\0' as ::core::ffi::c_char;
        if *arg != 0 {
            name = arg;
        }
        if !num.is_null()
            && !(xstrtoumax(
                num,
                &raw mut end,
                10 as ::core::ffi::c_int,
                &raw mut u,
                b"\0".as_ptr() as *const ::core::ffi::c_char,
            )
            .0 == strtol_error::LONGINT_OK.0
                && u <= field_max)
        {
            invalid_num = num;
        }
    } else {
        let mut u1: uintmax_t = 0;
        's_74: {
            match if '0' as ::core::ffi::c_int <= *arg as ::core::ffi::c_int
                && *arg as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            {
                xstrtoumax(
                    arg,
                    &raw mut end,
                    10 as ::core::ffi::c_int,
                    &raw mut u1,
                    b"\0".as_ptr() as *const ::core::ffi::c_char,
                )
                .0
            } else {
                strtol_error::LONGINT_INVALID.0
            } {
                0 => {
                    if u1 <= field_max {
                        u = u1;
                        break 's_74;
                    }
                }
                1 => {}
                _ => {
                    name = arg;
                    break 's_74;
                }
            }
            invalid_num = arg;
        }
    }
    if !invalid_num.is_null() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                quotearg_colon(invalid_num),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Invalid owner or group ID\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    quotearg_colon(invalid_num),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid owner or group ID\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        fatal_exit();
    }
    if !name.is_null() {
        *name_option = name;
    }
    return u;
}
pub const TAR_SIZE_SUFFIXES: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"bBcGgkKMmPTtw\0") };
static mut sort_mode_arg: [*const ::core::ffi::c_char; 4] = [
    b"none\0".as_ptr() as *const ::core::ffi::c_char,
    b"name\0".as_ptr() as *const ::core::ffi::c_char,
    b"inode\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut sort_mode_flag: [::core::ffi::c_int; 3] = [
    savedir_option::SAVEDIR_SORT_NONE.0 as ::core::ffi::c_int,
    savedir_option::SAVEDIR_SORT_NAME.0 as ::core::ffi::c_int,
    savedir_option::SAVEDIR_SORT_INODE.0 as ::core::ffi::c_int,
];
static mut hole_detection_args: [*const ::core::ffi::c_char; 3] = [
    b"raw\0".as_ptr() as *const ::core::ffi::c_char,
    b"seek\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
static mut hole_detection_types: [::core::ffi::c_int; 2] = [
    hole_detection_method::HOLE_DETECTION_RAW.0 as ::core::ffi::c_int,
    hole_detection_method::HOLE_DETECTION_SEEK.0 as ::core::ffi::c_int,
];
unsafe extern "C" fn set_old_files_option(
    mut code: ::core::ffi::c_int,
    mut loc: *mut option_locus,
) {
    let mut prev: *mut option_locus = ::core::ptr::null_mut::<option_locus>();
    static mut code_to_opt: [*const ::core::ffi::c_char; 7] = [
        b"--overwrite-dir\0".as_ptr() as *const ::core::ffi::c_char,
        b"--no-overwrite-dir\0".as_ptr() as *const ::core::ffi::c_char,
        b"--overwrite\0".as_ptr() as *const ::core::ffi::c_char,
        b"--unlink-first\0".as_ptr() as *const ::core::ffi::c_char,
        b"--keep-old-files\0".as_ptr() as *const ::core::ffi::c_char,
        b"--skip-old-files\0".as_ptr() as *const ::core::ffi::c_char,
        b"--keep-newer-files\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    prev = optloc_save(option_class_0::OC_OLD_FILES.0, loc);
    if !prev.is_null()
        && optloc_eq(loc, prev) != 0
        && code as ::core::ffi::c_uint != old_files_option.0
    {
        option_conflict_error(
            code_to_opt[code as usize],
            code_to_opt[old_files_option.0 as usize],
        );
    }
    old_files_option = old_files(code as ::core::ffi::c_uint);
}
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    let mut args: *mut tar_args = (*state).input as *mut tar_args;
    's_1650: {
        'c_10471: {
            match key {
                ARGP_KEY_INIT => {
                    if !(*(*state).root_argp).children.is_null() {
                        let mut i: ::core::ffi::c_int = 0;
                        i = 0 as ::core::ffi::c_int;
                        while !(*(*(*state).root_argp).children.offset(i as isize))
                            .argp
                            .is_null()
                        {
                            *(*state).child_inputs.offset(i as isize) = (*state).input;
                            i += 1;
                        }
                    }
                    break 's_1650;
                }
                ARGP_KEY_ARG => {
                    name_add_name(arg);
                    break 's_1650;
                }
                65 => {
                    set_subcommand_option(subcommand::CAT_SUBCOMMAND);
                    break 's_1650;
                }
                97 => {
                    (*args).compress_autodetect = r#true != 0;
                    break 's_1650;
                }
                155 => {
                    (*args).compress_autodetect = r#false != 0;
                    break 's_1650;
                }
                98 => {
                    let mut u: uintmax_t = 0;
                    if !(xstrtoumax(
                        arg,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                        &raw mut u,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .0 == strtol_error::LONGINT_OK.0
                        && {
                            blocking_factor = u as ::core::ffi::c_int;
                            u == blocking_factor as uintmax_t
                        }
                        && (0 as ::core::ffi::c_int) < blocking_factor
                        && {
                            record_size = u.wrapping_mul(BLOCKSIZE as uintmax_t) as size_t;
                            u == (record_size as uintmax_t).wrapping_div(BLOCKSIZE as uintmax_t)
                        })
                    {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                quotearg_colon(arg),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Invalid blocking factor\0".as_ptr()
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
                                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    quotearg_colon(arg),
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid blocking factor\0".as_ptr()
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
                        usage(PAXEXIT_FAILURE);
                    }
                    break 's_1650;
                }
                66 => {
                    read_full_records_option = r#true != 0;
                    break 's_1650;
                }
                99 => {
                    set_subcommand_option(subcommand::CREATE_SUBCOMMAND);
                    break 's_1650;
                }
                134 => {
                    set_mtime_option = set_mtime_option_mode::CLAMP_MTIME;
                    break 's_1650;
                }
                100 => {
                    set_subcommand_option(subcommand::DIFF_SUBCOMMAND);
                    break 's_1650;
                }
                70 => {
                    info_script_option = arg;
                    multi_volume_option = r#true != 0;
                    break 's_1650;
                }
                102 => {
                    if archive_names == allocated_archive_names {
                        archive_name_array = x2nrealloc(
                            archive_name_array as *mut ::core::ffi::c_void,
                            &raw mut allocated_archive_names,
                            ::core::mem::size_of::<*const ::core::ffi::c_char>(),
                        )
                            as *mut *const ::core::ffi::c_char;
                    }
                    let c2rust_fresh4 = archive_names;
                    archive_names = archive_names.wrapping_add(1);
                    *archive_name_array.offset(c2rust_fresh4 as isize) = arg;
                    break 's_1650;
                }
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    argp_error(
                        state,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Options '-[0-7][lmh]' not supported by *this* tar\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                    );
                    exit(EX_USAGE);
                }
                139 => {
                    full_time_option = r#true != 0;
                    break 's_1650;
                }
                103 => {
                    optloc_save(option_class_0::OC_LISTED_INCREMENTAL.0, (*args).loc);
                    listed_incremental_option = arg;
                    after_date_option = r#true;
                    break 'c_10471;
                }
                71 => {
                    break 'c_10471;
                }
                104 => {
                    dereference_option = r#true != 0;
                    break 's_1650;
                }
                136 => {
                    hard_dereference_option = r#true != 0;
                    break 's_1650;
                }
                105 => {
                    ignore_zeros_option = r#true != 0;
                    break 's_1650;
                }
                106 => {
                    set_use_compress_program_option(BZIP2_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                74 => {
                    set_use_compress_program_option(XZ_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                107 => {
                    set_old_files_option(
                        old_files::KEEP_OLD_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                75 => {
                    optloc_save(option_class_0::OC_STARTING_FILE.0, (*args).loc);
                    add_starting_file(arg);
                    break 's_1650;
                }
                169 => {
                    one_file_system_option = r#true != 0;
                    break 's_1650;
                }
                170 => {
                    optloc_save(option_class_0::OC_ONE_TOP_LEVEL.0, (*args).loc);
                    one_top_level_option = r#true != 0;
                    one_top_level_dir = arg;
                    break 's_1650;
                }
                108 => {
                    check_links_option = 1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                76 => {
                    let mut u_0: uintmax_t = 0;
                    let mut p: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    if xstrtoumax(
                        arg,
                        &raw mut p,
                        10 as ::core::ffi::c_int,
                        &raw mut u_0,
                        TAR_SIZE_SUFFIXES.as_ptr(),
                    )
                    .0 != strtol_error::LONGINT_OK.0
                    {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                quotearg_colon(arg),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Invalid tape length\0".as_ptr() as *const ::core::ffi::c_char,
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
                                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    quotearg_colon(arg),
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid tape length\0".as_ptr()
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
                        usage(PAXEXIT_FAILURE);
                    }
                    if p > arg
                        && (strchr(
                            b"bBcGgkKMmPTtw\0".as_ptr() as *const ::core::ffi::c_char,
                            *p.offset(-1isize) as ::core::ffi::c_int,
                        ) as *const ::core::ffi::c_char)
                            .is_null()
                    {
                        tape_length_option = 1024 as ::core::ffi::c_int as tarlong * u_0 as tarlong;
                    } else {
                        tape_length_option = u_0 as tarlong;
                    }
                    multi_volume_option = r#true != 0;
                    break 's_1650;
                }
                147 => {
                    let mut p_0: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    incremental_level =
                        strtoul(arg, &raw mut p_0, 10 as ::core::ffi::c_int) as ::core::ffi::c_int;
                    if *p_0 != 0 {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Invalid incremental level value\0".as_ptr()
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
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid incremental level value\0".as_ptr()
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
                        usage(PAXEXIT_FAILURE);
                    }
                    break 's_1650;
                }
                148 => {
                    set_use_compress_program_option(LZIP_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                149 => {
                    set_use_compress_program_option(LZMA_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                150 => {
                    set_use_compress_program_option(LZOP_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                109 => {
                    touch_option = r#true != 0;
                    break 's_1650;
                }
                77 => {
                    multi_volume_option = r#true != 0;
                    break 's_1650;
                }
                152 => {
                    get_date_or_file(
                        args,
                        b"--mtime\0".as_ptr() as *const ::core::ffi::c_char,
                        arg,
                        &raw mut mtime_option,
                    );
                    if set_mtime_option.0 == set_mtime_option_mode::USE_FILE_MTIME.0 {
                        set_mtime_option = set_mtime_option_mode::FORCE_MTIME;
                    }
                    break 's_1650;
                }
                110 => {
                    seek_option = 1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                163 => {
                    seek_option = 0 as ::core::ffi::c_int;
                    break 's_1650;
                }
                78 => {
                    after_date_option = r#true;
                }
                153 => {}
                111 => {
                    (*args).o_option = r#true != 0;
                    break 's_1650;
                }
                79 => {
                    to_stdout_option = r#true != 0;
                    break 's_1650;
                }
                112 => {
                    same_permissions_option = r#true;
                    break 's_1650;
                }
                80 => {
                    optloc_save(option_class_0::OC_ABSOLUTE_NAMES.0, (*args).loc);
                    absolute_names_option = r#true != 0;
                    break 's_1650;
                }
                114 => {
                    set_subcommand_option(subcommand::APPEND_SUBCOMMAND);
                    break 's_1650;
                }
                82 => {
                    block_number_option = r#true != 0;
                    break 's_1650;
                }
                115 => {
                    optloc_save(option_class_0::OC_SAME_ORDER.0, (*args).loc);
                    same_order_option = r#true != 0;
                    break 's_1650;
                }
                83 => {
                    sparse_option = r#true != 0;
                    break 's_1650;
                }
                191 => {
                    set_old_files_option(
                        old_files::SKIP_OLD_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                193 => {
                    hole_detection = hole_detection_method(
                        hole_detection_types[__xargmatch_internal(
                            b"--hole-detection\0".as_ptr() as *const ::core::ffi::c_char,
                            arg,
                            &raw const hole_detection_args as *const *const ::core::ffi::c_char,
                            &raw const hole_detection_types as *const ::core::ffi::c_int
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<::core::ffi::c_int>(),
                            argmatch_die,
                            r#true != 0,
                        ) as usize] as ::core::ffi::c_uint,
                    );
                    sparse_option = r#true != 0;
                    break 's_1650;
                }
                194 => {
                    sparse_option = r#true != 0;
                    let mut p_1: *mut ::core::ffi::c_char =
                        ::core::ptr::null_mut::<::core::ffi::c_char>();
                    tar_sparse_major =
                        strtoul(arg, &raw mut p_1, 10 as ::core::ffi::c_int) as ::core::ffi::c_uint;
                    if *p_1 != 0 {
                        if *p_1 as ::core::ffi::c_int != '.' as ::core::ffi::c_int {
                            if error_hook.is_some() {
                                error_hook.expect("non-null function pointer")();
                            }
                            if 0 != 0 {
                                error(
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid sparse version value\0".as_ptr()
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
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"Invalid sparse version value\0".as_ptr()
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
                            usage(PAXEXIT_FAILURE);
                        }
                        tar_sparse_minor = strtoul(
                            p_1.offset(1 as ::core::ffi::c_int as isize),
                            &raw mut p_1,
                            10 as ::core::ffi::c_int,
                        ) as ::core::ffi::c_uint;
                        if *p_1 != 0 {
                            if error_hook.is_some() {
                                error_hook.expect("non-null function pointer")();
                            }
                            if 0 != 0 {
                                error(
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid sparse version value\0".as_ptr()
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
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"Invalid sparse version value\0".as_ptr()
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
                            usage(PAXEXIT_FAILURE);
                        }
                    }
                    break 's_1650;
                }
                116 => {
                    set_subcommand_option(subcommand::LIST_SUBCOMMAND);
                    verbose_option += 1;
                    break 's_1650;
                }
                197 => {
                    set_subcommand_option(subcommand::TEST_LABEL_SUBCOMMAND);
                    break 's_1650;
                }
                200 => {
                    set_transform_expr(arg);
                    break 's_1650;
                }
                117 => {
                    set_subcommand_option(subcommand::UPDATE_SUBCOMMAND);
                    break 's_1650;
                }
                85 => {
                    set_old_files_option(
                        old_files::UNLINK_FIRST_OLD_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                201 => {
                    utc_option = r#true != 0;
                    break 's_1650;
                }
                86 => {
                    volume_label_option = arg;
                    break 's_1650;
                }
                118 => {
                    verbose_option += 1;
                    warning_option |= WARN_VERBOSE_WARNINGS;
                    break 's_1650;
                }
                119 => {
                    interactive_option = r#true != 0;
                    break 's_1650;
                }
                87 => {
                    optloc_save(option_class_0::OC_VERIFY.0, (*args).loc);
                    verify_option = r#true != 0;
                    break 's_1650;
                }
                203 => {
                    set_warning_option(arg);
                    break 's_1650;
                }
                120 => {
                    set_subcommand_option(subcommand::EXTRACT_SUBCOMMAND);
                    break 's_1650;
                }
                122 => {
                    set_use_compress_program_option(GZIP_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                90 => {
                    set_use_compress_program_option(COMPRESS_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                207 => {
                    set_use_compress_program_option(ZSTD_PROGRAM.as_ptr(), (*args).loc);
                    break 's_1650;
                }
                129 => {
                    atime_preserve_option = atime_preserve(if !arg.is_null() {
                        atime_preserve_types[__xargmatch_internal(
                            b"--atime-preserve\0".as_ptr() as *const ::core::ffi::c_char,
                            arg,
                            &raw const atime_preserve_args as *const *const ::core::ffi::c_char,
                            &raw const atime_preserve_types as *const atime_preserve
                                as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<atime_preserve>(),
                            argmatch_die,
                            r#true != 0,
                        ) as usize]
                            .0
                    } else {
                        atime_preserve::replace_atime_preserve.0
                    });
                    if O_NOATIME == 0
                        && atime_preserve_option.0 == atime_preserve::system_atime_preserve.0
                    {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"--atime-preserve='system' is not supported on this platform\0"
                                        .as_ptr()
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
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"--atime-preserve='system' is not supported on this platform\0"
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
                        fatal_exit();
                    }
                    break 's_1650;
                }
                131 => {
                    check_device_option = r#true != 0;
                    break 's_1650;
                }
                156 => {
                    check_device_option = r#false != 0;
                    break 's_1650;
                }
                132 => {
                    if !arg.is_null() {
                        let mut p_2: *mut ::core::ffi::c_char =
                            ::core::ptr::null_mut::<::core::ffi::c_char>();
                        if *arg as ::core::ffi::c_int == '.' as ::core::ffi::c_int {
                            checkpoint_compile_action(b".\0".as_ptr() as *const ::core::ffi::c_char);
                            arg = arg.offset(1);
                        }
                        checkpoint_option = strtoul(arg, &raw mut p_2, 0 as ::core::ffi::c_int)
                            as ::core::ffi::c_uint;
                        if *p_2 != 0 {
                            if error_hook.is_some() {
                                error_hook.expect("non-null function pointer")();
                            }
                            if 0 != 0 {
                                error(
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"--checkpoint value is not an integer\0".as_ptr()
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
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"--checkpoint value is not an integer\0".as_ptr()
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
                            fatal_exit();
                        }
                    } else {
                        checkpoint_option = DEFAULT_CHECKPOINT as ::core::ffi::c_uint;
                    }
                    break 's_1650;
                }
                133 => {
                    checkpoint_compile_action(arg);
                    break 's_1650;
                }
                130 => {
                    backup_option = r#true != 0;
                    if !arg.is_null() {
                        (*args).version_control_string = arg;
                    }
                    break 's_1650;
                }
                135 => {
                    delay_directory_restore_option = r#true != 0;
                    break 's_1650;
                }
                157 => {
                    delay_directory_restore_option = r#false != 0;
                    break 's_1650;
                }
                137 => {
                    set_subcommand_option(subcommand::DELETE_SUBCOMMAND);
                    break 's_1650;
                }
                138 => {
                    force_local_option = r#true != 0;
                    break 's_1650;
                }
                72 => {
                    set_archive_format(arg);
                    break 's_1650;
                }
                144 => {
                    index_file_name = arg;
                    break 's_1650;
                }
                142 => {
                    ignore_command_error_option = r#true != 0;
                    break 's_1650;
                }
                143 => {
                    ignore_failed_read_option = r#true != 0;
                    break 's_1650;
                }
                145 => {
                    keep_directory_symlink_option = r#true != 0;
                    break 's_1650;
                }
                146 => {
                    set_old_files_option(
                        old_files::KEEP_NEWER_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                140 => {
                    let mut u_1: uintmax_t = parse_owner_group(
                        arg,
                        (if (0 as ::core::ffi::c_int as gid_t) < -1 as ::core::ffi::c_int as gid_t {
                            -1 as ::core::ffi::c_int as gid_t
                        } else {
                            ((1 as ::core::ffi::c_int as gid_t)
                                << ::core::mem::size_of::<gid_t>()
                                    .wrapping_mul(CHAR_BIT as usize)
                                    .wrapping_sub(2usize))
                            .wrapping_sub(1 as gid_t)
                            .wrapping_mul(2 as gid_t)
                            .wrapping_add(1 as gid_t)
                        }) as uintmax_t,
                        &raw mut group_name_option,
                    );
                    if u_1 == UINTMAX_MAX as uintmax_t {
                        group_option = -1 as ::core::ffi::c_int as gid_t;
                        if !group_name_option.is_null() {
                            gname_to_gid(group_name_option, &raw mut group_option);
                        }
                    } else {
                        group_option = u_1 as gid_t;
                    }
                    break 's_1650;
                }
                141 => {
                    group_map_read(arg);
                    break 's_1650;
                }
                151 => {
                    mode_option = mode_compile(arg) as *mut mode_change;
                    if mode_option.is_null() {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Invalid mode given on option\0".as_ptr()
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
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid mode given on option\0".as_ptr()
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
                        fatal_exit();
                    }
                    initial_umask = umask(0 as __mode_t) as mode_t;
                    umask(initial_umask);
                    break 's_1650;
                }
                158 => {
                    ignore_command_error_option = r#false != 0;
                    break 's_1650;
                }
                159 => {
                    set_old_files_option(
                        old_files::NO_OVERWRITE_DIR_OLD_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                160 => {
                    while *arg != 0 {
                        set_char_quoting(
                            ::core::ptr::null_mut::<quoting_options>(),
                            *arg,
                            0 as ::core::ffi::c_int,
                        );
                        arg = arg.offset(1);
                    }
                    break 's_1650;
                }
                166 => {
                    numeric_owner_option = r#true != 0;
                    break 's_1650;
                }
                167 => {
                    optloc_save(option_class_0::OC_OCCURRENCE.0, (*args).loc);
                    if arg.is_null() {
                        occurrence_option = 1 as uintmax_t;
                    } else {
                        let mut u_2: uintmax_t = 0;
                        if xstrtoumax(
                            arg,
                            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                            10 as ::core::ffi::c_int,
                            &raw mut u_2,
                            b"\0".as_ptr() as *const ::core::ffi::c_char,
                        )
                        .0 == strtol_error::LONGINT_OK.0
                        {
                            occurrence_option = u_2;
                        } else {
                            if error_hook.is_some() {
                                error_hook.expect("non-null function pointer")();
                            }
                            if 0 != 0 {
                                error(
                                    0 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    quotearg_colon(arg),
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid number\0".as_ptr() as *const ::core::ffi::c_char,
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
                                        b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                        quotearg_colon(arg),
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"Invalid number\0".as_ptr()
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
                            fatal_exit();
                        }
                    }
                    break 's_1650;
                }
                168 => {
                    set_archive_format(b"v7\0".as_ptr() as *const ::core::ffi::c_char);
                    break 's_1650;
                }
                171 => {
                    set_old_files_option(
                        old_files::DEFAULT_OLD_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                172 => {
                    set_old_files_option(
                        old_files::OVERWRITE_OLD_FILES.0 as ::core::ffi::c_int,
                        (*args).loc,
                    );
                    break 's_1650;
                }
                173 => {
                    let mut u_3: uintmax_t = parse_owner_group(
                        arg,
                        (if (0 as ::core::ffi::c_int as uid_t) < -1 as ::core::ffi::c_int as uid_t {
                            -1 as ::core::ffi::c_int as uid_t
                        } else {
                            ((1 as ::core::ffi::c_int as uid_t)
                                << ::core::mem::size_of::<uid_t>()
                                    .wrapping_mul(CHAR_BIT as usize)
                                    .wrapping_sub(2usize))
                            .wrapping_sub(1 as uid_t)
                            .wrapping_mul(2 as uid_t)
                            .wrapping_add(1 as uid_t)
                        }) as uintmax_t,
                        &raw mut owner_name_option,
                    );
                    if u_3 == UINTMAX_MAX as uintmax_t {
                        owner_option = -1 as ::core::ffi::c_int as uid_t;
                        if !owner_name_option.is_null() {
                            uname_to_uid(owner_name_option, &raw mut owner_option);
                        }
                    } else {
                        owner_option = u_3 as uid_t;
                    }
                    break 's_1650;
                }
                174 => {
                    owner_map_read(arg);
                    break 's_1650;
                }
                177 => {
                    while *arg != 0 {
                        set_char_quoting(
                            ::core::ptr::null_mut::<quoting_options>(),
                            *arg,
                            1 as ::core::ffi::c_int,
                        );
                        arg = arg.offset(1);
                    }
                    break 's_1650;
                }
                178 => {
                    tar_set_quoting_style(arg);
                    break 's_1650;
                }
                175 => {
                    let mut tmp: *mut ::core::ffi::c_char = expand_pax_option(args, arg);
                    (*args).pax_option = r#true != 0;
                    xheader_set_option(tmp);
                    free(tmp as *mut ::core::ffi::c_void);
                    break 's_1650;
                }
                176 => {
                    set_archive_format(b"posix\0".as_ptr() as *const ::core::ffi::c_char);
                    break 's_1650;
                }
                179 => {
                    let mut u_4: uintmax_t = 0;
                    if !(xstrtoumax(
                        arg,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                        &raw mut u_4,
                        TAR_SIZE_SUFFIXES.as_ptr(),
                    )
                    .0 == strtol_error::LONGINT_OK.0
                        && u_4 == u_4)
                    {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                quotearg_colon(arg),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Invalid record size\0".as_ptr() as *const ::core::ffi::c_char,
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
                                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    quotearg_colon(arg),
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid record size\0".as_ptr()
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
                        usage(PAXEXIT_FAILURE);
                    }
                    record_size = u_4 as size_t;
                    if record_size.wrapping_rem(BLOCKSIZE as size_t) != 0 as size_t {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Record size must be a multiple of %d.\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                512 as ::core::ffi::c_int,
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
                                        b"Record size must be a multiple of %d.\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                    512 as ::core::ffi::c_int,
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                        usage(PAXEXIT_FAILURE);
                    }
                    blocking_factor =
                        record_size.wrapping_div(BLOCKSIZE as size_t) as ::core::ffi::c_int;
                    break 's_1650;
                }
                180 => {
                    recursive_unlink_option = r#true != 0;
                    break 's_1650;
                }
                181 => {
                    remove_files_option = r#true != 0;
                    break 's_1650;
                }
                182 => {
                    restrict_option = r#true != 0;
                    break 's_1650;
                }
                183 => {
                    rmt_command = arg;
                    break 's_1650;
                }
                184 => {
                    rsh_command_option = arg;
                    break 's_1650;
                }
                187 => {
                    let mut s: *mut ::core::ffi::c_char = format_default_settings();
                    printf(b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, s);
                    close_stdout();
                    free(s as *mut ::core::ffi::c_void);
                    exit(0 as ::core::ffi::c_int);
                }
                189 => {
                    show_snapshot_field_ranges();
                    close_stdout();
                    exit(0 as ::core::ffi::c_int);
                }
                195 => {
                    let mut u_5: uintmax_t = 0;
                    if !(xstrtoumax(
                        arg,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                        &raw mut u_5,
                        b"\0".as_ptr() as *const ::core::ffi::c_char,
                    )
                    .0 == strtol_error::LONGINT_OK.0
                        && u_5 == u_5)
                    {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                quotearg_colon(arg),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Invalid number of elements\0".as_ptr()
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
                                    b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    quotearg_colon(arg),
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Invalid number of elements\0".as_ptr()
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
                        usage(PAXEXIT_FAILURE);
                    }
                    strip_name_components = u_5 as size_t;
                    break 's_1650;
                }
                188 => {
                    show_omitted_dirs_option = r#true != 0;
                    break 's_1650;
                }
                190 => {
                    show_transformed_names_option = r#true != 0;
                    break 's_1650;
                }
                192 => {
                    savedir_sort_order = sort_mode_flag[__xargmatch_internal(
                        b"--sort\0".as_ptr() as *const ::core::ffi::c_char,
                        arg,
                        &raw const sort_mode_arg as *const *const ::core::ffi::c_char,
                        &raw mut sort_mode_flag as *mut ::core::ffi::c_int
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<::core::ffi::c_int>(),
                        argmatch_die,
                        r#true != 0,
                    ) as usize];
                    break 's_1650;
                }
                196 => {
                    backup_option = r#true != 0;
                    (*args).backup_suffix_string = arg;
                    break 's_1650;
                }
                199 => {
                    if !to_command_option.is_null() {
                        if error_hook.is_some() {
                            error_hook.expect("non-null function pointer")();
                        }
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Only one --to-command option allowed\0".as_ptr()
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
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"Only one --to-command option allowed\0".as_ptr()
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
                        usage(PAXEXIT_FAILURE);
                    }
                    to_command_option = arg;
                    break 's_1650;
                }
                198 => {
                    if !arg.is_null() {
                        set_stat_signal(arg);
                    } else {
                        totals_option = r#true != 0;
                    }
                    break 's_1650;
                }
                73 => {
                    set_use_compress_program_option(arg, (*args).loc);
                    break 's_1650;
                }
                202 => {
                    volno_file_option = arg;
                    break 's_1650;
                }
                161 => {
                    same_owner_option = -1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                162 => {
                    same_permissions_option = -1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                128 => {
                    set_archive_format(b"posix\0".as_ptr() as *const ::core::ffi::c_char);
                    acls_option = 1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                154 => {
                    acls_option = -1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                186 => {
                    set_archive_format(b"posix\0".as_ptr() as *const ::core::ffi::c_char);
                    selinux_context_option = 1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                164 => {
                    selinux_context_option = -1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                204 => {
                    set_xattr_option(1 as ::core::ffi::c_int);
                    break 's_1650;
                }
                165 => {
                    set_xattr_option(-1 as ::core::ffi::c_int);
                    break 's_1650;
                }
                206 | 205 => {
                    set_xattr_option(1 as ::core::ffi::c_int);
                    xattrs_mask_add(
                        arg,
                        key == C2Rust_Unnamed_3::XATTR_INCLUDE.0 as ::core::ffi::c_int,
                    );
                    break 's_1650;
                }
                185 => {
                    same_owner_option = 1 as ::core::ffi::c_int;
                    break 's_1650;
                }
                ARGP_KEY_ERROR => {
                    if (*(*args).loc).source.0 == option_source::OPTS_FILE.0 {
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"%s:%lu: location of the error\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                (*(*args).loc).name,
                                (*(*args).loc).line as ::core::ffi::c_ulong,
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
                                        b"%s:%lu: location of the error\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                    (*(*args).loc).name,
                                    (*(*args).loc).line as ::core::ffi::c_ulong,
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                    } else if (*(*args).loc).source.0 == option_source::OPTS_ENVIRON.0 {
                        if 0 != 0 {
                            error(
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"error parsing %s\0".as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                (*(*args).loc).name,
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
                                        b"error parsing %s\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                    (*(*args).loc).name,
                                );
                                if __errstatus != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            });
                        };
                    }
                    exit(EX_USAGE);
                }
                _ => return ARGP_ERR_UNKNOWN,
            }
            if 0 as __syscall_slong_t <= newer_mtime_option.tv_nsec {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"More than one threshold date\0".as_ptr()
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
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"More than one threshold date\0".as_ptr()
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
                usage(PAXEXIT_FAILURE);
            }
            get_date_or_file(
                args,
                if key == C2Rust_Unnamed_3::NEWER_MTIME_OPTION.0 as ::core::ffi::c_int {
                    b"--newer-mtime\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"--after-date\0".as_ptr() as *const ::core::ffi::c_char
                },
                arg,
                &raw mut newer_mtime_option,
            );
            optloc_save(option_class_0::OC_NEWER.0, (*args).loc);
            break 's_1650;
        }
        incremental_option = r#true != 0;
    }
    return 0 as error_t;
}
static mut argp_children: [argp_child; 2] = unsafe {
    [
        argp_child {
            argp: &raw const names_argp as *mut argp,
            flags: 0 as ::core::ffi::c_int,
            header: ::core::ptr::null::<::core::ffi::c_char>(),
            group: C2Rust_Unnamed_4::GRID_FILE_NAME.0 as ::core::ffi::c_int,
        },
        argp_child {
            argp: ::core::ptr::null::<argp>(),
            flags: 0,
            header: ::core::ptr::null::<::core::ffi::c_char>(),
            group: 0,
        },
    ]
};
static mut argp: argp = unsafe {
    argp {
        options: &raw const options as *mut argp_option,
        parser: Some(
            parse_opt
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_char,
                    *mut argp_state,
                ) -> error_t,
        ),
        args_doc: b"[FILE]...\0".as_ptr() as *const ::core::ffi::c_char,
        doc: &raw const doc as *const ::core::ffi::c_char,
        children: &raw const argp_children as *mut argp_child,
        help_filter: Some(
            tar_help_filter
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_char,
                    *mut ::core::ffi::c_void,
                ) -> *mut ::core::ffi::c_char,
        ),
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
#[export_name = "rboxc_tar_usage"]
pub unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) {
    argp_help(
        &raw mut argp,
        stderr,
        ARGP_HELP_SEE as ::core::ffi::c_uint,
        program_name as *mut ::core::ffi::c_char,
    );
    close_stdout();
    exit(status);
}
unsafe extern "C" fn find_argp_option_key(
    mut o: *const argp_option,
    mut key: ::core::ffi::c_int,
) -> *const argp_option {
    while !((*o).name.is_null()
        && (*o).key == 0 as ::core::ffi::c_int
        && (*o).arg.is_null()
        && (*o).flags == 0 as ::core::ffi::c_int
        && (*o).doc.is_null())
    {
        if (*o).key == key {
            return o;
        }
        o = o.offset(1);
    }
    return ::core::ptr::null::<argp_option>();
}
unsafe extern "C" fn find_argp_option(
    mut ap: *mut argp,
    mut key: ::core::ffi::c_int,
) -> *const argp_option {
    let mut p: *const argp_option = ::core::ptr::null::<argp_option>();
    let mut child: *const argp_child = ::core::ptr::null::<argp_child>();
    p = find_argp_option_key((*ap).options, key);
    if p.is_null() && !(*ap).children.is_null() {
        child = (*ap).children;
        while !(*child).argp.is_null() {
            p = find_argp_option_key((*(*child).argp).options, key);
            if !p.is_null() {
                break;
            }
            child = child.offset(1);
        }
    }
    return p;
}
static mut tar_authors: [*const ::core::ffi::c_char; 3] = [
    b"John Gilmore\0".as_ptr() as *const ::core::ffi::c_char,
    b"Jay Fenlason\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
pub const SUBCL_READ: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SUBCL_WRITE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SUBCL_UPDATE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const SUBCL_TEST: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const SUBCL_OCCUR: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
static mut subcommand_class: [::core::ffi::c_int; 10] = [
    0 as ::core::ffi::c_int,
    SUBCL_WRITE | SUBCL_UPDATE,
    SUBCL_WRITE,
    SUBCL_WRITE,
    SUBCL_WRITE | SUBCL_UPDATE | SUBCL_OCCUR,
    SUBCL_READ | SUBCL_OCCUR,
    SUBCL_READ | SUBCL_OCCUR,
    SUBCL_READ | SUBCL_OCCUR,
    SUBCL_WRITE | SUBCL_UPDATE,
    SUBCL_TEST,
];
#[export_name = "rboxc_tar_more_options"]
pub unsafe extern "C" fn more_options(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut loc: *mut option_locus,
) {
    let mut args: tar_args = tar_args {
        loc: loc,
        textual_date: ::core::ptr::null_mut::<textual_date>(),
        o_option: r#false != 0,
        pax_option: r#false != 0,
        compress_autodetect: r#false != 0,
        backup_suffix_string: ::core::ptr::null::<::core::ffi::c_char>(),
        version_control_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    argp_parse(
        &raw mut names_argp,
        argc,
        argv,
        (ARGP_IN_ORDER | ARGP_NO_EXIT | ARGP_NO_ERRS) as ::core::ffi::c_uint,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut args as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn parse_default_options(mut args: *mut tar_args) {
    let mut opts: *mut ::core::ffi::c_char =
        getenv(b"TAR_OPTIONS\0".as_ptr() as *const ::core::ffi::c_char);

    let mut loc: option_locus = option_locus {
        source: option_source::OPTS_ENVIRON,
        name: b"TAR_OPTIONS\0".as_ptr() as *const ::core::ffi::c_char,
        line: 0 as size_t,
        prev: ::core::ptr::null_mut::<option_locus>(),
    };
    let mut save_loc_ptr: *mut option_locus = ::core::ptr::null_mut::<option_locus>();
    if opts.is_null() {
        return;
    }
    RBOXC_DEFAULT_WORDS.ws_offs = 1 as size_t;
    if wordsplit(
        opts,
        &raw mut RBOXC_DEFAULT_WORDS,
        (WRDSF_DEFFLAGS | WRDSF_DOOFFS) as ::core::ffi::c_uint,
    ) != 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"cannot split TAR_OPTIONS: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                wordsplit_strerror(&raw mut RBOXC_DEFAULT_WORDS),
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
                        b"cannot split TAR_OPTIONS: %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    wordsplit_strerror(&raw mut RBOXC_DEFAULT_WORDS),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        fatal_exit();
    }
    if RBOXC_DEFAULT_WORDS.ws_wordc != 0 {
        let mut idx: ::core::ffi::c_int = 0;
        *RBOXC_DEFAULT_WORDS.ws_wordv.offset(0isize) = program_name as *mut ::core::ffi::c_char;
        save_loc_ptr = (*args).loc;
        (*args).loc = &raw mut loc;
        if argp_parse(
            &raw mut argp,
            RBOXC_DEFAULT_WORDS.ws_offs.wrapping_add(RBOXC_DEFAULT_WORDS.ws_wordc) as ::core::ffi::c_int,
            RBOXC_DEFAULT_WORDS.ws_wordv,
            (ARGP_IN_ORDER | ARGP_NO_EXIT) as ::core::ffi::c_uint,
            &raw mut idx,
            args as *mut ::core::ffi::c_void,
        ) != 0
        {
            abort();
        }
        (*args).loc = save_loc_ptr;
        if name_more_files() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"non-option arguments in %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    loc.name,
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
                            b"non-option arguments in %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        loc.name,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }

    }

}
unsafe extern "C" fn decode_options(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut idx: ::core::ffi::c_int = 0;
    let mut loc: option_locus = option_locus {
        source: option_source::OPTS_COMMAND_LINE,
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        line: 0 as size_t,
        prev: ::core::ptr::null_mut::<option_locus>(),
    };
    let mut args: tar_args = tar_args {
        loc: &raw mut loc,
        textual_date: ::core::ptr::null_mut::<textual_date>(),
        o_option: r#false != 0,
        pax_option: r#false != 0,
        compress_autodetect: r#false != 0,
        backup_suffix_string: ::core::ptr::null::<::core::ffi::c_char>(),
        version_control_string: ::core::ptr::null::<::core::ffi::c_char>(),
    };
    argp_version_setup(
        b"tar\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut tar_authors as *mut *const ::core::ffi::c_char,
    );
    args.backup_suffix_string =
        getenv(b"SIMPLE_BACKUP_SUFFIX\0".as_ptr() as *const ::core::ffi::c_char);
    posixly_correct =
        !getenv(b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char).is_null();
    subcommand_option = subcommand::UNKNOWN_SUBCOMMAND;
    archive_format_0 = archive_format::DEFAULT_FORMAT;
    blocking_factor = DEFAULT_BLOCKING;
    record_size = (DEFAULT_BLOCKING * BLOCKSIZE) as size_t;
    excluded = new_exclude();
    hole_detection = hole_detection_method::HOLE_DETECTION_DEFAULT;
    newer_mtime_option.tv_sec =
        !if (0 as ::core::ffi::c_int as time_t) < -1 as ::core::ffi::c_int as time_t {
            -1 as ::core::ffi::c_int as time_t
        } else {
            (((1 as ::core::ffi::c_int as time_t)
                << ::core::mem::size_of::<time_t>()
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(2usize))
                - 1 as time_t)
                * 2 as time_t
                + 1 as time_t
        } as __time_t;
    newer_mtime_option.tv_nsec = -1 as __syscall_slong_t;
    mtime_option.tv_sec =
        !if (0 as ::core::ffi::c_int as time_t) < -1 as ::core::ffi::c_int as time_t {
            -1 as ::core::ffi::c_int as time_t
        } else {
            (((1 as ::core::ffi::c_int as time_t)
                << ::core::mem::size_of::<time_t>()
                    .wrapping_mul(CHAR_BIT as usize)
                    .wrapping_sub(2usize))
                - 1 as time_t)
                * 2 as time_t
                + 1 as time_t
        } as __time_t;
    mtime_option.tv_nsec = -1 as __syscall_slong_t;
    recursion_option = FNM_LEADING_DIR;
    unquote_option = r#true != 0;
    tar_sparse_major = 1 as ::core::ffi::c_uint;
    tar_sparse_minor = 0 as ::core::ffi::c_uint;
    savedir_sort_order = savedir_option::SAVEDIR_SORT_NONE.0 as ::core::ffi::c_int;
    owner_option = -1 as ::core::ffi::c_int as uid_t;
    owner_name_option = ::core::ptr::null::<::core::ffi::c_char>();
    group_option = -1 as ::core::ffi::c_int as gid_t;
    group_name_option = ::core::ptr::null::<::core::ffi::c_char>();
    check_device_option = r#true != 0;
    incremental_level = -1 as ::core::ffi::c_int;
    seek_option = -1 as ::core::ffi::c_int;
    if argc > 1 as ::core::ffi::c_int
        && *(*argv.offset(1isize)).offset(0isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int
    {
        let mut new_argc: ::core::ffi::c_int = 0;
        let mut new_argv: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut r#in: *const *mut ::core::ffi::c_char =
            ::core::ptr::null::<*mut ::core::ffi::c_char>();
        let mut out: *mut *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
        let mut letter: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut buffer: [::core::ffi::c_char; 3] = [0; 3];
        buffer[0usize] = '-' as ::core::ffi::c_char;
        buffer[2usize] = '\0' as ::core::ffi::c_char;
        new_argc = ((argc - 1 as ::core::ffi::c_int) as size_t)
            .wrapping_add(strlen(*argv.offset(1isize))) as ::core::ffi::c_int;
        new_argv = xmalloc(
            ((new_argc + 1 as ::core::ffi::c_int) as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        ) as *mut *mut ::core::ffi::c_char;
        rboxc_tar_own_argument(new_argv.cast());
        r#in = argv;
        out = new_argv;
        let c2rust_fresh7 = r#in;
        r#in = r#in.offset(1);
        let c2rust_fresh8 = out;
        out = out.offset(1);
        *c2rust_fresh8 = *c2rust_fresh7;
        let c2rust_fresh9 = r#in;
        r#in = r#in.offset(1);
        letter = *c2rust_fresh9;
        while *letter != 0 {
            let mut opt: *const argp_option = ::core::ptr::null::<argp_option>();
            buffer[1usize] = *letter;
            let c2rust_fresh10 = out;
            out = out.offset(1);
            *c2rust_fresh10 = xstrdup(&raw mut buffer as *mut ::core::ffi::c_char);
            rboxc_tar_own_argument((*c2rust_fresh10).cast());
            opt = find_argp_option(&raw mut argp, *letter as ::core::ffi::c_int);
            if !opt.is_null() && !(*opt).arg.is_null() {
                if r#in < argv.offset(argc as isize) as *const *mut ::core::ffi::c_char {
                    let c2rust_fresh11 = r#in;
                    r#in = r#in.offset(1);
                    let c2rust_fresh12 = out;
                    out = out.offset(1);
                    *c2rust_fresh12 = *c2rust_fresh11;
                } else {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"Old option '%c' requires an argument.\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            *letter as ::core::ffi::c_int,
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
                                    b"Old option '%c' requires an argument.\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                *letter as ::core::ffi::c_int,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                    usage(PAXEXIT_FAILURE);
                }
            }
            letter = letter.offset(1);
        }
        while r#in < argv.offset(argc as isize) as *const *mut ::core::ffi::c_char {
            let c2rust_fresh13 = r#in;
            r#in = r#in.offset(1);
            let c2rust_fresh14 = out;
            out = out.offset(1);
            *c2rust_fresh14 = *c2rust_fresh13;
        }
        *out = ::core::ptr::null_mut::<::core::ffi::c_char>();
        argc = new_argc;
        argv = new_argv;
    }
    parse_default_options(&raw mut args);
    if argp_parse(
        &raw mut argp,
        argc,
        argv,
        ARGP_IN_ORDER as ::core::ffi::c_uint,
        &raw mut idx,
        &raw mut args as *mut ::core::ffi::c_void,
    ) != 0
    {
        exit(TAREXIT_FAILURE);
    }
    if args.o_option {
        if subcommand_option.0 == subcommand::CREATE_SUBCOMMAND.0 {
            set_archive_format(b"v7\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            same_owner_option = -1 as ::core::ffi::c_int;
        }
    }
    while idx < argc {
        name_add_name(*argv.offset(idx as isize));
        idx += 1;
    }
    if archive_format_0.0 == archive_format::DEFAULT_FORMAT.0 {
        if args.pax_option {
            archive_format_0 = archive_format::POSIX_FORMAT;
        } else {
            archive_format_0 = archive_format::GNU_FORMAT;
        }
    }
    if !volume_label_option.is_null() && subcommand_option.0 == subcommand::CREATE_SUBCOMMAND.0
        || incremental_option as ::core::ffi::c_int != 0
        || multi_volume_option as ::core::ffi::c_int != 0
        || sparse_option as ::core::ffi::c_int != 0
    {
        assert_format(
            ((1 as ::core::ffi::c_int) << archive_format::OLDGNU_FORMAT.0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << archive_format::GNU_FORMAT.0 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int) << archive_format::POSIX_FORMAT.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint,
        );
    }
    if occurrence_option != 0 {
        if !name_more_files() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--occurrence is meaningless without a file list\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"--occurrence is meaningless without a file list\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
        if subcommand_class[subcommand_option.0 as usize] & 0x10 as ::core::ffi::c_int == 0 {
            if option_set_in_cl(option_class_0::OC_OCCURRENCE.0 as ::core::ffi::c_int) != 0 {
                option_conflict_error(
                    b"--occurrence\0".as_ptr() as *const ::core::ffi::c_char,
                    subcommand_string(subcommand_option),
                );
            } else {
                occurrence_option = 0 as uintmax_t;
            }
        }
    }
    if archive_names == 0 as size_t {
        archive_names = 1 as size_t;
        *archive_name_array.offset(0isize) =
            getenv(b"TAPE\0".as_ptr() as *const ::core::ffi::c_char);
        if (*archive_name_array.offset(0isize)).is_null() {
            *archive_name_array.offset(0isize) = DEFAULT_ARCHIVE.as_ptr();
        }
    }
    if archive_names > 1 as size_t && !multi_volume_option {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Multiple archive files require '-M' option\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Multiple archive files require '-M' option\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
    if !listed_incremental_option.is_null() && 0 as __syscall_slong_t <= newer_mtime_option.tv_nsec
    {
        let mut listed_loc: *mut option_locus =
            optloc_lookup(option_class_0::OC_LISTED_INCREMENTAL.0 as ::core::ffi::c_int);
        let mut newer_loc: *mut option_locus =
            optloc_lookup(option_class_0::OC_NEWER.0 as ::core::ffi::c_int);
        if optloc_eq(listed_loc, newer_loc) != 0 {
            option_conflict_error(
                b"--listed-incremental\0".as_ptr() as *const ::core::ffi::c_char,
                b"--newer\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if (*listed_loc).source.0 == option_source::OPTS_COMMAND_LINE.0 {
            listed_incremental_option = ::core::ptr::null::<::core::ffi::c_char>();
        } else {
            memset(
                &raw mut newer_mtime_option as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<timespec>(),
            );
        }
    }
    if incremental_level != -1 as ::core::ffi::c_int && listed_incremental_option.is_null() {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"--level is meaningless without --listed-incremental\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--level is meaningless without --listed-incremental\0".as_ptr()
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
    if !volume_label_option.is_null() {
        if archive_format_0.0 == archive_format::GNU_FORMAT.0
            || archive_format_0.0 == archive_format::OLDGNU_FORMAT.0
        {
            let mut volume_label_max_len: size_t =
                ::core::mem::size_of::<[::core::ffi::c_char; 100]>()
                    .wrapping_sub(1 as size_t)
                    .wrapping_sub(if multi_volume_option as ::core::ffi::c_int != 0 {
                        ::core::mem::size_of::<[::core::ffi::c_char; 9]>()
                            .wrapping_sub(1 as size_t)
                            .wrapping_add(
                                ::core::mem::size_of::<::core::ffi::c_int>()
                                    .wrapping_mul(8 as size_t)
                                    .wrapping_sub(
                                        !((0 as ::core::ffi::c_int) < -1 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as size_t,
                                    )
                                    .wrapping_mul(146 as size_t)
                                    .wrapping_add(484 as size_t)
                                    .wrapping_div(485 as size_t)
                                    .wrapping_add(
                                        !((0 as ::core::ffi::c_int) < -1 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int
                                            as size_t,
                                    ),
                            )
                            .wrapping_sub(1 as size_t)
                    } else {
                        0 as size_t
                    });
            if volume_label_max_len < strlen(volume_label_option) {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcngettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s: Volume label is too long (limit is %lu byte)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"%s: Volume label is too long (limit is %lu bytes)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            volume_label_max_len as ::core::ffi::c_ulong,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_colon(volume_label_option),
                        volume_label_max_len as ::core::ffi::c_ulong,
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
                            dcngettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"%s: Volume label is too long (limit is %lu byte)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                b"%s: Volume label is too long (limit is %lu bytes)\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                volume_label_max_len as ::core::ffi::c_ulong,
                                5 as ::core::ffi::c_int,
                            ),
                            quotearg_colon(volume_label_option),
                            volume_label_max_len as ::core::ffi::c_ulong,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                usage(PAXEXIT_FAILURE);
            }
        }
    }
    if verify_option {
        if multi_volume_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot verify multi-volume archives\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot verify multi-volume archives\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
        if !use_compress_program_option.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot verify compressed archives\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot verify compressed archives\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
        if subcommand_class[subcommand_option.0 as usize] & 0x2 as ::core::ffi::c_int == 0 {
            if option_set_in_cl(option_class_0::OC_VERIFY.0 as ::core::ffi::c_int) != 0 {
                option_conflict_error(
                    b"--verify\0".as_ptr() as *const ::core::ffi::c_char,
                    subcommand_string(subcommand_option),
                );
            } else {
                verify_option = r#false != 0;
            }
        }
    }
    if !use_compress_program_option.is_null() {
        if multi_volume_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot use multi-volume compressed archives\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot use multi-volume compressed archives\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
        if subcommand_class[subcommand_option.0 as usize] & 0x4 as ::core::ffi::c_int != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot update compressed archives\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot update compressed archives\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
        if subcommand_option.0 == subcommand::CAT_SUBCOMMAND.0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot concatenate compressed archives\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot concatenate compressed archives\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
    }
    if set_mtime_option.0 == set_mtime_option_mode::CLAMP_MTIME.0 {
        if !(0 as __syscall_slong_t <= mtime_option.tv_nsec) {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--clamp-mtime needs a date specified using --mtime\0".as_ptr()
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
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"--clamp-mtime needs a date specified using --mtime\0".as_ptr()
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
            usage(PAXEXIT_FAILURE);
        }
    }
    if args.pax_option as ::core::ffi::c_int != 0
        && archive_format_0.0 != archive_format::POSIX_FORMAT.0
        && subcommand_class[subcommand_option.0 as usize] & 0x1 as ::core::ffi::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"--pax-option can be used only on POSIX archives\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--pax-option can be used only on POSIX archives\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
    if acls_option > 0 as ::core::ffi::c_int
        && archive_format_0.0 != archive_format::POSIX_FORMAT.0
        && subcommand_class[subcommand_option.0 as usize] & 0x1 as ::core::ffi::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"--acls can be used only on POSIX archives\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--acls can be used only on POSIX archives\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
    if selinux_context_option > 0 as ::core::ffi::c_int
        && archive_format_0.0 != archive_format::POSIX_FORMAT.0
        && subcommand_class[subcommand_option.0 as usize] & 0x1 as ::core::ffi::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"--selinux can be used only on POSIX archives\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--selinux can be used only on POSIX archives\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
    if xattrs_option > 0 as ::core::ffi::c_int
        && archive_format_0.0 != archive_format::POSIX_FORMAT.0
        && subcommand_class[subcommand_option.0 as usize] & 0x1 as ::core::ffi::c_int == 0
    {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"--xattrs can be used only on POSIX archives\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--xattrs can be used only on POSIX archives\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
    if starting_file_option as ::core::ffi::c_int != 0
        && subcommand_class[subcommand_option.0 as usize] & 0x1 as ::core::ffi::c_int == 0
    {
        if option_set_in_cl(option_class_0::OC_STARTING_FILE.0 as ::core::ffi::c_int) != 0 {
            option_conflict_error(
                b"--starting-file\0".as_ptr() as *const ::core::ffi::c_char,
                subcommand_string(subcommand_option),
            );
        } else {
            starting_file_option = r#false != 0;
        }
    }
    if same_order_option as ::core::ffi::c_int != 0
        && subcommand_class[subcommand_option.0 as usize] & 0x1 as ::core::ffi::c_int == 0
    {
        if option_set_in_cl(option_class_0::OC_SAME_ORDER.0 as ::core::ffi::c_int) != 0 {
            option_conflict_error(
                b"--same-order\0".as_ptr() as *const ::core::ffi::c_char,
                subcommand_string(subcommand_option),
            );
        } else {
            same_order_option = r#false != 0;
        }
    }
    if one_top_level_option {
        let mut base: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if absolute_names_option {
            let mut one_top_level_loc: *mut option_locus =
                optloc_lookup(option_class_0::OC_ONE_TOP_LEVEL.0 as ::core::ffi::c_int);
            let mut absolute_names_loc: *mut option_locus =
                optloc_lookup(option_class_0::OC_ABSOLUTE_NAMES.0 as ::core::ffi::c_int);
            if optloc_eq(one_top_level_loc, absolute_names_loc) != 0 {
                option_conflict_error(
                    b"--one-top-level\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--absolute-names\0".as_ptr() as *const ::core::ffi::c_char,
                );
            } else if (*one_top_level_loc).source.0 == option_source::OPTS_COMMAND_LINE.0 {
                absolute_names_option = r#false != 0;
            } else {
                one_top_level_option = r#false != 0;
            }
        }
        if one_top_level_option as ::core::ffi::c_int != 0 && one_top_level_dir.is_null() {
            base = base_name(*archive_name_array.offset(0isize));
            one_top_level_dir = strip_compression_suffix(base);
            free(base as *mut ::core::ffi::c_void);
            if one_top_level_dir.is_null() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR\0"
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
                                b"Cannot deduce top-level directory name; please set it explicitly with --one-top-level=DIR\0"
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
                usage(PAXEXIT_FAILURE);
            }
        }
    }
    if recursive_unlink_option {
        old_files_option = old_files::UNLINK_FIRST_OLD_FILES;
    }
    let mut base_open_flags: ::core::ffi::c_int = O_BINARY
        | O_CLOEXEC
        | O_NOCTTY
        | O_NONBLOCK
        | if dereference_option as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            O_NOFOLLOW
        }
        | if atime_preserve_option.0 == atime_preserve::system_atime_preserve.0 {
            O_NOATIME
        } else {
            0 as ::core::ffi::c_int
        };
    open_read_flags = O_RDONLY | base_open_flags;
    open_searchdir_flags = O_SEARCH | O_DIRECTORY | base_open_flags;
    fstatat_flags = if dereference_option as ::core::ffi::c_int != 0 {
        0 as ::core::ffi::c_int
    } else {
        AT_SYMLINK_NOFOLLOW
    };
    if subcommand_option.0 == subcommand::TEST_LABEL_SUBCOMMAND.0 {
        if !name_more_files() {
            verbose_option += 1;
        }
    } else if utc_option {
        verbose_option = 2 as ::core::ffi::c_int;
    }
    if tape_length_option != 0. && tape_length_option < record_size as tarlong {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Volume length cannot be less than record size\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Volume length cannot be less than record size\0".as_ptr()
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
        usage(PAXEXIT_FAILURE);
    }
    if same_order_option as ::core::ffi::c_int != 0 && !listed_incremental_option.is_null() {
        let mut preserve_order_loc: *mut option_locus =
            optloc_lookup(option_class_0::OC_SAME_ORDER.0 as ::core::ffi::c_int);
        let mut listed_incremental_loc: *mut option_locus =
            optloc_lookup(option_class_0::OC_LISTED_INCREMENTAL.0 as ::core::ffi::c_int);
        if optloc_eq(preserve_order_loc, listed_incremental_loc) != 0 {
            option_conflict_error(
                b"--preserve-order\0".as_ptr() as *const ::core::ffi::c_char,
                b"--listed-incremental\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if (*preserve_order_loc).source.0 == option_source::OPTS_COMMAND_LINE.0 {
            listed_incremental_option = ::core::ptr::null::<::core::ffi::c_char>();
        } else {
            same_order_option = r#false != 0;
        }
    }
    match subcommand_option {
        subcommand::CREATE_SUBCOMMAND => {
            if !name_more_files() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cowardly refusing to create an empty archive\0".as_ptr()
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
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"Cowardly refusing to create an empty archive\0".as_ptr()
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
                usage(PAXEXIT_FAILURE);
            }
            if args.compress_autodetect as ::core::ffi::c_int != 0
                && archive_names != 0
                && strcmp(
                    *archive_name_array.offset(0isize),
                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                ) != 0
            {
                set_compression_program_by_suffix(
                    *archive_name_array.offset(0isize),
                    use_compress_program_option,
                );
            }
        }
        subcommand::EXTRACT_SUBCOMMAND
        | subcommand::LIST_SUBCOMMAND
        | subcommand::DIFF_SUBCOMMAND
        | subcommand::TEST_LABEL_SUBCOMMAND => {
            archive_name_cursor = archive_name_array;
            while archive_name_cursor < archive_name_array.offset(archive_names as isize) {
                if strcmp(
                    *archive_name_cursor,
                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
                {
                    request_stdin(b"-f\0".as_ptr() as *const ::core::ffi::c_char);
                }
                archive_name_cursor = archive_name_cursor.offset(1);
            }
        }
        subcommand::CAT_SUBCOMMAND
        | subcommand::UPDATE_SUBCOMMAND
        | subcommand::APPEND_SUBCOMMAND => {
            archive_name_cursor = archive_name_array;
            while archive_name_cursor < archive_name_array.offset(archive_names as isize) {
                if strcmp(
                    *archive_name_cursor,
                    b"-\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0
                {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"Options '-Aru' are incompatible with '-f -'\0".as_ptr()
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
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Options '-Aru' are incompatible with '-f -'\0".as_ptr()
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
                    usage(PAXEXIT_FAILURE);
                }
                archive_name_cursor = archive_name_cursor.offset(1);
            }
        }
        _ => {}
    }
    if !index_file_name.is_null() {
        stdlis = fopen(
            index_file_name,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
        ) as *mut FILE;
        if stdlis.is_null() {
            open_fatal(index_file_name);
        }
    } else {
        stdlis = if to_stdout_option as ::core::ffi::c_int != 0 {
            stderr
        } else {
            stdout
        };
    }
    archive_name_cursor = archive_name_array;
    if !args.backup_suffix_string.is_null() {
        simple_backup_suffix = xstrdup(args.backup_suffix_string);
    }
    if backup_option {
        backup_type_0 = xget_version(
            b"--backup\0".as_ptr() as *const ::core::ffi::c_char,
            args.version_control_string,
        );
        if backup_type_0.0 == backup_type::no_backups.0
            || (to_stdout_option as ::core::ffi::c_int != 0 || !to_command_option.is_null())
        {
            backup_option = r#false != 0;
        }
    }
    checkpoint_finish_compile();
    report_textual_dates(&raw mut args);
}
unsafe extern "C" fn rboxc_tar_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    set_start_time();
    set_program_name(*argv.offset(0isize));
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    ::core::ptr::write_volatile(&raw mut exit_failure, TAREXIT_FAILURE);
    exit_status = TAREXIT_SUCCESS;
    error_hook = Some(checkpoint_flush_actions as unsafe extern "C" fn() -> ())
        as Option<unsafe extern "C" fn() -> ()>;
    set_quoting_style(
        ::core::ptr::null_mut::<quoting_options>(),
        quoting_style::escape_quoting_style,
    );
    close_stdout_set_file_name(dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"stdout\0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    ));
    if stdopen() != 0 {
        if error_hook.is_some() {
            error_hook.expect("non-null function pointer")();
        }
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"failed to assert availability of the standard file descriptors\0".as_ptr()
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
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"failed to assert availability of the standard file descriptors\0".as_ptr()
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
        fatal_exit();
    }
    allocated_archive_names = 10 as size_t;
    archive_name_array = xmalloc(
        ::core::mem::size_of::<*const ::core::ffi::c_char>().wrapping_mul(allocated_archive_names),
    ) as *mut *const ::core::ffi::c_char;
    archive_names = 0 as size_t;
    signal(SIGCHLD, SIG_DFL);
    priv_set_remove_linkdir();
    decode_options(argc, argv);
    name_init();
    if !volno_file_option.is_null() {
        init_volume_number();
    }
    's_180: {
        match subcommand_option {
            subcommand::UNKNOWN_SUBCOMMAND => {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"You must specify one of the '-Acdtrux', '--delete' or '--test-label' options\0"
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
                                b"You must specify one of the '-Acdtrux', '--delete' or '--test-label' options\0"
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
                usage(PAXEXIT_FAILURE);
            }
            subcommand::CAT_SUBCOMMAND
            | subcommand::UPDATE_SUBCOMMAND
            | subcommand::APPEND_SUBCOMMAND => {}
            subcommand::DELETE_SUBCOMMAND => {
                delete_archive_members();
                break 's_180;
            }
            subcommand::CREATE_SUBCOMMAND => {
                create_archive();
                break 's_180;
            }
            subcommand::EXTRACT_SUBCOMMAND => {
                extr_init();
                read_and(Some(extract_archive as unsafe extern "C" fn() -> ()));
                extract_finish();
                break 's_180;
            }
            subcommand::LIST_SUBCOMMAND => {
                read_and(Some(list_archive as unsafe extern "C" fn() -> ()));
                break 's_180;
            }
            subcommand::DIFF_SUBCOMMAND => {
                diff_init();
                read_and(Some(diff_archive as unsafe extern "C" fn() -> ()));
                break 's_180;
            }
            subcommand::TEST_LABEL_SUBCOMMAND => {
                test_archive_label();
                break 's_180;
            }
            _ => {
                break 's_180;
            }
        }
        update_archive();
    }
    checkpoint_finish();
    if totals_option {
        print_total_stats();
    }
    if check_links_option != 0 {
        check_links();
    }
    if !volno_file_option.is_null() {
        closeout_volume_number();
    }
    if exit_status == TAREXIT_FAILURE {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Exiting with failure status due to previous errors\0".as_ptr()
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
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Exiting with failure status due to previous errors\0".as_ptr()
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
    if stdlis == stdout {
        close_stdout();
    } else if ferror_unlocked(stderr) != 0 || fclose(stderr) != 0 as ::core::ffi::c_int {
        set_exit_status(TAREXIT_FAILURE);
    }
    return exit_status;
}
#[export_name = "rboxc_tar_tar_stat_init"]
pub unsafe extern "C" fn tar_stat_init(mut st: *mut tar_stat_info) {
    memset(
        st as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tar_stat_info>(),
    );
}
#[export_name = "rboxc_tar_tar_stat_close"]
pub unsafe extern "C" fn tar_stat_close(mut st: *mut tar_stat_info) -> bool {
    let mut status: ::core::ffi::c_int = if !(*st).dirstream.is_null() {
        closedir((*st).dirstream)
    } else if (0 as ::core::ffi::c_int) < (*st).fd {
        close((*st).fd)
    } else {
        0 as ::core::ffi::c_int
    };
    (*st).dirstream = ::core::ptr::null_mut::<DIR>();
    (*st).fd = 0 as ::core::ffi::c_int;
    if status == 0 as ::core::ffi::c_int {
        return r#true != 0;
    } else {
        close_diag((*st).orig_file_name);
        return r#false != 0;
    };
}
#[export_name = "rboxc_tar_tar_stat_destroy"]
pub unsafe extern "C" fn tar_stat_destroy(mut st: *mut tar_stat_info) {
    tar_stat_close(st);
    xattr_map_free(&raw mut (*st).xattr_map);
    free((*st).orig_file_name as *mut ::core::ffi::c_void);
    free((*st).file_name as *mut ::core::ffi::c_void);
    free((*st).link_name as *mut ::core::ffi::c_void);
    free((*st).uname as *mut ::core::ffi::c_void);
    free((*st).gname as *mut ::core::ffi::c_void);
    free((*st).cntx_name as *mut ::core::ffi::c_void);
    free((*st).acls_a_ptr as *mut ::core::ffi::c_void);
    free((*st).acls_d_ptr as *mut ::core::ffi::c_void);
    free((*st).sparse_map as *mut ::core::ffi::c_void);
    free((*st).dumpdir as *mut ::core::ffi::c_void);
    xheader_destroy(&raw mut (*st).xhdr);
    info_free_exclist(st);
    memset(
        st as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<tar_stat_info>(),
    );
}
#[export_name = "rboxc_tar_tar_timespec_cmp"]
pub unsafe extern "C" fn tar_timespec_cmp(mut a: timespec, mut b: timespec) -> ::core::ffi::c_int {
    if (1 as ::core::ffi::c_int) << current_format.0
        & (1 as ::core::ffi::c_int) << archive_format::POSIX_FORMAT.0 as ::core::ffi::c_int
        == 0
    {
        b.tv_nsec = 0 as __syscall_slong_t;
        a.tv_nsec = b.tv_nsec;
    }
    return timespec_cmp(a, b);
}
#[export_name = "rboxc_tar_set_exit_status"]
pub unsafe extern "C" fn set_exit_status(mut val: ::core::ffi::c_int) {
    if val > exit_status {
        exit_status = val;
    }
}
pub const TTY_NAME: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"/dev/tty\0") };
pub const __CHAR_BIT__: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
static mut RBOXC_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_tar_error_prefix() {
    libc::fprintf(stderr.cast(), b"%s: \0".as_ptr().cast(), RBOXC_INVOCATION);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_tar(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    RBOXC_INVOCATION = if argv.is_null() || (*argv).is_null() {
        b"tar\0".as_ptr().cast()
    } else { *argv };
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_tar_error_prefix);
    }
    if libc::atexit(rboxc_tar_release_arguments) != 0 {
        libc::_exit(2);
    }
    rboxc_tar_main_inner(argc, argv)
}

static mut RBOXC_DEFAULT_WORDS: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: ::core::ptr::null::<::core::ffi::c_char>(),
        ws_comment: ::core::ptr::null::<::core::ffi::c_char>(),
        ws_escape: [::core::ptr::null::<::core::ffi::c_char>(); 2],
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
        ws_envbuf: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_getvar: None,
        ws_closure: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        ws_command: None,
        ws_input: ::core::ptr::null::<::core::ffi::c_char>(),
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        ws_head: ::core::ptr::null_mut::<wordsplit_node>(),
        ws_tail: ::core::ptr::null_mut::<wordsplit_node>(),
        ws_lvl: 0,
    };

extern "C" {
    #[link_name = "rboxc_tar_wordsplit_clearerr"]
    fn rboxc_wordsplit_clearerr(ws: *mut wordsplit);
}
static mut RBOXC_OWNED_ARGS: Vec<*mut ::core::ffi::c_void> = Vec::new();
unsafe fn rboxc_tar_own_argument(pointer: *mut ::core::ffi::c_void) {
    RBOXC_OWNED_ARGS.push(pointer);
}
extern "C" fn rboxc_tar_release_arguments() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        wordsplit_free(&raw mut RBOXC_DEFAULT_WORDS);
        rboxc_wordsplit_clearerr(&raw mut RBOXC_DEFAULT_WORDS);
        for pointer in ::core::mem::take(&mut *(&raw mut RBOXC_OWNED_ARGS)) {
            free(pointer);
        }
        *libc::__errno_location() = saved_errno;
    }
}
