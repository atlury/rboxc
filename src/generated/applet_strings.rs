// Generated from pinned GNU strings 2.47 by scripts/translate-entry-provider.py.
// Source SHA-256: af7fa2fbda1a3c2384a583e65a8696b345878111ee996acf66274826b46bc764
/* strings -- print the strings of printable characters in files
   Copyright (C) 1993-2026 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA
   02110-1301, USA.  */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_sym_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_pef_xlib_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_pef_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct plugin_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct mach_o_fat_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct mach_o_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct netbsd_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct cisco_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct osf_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct lynx_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct sgi_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct hppabsd_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct hpux_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct som_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct trad_core_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct mmo_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct elf_obj_tdata { _opaque: [u8; 0] }
#[repr(C)]
pub struct tekhex_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct ihex_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct verilog_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct srec_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct ecoff_tdata { _opaque: [u8; 0] }
#[repr(C)]
pub struct xcoff_tdata { _opaque: [u8; 0] }
#[repr(C)]
pub struct pe_tdata { _opaque: [u8; 0] }
#[repr(C)]
pub struct coff_tdata { _opaque: [u8; 0] }
#[repr(C)]
pub struct artdata { _opaque: [u8; 0] }
#[repr(C)]
pub struct aout_data_struct { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_link_hash_table { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_link_order { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_iovec { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_link_info { _opaque: [u8; 0] }
#[repr(C)]
pub struct bfd_link_hash_entry { _opaque: [u8; 0] }
#[repr(C)]
pub struct flag_info { _opaque: [u8; 0] }
#[repr(C)]
pub struct orl { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
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
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn __uflow(_: *mut FILE) -> ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
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
    #[link_name = "rboxc_strings_bfd_malloc_and_get_section"]
    fn bfd_malloc_and_get_section(
        abfd: *mut bfd,
        section: *mut asection,
        buf: *mut *mut bfd_byte,
    ) -> bool;
    #[link_name = "rboxc_strings_bfd_get_error"]
    fn bfd_get_error() -> bfd_error_type;
    #[link_name = "rboxc_strings_bfd_errmsg"]
    fn bfd_errmsg(error_tag: bfd_error_type) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_strings_bfd_set_error_program_name"]
    fn bfd_set_error_program_name(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_strings_bfd_init"]
    fn bfd_init() -> ::core::ffi::c_uint;
    #[link_name = "rboxc_strings_bfd_check_format"]
    fn bfd_check_format(abfd: *mut bfd, format: bfd_format) -> bool;
    #[link_name = "rboxc_strings_bfd_openr"]
    fn bfd_openr(
        filename: *const ::core::ffi::c_char,
        target_0: *const ::core::ffi::c_char,
    ) -> *mut bfd;
    #[link_name = "rboxc_strings_bfd_close"]
    fn bfd_close(abfd: *mut bfd) -> bool;
    fn getopt_long(
        argc: ::core::ffi::c_int,
        argv: *const *mut ::core::ffi::c_char,
        shortopts: *const ::core::ffi::c_char,
        longopts: *const option,
        longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_strings_expandargv"]
    fn expandargv(_: *mut ::core::ffi::c_int, _: *mut *mut *mut ::core::ffi::c_char);
    #[link_name = "rboxc_strings_xmalloc_set_program_name"]
    fn xmalloc_set_program_name(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_strings_xmalloc"]
    fn xmalloc(_: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_strings__sch_istable"]
    static _sch_istable: [::core::ffi::c_ushort; 256];
    #[link_name = "rboxc_strings_bfd_nonfatal"]
    fn bfd_nonfatal(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_strings_fatal"]
    fn fatal(_: *const ::core::ffi::c_char, ...) -> !;
    #[link_name = "rboxc_strings_non_fatal"]
    fn non_fatal(_: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_strings_set_default_bfd_target"]
    fn set_default_bfd_target();
    #[link_name = "rboxc_strings_list_supported_targets"]
    fn list_supported_targets(_: *const ::core::ffi::c_char, _: *mut FILE);
    #[link_name = "rboxc_strings_program_name"]
    static mut program_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_strings_print_version"]
    fn print_version(_: *const ::core::ffi::c_char);
}
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
pub type int64_t = i64;
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
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type bfd_vma = uint64_t;
pub type bfd_signed_vma = int64_t;
pub type bfd_size_type = uint64_t;
pub type symvalue = uint64_t;
pub type file_ptr = int64_t;
pub type ufile_ptr = uint64_t;
pub type flagword = uint32_t;
pub type bfd_byte = uint8_t;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct bfd {
    pub filename: *const ::core::ffi::c_char,
    pub xvec: *const bfd_target,
    pub iostream: *mut ::core::ffi::c_void,
    pub iovec: *const bfd_iovec,
    pub lru_prev: *mut bfd,
    pub lru_next: *mut bfd,
    pub r#where: ufile_ptr,
    pub mtime: ::core::ffi::c_long,
    pub id: ::core::ffi::c_uint,
    pub flags: flagword,
    #[bitfield(name = "format", ty = "bfd_format", bits = "0..=2")]
    #[bitfield(name = "direction", ty = "bfd_direction", bits = "3..=4")]
    #[bitfield(name = "last_io", ty = "bfd_last_io", bits = "5..=6")]
    #[bitfield(name = "cacheable", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "target_defaulted", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "opened_once", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(name = "mtime_set", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "no_export", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(
        name = "output_has_begun",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(name = "has_armap", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "link_mapless", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "is_thin_archive", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_fake_archive", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(
        name = "no_element_cache",
        ty = "::core::ffi::c_uint",
        bits = "17..=17"
    )]
    #[bitfield(
        name = "selective_search",
        ty = "::core::ffi::c_uint",
        bits = "18..=18"
    )]
    #[bitfield(
        name = "is_linker_output",
        ty = "::core::ffi::c_uint",
        bits = "19..=19"
    )]
    #[bitfield(name = "is_linker_input", ty = "::core::ffi::c_uint", bits = "20..=20")]
    #[bitfield(name = "plugin_format", ty = "bfd_plugin_format", bits = "21..=22")]
    #[bitfield(name = "lto_output", ty = "::core::ffi::c_uint", bits = "23..=23")]
    #[bitfield(name = "read_only", ty = "::core::ffi::c_uint", bits = "24..=24")]
    #[bitfield(name = "lto_type", ty = "bfd_lto_object_type", bits = "25..=27")]
    #[bitfield(
        name = "in_format_matches",
        ty = "::core::ffi::c_uint",
        bits = "28..=28"
    )]
    pub format_direction_last_io_cacheable_target_defaulted_opened_once_mtime_set_no_export_output_has_begun_has_armap_link_mapless_is_thin_archive_is_fake_archive_no_element_cache_selective_search_is_linker_output_is_linker_input_plugin_format_lto_output_read_only_lto_type_in_format_matches:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub plugin_dummy_bfd: *mut bfd,
    pub origin: ufile_ptr,
    pub proxy_handle: ufile_ptr_or_bfd,
    pub section_htab: bfd_hash_table,
    pub sections: *mut bfd_section,
    pub section_last: *mut bfd_section,
    pub object_only_section: *mut bfd_section,
    pub section_count: ::core::ffi::c_uint,
    pub archive_plugin_fd: ::core::ffi::c_int,
    pub archive_plugin_fd_open_count: ::core::ffi::c_uint,
    pub archive_pass: ::core::ffi::c_int,
    pub alloc_size: bfd_size_type,
    pub start_address: bfd_vma,
    pub outsymbols: *mut *mut bfd_symbol,
    pub symcount: ::core::ffi::c_uint,
    pub dynsymcount: ::core::ffi::c_uint,
    pub arch_info: *const bfd_arch_info,
    pub size: ufile_ptr,
    pub arelt_data: *mut ::core::ffi::c_void,
    pub my_archive: *mut bfd,
    pub archive_next: *mut bfd,
    pub archive_head: *mut bfd,
    pub nested_archives: *mut bfd,
    pub link: C2Rust_Unnamed_0,
    pub tdata: C2Rust_Unnamed,
    pub usrdata: *mut ::core::ffi::c_void,
    pub memory: *mut ::core::ffi::c_void,
    pub build_id: *const bfd_build_id,
    pub mmapped: *mut bfd_mmapped,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_mmapped {
    pub next: *mut bfd_mmapped,
    pub max_entry: ::core::ffi::c_uint,
    pub next_entry: ::core::ffi::c_uint,
    pub entries: [bfd_mmapped_entry; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_mmapped_entry {
    pub addr: *mut ::core::ffi::c_void,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_build_id {
    pub size: bfd_size_type,
    pub data: [bfd_byte; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub aout_data: *mut aout_data_struct,
    pub aout_ar_data: *mut artdata,
    pub coff_obj_data: *mut coff_tdata,
    pub pe_obj_data: *mut pe_tdata,
    pub xcoff_obj_data: *mut xcoff_tdata,
    pub ecoff_obj_data: *mut ecoff_tdata,
    pub srec_data: *mut srec_data_struct,
    pub verilog_data: *mut verilog_data_struct,
    pub ihex_data: *mut ihex_data_struct,
    pub tekhex_data: *mut tekhex_data_struct,
    pub elf_obj_data: *mut elf_obj_tdata,
    pub mmo_data: *mut mmo_data_struct,
    pub trad_core_data: *mut trad_core_struct,
    pub som_data: *mut som_data_struct,
    pub hpux_core_data: *mut hpux_core_struct,
    pub hppabsd_core_data: *mut hppabsd_core_struct,
    pub sgi_core_data: *mut sgi_core_struct,
    pub lynx_core_data: *mut lynx_core_struct,
    pub osf_core_data: *mut osf_core_struct,
    pub cisco_core_data: *mut cisco_core_struct,
    pub netbsd_core_data: *mut netbsd_core_struct,
    pub mach_o_data: *mut mach_o_data_struct,
    pub mach_o_fat_data: *mut mach_o_fat_data_struct,
    pub plugin_data: *mut plugin_data_struct,
    pub pef_data: *mut bfd_pef_data_struct,
    pub pef_xlib_data: *mut bfd_pef_xlib_data_struct,
    pub sym_data: *mut bfd_sym_data_struct,
    pub any: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub next: *mut bfd,
    pub hash: *mut bfd_link_hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_arch_info {
    pub bits_per_word: ::core::ffi::c_int,
    pub bits_per_address: ::core::ffi::c_int,
    pub bits_per_byte: ::core::ffi::c_int,
    pub arch: bfd_architecture,
    pub mach: ::core::ffi::c_ulong,
    pub arch_name: *const ::core::ffi::c_char,
    pub printable_name: *const ::core::ffi::c_char,
    pub section_align_power: ::core::ffi::c_uint,
    pub the_default: bool,
    pub compatible: Option<
        unsafe extern "C" fn(*const bfd_arch_info, *const bfd_arch_info) -> *const bfd_arch_info,
    >,
    pub scan:
        Option<unsafe extern "C" fn(*const bfd_arch_info, *const ::core::ffi::c_char) -> bool>,
    pub fill: Option<unsafe extern "C" fn(bfd_size_type, bool, bool) -> *mut ::core::ffi::c_void>,
    pub next: *const bfd_arch_info,
    pub max_reloc_offset_into_insn: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_architecture(pub ::core::ffi::c_uint);
impl bfd_architecture {
    pub const bfd_arch_unknown: Self = Self(0);
    pub const bfd_arch_obscure: Self = Self(1);
    pub const bfd_arch_m68k: Self = Self(2);
    pub const bfd_arch_vax: Self = Self(3);
    pub const bfd_arch_or1k: Self = Self(4);
    pub const bfd_arch_sparc: Self = Self(5);
    pub const bfd_arch_spu: Self = Self(6);
    pub const bfd_arch_mips: Self = Self(7);
    pub const bfd_arch_i386: Self = Self(8);
    pub const bfd_arch_iamcu: Self = Self(9);
    pub const bfd_arch_romp: Self = Self(10);
    pub const bfd_arch_convex: Self = Self(11);
    pub const bfd_arch_m98k: Self = Self(12);
    pub const bfd_arch_pyramid: Self = Self(13);
    pub const bfd_arch_h8300: Self = Self(14);
    pub const bfd_arch_pdp11: Self = Self(15);
    pub const bfd_arch_powerpc: Self = Self(16);
    pub const bfd_arch_rs6000: Self = Self(17);
    pub const bfd_arch_hppa: Self = Self(18);
    pub const bfd_arch_d10v: Self = Self(19);
    pub const bfd_arch_d30v: Self = Self(20);
    pub const bfd_arch_dlx: Self = Self(21);
    pub const bfd_arch_m68hc11: Self = Self(22);
    pub const bfd_arch_m68hc12: Self = Self(23);
    pub const bfd_arch_m9s12x: Self = Self(24);
    pub const bfd_arch_m9s12xg: Self = Self(25);
    pub const bfd_arch_s12z: Self = Self(26);
    pub const bfd_arch_z8k: Self = Self(27);
    pub const bfd_arch_sh: Self = Self(28);
    pub const bfd_arch_alpha: Self = Self(29);
    pub const bfd_arch_arm: Self = Self(30);
    pub const bfd_arch_nds32: Self = Self(31);
    pub const bfd_arch_ns32k: Self = Self(32);
    pub const bfd_arch_tic30: Self = Self(33);
    pub const bfd_arch_tic4x: Self = Self(34);
    pub const bfd_arch_tic54x: Self = Self(35);
    pub const bfd_arch_tic6x: Self = Self(36);
    pub const bfd_arch_v850: Self = Self(37);
    pub const bfd_arch_v850_rh850: Self = Self(38);
    pub const bfd_arch_arc: Self = Self(39);
    pub const bfd_arch_m32c: Self = Self(40);
    pub const bfd_arch_m32r: Self = Self(41);
    pub const bfd_arch_mn10200: Self = Self(42);
    pub const bfd_arch_mn10300: Self = Self(43);
    pub const bfd_arch_fr30: Self = Self(44);
    pub const bfd_arch_frv: Self = Self(45);
    pub const bfd_arch_moxie: Self = Self(46);
    pub const bfd_arch_ft32: Self = Self(47);
    pub const bfd_arch_mcore: Self = Self(48);
    pub const bfd_arch_mep: Self = Self(49);
    pub const bfd_arch_metag: Self = Self(50);
    pub const bfd_arch_ia64: Self = Self(51);
    pub const bfd_arch_ip2k: Self = Self(52);
    pub const bfd_arch_iq2000: Self = Self(53);
    pub const bfd_arch_bpf: Self = Self(54);
    pub const bfd_arch_epiphany: Self = Self(55);
    pub const bfd_arch_mt: Self = Self(56);
    pub const bfd_arch_pj: Self = Self(57);
    pub const bfd_arch_avr: Self = Self(58);
    pub const bfd_arch_bfin: Self = Self(59);
    pub const bfd_arch_cr16: Self = Self(60);
    pub const bfd_arch_crx: Self = Self(61);
    pub const bfd_arch_cris: Self = Self(62);
    pub const bfd_arch_riscv: Self = Self(63);
    pub const bfd_arch_rl78: Self = Self(64);
    pub const bfd_arch_rx: Self = Self(65);
    pub const bfd_arch_s390: Self = Self(66);
    pub const bfd_arch_score: Self = Self(67);
    pub const bfd_arch_mmix: Self = Self(68);
    pub const bfd_arch_xstormy16: Self = Self(69);
    pub const bfd_arch_msp430: Self = Self(70);
    pub const bfd_arch_xgate: Self = Self(71);
    pub const bfd_arch_xtensa: Self = Self(72);
    pub const bfd_arch_z80: Self = Self(73);
    pub const bfd_arch_lm32: Self = Self(74);
    pub const bfd_arch_microblaze: Self = Self(75);
    pub const bfd_arch_kvx: Self = Self(76);
    pub const bfd_arch_tilepro: Self = Self(77);
    pub const bfd_arch_tilegx: Self = Self(78);
    pub const bfd_arch_aarch64: Self = Self(79);
    pub const bfd_arch_visium: Self = Self(80);
    pub const bfd_arch_wasm32: Self = Self(81);
    pub const bfd_arch_pru: Self = Self(82);
    pub const bfd_arch_nfp: Self = Self(83);
    pub const bfd_arch_csky: Self = Self(84);
    pub const bfd_arch_loongarch: Self = Self(85);
    pub const bfd_arch_amdgcn: Self = Self(86);
    pub const bfd_arch_last: Self = Self(87);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_symbol {
    pub the_bfd: *mut bfd,
    pub name: *const ::core::ffi::c_char,
    pub value: symvalue,
    pub flags: flagword,
    pub section: *mut bfd_section,
    pub udata: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub p: *mut ::core::ffi::c_void,
    pub i: bfd_vma,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct bfd_section {
    pub name: *const ::core::ffi::c_char,
    pub next: *mut bfd_section,
    pub prev: *mut bfd_section,
    pub id: ::core::ffi::c_uint,
    pub section_id: ::core::ffi::c_uint,
    pub index: ::core::ffi::c_uint,
    pub flags: flagword,
    pub vma: bfd_vma,
    pub lma: bfd_vma,
    pub size: bfd_size_type,
    pub rawsize: bfd_size_type,
    pub compressed_size: bfd_size_type,
    pub output_offset: bfd_vma,
    pub output_section: *mut bfd_section,
    pub relocation: *mut reloc_cache_entry,
    pub orelocation: *mut *mut reloc_cache_entry,
    pub reloc_count: ::core::ffi::c_uint,
    pub alignment_power: ::core::ffi::c_uint,
    pub filepos: file_ptr,
    pub rel_filepos: file_ptr,
    pub line_filepos: file_ptr,
    pub userdata: *mut ::core::ffi::c_void,
    pub contents: *mut bfd_byte,
    pub lineno: *mut alent,
    pub lineno_count: ::core::ffi::c_uint,
    pub entsize: ::core::ffi::c_uint,
    pub kept_section: *mut bfd_section,
    pub moving_line_filepos: file_ptr,
    pub used_by_bfd: *mut ::core::ffi::c_void,
    pub constructor_chain: *mut relent_chain,
    pub owner: *mut bfd,
    pub symbol: *mut bfd_symbol,
    pub map_head: C2Rust_Unnamed_2,
    pub map_tail: C2Rust_Unnamed_2,
    pub already_assigned: *mut bfd_section,
    pub sec_info: *mut ::core::ffi::c_void,
    pub r#type: ::core::ffi::c_uint,
    pub target_index: ::core::ffi::c_int,
    #[bitfield(name = "user_set_vma", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "linker_mark", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "linker_has_input", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "gc_mark", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "compress_status", ty = "::core::ffi::c_uint", bits = "4..=5")]
    #[bitfield(name = "segment_mark", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "sec_info_type", ty = "::core::ffi::c_uint", bits = "7..=9")]
    #[bitfield(name = "use_rela_p", ty = "::core::ffi::c_uint", bits = "10..=10")]
    #[bitfield(name = "mmapped_p", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(name = "alloced", ty = "::core::ffi::c_uint", bits = "12..=12")]
    #[bitfield(name = "veneer", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "sec_flg0", ty = "::core::ffi::c_uint", bits = "14..=14")]
    #[bitfield(name = "sec_flg1", ty = "::core::ffi::c_uint", bits = "15..=15")]
    #[bitfield(name = "sec_flg2", ty = "::core::ffi::c_uint", bits = "16..=16")]
    #[bitfield(name = "sec_flg3", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "sec_flg4", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(name = "sec_flg5", ty = "::core::ffi::c_uint", bits = "19..=19")]
    pub user_set_vma_linker_mark_linker_has_input_gc_mark_compress_status_segment_mark_sec_info_type_use_rela_p_mmapped_p_alloced_veneer_sec_flg0_sec_flg1_sec_flg2_sec_flg3_sec_flg4_sec_flg5:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 5],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub link_order: *mut bfd_link_order,
    pub s: *mut bfd_section,
    pub linked_to_symbol_name: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct relent_chain {
    pub relent: arelent,
    pub next: *mut relent_chain,
}
pub type arelent = reloc_cache_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct reloc_cache_entry {
    pub sym_ptr_ptr: *mut *mut bfd_symbol,
    pub address: bfd_size_type,
    pub addend: bfd_vma,
    pub howto: *const reloc_howto_type,
}
pub type reloc_howto_type = reloc_howto_struct;
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct reloc_howto_struct {
    pub r#type: ::core::ffi::c_uint,
    #[bitfield(name = "size", ty = "::core::ffi::c_uint", bits = "0..=3")]
    #[bitfield(name = "bitsize", ty = "::core::ffi::c_uint", bits = "4..=10")]
    #[bitfield(name = "rightshift", ty = "::core::ffi::c_uint", bits = "11..=16")]
    #[bitfield(name = "bitpos", ty = "::core::ffi::c_uint", bits = "17..=22")]
    #[bitfield(
        name = "complain_on_overflow",
        ty = "complain_overflow",
        bits = "23..=24"
    )]
    #[bitfield(name = "negate", ty = "::core::ffi::c_uint", bits = "25..=25")]
    #[bitfield(name = "pc_relative", ty = "::core::ffi::c_uint", bits = "26..=26")]
    #[bitfield(name = "partial_inplace", ty = "::core::ffi::c_uint", bits = "27..=27")]
    #[bitfield(name = "pcrel_offset", ty = "::core::ffi::c_uint", bits = "28..=28")]
    #[bitfield(name = "install_addend", ty = "::core::ffi::c_uint", bits = "29..=29")]
    pub size_bitsize_rightshift_bitpos_complain_on_overflow_negate_pc_relative_partial_inplace_pcrel_offset_install_addend:
        [u8; 4],
    pub src_mask: bfd_vma,
    pub dst_mask: bfd_vma,
    pub special_function: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut arelent,
            *mut bfd_symbol,
            *mut ::core::ffi::c_void,
            *mut asection,
            *mut bfd,
            *mut *mut ::core::ffi::c_char,
        ) -> bfd_reloc_status_type,
    >,
    pub name: *const ::core::ffi::c_char,
}
pub type asection = bfd_section;
pub type bfd_reloc_status_type = bfd_reloc_status;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_reloc_status(pub ::core::ffi::c_uint);
impl bfd_reloc_status {
    pub const bfd_reloc_ok: Self = Self(2);
    pub const bfd_reloc_overflow: Self = Self(3);
    pub const bfd_reloc_outofrange: Self = Self(4);
    pub const bfd_reloc_continue: Self = Self(5);
    pub const bfd_reloc_notsupported: Self = Self(6);
    pub const bfd_reloc_other: Self = Self(7);
    pub const bfd_reloc_undefined: Self = Self(8);
    pub const bfd_reloc_dangerous: Self = Self(9);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct complain_overflow(pub ::core::ffi::c_uint);
impl complain_overflow {
    pub const complain_overflow_dont: Self = Self(0);
    pub const complain_overflow_bitfield: Self = Self(1);
    pub const complain_overflow_signed: Self = Self(2);
    pub const complain_overflow_unsigned: Self = Self(3);
}
pub type alent = lineno_cache_entry;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lineno_cache_entry {
    pub line_number: ::core::ffi::c_uint,
    pub u: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub sym: *mut bfd_symbol,
    pub offset: bfd_vma,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct bfd_hash_table {
    pub table: *mut *mut bfd_hash_entry,
    pub newfunc: Option<
        unsafe extern "C" fn(
            *mut bfd_hash_entry,
            *mut bfd_hash_table,
            *const ::core::ffi::c_char,
        ) -> *mut bfd_hash_entry,
    >,
    pub memory: *mut ::core::ffi::c_void,
    pub size: ::core::ffi::c_uint,
    pub count: ::core::ffi::c_uint,
    pub entsize: ::core::ffi::c_uint,
    #[bitfield(name = "frozen", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub frozen: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_hash_entry {
    pub next: *mut bfd_hash_entry,
    pub string: *const ::core::ffi::c_char,
    pub hash: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ufile_ptr_or_bfd {
    pub file_offset: ufile_ptr,
    pub abfd: *mut bfd,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_lto_object_type(pub ::core::ffi::c_uint);
impl bfd_lto_object_type {
    pub const lto_non_object: Self = Self(0);
    pub const lto_non_ir_object: Self = Self(1);
    pub const lto_slim_ir_object: Self = Self(2);
    pub const lto_fat_ir_object: Self = Self(3);
    pub const lto_mixed_object: Self = Self(4);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_plugin_format(pub ::core::ffi::c_uint);
impl bfd_plugin_format {
    pub const bfd_plugin_unknown: Self = Self(0);
    pub const bfd_plugin_yes: Self = Self(1);
    pub const bfd_plugin_yes_unused: Self = Self(2);
    pub const bfd_plugin_no: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_last_io(pub ::core::ffi::c_uint);
impl bfd_last_io {
    pub const bfd_io_seek: Self = Self(0);
    pub const bfd_io_read: Self = Self(1);
    pub const bfd_io_write: Self = Self(2);
    pub const bfd_io_force: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_direction(pub ::core::ffi::c_uint);
impl bfd_direction {
    pub const no_direction: Self = Self(0);
    pub const read_direction: Self = Self(1);
    pub const write_direction: Self = Self(2);
    pub const both_direction: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_format(pub ::core::ffi::c_uint);
impl bfd_format {
    pub const bfd_unknown: Self = Self(0);
    pub const bfd_object: Self = Self(1);
    pub const bfd_archive: Self = Self(2);
    pub const bfd_core: Self = Self(3);
    pub const bfd_type_end: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bfd_target {
    pub name: *const ::core::ffi::c_char,
    pub flavour: bfd_flavour,
    pub byteorder: bfd_endian,
    pub header_byteorder: bfd_endian,
    pub object_flags: flagword,
    pub section_flags: flagword,
    pub symbol_leading_char: ::core::ffi::c_char,
    pub ar_pad_char: ::core::ffi::c_char,
    pub ar_max_namelen: ::core::ffi::c_uchar,
    pub match_priority: ::core::ffi::c_uchar,
    pub keep_unused_section_symbols: bool,
    pub merge_sections: bool,
    pub bfd_getx64: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
    pub bfd_getx_signed_64: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> int64_t>,
    pub bfd_putx64: Option<unsafe extern "C" fn(uint64_t, *mut ::core::ffi::c_void) -> ()>,
    pub bfd_getx32: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_vma>,
    pub bfd_getx_signed_32:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_signed_vma>,
    pub bfd_putx32: Option<unsafe extern "C" fn(bfd_vma, *mut ::core::ffi::c_void) -> ()>,
    pub bfd_getx16: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_vma>,
    pub bfd_getx_signed_16:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_signed_vma>,
    pub bfd_putx16: Option<unsafe extern "C" fn(bfd_vma, *mut ::core::ffi::c_void) -> ()>,
    pub bfd_h_getx64: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
    pub bfd_h_getx_signed_64: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> int64_t>,
    pub bfd_h_putx64: Option<unsafe extern "C" fn(uint64_t, *mut ::core::ffi::c_void) -> ()>,
    pub bfd_h_getx32: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_vma>,
    pub bfd_h_getx_signed_32:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_signed_vma>,
    pub bfd_h_putx32: Option<unsafe extern "C" fn(bfd_vma, *mut ::core::ffi::c_void) -> ()>,
    pub bfd_h_getx16: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_vma>,
    pub bfd_h_getx_signed_16:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> bfd_signed_vma>,
    pub bfd_h_putx16: Option<unsafe extern "C" fn(bfd_vma, *mut ::core::ffi::c_void) -> ()>,
    pub _bfd_check_format: [Option<unsafe extern "C" fn(*mut bfd) -> bfd_cleanup>; 4],
    pub _bfd_set_format: [Option<unsafe extern "C" fn(*mut bfd) -> bool>; 4],
    pub _bfd_write_contents: [Option<unsafe extern "C" fn(*mut bfd) -> bool>; 4],
    pub _close_and_cleanup: Option<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_free_cached_info: Option<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _new_section_hook: Option<unsafe extern "C" fn(*mut bfd, sec_ptr) -> bool>,
    pub _bfd_get_section_contents: Option<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *mut ::core::ffi::c_void,
            file_ptr,
            bfd_size_type,
        ) -> bool,
    >,
    pub _bfd_copy_private_bfd_data: Option<unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool>,
    pub _bfd_merge_private_bfd_data:
        Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool>,
    pub _bfd_copy_private_section_data: Option<
        unsafe extern "C" fn(*mut bfd, sec_ptr, *mut bfd, sec_ptr, *mut bfd_link_info) -> bool,
    >,
    pub _bfd_copy_private_symbol_data: Option<
        unsafe extern "C" fn(*mut bfd, *mut *mut asymbol, *mut bfd, *mut *mut asymbol) -> bool,
    >,
    pub _bfd_copy_private_header_data: Option<unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool>,
    pub _bfd_set_private_flags: Option<unsafe extern "C" fn(*mut bfd, flagword) -> bool>,
    pub _bfd_print_private_bfd_data:
        Option<unsafe extern "C" fn(*mut bfd, *mut ::core::ffi::c_void) -> bool>,
    pub _core_file_failing_command:
        Option<unsafe extern "C" fn(*mut bfd) -> *mut ::core::ffi::c_char>,
    pub _core_file_failing_signal: Option<unsafe extern "C" fn(*mut bfd) -> ::core::ffi::c_int>,
    pub _core_file_matches_executable_p: Option<unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool>,
    pub _core_file_pid: Option<unsafe extern "C" fn(*mut bfd) -> ::core::ffi::c_int>,
    pub _bfd_slurp_armap: Option<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_slurp_extended_name_table: Option<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_construct_extended_name_table: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut ::core::ffi::c_char,
            *mut bfd_size_type,
            *mut *const ::core::ffi::c_char,
        ) -> bool,
    >,
    pub _bfd_truncate_arname: Option<
        unsafe extern "C" fn(*mut bfd, *const ::core::ffi::c_char, *mut ::core::ffi::c_char) -> (),
    >,
    pub write_armap: Option<
        unsafe extern "C" fn(
            *mut bfd,
            ::core::ffi::c_uint,
            *mut orl,
            ::core::ffi::c_uint,
            ::core::ffi::c_int,
        ) -> bool,
    >,
    pub _bfd_read_ar_hdr_fn: Option<unsafe extern "C" fn(*mut bfd) -> *mut ::core::ffi::c_void>,
    pub _bfd_write_ar_hdr_fn: Option<unsafe extern "C" fn(*mut bfd, *mut bfd) -> bool>,
    pub openr_next_archived_file: Option<unsafe extern "C" fn(*mut bfd, *mut bfd) -> *mut bfd>,
    pub _bfd_get_elt_at_index: Option<unsafe extern "C" fn(*mut bfd, symindex) -> *mut bfd>,
    pub _bfd_stat_arch_elt: Option<unsafe extern "C" fn(*mut bfd, *mut stat) -> ::core::ffi::c_int>,
    pub _bfd_update_armap_timestamp: Option<unsafe extern "C" fn(*mut bfd) -> bool>,
    pub _bfd_get_symtab_upper_bound: Option<unsafe extern "C" fn(*mut bfd) -> ::core::ffi::c_long>,
    pub _bfd_canonicalize_symtab:
        Option<unsafe extern "C" fn(*mut bfd, *mut *mut bfd_symbol) -> ::core::ffi::c_long>,
    pub _bfd_make_empty_symbol: Option<unsafe extern "C" fn(*mut bfd) -> *mut bfd_symbol>,
    pub _bfd_print_symbol: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut ::core::ffi::c_void,
            *mut bfd_symbol,
            bfd_print_symbol_type,
        ) -> (),
    >,
    pub _bfd_get_symbol_info:
        Option<unsafe extern "C" fn(*mut bfd, *mut bfd_symbol, *mut symbol_info) -> ()>,
    pub _bfd_get_symbol_version_string: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_symbol,
            bool,
            *mut bool,
        ) -> *const ::core::ffi::c_char,
    >,
    pub _bfd_is_local_label_name:
        Option<unsafe extern "C" fn(*mut bfd, *const ::core::ffi::c_char) -> bool>,
    pub _bfd_is_target_special_symbol: Option<unsafe extern "C" fn(*mut bfd, *mut asymbol) -> bool>,
    pub _get_lineno: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_symbol) -> *mut alent>,
    pub _bfd_find_nearest_line: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut bfd_symbol,
            *mut bfd_section,
            bfd_vma,
            *mut *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_uint,
            *mut ::core::ffi::c_uint,
        ) -> bool,
    >,
    pub _bfd_find_nearest_line_with_alt: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *const ::core::ffi::c_char,
            *mut *mut bfd_symbol,
            *mut bfd_section,
            bfd_vma,
            *mut *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_uint,
            *mut ::core::ffi::c_uint,
        ) -> bool,
    >,
    pub _bfd_find_line: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut bfd_symbol,
            *mut bfd_symbol,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_uint,
        ) -> bool,
    >,
    pub _bfd_find_inliner_info: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *const ::core::ffi::c_char,
            *mut *const ::core::ffi::c_char,
            *mut ::core::ffi::c_uint,
        ) -> bool,
    >,
    pub _bfd_make_debug_symbol: Option<unsafe extern "C" fn(*mut bfd) -> *mut asymbol>,
    pub _read_minisymbols: Option<
        unsafe extern "C" fn(
            *mut bfd,
            bool,
            *mut *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_uint,
        ) -> ::core::ffi::c_long,
    >,
    pub _minisymbol_to_symbol: Option<
        unsafe extern "C" fn(
            *mut bfd,
            bool,
            *const ::core::ffi::c_void,
            *mut asymbol,
        ) -> *mut asymbol,
    >,
    pub _get_reloc_upper_bound:
        Option<unsafe extern "C" fn(*mut bfd, sec_ptr) -> ::core::ffi::c_long>,
    pub _bfd_canonicalize_reloc: Option<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *mut *mut arelent,
            *mut *mut bfd_symbol,
        ) -> ::core::ffi::c_long,
    >,
    pub _bfd_finalize_section_relocs: Option<
        unsafe extern "C" fn(*mut bfd, sec_ptr, *mut *mut arelent, ::core::ffi::c_uint) -> bool,
    >,
    pub reloc_type_lookup:
        Option<unsafe extern "C" fn(*mut bfd, bfd_reloc_code_real_type) -> *const reloc_howto_type>,
    pub reloc_name_lookup: Option<
        unsafe extern "C" fn(*mut bfd, *const ::core::ffi::c_char) -> *const reloc_howto_type,
    >,
    pub _bfd_set_arch_mach:
        Option<unsafe extern "C" fn(*mut bfd, bfd_architecture, ::core::ffi::c_ulong) -> bool>,
    pub _bfd_set_section_contents: Option<
        unsafe extern "C" fn(
            *mut bfd,
            sec_ptr,
            *const ::core::ffi::c_void,
            file_ptr,
            bfd_size_type,
        ) -> bool,
    >,
    pub _bfd_sizeof_headers:
        Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> ::core::ffi::c_int>,
    pub _bfd_get_relocated_section_contents: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut bfd_link_info,
            *mut bfd_link_order,
            *mut bfd_byte,
            bool,
            *mut *mut bfd_symbol,
        ) -> *mut bfd_byte,
    >,
    pub _bfd_relax_section: Option<
        unsafe extern "C" fn(*mut bfd, *mut bfd_section, *mut bfd_link_info, *mut bool) -> bool,
    >,
    pub _bfd_link_hash_table_create:
        Option<unsafe extern "C" fn(*mut bfd) -> *mut bfd_link_hash_table>,
    pub _bfd_link_add_symbols: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool>,
    pub _bfd_link_just_syms: Option<unsafe extern "C" fn(*mut asection, *mut bfd_link_info) -> ()>,
    pub _bfd_copy_link_hash_symbol_type: Option<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_hash_entry, *mut bfd_link_hash_entry) -> (),
    >,
    pub _bfd_final_link: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool>,
    pub _bfd_link_split_section: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_section) -> bool>,
    pub _bfd_link_check_relocs: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool>,
    pub _bfd_gc_sections: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info) -> bool>,
    pub _bfd_lookup_section_flags:
        Option<unsafe extern "C" fn(*mut bfd_link_info, *mut flag_info, *mut asection) -> bool>,
    pub _bfd_is_group_section: Option<unsafe extern "C" fn(*mut bfd, *const bfd_section) -> bool>,
    pub _bfd_group_name:
        Option<unsafe extern "C" fn(*mut bfd, *const bfd_section) -> *const ::core::ffi::c_char>,
    pub _bfd_discard_group: Option<unsafe extern "C" fn(*mut bfd, *mut bfd_section) -> bool>,
    pub _section_already_linked:
        Option<unsafe extern "C" fn(*mut bfd, *mut asection, *mut bfd_link_info) -> bool>,
    pub _bfd_define_common_symbol: Option<
        unsafe extern "C" fn(*mut bfd, *mut bfd_link_info, *mut bfd_link_hash_entry) -> bool,
    >,
    pub _bfd_link_hide_symbol:
        Option<unsafe extern "C" fn(*mut bfd, *mut bfd_link_info, *mut bfd_link_hash_entry) -> ()>,
    pub _bfd_define_start_stop: Option<
        unsafe extern "C" fn(
            *mut bfd_link_info,
            *const ::core::ffi::c_char,
            *mut asection,
        ) -> *mut bfd_link_hash_entry,
    >,
    pub _bfd_get_dynamic_symtab_upper_bound:
        Option<unsafe extern "C" fn(*mut bfd) -> ::core::ffi::c_long>,
    pub _bfd_canonicalize_dynamic_symtab:
        Option<unsafe extern "C" fn(*mut bfd, *mut *mut bfd_symbol) -> ::core::ffi::c_long>,
    pub _bfd_get_synthetic_symtab: Option<
        unsafe extern "C" fn(
            *mut bfd,
            ::core::ffi::c_long,
            *mut *mut bfd_symbol,
            ::core::ffi::c_long,
            *mut *mut bfd_symbol,
            *mut *mut bfd_symbol,
        ) -> ::core::ffi::c_long,
    >,
    pub _bfd_get_dynamic_reloc_upper_bound:
        Option<unsafe extern "C" fn(*mut bfd) -> ::core::ffi::c_long>,
    pub _bfd_canonicalize_dynamic_reloc: Option<
        unsafe extern "C" fn(
            *mut bfd,
            *mut *mut arelent,
            *mut *mut bfd_symbol,
        ) -> ::core::ffi::c_long,
    >,
    pub alternative_target: *const bfd_target,
    pub backend_data: *const ::core::ffi::c_void,
}
pub type sec_ptr = *mut bfd_section;
pub type bfd_reloc_code_real_type = bfd_reloc_code_real;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_reloc_code_real(pub ::core::ffi::c_uint);
impl bfd_reloc_code_real {
    pub const _dummy_first_bfd_reloc_code_real: Self = Self(0);
    pub const BFD_RELOC_64: Self = Self(1);
    pub const BFD_RELOC_32: Self = Self(2);
    pub const BFD_RELOC_26: Self = Self(3);
    pub const BFD_RELOC_24: Self = Self(4);
    pub const BFD_RELOC_16: Self = Self(5);
    pub const BFD_RELOC_14: Self = Self(6);
    pub const BFD_RELOC_8: Self = Self(7);
    pub const BFD_RELOC_64_PCREL: Self = Self(8);
    pub const BFD_RELOC_32_PCREL: Self = Self(9);
    pub const BFD_RELOC_24_PCREL: Self = Self(10);
    pub const BFD_RELOC_16_PCREL: Self = Self(11);
    pub const BFD_RELOC_12_PCREL: Self = Self(12);
    pub const BFD_RELOC_8_PCREL: Self = Self(13);
    pub const BFD_RELOC_32_SECREL: Self = Self(14);
    pub const BFD_RELOC_16_SECIDX: Self = Self(15);
    pub const BFD_RELOC_64_GOT_PCREL: Self = Self(16);
    pub const BFD_RELOC_32_GOT_PCREL: Self = Self(17);
    pub const BFD_RELOC_16_GOT_PCREL: Self = Self(18);
    pub const BFD_RELOC_8_GOT_PCREL: Self = Self(19);
    pub const BFD_RELOC_64_GOTOFF: Self = Self(20);
    pub const BFD_RELOC_32_GOTOFF: Self = Self(21);
    pub const BFD_RELOC_16_GOTOFF: Self = Self(22);
    pub const BFD_RELOC_LO16_GOTOFF: Self = Self(23);
    pub const BFD_RELOC_HI16_GOTOFF: Self = Self(24);
    pub const BFD_RELOC_HI16_S_GOTOFF: Self = Self(25);
    pub const BFD_RELOC_8_GOTOFF: Self = Self(26);
    pub const BFD_RELOC_64_PLT_PCREL: Self = Self(27);
    pub const BFD_RELOC_32_PLT_PCREL: Self = Self(28);
    pub const BFD_RELOC_24_PLT_PCREL: Self = Self(29);
    pub const BFD_RELOC_16_PLT_PCREL: Self = Self(30);
    pub const BFD_RELOC_8_PLT_PCREL: Self = Self(31);
    pub const BFD_RELOC_64_PLTOFF: Self = Self(32);
    pub const BFD_RELOC_32_PLTOFF: Self = Self(33);
    pub const BFD_RELOC_16_PLTOFF: Self = Self(34);
    pub const BFD_RELOC_LO16_PLTOFF: Self = Self(35);
    pub const BFD_RELOC_HI16_PLTOFF: Self = Self(36);
    pub const BFD_RELOC_HI16_S_PLTOFF: Self = Self(37);
    pub const BFD_RELOC_8_PLTOFF: Self = Self(38);
    pub const BFD_RELOC_COPY: Self = Self(39);
    pub const BFD_RELOC_GLOB_DAT: Self = Self(40);
    pub const BFD_RELOC_JMP_SLOT: Self = Self(41);
    pub const BFD_RELOC_RELATIVE: Self = Self(42);
    pub const BFD_RELOC_IRELATIVE: Self = Self(43);
    pub const BFD_RELOC_SIZE32: Self = Self(44);
    pub const BFD_RELOC_SIZE64: Self = Self(45);
    pub const BFD_RELOC_68K_TLS_GD32: Self = Self(46);
    pub const BFD_RELOC_68K_TLS_GD16: Self = Self(47);
    pub const BFD_RELOC_68K_TLS_GD8: Self = Self(48);
    pub const BFD_RELOC_68K_TLS_LDM32: Self = Self(49);
    pub const BFD_RELOC_68K_TLS_LDM16: Self = Self(50);
    pub const BFD_RELOC_68K_TLS_LDM8: Self = Self(51);
    pub const BFD_RELOC_68K_TLS_LDO32: Self = Self(52);
    pub const BFD_RELOC_68K_TLS_LDO16: Self = Self(53);
    pub const BFD_RELOC_68K_TLS_LDO8: Self = Self(54);
    pub const BFD_RELOC_68K_TLS_IE32: Self = Self(55);
    pub const BFD_RELOC_68K_TLS_IE16: Self = Self(56);
    pub const BFD_RELOC_68K_TLS_IE8: Self = Self(57);
    pub const BFD_RELOC_68K_TLS_LE32: Self = Self(58);
    pub const BFD_RELOC_68K_TLS_LE16: Self = Self(59);
    pub const BFD_RELOC_68K_TLS_LE8: Self = Self(60);
    pub const BFD_RELOC_32_BASEREL: Self = Self(61);
    pub const BFD_RELOC_16_BASEREL: Self = Self(62);
    pub const BFD_RELOC_LO16_BASEREL: Self = Self(63);
    pub const BFD_RELOC_HI16_BASEREL: Self = Self(64);
    pub const BFD_RELOC_HI16_S_BASEREL: Self = Self(65);
    pub const BFD_RELOC_8_BASEREL: Self = Self(66);
    pub const BFD_RELOC_RVA: Self = Self(67);
    pub const BFD_RELOC_8_FFnn: Self = Self(68);
    pub const BFD_RELOC_32_PCREL_S2: Self = Self(69);
    pub const BFD_RELOC_16_PCREL_S2: Self = Self(70);
    pub const BFD_RELOC_23_PCREL_S2: Self = Self(71);
    pub const BFD_RELOC_HI22: Self = Self(72);
    pub const BFD_RELOC_LO10: Self = Self(73);
    pub const BFD_RELOC_GPREL16: Self = Self(74);
    pub const BFD_RELOC_GPREL32: Self = Self(75);
    pub const BFD_RELOC_NONE: Self = Self(76);
    pub const BFD_RELOC_SPARC_WDISP22: Self = Self(77);
    pub const BFD_RELOC_SPARC22: Self = Self(78);
    pub const BFD_RELOC_SPARC13: Self = Self(79);
    pub const BFD_RELOC_SPARC_GOT10: Self = Self(80);
    pub const BFD_RELOC_SPARC_GOT13: Self = Self(81);
    pub const BFD_RELOC_SPARC_GOT22: Self = Self(82);
    pub const BFD_RELOC_SPARC_PC10: Self = Self(83);
    pub const BFD_RELOC_SPARC_PC22: Self = Self(84);
    pub const BFD_RELOC_SPARC_WPLT30: Self = Self(85);
    pub const BFD_RELOC_SPARC_UA16: Self = Self(86);
    pub const BFD_RELOC_SPARC_UA32: Self = Self(87);
    pub const BFD_RELOC_SPARC_UA64: Self = Self(88);
    pub const BFD_RELOC_SPARC_GOTDATA_HIX22: Self = Self(89);
    pub const BFD_RELOC_SPARC_GOTDATA_LOX10: Self = Self(90);
    pub const BFD_RELOC_SPARC_GOTDATA_OP_HIX22: Self = Self(91);
    pub const BFD_RELOC_SPARC_GOTDATA_OP_LOX10: Self = Self(92);
    pub const BFD_RELOC_SPARC_GOTDATA_OP: Self = Self(93);
    pub const BFD_RELOC_SPARC_JMP_IREL: Self = Self(94);
    pub const BFD_RELOC_SPARC_BASE13: Self = Self(95);
    pub const BFD_RELOC_SPARC_BASE22: Self = Self(96);
    pub const BFD_RELOC_SPARC_10: Self = Self(97);
    pub const BFD_RELOC_SPARC_11: Self = Self(98);
    pub const BFD_RELOC_SPARC_OLO10: Self = Self(99);
    pub const BFD_RELOC_SPARC_HH22: Self = Self(100);
    pub const BFD_RELOC_SPARC_HM10: Self = Self(101);
    pub const BFD_RELOC_SPARC_LM22: Self = Self(102);
    pub const BFD_RELOC_SPARC_PC_HH22: Self = Self(103);
    pub const BFD_RELOC_SPARC_PC_HM10: Self = Self(104);
    pub const BFD_RELOC_SPARC_PC_LM22: Self = Self(105);
    pub const BFD_RELOC_SPARC_WDISP16: Self = Self(106);
    pub const BFD_RELOC_SPARC_WDISP19: Self = Self(107);
    pub const BFD_RELOC_SPARC_7: Self = Self(108);
    pub const BFD_RELOC_SPARC_6: Self = Self(109);
    pub const BFD_RELOC_SPARC_5: Self = Self(110);
    pub const BFD_RELOC_SPARC_HIX22: Self = Self(111);
    pub const BFD_RELOC_SPARC_LOX10: Self = Self(112);
    pub const BFD_RELOC_SPARC_H44: Self = Self(113);
    pub const BFD_RELOC_SPARC_M44: Self = Self(114);
    pub const BFD_RELOC_SPARC_L44: Self = Self(115);
    pub const BFD_RELOC_SPARC_REGISTER: Self = Self(116);
    pub const BFD_RELOC_SPARC_H34: Self = Self(117);
    pub const BFD_RELOC_SPARC_SIZE32: Self = Self(118);
    pub const BFD_RELOC_SPARC_SIZE64: Self = Self(119);
    pub const BFD_RELOC_SPARC_WDISP10: Self = Self(120);
    pub const BFD_RELOC_SPARC_REV32: Self = Self(121);
    pub const BFD_RELOC_SPARC_TLS_GD_HI22: Self = Self(122);
    pub const BFD_RELOC_SPARC_TLS_GD_LO10: Self = Self(123);
    pub const BFD_RELOC_SPARC_TLS_GD_ADD: Self = Self(124);
    pub const BFD_RELOC_SPARC_TLS_GD_CALL: Self = Self(125);
    pub const BFD_RELOC_SPARC_TLS_LDM_HI22: Self = Self(126);
    pub const BFD_RELOC_SPARC_TLS_LDM_LO10: Self = Self(127);
    pub const BFD_RELOC_SPARC_TLS_LDM_ADD: Self = Self(128);
    pub const BFD_RELOC_SPARC_TLS_LDM_CALL: Self = Self(129);
    pub const BFD_RELOC_SPARC_TLS_LDO_HIX22: Self = Self(130);
    pub const BFD_RELOC_SPARC_TLS_LDO_LOX10: Self = Self(131);
    pub const BFD_RELOC_SPARC_TLS_LDO_ADD: Self = Self(132);
    pub const BFD_RELOC_SPARC_TLS_IE_HI22: Self = Self(133);
    pub const BFD_RELOC_SPARC_TLS_IE_LO10: Self = Self(134);
    pub const BFD_RELOC_SPARC_TLS_IE_LD: Self = Self(135);
    pub const BFD_RELOC_SPARC_TLS_IE_LDX: Self = Self(136);
    pub const BFD_RELOC_SPARC_TLS_IE_ADD: Self = Self(137);
    pub const BFD_RELOC_SPARC_TLS_LE_HIX22: Self = Self(138);
    pub const BFD_RELOC_SPARC_TLS_LE_LOX10: Self = Self(139);
    pub const BFD_RELOC_SPARC_TLS_DTPMOD32: Self = Self(140);
    pub const BFD_RELOC_SPARC_TLS_DTPMOD64: Self = Self(141);
    pub const BFD_RELOC_SPARC_TLS_DTPOFF32: Self = Self(142);
    pub const BFD_RELOC_SPARC_TLS_DTPOFF64: Self = Self(143);
    pub const BFD_RELOC_SPARC_TLS_TPOFF32: Self = Self(144);
    pub const BFD_RELOC_SPARC_TLS_TPOFF64: Self = Self(145);
    pub const BFD_RELOC_SPU_IMM7: Self = Self(146);
    pub const BFD_RELOC_SPU_IMM8: Self = Self(147);
    pub const BFD_RELOC_SPU_IMM10: Self = Self(148);
    pub const BFD_RELOC_SPU_IMM10W: Self = Self(149);
    pub const BFD_RELOC_SPU_IMM16: Self = Self(150);
    pub const BFD_RELOC_SPU_IMM16W: Self = Self(151);
    pub const BFD_RELOC_SPU_IMM18: Self = Self(152);
    pub const BFD_RELOC_SPU_PCREL9a: Self = Self(153);
    pub const BFD_RELOC_SPU_PCREL9b: Self = Self(154);
    pub const BFD_RELOC_SPU_PCREL16: Self = Self(155);
    pub const BFD_RELOC_SPU_LO16: Self = Self(156);
    pub const BFD_RELOC_SPU_HI16: Self = Self(157);
    pub const BFD_RELOC_SPU_PPU32: Self = Self(158);
    pub const BFD_RELOC_SPU_PPU64: Self = Self(159);
    pub const BFD_RELOC_SPU_ADD_PIC: Self = Self(160);
    pub const BFD_RELOC_ALPHA_GPDISP_HI16: Self = Self(161);
    pub const BFD_RELOC_ALPHA_GPDISP_LO16: Self = Self(162);
    pub const BFD_RELOC_ALPHA_GPDISP: Self = Self(163);
    pub const BFD_RELOC_ALPHA_LITERAL: Self = Self(164);
    pub const BFD_RELOC_ALPHA_ELF_LITERAL: Self = Self(165);
    pub const BFD_RELOC_ALPHA_LITUSE: Self = Self(166);
    pub const BFD_RELOC_ALPHA_HINT: Self = Self(167);
    pub const BFD_RELOC_ALPHA_LINKAGE: Self = Self(168);
    pub const BFD_RELOC_ALPHA_CODEADDR: Self = Self(169);
    pub const BFD_RELOC_ALPHA_GPREL_HI16: Self = Self(170);
    pub const BFD_RELOC_ALPHA_GPREL_LO16: Self = Self(171);
    pub const BFD_RELOC_ALPHA_BRSGP: Self = Self(172);
    pub const BFD_RELOC_ALPHA_NOP: Self = Self(173);
    pub const BFD_RELOC_ALPHA_BSR: Self = Self(174);
    pub const BFD_RELOC_ALPHA_LDA: Self = Self(175);
    pub const BFD_RELOC_ALPHA_BOH: Self = Self(176);
    pub const BFD_RELOC_ALPHA_TLSGD: Self = Self(177);
    pub const BFD_RELOC_ALPHA_TLSLDM: Self = Self(178);
    pub const BFD_RELOC_ALPHA_DTPMOD64: Self = Self(179);
    pub const BFD_RELOC_ALPHA_GOTDTPREL16: Self = Self(180);
    pub const BFD_RELOC_ALPHA_DTPREL64: Self = Self(181);
    pub const BFD_RELOC_ALPHA_DTPREL_HI16: Self = Self(182);
    pub const BFD_RELOC_ALPHA_DTPREL_LO16: Self = Self(183);
    pub const BFD_RELOC_ALPHA_DTPREL16: Self = Self(184);
    pub const BFD_RELOC_ALPHA_GOTTPREL16: Self = Self(185);
    pub const BFD_RELOC_ALPHA_TPREL64: Self = Self(186);
    pub const BFD_RELOC_ALPHA_TPREL_HI16: Self = Self(187);
    pub const BFD_RELOC_ALPHA_TPREL_LO16: Self = Self(188);
    pub const BFD_RELOC_ALPHA_TPREL16: Self = Self(189);
    pub const BFD_RELOC_MIPS_JMP: Self = Self(190);
    pub const BFD_RELOC_MICROMIPS_JMP: Self = Self(191);
    pub const BFD_RELOC_MIPS16_JMP: Self = Self(192);
    pub const BFD_RELOC_MIPS16_GPREL: Self = Self(193);
    pub const BFD_RELOC_HI16: Self = Self(194);
    pub const BFD_RELOC_HI16_S: Self = Self(195);
    pub const BFD_RELOC_LO16: Self = Self(196);
    pub const BFD_RELOC_HI16_PCREL: Self = Self(197);
    pub const BFD_RELOC_HI16_S_PCREL: Self = Self(198);
    pub const BFD_RELOC_LO16_PCREL: Self = Self(199);
    pub const BFD_RELOC_MIPS16_GOT16: Self = Self(200);
    pub const BFD_RELOC_MIPS16_CALL16: Self = Self(201);
    pub const BFD_RELOC_MIPS16_HI16: Self = Self(202);
    pub const BFD_RELOC_MIPS16_HI16_S: Self = Self(203);
    pub const BFD_RELOC_MIPS16_LO16: Self = Self(204);
    pub const BFD_RELOC_MIPS16_TLS_GD: Self = Self(205);
    pub const BFD_RELOC_MIPS16_TLS_LDM: Self = Self(206);
    pub const BFD_RELOC_MIPS16_TLS_DTPREL_HI16: Self = Self(207);
    pub const BFD_RELOC_MIPS16_TLS_DTPREL_LO16: Self = Self(208);
    pub const BFD_RELOC_MIPS16_TLS_GOTTPREL: Self = Self(209);
    pub const BFD_RELOC_MIPS16_TLS_TPREL_HI16: Self = Self(210);
    pub const BFD_RELOC_MIPS16_TLS_TPREL_LO16: Self = Self(211);
    pub const BFD_RELOC_MIPS_LITERAL: Self = Self(212);
    pub const BFD_RELOC_MICROMIPS_LITERAL: Self = Self(213);
    pub const BFD_RELOC_MICROMIPS_7_PCREL_S1: Self = Self(214);
    pub const BFD_RELOC_MICROMIPS_10_PCREL_S1: Self = Self(215);
    pub const BFD_RELOC_MICROMIPS_16_PCREL_S1: Self = Self(216);
    pub const BFD_RELOC_MIPS16_16_PCREL_S1: Self = Self(217);
    pub const BFD_RELOC_MIPS_21_PCREL_S2: Self = Self(218);
    pub const BFD_RELOC_MIPS_26_PCREL_S2: Self = Self(219);
    pub const BFD_RELOC_MIPS_18_PCREL_S3: Self = Self(220);
    pub const BFD_RELOC_MIPS_19_PCREL_S2: Self = Self(221);
    pub const BFD_RELOC_MICROMIPS_GPREL16: Self = Self(222);
    pub const BFD_RELOC_MICROMIPS_HI16: Self = Self(223);
    pub const BFD_RELOC_MICROMIPS_HI16_S: Self = Self(224);
    pub const BFD_RELOC_MICROMIPS_LO16: Self = Self(225);
    pub const BFD_RELOC_MIPS_GOT16: Self = Self(226);
    pub const BFD_RELOC_MICROMIPS_GOT16: Self = Self(227);
    pub const BFD_RELOC_MIPS_CALL16: Self = Self(228);
    pub const BFD_RELOC_MICROMIPS_CALL16: Self = Self(229);
    pub const BFD_RELOC_MIPS_GOT_HI16: Self = Self(230);
    pub const BFD_RELOC_MICROMIPS_GOT_HI16: Self = Self(231);
    pub const BFD_RELOC_MIPS_GOT_LO16: Self = Self(232);
    pub const BFD_RELOC_MICROMIPS_GOT_LO16: Self = Self(233);
    pub const BFD_RELOC_MIPS_CALL_HI16: Self = Self(234);
    pub const BFD_RELOC_MICROMIPS_CALL_HI16: Self = Self(235);
    pub const BFD_RELOC_MIPS_CALL_LO16: Self = Self(236);
    pub const BFD_RELOC_MICROMIPS_CALL_LO16: Self = Self(237);
    pub const BFD_RELOC_MIPS_SUB: Self = Self(238);
    pub const BFD_RELOC_MICROMIPS_SUB: Self = Self(239);
    pub const BFD_RELOC_MIPS_GOT_PAGE: Self = Self(240);
    pub const BFD_RELOC_MICROMIPS_GOT_PAGE: Self = Self(241);
    pub const BFD_RELOC_MIPS_GOT_OFST: Self = Self(242);
    pub const BFD_RELOC_MICROMIPS_GOT_OFST: Self = Self(243);
    pub const BFD_RELOC_MIPS_GOT_DISP: Self = Self(244);
    pub const BFD_RELOC_MICROMIPS_GOT_DISP: Self = Self(245);
    pub const BFD_RELOC_MIPS_SHIFT5: Self = Self(246);
    pub const BFD_RELOC_MIPS_SHIFT6: Self = Self(247);
    pub const BFD_RELOC_MIPS_INSERT_A: Self = Self(248);
    pub const BFD_RELOC_MIPS_INSERT_B: Self = Self(249);
    pub const BFD_RELOC_MIPS_DELETE: Self = Self(250);
    pub const BFD_RELOC_MIPS_HIGHEST: Self = Self(251);
    pub const BFD_RELOC_MICROMIPS_HIGHEST: Self = Self(252);
    pub const BFD_RELOC_MIPS_HIGHER: Self = Self(253);
    pub const BFD_RELOC_MICROMIPS_HIGHER: Self = Self(254);
    pub const BFD_RELOC_MIPS_SCN_DISP: Self = Self(255);
    pub const BFD_RELOC_MICROMIPS_SCN_DISP: Self = Self(256);
    pub const BFD_RELOC_MIPS_16: Self = Self(257);
    pub const BFD_RELOC_MIPS_RELGOT: Self = Self(258);
    pub const BFD_RELOC_MIPS_JALR: Self = Self(259);
    pub const BFD_RELOC_MICROMIPS_JALR: Self = Self(260);
    pub const BFD_RELOC_MIPS_TLS_DTPMOD32: Self = Self(261);
    pub const BFD_RELOC_MIPS_TLS_DTPREL32: Self = Self(262);
    pub const BFD_RELOC_MIPS_TLS_DTPMOD64: Self = Self(263);
    pub const BFD_RELOC_MIPS_TLS_DTPREL64: Self = Self(264);
    pub const BFD_RELOC_MIPS_TLS_GD: Self = Self(265);
    pub const BFD_RELOC_MICROMIPS_TLS_GD: Self = Self(266);
    pub const BFD_RELOC_MIPS_TLS_LDM: Self = Self(267);
    pub const BFD_RELOC_MICROMIPS_TLS_LDM: Self = Self(268);
    pub const BFD_RELOC_MIPS_TLS_DTPREL_HI16: Self = Self(269);
    pub const BFD_RELOC_MICROMIPS_TLS_DTPREL_HI16: Self = Self(270);
    pub const BFD_RELOC_MIPS_TLS_DTPREL_LO16: Self = Self(271);
    pub const BFD_RELOC_MICROMIPS_TLS_DTPREL_LO16: Self = Self(272);
    pub const BFD_RELOC_MIPS_TLS_GOTTPREL: Self = Self(273);
    pub const BFD_RELOC_MICROMIPS_TLS_GOTTPREL: Self = Self(274);
    pub const BFD_RELOC_MIPS_TLS_TPREL32: Self = Self(275);
    pub const BFD_RELOC_MIPS_TLS_TPREL64: Self = Self(276);
    pub const BFD_RELOC_MIPS_TLS_TPREL_HI16: Self = Self(277);
    pub const BFD_RELOC_MICROMIPS_TLS_TPREL_HI16: Self = Self(278);
    pub const BFD_RELOC_MIPS_TLS_TPREL_LO16: Self = Self(279);
    pub const BFD_RELOC_MICROMIPS_TLS_TPREL_LO16: Self = Self(280);
    pub const BFD_RELOC_MIPS_EH: Self = Self(281);
    pub const BFD_RELOC_MOXIE_10_PCREL: Self = Self(282);
    pub const BFD_RELOC_FT32_10: Self = Self(283);
    pub const BFD_RELOC_FT32_20: Self = Self(284);
    pub const BFD_RELOC_FT32_17: Self = Self(285);
    pub const BFD_RELOC_FT32_18: Self = Self(286);
    pub const BFD_RELOC_FT32_RELAX: Self = Self(287);
    pub const BFD_RELOC_FT32_SC0: Self = Self(288);
    pub const BFD_RELOC_FT32_SC1: Self = Self(289);
    pub const BFD_RELOC_FT32_15: Self = Self(290);
    pub const BFD_RELOC_FT32_DIFF32: Self = Self(291);
    pub const BFD_RELOC_FRV_LABEL16: Self = Self(292);
    pub const BFD_RELOC_FRV_LABEL24: Self = Self(293);
    pub const BFD_RELOC_FRV_LO16: Self = Self(294);
    pub const BFD_RELOC_FRV_HI16: Self = Self(295);
    pub const BFD_RELOC_FRV_GPREL12: Self = Self(296);
    pub const BFD_RELOC_FRV_GPRELU12: Self = Self(297);
    pub const BFD_RELOC_FRV_GPREL32: Self = Self(298);
    pub const BFD_RELOC_FRV_GPRELHI: Self = Self(299);
    pub const BFD_RELOC_FRV_GPRELLO: Self = Self(300);
    pub const BFD_RELOC_FRV_GOT12: Self = Self(301);
    pub const BFD_RELOC_FRV_GOTHI: Self = Self(302);
    pub const BFD_RELOC_FRV_GOTLO: Self = Self(303);
    pub const BFD_RELOC_FRV_FUNCDESC: Self = Self(304);
    pub const BFD_RELOC_FRV_FUNCDESC_GOT12: Self = Self(305);
    pub const BFD_RELOC_FRV_FUNCDESC_GOTHI: Self = Self(306);
    pub const BFD_RELOC_FRV_FUNCDESC_GOTLO: Self = Self(307);
    pub const BFD_RELOC_FRV_FUNCDESC_VALUE: Self = Self(308);
    pub const BFD_RELOC_FRV_FUNCDESC_GOTOFF12: Self = Self(309);
    pub const BFD_RELOC_FRV_FUNCDESC_GOTOFFHI: Self = Self(310);
    pub const BFD_RELOC_FRV_FUNCDESC_GOTOFFLO: Self = Self(311);
    pub const BFD_RELOC_FRV_GOTOFF12: Self = Self(312);
    pub const BFD_RELOC_FRV_GETTLSOFF: Self = Self(313);
    pub const BFD_RELOC_FRV_TLSDESC_VALUE: Self = Self(314);
    pub const BFD_RELOC_FRV_GOTTLSDESC12: Self = Self(315);
    pub const BFD_RELOC_FRV_GOTTLSDESCHI: Self = Self(316);
    pub const BFD_RELOC_FRV_GOTTLSDESCLO: Self = Self(317);
    pub const BFD_RELOC_FRV_TLSMOFF12: Self = Self(318);
    pub const BFD_RELOC_FRV_TLSMOFFHI: Self = Self(319);
    pub const BFD_RELOC_FRV_TLSMOFFLO: Self = Self(320);
    pub const BFD_RELOC_FRV_GOTTLSOFF12: Self = Self(321);
    pub const BFD_RELOC_FRV_GOTTLSOFFHI: Self = Self(322);
    pub const BFD_RELOC_FRV_GOTTLSOFFLO: Self = Self(323);
    pub const BFD_RELOC_FRV_TLSOFF: Self = Self(324);
    pub const BFD_RELOC_FRV_TLSDESC_RELAX: Self = Self(325);
    pub const BFD_RELOC_FRV_GETTLSOFF_RELAX: Self = Self(326);
    pub const BFD_RELOC_FRV_TLSOFF_RELAX: Self = Self(327);
    pub const BFD_RELOC_FRV_TLSMOFF: Self = Self(328);
    pub const BFD_RELOC_MN10300_GOTOFF24: Self = Self(329);
    pub const BFD_RELOC_MN10300_GOT32: Self = Self(330);
    pub const BFD_RELOC_MN10300_GOT24: Self = Self(331);
    pub const BFD_RELOC_MN10300_GOT16: Self = Self(332);
    pub const BFD_RELOC_MN10300_SYM_DIFF: Self = Self(333);
    pub const BFD_RELOC_MN10300_ALIGN: Self = Self(334);
    pub const BFD_RELOC_MN10300_TLS_GD: Self = Self(335);
    pub const BFD_RELOC_MN10300_TLS_LD: Self = Self(336);
    pub const BFD_RELOC_MN10300_TLS_LDO: Self = Self(337);
    pub const BFD_RELOC_MN10300_TLS_GOTIE: Self = Self(338);
    pub const BFD_RELOC_MN10300_TLS_IE: Self = Self(339);
    pub const BFD_RELOC_MN10300_TLS_LE: Self = Self(340);
    pub const BFD_RELOC_MN10300_TLS_DTPMOD: Self = Self(341);
    pub const BFD_RELOC_MN10300_TLS_DTPOFF: Self = Self(342);
    pub const BFD_RELOC_MN10300_TLS_TPOFF: Self = Self(343);
    pub const BFD_RELOC_386_GOT32: Self = Self(344);
    pub const BFD_RELOC_386_PLT32: Self = Self(345);
    pub const BFD_RELOC_386_TLS_TPOFF: Self = Self(346);
    pub const BFD_RELOC_386_TLS_IE: Self = Self(347);
    pub const BFD_RELOC_386_TLS_GOTIE: Self = Self(348);
    pub const BFD_RELOC_386_TLS_LE: Self = Self(349);
    pub const BFD_RELOC_386_TLS_GD: Self = Self(350);
    pub const BFD_RELOC_386_TLS_LDM: Self = Self(351);
    pub const BFD_RELOC_386_TLS_LDO_32: Self = Self(352);
    pub const BFD_RELOC_386_TLS_IE_32: Self = Self(353);
    pub const BFD_RELOC_386_TLS_LE_32: Self = Self(354);
    pub const BFD_RELOC_386_TLS_DTPMOD32: Self = Self(355);
    pub const BFD_RELOC_386_TLS_DTPOFF32: Self = Self(356);
    pub const BFD_RELOC_386_TLS_TPOFF32: Self = Self(357);
    pub const BFD_RELOC_386_TLS_GOTDESC: Self = Self(358);
    pub const BFD_RELOC_386_TLS_DESC_CALL: Self = Self(359);
    pub const BFD_RELOC_386_TLS_DESC: Self = Self(360);
    pub const BFD_RELOC_386_GOT32X: Self = Self(361);
    pub const BFD_RELOC_X86_64_GOT32: Self = Self(362);
    pub const BFD_RELOC_X86_64_GOTPCREL: Self = Self(363);
    pub const BFD_RELOC_X86_64_32S: Self = Self(364);
    pub const BFD_RELOC_X86_64_DTPMOD64: Self = Self(365);
    pub const BFD_RELOC_X86_64_DTPOFF64: Self = Self(366);
    pub const BFD_RELOC_X86_64_TPOFF64: Self = Self(367);
    pub const BFD_RELOC_X86_64_TLSGD: Self = Self(368);
    pub const BFD_RELOC_X86_64_TLSLD: Self = Self(369);
    pub const BFD_RELOC_X86_64_DTPOFF32: Self = Self(370);
    pub const BFD_RELOC_X86_64_GOTTPOFF: Self = Self(371);
    pub const BFD_RELOC_X86_64_TPOFF32: Self = Self(372);
    pub const BFD_RELOC_X86_64_GOTPC32: Self = Self(373);
    pub const BFD_RELOC_X86_64_GOT64: Self = Self(374);
    pub const BFD_RELOC_X86_64_GOTPCREL64: Self = Self(375);
    pub const BFD_RELOC_X86_64_GOTPLT64: Self = Self(376);
    pub const BFD_RELOC_X86_64_GOTPC32_TLSDESC: Self = Self(377);
    pub const BFD_RELOC_X86_64_TLSDESC_CALL: Self = Self(378);
    pub const BFD_RELOC_X86_64_TLSDESC: Self = Self(379);
    pub const BFD_RELOC_X86_64_PC32_BND: Self = Self(380);
    pub const BFD_RELOC_X86_64_PLT32_BND: Self = Self(381);
    pub const BFD_RELOC_X86_64_GOTPCRELX: Self = Self(382);
    pub const BFD_RELOC_X86_64_REX_GOTPCRELX: Self = Self(383);
    pub const BFD_RELOC_X86_64_CODE_4_GOTPCRELX: Self = Self(384);
    pub const BFD_RELOC_X86_64_CODE_4_GOTTPOFF: Self = Self(385);
    pub const BFD_RELOC_X86_64_CODE_4_GOTPC32_TLSDESC: Self = Self(386);
    pub const BFD_RELOC_X86_64_CODE_5_GOTPCRELX: Self = Self(387);
    pub const BFD_RELOC_X86_64_CODE_5_GOTTPOFF: Self = Self(388);
    pub const BFD_RELOC_X86_64_CODE_5_GOTPC32_TLSDESC: Self = Self(389);
    pub const BFD_RELOC_X86_64_CODE_6_GOTPCRELX: Self = Self(390);
    pub const BFD_RELOC_X86_64_CODE_6_GOTTPOFF: Self = Self(391);
    pub const BFD_RELOC_X86_64_CODE_6_GOTPC32_TLSDESC: Self = Self(392);
    pub const BFD_RELOC_NS32K_IMM_8: Self = Self(393);
    pub const BFD_RELOC_NS32K_IMM_16: Self = Self(394);
    pub const BFD_RELOC_NS32K_IMM_32: Self = Self(395);
    pub const BFD_RELOC_NS32K_IMM_8_PCREL: Self = Self(396);
    pub const BFD_RELOC_NS32K_IMM_16_PCREL: Self = Self(397);
    pub const BFD_RELOC_NS32K_IMM_32_PCREL: Self = Self(398);
    pub const BFD_RELOC_NS32K_DISP_8: Self = Self(399);
    pub const BFD_RELOC_NS32K_DISP_16: Self = Self(400);
    pub const BFD_RELOC_NS32K_DISP_32: Self = Self(401);
    pub const BFD_RELOC_NS32K_DISP_8_PCREL: Self = Self(402);
    pub const BFD_RELOC_NS32K_DISP_16_PCREL: Self = Self(403);
    pub const BFD_RELOC_NS32K_DISP_32_PCREL: Self = Self(404);
    pub const BFD_RELOC_PDP11_DISP_6_PCREL: Self = Self(405);
    pub const BFD_RELOC_PJ_CODE_HI16: Self = Self(406);
    pub const BFD_RELOC_PJ_CODE_LO16: Self = Self(407);
    pub const BFD_RELOC_PJ_CODE_DIR16: Self = Self(408);
    pub const BFD_RELOC_PJ_CODE_DIR32: Self = Self(409);
    pub const BFD_RELOC_PPC_B26: Self = Self(410);
    pub const BFD_RELOC_PPC_BA26: Self = Self(411);
    pub const BFD_RELOC_PPC_TOC16: Self = Self(412);
    pub const BFD_RELOC_PPC_TOC16_LO: Self = Self(413);
    pub const BFD_RELOC_PPC_TOC16_HI: Self = Self(414);
    pub const BFD_RELOC_PPC_B16: Self = Self(415);
    pub const BFD_RELOC_PPC_B16_BRTAKEN: Self = Self(416);
    pub const BFD_RELOC_PPC_B16_BRNTAKEN: Self = Self(417);
    pub const BFD_RELOC_PPC_BA16: Self = Self(418);
    pub const BFD_RELOC_PPC_BA16_BRTAKEN: Self = Self(419);
    pub const BFD_RELOC_PPC_BA16_BRNTAKEN: Self = Self(420);
    pub const BFD_RELOC_PPC_LOCAL24PC: Self = Self(421);
    pub const BFD_RELOC_PPC_EMB_NADDR32: Self = Self(422);
    pub const BFD_RELOC_PPC_EMB_NADDR16: Self = Self(423);
    pub const BFD_RELOC_PPC_EMB_NADDR16_LO: Self = Self(424);
    pub const BFD_RELOC_PPC_EMB_NADDR16_HI: Self = Self(425);
    pub const BFD_RELOC_PPC_EMB_NADDR16_HA: Self = Self(426);
    pub const BFD_RELOC_PPC_EMB_SDAI16: Self = Self(427);
    pub const BFD_RELOC_PPC_EMB_SDA2I16: Self = Self(428);
    pub const BFD_RELOC_PPC_EMB_SDA2REL: Self = Self(429);
    pub const BFD_RELOC_PPC_EMB_SDA21: Self = Self(430);
    pub const BFD_RELOC_PPC_EMB_MRKREF: Self = Self(431);
    pub const BFD_RELOC_PPC_EMB_RELSEC16: Self = Self(432);
    pub const BFD_RELOC_PPC_EMB_RELST_LO: Self = Self(433);
    pub const BFD_RELOC_PPC_EMB_RELST_HI: Self = Self(434);
    pub const BFD_RELOC_PPC_EMB_RELST_HA: Self = Self(435);
    pub const BFD_RELOC_PPC_EMB_BIT_FLD: Self = Self(436);
    pub const BFD_RELOC_PPC_EMB_RELSDA: Self = Self(437);
    pub const BFD_RELOC_PPC_VLE_REL8: Self = Self(438);
    pub const BFD_RELOC_PPC_VLE_REL15: Self = Self(439);
    pub const BFD_RELOC_PPC_VLE_REL24: Self = Self(440);
    pub const BFD_RELOC_PPC_VLE_LO16A: Self = Self(441);
    pub const BFD_RELOC_PPC_VLE_LO16D: Self = Self(442);
    pub const BFD_RELOC_PPC_VLE_HI16A: Self = Self(443);
    pub const BFD_RELOC_PPC_VLE_HI16D: Self = Self(444);
    pub const BFD_RELOC_PPC_VLE_HA16A: Self = Self(445);
    pub const BFD_RELOC_PPC_VLE_HA16D: Self = Self(446);
    pub const BFD_RELOC_PPC_VLE_SDA21: Self = Self(447);
    pub const BFD_RELOC_PPC_VLE_SDA21_LO: Self = Self(448);
    pub const BFD_RELOC_PPC_VLE_SDAREL_LO16A: Self = Self(449);
    pub const BFD_RELOC_PPC_VLE_SDAREL_LO16D: Self = Self(450);
    pub const BFD_RELOC_PPC_VLE_SDAREL_HI16A: Self = Self(451);
    pub const BFD_RELOC_PPC_VLE_SDAREL_HI16D: Self = Self(452);
    pub const BFD_RELOC_PPC_VLE_SDAREL_HA16A: Self = Self(453);
    pub const BFD_RELOC_PPC_VLE_SDAREL_HA16D: Self = Self(454);
    pub const BFD_RELOC_PPC_16DX_HA: Self = Self(455);
    pub const BFD_RELOC_PPC_REL16DX_HA: Self = Self(456);
    pub const BFD_RELOC_PPC_NEG: Self = Self(457);
    pub const BFD_RELOC_PPC64_HIGHER: Self = Self(458);
    pub const BFD_RELOC_PPC64_HIGHER_S: Self = Self(459);
    pub const BFD_RELOC_PPC64_HIGHEST: Self = Self(460);
    pub const BFD_RELOC_PPC64_HIGHEST_S: Self = Self(461);
    pub const BFD_RELOC_PPC64_TOC16_LO: Self = Self(462);
    pub const BFD_RELOC_PPC64_TOC16_HI: Self = Self(463);
    pub const BFD_RELOC_PPC64_TOC16_HA: Self = Self(464);
    pub const BFD_RELOC_PPC64_TOC: Self = Self(465);
    pub const BFD_RELOC_PPC64_PLTGOT16: Self = Self(466);
    pub const BFD_RELOC_PPC64_PLTGOT16_LO: Self = Self(467);
    pub const BFD_RELOC_PPC64_PLTGOT16_HI: Self = Self(468);
    pub const BFD_RELOC_PPC64_PLTGOT16_HA: Self = Self(469);
    pub const BFD_RELOC_PPC64_ADDR16_DS: Self = Self(470);
    pub const BFD_RELOC_PPC64_ADDR16_LO_DS: Self = Self(471);
    pub const BFD_RELOC_PPC64_GOT16_DS: Self = Self(472);
    pub const BFD_RELOC_PPC64_GOT16_LO_DS: Self = Self(473);
    pub const BFD_RELOC_PPC64_PLT16_LO_DS: Self = Self(474);
    pub const BFD_RELOC_PPC64_SECTOFF_DS: Self = Self(475);
    pub const BFD_RELOC_PPC64_SECTOFF_LO_DS: Self = Self(476);
    pub const BFD_RELOC_PPC64_TOC16_DS: Self = Self(477);
    pub const BFD_RELOC_PPC64_TOC16_LO_DS: Self = Self(478);
    pub const BFD_RELOC_PPC64_PLTGOT16_DS: Self = Self(479);
    pub const BFD_RELOC_PPC64_PLTGOT16_LO_DS: Self = Self(480);
    pub const BFD_RELOC_PPC64_ADDR16_HIGH: Self = Self(481);
    pub const BFD_RELOC_PPC64_ADDR16_HIGHA: Self = Self(482);
    pub const BFD_RELOC_PPC64_REL16_HIGH: Self = Self(483);
    pub const BFD_RELOC_PPC64_REL16_HIGHA: Self = Self(484);
    pub const BFD_RELOC_PPC64_REL16_HIGHER: Self = Self(485);
    pub const BFD_RELOC_PPC64_REL16_HIGHERA: Self = Self(486);
    pub const BFD_RELOC_PPC64_REL16_HIGHEST: Self = Self(487);
    pub const BFD_RELOC_PPC64_REL16_HIGHESTA: Self = Self(488);
    pub const BFD_RELOC_PPC64_ADDR64_LOCAL: Self = Self(489);
    pub const BFD_RELOC_PPC64_ENTRY: Self = Self(490);
    pub const BFD_RELOC_PPC64_REL24_NOTOC: Self = Self(491);
    pub const BFD_RELOC_PPC64_REL24_P9NOTOC: Self = Self(492);
    pub const BFD_RELOC_PPC64_D34: Self = Self(493);
    pub const BFD_RELOC_PPC64_D34_LO: Self = Self(494);
    pub const BFD_RELOC_PPC64_D34_HI30: Self = Self(495);
    pub const BFD_RELOC_PPC64_D34_HA30: Self = Self(496);
    pub const BFD_RELOC_PPC64_PCREL34: Self = Self(497);
    pub const BFD_RELOC_PPC64_GOT_PCREL34: Self = Self(498);
    pub const BFD_RELOC_PPC64_PLT_PCREL34: Self = Self(499);
    pub const BFD_RELOC_PPC64_ADDR16_HIGHER34: Self = Self(500);
    pub const BFD_RELOC_PPC64_ADDR16_HIGHERA34: Self = Self(501);
    pub const BFD_RELOC_PPC64_ADDR16_HIGHEST34: Self = Self(502);
    pub const BFD_RELOC_PPC64_ADDR16_HIGHESTA34: Self = Self(503);
    pub const BFD_RELOC_PPC64_REL16_HIGHER34: Self = Self(504);
    pub const BFD_RELOC_PPC64_REL16_HIGHERA34: Self = Self(505);
    pub const BFD_RELOC_PPC64_REL16_HIGHEST34: Self = Self(506);
    pub const BFD_RELOC_PPC64_REL16_HIGHESTA34: Self = Self(507);
    pub const BFD_RELOC_PPC64_D28: Self = Self(508);
    pub const BFD_RELOC_PPC64_PCREL28: Self = Self(509);
    pub const BFD_RELOC_PPC_TLS: Self = Self(510);
    pub const BFD_RELOC_PPC_TLSGD: Self = Self(511);
    pub const BFD_RELOC_PPC_TLSLD: Self = Self(512);
    pub const BFD_RELOC_PPC_TLSLE: Self = Self(513);
    pub const BFD_RELOC_PPC_TLSIE: Self = Self(514);
    pub const BFD_RELOC_PPC_TLSM: Self = Self(515);
    pub const BFD_RELOC_PPC_TLSML: Self = Self(516);
    pub const BFD_RELOC_PPC_DTPMOD: Self = Self(517);
    pub const BFD_RELOC_PPC_TPREL16: Self = Self(518);
    pub const BFD_RELOC_PPC_TPREL16_LO: Self = Self(519);
    pub const BFD_RELOC_PPC_TPREL16_HI: Self = Self(520);
    pub const BFD_RELOC_PPC_TPREL16_HA: Self = Self(521);
    pub const BFD_RELOC_PPC_TPREL: Self = Self(522);
    pub const BFD_RELOC_PPC_DTPREL16: Self = Self(523);
    pub const BFD_RELOC_PPC_DTPREL16_LO: Self = Self(524);
    pub const BFD_RELOC_PPC_DTPREL16_HI: Self = Self(525);
    pub const BFD_RELOC_PPC_DTPREL16_HA: Self = Self(526);
    pub const BFD_RELOC_PPC_DTPREL: Self = Self(527);
    pub const BFD_RELOC_PPC_GOT_TLSGD16: Self = Self(528);
    pub const BFD_RELOC_PPC_GOT_TLSGD16_LO: Self = Self(529);
    pub const BFD_RELOC_PPC_GOT_TLSGD16_HI: Self = Self(530);
    pub const BFD_RELOC_PPC_GOT_TLSGD16_HA: Self = Self(531);
    pub const BFD_RELOC_PPC_GOT_TLSLD16: Self = Self(532);
    pub const BFD_RELOC_PPC_GOT_TLSLD16_LO: Self = Self(533);
    pub const BFD_RELOC_PPC_GOT_TLSLD16_HI: Self = Self(534);
    pub const BFD_RELOC_PPC_GOT_TLSLD16_HA: Self = Self(535);
    pub const BFD_RELOC_PPC_GOT_TPREL16: Self = Self(536);
    pub const BFD_RELOC_PPC_GOT_TPREL16_LO: Self = Self(537);
    pub const BFD_RELOC_PPC_GOT_TPREL16_HI: Self = Self(538);
    pub const BFD_RELOC_PPC_GOT_TPREL16_HA: Self = Self(539);
    pub const BFD_RELOC_PPC_GOT_DTPREL16: Self = Self(540);
    pub const BFD_RELOC_PPC_GOT_DTPREL16_LO: Self = Self(541);
    pub const BFD_RELOC_PPC_GOT_DTPREL16_HI: Self = Self(542);
    pub const BFD_RELOC_PPC_GOT_DTPREL16_HA: Self = Self(543);
    pub const BFD_RELOC_PPC64_TLSGD: Self = Self(544);
    pub const BFD_RELOC_PPC64_TLSLD: Self = Self(545);
    pub const BFD_RELOC_PPC64_TLSLE: Self = Self(546);
    pub const BFD_RELOC_PPC64_TLSIE: Self = Self(547);
    pub const BFD_RELOC_PPC64_TLSM: Self = Self(548);
    pub const BFD_RELOC_PPC64_TLSML: Self = Self(549);
    pub const BFD_RELOC_PPC64_TPREL16_DS: Self = Self(550);
    pub const BFD_RELOC_PPC64_TPREL16_LO_DS: Self = Self(551);
    pub const BFD_RELOC_PPC64_TPREL16_HIGH: Self = Self(552);
    pub const BFD_RELOC_PPC64_TPREL16_HIGHA: Self = Self(553);
    pub const BFD_RELOC_PPC64_TPREL16_HIGHER: Self = Self(554);
    pub const BFD_RELOC_PPC64_TPREL16_HIGHERA: Self = Self(555);
    pub const BFD_RELOC_PPC64_TPREL16_HIGHEST: Self = Self(556);
    pub const BFD_RELOC_PPC64_TPREL16_HIGHESTA: Self = Self(557);
    pub const BFD_RELOC_PPC64_DTPREL16_DS: Self = Self(558);
    pub const BFD_RELOC_PPC64_DTPREL16_LO_DS: Self = Self(559);
    pub const BFD_RELOC_PPC64_DTPREL16_HIGH: Self = Self(560);
    pub const BFD_RELOC_PPC64_DTPREL16_HIGHA: Self = Self(561);
    pub const BFD_RELOC_PPC64_DTPREL16_HIGHER: Self = Self(562);
    pub const BFD_RELOC_PPC64_DTPREL16_HIGHERA: Self = Self(563);
    pub const BFD_RELOC_PPC64_DTPREL16_HIGHEST: Self = Self(564);
    pub const BFD_RELOC_PPC64_DTPREL16_HIGHESTA: Self = Self(565);
    pub const BFD_RELOC_PPC64_TPREL34: Self = Self(566);
    pub const BFD_RELOC_PPC64_DTPREL34: Self = Self(567);
    pub const BFD_RELOC_PPC64_GOT_TLSGD_PCREL34: Self = Self(568);
    pub const BFD_RELOC_PPC64_GOT_TLSLD_PCREL34: Self = Self(569);
    pub const BFD_RELOC_PPC64_GOT_TPREL_PCREL34: Self = Self(570);
    pub const BFD_RELOC_PPC64_GOT_DTPREL_PCREL34: Self = Self(571);
    pub const BFD_RELOC_PPC64_TLS_PCREL: Self = Self(572);
    pub const BFD_RELOC_CTOR: Self = Self(573);
    pub const BFD_RELOC_ARM_PCREL_BRANCH: Self = Self(574);
    pub const BFD_RELOC_ARM_PCREL_BLX: Self = Self(575);
    pub const BFD_RELOC_THUMB_PCREL_BLX: Self = Self(576);
    pub const BFD_RELOC_ARM_PCREL_CALL: Self = Self(577);
    pub const BFD_RELOC_ARM_PCREL_JUMP: Self = Self(578);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH5: Self = Self(579);
    pub const BFD_RELOC_THUMB_PCREL_BFCSEL: Self = Self(580);
    pub const BFD_RELOC_ARM_THUMB_BF17: Self = Self(581);
    pub const BFD_RELOC_ARM_THUMB_BF13: Self = Self(582);
    pub const BFD_RELOC_ARM_THUMB_BF19: Self = Self(583);
    pub const BFD_RELOC_ARM_THUMB_LOOP12: Self = Self(584);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH7: Self = Self(585);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH9: Self = Self(586);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH12: Self = Self(587);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH20: Self = Self(588);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH23: Self = Self(589);
    pub const BFD_RELOC_THUMB_PCREL_BRANCH25: Self = Self(590);
    pub const BFD_RELOC_ARM_OFFSET_IMM: Self = Self(591);
    pub const BFD_RELOC_ARM_THUMB_OFFSET: Self = Self(592);
    pub const BFD_RELOC_ARM_TARGET1: Self = Self(593);
    pub const BFD_RELOC_ARM_ROSEGREL32: Self = Self(594);
    pub const BFD_RELOC_ARM_SBREL32: Self = Self(595);
    pub const BFD_RELOC_ARM_TARGET2: Self = Self(596);
    pub const BFD_RELOC_ARM_PREL31: Self = Self(597);
    pub const BFD_RELOC_ARM_MOVW: Self = Self(598);
    pub const BFD_RELOC_ARM_MOVT: Self = Self(599);
    pub const BFD_RELOC_ARM_MOVW_PCREL: Self = Self(600);
    pub const BFD_RELOC_ARM_MOVT_PCREL: Self = Self(601);
    pub const BFD_RELOC_ARM_THUMB_MOVW: Self = Self(602);
    pub const BFD_RELOC_ARM_THUMB_MOVT: Self = Self(603);
    pub const BFD_RELOC_ARM_THUMB_MOVW_PCREL: Self = Self(604);
    pub const BFD_RELOC_ARM_THUMB_MOVT_PCREL: Self = Self(605);
    pub const BFD_RELOC_ARM_GOTFUNCDESC: Self = Self(606);
    pub const BFD_RELOC_ARM_GOTOFFFUNCDESC: Self = Self(607);
    pub const BFD_RELOC_ARM_FUNCDESC: Self = Self(608);
    pub const BFD_RELOC_ARM_FUNCDESC_VALUE: Self = Self(609);
    pub const BFD_RELOC_ARM_TLS_GD32_FDPIC: Self = Self(610);
    pub const BFD_RELOC_ARM_TLS_LDM32_FDPIC: Self = Self(611);
    pub const BFD_RELOC_ARM_TLS_IE32_FDPIC: Self = Self(612);
    pub const BFD_RELOC_ARM_GOT32: Self = Self(613);
    pub const BFD_RELOC_ARM_GOT_PREL: Self = Self(614);
    pub const BFD_RELOC_ARM_TLS_GD32: Self = Self(615);
    pub const BFD_RELOC_ARM_TLS_LDO32: Self = Self(616);
    pub const BFD_RELOC_ARM_TLS_LDM32: Self = Self(617);
    pub const BFD_RELOC_ARM_TLS_DTPOFF32: Self = Self(618);
    pub const BFD_RELOC_ARM_TLS_DTPMOD32: Self = Self(619);
    pub const BFD_RELOC_ARM_TLS_TPOFF32: Self = Self(620);
    pub const BFD_RELOC_ARM_TLS_IE32: Self = Self(621);
    pub const BFD_RELOC_ARM_TLS_LE32: Self = Self(622);
    pub const BFD_RELOC_ARM_TLS_GOTDESC: Self = Self(623);
    pub const BFD_RELOC_ARM_TLS_CALL: Self = Self(624);
    pub const BFD_RELOC_ARM_THM_TLS_CALL: Self = Self(625);
    pub const BFD_RELOC_ARM_TLS_DESCSEQ: Self = Self(626);
    pub const BFD_RELOC_ARM_THM_TLS_DESCSEQ: Self = Self(627);
    pub const BFD_RELOC_ARM_TLS_DESC: Self = Self(628);
    pub const BFD_RELOC_ARM_ALU_PC_G0_NC: Self = Self(629);
    pub const BFD_RELOC_ARM_ALU_PC_G0: Self = Self(630);
    pub const BFD_RELOC_ARM_ALU_PC_G1_NC: Self = Self(631);
    pub const BFD_RELOC_ARM_ALU_PC_G1: Self = Self(632);
    pub const BFD_RELOC_ARM_ALU_PC_G2: Self = Self(633);
    pub const BFD_RELOC_ARM_LDR_PC_G0: Self = Self(634);
    pub const BFD_RELOC_ARM_LDR_PC_G1: Self = Self(635);
    pub const BFD_RELOC_ARM_LDR_PC_G2: Self = Self(636);
    pub const BFD_RELOC_ARM_LDRS_PC_G0: Self = Self(637);
    pub const BFD_RELOC_ARM_LDRS_PC_G1: Self = Self(638);
    pub const BFD_RELOC_ARM_LDRS_PC_G2: Self = Self(639);
    pub const BFD_RELOC_ARM_LDC_PC_G0: Self = Self(640);
    pub const BFD_RELOC_ARM_LDC_PC_G1: Self = Self(641);
    pub const BFD_RELOC_ARM_LDC_PC_G2: Self = Self(642);
    pub const BFD_RELOC_ARM_ALU_SB_G0_NC: Self = Self(643);
    pub const BFD_RELOC_ARM_ALU_SB_G0: Self = Self(644);
    pub const BFD_RELOC_ARM_ALU_SB_G1_NC: Self = Self(645);
    pub const BFD_RELOC_ARM_ALU_SB_G1: Self = Self(646);
    pub const BFD_RELOC_ARM_ALU_SB_G2: Self = Self(647);
    pub const BFD_RELOC_ARM_LDR_SB_G0: Self = Self(648);
    pub const BFD_RELOC_ARM_LDR_SB_G1: Self = Self(649);
    pub const BFD_RELOC_ARM_LDR_SB_G2: Self = Self(650);
    pub const BFD_RELOC_ARM_LDRS_SB_G0: Self = Self(651);
    pub const BFD_RELOC_ARM_LDRS_SB_G1: Self = Self(652);
    pub const BFD_RELOC_ARM_LDRS_SB_G2: Self = Self(653);
    pub const BFD_RELOC_ARM_LDC_SB_G0: Self = Self(654);
    pub const BFD_RELOC_ARM_LDC_SB_G1: Self = Self(655);
    pub const BFD_RELOC_ARM_LDC_SB_G2: Self = Self(656);
    pub const BFD_RELOC_ARM_V4BX: Self = Self(657);
    pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G0_NC: Self = Self(658);
    pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G1_NC: Self = Self(659);
    pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G2_NC: Self = Self(660);
    pub const BFD_RELOC_ARM_THUMB_ALU_ABS_G3_NC: Self = Self(661);
    pub const BFD_RELOC_ARM_IMMEDIATE: Self = Self(662);
    pub const BFD_RELOC_ARM_ADRL_IMMEDIATE: Self = Self(663);
    pub const BFD_RELOC_ARM_T32_IMMEDIATE: Self = Self(664);
    pub const BFD_RELOC_ARM_T32_ADD_IMM: Self = Self(665);
    pub const BFD_RELOC_ARM_T32_IMM12: Self = Self(666);
    pub const BFD_RELOC_ARM_T32_ADD_PC12: Self = Self(667);
    pub const BFD_RELOC_ARM_SHIFT_IMM: Self = Self(668);
    pub const BFD_RELOC_ARM_SMC: Self = Self(669);
    pub const BFD_RELOC_ARM_HVC: Self = Self(670);
    pub const BFD_RELOC_ARM_SWI: Self = Self(671);
    pub const BFD_RELOC_ARM_MULTI: Self = Self(672);
    pub const BFD_RELOC_ARM_CP_OFF_IMM: Self = Self(673);
    pub const BFD_RELOC_ARM_CP_OFF_IMM_S2: Self = Self(674);
    pub const BFD_RELOC_ARM_T32_CP_OFF_IMM: Self = Self(675);
    pub const BFD_RELOC_ARM_T32_CP_OFF_IMM_S2: Self = Self(676);
    pub const BFD_RELOC_ARM_T32_VLDR_VSTR_OFF_IMM: Self = Self(677);
    pub const BFD_RELOC_ARM_ADR_IMM: Self = Self(678);
    pub const BFD_RELOC_ARM_LDR_IMM: Self = Self(679);
    pub const BFD_RELOC_ARM_LITERAL: Self = Self(680);
    pub const BFD_RELOC_ARM_IN_POOL: Self = Self(681);
    pub const BFD_RELOC_ARM_OFFSET_IMM8: Self = Self(682);
    pub const BFD_RELOC_ARM_T32_OFFSET_U8: Self = Self(683);
    pub const BFD_RELOC_ARM_T32_OFFSET_IMM: Self = Self(684);
    pub const BFD_RELOC_ARM_HWLITERAL: Self = Self(685);
    pub const BFD_RELOC_ARM_THUMB_ADD: Self = Self(686);
    pub const BFD_RELOC_ARM_THUMB_IMM: Self = Self(687);
    pub const BFD_RELOC_ARM_THUMB_SHIFT: Self = Self(688);
    pub const BFD_RELOC_SH_PCDISP8BY2: Self = Self(689);
    pub const BFD_RELOC_SH_PCDISP12BY2: Self = Self(690);
    pub const BFD_RELOC_SH_IMM3: Self = Self(691);
    pub const BFD_RELOC_SH_IMM3U: Self = Self(692);
    pub const BFD_RELOC_SH_DISP12: Self = Self(693);
    pub const BFD_RELOC_SH_DISP12BY2: Self = Self(694);
    pub const BFD_RELOC_SH_DISP12BY4: Self = Self(695);
    pub const BFD_RELOC_SH_DISP12BY8: Self = Self(696);
    pub const BFD_RELOC_SH_DISP20: Self = Self(697);
    pub const BFD_RELOC_SH_DISP20BY8: Self = Self(698);
    pub const BFD_RELOC_SH_IMM4: Self = Self(699);
    pub const BFD_RELOC_SH_IMM4BY2: Self = Self(700);
    pub const BFD_RELOC_SH_IMM4BY4: Self = Self(701);
    pub const BFD_RELOC_SH_IMM8: Self = Self(702);
    pub const BFD_RELOC_SH_IMM8BY2: Self = Self(703);
    pub const BFD_RELOC_SH_IMM8BY4: Self = Self(704);
    pub const BFD_RELOC_SH_PCRELIMM8BY2: Self = Self(705);
    pub const BFD_RELOC_SH_PCRELIMM8BY4: Self = Self(706);
    pub const BFD_RELOC_SH_SWITCH16: Self = Self(707);
    pub const BFD_RELOC_SH_SWITCH32: Self = Self(708);
    pub const BFD_RELOC_SH_USES: Self = Self(709);
    pub const BFD_RELOC_SH_COUNT: Self = Self(710);
    pub const BFD_RELOC_SH_ALIGN: Self = Self(711);
    pub const BFD_RELOC_SH_CODE: Self = Self(712);
    pub const BFD_RELOC_SH_DATA: Self = Self(713);
    pub const BFD_RELOC_SH_LABEL: Self = Self(714);
    pub const BFD_RELOC_SH_LOOP_START: Self = Self(715);
    pub const BFD_RELOC_SH_LOOP_END: Self = Self(716);
    pub const BFD_RELOC_SH_COPY64: Self = Self(717);
    pub const BFD_RELOC_SH_GLOB_DAT64: Self = Self(718);
    pub const BFD_RELOC_SH_JMP_SLOT64: Self = Self(719);
    pub const BFD_RELOC_SH_RELATIVE64: Self = Self(720);
    pub const BFD_RELOC_SH_GOT10BY4: Self = Self(721);
    pub const BFD_RELOC_SH_GOT10BY8: Self = Self(722);
    pub const BFD_RELOC_SH_GOT32: Self = Self(723);
    pub const BFD_RELOC_SH_GOTPLT10BY4: Self = Self(724);
    pub const BFD_RELOC_SH_GOTPLT10BY8: Self = Self(725);
    pub const BFD_RELOC_SH_GOTPLT32: Self = Self(726);
    pub const BFD_RELOC_SH_SHMEDIA_CODE: Self = Self(727);
    pub const BFD_RELOC_SH_PT_16: Self = Self(728);
    pub const BFD_RELOC_SH_TLS_GD_32: Self = Self(729);
    pub const BFD_RELOC_SH_TLS_LD_32: Self = Self(730);
    pub const BFD_RELOC_SH_TLS_LDO_32: Self = Self(731);
    pub const BFD_RELOC_SH_TLS_IE_32: Self = Self(732);
    pub const BFD_RELOC_SH_TLS_LE_32: Self = Self(733);
    pub const BFD_RELOC_SH_TLS_DTPMOD32: Self = Self(734);
    pub const BFD_RELOC_SH_TLS_DTPOFF32: Self = Self(735);
    pub const BFD_RELOC_SH_TLS_TPOFF32: Self = Self(736);
    pub const BFD_RELOC_SH_GOT20: Self = Self(737);
    pub const BFD_RELOC_SH_GOTOFF20: Self = Self(738);
    pub const BFD_RELOC_SH_GOTFUNCDESC: Self = Self(739);
    pub const BFD_RELOC_SH_GOTFUNCDESC20: Self = Self(740);
    pub const BFD_RELOC_SH_GOTOFFFUNCDESC: Self = Self(741);
    pub const BFD_RELOC_SH_GOTOFFFUNCDESC20: Self = Self(742);
    pub const BFD_RELOC_SH_FUNCDESC: Self = Self(743);
    pub const BFD_RELOC_ARC_N8: Self = Self(744);
    pub const BFD_RELOC_ARC_N16: Self = Self(745);
    pub const BFD_RELOC_ARC_N24: Self = Self(746);
    pub const BFD_RELOC_ARC_N32: Self = Self(747);
    pub const BFD_RELOC_ARC_SDA: Self = Self(748);
    pub const BFD_RELOC_ARC_SECTOFF: Self = Self(749);
    pub const BFD_RELOC_ARC_S21H_PCREL: Self = Self(750);
    pub const BFD_RELOC_ARC_S21W_PCREL: Self = Self(751);
    pub const BFD_RELOC_ARC_S25H_PCREL: Self = Self(752);
    pub const BFD_RELOC_ARC_S25W_PCREL: Self = Self(753);
    pub const BFD_RELOC_ARC_SDA32: Self = Self(754);
    pub const BFD_RELOC_ARC_SDA_LDST: Self = Self(755);
    pub const BFD_RELOC_ARC_SDA_LDST1: Self = Self(756);
    pub const BFD_RELOC_ARC_SDA_LDST2: Self = Self(757);
    pub const BFD_RELOC_ARC_SDA16_LD: Self = Self(758);
    pub const BFD_RELOC_ARC_SDA16_LD1: Self = Self(759);
    pub const BFD_RELOC_ARC_SDA16_LD2: Self = Self(760);
    pub const BFD_RELOC_ARC_S13_PCREL: Self = Self(761);
    pub const BFD_RELOC_ARC_W: Self = Self(762);
    pub const BFD_RELOC_ARC_32_ME: Self = Self(763);
    pub const BFD_RELOC_ARC_32_ME_S: Self = Self(764);
    pub const BFD_RELOC_ARC_N32_ME: Self = Self(765);
    pub const BFD_RELOC_ARC_SECTOFF_ME: Self = Self(766);
    pub const BFD_RELOC_ARC_SDA32_ME: Self = Self(767);
    pub const BFD_RELOC_ARC_W_ME: Self = Self(768);
    pub const BFD_RELOC_AC_SECTOFF_U8: Self = Self(769);
    pub const BFD_RELOC_AC_SECTOFF_U8_1: Self = Self(770);
    pub const BFD_RELOC_AC_SECTOFF_U8_2: Self = Self(771);
    pub const BFD_RELOC_AC_SECTOFF_S9: Self = Self(772);
    pub const BFD_RELOC_AC_SECTOFF_S9_1: Self = Self(773);
    pub const BFD_RELOC_AC_SECTOFF_S9_2: Self = Self(774);
    pub const BFD_RELOC_ARC_SECTOFF_ME_1: Self = Self(775);
    pub const BFD_RELOC_ARC_SECTOFF_ME_2: Self = Self(776);
    pub const BFD_RELOC_ARC_SECTOFF_1: Self = Self(777);
    pub const BFD_RELOC_ARC_SECTOFF_2: Self = Self(778);
    pub const BFD_RELOC_ARC_SDA_12: Self = Self(779);
    pub const BFD_RELOC_ARC_SDA16_ST2: Self = Self(780);
    pub const BFD_RELOC_ARC_32_PCREL: Self = Self(781);
    pub const BFD_RELOC_ARC_GOT32: Self = Self(782);
    pub const BFD_RELOC_ARC_GOTPC: Self = Self(783);
    pub const BFD_RELOC_ARC_S21W_PCREL_PLT: Self = Self(784);
    pub const BFD_RELOC_ARC_S25H_PCREL_PLT: Self = Self(785);
    pub const BFD_RELOC_ARC_TLS_DTPMOD: Self = Self(786);
    pub const BFD_RELOC_ARC_TLS_TPOFF: Self = Self(787);
    pub const BFD_RELOC_ARC_TLS_GD_GOT: Self = Self(788);
    pub const BFD_RELOC_ARC_TLS_GD_LD: Self = Self(789);
    pub const BFD_RELOC_ARC_TLS_GD_CALL: Self = Self(790);
    pub const BFD_RELOC_ARC_TLS_IE_GOT: Self = Self(791);
    pub const BFD_RELOC_ARC_TLS_DTPOFF: Self = Self(792);
    pub const BFD_RELOC_ARC_TLS_DTPOFF_S9: Self = Self(793);
    pub const BFD_RELOC_ARC_TLS_LE_S9: Self = Self(794);
    pub const BFD_RELOC_ARC_TLS_LE_32: Self = Self(795);
    pub const BFD_RELOC_ARC_S25W_PCREL_PLT: Self = Self(796);
    pub const BFD_RELOC_ARC_S21H_PCREL_PLT: Self = Self(797);
    pub const BFD_RELOC_ARC_NPS_CMEM16: Self = Self(798);
    pub const BFD_RELOC_ARC_JLI_SECTOFF: Self = Self(799);
    pub const BFD_RELOC_BFIN_16_IMM: Self = Self(800);
    pub const BFD_RELOC_BFIN_16_HIGH: Self = Self(801);
    pub const BFD_RELOC_BFIN_4_PCREL: Self = Self(802);
    pub const BFD_RELOC_BFIN_5_PCREL: Self = Self(803);
    pub const BFD_RELOC_BFIN_16_LOW: Self = Self(804);
    pub const BFD_RELOC_BFIN_10_PCREL: Self = Self(805);
    pub const BFD_RELOC_BFIN_11_PCREL: Self = Self(806);
    pub const BFD_RELOC_BFIN_12_PCREL_JUMP: Self = Self(807);
    pub const BFD_RELOC_BFIN_12_PCREL_JUMP_S: Self = Self(808);
    pub const BFD_RELOC_BFIN_24_PCREL_CALL_X: Self = Self(809);
    pub const BFD_RELOC_BFIN_24_PCREL_JUMP_L: Self = Self(810);
    pub const BFD_RELOC_BFIN_GOT17M4: Self = Self(811);
    pub const BFD_RELOC_BFIN_GOTHI: Self = Self(812);
    pub const BFD_RELOC_BFIN_GOTLO: Self = Self(813);
    pub const BFD_RELOC_BFIN_FUNCDESC: Self = Self(814);
    pub const BFD_RELOC_BFIN_FUNCDESC_GOT17M4: Self = Self(815);
    pub const BFD_RELOC_BFIN_FUNCDESC_GOTHI: Self = Self(816);
    pub const BFD_RELOC_BFIN_FUNCDESC_GOTLO: Self = Self(817);
    pub const BFD_RELOC_BFIN_FUNCDESC_VALUE: Self = Self(818);
    pub const BFD_RELOC_BFIN_FUNCDESC_GOTOFF17M4: Self = Self(819);
    pub const BFD_RELOC_BFIN_FUNCDESC_GOTOFFHI: Self = Self(820);
    pub const BFD_RELOC_BFIN_FUNCDESC_GOTOFFLO: Self = Self(821);
    pub const BFD_RELOC_BFIN_GOTOFF17M4: Self = Self(822);
    pub const BFD_RELOC_BFIN_GOT: Self = Self(823);
    pub const BFD_RELOC_BFIN_PLTPC: Self = Self(824);
    pub const BFD_ARELOC_BFIN_PUSH: Self = Self(825);
    pub const BFD_ARELOC_BFIN_CONST: Self = Self(826);
    pub const BFD_ARELOC_BFIN_ADD: Self = Self(827);
    pub const BFD_ARELOC_BFIN_SUB: Self = Self(828);
    pub const BFD_ARELOC_BFIN_MULT: Self = Self(829);
    pub const BFD_ARELOC_BFIN_DIV: Self = Self(830);
    pub const BFD_ARELOC_BFIN_MOD: Self = Self(831);
    pub const BFD_ARELOC_BFIN_LSHIFT: Self = Self(832);
    pub const BFD_ARELOC_BFIN_RSHIFT: Self = Self(833);
    pub const BFD_ARELOC_BFIN_AND: Self = Self(834);
    pub const BFD_ARELOC_BFIN_OR: Self = Self(835);
    pub const BFD_ARELOC_BFIN_XOR: Self = Self(836);
    pub const BFD_ARELOC_BFIN_LAND: Self = Self(837);
    pub const BFD_ARELOC_BFIN_LOR: Self = Self(838);
    pub const BFD_ARELOC_BFIN_LEN: Self = Self(839);
    pub const BFD_ARELOC_BFIN_NEG: Self = Self(840);
    pub const BFD_ARELOC_BFIN_COMP: Self = Self(841);
    pub const BFD_ARELOC_BFIN_PAGE: Self = Self(842);
    pub const BFD_ARELOC_BFIN_HWPAGE: Self = Self(843);
    pub const BFD_ARELOC_BFIN_ADDR: Self = Self(844);
    pub const BFD_RELOC_D10V_10_PCREL_R: Self = Self(845);
    pub const BFD_RELOC_D10V_10_PCREL_L: Self = Self(846);
    pub const BFD_RELOC_D10V_18: Self = Self(847);
    pub const BFD_RELOC_D10V_18_PCREL: Self = Self(848);
    pub const BFD_RELOC_D30V_6: Self = Self(849);
    pub const BFD_RELOC_D30V_9_PCREL: Self = Self(850);
    pub const BFD_RELOC_D30V_9_PCREL_R: Self = Self(851);
    pub const BFD_RELOC_D30V_15: Self = Self(852);
    pub const BFD_RELOC_D30V_15_PCREL: Self = Self(853);
    pub const BFD_RELOC_D30V_15_PCREL_R: Self = Self(854);
    pub const BFD_RELOC_D30V_21: Self = Self(855);
    pub const BFD_RELOC_D30V_21_PCREL: Self = Self(856);
    pub const BFD_RELOC_D30V_21_PCREL_R: Self = Self(857);
    pub const BFD_RELOC_D30V_32: Self = Self(858);
    pub const BFD_RELOC_D30V_32_PCREL: Self = Self(859);
    pub const BFD_RELOC_DLX_HI16_S: Self = Self(860);
    pub const BFD_RELOC_DLX_LO16: Self = Self(861);
    pub const BFD_RELOC_DLX_JMP26: Self = Self(862);
    pub const BFD_RELOC_M32C_HI8: Self = Self(863);
    pub const BFD_RELOC_M32C_RL_JUMP: Self = Self(864);
    pub const BFD_RELOC_M32C_RL_1ADDR: Self = Self(865);
    pub const BFD_RELOC_M32C_RL_2ADDR: Self = Self(866);
    pub const BFD_RELOC_M32R_24: Self = Self(867);
    pub const BFD_RELOC_M32R_10_PCREL: Self = Self(868);
    pub const BFD_RELOC_M32R_18_PCREL: Self = Self(869);
    pub const BFD_RELOC_M32R_26_PCREL: Self = Self(870);
    pub const BFD_RELOC_M32R_HI16_ULO: Self = Self(871);
    pub const BFD_RELOC_M32R_HI16_SLO: Self = Self(872);
    pub const BFD_RELOC_M32R_LO16: Self = Self(873);
    pub const BFD_RELOC_M32R_SDA16: Self = Self(874);
    pub const BFD_RELOC_M32R_GOT24: Self = Self(875);
    pub const BFD_RELOC_M32R_26_PLTREL: Self = Self(876);
    pub const BFD_RELOC_M32R_GOTOFF: Self = Self(877);
    pub const BFD_RELOC_M32R_GOTPC24: Self = Self(878);
    pub const BFD_RELOC_M32R_GOT16_HI_ULO: Self = Self(879);
    pub const BFD_RELOC_M32R_GOT16_HI_SLO: Self = Self(880);
    pub const BFD_RELOC_M32R_GOT16_LO: Self = Self(881);
    pub const BFD_RELOC_M32R_GOTPC_HI_ULO: Self = Self(882);
    pub const BFD_RELOC_M32R_GOTPC_HI_SLO: Self = Self(883);
    pub const BFD_RELOC_M32R_GOTPC_LO: Self = Self(884);
    pub const BFD_RELOC_NDS32_20: Self = Self(885);
    pub const BFD_RELOC_NDS32_9_PCREL: Self = Self(886);
    pub const BFD_RELOC_NDS32_WORD_9_PCREL: Self = Self(887);
    pub const BFD_RELOC_NDS32_15_PCREL: Self = Self(888);
    pub const BFD_RELOC_NDS32_17_PCREL: Self = Self(889);
    pub const BFD_RELOC_NDS32_25_PCREL: Self = Self(890);
    pub const BFD_RELOC_NDS32_HI20: Self = Self(891);
    pub const BFD_RELOC_NDS32_LO12S3: Self = Self(892);
    pub const BFD_RELOC_NDS32_LO12S2: Self = Self(893);
    pub const BFD_RELOC_NDS32_LO12S1: Self = Self(894);
    pub const BFD_RELOC_NDS32_LO12S0: Self = Self(895);
    pub const BFD_RELOC_NDS32_LO12S0_ORI: Self = Self(896);
    pub const BFD_RELOC_NDS32_SDA15S3: Self = Self(897);
    pub const BFD_RELOC_NDS32_SDA15S2: Self = Self(898);
    pub const BFD_RELOC_NDS32_SDA15S1: Self = Self(899);
    pub const BFD_RELOC_NDS32_SDA15S0: Self = Self(900);
    pub const BFD_RELOC_NDS32_SDA16S3: Self = Self(901);
    pub const BFD_RELOC_NDS32_SDA17S2: Self = Self(902);
    pub const BFD_RELOC_NDS32_SDA18S1: Self = Self(903);
    pub const BFD_RELOC_NDS32_SDA19S0: Self = Self(904);
    pub const BFD_RELOC_NDS32_GOT20: Self = Self(905);
    pub const BFD_RELOC_NDS32_9_PLTREL: Self = Self(906);
    pub const BFD_RELOC_NDS32_25_PLTREL: Self = Self(907);
    pub const BFD_RELOC_NDS32_GOTOFF: Self = Self(908);
    pub const BFD_RELOC_NDS32_GOTOFF_HI20: Self = Self(909);
    pub const BFD_RELOC_NDS32_GOTOFF_LO12: Self = Self(910);
    pub const BFD_RELOC_NDS32_GOTPC20: Self = Self(911);
    pub const BFD_RELOC_NDS32_GOT_HI20: Self = Self(912);
    pub const BFD_RELOC_NDS32_GOT_LO12: Self = Self(913);
    pub const BFD_RELOC_NDS32_GOTPC_HI20: Self = Self(914);
    pub const BFD_RELOC_NDS32_GOTPC_LO12: Self = Self(915);
    pub const BFD_RELOC_NDS32_INSN16: Self = Self(916);
    pub const BFD_RELOC_NDS32_LABEL: Self = Self(917);
    pub const BFD_RELOC_NDS32_LONGCALL1: Self = Self(918);
    pub const BFD_RELOC_NDS32_LONGCALL2: Self = Self(919);
    pub const BFD_RELOC_NDS32_LONGCALL3: Self = Self(920);
    pub const BFD_RELOC_NDS32_LONGJUMP1: Self = Self(921);
    pub const BFD_RELOC_NDS32_LONGJUMP2: Self = Self(922);
    pub const BFD_RELOC_NDS32_LONGJUMP3: Self = Self(923);
    pub const BFD_RELOC_NDS32_LOADSTORE: Self = Self(924);
    pub const BFD_RELOC_NDS32_9_FIXED: Self = Self(925);
    pub const BFD_RELOC_NDS32_15_FIXED: Self = Self(926);
    pub const BFD_RELOC_NDS32_17_FIXED: Self = Self(927);
    pub const BFD_RELOC_NDS32_25_FIXED: Self = Self(928);
    pub const BFD_RELOC_NDS32_LONGCALL4: Self = Self(929);
    pub const BFD_RELOC_NDS32_LONGCALL5: Self = Self(930);
    pub const BFD_RELOC_NDS32_LONGCALL6: Self = Self(931);
    pub const BFD_RELOC_NDS32_LONGJUMP4: Self = Self(932);
    pub const BFD_RELOC_NDS32_LONGJUMP5: Self = Self(933);
    pub const BFD_RELOC_NDS32_LONGJUMP6: Self = Self(934);
    pub const BFD_RELOC_NDS32_LONGJUMP7: Self = Self(935);
    pub const BFD_RELOC_NDS32_PLTREL_HI20: Self = Self(936);
    pub const BFD_RELOC_NDS32_PLTREL_LO12: Self = Self(937);
    pub const BFD_RELOC_NDS32_PLT_GOTREL_HI20: Self = Self(938);
    pub const BFD_RELOC_NDS32_PLT_GOTREL_LO12: Self = Self(939);
    pub const BFD_RELOC_NDS32_SDA12S2_DP: Self = Self(940);
    pub const BFD_RELOC_NDS32_SDA12S2_SP: Self = Self(941);
    pub const BFD_RELOC_NDS32_LO12S2_DP: Self = Self(942);
    pub const BFD_RELOC_NDS32_LO12S2_SP: Self = Self(943);
    pub const BFD_RELOC_NDS32_DWARF2_OP1: Self = Self(944);
    pub const BFD_RELOC_NDS32_DWARF2_OP2: Self = Self(945);
    pub const BFD_RELOC_NDS32_DWARF2_LEB: Self = Self(946);
    pub const BFD_RELOC_NDS32_UPDATE_TA: Self = Self(947);
    pub const BFD_RELOC_NDS32_PLT_GOTREL_LO20: Self = Self(948);
    pub const BFD_RELOC_NDS32_PLT_GOTREL_LO15: Self = Self(949);
    pub const BFD_RELOC_NDS32_PLT_GOTREL_LO19: Self = Self(950);
    pub const BFD_RELOC_NDS32_GOT_LO15: Self = Self(951);
    pub const BFD_RELOC_NDS32_GOT_LO19: Self = Self(952);
    pub const BFD_RELOC_NDS32_GOTOFF_LO15: Self = Self(953);
    pub const BFD_RELOC_NDS32_GOTOFF_LO19: Self = Self(954);
    pub const BFD_RELOC_NDS32_GOT15S2: Self = Self(955);
    pub const BFD_RELOC_NDS32_GOT17S2: Self = Self(956);
    pub const BFD_RELOC_NDS32_5: Self = Self(957);
    pub const BFD_RELOC_NDS32_10_UPCREL: Self = Self(958);
    pub const BFD_RELOC_NDS32_SDA_FP7U2_RELA: Self = Self(959);
    pub const BFD_RELOC_NDS32_RELAX_ENTRY: Self = Self(960);
    pub const BFD_RELOC_NDS32_GOT_SUFF: Self = Self(961);
    pub const BFD_RELOC_NDS32_GOTOFF_SUFF: Self = Self(962);
    pub const BFD_RELOC_NDS32_PLT_GOT_SUFF: Self = Self(963);
    pub const BFD_RELOC_NDS32_MULCALL_SUFF: Self = Self(964);
    pub const BFD_RELOC_NDS32_PTR: Self = Self(965);
    pub const BFD_RELOC_NDS32_PTR_COUNT: Self = Self(966);
    pub const BFD_RELOC_NDS32_PTR_RESOLVED: Self = Self(967);
    pub const BFD_RELOC_NDS32_PLTBLOCK: Self = Self(968);
    pub const BFD_RELOC_NDS32_RELAX_REGION_BEGIN: Self = Self(969);
    pub const BFD_RELOC_NDS32_RELAX_REGION_END: Self = Self(970);
    pub const BFD_RELOC_NDS32_MINUEND: Self = Self(971);
    pub const BFD_RELOC_NDS32_SUBTRAHEND: Self = Self(972);
    pub const BFD_RELOC_NDS32_DIFF8: Self = Self(973);
    pub const BFD_RELOC_NDS32_DIFF16: Self = Self(974);
    pub const BFD_RELOC_NDS32_DIFF32: Self = Self(975);
    pub const BFD_RELOC_NDS32_DIFF_ULEB128: Self = Self(976);
    pub const BFD_RELOC_NDS32_EMPTY: Self = Self(977);
    pub const BFD_RELOC_NDS32_25_ABS: Self = Self(978);
    pub const BFD_RELOC_NDS32_DATA: Self = Self(979);
    pub const BFD_RELOC_NDS32_TRAN: Self = Self(980);
    pub const BFD_RELOC_NDS32_17IFC_PCREL: Self = Self(981);
    pub const BFD_RELOC_NDS32_10IFCU_PCREL: Self = Self(982);
    pub const BFD_RELOC_NDS32_TPOFF: Self = Self(983);
    pub const BFD_RELOC_NDS32_GOTTPOFF: Self = Self(984);
    pub const BFD_RELOC_NDS32_TLS_LE_HI20: Self = Self(985);
    pub const BFD_RELOC_NDS32_TLS_LE_LO12: Self = Self(986);
    pub const BFD_RELOC_NDS32_TLS_LE_20: Self = Self(987);
    pub const BFD_RELOC_NDS32_TLS_LE_15S0: Self = Self(988);
    pub const BFD_RELOC_NDS32_TLS_LE_15S1: Self = Self(989);
    pub const BFD_RELOC_NDS32_TLS_LE_15S2: Self = Self(990);
    pub const BFD_RELOC_NDS32_TLS_LE_ADD: Self = Self(991);
    pub const BFD_RELOC_NDS32_TLS_LE_LS: Self = Self(992);
    pub const BFD_RELOC_NDS32_TLS_IE_HI20: Self = Self(993);
    pub const BFD_RELOC_NDS32_TLS_IE_LO12: Self = Self(994);
    pub const BFD_RELOC_NDS32_TLS_IE_LO12S2: Self = Self(995);
    pub const BFD_RELOC_NDS32_TLS_IEGP_HI20: Self = Self(996);
    pub const BFD_RELOC_NDS32_TLS_IEGP_LO12: Self = Self(997);
    pub const BFD_RELOC_NDS32_TLS_IEGP_LO12S2: Self = Self(998);
    pub const BFD_RELOC_NDS32_TLS_IEGP_LW: Self = Self(999);
    pub const BFD_RELOC_NDS32_TLS_DESC: Self = Self(1000);
    pub const BFD_RELOC_NDS32_TLS_DESC_HI20: Self = Self(1001);
    pub const BFD_RELOC_NDS32_TLS_DESC_LO12: Self = Self(1002);
    pub const BFD_RELOC_NDS32_TLS_DESC_20: Self = Self(1003);
    pub const BFD_RELOC_NDS32_TLS_DESC_SDA17S2: Self = Self(1004);
    pub const BFD_RELOC_NDS32_TLS_DESC_ADD: Self = Self(1005);
    pub const BFD_RELOC_NDS32_TLS_DESC_FUNC: Self = Self(1006);
    pub const BFD_RELOC_NDS32_TLS_DESC_CALL: Self = Self(1007);
    pub const BFD_RELOC_NDS32_TLS_DESC_MEM: Self = Self(1008);
    pub const BFD_RELOC_NDS32_REMOVE: Self = Self(1009);
    pub const BFD_RELOC_NDS32_GROUP: Self = Self(1010);
    pub const BFD_RELOC_NDS32_LSI: Self = Self(1011);
    pub const BFD_RELOC_V850_9_PCREL: Self = Self(1012);
    pub const BFD_RELOC_V850_22_PCREL: Self = Self(1013);
    pub const BFD_RELOC_V850_SDA_16_16_OFFSET: Self = Self(1014);
    pub const BFD_RELOC_V850_SDA_15_16_OFFSET: Self = Self(1015);
    pub const BFD_RELOC_V850_ZDA_16_16_OFFSET: Self = Self(1016);
    pub const BFD_RELOC_V850_ZDA_15_16_OFFSET: Self = Self(1017);
    pub const BFD_RELOC_V850_TDA_6_8_OFFSET: Self = Self(1018);
    pub const BFD_RELOC_V850_TDA_7_8_OFFSET: Self = Self(1019);
    pub const BFD_RELOC_V850_TDA_7_7_OFFSET: Self = Self(1020);
    pub const BFD_RELOC_V850_TDA_16_16_OFFSET: Self = Self(1021);
    pub const BFD_RELOC_V850_TDA_4_5_OFFSET: Self = Self(1022);
    pub const BFD_RELOC_V850_TDA_4_4_OFFSET: Self = Self(1023);
    pub const BFD_RELOC_V850_SDA_16_16_SPLIT_OFFSET: Self = Self(1024);
    pub const BFD_RELOC_V850_ZDA_16_16_SPLIT_OFFSET: Self = Self(1025);
    pub const BFD_RELOC_V850_CALLT_6_7_OFFSET: Self = Self(1026);
    pub const BFD_RELOC_V850_CALLT_16_16_OFFSET: Self = Self(1027);
    pub const BFD_RELOC_V850_LONGCALL: Self = Self(1028);
    pub const BFD_RELOC_V850_LONGJUMP: Self = Self(1029);
    pub const BFD_RELOC_V850_ALIGN: Self = Self(1030);
    pub const BFD_RELOC_V850_LO16_SPLIT_OFFSET: Self = Self(1031);
    pub const BFD_RELOC_V850_16_PCREL: Self = Self(1032);
    pub const BFD_RELOC_V850_17_PCREL: Self = Self(1033);
    pub const BFD_RELOC_V850_23: Self = Self(1034);
    pub const BFD_RELOC_V850_32_PCREL: Self = Self(1035);
    pub const BFD_RELOC_V850_32_ABS: Self = Self(1036);
    pub const BFD_RELOC_V850_16_SPLIT_OFFSET: Self = Self(1037);
    pub const BFD_RELOC_V850_16_S1: Self = Self(1038);
    pub const BFD_RELOC_V850_LO16_S1: Self = Self(1039);
    pub const BFD_RELOC_V850_CALLT_15_16_OFFSET: Self = Self(1040);
    pub const BFD_RELOC_V850_16_GOT: Self = Self(1041);
    pub const BFD_RELOC_V850_32_GOT: Self = Self(1042);
    pub const BFD_RELOC_V850_22_PLT_PCREL: Self = Self(1043);
    pub const BFD_RELOC_V850_32_PLT_PCREL: Self = Self(1044);
    pub const BFD_RELOC_V850_CODE: Self = Self(1045);
    pub const BFD_RELOC_V850_DATA: Self = Self(1046);
    pub const BFD_RELOC_TIC30_LDP: Self = Self(1047);
    pub const BFD_RELOC_TIC54X_PARTLS7: Self = Self(1048);
    pub const BFD_RELOC_TIC54X_PARTMS9: Self = Self(1049);
    pub const BFD_RELOC_TIC54X_23: Self = Self(1050);
    pub const BFD_RELOC_TIC54X_16_OF_23: Self = Self(1051);
    pub const BFD_RELOC_TIC54X_MS7_OF_23: Self = Self(1052);
    pub const BFD_RELOC_C6000_PCR_S21: Self = Self(1053);
    pub const BFD_RELOC_C6000_PCR_S12: Self = Self(1054);
    pub const BFD_RELOC_C6000_PCR_S10: Self = Self(1055);
    pub const BFD_RELOC_C6000_PCR_S7: Self = Self(1056);
    pub const BFD_RELOC_C6000_ABS_S16: Self = Self(1057);
    pub const BFD_RELOC_C6000_ABS_L16: Self = Self(1058);
    pub const BFD_RELOC_C6000_ABS_H16: Self = Self(1059);
    pub const BFD_RELOC_C6000_SBR_U15_B: Self = Self(1060);
    pub const BFD_RELOC_C6000_SBR_U15_H: Self = Self(1061);
    pub const BFD_RELOC_C6000_SBR_U15_W: Self = Self(1062);
    pub const BFD_RELOC_C6000_SBR_S16: Self = Self(1063);
    pub const BFD_RELOC_C6000_SBR_L16_B: Self = Self(1064);
    pub const BFD_RELOC_C6000_SBR_L16_H: Self = Self(1065);
    pub const BFD_RELOC_C6000_SBR_L16_W: Self = Self(1066);
    pub const BFD_RELOC_C6000_SBR_H16_B: Self = Self(1067);
    pub const BFD_RELOC_C6000_SBR_H16_H: Self = Self(1068);
    pub const BFD_RELOC_C6000_SBR_H16_W: Self = Self(1069);
    pub const BFD_RELOC_C6000_SBR_GOT_U15_W: Self = Self(1070);
    pub const BFD_RELOC_C6000_SBR_GOT_L16_W: Self = Self(1071);
    pub const BFD_RELOC_C6000_SBR_GOT_H16_W: Self = Self(1072);
    pub const BFD_RELOC_C6000_DSBT_INDEX: Self = Self(1073);
    pub const BFD_RELOC_C6000_PREL31: Self = Self(1074);
    pub const BFD_RELOC_C6000_EHTYPE: Self = Self(1075);
    pub const BFD_RELOC_C6000_PCR_H16: Self = Self(1076);
    pub const BFD_RELOC_C6000_PCR_L16: Self = Self(1077);
    pub const BFD_RELOC_C6000_ALIGN: Self = Self(1078);
    pub const BFD_RELOC_C6000_FPHEAD: Self = Self(1079);
    pub const BFD_RELOC_C6000_NOCMP: Self = Self(1080);
    pub const BFD_RELOC_FR30_48: Self = Self(1081);
    pub const BFD_RELOC_FR30_20: Self = Self(1082);
    pub const BFD_RELOC_FR30_6_IN_4: Self = Self(1083);
    pub const BFD_RELOC_FR30_8_IN_8: Self = Self(1084);
    pub const BFD_RELOC_FR30_9_IN_8: Self = Self(1085);
    pub const BFD_RELOC_FR30_10_IN_8: Self = Self(1086);
    pub const BFD_RELOC_FR30_9_PCREL: Self = Self(1087);
    pub const BFD_RELOC_FR30_12_PCREL: Self = Self(1088);
    pub const BFD_RELOC_MCORE_PCREL_IMM8BY4: Self = Self(1089);
    pub const BFD_RELOC_MCORE_PCREL_IMM11BY2: Self = Self(1090);
    pub const BFD_RELOC_MCORE_PCREL_IMM4BY2: Self = Self(1091);
    pub const BFD_RELOC_MCORE_PCREL_JSR_IMM11BY2: Self = Self(1092);
    pub const BFD_RELOC_MEP_8: Self = Self(1093);
    pub const BFD_RELOC_MEP_16: Self = Self(1094);
    pub const BFD_RELOC_MEP_32: Self = Self(1095);
    pub const BFD_RELOC_MEP_PCREL8A2: Self = Self(1096);
    pub const BFD_RELOC_MEP_PCREL12A2: Self = Self(1097);
    pub const BFD_RELOC_MEP_PCREL17A2: Self = Self(1098);
    pub const BFD_RELOC_MEP_PCREL24A2: Self = Self(1099);
    pub const BFD_RELOC_MEP_PCABS24A2: Self = Self(1100);
    pub const BFD_RELOC_MEP_LOW16: Self = Self(1101);
    pub const BFD_RELOC_MEP_HI16U: Self = Self(1102);
    pub const BFD_RELOC_MEP_HI16S: Self = Self(1103);
    pub const BFD_RELOC_MEP_GPREL: Self = Self(1104);
    pub const BFD_RELOC_MEP_TPREL: Self = Self(1105);
    pub const BFD_RELOC_MEP_TPREL7: Self = Self(1106);
    pub const BFD_RELOC_MEP_TPREL7A2: Self = Self(1107);
    pub const BFD_RELOC_MEP_TPREL7A4: Self = Self(1108);
    pub const BFD_RELOC_MEP_UIMM24: Self = Self(1109);
    pub const BFD_RELOC_MEP_ADDR24A4: Self = Self(1110);
    pub const BFD_RELOC_MEP_GNU_VTINHERIT: Self = Self(1111);
    pub const BFD_RELOC_MEP_GNU_VTENTRY: Self = Self(1112);
    pub const BFD_RELOC_METAG_HIADDR16: Self = Self(1113);
    pub const BFD_RELOC_METAG_LOADDR16: Self = Self(1114);
    pub const BFD_RELOC_METAG_RELBRANCH: Self = Self(1115);
    pub const BFD_RELOC_METAG_GETSETOFF: Self = Self(1116);
    pub const BFD_RELOC_METAG_HIOG: Self = Self(1117);
    pub const BFD_RELOC_METAG_LOOG: Self = Self(1118);
    pub const BFD_RELOC_METAG_REL8: Self = Self(1119);
    pub const BFD_RELOC_METAG_REL16: Self = Self(1120);
    pub const BFD_RELOC_METAG_GETSET_GOTOFF: Self = Self(1121);
    pub const BFD_RELOC_METAG_GETSET_GOT: Self = Self(1122);
    pub const BFD_RELOC_METAG_HI16_GOTPC: Self = Self(1123);
    pub const BFD_RELOC_METAG_LO16_GOTPC: Self = Self(1124);
    pub const BFD_RELOC_METAG_HI16_PLT: Self = Self(1125);
    pub const BFD_RELOC_METAG_LO16_PLT: Self = Self(1126);
    pub const BFD_RELOC_METAG_RELBRANCH_PLT: Self = Self(1127);
    pub const BFD_RELOC_METAG_PLT: Self = Self(1128);
    pub const BFD_RELOC_METAG_TLS_GD: Self = Self(1129);
    pub const BFD_RELOC_METAG_TLS_LDM: Self = Self(1130);
    pub const BFD_RELOC_METAG_TLS_LDO_HI16: Self = Self(1131);
    pub const BFD_RELOC_METAG_TLS_LDO_LO16: Self = Self(1132);
    pub const BFD_RELOC_METAG_TLS_LDO: Self = Self(1133);
    pub const BFD_RELOC_METAG_TLS_IE: Self = Self(1134);
    pub const BFD_RELOC_METAG_TLS_IENONPIC: Self = Self(1135);
    pub const BFD_RELOC_METAG_TLS_IENONPIC_HI16: Self = Self(1136);
    pub const BFD_RELOC_METAG_TLS_IENONPIC_LO16: Self = Self(1137);
    pub const BFD_RELOC_METAG_TLS_TPOFF: Self = Self(1138);
    pub const BFD_RELOC_METAG_TLS_DTPMOD: Self = Self(1139);
    pub const BFD_RELOC_METAG_TLS_DTPOFF: Self = Self(1140);
    pub const BFD_RELOC_METAG_TLS_LE: Self = Self(1141);
    pub const BFD_RELOC_METAG_TLS_LE_HI16: Self = Self(1142);
    pub const BFD_RELOC_METAG_TLS_LE_LO16: Self = Self(1143);
    pub const BFD_RELOC_MMIX_GETA: Self = Self(1144);
    pub const BFD_RELOC_MMIX_GETA_1: Self = Self(1145);
    pub const BFD_RELOC_MMIX_GETA_2: Self = Self(1146);
    pub const BFD_RELOC_MMIX_GETA_3: Self = Self(1147);
    pub const BFD_RELOC_MMIX_CBRANCH: Self = Self(1148);
    pub const BFD_RELOC_MMIX_CBRANCH_J: Self = Self(1149);
    pub const BFD_RELOC_MMIX_CBRANCH_1: Self = Self(1150);
    pub const BFD_RELOC_MMIX_CBRANCH_2: Self = Self(1151);
    pub const BFD_RELOC_MMIX_CBRANCH_3: Self = Self(1152);
    pub const BFD_RELOC_MMIX_PUSHJ: Self = Self(1153);
    pub const BFD_RELOC_MMIX_PUSHJ_1: Self = Self(1154);
    pub const BFD_RELOC_MMIX_PUSHJ_2: Self = Self(1155);
    pub const BFD_RELOC_MMIX_PUSHJ_3: Self = Self(1156);
    pub const BFD_RELOC_MMIX_PUSHJ_STUBBABLE: Self = Self(1157);
    pub const BFD_RELOC_MMIX_JMP: Self = Self(1158);
    pub const BFD_RELOC_MMIX_JMP_1: Self = Self(1159);
    pub const BFD_RELOC_MMIX_JMP_2: Self = Self(1160);
    pub const BFD_RELOC_MMIX_JMP_3: Self = Self(1161);
    pub const BFD_RELOC_MMIX_ADDR19: Self = Self(1162);
    pub const BFD_RELOC_MMIX_ADDR27: Self = Self(1163);
    pub const BFD_RELOC_MMIX_REG_OR_BYTE: Self = Self(1164);
    pub const BFD_RELOC_MMIX_REG: Self = Self(1165);
    pub const BFD_RELOC_MMIX_BASE_PLUS_OFFSET: Self = Self(1166);
    pub const BFD_RELOC_MMIX_LOCAL: Self = Self(1167);
    pub const BFD_RELOC_AVR_7_PCREL: Self = Self(1168);
    pub const BFD_RELOC_AVR_13_PCREL: Self = Self(1169);
    pub const BFD_RELOC_AVR_16_PM: Self = Self(1170);
    pub const BFD_RELOC_AVR_LO8_LDI: Self = Self(1171);
    pub const BFD_RELOC_AVR_HI8_LDI: Self = Self(1172);
    pub const BFD_RELOC_AVR_HH8_LDI: Self = Self(1173);
    pub const BFD_RELOC_AVR_MS8_LDI: Self = Self(1174);
    pub const BFD_RELOC_AVR_LO8_LDI_NEG: Self = Self(1175);
    pub const BFD_RELOC_AVR_HI8_LDI_NEG: Self = Self(1176);
    pub const BFD_RELOC_AVR_HH8_LDI_NEG: Self = Self(1177);
    pub const BFD_RELOC_AVR_MS8_LDI_NEG: Self = Self(1178);
    pub const BFD_RELOC_AVR_LO8_LDI_PM: Self = Self(1179);
    pub const BFD_RELOC_AVR_LO8_LDI_GS: Self = Self(1180);
    pub const BFD_RELOC_AVR_HI8_LDI_PM: Self = Self(1181);
    pub const BFD_RELOC_AVR_HI8_LDI_GS: Self = Self(1182);
    pub const BFD_RELOC_AVR_HH8_LDI_PM: Self = Self(1183);
    pub const BFD_RELOC_AVR_LO8_LDI_PM_NEG: Self = Self(1184);
    pub const BFD_RELOC_AVR_HI8_LDI_PM_NEG: Self = Self(1185);
    pub const BFD_RELOC_AVR_HH8_LDI_PM_NEG: Self = Self(1186);
    pub const BFD_RELOC_AVR_CALL: Self = Self(1187);
    pub const BFD_RELOC_AVR_LDI: Self = Self(1188);
    pub const BFD_RELOC_AVR_6: Self = Self(1189);
    pub const BFD_RELOC_AVR_6_ADIW: Self = Self(1190);
    pub const BFD_RELOC_AVR_8_LO: Self = Self(1191);
    pub const BFD_RELOC_AVR_8_HI: Self = Self(1192);
    pub const BFD_RELOC_AVR_8_HLO: Self = Self(1193);
    pub const BFD_RELOC_AVR_DIFF8: Self = Self(1194);
    pub const BFD_RELOC_AVR_DIFF16: Self = Self(1195);
    pub const BFD_RELOC_AVR_DIFF32: Self = Self(1196);
    pub const BFD_RELOC_AVR_LDS_STS_16: Self = Self(1197);
    pub const BFD_RELOC_AVR_PORT6: Self = Self(1198);
    pub const BFD_RELOC_AVR_PORT5: Self = Self(1199);
    pub const BFD_RELOC_RISCV_HI20: Self = Self(1200);
    pub const BFD_RELOC_RISCV_PCREL_HI20: Self = Self(1201);
    pub const BFD_RELOC_RISCV_PCREL_LO12_I: Self = Self(1202);
    pub const BFD_RELOC_RISCV_PCREL_LO12_S: Self = Self(1203);
    pub const BFD_RELOC_RISCV_LO12_I: Self = Self(1204);
    pub const BFD_RELOC_RISCV_LO12_S: Self = Self(1205);
    pub const BFD_RELOC_RISCV_GPREL12_I: Self = Self(1206);
    pub const BFD_RELOC_RISCV_GPREL12_S: Self = Self(1207);
    pub const BFD_RELOC_RISCV_TPREL_HI20: Self = Self(1208);
    pub const BFD_RELOC_RISCV_TPREL_LO12_I: Self = Self(1209);
    pub const BFD_RELOC_RISCV_TPREL_LO12_S: Self = Self(1210);
    pub const BFD_RELOC_RISCV_TPREL_ADD: Self = Self(1211);
    pub const BFD_RELOC_RISCV_CALL: Self = Self(1212);
    pub const BFD_RELOC_RISCV_CALL_PLT: Self = Self(1213);
    pub const BFD_RELOC_RISCV_ADD8: Self = Self(1214);
    pub const BFD_RELOC_RISCV_ADD16: Self = Self(1215);
    pub const BFD_RELOC_RISCV_ADD32: Self = Self(1216);
    pub const BFD_RELOC_RISCV_ADD64: Self = Self(1217);
    pub const BFD_RELOC_RISCV_SUB8: Self = Self(1218);
    pub const BFD_RELOC_RISCV_SUB16: Self = Self(1219);
    pub const BFD_RELOC_RISCV_SUB32: Self = Self(1220);
    pub const BFD_RELOC_RISCV_SUB64: Self = Self(1221);
    pub const BFD_RELOC_RISCV_GOT_HI20: Self = Self(1222);
    pub const BFD_RELOC_RISCV_TLS_GOT_HI20: Self = Self(1223);
    pub const BFD_RELOC_RISCV_TLS_GD_HI20: Self = Self(1224);
    pub const BFD_RELOC_RISCV_JMP: Self = Self(1225);
    pub const BFD_RELOC_RISCV_TLS_DTPMOD32: Self = Self(1226);
    pub const BFD_RELOC_RISCV_TLS_DTPREL32: Self = Self(1227);
    pub const BFD_RELOC_RISCV_TLS_DTPMOD64: Self = Self(1228);
    pub const BFD_RELOC_RISCV_TLS_DTPREL64: Self = Self(1229);
    pub const BFD_RELOC_RISCV_TLS_TPREL32: Self = Self(1230);
    pub const BFD_RELOC_RISCV_TLS_TPREL64: Self = Self(1231);
    pub const BFD_RELOC_RISCV_TLSDESC_HI20: Self = Self(1232);
    pub const BFD_RELOC_RISCV_TLSDESC_LOAD_LO12: Self = Self(1233);
    pub const BFD_RELOC_RISCV_TLSDESC_ADD_LO12: Self = Self(1234);
    pub const BFD_RELOC_RISCV_TLSDESC_CALL: Self = Self(1235);
    pub const BFD_RELOC_RISCV_ALIGN: Self = Self(1236);
    pub const BFD_RELOC_RISCV_RVC_BRANCH: Self = Self(1237);
    pub const BFD_RELOC_RISCV_RVC_JUMP: Self = Self(1238);
    pub const BFD_RELOC_RISCV_RELAX: Self = Self(1239);
    pub const BFD_RELOC_RISCV_CFA: Self = Self(1240);
    pub const BFD_RELOC_RISCV_SUB6: Self = Self(1241);
    pub const BFD_RELOC_RISCV_SET6: Self = Self(1242);
    pub const BFD_RELOC_RISCV_SET8: Self = Self(1243);
    pub const BFD_RELOC_RISCV_SET16: Self = Self(1244);
    pub const BFD_RELOC_RISCV_SET32: Self = Self(1245);
    pub const BFD_RELOC_RISCV_SET_ULEB128: Self = Self(1246);
    pub const BFD_RELOC_RISCV_SUB_ULEB128: Self = Self(1247);
    pub const BFD_RELOC_RL78_NEG8: Self = Self(1248);
    pub const BFD_RELOC_RL78_NEG16: Self = Self(1249);
    pub const BFD_RELOC_RL78_NEG24: Self = Self(1250);
    pub const BFD_RELOC_RL78_NEG32: Self = Self(1251);
    pub const BFD_RELOC_RL78_16_OP: Self = Self(1252);
    pub const BFD_RELOC_RL78_24_OP: Self = Self(1253);
    pub const BFD_RELOC_RL78_32_OP: Self = Self(1254);
    pub const BFD_RELOC_RL78_8U: Self = Self(1255);
    pub const BFD_RELOC_RL78_16U: Self = Self(1256);
    pub const BFD_RELOC_RL78_24U: Self = Self(1257);
    pub const BFD_RELOC_RL78_DIR3U_PCREL: Self = Self(1258);
    pub const BFD_RELOC_RL78_DIFF: Self = Self(1259);
    pub const BFD_RELOC_RL78_GPRELB: Self = Self(1260);
    pub const BFD_RELOC_RL78_GPRELW: Self = Self(1261);
    pub const BFD_RELOC_RL78_GPRELL: Self = Self(1262);
    pub const BFD_RELOC_RL78_SYM: Self = Self(1263);
    pub const BFD_RELOC_RL78_OP_SUBTRACT: Self = Self(1264);
    pub const BFD_RELOC_RL78_OP_NEG: Self = Self(1265);
    pub const BFD_RELOC_RL78_OP_AND: Self = Self(1266);
    pub const BFD_RELOC_RL78_OP_SHRA: Self = Self(1267);
    pub const BFD_RELOC_RL78_ABS8: Self = Self(1268);
    pub const BFD_RELOC_RL78_ABS16: Self = Self(1269);
    pub const BFD_RELOC_RL78_ABS16_REV: Self = Self(1270);
    pub const BFD_RELOC_RL78_ABS32: Self = Self(1271);
    pub const BFD_RELOC_RL78_ABS32_REV: Self = Self(1272);
    pub const BFD_RELOC_RL78_ABS16U: Self = Self(1273);
    pub const BFD_RELOC_RL78_ABS16UW: Self = Self(1274);
    pub const BFD_RELOC_RL78_ABS16UL: Self = Self(1275);
    pub const BFD_RELOC_RL78_RELAX: Self = Self(1276);
    pub const BFD_RELOC_RL78_HI16: Self = Self(1277);
    pub const BFD_RELOC_RL78_HI8: Self = Self(1278);
    pub const BFD_RELOC_RL78_LO16: Self = Self(1279);
    pub const BFD_RELOC_RL78_CODE: Self = Self(1280);
    pub const BFD_RELOC_RL78_SADDR: Self = Self(1281);
    pub const BFD_RELOC_RX_NEG8: Self = Self(1282);
    pub const BFD_RELOC_RX_NEG16: Self = Self(1283);
    pub const BFD_RELOC_RX_NEG24: Self = Self(1284);
    pub const BFD_RELOC_RX_NEG32: Self = Self(1285);
    pub const BFD_RELOC_RX_16_OP: Self = Self(1286);
    pub const BFD_RELOC_RX_24_OP: Self = Self(1287);
    pub const BFD_RELOC_RX_32_OP: Self = Self(1288);
    pub const BFD_RELOC_RX_8U: Self = Self(1289);
    pub const BFD_RELOC_RX_16U: Self = Self(1290);
    pub const BFD_RELOC_RX_24U: Self = Self(1291);
    pub const BFD_RELOC_RX_DIR3U_PCREL: Self = Self(1292);
    pub const BFD_RELOC_RX_DIFF: Self = Self(1293);
    pub const BFD_RELOC_RX_GPRELB: Self = Self(1294);
    pub const BFD_RELOC_RX_GPRELW: Self = Self(1295);
    pub const BFD_RELOC_RX_GPRELL: Self = Self(1296);
    pub const BFD_RELOC_RX_SYM: Self = Self(1297);
    pub const BFD_RELOC_RX_OP_SUBTRACT: Self = Self(1298);
    pub const BFD_RELOC_RX_OP_NEG: Self = Self(1299);
    pub const BFD_RELOC_RX_ABS8: Self = Self(1300);
    pub const BFD_RELOC_RX_ABS16: Self = Self(1301);
    pub const BFD_RELOC_RX_ABS16_REV: Self = Self(1302);
    pub const BFD_RELOC_RX_ABS32: Self = Self(1303);
    pub const BFD_RELOC_RX_ABS32_REV: Self = Self(1304);
    pub const BFD_RELOC_RX_ABS16U: Self = Self(1305);
    pub const BFD_RELOC_RX_ABS16UW: Self = Self(1306);
    pub const BFD_RELOC_RX_ABS16UL: Self = Self(1307);
    pub const BFD_RELOC_RX_RELAX: Self = Self(1308);
    pub const BFD_RELOC_390_12: Self = Self(1309);
    pub const BFD_RELOC_390_GOT12: Self = Self(1310);
    pub const BFD_RELOC_390_GOT16: Self = Self(1311);
    pub const BFD_RELOC_390_GOT32: Self = Self(1312);
    pub const BFD_RELOC_390_PC12DBL: Self = Self(1313);
    pub const BFD_RELOC_390_PLT12DBL: Self = Self(1314);
    pub const BFD_RELOC_390_PC16DBL: Self = Self(1315);
    pub const BFD_RELOC_390_PLT16DBL: Self = Self(1316);
    pub const BFD_RELOC_390_PC24DBL: Self = Self(1317);
    pub const BFD_RELOC_390_PLT24DBL: Self = Self(1318);
    pub const BFD_RELOC_390_PC32DBL: Self = Self(1319);
    pub const BFD_RELOC_390_PLT32DBL: Self = Self(1320);
    pub const BFD_RELOC_390_GOTPCDBL: Self = Self(1321);
    pub const BFD_RELOC_390_GOT64: Self = Self(1322);
    pub const BFD_RELOC_390_GOTENT: Self = Self(1323);
    pub const BFD_RELOC_390_GOTPLT12: Self = Self(1324);
    pub const BFD_RELOC_390_GOTPLT16: Self = Self(1325);
    pub const BFD_RELOC_390_GOTPLT32: Self = Self(1326);
    pub const BFD_RELOC_390_GOTPLT64: Self = Self(1327);
    pub const BFD_RELOC_390_GOTPLTENT: Self = Self(1328);
    pub const BFD_RELOC_390_PLTOFF16: Self = Self(1329);
    pub const BFD_RELOC_390_PLTOFF32: Self = Self(1330);
    pub const BFD_RELOC_390_PLTOFF64: Self = Self(1331);
    pub const BFD_RELOC_390_TLS_LOAD: Self = Self(1332);
    pub const BFD_RELOC_390_TLS_GDCALL: Self = Self(1333);
    pub const BFD_RELOC_390_TLS_LDCALL: Self = Self(1334);
    pub const BFD_RELOC_390_TLS_GD32: Self = Self(1335);
    pub const BFD_RELOC_390_TLS_GD64: Self = Self(1336);
    pub const BFD_RELOC_390_TLS_GOTIE12: Self = Self(1337);
    pub const BFD_RELOC_390_TLS_GOTIE32: Self = Self(1338);
    pub const BFD_RELOC_390_TLS_GOTIE64: Self = Self(1339);
    pub const BFD_RELOC_390_TLS_LDM32: Self = Self(1340);
    pub const BFD_RELOC_390_TLS_LDM64: Self = Self(1341);
    pub const BFD_RELOC_390_TLS_IE32: Self = Self(1342);
    pub const BFD_RELOC_390_TLS_IE64: Self = Self(1343);
    pub const BFD_RELOC_390_TLS_IEENT: Self = Self(1344);
    pub const BFD_RELOC_390_TLS_LE32: Self = Self(1345);
    pub const BFD_RELOC_390_TLS_LE64: Self = Self(1346);
    pub const BFD_RELOC_390_TLS_LDO32: Self = Self(1347);
    pub const BFD_RELOC_390_TLS_LDO64: Self = Self(1348);
    pub const BFD_RELOC_390_TLS_DTPMOD: Self = Self(1349);
    pub const BFD_RELOC_390_TLS_DTPOFF: Self = Self(1350);
    pub const BFD_RELOC_390_TLS_TPOFF: Self = Self(1351);
    pub const BFD_RELOC_390_20: Self = Self(1352);
    pub const BFD_RELOC_390_GOT20: Self = Self(1353);
    pub const BFD_RELOC_390_GOTPLT20: Self = Self(1354);
    pub const BFD_RELOC_390_TLS_GOTIE20: Self = Self(1355);
    pub const BFD_RELOC_SCORE_GPREL15: Self = Self(1356);
    pub const BFD_RELOC_SCORE_DUMMY2: Self = Self(1357);
    pub const BFD_RELOC_SCORE_JMP: Self = Self(1358);
    pub const BFD_RELOC_SCORE_BRANCH: Self = Self(1359);
    pub const BFD_RELOC_SCORE_IMM30: Self = Self(1360);
    pub const BFD_RELOC_SCORE_IMM32: Self = Self(1361);
    pub const BFD_RELOC_SCORE16_JMP: Self = Self(1362);
    pub const BFD_RELOC_SCORE16_BRANCH: Self = Self(1363);
    pub const BFD_RELOC_SCORE_BCMP: Self = Self(1364);
    pub const BFD_RELOC_SCORE_GOT15: Self = Self(1365);
    pub const BFD_RELOC_SCORE_GOT_LO16: Self = Self(1366);
    pub const BFD_RELOC_SCORE_CALL15: Self = Self(1367);
    pub const BFD_RELOC_SCORE_DUMMY_HI16: Self = Self(1368);
    pub const BFD_RELOC_IP2K_FR9: Self = Self(1369);
    pub const BFD_RELOC_IP2K_BANK: Self = Self(1370);
    pub const BFD_RELOC_IP2K_ADDR16CJP: Self = Self(1371);
    pub const BFD_RELOC_IP2K_PAGE3: Self = Self(1372);
    pub const BFD_RELOC_IP2K_LO8DATA: Self = Self(1373);
    pub const BFD_RELOC_IP2K_HI8DATA: Self = Self(1374);
    pub const BFD_RELOC_IP2K_EX8DATA: Self = Self(1375);
    pub const BFD_RELOC_IP2K_LO8INSN: Self = Self(1376);
    pub const BFD_RELOC_IP2K_HI8INSN: Self = Self(1377);
    pub const BFD_RELOC_IP2K_PC_SKIP: Self = Self(1378);
    pub const BFD_RELOC_IP2K_TEXT: Self = Self(1379);
    pub const BFD_RELOC_IP2K_FR_OFFSET: Self = Self(1380);
    pub const BFD_RELOC_VTABLE_INHERIT: Self = Self(1381);
    pub const BFD_RELOC_VTABLE_ENTRY: Self = Self(1382);
    pub const BFD_RELOC_IA64_IMM14: Self = Self(1383);
    pub const BFD_RELOC_IA64_IMM22: Self = Self(1384);
    pub const BFD_RELOC_IA64_IMM64: Self = Self(1385);
    pub const BFD_RELOC_IA64_DIR32MSB: Self = Self(1386);
    pub const BFD_RELOC_IA64_DIR32LSB: Self = Self(1387);
    pub const BFD_RELOC_IA64_DIR64MSB: Self = Self(1388);
    pub const BFD_RELOC_IA64_DIR64LSB: Self = Self(1389);
    pub const BFD_RELOC_IA64_GPREL22: Self = Self(1390);
    pub const BFD_RELOC_IA64_GPREL64I: Self = Self(1391);
    pub const BFD_RELOC_IA64_GPREL32MSB: Self = Self(1392);
    pub const BFD_RELOC_IA64_GPREL32LSB: Self = Self(1393);
    pub const BFD_RELOC_IA64_GPREL64MSB: Self = Self(1394);
    pub const BFD_RELOC_IA64_GPREL64LSB: Self = Self(1395);
    pub const BFD_RELOC_IA64_LTOFF22: Self = Self(1396);
    pub const BFD_RELOC_IA64_LTOFF64I: Self = Self(1397);
    pub const BFD_RELOC_IA64_PLTOFF22: Self = Self(1398);
    pub const BFD_RELOC_IA64_PLTOFF64I: Self = Self(1399);
    pub const BFD_RELOC_IA64_PLTOFF64MSB: Self = Self(1400);
    pub const BFD_RELOC_IA64_PLTOFF64LSB: Self = Self(1401);
    pub const BFD_RELOC_IA64_FPTR64I: Self = Self(1402);
    pub const BFD_RELOC_IA64_FPTR32MSB: Self = Self(1403);
    pub const BFD_RELOC_IA64_FPTR32LSB: Self = Self(1404);
    pub const BFD_RELOC_IA64_FPTR64MSB: Self = Self(1405);
    pub const BFD_RELOC_IA64_FPTR64LSB: Self = Self(1406);
    pub const BFD_RELOC_IA64_PCREL21B: Self = Self(1407);
    pub const BFD_RELOC_IA64_PCREL21BI: Self = Self(1408);
    pub const BFD_RELOC_IA64_PCREL21M: Self = Self(1409);
    pub const BFD_RELOC_IA64_PCREL21F: Self = Self(1410);
    pub const BFD_RELOC_IA64_PCREL22: Self = Self(1411);
    pub const BFD_RELOC_IA64_PCREL60B: Self = Self(1412);
    pub const BFD_RELOC_IA64_PCREL64I: Self = Self(1413);
    pub const BFD_RELOC_IA64_PCREL32MSB: Self = Self(1414);
    pub const BFD_RELOC_IA64_PCREL32LSB: Self = Self(1415);
    pub const BFD_RELOC_IA64_PCREL64MSB: Self = Self(1416);
    pub const BFD_RELOC_IA64_PCREL64LSB: Self = Self(1417);
    pub const BFD_RELOC_IA64_LTOFF_FPTR22: Self = Self(1418);
    pub const BFD_RELOC_IA64_LTOFF_FPTR64I: Self = Self(1419);
    pub const BFD_RELOC_IA64_LTOFF_FPTR32MSB: Self = Self(1420);
    pub const BFD_RELOC_IA64_LTOFF_FPTR32LSB: Self = Self(1421);
    pub const BFD_RELOC_IA64_LTOFF_FPTR64MSB: Self = Self(1422);
    pub const BFD_RELOC_IA64_LTOFF_FPTR64LSB: Self = Self(1423);
    pub const BFD_RELOC_IA64_SEGREL32MSB: Self = Self(1424);
    pub const BFD_RELOC_IA64_SEGREL32LSB: Self = Self(1425);
    pub const BFD_RELOC_IA64_SEGREL64MSB: Self = Self(1426);
    pub const BFD_RELOC_IA64_SEGREL64LSB: Self = Self(1427);
    pub const BFD_RELOC_IA64_SECREL32MSB: Self = Self(1428);
    pub const BFD_RELOC_IA64_SECREL32LSB: Self = Self(1429);
    pub const BFD_RELOC_IA64_SECREL64MSB: Self = Self(1430);
    pub const BFD_RELOC_IA64_SECREL64LSB: Self = Self(1431);
    pub const BFD_RELOC_IA64_REL32MSB: Self = Self(1432);
    pub const BFD_RELOC_IA64_REL32LSB: Self = Self(1433);
    pub const BFD_RELOC_IA64_REL64MSB: Self = Self(1434);
    pub const BFD_RELOC_IA64_REL64LSB: Self = Self(1435);
    pub const BFD_RELOC_IA64_LTV32MSB: Self = Self(1436);
    pub const BFD_RELOC_IA64_LTV32LSB: Self = Self(1437);
    pub const BFD_RELOC_IA64_LTV64MSB: Self = Self(1438);
    pub const BFD_RELOC_IA64_LTV64LSB: Self = Self(1439);
    pub const BFD_RELOC_IA64_IPLTMSB: Self = Self(1440);
    pub const BFD_RELOC_IA64_IPLTLSB: Self = Self(1441);
    pub const BFD_RELOC_IA64_LTOFF22X: Self = Self(1442);
    pub const BFD_RELOC_IA64_LDXMOV: Self = Self(1443);
    pub const BFD_RELOC_IA64_TPREL14: Self = Self(1444);
    pub const BFD_RELOC_IA64_TPREL22: Self = Self(1445);
    pub const BFD_RELOC_IA64_TPREL64I: Self = Self(1446);
    pub const BFD_RELOC_IA64_TPREL64MSB: Self = Self(1447);
    pub const BFD_RELOC_IA64_TPREL64LSB: Self = Self(1448);
    pub const BFD_RELOC_IA64_LTOFF_TPREL22: Self = Self(1449);
    pub const BFD_RELOC_IA64_DTPMOD64MSB: Self = Self(1450);
    pub const BFD_RELOC_IA64_DTPMOD64LSB: Self = Self(1451);
    pub const BFD_RELOC_IA64_LTOFF_DTPMOD22: Self = Self(1452);
    pub const BFD_RELOC_IA64_DTPREL14: Self = Self(1453);
    pub const BFD_RELOC_IA64_DTPREL22: Self = Self(1454);
    pub const BFD_RELOC_IA64_DTPREL64I: Self = Self(1455);
    pub const BFD_RELOC_IA64_DTPREL32MSB: Self = Self(1456);
    pub const BFD_RELOC_IA64_DTPREL32LSB: Self = Self(1457);
    pub const BFD_RELOC_IA64_DTPREL64MSB: Self = Self(1458);
    pub const BFD_RELOC_IA64_DTPREL64LSB: Self = Self(1459);
    pub const BFD_RELOC_IA64_LTOFF_DTPREL22: Self = Self(1460);
    pub const BFD_RELOC_M68HC11_HI8: Self = Self(1461);
    pub const BFD_RELOC_M68HC11_LO8: Self = Self(1462);
    pub const BFD_RELOC_M68HC11_3B: Self = Self(1463);
    pub const BFD_RELOC_M68HC11_RL_JUMP: Self = Self(1464);
    pub const BFD_RELOC_M68HC11_RL_GROUP: Self = Self(1465);
    pub const BFD_RELOC_M68HC11_LO16: Self = Self(1466);
    pub const BFD_RELOC_M68HC11_PAGE: Self = Self(1467);
    pub const BFD_RELOC_M68HC11_24: Self = Self(1468);
    pub const BFD_RELOC_M68HC12_5B: Self = Self(1469);
    pub const BFD_RELOC_XGATE_RL_JUMP: Self = Self(1470);
    pub const BFD_RELOC_XGATE_RL_GROUP: Self = Self(1471);
    pub const BFD_RELOC_XGATE_LO16: Self = Self(1472);
    pub const BFD_RELOC_XGATE_GPAGE: Self = Self(1473);
    pub const BFD_RELOC_XGATE_24: Self = Self(1474);
    pub const BFD_RELOC_XGATE_PCREL_9: Self = Self(1475);
    pub const BFD_RELOC_XGATE_PCREL_10: Self = Self(1476);
    pub const BFD_RELOC_XGATE_IMM8_LO: Self = Self(1477);
    pub const BFD_RELOC_XGATE_IMM8_HI: Self = Self(1478);
    pub const BFD_RELOC_XGATE_IMM3: Self = Self(1479);
    pub const BFD_RELOC_XGATE_IMM4: Self = Self(1480);
    pub const BFD_RELOC_XGATE_IMM5: Self = Self(1481);
    pub const BFD_RELOC_M68HC12_9B: Self = Self(1482);
    pub const BFD_RELOC_M68HC12_16B: Self = Self(1483);
    pub const BFD_RELOC_M68HC12_9_PCREL: Self = Self(1484);
    pub const BFD_RELOC_M68HC12_10_PCREL: Self = Self(1485);
    pub const BFD_RELOC_M68HC12_LO8XG: Self = Self(1486);
    pub const BFD_RELOC_M68HC12_HI8XG: Self = Self(1487);
    pub const BFD_RELOC_CR16_NUM8: Self = Self(1488);
    pub const BFD_RELOC_CR16_NUM16: Self = Self(1489);
    pub const BFD_RELOC_CR16_NUM32: Self = Self(1490);
    pub const BFD_RELOC_CR16_NUM32a: Self = Self(1491);
    pub const BFD_RELOC_CR16_REGREL0: Self = Self(1492);
    pub const BFD_RELOC_CR16_REGREL4: Self = Self(1493);
    pub const BFD_RELOC_CR16_REGREL4a: Self = Self(1494);
    pub const BFD_RELOC_CR16_REGREL14: Self = Self(1495);
    pub const BFD_RELOC_CR16_REGREL14a: Self = Self(1496);
    pub const BFD_RELOC_CR16_REGREL16: Self = Self(1497);
    pub const BFD_RELOC_CR16_REGREL20: Self = Self(1498);
    pub const BFD_RELOC_CR16_REGREL20a: Self = Self(1499);
    pub const BFD_RELOC_CR16_ABS20: Self = Self(1500);
    pub const BFD_RELOC_CR16_ABS24: Self = Self(1501);
    pub const BFD_RELOC_CR16_IMM4: Self = Self(1502);
    pub const BFD_RELOC_CR16_IMM8: Self = Self(1503);
    pub const BFD_RELOC_CR16_IMM16: Self = Self(1504);
    pub const BFD_RELOC_CR16_IMM20: Self = Self(1505);
    pub const BFD_RELOC_CR16_IMM24: Self = Self(1506);
    pub const BFD_RELOC_CR16_IMM32: Self = Self(1507);
    pub const BFD_RELOC_CR16_IMM32a: Self = Self(1508);
    pub const BFD_RELOC_CR16_DISP4: Self = Self(1509);
    pub const BFD_RELOC_CR16_DISP8: Self = Self(1510);
    pub const BFD_RELOC_CR16_DISP16: Self = Self(1511);
    pub const BFD_RELOC_CR16_DISP20: Self = Self(1512);
    pub const BFD_RELOC_CR16_DISP24: Self = Self(1513);
    pub const BFD_RELOC_CR16_DISP24a: Self = Self(1514);
    pub const BFD_RELOC_CR16_SWITCH8: Self = Self(1515);
    pub const BFD_RELOC_CR16_SWITCH16: Self = Self(1516);
    pub const BFD_RELOC_CR16_SWITCH32: Self = Self(1517);
    pub const BFD_RELOC_CR16_GOT_REGREL20: Self = Self(1518);
    pub const BFD_RELOC_CR16_GOTC_REGREL20: Self = Self(1519);
    pub const BFD_RELOC_CRX_REL4: Self = Self(1520);
    pub const BFD_RELOC_CRX_REL8: Self = Self(1521);
    pub const BFD_RELOC_CRX_REL8_CMP: Self = Self(1522);
    pub const BFD_RELOC_CRX_REL16: Self = Self(1523);
    pub const BFD_RELOC_CRX_REL24: Self = Self(1524);
    pub const BFD_RELOC_CRX_REL32: Self = Self(1525);
    pub const BFD_RELOC_CRX_REGREL12: Self = Self(1526);
    pub const BFD_RELOC_CRX_REGREL22: Self = Self(1527);
    pub const BFD_RELOC_CRX_REGREL28: Self = Self(1528);
    pub const BFD_RELOC_CRX_REGREL32: Self = Self(1529);
    pub const BFD_RELOC_CRX_ABS16: Self = Self(1530);
    pub const BFD_RELOC_CRX_ABS32: Self = Self(1531);
    pub const BFD_RELOC_CRX_NUM8: Self = Self(1532);
    pub const BFD_RELOC_CRX_NUM16: Self = Self(1533);
    pub const BFD_RELOC_CRX_NUM32: Self = Self(1534);
    pub const BFD_RELOC_CRX_IMM16: Self = Self(1535);
    pub const BFD_RELOC_CRX_IMM32: Self = Self(1536);
    pub const BFD_RELOC_CRX_SWITCH8: Self = Self(1537);
    pub const BFD_RELOC_CRX_SWITCH16: Self = Self(1538);
    pub const BFD_RELOC_CRX_SWITCH32: Self = Self(1539);
    pub const BFD_RELOC_CRIS_BDISP8: Self = Self(1540);
    pub const BFD_RELOC_CRIS_UNSIGNED_5: Self = Self(1541);
    pub const BFD_RELOC_CRIS_SIGNED_6: Self = Self(1542);
    pub const BFD_RELOC_CRIS_UNSIGNED_6: Self = Self(1543);
    pub const BFD_RELOC_CRIS_SIGNED_8: Self = Self(1544);
    pub const BFD_RELOC_CRIS_UNSIGNED_8: Self = Self(1545);
    pub const BFD_RELOC_CRIS_SIGNED_16: Self = Self(1546);
    pub const BFD_RELOC_CRIS_UNSIGNED_16: Self = Self(1547);
    pub const BFD_RELOC_CRIS_LAPCQ_OFFSET: Self = Self(1548);
    pub const BFD_RELOC_CRIS_UNSIGNED_4: Self = Self(1549);
    pub const BFD_RELOC_CRIS_32_GOT: Self = Self(1550);
    pub const BFD_RELOC_CRIS_16_GOT: Self = Self(1551);
    pub const BFD_RELOC_CRIS_32_GOTPLT: Self = Self(1552);
    pub const BFD_RELOC_CRIS_16_GOTPLT: Self = Self(1553);
    pub const BFD_RELOC_CRIS_32_GOTREL: Self = Self(1554);
    pub const BFD_RELOC_CRIS_32_PLT_GOTREL: Self = Self(1555);
    pub const BFD_RELOC_CRIS_32_GOT_GD: Self = Self(1556);
    pub const BFD_RELOC_CRIS_16_GOT_GD: Self = Self(1557);
    pub const BFD_RELOC_CRIS_32_GD: Self = Self(1558);
    pub const BFD_RELOC_CRIS_DTP: Self = Self(1559);
    pub const BFD_RELOC_CRIS_32_DTPREL: Self = Self(1560);
    pub const BFD_RELOC_CRIS_16_DTPREL: Self = Self(1561);
    pub const BFD_RELOC_CRIS_32_GOT_TPREL: Self = Self(1562);
    pub const BFD_RELOC_CRIS_16_GOT_TPREL: Self = Self(1563);
    pub const BFD_RELOC_CRIS_32_TPREL: Self = Self(1564);
    pub const BFD_RELOC_CRIS_16_TPREL: Self = Self(1565);
    pub const BFD_RELOC_CRIS_DTPMOD: Self = Self(1566);
    pub const BFD_RELOC_CRIS_32_IE: Self = Self(1567);
    pub const BFD_RELOC_OR1K_REL_26: Self = Self(1568);
    pub const BFD_RELOC_OR1K_SLO16: Self = Self(1569);
    pub const BFD_RELOC_OR1K_PCREL_PG21: Self = Self(1570);
    pub const BFD_RELOC_OR1K_LO13: Self = Self(1571);
    pub const BFD_RELOC_OR1K_SLO13: Self = Self(1572);
    pub const BFD_RELOC_OR1K_GOTPC_HI16: Self = Self(1573);
    pub const BFD_RELOC_OR1K_GOTPC_LO16: Self = Self(1574);
    pub const BFD_RELOC_OR1K_GOT_AHI16: Self = Self(1575);
    pub const BFD_RELOC_OR1K_GOT16: Self = Self(1576);
    pub const BFD_RELOC_OR1K_GOT_PG21: Self = Self(1577);
    pub const BFD_RELOC_OR1K_GOT_LO13: Self = Self(1578);
    pub const BFD_RELOC_OR1K_PLT26: Self = Self(1579);
    pub const BFD_RELOC_OR1K_PLTA26: Self = Self(1580);
    pub const BFD_RELOC_OR1K_GOTOFF_SLO16: Self = Self(1581);
    pub const BFD_RELOC_OR1K_TLS_GD_HI16: Self = Self(1582);
    pub const BFD_RELOC_OR1K_TLS_GD_LO16: Self = Self(1583);
    pub const BFD_RELOC_OR1K_TLS_GD_PG21: Self = Self(1584);
    pub const BFD_RELOC_OR1K_TLS_GD_LO13: Self = Self(1585);
    pub const BFD_RELOC_OR1K_TLS_LDM_HI16: Self = Self(1586);
    pub const BFD_RELOC_OR1K_TLS_LDM_LO16: Self = Self(1587);
    pub const BFD_RELOC_OR1K_TLS_LDM_PG21: Self = Self(1588);
    pub const BFD_RELOC_OR1K_TLS_LDM_LO13: Self = Self(1589);
    pub const BFD_RELOC_OR1K_TLS_LDO_HI16: Self = Self(1590);
    pub const BFD_RELOC_OR1K_TLS_LDO_LO16: Self = Self(1591);
    pub const BFD_RELOC_OR1K_TLS_IE_HI16: Self = Self(1592);
    pub const BFD_RELOC_OR1K_TLS_IE_AHI16: Self = Self(1593);
    pub const BFD_RELOC_OR1K_TLS_IE_LO16: Self = Self(1594);
    pub const BFD_RELOC_OR1K_TLS_IE_PG21: Self = Self(1595);
    pub const BFD_RELOC_OR1K_TLS_IE_LO13: Self = Self(1596);
    pub const BFD_RELOC_OR1K_TLS_LE_HI16: Self = Self(1597);
    pub const BFD_RELOC_OR1K_TLS_LE_AHI16: Self = Self(1598);
    pub const BFD_RELOC_OR1K_TLS_LE_LO16: Self = Self(1599);
    pub const BFD_RELOC_OR1K_TLS_LE_SLO16: Self = Self(1600);
    pub const BFD_RELOC_OR1K_TLS_TPOFF: Self = Self(1601);
    pub const BFD_RELOC_OR1K_TLS_DTPOFF: Self = Self(1602);
    pub const BFD_RELOC_OR1K_TLS_DTPMOD: Self = Self(1603);
    pub const BFD_RELOC_H8_DIR16A8: Self = Self(1604);
    pub const BFD_RELOC_H8_DIR16R8: Self = Self(1605);
    pub const BFD_RELOC_H8_DIR24A8: Self = Self(1606);
    pub const BFD_RELOC_H8_DIR24R8: Self = Self(1607);
    pub const BFD_RELOC_H8_DIR32A16: Self = Self(1608);
    pub const BFD_RELOC_H8_DISP32A16: Self = Self(1609);
    pub const BFD_RELOC_XSTORMY16_REL_12: Self = Self(1610);
    pub const BFD_RELOC_XSTORMY16_12: Self = Self(1611);
    pub const BFD_RELOC_XSTORMY16_24: Self = Self(1612);
    pub const BFD_RELOC_XSTORMY16_FPTR16: Self = Self(1613);
    pub const BFD_RELOC_RELC: Self = Self(1614);
    pub const BFD_RELOC_MT_PC16: Self = Self(1615);
    pub const BFD_RELOC_MT_HI16: Self = Self(1616);
    pub const BFD_RELOC_MT_LO16: Self = Self(1617);
    pub const BFD_RELOC_MT_GNU_VTINHERIT: Self = Self(1618);
    pub const BFD_RELOC_MT_GNU_VTENTRY: Self = Self(1619);
    pub const BFD_RELOC_MT_PCINSN8: Self = Self(1620);
    pub const BFD_RELOC_MSP430_10_PCREL: Self = Self(1621);
    pub const BFD_RELOC_MSP430_16_PCREL: Self = Self(1622);
    pub const BFD_RELOC_MSP430_16: Self = Self(1623);
    pub const BFD_RELOC_MSP430_2X_PCREL: Self = Self(1624);
    pub const BFD_RELOC_MSP430_RL_PCREL: Self = Self(1625);
    pub const BFD_RELOC_MSP430_ABS8: Self = Self(1626);
    pub const BFD_RELOC_MSP430X_PCR20_EXT_SRC: Self = Self(1627);
    pub const BFD_RELOC_MSP430X_PCR20_EXT_DST: Self = Self(1628);
    pub const BFD_RELOC_MSP430X_PCR20_EXT_ODST: Self = Self(1629);
    pub const BFD_RELOC_MSP430X_ABS20_EXT_SRC: Self = Self(1630);
    pub const BFD_RELOC_MSP430X_ABS20_EXT_DST: Self = Self(1631);
    pub const BFD_RELOC_MSP430X_ABS20_EXT_ODST: Self = Self(1632);
    pub const BFD_RELOC_MSP430X_ABS20_ADR_SRC: Self = Self(1633);
    pub const BFD_RELOC_MSP430X_ABS20_ADR_DST: Self = Self(1634);
    pub const BFD_RELOC_MSP430X_PCR16: Self = Self(1635);
    pub const BFD_RELOC_MSP430X_PCR20_CALL: Self = Self(1636);
    pub const BFD_RELOC_MSP430X_ABS16: Self = Self(1637);
    pub const BFD_RELOC_MSP430_ABS_HI16: Self = Self(1638);
    pub const BFD_RELOC_MSP430_PREL31: Self = Self(1639);
    pub const BFD_RELOC_MSP430_SYM_DIFF: Self = Self(1640);
    pub const BFD_RELOC_MSP430_SET_ULEB128: Self = Self(1641);
    pub const BFD_RELOC_MSP430_SUB_ULEB128: Self = Self(1642);
    pub const BFD_RELOC_PRU_U16: Self = Self(1643);
    pub const BFD_RELOC_PRU_U16_PMEMIMM: Self = Self(1644);
    pub const BFD_RELOC_PRU_LDI32: Self = Self(1645);
    pub const BFD_RELOC_PRU_S10_PCREL: Self = Self(1646);
    pub const BFD_RELOC_PRU_U8_PCREL: Self = Self(1647);
    pub const BFD_RELOC_PRU_32_PMEM: Self = Self(1648);
    pub const BFD_RELOC_PRU_16_PMEM: Self = Self(1649);
    pub const BFD_RELOC_PRU_GNU_DIFF8: Self = Self(1650);
    pub const BFD_RELOC_PRU_GNU_DIFF16: Self = Self(1651);
    pub const BFD_RELOC_PRU_GNU_DIFF32: Self = Self(1652);
    pub const BFD_RELOC_PRU_GNU_DIFF16_PMEM: Self = Self(1653);
    pub const BFD_RELOC_PRU_GNU_DIFF32_PMEM: Self = Self(1654);
    pub const BFD_RELOC_IQ2000_OFFSET_16: Self = Self(1655);
    pub const BFD_RELOC_IQ2000_OFFSET_21: Self = Self(1656);
    pub const BFD_RELOC_IQ2000_UHI16: Self = Self(1657);
    pub const BFD_RELOC_XTENSA_RTLD: Self = Self(1658);
    pub const BFD_RELOC_XTENSA_PLT: Self = Self(1659);
    pub const BFD_RELOC_XTENSA_DIFF8: Self = Self(1660);
    pub const BFD_RELOC_XTENSA_DIFF16: Self = Self(1661);
    pub const BFD_RELOC_XTENSA_DIFF32: Self = Self(1662);
    pub const BFD_RELOC_XTENSA_SLOT0_OP: Self = Self(1663);
    pub const BFD_RELOC_XTENSA_SLOT1_OP: Self = Self(1664);
    pub const BFD_RELOC_XTENSA_SLOT2_OP: Self = Self(1665);
    pub const BFD_RELOC_XTENSA_SLOT3_OP: Self = Self(1666);
    pub const BFD_RELOC_XTENSA_SLOT4_OP: Self = Self(1667);
    pub const BFD_RELOC_XTENSA_SLOT5_OP: Self = Self(1668);
    pub const BFD_RELOC_XTENSA_SLOT6_OP: Self = Self(1669);
    pub const BFD_RELOC_XTENSA_SLOT7_OP: Self = Self(1670);
    pub const BFD_RELOC_XTENSA_SLOT8_OP: Self = Self(1671);
    pub const BFD_RELOC_XTENSA_SLOT9_OP: Self = Self(1672);
    pub const BFD_RELOC_XTENSA_SLOT10_OP: Self = Self(1673);
    pub const BFD_RELOC_XTENSA_SLOT11_OP: Self = Self(1674);
    pub const BFD_RELOC_XTENSA_SLOT12_OP: Self = Self(1675);
    pub const BFD_RELOC_XTENSA_SLOT13_OP: Self = Self(1676);
    pub const BFD_RELOC_XTENSA_SLOT14_OP: Self = Self(1677);
    pub const BFD_RELOC_XTENSA_SLOT0_ALT: Self = Self(1678);
    pub const BFD_RELOC_XTENSA_SLOT1_ALT: Self = Self(1679);
    pub const BFD_RELOC_XTENSA_SLOT2_ALT: Self = Self(1680);
    pub const BFD_RELOC_XTENSA_SLOT3_ALT: Self = Self(1681);
    pub const BFD_RELOC_XTENSA_SLOT4_ALT: Self = Self(1682);
    pub const BFD_RELOC_XTENSA_SLOT5_ALT: Self = Self(1683);
    pub const BFD_RELOC_XTENSA_SLOT6_ALT: Self = Self(1684);
    pub const BFD_RELOC_XTENSA_SLOT7_ALT: Self = Self(1685);
    pub const BFD_RELOC_XTENSA_SLOT8_ALT: Self = Self(1686);
    pub const BFD_RELOC_XTENSA_SLOT9_ALT: Self = Self(1687);
    pub const BFD_RELOC_XTENSA_SLOT10_ALT: Self = Self(1688);
    pub const BFD_RELOC_XTENSA_SLOT11_ALT: Self = Self(1689);
    pub const BFD_RELOC_XTENSA_SLOT12_ALT: Self = Self(1690);
    pub const BFD_RELOC_XTENSA_SLOT13_ALT: Self = Self(1691);
    pub const BFD_RELOC_XTENSA_SLOT14_ALT: Self = Self(1692);
    pub const BFD_RELOC_XTENSA_OP0: Self = Self(1693);
    pub const BFD_RELOC_XTENSA_OP1: Self = Self(1694);
    pub const BFD_RELOC_XTENSA_OP2: Self = Self(1695);
    pub const BFD_RELOC_XTENSA_ASM_EXPAND: Self = Self(1696);
    pub const BFD_RELOC_XTENSA_ASM_SIMPLIFY: Self = Self(1697);
    pub const BFD_RELOC_XTENSA_TLSDESC_FN: Self = Self(1698);
    pub const BFD_RELOC_XTENSA_TLSDESC_ARG: Self = Self(1699);
    pub const BFD_RELOC_XTENSA_TLS_DTPOFF: Self = Self(1700);
    pub const BFD_RELOC_XTENSA_TLS_TPOFF: Self = Self(1701);
    pub const BFD_RELOC_XTENSA_TLS_FUNC: Self = Self(1702);
    pub const BFD_RELOC_XTENSA_TLS_ARG: Self = Self(1703);
    pub const BFD_RELOC_XTENSA_TLS_CALL: Self = Self(1704);
    pub const BFD_RELOC_XTENSA_PDIFF8: Self = Self(1705);
    pub const BFD_RELOC_XTENSA_PDIFF16: Self = Self(1706);
    pub const BFD_RELOC_XTENSA_PDIFF32: Self = Self(1707);
    pub const BFD_RELOC_XTENSA_NDIFF8: Self = Self(1708);
    pub const BFD_RELOC_XTENSA_NDIFF16: Self = Self(1709);
    pub const BFD_RELOC_XTENSA_NDIFF32: Self = Self(1710);
    pub const BFD_RELOC_Z80_DISP8: Self = Self(1711);
    pub const BFD_RELOC_Z80_BYTE0: Self = Self(1712);
    pub const BFD_RELOC_Z80_BYTE1: Self = Self(1713);
    pub const BFD_RELOC_Z80_BYTE2: Self = Self(1714);
    pub const BFD_RELOC_Z80_BYTE3: Self = Self(1715);
    pub const BFD_RELOC_Z80_WORD0: Self = Self(1716);
    pub const BFD_RELOC_Z80_WORD1: Self = Self(1717);
    pub const BFD_RELOC_Z80_16_BE: Self = Self(1718);
    pub const BFD_RELOC_Z8K_DISP7: Self = Self(1719);
    pub const BFD_RELOC_Z8K_CALLR: Self = Self(1720);
    pub const BFD_RELOC_Z8K_IMM4L: Self = Self(1721);
    pub const BFD_RELOC_LM32_CALL: Self = Self(1722);
    pub const BFD_RELOC_LM32_BRANCH: Self = Self(1723);
    pub const BFD_RELOC_LM32_16_GOT: Self = Self(1724);
    pub const BFD_RELOC_MACH_O_SECTDIFF: Self = Self(1725);
    pub const BFD_RELOC_MACH_O_LOCAL_SECTDIFF: Self = Self(1726);
    pub const BFD_RELOC_MACH_O_PAIR: Self = Self(1727);
    pub const BFD_RELOC_MACH_O_SUBTRACTOR32: Self = Self(1728);
    pub const BFD_RELOC_MACH_O_SUBTRACTOR64: Self = Self(1729);
    pub const BFD_RELOC_MACH_O_X86_64_BRANCH32: Self = Self(1730);
    pub const BFD_RELOC_MACH_O_X86_64_BRANCH8: Self = Self(1731);
    pub const BFD_RELOC_MACH_O_X86_64_GOT: Self = Self(1732);
    pub const BFD_RELOC_MACH_O_X86_64_GOT_LOAD: Self = Self(1733);
    pub const BFD_RELOC_MACH_O_X86_64_PCREL32_1: Self = Self(1734);
    pub const BFD_RELOC_MACH_O_X86_64_PCREL32_2: Self = Self(1735);
    pub const BFD_RELOC_MACH_O_X86_64_PCREL32_4: Self = Self(1736);
    pub const BFD_RELOC_MACH_O_X86_64_TLV: Self = Self(1737);
    pub const BFD_RELOC_MACH_O_ARM64_ADDEND: Self = Self(1738);
    pub const BFD_RELOC_MACH_O_ARM64_GOT_LOAD_PAGE21: Self = Self(1739);
    pub const BFD_RELOC_MACH_O_ARM64_GOT_LOAD_PAGEOFF12: Self = Self(1740);
    pub const BFD_RELOC_MACH_O_ARM64_POINTER_TO_GOT: Self = Self(1741);
    pub const BFD_RELOC_MICROBLAZE_32_LO: Self = Self(1742);
    pub const BFD_RELOC_MICROBLAZE_32_LO_PCREL: Self = Self(1743);
    pub const BFD_RELOC_MICROBLAZE_32_ROSDA: Self = Self(1744);
    pub const BFD_RELOC_MICROBLAZE_32_RWSDA: Self = Self(1745);
    pub const BFD_RELOC_MICROBLAZE_32_SYM_OP_SYM: Self = Self(1746);
    pub const BFD_RELOC_MICROBLAZE_32_NONE: Self = Self(1747);
    pub const BFD_RELOC_MICROBLAZE_64_NONE: Self = Self(1748);
    pub const BFD_RELOC_MICROBLAZE_64_GOTPC: Self = Self(1749);
    pub const BFD_RELOC_MICROBLAZE_64_GOT: Self = Self(1750);
    pub const BFD_RELOC_MICROBLAZE_64_PLT: Self = Self(1751);
    pub const BFD_RELOC_MICROBLAZE_64_GOTOFF: Self = Self(1752);
    pub const BFD_RELOC_MICROBLAZE_32_GOTOFF: Self = Self(1753);
    pub const BFD_RELOC_MICROBLAZE_64_TLS: Self = Self(1754);
    pub const BFD_RELOC_MICROBLAZE_64_TLSGD: Self = Self(1755);
    pub const BFD_RELOC_MICROBLAZE_64_TLSLD: Self = Self(1756);
    pub const BFD_RELOC_MICROBLAZE_32_TLSDTPMOD: Self = Self(1757);
    pub const BFD_RELOC_MICROBLAZE_32_TLSDTPREL: Self = Self(1758);
    pub const BFD_RELOC_MICROBLAZE_64_TLSDTPREL: Self = Self(1759);
    pub const BFD_RELOC_MICROBLAZE_64_TLSGOTTPREL: Self = Self(1760);
    pub const BFD_RELOC_MICROBLAZE_64_TLSTPREL: Self = Self(1761);
    pub const BFD_RELOC_MICROBLAZE_64_TEXTPCREL: Self = Self(1762);
    pub const BFD_RELOC_MICROBLAZE_64_TEXTREL: Self = Self(1763);
    pub const BFD_RELOC_KVX_RELOC_START: Self = Self(1764);
    pub const BFD_RELOC_KVX_NONE: Self = Self(1765);
    pub const BFD_RELOC_KVX_16: Self = Self(1766);
    pub const BFD_RELOC_KVX_32: Self = Self(1767);
    pub const BFD_RELOC_KVX_64: Self = Self(1768);
    pub const BFD_RELOC_KVX_S16_PCREL: Self = Self(1769);
    pub const BFD_RELOC_KVX_PCREL17: Self = Self(1770);
    pub const BFD_RELOC_KVX_PCREL27: Self = Self(1771);
    pub const BFD_RELOC_KVX_32_PCREL: Self = Self(1772);
    pub const BFD_RELOC_KVX_S37_PCREL_LO10: Self = Self(1773);
    pub const BFD_RELOC_KVX_S37_PCREL_UP27: Self = Self(1774);
    pub const BFD_RELOC_KVX_S43_PCREL_LO10: Self = Self(1775);
    pub const BFD_RELOC_KVX_S43_PCREL_UP27: Self = Self(1776);
    pub const BFD_RELOC_KVX_S43_PCREL_EX6: Self = Self(1777);
    pub const BFD_RELOC_KVX_S64_PCREL_LO10: Self = Self(1778);
    pub const BFD_RELOC_KVX_S64_PCREL_UP27: Self = Self(1779);
    pub const BFD_RELOC_KVX_S64_PCREL_EX27: Self = Self(1780);
    pub const BFD_RELOC_KVX_64_PCREL: Self = Self(1781);
    pub const BFD_RELOC_KVX_S16: Self = Self(1782);
    pub const BFD_RELOC_KVX_S32_LO5: Self = Self(1783);
    pub const BFD_RELOC_KVX_S32_UP27: Self = Self(1784);
    pub const BFD_RELOC_KVX_S37_LO10: Self = Self(1785);
    pub const BFD_RELOC_KVX_S37_UP27: Self = Self(1786);
    pub const BFD_RELOC_KVX_S37_GOTOFF_LO10: Self = Self(1787);
    pub const BFD_RELOC_KVX_S37_GOTOFF_UP27: Self = Self(1788);
    pub const BFD_RELOC_KVX_S43_GOTOFF_LO10: Self = Self(1789);
    pub const BFD_RELOC_KVX_S43_GOTOFF_UP27: Self = Self(1790);
    pub const BFD_RELOC_KVX_S43_GOTOFF_EX6: Self = Self(1791);
    pub const BFD_RELOC_KVX_32_GOTOFF: Self = Self(1792);
    pub const BFD_RELOC_KVX_64_GOTOFF: Self = Self(1793);
    pub const BFD_RELOC_KVX_32_GOT: Self = Self(1794);
    pub const BFD_RELOC_KVX_S37_GOT_LO10: Self = Self(1795);
    pub const BFD_RELOC_KVX_S37_GOT_UP27: Self = Self(1796);
    pub const BFD_RELOC_KVX_S43_GOT_LO10: Self = Self(1797);
    pub const BFD_RELOC_KVX_S43_GOT_UP27: Self = Self(1798);
    pub const BFD_RELOC_KVX_S43_GOT_EX6: Self = Self(1799);
    pub const BFD_RELOC_KVX_64_GOT: Self = Self(1800);
    pub const BFD_RELOC_KVX_GLOB_DAT: Self = Self(1801);
    pub const BFD_RELOC_KVX_COPY: Self = Self(1802);
    pub const BFD_RELOC_KVX_JMP_SLOT: Self = Self(1803);
    pub const BFD_RELOC_KVX_RELATIVE: Self = Self(1804);
    pub const BFD_RELOC_KVX_S43_LO10: Self = Self(1805);
    pub const BFD_RELOC_KVX_S43_UP27: Self = Self(1806);
    pub const BFD_RELOC_KVX_S43_EX6: Self = Self(1807);
    pub const BFD_RELOC_KVX_S64_LO10: Self = Self(1808);
    pub const BFD_RELOC_KVX_S64_UP27: Self = Self(1809);
    pub const BFD_RELOC_KVX_S64_EX27: Self = Self(1810);
    pub const BFD_RELOC_KVX_S37_GOTADDR_LO10: Self = Self(1811);
    pub const BFD_RELOC_KVX_S37_GOTADDR_UP27: Self = Self(1812);
    pub const BFD_RELOC_KVX_S43_GOTADDR_LO10: Self = Self(1813);
    pub const BFD_RELOC_KVX_S43_GOTADDR_UP27: Self = Self(1814);
    pub const BFD_RELOC_KVX_S43_GOTADDR_EX6: Self = Self(1815);
    pub const BFD_RELOC_KVX_S64_GOTADDR_LO10: Self = Self(1816);
    pub const BFD_RELOC_KVX_S64_GOTADDR_UP27: Self = Self(1817);
    pub const BFD_RELOC_KVX_S64_GOTADDR_EX27: Self = Self(1818);
    pub const BFD_RELOC_KVX_64_DTPMOD: Self = Self(1819);
    pub const BFD_RELOC_KVX_64_DTPOFF: Self = Self(1820);
    pub const BFD_RELOC_KVX_S37_TLS_DTPOFF_LO10: Self = Self(1821);
    pub const BFD_RELOC_KVX_S37_TLS_DTPOFF_UP27: Self = Self(1822);
    pub const BFD_RELOC_KVX_S43_TLS_DTPOFF_LO10: Self = Self(1823);
    pub const BFD_RELOC_KVX_S43_TLS_DTPOFF_UP27: Self = Self(1824);
    pub const BFD_RELOC_KVX_S43_TLS_DTPOFF_EX6: Self = Self(1825);
    pub const BFD_RELOC_KVX_S37_TLS_GD_LO10: Self = Self(1826);
    pub const BFD_RELOC_KVX_S37_TLS_GD_UP27: Self = Self(1827);
    pub const BFD_RELOC_KVX_S43_TLS_GD_LO10: Self = Self(1828);
    pub const BFD_RELOC_KVX_S43_TLS_GD_UP27: Self = Self(1829);
    pub const BFD_RELOC_KVX_S43_TLS_GD_EX6: Self = Self(1830);
    pub const BFD_RELOC_KVX_S37_TLS_LD_LO10: Self = Self(1831);
    pub const BFD_RELOC_KVX_S37_TLS_LD_UP27: Self = Self(1832);
    pub const BFD_RELOC_KVX_S43_TLS_LD_LO10: Self = Self(1833);
    pub const BFD_RELOC_KVX_S43_TLS_LD_UP27: Self = Self(1834);
    pub const BFD_RELOC_KVX_S43_TLS_LD_EX6: Self = Self(1835);
    pub const BFD_RELOC_KVX_64_TPOFF: Self = Self(1836);
    pub const BFD_RELOC_KVX_S37_TLS_IE_LO10: Self = Self(1837);
    pub const BFD_RELOC_KVX_S37_TLS_IE_UP27: Self = Self(1838);
    pub const BFD_RELOC_KVX_S43_TLS_IE_LO10: Self = Self(1839);
    pub const BFD_RELOC_KVX_S43_TLS_IE_UP27: Self = Self(1840);
    pub const BFD_RELOC_KVX_S43_TLS_IE_EX6: Self = Self(1841);
    pub const BFD_RELOC_KVX_S37_TLS_LE_LO10: Self = Self(1842);
    pub const BFD_RELOC_KVX_S37_TLS_LE_UP27: Self = Self(1843);
    pub const BFD_RELOC_KVX_S43_TLS_LE_LO10: Self = Self(1844);
    pub const BFD_RELOC_KVX_S43_TLS_LE_UP27: Self = Self(1845);
    pub const BFD_RELOC_KVX_S43_TLS_LE_EX6: Self = Self(1846);
    pub const BFD_RELOC_KVX_8: Self = Self(1847);
    pub const BFD_RELOC_KVX_RELOC_END: Self = Self(1848);
    pub const BFD_RELOC_AARCH64_RELOC_START: Self = Self(1849);
    pub const BFD_RELOC_AARCH64_NULL: Self = Self(1850);
    pub const BFD_RELOC_AARCH64_NONE: Self = Self(1851);
    pub const BFD_RELOC_AARCH64_64: Self = Self(1852);
    pub const BFD_RELOC_AARCH64_32: Self = Self(1853);
    pub const BFD_RELOC_AARCH64_16: Self = Self(1854);
    pub const BFD_RELOC_AARCH64_64_PCREL: Self = Self(1855);
    pub const BFD_RELOC_AARCH64_32_PCREL: Self = Self(1856);
    pub const BFD_RELOC_AARCH64_16_PCREL: Self = Self(1857);
    pub const BFD_RELOC_AARCH64_MOVW_G0: Self = Self(1858);
    pub const BFD_RELOC_AARCH64_MOVW_G0_NC: Self = Self(1859);
    pub const BFD_RELOC_AARCH64_MOVW_G1: Self = Self(1860);
    pub const BFD_RELOC_AARCH64_MOVW_G1_NC: Self = Self(1861);
    pub const BFD_RELOC_AARCH64_MOVW_G2: Self = Self(1862);
    pub const BFD_RELOC_AARCH64_MOVW_G2_NC: Self = Self(1863);
    pub const BFD_RELOC_AARCH64_MOVW_G3: Self = Self(1864);
    pub const BFD_RELOC_AARCH64_MOVW_G0_S: Self = Self(1865);
    pub const BFD_RELOC_AARCH64_MOVW_G1_S: Self = Self(1866);
    pub const BFD_RELOC_AARCH64_MOVW_G2_S: Self = Self(1867);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G0: Self = Self(1868);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G0_NC: Self = Self(1869);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G1: Self = Self(1870);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G1_NC: Self = Self(1871);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G2: Self = Self(1872);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G2_NC: Self = Self(1873);
    pub const BFD_RELOC_AARCH64_MOVW_PREL_G3: Self = Self(1874);
    pub const BFD_RELOC_AARCH64_LD_LO19_PCREL: Self = Self(1875);
    pub const BFD_RELOC_AARCH64_ADR_LO21_PCREL: Self = Self(1876);
    pub const BFD_RELOC_AARCH64_ADR_HI21_PCREL: Self = Self(1877);
    pub const BFD_RELOC_AARCH64_ADR_HI21_NC_PCREL: Self = Self(1878);
    pub const BFD_RELOC_AARCH64_ADD_LO12: Self = Self(1879);
    pub const BFD_RELOC_AARCH64_LDST8_LO12: Self = Self(1880);
    pub const BFD_RELOC_AARCH64_TSTBR14: Self = Self(1881);
    pub const BFD_RELOC_AARCH64_BRANCH19: Self = Self(1882);
    pub const BFD_RELOC_AARCH64_JUMP26: Self = Self(1883);
    pub const BFD_RELOC_AARCH64_CALL26: Self = Self(1884);
    pub const BFD_RELOC_AARCH64_LDST16_LO12: Self = Self(1885);
    pub const BFD_RELOC_AARCH64_LDST32_LO12: Self = Self(1886);
    pub const BFD_RELOC_AARCH64_LDST64_LO12: Self = Self(1887);
    pub const BFD_RELOC_AARCH64_LDST128_LO12: Self = Self(1888);
    pub const BFD_RELOC_AARCH64_GOT_LD_PREL19: Self = Self(1889);
    pub const BFD_RELOC_AARCH64_ADR_GOT_PAGE: Self = Self(1890);
    pub const BFD_RELOC_AARCH64_LD64_GOT_LO12_NC: Self = Self(1891);
    pub const BFD_RELOC_AARCH64_LD32_GOT_LO12_NC: Self = Self(1892);
    pub const BFD_RELOC_AARCH64_MOVW_GOTOFF_G0_NC: Self = Self(1893);
    pub const BFD_RELOC_AARCH64_MOVW_GOTOFF_G1: Self = Self(1894);
    pub const BFD_RELOC_AARCH64_LD64_GOTOFF_LO15: Self = Self(1895);
    pub const BFD_RELOC_AARCH64_LD32_GOTPAGE_LO14: Self = Self(1896);
    pub const BFD_RELOC_AARCH64_LD64_GOTPAGE_LO15: Self = Self(1897);
    pub const BFD_RELOC_AARCH64_TLSGD_ADR_PAGE21: Self = Self(1898);
    pub const BFD_RELOC_AARCH64_TLSGD_ADR_PREL21: Self = Self(1899);
    pub const BFD_RELOC_AARCH64_TLSGD_ADD_LO12_NC: Self = Self(1900);
    pub const BFD_RELOC_AARCH64_TLSGD_MOVW_G0_NC: Self = Self(1901);
    pub const BFD_RELOC_AARCH64_TLSGD_MOVW_G1: Self = Self(1902);
    pub const BFD_RELOC_AARCH64_TLSIE_ADR_GOTTPREL_PAGE21: Self = Self(1903);
    pub const BFD_RELOC_AARCH64_TLSIE_LD64_GOTTPREL_LO12_NC: Self = Self(1904);
    pub const BFD_RELOC_AARCH64_TLSIE_LD32_GOTTPREL_LO12_NC: Self = Self(1905);
    pub const BFD_RELOC_AARCH64_TLSIE_LD_GOTTPREL_PREL19: Self = Self(1906);
    pub const BFD_RELOC_AARCH64_TLSIE_MOVW_GOTTPREL_G0_NC: Self = Self(1907);
    pub const BFD_RELOC_AARCH64_TLSIE_MOVW_GOTTPREL_G1: Self = Self(1908);
    pub const BFD_RELOC_AARCH64_TLSLD_ADD_DTPREL_HI12: Self = Self(1909);
    pub const BFD_RELOC_AARCH64_TLSLD_ADD_DTPREL_LO12: Self = Self(1910);
    pub const BFD_RELOC_AARCH64_TLSLD_ADD_DTPREL_LO12_NC: Self = Self(1911);
    pub const BFD_RELOC_AARCH64_TLSLD_ADD_LO12_NC: Self = Self(1912);
    pub const BFD_RELOC_AARCH64_TLSLD_ADR_PAGE21: Self = Self(1913);
    pub const BFD_RELOC_AARCH64_TLSLD_ADR_PREL21: Self = Self(1914);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST16_DTPREL_LO12: Self = Self(1915);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST16_DTPREL_LO12_NC: Self = Self(1916);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST32_DTPREL_LO12: Self = Self(1917);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST32_DTPREL_LO12_NC: Self = Self(1918);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST64_DTPREL_LO12: Self = Self(1919);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST64_DTPREL_LO12_NC: Self = Self(1920);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST8_DTPREL_LO12: Self = Self(1921);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST8_DTPREL_LO12_NC: Self = Self(1922);
    pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G0: Self = Self(1923);
    pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G0_NC: Self = Self(1924);
    pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G1: Self = Self(1925);
    pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G1_NC: Self = Self(1926);
    pub const BFD_RELOC_AARCH64_TLSLD_MOVW_DTPREL_G2: Self = Self(1927);
    pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G2: Self = Self(1928);
    pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G1: Self = Self(1929);
    pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G1_NC: Self = Self(1930);
    pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G0: Self = Self(1931);
    pub const BFD_RELOC_AARCH64_TLSLE_MOVW_TPREL_G0_NC: Self = Self(1932);
    pub const BFD_RELOC_AARCH64_TLSLE_ADD_TPREL_HI12: Self = Self(1933);
    pub const BFD_RELOC_AARCH64_TLSLE_ADD_TPREL_LO12: Self = Self(1934);
    pub const BFD_RELOC_AARCH64_TLSLE_ADD_TPREL_LO12_NC: Self = Self(1935);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST16_TPREL_LO12: Self = Self(1936);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST16_TPREL_LO12_NC: Self = Self(1937);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST32_TPREL_LO12: Self = Self(1938);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST32_TPREL_LO12_NC: Self = Self(1939);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST64_TPREL_LO12: Self = Self(1940);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST64_TPREL_LO12_NC: Self = Self(1941);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST8_TPREL_LO12: Self = Self(1942);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST8_TPREL_LO12_NC: Self = Self(1943);
    pub const BFD_RELOC_AARCH64_TLSDESC_LD_PREL19: Self = Self(1944);
    pub const BFD_RELOC_AARCH64_TLSDESC_ADR_PREL21: Self = Self(1945);
    pub const BFD_RELOC_AARCH64_TLSDESC_ADR_PAGE21: Self = Self(1946);
    pub const BFD_RELOC_AARCH64_TLSDESC_LD64_LO12: Self = Self(1947);
    pub const BFD_RELOC_AARCH64_TLSDESC_LD32_LO12_NC: Self = Self(1948);
    pub const BFD_RELOC_AARCH64_TLSDESC_ADD_LO12: Self = Self(1949);
    pub const BFD_RELOC_AARCH64_TLSDESC_OFF_G1: Self = Self(1950);
    pub const BFD_RELOC_AARCH64_TLSDESC_OFF_G0_NC: Self = Self(1951);
    pub const BFD_RELOC_AARCH64_TLSDESC_LDR: Self = Self(1952);
    pub const BFD_RELOC_AARCH64_TLSDESC_ADD: Self = Self(1953);
    pub const BFD_RELOC_AARCH64_TLSDESC_CALL: Self = Self(1954);
    pub const BFD_RELOC_AARCH64_COPY: Self = Self(1955);
    pub const BFD_RELOC_AARCH64_GLOB_DAT: Self = Self(1956);
    pub const BFD_RELOC_AARCH64_JUMP_SLOT: Self = Self(1957);
    pub const BFD_RELOC_AARCH64_RELATIVE: Self = Self(1958);
    pub const BFD_RELOC_AARCH64_TLS_DTPMOD: Self = Self(1959);
    pub const BFD_RELOC_AARCH64_TLS_DTPREL: Self = Self(1960);
    pub const BFD_RELOC_AARCH64_TLS_TPREL: Self = Self(1961);
    pub const BFD_RELOC_AARCH64_TLSDESC: Self = Self(1962);
    pub const BFD_RELOC_AARCH64_IRELATIVE: Self = Self(1963);
    pub const BFD_RELOC_AARCH64_RELOC_END: Self = Self(1964);
    pub const BFD_RELOC_AARCH64_GAS_INTERNAL_FIXUP: Self = Self(1965);
    pub const BFD_RELOC_AARCH64_LDST_LO12: Self = Self(1966);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST_DTPREL_LO12: Self = Self(1967);
    pub const BFD_RELOC_AARCH64_TLSLD_LDST_DTPREL_LO12_NC: Self = Self(1968);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST_TPREL_LO12: Self = Self(1969);
    pub const BFD_RELOC_AARCH64_TLSLE_LDST_TPREL_LO12_NC: Self = Self(1970);
    pub const BFD_RELOC_AARCH64_LD_GOT_LO12_NC: Self = Self(1971);
    pub const BFD_RELOC_AARCH64_TLSIE_LD_GOTTPREL_LO12_NC: Self = Self(1972);
    pub const BFD_RELOC_AARCH64_TLSDESC_LD_LO12_NC: Self = Self(1973);
    pub const BFD_RELOC_AARCH64_BRANCH9: Self = Self(1974);
    pub const BFD_RELOC_TILEPRO_BROFF_X1: Self = Self(1975);
    pub const BFD_RELOC_TILEPRO_JOFFLONG_X1: Self = Self(1976);
    pub const BFD_RELOC_TILEPRO_JOFFLONG_X1_PLT: Self = Self(1977);
    pub const BFD_RELOC_TILEPRO_IMM8_X0: Self = Self(1978);
    pub const BFD_RELOC_TILEPRO_IMM8_Y0: Self = Self(1979);
    pub const BFD_RELOC_TILEPRO_IMM8_X1: Self = Self(1980);
    pub const BFD_RELOC_TILEPRO_IMM8_Y1: Self = Self(1981);
    pub const BFD_RELOC_TILEPRO_DEST_IMM8_X1: Self = Self(1982);
    pub const BFD_RELOC_TILEPRO_MT_IMM15_X1: Self = Self(1983);
    pub const BFD_RELOC_TILEPRO_MF_IMM15_X1: Self = Self(1984);
    pub const BFD_RELOC_TILEPRO_IMM16_X0: Self = Self(1985);
    pub const BFD_RELOC_TILEPRO_IMM16_X1: Self = Self(1986);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_LO: Self = Self(1987);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_LO: Self = Self(1988);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_HI: Self = Self(1989);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_HI: Self = Self(1990);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_HA: Self = Self(1991);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_HA: Self = Self(1992);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_PCREL: Self = Self(1993);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_PCREL: Self = Self(1994);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_LO_PCREL: Self = Self(1995);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_LO_PCREL: Self = Self(1996);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_HI_PCREL: Self = Self(1997);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_HI_PCREL: Self = Self(1998);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_HA_PCREL: Self = Self(1999);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_HA_PCREL: Self = Self(2000);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT: Self = Self(2001);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT: Self = Self(2002);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT_LO: Self = Self(2003);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT_LO: Self = Self(2004);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT_HI: Self = Self(2005);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT_HI: Self = Self(2006);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_GOT_HA: Self = Self(2007);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_GOT_HA: Self = Self(2008);
    pub const BFD_RELOC_TILEPRO_MMSTART_X0: Self = Self(2009);
    pub const BFD_RELOC_TILEPRO_MMEND_X0: Self = Self(2010);
    pub const BFD_RELOC_TILEPRO_MMSTART_X1: Self = Self(2011);
    pub const BFD_RELOC_TILEPRO_MMEND_X1: Self = Self(2012);
    pub const BFD_RELOC_TILEPRO_SHAMT_X0: Self = Self(2013);
    pub const BFD_RELOC_TILEPRO_SHAMT_X1: Self = Self(2014);
    pub const BFD_RELOC_TILEPRO_SHAMT_Y0: Self = Self(2015);
    pub const BFD_RELOC_TILEPRO_SHAMT_Y1: Self = Self(2016);
    pub const BFD_RELOC_TILEPRO_TLS_GD_CALL: Self = Self(2017);
    pub const BFD_RELOC_TILEPRO_IMM8_X0_TLS_GD_ADD: Self = Self(2018);
    pub const BFD_RELOC_TILEPRO_IMM8_X1_TLS_GD_ADD: Self = Self(2019);
    pub const BFD_RELOC_TILEPRO_IMM8_Y0_TLS_GD_ADD: Self = Self(2020);
    pub const BFD_RELOC_TILEPRO_IMM8_Y1_TLS_GD_ADD: Self = Self(2021);
    pub const BFD_RELOC_TILEPRO_TLS_IE_LOAD: Self = Self(2022);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD: Self = Self(2023);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD: Self = Self(2024);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD_LO: Self = Self(2025);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD_LO: Self = Self(2026);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD_HI: Self = Self(2027);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD_HI: Self = Self(2028);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_GD_HA: Self = Self(2029);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_GD_HA: Self = Self(2030);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE: Self = Self(2031);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE: Self = Self(2032);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE_LO: Self = Self(2033);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE_LO: Self = Self(2034);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE_HI: Self = Self(2035);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE_HI: Self = Self(2036);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_IE_HA: Self = Self(2037);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_IE_HA: Self = Self(2038);
    pub const BFD_RELOC_TILEPRO_TLS_DTPMOD32: Self = Self(2039);
    pub const BFD_RELOC_TILEPRO_TLS_DTPOFF32: Self = Self(2040);
    pub const BFD_RELOC_TILEPRO_TLS_TPOFF32: Self = Self(2041);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE: Self = Self(2042);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE: Self = Self(2043);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE_LO: Self = Self(2044);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE_LO: Self = Self(2045);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE_HI: Self = Self(2046);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE_HI: Self = Self(2047);
    pub const BFD_RELOC_TILEPRO_IMM16_X0_TLS_LE_HA: Self = Self(2048);
    pub const BFD_RELOC_TILEPRO_IMM16_X1_TLS_LE_HA: Self = Self(2049);
    pub const BFD_RELOC_TILEGX_HW0: Self = Self(2050);
    pub const BFD_RELOC_TILEGX_HW1: Self = Self(2051);
    pub const BFD_RELOC_TILEGX_HW2: Self = Self(2052);
    pub const BFD_RELOC_TILEGX_HW3: Self = Self(2053);
    pub const BFD_RELOC_TILEGX_HW0_LAST: Self = Self(2054);
    pub const BFD_RELOC_TILEGX_HW1_LAST: Self = Self(2055);
    pub const BFD_RELOC_TILEGX_HW2_LAST: Self = Self(2056);
    pub const BFD_RELOC_TILEGX_BROFF_X1: Self = Self(2057);
    pub const BFD_RELOC_TILEGX_JUMPOFF_X1: Self = Self(2058);
    pub const BFD_RELOC_TILEGX_JUMPOFF_X1_PLT: Self = Self(2059);
    pub const BFD_RELOC_TILEGX_IMM8_X0: Self = Self(2060);
    pub const BFD_RELOC_TILEGX_IMM8_Y0: Self = Self(2061);
    pub const BFD_RELOC_TILEGX_IMM8_X1: Self = Self(2062);
    pub const BFD_RELOC_TILEGX_IMM8_Y1: Self = Self(2063);
    pub const BFD_RELOC_TILEGX_DEST_IMM8_X1: Self = Self(2064);
    pub const BFD_RELOC_TILEGX_MT_IMM14_X1: Self = Self(2065);
    pub const BFD_RELOC_TILEGX_MF_IMM14_X1: Self = Self(2066);
    pub const BFD_RELOC_TILEGX_MMSTART_X0: Self = Self(2067);
    pub const BFD_RELOC_TILEGX_MMEND_X0: Self = Self(2068);
    pub const BFD_RELOC_TILEGX_SHAMT_X0: Self = Self(2069);
    pub const BFD_RELOC_TILEGX_SHAMT_X1: Self = Self(2070);
    pub const BFD_RELOC_TILEGX_SHAMT_Y0: Self = Self(2071);
    pub const BFD_RELOC_TILEGX_SHAMT_Y1: Self = Self(2072);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0: Self = Self(2073);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0: Self = Self(2074);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1: Self = Self(2075);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1: Self = Self(2076);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW2: Self = Self(2077);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW2: Self = Self(2078);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW3: Self = Self(2079);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW3: Self = Self(2080);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST: Self = Self(2081);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST: Self = Self(2082);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST: Self = Self(2083);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST: Self = Self(2084);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_LAST: Self = Self(2085);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_LAST: Self = Self(2086);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_PCREL: Self = Self(2087);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_PCREL: Self = Self(2088);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_PCREL: Self = Self(2089);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_PCREL: Self = Self(2090);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_PCREL: Self = Self(2091);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_PCREL: Self = Self(2092);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW3_PCREL: Self = Self(2093);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW3_PCREL: Self = Self(2094);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_PCREL: Self = Self(2095);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_PCREL: Self = Self(2096);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_PCREL: Self = Self(2097);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_PCREL: Self = Self(2098);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_LAST_PCREL: Self = Self(2099);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_LAST_PCREL: Self = Self(2100);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_GOT: Self = Self(2101);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_GOT: Self = Self(2102);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_PLT_PCREL: Self = Self(2103);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_PLT_PCREL: Self = Self(2104);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_PLT_PCREL: Self = Self(2105);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_PLT_PCREL: Self = Self(2106);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_PLT_PCREL: Self = Self(2107);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_PLT_PCREL: Self = Self(2108);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_GOT: Self = Self(2109);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_GOT: Self = Self(2110);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_GOT: Self = Self(2111);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_GOT: Self = Self(2112);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW3_PLT_PCREL: Self = Self(2113);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW3_PLT_PCREL: Self = Self(2114);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_TLS_GD: Self = Self(2115);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_TLS_GD: Self = Self(2116);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_TLS_LE: Self = Self(2117);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_TLS_LE: Self = Self(2118);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_TLS_LE: Self = Self(2119);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_TLS_LE: Self = Self(2120);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_TLS_LE: Self = Self(2121);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_TLS_LE: Self = Self(2122);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_TLS_GD: Self = Self(2123);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_TLS_GD: Self = Self(2124);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_TLS_GD: Self = Self(2125);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_TLS_GD: Self = Self(2126);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_TLS_IE: Self = Self(2127);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_TLS_IE: Self = Self(2128);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_PLT_PCREL: Self = Self(2129);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_PLT_PCREL: Self = Self(2130);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_PLT_PCREL: Self = Self(2131);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_PLT_PCREL: Self = Self(2132);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW2_LAST_PLT_PCREL: Self = Self(2133);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW2_LAST_PLT_PCREL: Self = Self(2134);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW0_LAST_TLS_IE: Self = Self(2135);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW0_LAST_TLS_IE: Self = Self(2136);
    pub const BFD_RELOC_TILEGX_IMM16_X0_HW1_LAST_TLS_IE: Self = Self(2137);
    pub const BFD_RELOC_TILEGX_IMM16_X1_HW1_LAST_TLS_IE: Self = Self(2138);
    pub const BFD_RELOC_TILEGX_TLS_DTPMOD64: Self = Self(2139);
    pub const BFD_RELOC_TILEGX_TLS_DTPOFF64: Self = Self(2140);
    pub const BFD_RELOC_TILEGX_TLS_TPOFF64: Self = Self(2141);
    pub const BFD_RELOC_TILEGX_TLS_DTPMOD32: Self = Self(2142);
    pub const BFD_RELOC_TILEGX_TLS_DTPOFF32: Self = Self(2143);
    pub const BFD_RELOC_TILEGX_TLS_TPOFF32: Self = Self(2144);
    pub const BFD_RELOC_TILEGX_TLS_GD_CALL: Self = Self(2145);
    pub const BFD_RELOC_TILEGX_IMM8_X0_TLS_GD_ADD: Self = Self(2146);
    pub const BFD_RELOC_TILEGX_IMM8_X1_TLS_GD_ADD: Self = Self(2147);
    pub const BFD_RELOC_TILEGX_IMM8_Y0_TLS_GD_ADD: Self = Self(2148);
    pub const BFD_RELOC_TILEGX_IMM8_Y1_TLS_GD_ADD: Self = Self(2149);
    pub const BFD_RELOC_TILEGX_TLS_IE_LOAD: Self = Self(2150);
    pub const BFD_RELOC_TILEGX_IMM8_X0_TLS_ADD: Self = Self(2151);
    pub const BFD_RELOC_TILEGX_IMM8_X1_TLS_ADD: Self = Self(2152);
    pub const BFD_RELOC_TILEGX_IMM8_Y0_TLS_ADD: Self = Self(2153);
    pub const BFD_RELOC_TILEGX_IMM8_Y1_TLS_ADD: Self = Self(2154);
    pub const BFD_RELOC_BPF_64: Self = Self(2155);
    pub const BFD_RELOC_BPF_DISP32: Self = Self(2156);
    pub const BFD_RELOC_BPF_DISPCALL32: Self = Self(2157);
    pub const BFD_RELOC_BPF_DISP16: Self = Self(2158);
    pub const BFD_RELOC_EPIPHANY_SIMM8: Self = Self(2159);
    pub const BFD_RELOC_EPIPHANY_SIMM24: Self = Self(2160);
    pub const BFD_RELOC_EPIPHANY_HIGH: Self = Self(2161);
    pub const BFD_RELOC_EPIPHANY_LOW: Self = Self(2162);
    pub const BFD_RELOC_EPIPHANY_SIMM11: Self = Self(2163);
    pub const BFD_RELOC_EPIPHANY_IMM11: Self = Self(2164);
    pub const BFD_RELOC_EPIPHANY_IMM8: Self = Self(2165);
    pub const BFD_RELOC_VISIUM_HI16: Self = Self(2166);
    pub const BFD_RELOC_VISIUM_LO16: Self = Self(2167);
    pub const BFD_RELOC_VISIUM_IM16: Self = Self(2168);
    pub const BFD_RELOC_VISIUM_REL16: Self = Self(2169);
    pub const BFD_RELOC_VISIUM_HI16_PCREL: Self = Self(2170);
    pub const BFD_RELOC_VISIUM_LO16_PCREL: Self = Self(2171);
    pub const BFD_RELOC_VISIUM_IM16_PCREL: Self = Self(2172);
    pub const BFD_RELOC_WASM32_LEB128: Self = Self(2173);
    pub const BFD_RELOC_WASM32_LEB128_GOT: Self = Self(2174);
    pub const BFD_RELOC_WASM32_LEB128_GOT_CODE: Self = Self(2175);
    pub const BFD_RELOC_WASM32_LEB128_PLT: Self = Self(2176);
    pub const BFD_RELOC_WASM32_PLT_INDEX: Self = Self(2177);
    pub const BFD_RELOC_WASM32_ABS32_CODE: Self = Self(2178);
    pub const BFD_RELOC_WASM32_CODE_POINTER: Self = Self(2179);
    pub const BFD_RELOC_WASM32_INDEX: Self = Self(2180);
    pub const BFD_RELOC_WASM32_PLT_SIG: Self = Self(2181);
    pub const BFD_RELOC_CKCORE_NONE: Self = Self(2182);
    pub const BFD_RELOC_CKCORE_ADDR32: Self = Self(2183);
    pub const BFD_RELOC_CKCORE_PCREL_IMM8BY4: Self = Self(2184);
    pub const BFD_RELOC_CKCORE_PCREL_IMM11BY2: Self = Self(2185);
    pub const BFD_RELOC_CKCORE_PCREL_IMM4BY2: Self = Self(2186);
    pub const BFD_RELOC_CKCORE_PCREL32: Self = Self(2187);
    pub const BFD_RELOC_CKCORE_PCREL_JSR_IMM11BY2: Self = Self(2188);
    pub const BFD_RELOC_CKCORE_GNU_VTINHERIT: Self = Self(2189);
    pub const BFD_RELOC_CKCORE_GNU_VTENTRY: Self = Self(2190);
    pub const BFD_RELOC_CKCORE_RELATIVE: Self = Self(2191);
    pub const BFD_RELOC_CKCORE_COPY: Self = Self(2192);
    pub const BFD_RELOC_CKCORE_GLOB_DAT: Self = Self(2193);
    pub const BFD_RELOC_CKCORE_JUMP_SLOT: Self = Self(2194);
    pub const BFD_RELOC_CKCORE_GOTOFF: Self = Self(2195);
    pub const BFD_RELOC_CKCORE_GOTPC: Self = Self(2196);
    pub const BFD_RELOC_CKCORE_GOT32: Self = Self(2197);
    pub const BFD_RELOC_CKCORE_PLT32: Self = Self(2198);
    pub const BFD_RELOC_CKCORE_ADDRGOT: Self = Self(2199);
    pub const BFD_RELOC_CKCORE_ADDRPLT: Self = Self(2200);
    pub const BFD_RELOC_CKCORE_PCREL_IMM26BY2: Self = Self(2201);
    pub const BFD_RELOC_CKCORE_PCREL_IMM16BY2: Self = Self(2202);
    pub const BFD_RELOC_CKCORE_PCREL_IMM16BY4: Self = Self(2203);
    pub const BFD_RELOC_CKCORE_PCREL_IMM10BY2: Self = Self(2204);
    pub const BFD_RELOC_CKCORE_PCREL_IMM10BY4: Self = Self(2205);
    pub const BFD_RELOC_CKCORE_ADDR_HI16: Self = Self(2206);
    pub const BFD_RELOC_CKCORE_ADDR_LO16: Self = Self(2207);
    pub const BFD_RELOC_CKCORE_GOTPC_HI16: Self = Self(2208);
    pub const BFD_RELOC_CKCORE_GOTPC_LO16: Self = Self(2209);
    pub const BFD_RELOC_CKCORE_GOTOFF_HI16: Self = Self(2210);
    pub const BFD_RELOC_CKCORE_GOTOFF_LO16: Self = Self(2211);
    pub const BFD_RELOC_CKCORE_GOT12: Self = Self(2212);
    pub const BFD_RELOC_CKCORE_GOT_HI16: Self = Self(2213);
    pub const BFD_RELOC_CKCORE_GOT_LO16: Self = Self(2214);
    pub const BFD_RELOC_CKCORE_PLT12: Self = Self(2215);
    pub const BFD_RELOC_CKCORE_PLT_HI16: Self = Self(2216);
    pub const BFD_RELOC_CKCORE_PLT_LO16: Self = Self(2217);
    pub const BFD_RELOC_CKCORE_ADDRGOT_HI16: Self = Self(2218);
    pub const BFD_RELOC_CKCORE_ADDRGOT_LO16: Self = Self(2219);
    pub const BFD_RELOC_CKCORE_ADDRPLT_HI16: Self = Self(2220);
    pub const BFD_RELOC_CKCORE_ADDRPLT_LO16: Self = Self(2221);
    pub const BFD_RELOC_CKCORE_PCREL_JSR_IMM26BY2: Self = Self(2222);
    pub const BFD_RELOC_CKCORE_TOFFSET_LO16: Self = Self(2223);
    pub const BFD_RELOC_CKCORE_DOFFSET_LO16: Self = Self(2224);
    pub const BFD_RELOC_CKCORE_PCREL_IMM18BY2: Self = Self(2225);
    pub const BFD_RELOC_CKCORE_DOFFSET_IMM18: Self = Self(2226);
    pub const BFD_RELOC_CKCORE_DOFFSET_IMM18BY2: Self = Self(2227);
    pub const BFD_RELOC_CKCORE_DOFFSET_IMM18BY4: Self = Self(2228);
    pub const BFD_RELOC_CKCORE_GOTOFF_IMM18: Self = Self(2229);
    pub const BFD_RELOC_CKCORE_GOT_IMM18BY4: Self = Self(2230);
    pub const BFD_RELOC_CKCORE_PLT_IMM18BY4: Self = Self(2231);
    pub const BFD_RELOC_CKCORE_PCREL_IMM7BY4: Self = Self(2232);
    pub const BFD_RELOC_CKCORE_TLS_LE32: Self = Self(2233);
    pub const BFD_RELOC_CKCORE_TLS_IE32: Self = Self(2234);
    pub const BFD_RELOC_CKCORE_TLS_GD32: Self = Self(2235);
    pub const BFD_RELOC_CKCORE_TLS_LDM32: Self = Self(2236);
    pub const BFD_RELOC_CKCORE_TLS_LDO32: Self = Self(2237);
    pub const BFD_RELOC_CKCORE_TLS_DTPMOD32: Self = Self(2238);
    pub const BFD_RELOC_CKCORE_TLS_DTPOFF32: Self = Self(2239);
    pub const BFD_RELOC_CKCORE_TLS_TPOFF32: Self = Self(2240);
    pub const BFD_RELOC_CKCORE_PCREL_FLRW_IMM8BY4: Self = Self(2241);
    pub const BFD_RELOC_CKCORE_NOJSRI: Self = Self(2242);
    pub const BFD_RELOC_CKCORE_CALLGRAPH: Self = Self(2243);
    pub const BFD_RELOC_CKCORE_IRELATIVE: Self = Self(2244);
    pub const BFD_RELOC_CKCORE_PCREL_BLOOP_IMM4BY4: Self = Self(2245);
    pub const BFD_RELOC_CKCORE_PCREL_BLOOP_IMM12BY4: Self = Self(2246);
    pub const BFD_RELOC_S12Z_OPR: Self = Self(2247);
    pub const BFD_RELOC_S12Z_15_PCREL: Self = Self(2248);
    pub const BFD_RELOC_LARCH_TLS_DTPMOD32: Self = Self(2249);
    pub const BFD_RELOC_LARCH_TLS_DTPREL32: Self = Self(2250);
    pub const BFD_RELOC_LARCH_TLS_DTPMOD64: Self = Self(2251);
    pub const BFD_RELOC_LARCH_TLS_DTPREL64: Self = Self(2252);
    pub const BFD_RELOC_LARCH_TLS_TPREL32: Self = Self(2253);
    pub const BFD_RELOC_LARCH_TLS_TPREL64: Self = Self(2254);
    pub const BFD_RELOC_LARCH_TLS_DESC32: Self = Self(2255);
    pub const BFD_RELOC_LARCH_TLS_DESC64: Self = Self(2256);
    pub const BFD_RELOC_LARCH_MARK_LA: Self = Self(2257);
    pub const BFD_RELOC_LARCH_MARK_PCREL: Self = Self(2258);
    pub const BFD_RELOC_LARCH_SOP_PUSH_PCREL: Self = Self(2259);
    pub const BFD_RELOC_LARCH_SOP_PUSH_ABSOLUTE: Self = Self(2260);
    pub const BFD_RELOC_LARCH_SOP_PUSH_DUP: Self = Self(2261);
    pub const BFD_RELOC_LARCH_SOP_PUSH_GPREL: Self = Self(2262);
    pub const BFD_RELOC_LARCH_SOP_PUSH_TLS_TPREL: Self = Self(2263);
    pub const BFD_RELOC_LARCH_SOP_PUSH_TLS_GOT: Self = Self(2264);
    pub const BFD_RELOC_LARCH_SOP_PUSH_TLS_GD: Self = Self(2265);
    pub const BFD_RELOC_LARCH_SOP_PUSH_PLT_PCREL: Self = Self(2266);
    pub const BFD_RELOC_LARCH_SOP_ASSERT: Self = Self(2267);
    pub const BFD_RELOC_LARCH_SOP_NOT: Self = Self(2268);
    pub const BFD_RELOC_LARCH_SOP_SUB: Self = Self(2269);
    pub const BFD_RELOC_LARCH_SOP_SL: Self = Self(2270);
    pub const BFD_RELOC_LARCH_SOP_SR: Self = Self(2271);
    pub const BFD_RELOC_LARCH_SOP_ADD: Self = Self(2272);
    pub const BFD_RELOC_LARCH_SOP_AND: Self = Self(2273);
    pub const BFD_RELOC_LARCH_SOP_IF_ELSE: Self = Self(2274);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_10_5: Self = Self(2275);
    pub const BFD_RELOC_LARCH_SOP_POP_32_U_10_12: Self = Self(2276);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_10_12: Self = Self(2277);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_10_16: Self = Self(2278);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_10_16_S2: Self = Self(2279);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_5_20: Self = Self(2280);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_0_5_10_16_S2: Self = Self(2281);
    pub const BFD_RELOC_LARCH_SOP_POP_32_S_0_10_10_16_S2: Self = Self(2282);
    pub const BFD_RELOC_LARCH_SOP_POP_32_U: Self = Self(2283);
    pub const BFD_RELOC_LARCH_ADD8: Self = Self(2284);
    pub const BFD_RELOC_LARCH_ADD16: Self = Self(2285);
    pub const BFD_RELOC_LARCH_ADD24: Self = Self(2286);
    pub const BFD_RELOC_LARCH_ADD32: Self = Self(2287);
    pub const BFD_RELOC_LARCH_ADD64: Self = Self(2288);
    pub const BFD_RELOC_LARCH_SUB8: Self = Self(2289);
    pub const BFD_RELOC_LARCH_SUB16: Self = Self(2290);
    pub const BFD_RELOC_LARCH_SUB24: Self = Self(2291);
    pub const BFD_RELOC_LARCH_SUB32: Self = Self(2292);
    pub const BFD_RELOC_LARCH_SUB64: Self = Self(2293);
    pub const BFD_RELOC_LARCH_B16: Self = Self(2294);
    pub const BFD_RELOC_LARCH_B21: Self = Self(2295);
    pub const BFD_RELOC_LARCH_B26: Self = Self(2296);
    pub const BFD_RELOC_LARCH_ABS_HI20: Self = Self(2297);
    pub const BFD_RELOC_LARCH_ABS_LO12: Self = Self(2298);
    pub const BFD_RELOC_LARCH_ABS64_LO20: Self = Self(2299);
    pub const BFD_RELOC_LARCH_ABS64_HI12: Self = Self(2300);
    pub const BFD_RELOC_LARCH_PCALA_HI20: Self = Self(2301);
    pub const BFD_RELOC_LARCH_PCALA_LO12: Self = Self(2302);
    pub const BFD_RELOC_LARCH_PCALA64_LO20: Self = Self(2303);
    pub const BFD_RELOC_LARCH_PCALA64_HI12: Self = Self(2304);
    pub const BFD_RELOC_LARCH_GOT_PC_HI20: Self = Self(2305);
    pub const BFD_RELOC_LARCH_GOT_PC_LO12: Self = Self(2306);
    pub const BFD_RELOC_LARCH_GOT64_PC_LO20: Self = Self(2307);
    pub const BFD_RELOC_LARCH_GOT64_PC_HI12: Self = Self(2308);
    pub const BFD_RELOC_LARCH_GOT_HI20: Self = Self(2309);
    pub const BFD_RELOC_LARCH_GOT_LO12: Self = Self(2310);
    pub const BFD_RELOC_LARCH_GOT64_LO20: Self = Self(2311);
    pub const BFD_RELOC_LARCH_GOT64_HI12: Self = Self(2312);
    pub const BFD_RELOC_LARCH_TLS_LE_HI20: Self = Self(2313);
    pub const BFD_RELOC_LARCH_TLS_LE_LO12: Self = Self(2314);
    pub const BFD_RELOC_LARCH_TLS_LE64_LO20: Self = Self(2315);
    pub const BFD_RELOC_LARCH_TLS_LE64_HI12: Self = Self(2316);
    pub const BFD_RELOC_LARCH_TLS_IE_PC_HI20: Self = Self(2317);
    pub const BFD_RELOC_LARCH_TLS_IE_PC_LO12: Self = Self(2318);
    pub const BFD_RELOC_LARCH_TLS_IE64_PC_LO20: Self = Self(2319);
    pub const BFD_RELOC_LARCH_TLS_IE64_PC_HI12: Self = Self(2320);
    pub const BFD_RELOC_LARCH_TLS_IE_HI20: Self = Self(2321);
    pub const BFD_RELOC_LARCH_TLS_IE_LO12: Self = Self(2322);
    pub const BFD_RELOC_LARCH_TLS_IE64_LO20: Self = Self(2323);
    pub const BFD_RELOC_LARCH_TLS_IE64_HI12: Self = Self(2324);
    pub const BFD_RELOC_LARCH_TLS_LD_PC_HI20: Self = Self(2325);
    pub const BFD_RELOC_LARCH_TLS_LD_HI20: Self = Self(2326);
    pub const BFD_RELOC_LARCH_TLS_GD_PC_HI20: Self = Self(2327);
    pub const BFD_RELOC_LARCH_TLS_GD_HI20: Self = Self(2328);
    pub const BFD_RELOC_LARCH_RELAX: Self = Self(2329);
    pub const BFD_RELOC_LARCH_DELETE: Self = Self(2330);
    pub const BFD_RELOC_LARCH_ALIGN: Self = Self(2331);
    pub const BFD_RELOC_LARCH_PCREL20_S2: Self = Self(2332);
    pub const BFD_RELOC_LARCH_CFA: Self = Self(2333);
    pub const BFD_RELOC_LARCH_ADD6: Self = Self(2334);
    pub const BFD_RELOC_LARCH_SUB6: Self = Self(2335);
    pub const BFD_RELOC_LARCH_ADD_ULEB128: Self = Self(2336);
    pub const BFD_RELOC_LARCH_SUB_ULEB128: Self = Self(2337);
    pub const BFD_RELOC_LARCH_CALL36: Self = Self(2338);
    pub const BFD_RELOC_LARCH_TLS_DESC_PC_HI20: Self = Self(2339);
    pub const BFD_RELOC_LARCH_TLS_DESC_PC_LO12: Self = Self(2340);
    pub const BFD_RELOC_LARCH_TLS_DESC64_PC_LO20: Self = Self(2341);
    pub const BFD_RELOC_LARCH_TLS_DESC64_PC_HI12: Self = Self(2342);
    pub const BFD_RELOC_LARCH_TLS_DESC_HI20: Self = Self(2343);
    pub const BFD_RELOC_LARCH_TLS_DESC_LO12: Self = Self(2344);
    pub const BFD_RELOC_LARCH_TLS_DESC64_LO20: Self = Self(2345);
    pub const BFD_RELOC_LARCH_TLS_DESC64_HI12: Self = Self(2346);
    pub const BFD_RELOC_LARCH_TLS_DESC_LD: Self = Self(2347);
    pub const BFD_RELOC_LARCH_TLS_DESC_CALL: Self = Self(2348);
    pub const BFD_RELOC_LARCH_TLS_LE_HI20_R: Self = Self(2349);
    pub const BFD_RELOC_LARCH_TLS_LE_ADD_R: Self = Self(2350);
    pub const BFD_RELOC_LARCH_TLS_LE_LO12_R: Self = Self(2351);
    pub const BFD_RELOC_LARCH_TLS_LD_PCREL20_S2: Self = Self(2352);
    pub const BFD_RELOC_LARCH_TLS_GD_PCREL20_S2: Self = Self(2353);
    pub const BFD_RELOC_LARCH_TLS_DESC_PCREL20_S2: Self = Self(2354);
    pub const BFD_RELOC_LARCH_CALL30: Self = Self(2355);
    pub const BFD_RELOC_LARCH_PCADD_HI20: Self = Self(2356);
    pub const BFD_RELOC_LARCH_PCADD_LO12: Self = Self(2357);
    pub const BFD_RELOC_LARCH_GOT_PCADD_HI20: Self = Self(2358);
    pub const BFD_RELOC_LARCH_GOT_PCADD_LO12: Self = Self(2359);
    pub const BFD_RELOC_LARCH_TLS_IE_PCADD_HI20: Self = Self(2360);
    pub const BFD_RELOC_LARCH_TLS_IE_PCADD_LO12: Self = Self(2361);
    pub const BFD_RELOC_LARCH_TLS_LD_PCADD_HI20: Self = Self(2362);
    pub const BFD_RELOC_LARCH_TLS_LD_PCADD_LO12: Self = Self(2363);
    pub const BFD_RELOC_LARCH_TLS_GD_PCADD_HI20: Self = Self(2364);
    pub const BFD_RELOC_LARCH_TLS_GD_PCADD_LO12: Self = Self(2365);
    pub const BFD_RELOC_LARCH_TLS_DESC_PCADD_HI20: Self = Self(2366);
    pub const BFD_RELOC_LARCH_TLS_DESC_PCADD_LO12: Self = Self(2367);
    pub const BFD_RELOC_UNUSED: Self = Self(2368);
}
pub type asymbol = bfd_symbol;
pub type symbol_info = _symbol_info;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _symbol_info {
    pub value: symvalue,
    pub name: *const ::core::ffi::c_char,
    pub stab_name: *const ::core::ffi::c_char,
    pub stab_type: ::core::ffi::c_uchar,
    pub stab_other: ::core::ffi::c_char,
    pub stab_desc: ::core::ffi::c_short,
    pub r#type: ::core::ffi::c_char,
}
pub type bfd_print_symbol_type = bfd_print_symbol;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_print_symbol(pub ::core::ffi::c_uint);
impl bfd_print_symbol {
    pub const bfd_print_symbol_name: Self = Self(0);
    pub const bfd_print_symbol_more: Self = Self(1);
    pub const bfd_print_symbol_all: Self = Self(2);
}
pub type symindex = ::core::ffi::c_ulong;
pub type bfd_cleanup = Option<unsafe extern "C" fn(*mut bfd) -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_endian(pub ::core::ffi::c_uint);
impl bfd_endian {
    pub const BFD_ENDIAN_BIG: Self = Self(0);
    pub const BFD_ENDIAN_LITTLE: Self = Self(1);
    pub const BFD_ENDIAN_UNKNOWN: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_flavour(pub ::core::ffi::c_uint);
impl bfd_flavour {
    pub const bfd_target_unknown_flavour: Self = Self(0);
    pub const bfd_target_aout_flavour: Self = Self(1);
    pub const bfd_target_coff_flavour: Self = Self(2);
    pub const bfd_target_ecoff_flavour: Self = Self(3);
    pub const bfd_target_xcoff_flavour: Self = Self(4);
    pub const bfd_target_elf_flavour: Self = Self(5);
    pub const bfd_target_tekhex_flavour: Self = Self(6);
    pub const bfd_target_srec_flavour: Self = Self(7);
    pub const bfd_target_verilog_flavour: Self = Self(8);
    pub const bfd_target_ihex_flavour: Self = Self(9);
    pub const bfd_target_som_flavour: Self = Self(10);
    pub const bfd_target_msdos_flavour: Self = Self(11);
    pub const bfd_target_evax_flavour: Self = Self(12);
    pub const bfd_target_mmo_flavour: Self = Self(13);
    pub const bfd_target_mach_o_flavour: Self = Self(14);
    pub const bfd_target_pef_flavour: Self = Self(15);
    pub const bfd_target_pef_xlib_flavour: Self = Self(16);
    pub const bfd_target_sym_flavour: Self = Self(17);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct bfd_error(pub ::core::ffi::c_uint);
impl bfd_error {
    pub const bfd_error_no_error: Self = Self(0);
    pub const bfd_error_system_call: Self = Self(1);
    pub const bfd_error_invalid_target: Self = Self(2);
    pub const bfd_error_wrong_format: Self = Self(3);
    pub const bfd_error_wrong_object_format: Self = Self(4);
    pub const bfd_error_invalid_operation: Self = Self(5);
    pub const bfd_error_no_memory: Self = Self(6);
    pub const bfd_error_no_symbols: Self = Self(7);
    pub const bfd_error_no_armap: Self = Self(8);
    pub const bfd_error_no_more_archived_files: Self = Self(9);
    pub const bfd_error_malformed_archive: Self = Self(10);
    pub const bfd_error_missing_dso: Self = Self(11);
    pub const bfd_error_file_not_recognized: Self = Self(12);
    pub const bfd_error_file_ambiguously_recognized: Self = Self(13);
    pub const bfd_error_no_contents: Self = Self(14);
    pub const bfd_error_nonrepresentable_section: Self = Self(15);
    pub const bfd_error_no_debug_section: Self = Self(16);
    pub const bfd_error_bad_value: Self = Self(17);
    pub const bfd_error_file_truncated: Self = Self(18);
    pub const bfd_error_file_too_big: Self = Self(19);
    pub const bfd_error_sorry: Self = Self(20);
    pub const bfd_error_on_input: Self = Self(21);
    pub const bfd_error_invalid_error_code: Self = Self(22);
}
pub type bfd_error_type = bfd_error;
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
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const _sch_isblank: Self = Self(1);
    pub const _sch_iscntrl: Self = Self(2);
    pub const _sch_isdigit: Self = Self(4);
    pub const _sch_islower: Self = Self(8);
    pub const _sch_isprint: Self = Self(16);
    pub const _sch_ispunct: Self = Self(32);
    pub const _sch_isspace: Self = Self(64);
    pub const _sch_isupper: Self = Self(128);
    pub const _sch_isxdigit: Self = Self(256);
    pub const _sch_isidst: Self = Self(512);
    pub const _sch_isvsp: Self = Self(1024);
    pub const _sch_isnvsp: Self = Self(2048);
    pub const _sch_isalpha: Self = Self(136);
    pub const _sch_isalnum: Self = Self(140);
    pub const _sch_isidnum: Self = Self(516);
    pub const _sch_isgraph: Self = Self(172);
    pub const _sch_iscppsp: Self = Self(3072);
    pub const _sch_isbasic: Self = Self(3088);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct unicode_display_type(pub ::core::ffi::c_uint);
impl unicode_display_type {
    pub const unicode_default: Self = Self(0);
    pub const unicode_locale: Self = Self(1);
    pub const unicode_escape: Self = Self(2);
    pub const unicode_hex: Self = Self(3);
    pub const unicode_highlight: Self = Self(4);
    pub const unicode_invalid: Self = Self(5);
}
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
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
unsafe extern "C" fn putchar(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return putc(__c, stdout);
}
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const REPORT_BUGS_TO: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b"<https://sourceware.org/bugzilla/>\0",
    )
};
pub const FOPEN_RB: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"r\0") };
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const DEFAULT_STRINGS_ALL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PACKAGE: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"binutils\0") };
pub const SEC_ALLOC: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SEC_LOAD: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const SEC_HAS_CONTENTS: ::core::ffi::c_int = 0x100 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn bfd_section_size(mut sec: *const asection) -> bfd_size_type {
    return (*sec).size;
}
pub const BFD_INIT_MAGIC: usize = ::core::mem::size_of::<bfd_section>();
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut unicode_display: unicode_display_type = unicode_display_type::unicode_default;
pub const DATA_FLAGS: ::core::ffi::c_int = SEC_ALLOC | SEC_LOAD | SEC_HAS_CONTENTS;
static mut address_radix: ::core::ffi::c_int = 0;
static mut string_min: ::core::ffi::c_uint = 0;
static mut include_all_whitespace: bool = false;
static mut print_addresses: bool = false;
static mut print_filenames: bool = false;
static mut datasection_only: bool = false;
static mut target: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut encoding: ::core::ffi::c_char = 0;
static mut encoding_bytes: ::core::ffi::c_int = 0;
static mut output_separator: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut long_options: [option; 13] = [
    option {
        name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'a' as ::core::ffi::c_int,
    },
    option {
        name: b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"data\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"encoding\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'e' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'h' as ::core::ffi::c_int,
    },
    option {
        name: b"include-all-whitespace\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'w' as ::core::ffi::c_int,
    },
    option {
        name: b"output-separator\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"print-file-name\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'f' as ::core::ffi::c_int,
    },
    option {
        name: b"radix\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 't' as ::core::ffi::c_int,
    },
    option {
        name: b"target\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'T' as ::core::ffi::c_int,
    },
    option {
        name: b"unicode\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'U' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: 0 as ::core::ffi::c_int,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn set_string_min(mut arg: *const ::core::ffi::c_char) {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut l: ::core::ffi::c_ulong = strtoul(arg, &raw mut s, 0 as ::core::ffi::c_int);
    if !s.is_null() && *s as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        fatal(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"invalid integer argument %s\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            arg,
        );
    }
    string_min = l as ::core::ffi::c_uint;
    if l != string_min as ::core::ffi::c_ulong {
        fatal(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"minimum string length is too big: %s\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            arg,
        );
    }
    if string_min < 1 as ::core::ffi::c_uint {
        fatal(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"minimum string length is too small: %s\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            arg,
        );
    }
    if string_min.wrapping_add(1 as ::core::ffi::c_uint) == 0 as ::core::ffi::c_uint {
        fatal(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"minimum string length %s is too big\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            arg,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_strings(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut optc: ::core::ffi::c_int = 0;
    let mut exit_status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut files_given: bool = r#false != 0;
    let mut numeric_opt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    program_name = *argv.offset(0isize);
    xmalloc_set_program_name(program_name);
    bfd_set_error_program_name(program_name);
    expandargv(&raw mut argc, &raw mut argv);
    string_min = 4 as ::core::ffi::c_uint;
    include_all_whitespace = r#false != 0;
    print_addresses = r#false != 0;
    print_filenames = r#false != 0;
    datasection_only = r#false != 0;
    target = ::core::ptr::null_mut::<::core::ffi::c_char>();
    encoding = 's' as ::core::ffi::c_char;
    output_separator = ::core::ptr::null_mut::<::core::ffi::c_char>();
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"adfhHn:wot:e:T:s:U:Vv0123456789\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if optc == EOF {
            break;
        }
        match optc {
            97 => {
                datasection_only = r#false != 0;
            }
            100 => {
                datasection_only = r#true != 0;
            }
            102 => {
                print_filenames = r#true != 0;
            }
            72 | 104 => {
                usage(stdout, 0 as ::core::ffi::c_int);
            }
            110 => {
                set_string_min(optarg);
            }
            119 => {
                include_all_whitespace = r#true != 0;
            }
            111 => {
                print_addresses = r#true != 0;
                address_radix = 8 as ::core::ffi::c_int;
            }
            116 => {
                print_addresses = r#true != 0;
                if *optarg.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    usage(stderr, 1 as ::core::ffi::c_int);
                }
                match *optarg.offset(0isize) as ::core::ffi::c_int {
                    111 => {
                        address_radix = 8 as ::core::ffi::c_int;
                    }
                    100 => {
                        address_radix = 10 as ::core::ffi::c_int;
                    }
                    120 => {
                        address_radix = 16 as ::core::ffi::c_int;
                    }
                    _ => {
                        usage(stderr, 1 as ::core::ffi::c_int);
                    }
                }
            }
            84 => {
                target = optarg;
            }
            101 => {
                if *optarg.offset(1isize) as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    usage(stderr, 1 as ::core::ffi::c_int);
                }
                encoding = *optarg.offset(0isize);
            }
            115 => {
                output_separator = optarg;
            }
            85 => {
                if strcmp(optarg, b"default\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(optarg, b"d\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    unicode_display = unicode_display_type::unicode_default;
                } else if strcmp(optarg, b"locale\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(optarg, b"l\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    unicode_display = unicode_display_type::unicode_locale;
                } else if strcmp(optarg, b"escape\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(optarg, b"e\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    unicode_display = unicode_display_type::unicode_escape;
                } else if strcmp(optarg, b"invalid\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(optarg, b"i\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    unicode_display = unicode_display_type::unicode_invalid;
                } else if strcmp(optarg, b"hex\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                    || strcmp(optarg, b"x\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    unicode_display = unicode_display_type::unicode_hex;
                } else if strcmp(
                    optarg,
                    b"highlight\0".as_ptr() as *const ::core::ffi::c_char,
                ) == 0 as ::core::ffi::c_int
                    || strcmp(optarg, b"h\0".as_ptr() as *const ::core::ffi::c_char)
                        == 0 as ::core::ffi::c_int
                {
                    unicode_display = unicode_display_type::unicode_highlight;
                } else {
                    fatal(
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"invalid argument to -U/--unicode: %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        optarg,
                    );
                }
            }
            86 | 118 => {
                print_version(b"strings\0".as_ptr() as *const ::core::ffi::c_char);
            }
            63 => {
                usage(stderr, 1 as ::core::ffi::c_int);
            }
            _ => {
                numeric_opt = optind;
            }
        }
    }
    if unicode_display.0 != unicode_display_type::unicode_default.0 {
        encoding = 'S' as ::core::ffi::c_char;
    }
    if numeric_opt != 0 as ::core::ffi::c_int {
        set_string_min(
            (*argv.offset((numeric_opt - 1 as ::core::ffi::c_int) as isize))
                .offset(1 as ::core::ffi::c_int as isize),
        );
    }
    match encoding as ::core::ffi::c_int {
        83 | 115 => {
            encoding_bytes = 1 as ::core::ffi::c_int;
        }
        98 | 108 => {
            encoding_bytes = 2 as ::core::ffi::c_int;
        }
        66 | 76 => {
            encoding_bytes = 4 as ::core::ffi::c_int;
        }
        _ => {
            usage(stderr, 1 as ::core::ffi::c_int);
        }
    }
    if bfd_init() as usize != BFD_INIT_MAGIC {
        fatal(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"fatal error: libbfd ABI mismatch\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
    set_default_bfd_target();
    if optind >= argc {
        datasection_only = r#false != 0;
        print_strings(
            b"{standard input}\0".as_ptr() as *const ::core::ffi::c_char,
            stdin,
            0 as file_ptr,
            0 as ::core::ffi::c_int,
            NULL as *mut ::core::ffi::c_char,
        );
        files_given = r#true != 0;
    } else {
        while optind < argc {
            if strcmp(
                *argv.offset(optind as isize),
                b"-\0".as_ptr() as *const ::core::ffi::c_char,
            ) == 0 as ::core::ffi::c_int
            {
                datasection_only = r#false != 0;
            } else {
                files_given = r#true != 0;
                exit_status |= !strings_file(*argv.offset(optind as isize)) as ::core::ffi::c_int;
            }
            optind += 1;
        }
    }
    if !files_given {
        usage(stderr, 1 as ::core::ffi::c_int);
    }
    return exit_status;
}
unsafe extern "C" fn strings_a_section(
    mut abfd: *mut bfd,
    mut sect: *mut asection,
    mut filename: *const ::core::ffi::c_char,
    mut got_a_section: *mut bool,
) {
    let mut sectsize: bfd_size_type = 0;
    let mut mem: *mut bfd_byte = ::core::ptr::null_mut::<bfd_byte>();
    if (*sect).flags & DATA_FLAGS as flagword != DATA_FLAGS as flagword {
        return;
    }
    sectsize = bfd_section_size(sect);
    if sectsize == 0 as bfd_size_type {
        return;
    }
    if !bfd_malloc_and_get_section(abfd, sect, &raw mut mem) {
        non_fatal(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: Reading section %s failed: %s\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            filename,
            (*sect).name,
            bfd_errmsg(bfd_get_error()),
        );
        return;
    }
    *got_a_section = r#true != 0;
    print_strings(
        filename,
        ::core::ptr::null_mut::<FILE>(),
        (*sect).filepos,
        sectsize as ::core::ffi::c_int,
        mem as *mut ::core::ffi::c_char,
    );
    free(mem as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn strings_object_file(mut file: *const ::core::ffi::c_char) -> bool {
    let mut abfd: *mut bfd = ::core::ptr::null_mut::<bfd>();
    let mut s: *mut asection = ::core::ptr::null_mut::<asection>();
    let mut got_a_section: bool = false;
    abfd = bfd_openr(file, target);
    if abfd.is_null() {
        return r#false != 0;
    }
    if !bfd_check_format(abfd, bfd_format::bfd_object) {
        bfd_close(abfd);
        return r#false != 0;
    }
    got_a_section = r#false != 0;
    s = (*abfd).sections as *mut asection;
    while !s.is_null() {
        strings_a_section(abfd, s, file, &raw mut got_a_section);
        s = (*s).next as *mut asection;
    }
    if !bfd_close(abfd) {
        bfd_nonfatal(file);
        return r#false != 0;
    }
    return got_a_section;
}
unsafe extern "C" fn strings_file(mut file: *mut ::core::ffi::c_char) -> bool {
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
    if stat(file, &raw mut st) < 0 as ::core::ffi::c_int {
        if *__errno_location() == ENOENT {
            non_fatal(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"'%s': No such file\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                file,
            );
        } else {
            non_fatal(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Warning: could not locate '%s'.  reason: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                file,
                strerror(*__errno_location()),
            );
        }
        return r#false != 0;
    } else if st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t {
        non_fatal(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Warning: '%s' is a directory\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            file,
        );
        return r#false != 0;
    }
    if !datasection_only || !strings_object_file(file) {
        let mut stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
        stream = fopen(file, FOPEN_RB.as_ptr()) as *mut FILE;
        if stream.is_null() {
            fprintf(
                stderr,
                b"%s: \0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
            );
            perror(file);
            return r#false != 0;
        }
        print_strings(
            file,
            stream,
            0 as ::core::ffi::c_int as file_ptr,
            0 as ::core::ffi::c_int,
            NULL as *mut ::core::ffi::c_char,
        );
        if fclose(stream) == EOF {
            fprintf(
                stderr,
                b"%s: \0".as_ptr() as *const ::core::ffi::c_char,
                program_name,
            );
            perror(file);
            return r#false != 0;
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn get_char(
    mut stream: *mut FILE,
    mut address: *mut file_ptr,
    mut magiccount: *mut ::core::ffi::c_int,
    mut magic: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_long {
    let mut c: ::core::ffi::c_int = 0;
    let mut i: ::core::ffi::c_int = 0;
    let mut r: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
    i = 0 as ::core::ffi::c_int;
    while i < encoding_bytes {
        if *magiccount != 0 {
            *magiccount -= 1;
            let c2rust_fresh1 = *magic;
            *magic = (*magic).offset(1);
            c = *c2rust_fresh1 as ::core::ffi::c_int;
        } else {
            if stream.is_null() {
                return EOF as ::core::ffi::c_long;
            }
            c = getc_unlocked(stream);
            if c == EOF {
                return EOF as ::core::ffi::c_long;
            }
        }
        *address += 1;
        r = r << 8 as ::core::ffi::c_int | (c & 0xff as ::core::ffi::c_int) as ::core::ffi::c_long;
        i += 1;
    }
    match encoding as ::core::ffi::c_int {
        108 => {
            r = (r & 0xff as ::core::ffi::c_long) << 8 as ::core::ffi::c_int
                | (r & 0xff00 as ::core::ffi::c_long) >> 8 as ::core::ffi::c_int;
        }
        76 => {
            r = (r & 0xff as ::core::ffi::c_long) << 24 as ::core::ffi::c_int
                | (r & 0xff00 as ::core::ffi::c_long) << 8 as ::core::ffi::c_int
                | (r & 0xff0000 as ::core::ffi::c_long) >> 8 as ::core::ffi::c_int
                | (r & 0xff000000 as ::core::ffi::c_uint as ::core::ffi::c_long)
                    >> 24 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return r;
}
unsafe extern "C" fn unget_part_char(
    mut c: ::core::ffi::c_long,
    mut address: *mut file_ptr,
    mut magiccount: *mut ::core::ffi::c_int,
    mut magic: *mut *mut ::core::ffi::c_char,
) {
    static mut tmp: [::core::ffi::c_char; 4] = [0; 4];
    if encoding_bytes > 1 as ::core::ffi::c_int {
        *address -= (encoding_bytes - 1 as ::core::ffi::c_int) as file_ptr;
        if *magiccount == 0 as ::core::ffi::c_int {
            match encoding as ::core::ffi::c_int {
                98 => {
                    tmp[0usize] = (c & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
                    *magiccount = 1 as ::core::ffi::c_int;
                }
                108 => {
                    tmp[0usize] = (c >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
                        as ::core::ffi::c_char;
                    *magiccount = 1 as ::core::ffi::c_int;
                }
                66 => {
                    tmp[0usize] = (c >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
                        as ::core::ffi::c_char;
                    tmp[1usize] = (c >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
                        as ::core::ffi::c_char;
                    tmp[2usize] = (c & 0xff as ::core::ffi::c_long) as ::core::ffi::c_char;
                    *magiccount = 3 as ::core::ffi::c_int;
                }
                76 => {
                    tmp[0usize] = (c >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
                        as ::core::ffi::c_char;
                    tmp[1usize] = (c >> 16 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
                        as ::core::ffi::c_char;
                    tmp[2usize] = (c >> 24 as ::core::ffi::c_int & 0xff as ::core::ffi::c_long)
                        as ::core::ffi::c_char;
                    *magiccount = 3 as ::core::ffi::c_int;
                }
                _ => {}
            }
            *magic = &raw mut tmp as *mut ::core::ffi::c_char;
        } else {
            *magic = (*magic).offset(-((encoding_bytes - 1 as ::core::ffi::c_int) as isize));
            *magiccount += encoding_bytes - 1 as ::core::ffi::c_int;
        }
    }
}
unsafe extern "C" fn print_filename_and_address(
    mut filename: *const ::core::ffi::c_char,
    mut address: file_ptr,
) {
    if print_filenames {
        printf(b"%s: \0".as_ptr() as *const ::core::ffi::c_char, filename);
    }
    if !print_addresses {
        return;
    }
    match address_radix {
        8 => {
            if ::core::mem::size_of::<file_ptr>() > ::core::mem::size_of::<::core::ffi::c_long>() {
                printf(
                    b"%7llo \0".as_ptr() as *const ::core::ffi::c_char,
                    address as ::core::ffi::c_ulonglong,
                );
            } else {
                printf(
                    b"%7lo \0".as_ptr() as *const ::core::ffi::c_char,
                    address as ::core::ffi::c_ulong,
                );
            }
        }
        10 => {
            if ::core::mem::size_of::<file_ptr>() > ::core::mem::size_of::<::core::ffi::c_long>() {
                printf(
                    b"%7llu \0".as_ptr() as *const ::core::ffi::c_char,
                    address as ::core::ffi::c_ulonglong,
                );
            } else {
                printf(
                    b"%7ld \0".as_ptr() as *const ::core::ffi::c_char,
                    address as ::core::ffi::c_long,
                );
            }
        }
        16 => {
            if ::core::mem::size_of::<file_ptr>() > ::core::mem::size_of::<::core::ffi::c_long>() {
                printf(
                    b"%7llx \0".as_ptr() as *const ::core::ffi::c_char,
                    address as ::core::ffi::c_ulonglong,
                );
            } else {
                printf(
                    b"%7lx \0".as_ptr() as *const ::core::ffi::c_char,
                    address as ::core::ffi::c_ulong,
                );
            }
        }
        _ => {}
    };
}
unsafe extern "C" fn is_valid_utf8(
    mut buffer: *const ::core::ffi::c_uchar,
    mut buflen: ::core::ffi::c_ulong,
) -> ::core::ffi::c_uint {
    if (*buffer.offset(0isize) as ::core::ffi::c_int) < 0xc0 as ::core::ffi::c_int {
        return 0 as ::core::ffi::c_uint;
    }
    if buflen < 2 as ::core::ffi::c_ulong {
        return 0 as ::core::ffi::c_uint;
    }
    if *buffer.offset(1isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_uint;
    }
    if *buffer.offset(0isize) as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return 2 as ::core::ffi::c_uint;
    }
    if buflen < 3 as ::core::ffi::c_ulong {
        return 0 as ::core::ffi::c_uint;
    }
    if *buffer.offset(2isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_uint;
    }
    if *buffer.offset(0isize) as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        return 3 as ::core::ffi::c_uint;
    }
    if buflen < 4 as ::core::ffi::c_ulong {
        return 0 as ::core::ffi::c_uint;
    }
    if *buffer.offset(3isize) as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_uint;
    }
    return 4 as ::core::ffi::c_uint;
}
unsafe extern "C" fn display_utf8_char(
    mut buffer: *const ::core::ffi::c_uchar,
) -> ::core::ffi::c_uint {
    let mut j: ::core::ffi::c_uint = 0;
    let mut utf8_len: ::core::ffi::c_uint = 0;
    match *buffer.offset(0isize) as ::core::ffi::c_int & 0x30 as ::core::ffi::c_int {
        0 | 0x10 => {
            utf8_len = 2 as ::core::ffi::c_uint;
        }
        0x20 => {
            utf8_len = 3 as ::core::ffi::c_uint;
        }
        _ => {
            utf8_len = 4 as ::core::ffi::c_uint;
        }
    }
    match unicode_display {
        unicode_display_type::unicode_escape | unicode_display_type::unicode_highlight => {
            if unicode_display.0 == unicode_display_type::unicode_highlight.0
                && isatty(1 as ::core::ffi::c_int) != 0
            {
                printf(b"\x1B[31;47m\0".as_ptr() as *const ::core::ffi::c_char);
            }
            match utf8_len {
                2 => {
                    printf(
                        b"\\u%02x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                        (*buffer.offset(0isize) as ::core::ffi::c_int & 0x1c as ::core::ffi::c_int)
                            >> 2 as ::core::ffi::c_int,
                        (*buffer.offset(0isize) as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int
                            | *buffer.offset(1isize) as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_int,
                    );
                }
                3 => {
                    printf(
                        b"\\u%02x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                        (*buffer.offset(0isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                            << 4 as ::core::ffi::c_int
                            | (*buffer.offset(1isize) as ::core::ffi::c_int
                                & 0x3c as ::core::ffi::c_int)
                                >> 2 as ::core::ffi::c_int,
                        (*buffer.offset(1isize) as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int
                            | *buffer.offset(2isize) as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_int,
                    );
                }
                4 => {
                    printf(
                        b"\\u%02x%02x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                        (*buffer.offset(0isize) as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
                            << 2 as ::core::ffi::c_int
                            | (*buffer.offset(1isize) as ::core::ffi::c_int
                                & 0x30 as ::core::ffi::c_int)
                                >> 4 as ::core::ffi::c_int,
                        (*buffer.offset(1isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                            << 4 as ::core::ffi::c_int
                            | (*buffer.offset(2isize) as ::core::ffi::c_int
                                & 0x3c as ::core::ffi::c_int)
                                >> 2 as ::core::ffi::c_int,
                        (*buffer.offset(2isize) as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                            << 6 as ::core::ffi::c_int
                            | *buffer.offset(3isize) as ::core::ffi::c_int
                                & 0x3f as ::core::ffi::c_int,
                    );
                }
                _ => {}
            }
            if unicode_display.0 == unicode_display_type::unicode_highlight.0
                && isatty(1 as ::core::ffi::c_int) != 0
            {
                printf(b"\x1B[0m\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
        unicode_display_type::unicode_hex => {
            putchar('<' as ::core::ffi::c_int);
            printf(b"0x\0".as_ptr() as *const ::core::ffi::c_char);
            j = 0 as ::core::ffi::c_uint;
            while j < utf8_len {
                printf(
                    b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    *buffer.offset(j as isize) as ::core::ffi::c_int,
                );
                j = j.wrapping_add(1);
            }
            putchar('>' as ::core::ffi::c_int);
        }
        unicode_display_type::unicode_locale => {
            printf(b"%.1s\0".as_ptr() as *const ::core::ffi::c_char, buffer);
        }
        _ => {
            fprintf(
                stderr,
                b"ICE: unexpected unicode display type\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    return utf8_len;
}
unsafe extern "C" fn print_unicode_buffer(
    mut filename: *const ::core::ffi::c_char,
    mut address: file_ptr,
    mut buffer: *const ::core::ffi::c_uchar,
    mut buflen: ::core::ffi::c_ulong,
) {
    if filename.is_null()
        || buffer.is_null()
        || unicode_display.0 == unicode_display_type::unicode_default.0
        || encoding as ::core::ffi::c_int != 'S' as ::core::ffi::c_int
        || encoding_bytes != 1 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"ICE: bad arguments to print_unicode_buffer\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    if buflen == 0 as ::core::ffi::c_ulong {
        return;
    }
    let mut start_point: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut i: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
    let mut char_len: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
    let mut num_found: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    i = 0 as ::core::ffi::c_ulong;
    's_111: while i < buflen {
        let mut c: ::core::ffi::c_int = *buffer.offset(i as isize) as ::core::ffi::c_int;
        char_len = 1 as ::core::ffi::c_uint;
        's_32: {
            if !(c >= 0 as ::core::ffi::c_int
                && c <= 255 as ::core::ffi::c_int
                && (c == '\t' as ::core::ffi::c_int
                    || _sch_istable[(c & 0xff as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_4::_sch_isprint.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0
                    || encoding as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                        && c > 127 as ::core::ffi::c_int
                    || include_all_whitespace as ::core::ffi::c_int != 0
                        && _sch_istable[(c & 0xff as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            & C2Rust_Unnamed_4::_sch_isspace.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0))
            {
                num_found = 0 as ::core::ffi::c_uint;
            } else {
                if c > 126 as ::core::ffi::c_int {
                    if c < 0xc0 as ::core::ffi::c_int {
                        num_found = 0 as ::core::ffi::c_uint;
                        break 's_32;
                    } else {
                        char_len = is_valid_utf8(buffer.offset(i as isize), buflen.wrapping_sub(i));
                        if char_len == 0 as ::core::ffi::c_uint {
                            char_len = 1 as ::core::ffi::c_uint;
                            num_found = 0 as ::core::ffi::c_uint;
                            break 's_32;
                        } else if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                            num_found = 0 as ::core::ffi::c_uint;
                            break 's_32;
                        }
                    }
                }
                if num_found == 0 as ::core::ffi::c_uint {
                    start_point = i;
                }
                num_found = num_found.wrapping_add(1);
                if num_found >= string_min {
                    break 's_111;
                }
            }
        }
        i = i.wrapping_add(char_len as ::core::ffi::c_ulong);
    }
    if num_found < string_min {
        return;
    }
    print_filename_and_address(
        filename,
        (address as ::core::ffi::c_ulong).wrapping_add(start_point) as file_ptr,
    );
    i = start_point;
    while i < buflen {
        let mut c_0: ::core::ffi::c_int = *buffer.offset(i as isize) as ::core::ffi::c_int;
        char_len = 1 as ::core::ffi::c_uint;
        if !(c_0 >= 0 as ::core::ffi::c_int
            && c_0 <= 255 as ::core::ffi::c_int
            && (c_0 == '\t' as ::core::ffi::c_int
                || _sch_istable[(c_0 & 0xff as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    & C2Rust_Unnamed_4::_sch_isprint.0 as ::core::ffi::c_int
                        as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                || encoding as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                    && c_0 > 127 as ::core::ffi::c_int
                || include_all_whitespace as ::core::ffi::c_int != 0
                    && _sch_istable[(c_0 & 0xff as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_4::_sch_isspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0))
        {
            break;
        }
        if c_0 < 127 as ::core::ffi::c_int {
            putchar(c_0);
        } else {
            if is_valid_utf8(buffer.offset(i as isize), buflen.wrapping_sub(i)) == 0 {
                break;
            }
            if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                break;
            }
            char_len = display_utf8_char(buffer.offset(i as isize));
        }
        i = i.wrapping_add(char_len as ::core::ffi::c_ulong);
    }
    if !output_separator.is_null() {
        fputs(output_separator, stdout);
    } else {
        putchar('\n' as ::core::ffi::c_int);
    }
    print_unicode_buffer(
        filename,
        (address as ::core::ffi::c_ulong).wrapping_add(i) as file_ptr,
        buffer.offset(i as isize),
        buflen.wrapping_sub(i),
    );
}
unsafe extern "C" fn get_unicode_byte(
    mut stream: *mut FILE,
    mut putback: *mut ::core::ffi::c_uchar,
    mut num_putback: *mut ::core::ffi::c_uint,
    mut num_read: *mut ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    if *num_putback > 0 as ::core::ffi::c_uint {
        *num_putback = (*num_putback).wrapping_sub(1 as ::core::ffi::c_uint);
        return *putback.offset(*num_putback as isize) as ::core::ffi::c_int;
    }
    *num_read = (*num_read).wrapping_add(1 as ::core::ffi::c_uint);
    return getc_unlocked(stream);
}
unsafe extern "C" fn print_unicode_stream_body(
    mut filename: *const ::core::ffi::c_char,
    mut address: file_ptr,
    mut stream: *mut FILE,
    mut putback_buf: *mut ::core::ffi::c_uchar,
    mut num_putback: ::core::ffi::c_uint,
    mut print_buf: *mut ::core::ffi::c_uchar,
) {
    let mut start_point: file_ptr = 0 as file_ptr;
    let mut num_read: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut num_chars: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut num_print: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut c: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while num_chars < string_min {
        c = get_unicode_byte(stream, putback_buf, &raw mut num_putback, &raw mut num_read);
        if c == EOF {
            break;
        }
        if !(c >= 0 as ::core::ffi::c_int
            && c <= 255 as ::core::ffi::c_int
            && (c == '\t' as ::core::ffi::c_int
                || _sch_istable[(c & 0xff as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
                    & C2Rust_Unnamed_4::_sch_isprint.0 as ::core::ffi::c_int
                        as ::core::ffi::c_ushort as ::core::ffi::c_int
                    != 0
                || encoding as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                    && c > 127 as ::core::ffi::c_int
                || include_all_whitespace as ::core::ffi::c_int != 0
                    && _sch_istable[(c & 0xff as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_4::_sch_isspace.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0))
        {
            num_print = 0 as ::core::ffi::c_uint;
            num_chars = num_print;
        } else {
            if num_chars == 0 as ::core::ffi::c_uint {
                start_point = num_read.wrapping_sub(1 as ::core::ffi::c_uint) as file_ptr;
            }
            if c < 127 as ::core::ffi::c_int {
                *print_buf.offset(num_print as isize) = c as ::core::ffi::c_uchar;
                num_chars = num_chars.wrapping_add(1);
                num_print = num_print.wrapping_add(1);
            } else if c < 0xc0 as ::core::ffi::c_int {
                num_print = 0 as ::core::ffi::c_uint;
                num_chars = num_print;
            } else {
                let mut utf8: [::core::ffi::c_char; 4] = [0; 4];
                utf8[0usize] = c as ::core::ffi::c_char;
                c = get_unicode_byte(stream, putback_buf, &raw mut num_putback, &raw mut num_read);
                if c == EOF {
                    break;
                }
                utf8[1usize] = c as ::core::ffi::c_char;
                if utf8[1usize] as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    let c2rust_fresh2 = num_putback;
                    num_putback = num_putback.wrapping_add(1);
                    *putback_buf.offset(c2rust_fresh2 as isize) =
                        utf8[1usize] as ::core::ffi::c_uchar;
                    num_print = 0 as ::core::ffi::c_uint;
                    num_chars = num_print;
                } else if utf8[0usize] as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                        let c2rust_fresh3 = num_putback;
                        num_putback = num_putback.wrapping_add(1);
                        *putback_buf.offset(c2rust_fresh3 as isize) =
                            utf8[1usize] as ::core::ffi::c_uchar;
                        num_print = 0 as ::core::ffi::c_uint;
                        num_chars = num_print;
                    } else {
                        let c2rust_fresh4 = num_print;
                        num_print = num_print.wrapping_add(1);
                        *print_buf.offset(c2rust_fresh4 as isize) =
                            utf8[0usize] as ::core::ffi::c_uchar;
                        let c2rust_fresh5 = num_print;
                        num_print = num_print.wrapping_add(1);
                        *print_buf.offset(c2rust_fresh5 as isize) =
                            utf8[1usize] as ::core::ffi::c_uchar;
                        num_chars = num_chars.wrapping_add(1);
                    }
                } else {
                    c = get_unicode_byte(
                        stream,
                        putback_buf,
                        &raw mut num_putback,
                        &raw mut num_read,
                    );
                    if c == EOF {
                        break;
                    }
                    utf8[2usize] = c as ::core::ffi::c_char;
                    if utf8[2usize] as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        != 0x80 as ::core::ffi::c_int
                    {
                        let c2rust_fresh6 = num_putback;
                        num_putback = num_putback.wrapping_add(1);
                        *putback_buf.offset(c2rust_fresh6 as isize) =
                            utf8[2usize] as ::core::ffi::c_uchar;
                        let c2rust_fresh7 = num_putback;
                        num_putback = num_putback.wrapping_add(1);
                        *putback_buf.offset(c2rust_fresh7 as isize) =
                            utf8[1usize] as ::core::ffi::c_uchar;
                        num_print = 0 as ::core::ffi::c_uint;
                        num_chars = num_print;
                    } else if utf8[0usize] as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                            let c2rust_fresh8 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh8 as isize) =
                                utf8[2usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh9 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh9 as isize) =
                                utf8[1usize] as ::core::ffi::c_uchar;
                            num_print = 0 as ::core::ffi::c_uint;
                            num_chars = num_print;
                        } else {
                            let c2rust_fresh10 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh10 as isize) =
                                utf8[0usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh11 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh11 as isize) =
                                utf8[1usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh12 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh12 as isize) =
                                utf8[2usize] as ::core::ffi::c_uchar;
                            num_chars = num_chars.wrapping_add(1);
                        }
                    } else {
                        c = get_unicode_byte(
                            stream,
                            putback_buf,
                            &raw mut num_putback,
                            &raw mut num_read,
                        );
                        if c == EOF {
                            break;
                        }
                        utf8[3usize] = c as ::core::ffi::c_char;
                        if utf8[3usize] as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            != 0x80 as ::core::ffi::c_int
                        {
                            let c2rust_fresh13 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh13 as isize) =
                                utf8[3usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh14 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh14 as isize) =
                                utf8[2usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh15 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh15 as isize) =
                                utf8[1usize] as ::core::ffi::c_uchar;
                            num_print = 0 as ::core::ffi::c_uint;
                            num_chars = num_print;
                        } else if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                            let c2rust_fresh16 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh16 as isize) =
                                utf8[3usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh17 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh17 as isize) =
                                utf8[1usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh18 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh18 as isize) =
                                utf8[2usize] as ::core::ffi::c_uchar;
                            num_print = 0 as ::core::ffi::c_uint;
                            num_chars = num_print;
                        } else {
                            let c2rust_fresh19 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh19 as isize) =
                                utf8[0usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh20 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh20 as isize) =
                                utf8[1usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh21 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh21 as isize) =
                                utf8[2usize] as ::core::ffi::c_uchar;
                            let c2rust_fresh22 = num_print;
                            num_print = num_print.wrapping_add(1);
                            *print_buf.offset(c2rust_fresh22 as isize) =
                                utf8[3usize] as ::core::ffi::c_uchar;
                            num_chars = num_chars.wrapping_add(1);
                        }
                    }
                }
            }
        }
    }
    if num_chars >= string_min {
        print_filename_and_address(filename, address + start_point);
        let mut i: ::core::ffi::c_uint = 0;
        i = 0 as ::core::ffi::c_uint;
        while i < num_print {
            if (*print_buf.offset(i as isize) as ::core::ffi::c_int) < 127 as ::core::ffi::c_int {
                let c2rust_fresh23 = i;
                i = i.wrapping_add(1);
                putchar(*print_buf.offset(c2rust_fresh23 as isize) as ::core::ffi::c_int);
            } else {
                i = i.wrapping_add(display_utf8_char(print_buf.offset(i as isize)));
            }
        }
        loop {
            c = get_unicode_byte(stream, putback_buf, &raw mut num_putback, &raw mut num_read);
            if c == EOF {
                break;
            }
            if !(c >= 0 as ::core::ffi::c_int
                && c <= 255 as ::core::ffi::c_int
                && (c == '\t' as ::core::ffi::c_int
                    || _sch_istable[(c & 0xff as ::core::ffi::c_int) as usize]
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_4::_sch_isprint.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0
                    || encoding as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                        && c > 127 as ::core::ffi::c_int
                    || include_all_whitespace as ::core::ffi::c_int != 0
                        && _sch_istable[(c & 0xff as ::core::ffi::c_int) as usize]
                            as ::core::ffi::c_int
                            & C2Rust_Unnamed_4::_sch_isspace.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0))
            {
                break;
            }
            if c < 127 as ::core::ffi::c_int {
                putchar(c);
            } else {
                if c < 0xc0 as ::core::ffi::c_int {
                    break;
                }
                let mut utf8_0: [::core::ffi::c_uchar; 4] = [0; 4];
                utf8_0[0usize] = c as ::core::ffi::c_uchar;
                c = get_unicode_byte(stream, putback_buf, &raw mut num_putback, &raw mut num_read);
                if c == EOF {
                    break;
                }
                utf8_0[1usize] = c as ::core::ffi::c_uchar;
                if utf8_0[1usize] as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                    != 0x80 as ::core::ffi::c_int
                {
                    let c2rust_fresh24 = num_putback;
                    num_putback = num_putback.wrapping_add(1);
                    *putback_buf.offset(c2rust_fresh24 as isize) = utf8_0[1usize];
                    break;
                } else if utf8_0[0usize] as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                        let c2rust_fresh25 = num_putback;
                        num_putback = num_putback.wrapping_add(1);
                        *putback_buf.offset(c2rust_fresh25 as isize) = utf8_0[1usize];
                        break;
                    } else {
                        display_utf8_char(&raw mut utf8_0 as *mut ::core::ffi::c_uchar);
                    }
                } else {
                    c = get_unicode_byte(
                        stream,
                        putback_buf,
                        &raw mut num_putback,
                        &raw mut num_read,
                    );
                    if c == EOF {
                        break;
                    }
                    utf8_0[2usize] = c as ::core::ffi::c_uchar;
                    if utf8_0[2usize] as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                        != 0x80 as ::core::ffi::c_int
                    {
                        let c2rust_fresh26 = num_putback;
                        num_putback = num_putback.wrapping_add(1);
                        *putback_buf.offset(c2rust_fresh26 as isize) = utf8_0[2usize];
                        let c2rust_fresh27 = num_putback;
                        num_putback = num_putback.wrapping_add(1);
                        *putback_buf.offset(c2rust_fresh27 as isize) = utf8_0[1usize];
                        break;
                    } else if utf8_0[0usize] as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                    {
                        if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                            let c2rust_fresh28 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh28 as isize) = utf8_0[2usize];
                            let c2rust_fresh29 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh29 as isize) = utf8_0[1usize];
                            break;
                        } else {
                            display_utf8_char(&raw mut utf8_0 as *mut ::core::ffi::c_uchar);
                        }
                    } else {
                        c = get_unicode_byte(
                            stream,
                            putback_buf,
                            &raw mut num_putback,
                            &raw mut num_read,
                        );
                        if c == EOF {
                            break;
                        }
                        utf8_0[3usize] = c as ::core::ffi::c_uchar;
                        if utf8_0[3usize] as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int
                            != 0x80 as ::core::ffi::c_int
                        {
                            let c2rust_fresh30 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh30 as isize) = utf8_0[3usize];
                            let c2rust_fresh31 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh31 as isize) = utf8_0[2usize];
                            let c2rust_fresh32 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh32 as isize) = utf8_0[1usize];
                            break;
                        } else if unicode_display.0 == unicode_display_type::unicode_invalid.0 {
                            let c2rust_fresh33 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh33 as isize) = utf8_0[3usize];
                            let c2rust_fresh34 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh34 as isize) = utf8_0[2usize];
                            let c2rust_fresh35 = num_putback;
                            num_putback = num_putback.wrapping_add(1);
                            *putback_buf.offset(c2rust_fresh35 as isize) = utf8_0[1usize];
                            break;
                        } else {
                            display_utf8_char(&raw mut utf8_0 as *mut ::core::ffi::c_uchar);
                        }
                    }
                }
            }
        }
        if !output_separator.is_null() {
            fputs(output_separator, stdout);
        } else {
            putchar('\n' as ::core::ffi::c_int);
        }
    }
    if c != EOF {
        print_unicode_stream_body(
            filename,
            address + num_read as file_ptr,
            stream,
            putback_buf,
            num_putback,
            print_buf,
        );
    }
}
unsafe extern "C" fn print_unicode_stream(
    mut filename: *const ::core::ffi::c_char,
    mut address: file_ptr,
    mut stream: *mut FILE,
) {
    if filename.is_null()
        || stream.is_null()
        || unicode_display.0 == unicode_display_type::unicode_default.0
        || encoding as ::core::ffi::c_int != 'S' as ::core::ffi::c_int
        || encoding_bytes != 1 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"ICE: bad arguments to print_unicode_stream\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return;
    }
    let mut amt: size_t = string_min as size_t;
    amt = (4 as size_t).wrapping_mul(amt).wrapping_add(1 as size_t);
    let mut print_buf: *mut ::core::ffi::c_uchar = xmalloc(amt) as *mut ::core::ffi::c_uchar;
    let mut putback_buf: [::core::ffi::c_uchar; 5] = [0; 5];
    let mut num_putback: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    print_unicode_stream_body(
        filename,
        address,
        stream,
        &raw mut putback_buf as *mut ::core::ffi::c_uchar,
        num_putback,
        print_buf,
    );
    free(print_buf as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn print_strings(
    mut filename: *const ::core::ffi::c_char,
    mut stream: *mut FILE,
    mut address: file_ptr,
    mut magiccount: ::core::ffi::c_int,
    mut magic: *mut ::core::ffi::c_char,
) {
    if unicode_display.0 != unicode_display_type::unicode_default.0 {
        if !magic.is_null() {
            print_unicode_buffer(
                filename,
                address,
                magic as *const ::core::ffi::c_uchar,
                magiccount as ::core::ffi::c_ulong,
            );
        }
        if !stream.is_null() {
            print_unicode_stream(filename, address, stream);
        }
        return;
    }
    let mut buf: *mut ::core::ffi::c_char = xmalloc(
        ::core::mem::size_of::<::core::ffi::c_char>()
            .wrapping_mul(string_min.wrapping_add(1 as ::core::ffi::c_uint) as size_t),
    ) as *mut ::core::ffi::c_char;
    loop {
        let mut start: file_ptr = 0;
        let mut i: ::core::ffi::c_uint = 0;
        let mut c: ::core::ffi::c_long = 0;
        's_85: loop {
            start = address;
            i = 0 as ::core::ffi::c_uint;
            loop {
                if i >= string_min {
                    break 's_85;
                }
                c = get_char(
                    stream,
                    &raw mut address,
                    &raw mut magiccount,
                    &raw mut magic,
                );
                if c == EOF as ::core::ffi::c_long {
                    free(buf as *mut ::core::ffi::c_void);
                    return;
                }
                if !(c >= 0 as ::core::ffi::c_long
                    && c <= 255 as ::core::ffi::c_long
                    && (c == '\t' as ::core::ffi::c_long
                        || _sch_istable[(c & 0xff as ::core::ffi::c_long) as usize]
                            as ::core::ffi::c_int
                            & C2Rust_Unnamed_4::_sch_isprint.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0
                        || encoding as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                            && c > 127 as ::core::ffi::c_long
                        || include_all_whitespace as ::core::ffi::c_int != 0
                            && _sch_istable[(c & 0xff as ::core::ffi::c_long) as usize]
                                as ::core::ffi::c_int
                                & C2Rust_Unnamed_4::_sch_isspace.0 as ::core::ffi::c_int
                                    as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0))
                {
                    unget_part_char(c, &raw mut address, &raw mut magiccount, &raw mut magic);
                    break;
                } else {
                    *buf.offset(i as isize) = c as ::core::ffi::c_char;
                    i = i.wrapping_add(1);
                }
            }
        }
        print_filename_and_address(filename, start);
        *buf.offset(i as isize) = '\0' as ::core::ffi::c_char;
        fputs(buf, stdout);
        loop {
            c = get_char(
                stream,
                &raw mut address,
                &raw mut magiccount,
                &raw mut magic,
            );
            if c == EOF as ::core::ffi::c_long {
                break;
            }
            if !(c >= 0 as ::core::ffi::c_long
                && c <= 255 as ::core::ffi::c_long
                && (c == '\t' as ::core::ffi::c_long
                    || _sch_istable[(c & 0xff as ::core::ffi::c_long) as usize]
                        as ::core::ffi::c_int
                        & C2Rust_Unnamed_4::_sch_isprint.0 as ::core::ffi::c_int
                            as ::core::ffi::c_ushort
                            as ::core::ffi::c_int
                        != 0
                    || encoding as ::core::ffi::c_int == 'S' as ::core::ffi::c_int
                        && c > 127 as ::core::ffi::c_long
                    || include_all_whitespace as ::core::ffi::c_int != 0
                        && _sch_istable[(c & 0xff as ::core::ffi::c_long) as usize]
                            as ::core::ffi::c_int
                            & C2Rust_Unnamed_4::_sch_isspace.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0))
            {
                unget_part_char(c, &raw mut address, &raw mut magiccount, &raw mut magic);
                break;
            } else {
                putchar(c as ::core::ffi::c_int);
            }
        }
        if !output_separator.is_null() {
            fputs(output_separator, stdout);
        } else {
            putchar('\n' as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn usage(mut stream: *mut FILE, mut status: ::core::ffi::c_int) -> ! {
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [option(s)] [file(s)]\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        program_name,
    );
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b" Display printable strings in [file(s)] (stdin by default)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b" The options are:\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -a - --all                Scan the entire file, not just the data section [default]\n  -d --data                 Only scan the data sections in the file\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        stream,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -f --print-file-name      Print the name of the file before each string\n  -n <number>               Locate & print any sequence of at least <number>\n    --bytes=<number>         displayable characters.  (The default is 4).\n  -t --radix={o,d,x}        Print the location of the string in base 8, 10 or 16\n  -w --include-all-whitespace Include all whitespace as valid string characters\n  -o                        An alias for --radix=o\n  -T --target=<BFDNAME>     Specify the binary file format\n  -e --encoding={s,S,b,l,B,L} Select character size and endianness:\n                            s = 7-bit, S = 8-bit, {b,l} = 16-bit, {B,L} = 32-bit\n  --unicode={default|locale|invalid|hex|escape|highlight}\n  -U {d|l|i|x|e|h}          Specify how to treat UTF-8 encoded unicode characters\n  -s --output-separator=<string> String used to separate strings in output.\n  @<file>                   Read options from <file>\n  -h --help                 Display this information\n  -v -V --version           Print the program's version number\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    list_supported_targets(program_name, stream);
    if REPORT_BUGS_TO[0usize] as ::core::ffi::c_int != 0 && status == 0 as ::core::ffi::c_int {
        fprintf(
            stream,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Report bugs to %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            REPORT_BUGS_TO.as_ptr(),
        );
    }
    exit(status);
}
pub const LOCALEDIR: [::core::ffi::c_char; 47] = unsafe {
    ::core::mem::transmute::<[u8; 47], [::core::ffi::c_char; 47]>(
        *b"/root/rboxc/build/oracle/binutils/share/locale\0",
    )
};
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
