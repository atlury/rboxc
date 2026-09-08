// Generated from pinned GNU Cpio 2.15 by scripts/translate-cpio.py.
// Source SHA-256: d5f18852c30da812a4af809d1d1382debe3b8a055a6d99de4063da24c012e652
/* main.c - main program and argument processing for cpio.
   Copyright (C) 1990-2024 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public
   License along with this program; if not, write to the Free
   Software Foundation, Inc., 51 Franklin Street, Fifth Floor,
   Boston, MA 02110-1301 USA.  */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
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
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn lstat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn geteuid() -> __uid_t;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stderr: *mut FILE;
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
    #[link_name = "rboxc_cpio_cpio_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
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
    #[link_name = "rboxc_cpio_cpio_error_hook"]
    static mut error_hook: Option<unsafe extern "C" fn() -> ()>;
    #[link_name = "rboxc_cpio_cpio_pax_exit"]
    fn pax_exit();
    #[link_name = "rboxc_cpio_cpio_argp_parse"]
    fn argp_parse(
        __argp: *const argp,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
        __flags: ::core::ffi::c_uint,
        __arg_index: *mut ::core::ffi::c_int,
        __input: *mut ::core::ffi::c_void,
    ) -> error_t;
    #[link_name = "rboxc_cpio_cpio_argp_help"]
    fn argp_help(
        __argp: *const argp,
        __stream: *mut FILE,
        __flags: ::core::ffi::c_uint,
        __name: *mut ::core::ffi::c_char,
    );
    #[link_name = "rboxc_cpio_cpio_argp_version_setup"]
    fn argp_version_setup(
        name: *const ::core::ffi::c_char,
        authors: *const *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_cpio_cpio_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_cpio_cpio_close_stdout"]
    fn close_stdout();
    #[link_name = "rboxc_cpio_cpio_quotearg_colon"]
    fn quotearg_colon(arg: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_archive_format"]
    static mut archive_format_0: archive_format;
    #[link_name = "rboxc_cpio_cpio_reset_time_flag"]
    static mut reset_time_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_io_block_size"]
    static mut io_block_size: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_create_dir_flag"]
    static mut create_dir_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_rename_flag"]
    static mut rename_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_rename_batch_file"]
    static mut rename_batch_file: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_table_flag"]
    static mut table_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_unconditional_flag"]
    static mut unconditional_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_verbose_flag"]
    static mut verbose_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_dot_flag"]
    static mut dot_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_link_flag"]
    static mut link_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_retain_time_flag"]
    static mut retain_time_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_crc_i_flag"]
    static mut crc_i_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_append_flag"]
    static mut append_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_swap_bytes_flag"]
    static mut swap_bytes_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_swap_halfwords_flag"]
    static mut swap_halfwords_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_set_owner_flag"]
    static mut set_owner_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_set_owner"]
    static mut set_owner: uid_t;
    #[link_name = "rboxc_cpio_cpio_set_group_flag"]
    static mut set_group_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_set_group"]
    static mut set_group: gid_t;
    #[link_name = "rboxc_cpio_cpio_no_chown_flag"]
    static mut no_chown_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_sparse_flag"]
    static mut sparse_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_quiet_flag"]
    static mut quiet_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_only_verify_crc_flag"]
    static mut only_verify_crc_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_no_abs_paths_flag"]
    static mut no_abs_paths_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_warn_option"]
    static mut warn_option: ::core::ffi::c_uint;
    #[link_name = "rboxc_cpio_cpio_renumber_inodes_option"]
    static mut renumber_inodes_option: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_ignore_devno_option"]
    static mut ignore_devno_option: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_ignore_dirnlink_option"]
    static mut ignore_dirnlink_option: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_to_stdout_option"]
    static mut to_stdout_option: bool;
    #[link_name = "rboxc_cpio_cpio_copy_matching_files"]
    static mut copy_matching_files: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_numeric_uid"]
    static mut numeric_uid: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_pattern_file_name"]
    static mut pattern_file_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_archive_des"]
    static mut archive_des: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_archive_name"]
    static mut archive_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_rsh_command_option"]
    static mut rsh_command_option: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_input_buffer"]
    static mut input_buffer: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_output_buffer"]
    static mut output_buffer: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_in_buff"]
    static mut in_buff: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_out_buff"]
    static mut out_buff: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_input_buffer_size"]
    static mut input_buffer_size: size_t;
    #[link_name = "rboxc_cpio_cpio_input_size"]
    static mut input_size: size_t;
    #[link_name = "rboxc_cpio_cpio_output_size"]
    static mut output_size: size_t;
    #[link_name = "rboxc_cpio_cpio_input_bytes"]
    static mut input_bytes: off_t;
    #[link_name = "rboxc_cpio_cpio_output_bytes"]
    static mut output_bytes: off_t;
    #[link_name = "rboxc_cpio_cpio_directory_name"]
    static mut directory_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_save_patterns"]
    static mut save_patterns: *mut *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_num_patterns"]
    static mut num_patterns: ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_name_end"]
    static mut name_end: ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_xstat"]
    static mut xstat: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
    #[link_name = "rboxc_cpio_cpio_copy_function"]
    static mut copy_function: Option<unsafe extern "C" fn() -> ()>;
    #[link_name = "rboxc_cpio_cpio_change_directory_option"]
    static mut change_directory_option: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_process_copy_in"]
    fn process_copy_in();
    #[link_name = "rboxc_cpio_cpio_process_copy_out"]
    fn process_copy_out();
    #[link_name = "rboxc_cpio_cpio_process_copy_pass"]
    fn process_copy_pass();
    #[link_name = "rboxc_cpio_cpio_parse_user_spec"]
    fn parse_user_spec(
        spec_arg: *const ::core::ffi::c_char,
        uid: *mut uid_t,
        gid: *mut gid_t,
        username_arg: *mut *mut ::core::ffi::c_char,
        groupname_arg: *mut *mut ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_cpio_cpio_open_archive"]
    fn open_archive(file: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_set_new_media_message"]
    fn set_new_media_message(message: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_cpio_cpio_arf_stores_inode_p"]
    fn arf_stores_inode_p(arf: archive_format) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_rmt_close__"]
    fn rmt_close__(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_cpio_cpio_force_local_option"]
    static mut force_local_option: bool;
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
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct archive_format(pub ::core::ffi::c_uint);
impl archive_format {
    pub const arf_unknown: Self = Self(0);
    pub const arf_binary: Self = Self(1);
    pub const arf_oldascii: Self = Self(2);
    pub const arf_newascii: Self = Self(3);
    pub const arf_crcascii: Self = Self(4);
    pub const arf_tar: Self = Self(5);
    pub const arf_ustar: Self = Self(6);
    pub const arf_hpoldascii: Self = Self(7);
    pub const arf_hpbinary: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct warn_tab {
    pub name: *mut ::core::ffi::c_char,
    pub flag: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct cpio_options(pub ::core::ffi::c_uint);
impl cpio_options {
    pub const NO_ABSOLUTE_FILENAMES_OPTION: Self = Self(256);
    pub const ABSOLUTE_FILENAMES_OPTION: Self = Self(257);
    pub const NO_PRESERVE_OWNER_OPTION: Self = Self(258);
    pub const ONLY_VERIFY_CRC_OPTION: Self = Self(259);
    pub const RENAME_BATCH_FILE_OPTION: Self = Self(260);
    pub const RSH_COMMAND_OPTION: Self = Self(261);
    pub const QUIET_OPTION: Self = Self(262);
    pub const SPARSE_OPTION: Self = Self(263);
    pub const FORCE_LOCAL_OPTION: Self = Self(264);
    pub const DEBUG_OPTION: Self = Self(265);
    pub const BLOCK_SIZE_OPTION: Self = Self(266);
    pub const TO_STDOUT_OPTION: Self = Self(267);
    pub const RENUMBER_INODES_OPTION: Self = Self(268);
    pub const IGNORE_DEVNO_OPTION: Self = Self(269);
    pub const IGNORE_DIRNLINK_OPTION: Self = Self(270);
    pub const DEVICE_INDEPENDENT_OPTION: Self = Self(271);
}
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LOCALEDIR: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"/root/rboxc/build/oracle/cpio/share/locale\0",
    )
};
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const PAXEXIT_FAILURE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PACKAGE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"cpio\0") };
pub const OPTION_HIDDEN: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const OPTION_ALIAS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const ARGP_ERR_UNKNOWN: ::core::ffi::c_int = E2BIG;
pub const ARGP_IN_ORDER: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const ARGP_HELP_SEE: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const CPIO_WARN_TRUNCATE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const CPIO_WARN_INTERDIR: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const CPIO_WARN_ALL: ::core::ffi::c_uint = -1 as ::core::ffi::c_int as ::core::ffi::c_uint;
pub const DISK_IO_BLOCK_SIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
pub const __REM_BIAS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 30 as ::core::ffi::c_int;
#[export_name = "rboxc_cpio_cpio_program_authors"]
pub static mut program_authors: [*const ::core::ffi::c_char; 5] = [
    b"Phil Nelson\0".as_ptr() as *const ::core::ffi::c_char,
    b"David MacKenzie\0".as_ptr() as *const ::core::ffi::c_char,
    b"John Oleynick\0".as_ptr() as *const ::core::ffi::c_char,
    b"Sergey Poznyakoff\0".as_ptr() as *const ::core::ffi::c_char,
    ::core::ptr::null::<::core::ffi::c_char>(),
];
#[export_name = "rboxc_cpio_cpio_argp_program_bug_address"]
pub static mut argp_program_bug_address: *const ::core::ffi::c_char =
    b"<bug-cpio@gnu.org>\0".as_ptr() as *const ::core::ffi::c_char;
static mut doc: [::core::ffi::c_char; 300] = unsafe {
    ::core::mem::transmute::<
        [u8; 300],
        [::core::ffi::c_char; 300],
    >(
        *b"GNU `cpio' copies files to and from archives\n\nExamples:\n  # Copy files named in name-list to the archive\n  cpio -o < name-list [> archive]\n  # Extract files from the archive\n  cpio -i [< archive]\n  # Copy files named in name-list to destination-directory\n  cpio -p destination-directory < name-list\n\0",
    )
};
static mut options: [argp_option; 58] = [
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Main operation mode:\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_7,
    },
    argp_option {
        name: b"create\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'o' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Create the archive (run in copy-out mode)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_7,
    },
    argp_option {
        name: b"extract\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'i' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Extract files from an archive (run in copy-in mode)\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_7,
    },
    argp_option {
        name: b"pass-through\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'p' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Run in copy-pass mode\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_7,
    },
    argp_option {
        name: b"list\0".as_ptr() as *const ::core::ffi::c_char,
        key: 't' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Print a table of contents of the input\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_7,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid in any mode:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6,
    },
    argp_option {
        name: b"directory\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'D' as ::core::ffi::c_int,
        arg: b"DIR\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Change to directory DIR\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"force-local\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::FORCE_LOCAL_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Archive file is local, even if its name contains colons\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"format\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'H' as ::core::ffi::c_int,
        arg: b"FORMAT\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Use given archive FORMAT\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'B' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Set the I/O block size to 5120 bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"block-size\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::BLOCK_SIZE_OPTION.0 as ::core::ffi::c_int,
        arg: b"BLOCK-SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Set the I/O block size to BLOCK-SIZE * 512 bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'c' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Use the old portable (ASCII) archive format\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"dot\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'V' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Print a \".\" for each file processed\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"io-size\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'C' as ::core::ffi::c_int,
        arg: b"NUMBER\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Set the I/O block size to the given NUMBER of bytes\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::QUIET_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Do not print the number of blocks copied\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'v' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Verbosely list the files processed\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"warning\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'W' as ::core::ffi::c_int,
        arg: b"FLAG\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Control warning display. Currently FLAG is one of 'none', 'truncate', 'all'. Multiple options accumulate.\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"owner\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'R' as ::core::ffi::c_int,
        arg: b"[USER][:.][GROUP]\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Set the ownership of all files created to the specified USER and/or GROUP\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_6 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid in copy-in and copy-out modes\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_5,
    },
    argp_option {
        name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'F' as ::core::ffi::c_int,
        arg: b"[[USER@]HOST:]FILE-NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Use this FILE-NAME instead of standard input or output. Optional USER and HOST specify the user and host names in case of a remote archive\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_5 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"message\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'M' as ::core::ffi::c_int,
        arg: b"STRING\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Print STRING when the end of a volume of the backup media is reached\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_5 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rsh-command\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::RSH_COMMAND_OPTION.0 as ::core::ffi::c_int,
        arg: b"COMMAND\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Use COMMAND instead of rsh\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_5 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid only in copy-in mode:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_4,
    },
    argp_option {
        name: b"nonmatching\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'f' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Only copy files that do not match any of the given patterns\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"numeric-uid-gid\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'n' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"In the verbose table of contents listing, show numeric UID and GID\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"pattern-file\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'E' as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Read additional patterns specifying filenames to extract or list from FILE\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: 210 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"only-verify-crc\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::ONLY_VERIFY_CRC_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"When reading a CRC format archive, only verify the CRC's of each file in the archive, don't actually extract the files\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: 210 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rename\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'r' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Interactively rename files\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"rename-batch-file\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::RENAME_BATCH_FILE_OPTION.0 as ::core::ffi::c_int,
        arg: b"FILE\0".as_ptr() as *const ::core::ffi::c_char,
        flags: OPTION_HIDDEN,
        doc: b"\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"swap\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'b' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Swap both halfwords of words and bytes of halfwords in the data. Equivalent to -sS\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"swap-bytes\0".as_ptr() as *const ::core::ffi::c_char,
        key: 's' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Swap the bytes of each halfword in the files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"swap-halfwords\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'S' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Swap the halfwords of each word (4 bytes) in the files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::TO_STDOUT_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Extract files to standard output\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'I' as ::core::ffi::c_int,
        arg: b"[[USER@]HOST:]FILE-NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Archive filename to use instead of standard input. Optional USER and HOST specify the user and host names in case of a remote archive\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_4 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid only in copy-out mode:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_3,
    },
    argp_option {
        name: b"append\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'A' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Append to an existing archive.\0".as_ptr() as *const ::core::ffi::c_char,
        group: GRID_3 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 'O' as ::core::ffi::c_int,
        arg: b"[[USER@]HOST:]FILE-NAME\0".as_ptr() as *const ::core::ffi::c_char,
        flags: 0 as ::core::ffi::c_int,
        doc: b"Archive filename to use instead of standard output. Optional USER and HOST specify the user and host names in case of a remote archive\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_3 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"renumber-inodes\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::RENUMBER_INODES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Renumber inodes\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"ignore-devno\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::IGNORE_DEVNO_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Don't store device numbers\0".as_ptr() as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"ignore-dirnlink\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::IGNORE_DIRNLINK_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"ignore number of links of a directory; always assume 2\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"device-independent\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::DEVICE_INDEPENDENT_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Create device-independent (reproducible) archives\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: 0,
    },
    argp_option {
        name: b"reproducible\0".as_ptr() as *const ::core::ffi::c_char,
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: OPTION_ALIAS,
        doc: ::core::ptr::null::<::core::ffi::c_char>(),
        group: 0,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid only in copy-pass mode:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_2,
    },
    argp_option {
        name: b"link\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'l' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Link files instead of copying them, when  possible\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_2 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid in copy-in and copy-out modes:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_1,
    },
    argp_option {
        name: b"absolute-filenames\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::ABSOLUTE_FILENAMES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Do not strip file system prefix components from the file names\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-absolute-filenames\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::NO_ABSOLUTE_FILENAMES_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Create all files relative to the current directory\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_1 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid in copy-out and copy-pass modes:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_0,
    },
    argp_option {
        name: b"null\0".as_ptr() as *const ::core::ffi::c_char,
        key: '0' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Filenames in the list are delimited by null characters instead of newlines\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"dereference\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'L' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Dereference  symbolic  links  (copy  the files that they point to instead of copying the links).\0"
            .as_ptr() as *const ::core::ffi::c_char,
        group: GRID_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"reset-access-time\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'a' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Reset the access times of files after reading them\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID_0 + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        key: 0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Operation modifiers valid in copy-in and copy-pass modes:\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID,
    },
    argp_option {
        name: b"preserve-modification-time\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'm' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Retain previous file modification times when creating files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"make-directories\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'd' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Create leading directories where needed\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"no-preserve-owner\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::NO_PRESERVE_OWNER_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Do not change the ownership of the files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"unconditional\0".as_ptr() as *const ::core::ffi::c_char,
        key: 'u' as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Replace all files unconditionally\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID + 1 as ::core::ffi::c_int,
    },
    argp_option {
        name: b"sparse\0".as_ptr() as *const ::core::ffi::c_char,
        key: cpio_options::SPARSE_OPTION.0 as ::core::ffi::c_int,
        arg: ::core::ptr::null::<::core::ffi::c_char>(),
        flags: 0 as ::core::ffi::c_int,
        doc: b"Write files with large blocks of zeros as sparse files\0".as_ptr()
            as *const ::core::ffi::c_char,
        group: GRID + 1 as ::core::ffi::c_int,
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
pub const GRID_7: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const GRID_6: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const GRID_5: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const GRID_4: ::core::ffi::c_int = 200 as ::core::ffi::c_int;
pub const GRID_3: ::core::ffi::c_int = 300 as ::core::ffi::c_int;
pub const GRID_2: ::core::ffi::c_int = 400 as ::core::ffi::c_int;
pub const GRID_1: ::core::ffi::c_int = 500 as ::core::ffi::c_int;
pub const GRID_0: ::core::ffi::c_int = 600 as ::core::ffi::c_int;
pub const GRID: ::core::ffi::c_int = 700 as ::core::ffi::c_int;
static mut input_archive_name: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut output_archive_name: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
unsafe extern "C" fn warn_control(mut arg: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    static mut warn_tab: [warn_tab; 5] = [
        warn_tab {
            name: b"none\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            flag: CPIO_WARN_ALL as ::core::ffi::c_int,
        },
        warn_tab {
            name: b"truncate\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            flag: CPIO_WARN_TRUNCATE,
        },
        warn_tab {
            name: b"all\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            flag: CPIO_WARN_ALL as ::core::ffi::c_int,
        },
        warn_tab {
            name: b"interdir\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            flag: CPIO_WARN_INTERDIR,
        },
        warn_tab {
            name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            flag: 0,
        },
    ];
    let mut wt: *mut warn_tab = ::core::ptr::null_mut::<warn_tab>();
    let mut offset: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if strcmp(arg, b"none\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
        warn_option = 0 as ::core::ffi::c_uint;
        return 0 as ::core::ffi::c_int;
    }
    if strlen(arg) > 2 as size_t
        && memcmp(
            arg as *const ::core::ffi::c_void,
            b"no-\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            3 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        offset = 3 as ::core::ffi::c_int;
    }
    wt = &raw mut warn_tab as *mut warn_tab as *mut warn_tab;
    while !(*wt).name.is_null() {
        if strcmp(arg.offset(offset as isize), (*wt).name) == 0 as ::core::ffi::c_int {
            if offset != 0 {
                warn_option &= !(*wt).flag as ::core::ffi::c_uint;
            } else {
                warn_option |= (*wt).flag as ::core::ffi::c_uint;
            }
            return 0 as ::core::ffi::c_int;
        }
        wt = wt.offset(1);
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn parse_opt(
    mut key: ::core::ffi::c_int,
    mut arg: *mut ::core::ffi::c_char,
    mut state: *mut argp_state,
) -> error_t {
    match key {
        48 => {
            name_end = '\0' as ::core::ffi::c_char;
        }
        97 => {
            reset_time_flag = r#true;
        }
        65 => {
            append_flag = r#true;
        }
        98 => {
            swap_bytes_flag = r#true;
            swap_halfwords_flag = r#true;
        }
        66 => {
            io_block_size = 5120 as ::core::ffi::c_int;
        }
        266 => {
            io_block_size = atoi(arg);
            if io_block_size < 1 as ::core::ffi::c_int
                || io_block_size > INT_MAX / 512 as ::core::ffi::c_int
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
                            b"invalid block size\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"invalid block size\0".as_ptr() as *const ::core::ffi::c_char,
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
            io_block_size *= 512 as ::core::ffi::c_int;
        }
        99 => {
            if archive_format_0.0 != archive_format::arf_unknown.0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Archive format multiply defined\0".as_ptr()
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
                                b"Archive format multiply defined\0".as_ptr()
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
            archive_format_0 = archive_format::arf_oldascii;
        }
        67 => {
            io_block_size = atoi(arg);
            if io_block_size < 1 as ::core::ffi::c_int {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"invalid block size\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"invalid block size\0".as_ptr() as *const ::core::ffi::c_char,
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
        100 => {
            create_dir_flag = r#true;
        }
        68 => {
            change_directory_option = arg;
        }
        102 => {
            copy_matching_files = r#false;
        }
        69 => {
            pattern_file_name = arg;
        }
        70 => {
            archive_name = arg;
        }
        72 => {
            if archive_format_0.0 != archive_format::arf_unknown.0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Archive format multiply defined\0".as_ptr()
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
                                b"Archive format multiply defined\0".as_ptr()
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
            if strcasecmp(arg, b"crc\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_crcascii;
            } else if strcasecmp(arg, b"newc\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_newascii;
            } else if strcasecmp(arg, b"odc\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_oldascii;
            } else if strcasecmp(arg, b"bin\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_binary;
            } else if strcasecmp(arg, b"ustar\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_ustar;
            } else if strcasecmp(arg, b"tar\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_tar;
            } else if strcasecmp(arg, b"hpodc\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_hpoldascii;
            } else if strcasecmp(arg, b"hpbin\0".as_ptr() as *const ::core::ffi::c_char) == 0 {
                archive_format_0 = archive_format::arf_hpbinary;
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
                            b"invalid archive format `%s'; valid formats are:\ncrc newc odc bin ustar tar (all-caps also recognized)\0"
                                .as_ptr() as *const ::core::ffi::c_char,
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
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"invalid archive format `%s'; valid formats are:\ncrc newc odc bin ustar tar (all-caps also recognized)\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
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
                usage(PAXEXIT_FAILURE);
            }
        }
        105 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Mode already defined\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"Mode already defined\0".as_ptr() as *const ::core::ffi::c_char,
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
            copy_function = Some(process_copy_in as unsafe extern "C" fn() -> ());
        }
        73 => {
            input_archive_name = arg;
        }
        108 => {
            link_flag = r#true;
        }
        76 => {
            xstat = ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
            >(Some(
                stat as unsafe extern "C" fn(
                    *const ::core::ffi::c_char,
                    *mut stat,
                ) -> ::core::ffi::c_int,
            ));
        }
        109 => {
            retain_time_flag = r#true;
        }
        77 => {
            set_new_media_message(arg);
        }
        110 => {
            numeric_uid = r#true;
        }
        256 => {
            no_abs_paths_flag = r#true;
        }
        257 => {
            no_abs_paths_flag = r#false;
        }
        258 => {
            if set_owner_flag != 0 || set_group_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"--no-preserve-owner cannot be used with --owner\0".as_ptr()
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
                                b"--no-preserve-owner cannot be used with --owner\0".as_ptr()
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
            no_chown_flag = r#true;
        }
        111 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Mode already defined\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"Mode already defined\0".as_ptr() as *const ::core::ffi::c_char,
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
            copy_function = Some(process_copy_out as unsafe extern "C" fn() -> ());
        }
        79 => {
            output_archive_name = arg;
        }
        259 => {
            only_verify_crc_flag = r#true;
        }
        112 => {
            if copy_function.is_some() {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Mode already defined\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"Mode already defined\0".as_ptr() as *const ::core::ffi::c_char,
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
            copy_function = Some(process_copy_pass as unsafe extern "C" fn() -> ());
        }
        269 => {
            ignore_devno_option = 1 as ::core::ffi::c_int;
        }
        268 => {
            renumber_inodes_option = 1 as ::core::ffi::c_int;
        }
        270 => {
            ignore_dirnlink_option = 1 as ::core::ffi::c_int;
        }
        271 => {
            ignore_dirnlink_option = 1 as ::core::ffi::c_int;
            renumber_inodes_option = ignore_dirnlink_option;
            ignore_devno_option = renumber_inodes_option;
        }
        261 => {
            rsh_command_option = arg;
        }
        114 => {
            rename_flag = r#true;
        }
        260 => {
            rename_batch_file = arg;
        }
        262 => {
            quiet_flag = r#true;
        }
        82 => {
            if no_chown_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"--owner cannot be used with --no-preserve-owner\0".as_ptr()
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
                                b"--owner cannot be used with --no-preserve-owner\0".as_ptr()
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
            } else {
                let mut e: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
                let mut u: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                let mut g: *mut ::core::ffi::c_char =
                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                e = parse_user_spec(
                    arg,
                    &raw mut set_owner,
                    &raw mut set_group,
                    &raw mut u,
                    &raw mut g,
                );
                if !e.is_null() {
                    if error_hook.is_some() {
                        error_hook.expect("non-null function pointer")();
                    }
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            b"%s: %s\0".as_ptr() as *const ::core::ffi::c_char,
                            arg,
                            e,
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
                                arg,
                                e,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                    usage(PAXEXIT_FAILURE);
                }
                if !u.is_null() {
                    free(u as *mut ::core::ffi::c_void);
                    set_owner_flag = r#true;
                }
                if !g.is_null() {
                    free(g as *mut ::core::ffi::c_void);
                    set_group_flag = r#true;
                }
            }
        }
        115 => {
            swap_bytes_flag = r#true;
        }
        83 => {
            swap_halfwords_flag = r#true;
        }
        116 => {
            table_flag = r#true;
        }
        117 => {
            unconditional_flag = r#true;
        }
        118 => {
            verbose_flag = r#true;
        }
        86 => {
            dot_flag = r#true;
        }
        87 => {
            if warn_control(arg) != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Invalid value for --warning option: %s\0".as_ptr()
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
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"Invalid value for --warning option: %s\0".as_ptr()
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
                usage(PAXEXIT_FAILURE);
            }
        }
        263 => {
            sparse_flag = r#true;
        }
        264 => {
            force_local_option = true;
        }
        267 => {
            to_stdout_option = r#true != 0;
        }
        _ => return ARGP_ERR_UNKNOWN,
    }
    return 0 as error_t;
}
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
        args_doc: b"[destination-directory]\0".as_ptr() as *const ::core::ffi::c_char,
        doc: &raw const doc as *mut ::core::ffi::c_char,
        children: ::core::ptr::null::<argp_child>(),
        help_filter: None,
        argp_domain: ::core::ptr::null::<::core::ffi::c_char>(),
    }
};
unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) {
    argp_help(
        &raw mut argp,
        stderr,
        ARGP_HELP_SEE as ::core::ffi::c_uint,
        program_name as *mut ::core::ffi::c_char,
    );
    close_stdout();
    exit(status);
}
#[export_name = "rboxc_cpio_cpio_process_args"]
pub unsafe extern "C" fn process_args(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut index: ::core::ffi::c_int = 0;
    xstat = ::core::mem::transmute::<
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int>,
        Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    >(Some(
        lstat as unsafe extern "C" fn(*const ::core::ffi::c_char, *mut stat) -> ::core::ffi::c_int,
    ));
    if argp_parse(
        &raw mut argp,
        argc,
        argv,
        ARGP_IN_ORDER as ::core::ffi::c_uint,
        &raw mut index,
        NULL_0,
    ) != 0
    {
        exit(PAXEXIT_FAILURE);
    }
    if copy_function.is_none() {
        if table_flag != 0 {
            copy_function = Some(process_copy_in as unsafe extern "C" fn() -> ());
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
                        b"You must specify one of -oipt options.\0".as_ptr()
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
                            b"You must specify one of -oipt options.\0".as_ptr()
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
    if copy_function == Some(process_copy_in as unsafe extern "C" fn() -> ()) {
        archive_des = 0 as ::core::ffi::c_int;
        if link_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--link\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--link\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if reset_time_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--reset\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--reset\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if xstat
            != ::core::mem::transmute::<
                Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
                >,
                Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
            >(Some(
                lstat
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut stat,
                    ) -> ::core::ffi::c_int,
            ))
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
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--dereference\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--dereference\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if append_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--append\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--append\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if !output_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"-O\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"-O\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if renumber_inodes_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--renumber-inodes\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--renumber-inodes\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if ignore_devno_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--ignore-devno\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--ignore-devno\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--extract\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if to_stdout_option {
            if create_dir_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--make-directories\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"%s is meaningless with %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            b"--make-directories\0".as_ptr() as *const ::core::ffi::c_char,
                            b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                usage(PAXEXIT_FAILURE);
            }
            if rename_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--rename\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"%s is meaningless with %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            b"--rename\0".as_ptr() as *const ::core::ffi::c_char,
                            b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                usage(PAXEXIT_FAILURE);
            }
            if no_chown_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--no-preserve-owner\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"%s is meaningless with %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            b"--no-preserve-owner\0".as_ptr() as *const ::core::ffi::c_char,
                            b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                usage(PAXEXIT_FAILURE);
            }
            if set_owner_flag != 0 || set_group_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--owner\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"%s is meaningless with %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            b"--owner\0".as_ptr() as *const ::core::ffi::c_char,
                            b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                usage(PAXEXIT_FAILURE);
            }
            if retain_time_flag != 0 {
                if error_hook.is_some() {
                    error_hook.expect("non-null function pointer")();
                }
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--preserve-modification-time\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
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
                                b"%s is meaningless with %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            b"--preserve-modification-time\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
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
        if !archive_name.is_null() && !input_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Both -I and -F are used in copy-in mode\0".as_ptr()
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
                            b"Both -I and -F are used in copy-in mode\0".as_ptr()
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
        if archive_format_0.0 == archive_format::arf_crcascii.0 {
            crc_i_flag = r#true;
        }
        num_patterns = argc - index;
        save_patterns = argv.offset(index as isize);
        if !input_archive_name.is_null() {
            archive_name = input_archive_name;
        }
    } else if copy_function == Some(process_copy_out as unsafe extern "C" fn() -> ()) {
        if index != argc {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Too many arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"Too many arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
        archive_des = 1 as ::core::ffi::c_int;
        if create_dir_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--make-directories\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--make-directories\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if rename_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--rename\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--rename\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if table_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--list\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--list\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if unconditional_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--unconditional\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--unconditional\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if link_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--link\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--link\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if sparse_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--sparse\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--sparse\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if retain_time_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--preserve-modification-time\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--preserve-modification-time\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if no_chown_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--no-preserve-owner\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--no-preserve-owner\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if swap_bytes_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--swap-bytes (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--swap-bytes (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if swap_halfwords_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--swap-halfwords (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--swap-halfwords (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if to_stdout_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if append_flag != 0 && !(!archive_name.is_null() || !output_archive_name.is_null()) {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"--append is used but no archive file name is given (use -F or -O options)\0"
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
                            b"--append is used but no archive file name is given (use -F or -O options)\0"
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
        if !rename_batch_file.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--rename-batch-file\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--rename-batch-file\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if !input_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"-I\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--create\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"-I\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--create\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if !archive_name.is_null() && !output_archive_name.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Both -O and -F are used in copy-out mode\0".as_ptr()
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
                            b"Both -O and -F are used in copy-out mode\0".as_ptr()
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
        if archive_format_0.0 == archive_format::arf_unknown.0 {
            archive_format_0 = archive_format::arf_binary;
        }
        if !output_archive_name.is_null() {
            archive_name = output_archive_name;
        }
        if arf_stores_inode_p(archive_format_0) == 0 {
            ignore_devno_option = 0 as ::core::ffi::c_int;
            renumber_inodes_option = ignore_devno_option;
        }
    } else {
        if index < argc - 1 as ::core::ffi::c_int {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Too many arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"Too many arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
        } else if index > argc - 1 as ::core::ffi::c_int {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Not enough arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"Not enough arguments\0".as_ptr() as *const ::core::ffi::c_char,
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
        if archive_format_0.0 != archive_format::arf_unknown.0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Archive format is not specified in copy-pass mode (use --format option)\0"
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
                            b"Archive format is not specified in copy-pass mode (use --format option)\0"
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
        if swap_bytes_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--swap-bytes (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--swap-bytes (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if swap_halfwords_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--swap-halfwords (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--swap-halfwords (--swap)\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if table_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--list\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--list\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if rename_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--rename\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--rename\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if append_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--append\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--append\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if !rename_batch_file.is_null() {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--rename-batch-file\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--rename-batch-file\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if no_abs_paths_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--no-absolute-pathnames\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--no-absolute-pathnames\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if no_abs_paths_flag != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--absolute-pathnames\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--absolute-pathnames\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if to_stdout_option {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--to-stdout\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if renumber_inodes_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--renumber-inodes\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--renumber-inodes\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        if ignore_devno_option != 0 {
            if error_hook.is_some() {
                error_hook.expect("non-null function pointer")();
            }
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    b"--ignore-devno\0".as_ptr() as *const ::core::ffi::c_char,
                    b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
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
                            b"%s is meaningless with %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        b"--ignore-devno\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--pass-through\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
            usage(PAXEXIT_FAILURE);
        }
        directory_name = *argv.offset(index as isize);
    }
    if !archive_name.is_null() {
        if copy_function != Some(process_copy_in as unsafe extern "C" fn() -> ())
            && copy_function != Some(process_copy_out as unsafe extern "C" fn() -> ())
        {
            if 0 != 0 {
                error(
                    2 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"-F can be used only with --create or --extract\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"-F can be used only with --create or --extract\0".as_ptr()
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
        archive_des = open_archive(archive_name);
        if archive_des < 0 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    2 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot open %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_colon(archive_name),
                );
                if 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot open %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_colon(archive_name),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if set_owner_flag == r#false && set_group_flag == r#false && geteuid() != 0 {
        no_chown_flag = r#true;
    }
}
#[export_name = "rboxc_cpio_cpio_initialize_buffers"]
pub unsafe extern "C" fn initialize_buffers() {
    let mut in_buf_size: ::core::ffi::c_int = 0;
    let mut out_buf_size: ::core::ffi::c_int = 0;
    if copy_function == Some(process_copy_in as unsafe extern "C" fn() -> ()) {
        if io_block_size >= 512 as ::core::ffi::c_int {
            in_buf_size = 2 as ::core::ffi::c_int * io_block_size;
        } else {
            in_buf_size = 1024 as ::core::ffi::c_int;
        }
        out_buf_size = DISK_IO_BLOCK_SIZE;
    } else if copy_function == Some(process_copy_out as unsafe extern "C" fn() -> ()) {
        in_buf_size = DISK_IO_BLOCK_SIZE;
        out_buf_size = io_block_size;
    } else {
        in_buf_size = DISK_IO_BLOCK_SIZE;
        out_buf_size = DISK_IO_BLOCK_SIZE;
    }
    input_buffer = xmalloc(in_buf_size as size_t) as *mut ::core::ffi::c_char;
    in_buff = input_buffer;
    input_buffer_size = in_buf_size as size_t;
    input_size = 0 as size_t;
    input_bytes = 0 as off_t;
    output_buffer = xmalloc(out_buf_size as size_t) as *mut ::core::ffi::c_char;
    out_buff = output_buffer;
    output_size = 0 as size_t;
    output_bytes = 0 as off_t;
}
unsafe extern "C" fn rboxc_cpio_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    set_program_name(*argv.offset(0isize));
    argp_version_setup(
        b"cpio\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut program_authors as *mut *const ::core::ffi::c_char,
    );
    process_args(argc, argv);
    initialize_buffers();
    ::core::mem::transmute::<_, fn()>(
        Some(copy_function.expect("non-null function pointer")).expect("non-null function pointer"),
    )();
    if archive_des >= 0 as ::core::ffi::c_int
        && if archive_des >= __REM_BIAS {
            rmt_close__(archive_des - __REM_BIAS)
        } else {
            close(archive_des)
        } == -1 as ::core::ffi::c_int
    {
        if 0 != 0 {
            error(
                2 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"error closing archive\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
            );
            if 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"error closing archive\0".as_ptr() as *const ::core::ffi::c_char,
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
    pax_exit();
    panic!("Reached end of non-void function without returning");
}
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

include!("../bridges/cpio-invocation.rs");
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_cpio(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rboxc_cpio_setup(argv);
    rboxc_cpio_main_inner(argc, argv)
}
