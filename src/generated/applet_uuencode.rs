// Generated from pinned GNU Sharutils 4.15.2 by scripts/translate-sharutils.py.
// Source SHA-256: 9dcaa29c3b96346aeb8ac8f8dee06ece01522ff4e8d59142cd9cddcca44d64b5
/* uuencode utility.
   Copyright (C) */
/* Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3, or (at your option)
   any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program.  If not, see <http://www.gnu.org/licenses/>.

   --copyright-mark "(copyright \\(c\\)[ \t]+[*]/ *\")([12][90][0-9][0-9])"
 */
/* Copyright (c) 1983 Regents of the University of California.
   All rights reserved.

   Redistribution and use in source and binary forms, with or without
   modification, are permitted provided that the following conditions
   are met:
   1. Redistributions of source code must retain the above copyright
      notice, this list of conditions and the following disclaimer.
   2. Redistributions in binary form must reproduce the above copyright
      notice, this list of conditions and the following disclaimer in the
      documentation and/or other materials provided with the distribution.
   3. All advertising materials mentioning features or use of this software
      must display the following acknowledgement:
	 This product includes software developed by the University of
	 California, Berkeley and its contributors.
   4. Neither the name of the University nor the names of its contributors
      may be used to endorse or promote products derived from this software
      without specific prior written permission.

   THIS SOFTWARE IS PROVIDED BY THE REGENTS AND CONTRIBUTORS ``AS IS'' AND
   ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
   IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
   ARE DISCLAIMED.  IN NO EVENT SHALL THE REGENTS OR CONTRIBUTORS BE LIABLE
   FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
   DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
   OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
   HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
   LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
   OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
   SUCH DAMAGE.  */
/* Reworked to GNU style by Ian Lance Taylor, ian@airs.com, August 93.  */
/*=======================================================\
| uuencode [INPUT] OUTPUT				 |
| 							 |
| Encode a file so it can be mailed to a remote system.	 |
\=======================================================*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn freopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn __uflow(_: *mut FILE) -> ::core::ffi::c_int;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_sharutils_uuencode_optionProcess"]
    fn optionProcess(
        _: *mut tOptions,
        _: ::core::ffi::c_int,
        _: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_sharutils_uuencode_base64_encode"]
    fn base64_encode(
        r#in: *const ::core::ffi::c_char,
        inlen: size_t,
        out: *mut ::core::ffi::c_char,
        outlen: size_t,
    );
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_sharutils_uuencode_uuencodeOptions"]
    static mut uuencodeOptions: tOptions;
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
    #[link_name = "rboxc_sharutils_uuencode_fserr"]
    fn fserr(
        exit_code: ::core::ffi::c_int,
        op: *const ::core::ffi::c_char,
        r#fn: *const ::core::ffi::c_char,
    );
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
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uintptr_t = usize;
pub type opt_state_mask_t = uint32_t;
pub type proc_state_mask_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opt_desc {
    pub optIndex: uint16_t,
    pub optValue: uint16_t,
    pub optActualIndex: uint16_t,
    pub optActualValue: uint16_t,
    pub optEquivIndex: uint16_t,
    pub optMinCt: uint16_t,
    pub optMaxCt: uint16_t,
    pub optOccCt: uint16_t,
    pub fOptState: opt_state_mask_t,
    pub optUsage: uint32_t,
    pub optArg: opt_arg_union_t,
    pub optCookie: *mut ::core::ffi::c_void,
    pub pOptMust: *const ::core::ffi::c_int,
    pub pOptCant: *const ::core::ffi::c_int,
    pub pOptProc: tpOptProc,
    pub pzText: *const ::core::ffi::c_char,
    pub pz_NAME: *const ::core::ffi::c_char,
    pub pz_Name: *const ::core::ffi::c_char,
    pub pz_DisableName: *const ::core::ffi::c_char,
    pub pz_DisablePfx: *const ::core::ffi::c_char,
}
pub type tpOptProc = Option<tOptProc>;
pub type tOptProc = unsafe extern "C" fn(*mut tOptions, *mut tOptDesc) -> ();
pub type tOptDesc = opt_desc;
pub type tOptions = options;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub structVersion: ::core::ffi::c_int,
    pub origArgCt: ::core::ffi::c_uint,
    pub origArgVect: *mut *mut ::core::ffi::c_char,
    pub fOptSet: proc_state_mask_t,
    pub curOptIdx: ::core::ffi::c_uint,
    pub pzCurOpt: *mut ::core::ffi::c_char,
    pub pzProgPath: *const ::core::ffi::c_char,
    pub pzProgName: *const ::core::ffi::c_char,
    pub pzPROGNAME: *const ::core::ffi::c_char,
    pub pzRcName: *const ::core::ffi::c_char,
    pub pzCopyright: *const ::core::ffi::c_char,
    pub pzCopyNotice: *const ::core::ffi::c_char,
    pub pzFullVersion: *const ::core::ffi::c_char,
    pub papzHomeList: *const *const ::core::ffi::c_char,
    pub pzUsageTitle: *const ::core::ffi::c_char,
    pub pzExplain: *const ::core::ffi::c_char,
    pub pzDetail: *const ::core::ffi::c_char,
    pub pOptDesc: *mut tOptDesc,
    pub pzBugAddr: *const ::core::ffi::c_char,
    pub pExtensions: *mut ::core::ffi::c_void,
    pub pSavedState: *mut ::core::ffi::c_void,
    pub pUsageProc: tpUsageProc,
    pub pTransProc: Option<tOptionXlateProc>,
    pub specOptIdx: option_spec_idx_t,
    pub optCt: ::core::ffi::c_int,
    pub presetOptCt: ::core::ffi::c_int,
    pub pzFullUsage: *const ::core::ffi::c_char,
    pub pzShortUsage: *const ::core::ffi::c_char,
    pub originalOptArgArray: *const opt_arg_union_t,
    pub originalOptArgCookie: *const *mut ::core::ffi::c_void,
    pub pzPkgDataDir: *const ::core::ffi::c_char,
    pub pzPackager: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union opt_arg_union_t {
    pub argString: *const ::core::ffi::c_char,
    pub argEnum: uintptr_t,
    pub argIntptr: uintptr_t,
    pub argInt: ::core::ffi::c_long,
    pub argUint: ::core::ffi::c_ulong,
    pub argBool: ::core::ffi::c_uint,
    pub argFp: *mut FILE,
    pub argFd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option_spec_idx_t {
    pub more_help: uint16_t,
    pub save_opts: uint16_t,
    pub number_option: uint16_t,
    pub default_opt: uint16_t,
}
pub type tOptionXlateProc = unsafe extern "C" fn() -> ();
pub type tpUsageProc = Option<tUsageProc>;
pub type tUsageProc = unsafe extern "C" fn(*mut tOptions, ::core::ffi::c_int) -> ();
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const INDEX_OPT_BASE64: Self = Self(0);
    pub const INDEX_OPT_ENCODE_FILE_NAME: Self = Self(1);
    pub const INDEX_OPT_VERSION: Self = Self(2);
    pub const INDEX_OPT_HELP: Self = Self(3);
    pub const INDEX_OPT_MORE_HELP: Self = Self(4);
    pub const INDEX_OPT_SAVE_OPTS: Self = Self(5);
    pub const INDEX_OPT_LOAD_OPTS: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const UUENCODE_EXIT_SUCCESS: Self = Self(0);
    pub const UUENCODE_EXIT_FAILURE: Self = Self(1);
    pub const UUENCODE_EXIT_USAGE_ERROR: Self = Self(64);
    pub const UUENCODE_EXIT_NO_CONFIG_INPUT: Self = Self(66);
    pub const UUENCODE_EXIT_LIBOPTS_FAILURE: Self = Self(70);
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
pub const __S_IREAD: ::core::ffi::c_int = 0o400 as ::core::ffi::c_int;
pub const __S_IWRITE: ::core::ffi::c_int = 0o200 as ::core::ffi::c_int;
pub const __S_IEXEC: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const _IO_EOF_SEEN: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
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
unsafe extern "C" fn feof_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_EOF_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const OPTST_SET_MASK: ::core::ffi::c_uint = 0xf as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const S_IRUSR: ::core::ffi::c_int = __S_IREAD;
pub const S_IWUSR: ::core::ffi::c_int = __S_IWRITE;
pub const S_IRWXU: ::core::ffi::c_int = __S_IREAD | __S_IWRITE | __S_IEXEC;
pub const S_IRGRP: ::core::ffi::c_int = S_IRUSR >> 3 as ::core::ffi::c_int;
pub const S_IWGRP: ::core::ffi::c_int = S_IWUSR >> 3 as ::core::ffi::c_int;
pub const S_IRWXG: ::core::ffi::c_int = S_IRWXU >> 3 as ::core::ffi::c_int;
pub const S_IROTH: ::core::ffi::c_int = S_IRGRP >> 3 as ::core::ffi::c_int;
pub const S_IWOTH: ::core::ffi::c_int = S_IWGRP >> 3 as ::core::ffi::c_int;
pub const S_IRWXO: ::core::ffi::c_int = S_IRWXG >> 3 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
extern "C" {
    #[link_name = "rboxc_sharutils_uuencode_program_name"]
    static program_name: *const ::core::ffi::c_char;
}
#[inline]
unsafe extern "C" fn aoGetsText(mut pz: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    if pz.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    return dcgettext(::core::ptr::null::<::core::ffi::c_char>(), pz, LC_MESSAGES);
}
pub const IRWALL_MODE: ::core::ffi::c_int =
    S_IRUSR | S_IWUSR | S_IRGRP | S_IWGRP | S_IROTH | S_IWOTH;
#[export_name = "rboxc_sharutils_uuencode_output_name"]
pub static mut output_name: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_sharutils_uuencode_input_name"]
pub static mut input_name: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_sharutils_uuencode_uu_std"]
pub static mut uu_std: [::core::ffi::c_char; 64] = [
    '`' as ::core::ffi::c_char,
    '!' as ::core::ffi::c_char,
    '"' as ::core::ffi::c_char,
    '#' as ::core::ffi::c_char,
    '$' as ::core::ffi::c_char,
    '%' as ::core::ffi::c_char,
    '&' as ::core::ffi::c_char,
    '\'' as ::core::ffi::c_char,
    '(' as ::core::ffi::c_char,
    ')' as ::core::ffi::c_char,
    '*' as ::core::ffi::c_char,
    '+' as ::core::ffi::c_char,
    ',' as ::core::ffi::c_char,
    '-' as ::core::ffi::c_char,
    '.' as ::core::ffi::c_char,
    '/' as ::core::ffi::c_char,
    '0' as ::core::ffi::c_char,
    '1' as ::core::ffi::c_char,
    '2' as ::core::ffi::c_char,
    '3' as ::core::ffi::c_char,
    '4' as ::core::ffi::c_char,
    '5' as ::core::ffi::c_char,
    '6' as ::core::ffi::c_char,
    '7' as ::core::ffi::c_char,
    '8' as ::core::ffi::c_char,
    '9' as ::core::ffi::c_char,
    ':' as ::core::ffi::c_char,
    ';' as ::core::ffi::c_char,
    '<' as ::core::ffi::c_char,
    '=' as ::core::ffi::c_char,
    '>' as ::core::ffi::c_char,
    '?' as ::core::ffi::c_char,
    '@' as ::core::ffi::c_char,
    'A' as ::core::ffi::c_char,
    'B' as ::core::ffi::c_char,
    'C' as ::core::ffi::c_char,
    'D' as ::core::ffi::c_char,
    'E' as ::core::ffi::c_char,
    'F' as ::core::ffi::c_char,
    'G' as ::core::ffi::c_char,
    'H' as ::core::ffi::c_char,
    'I' as ::core::ffi::c_char,
    'J' as ::core::ffi::c_char,
    'K' as ::core::ffi::c_char,
    'L' as ::core::ffi::c_char,
    'M' as ::core::ffi::c_char,
    'N' as ::core::ffi::c_char,
    'O' as ::core::ffi::c_char,
    'P' as ::core::ffi::c_char,
    'Q' as ::core::ffi::c_char,
    'R' as ::core::ffi::c_char,
    'S' as ::core::ffi::c_char,
    'T' as ::core::ffi::c_char,
    'U' as ::core::ffi::c_char,
    'V' as ::core::ffi::c_char,
    'W' as ::core::ffi::c_char,
    'X' as ::core::ffi::c_char,
    'Y' as ::core::ffi::c_char,
    'Z' as ::core::ffi::c_char,
    '[' as ::core::ffi::c_char,
    '\\' as ::core::ffi::c_char,
    ']' as ::core::ffi::c_char,
    '^' as ::core::ffi::c_char,
    '_' as ::core::ffi::c_char,
];
#[export_name = "rboxc_sharutils_uuencode_trans_ptr"]
pub static mut trans_ptr: *const ::core::ffi::c_char =
    unsafe { &raw const uu_std as *const ::core::ffi::c_char };
#[inline]
unsafe extern "C" fn try_putchar(mut c: ::core::ffi::c_int) {
    if putchar_unlocked(c) == EOF {
        fserr(
            C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
            b"putchar\0".as_ptr() as *const ::core::ffi::c_char,
            aoGetsText(b"standard output\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
}
unsafe extern "C" fn encode_block(
    mut out: *mut ::core::ffi::c_char,
    mut r#in: *const ::core::ffi::c_uchar,
    mut in_len: size_t,
) -> size_t {
    let mut start: *mut ::core::ffi::c_char = out;
    while in_len >= 3 as size_t {
        let c2rust_fresh6 = out;
        out = out.offset(1);
        *c2rust_fresh6 = *trans_ptr.offset(
            (*r#in.offset(0isize) as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        let c2rust_fresh7 = out;
        out = out.offset(1);
        *c2rust_fresh7 = *trans_ptr.offset(
            (((*r#in.offset(0isize) as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int)
                + (*r#in.offset(1isize) as ::core::ffi::c_int >> 4 as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int)
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        let c2rust_fresh8 = out;
        out = out.offset(1);
        *c2rust_fresh8 = *trans_ptr.offset(
            (((*r#in.offset(1isize) as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int)
                + (*r#in.offset(2isize) as ::core::ffi::c_int >> 6 as ::core::ffi::c_int
                    & 0x3 as ::core::ffi::c_int)
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        let c2rust_fresh9 = out;
        out = out.offset(1);
        *c2rust_fresh9 = *trans_ptr.offset(
            (*r#in.offset(2isize) as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        in_len = in_len.wrapping_sub(3 as size_t);
        r#in = r#in.offset(3 as ::core::ffi::c_int as isize);
    }
    if in_len > 0 as size_t {
        let mut lc: ::core::ffi::c_uchar = (if in_len > 1 as size_t {
            *r#in.offset(1isize) as ::core::ffi::c_int
        } else {
            '\0' as ::core::ffi::c_int
        }) as ::core::ffi::c_uchar;
        let c2rust_fresh10 = out;
        out = out.offset(1);
        *c2rust_fresh10 = *trans_ptr.offset(
            (*r#in.offset(0isize) as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        let c2rust_fresh11 = out;
        out = out.offset(1);
        *c2rust_fresh11 = *trans_ptr.offset(
            (((*r#in.offset(0isize) as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int)
                + (lc as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        let c2rust_fresh12 = out;
        out = out.offset(1);
        *c2rust_fresh12 = *trans_ptr.offset(
            ((lc as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                & 0o77 as ::core::ffi::c_int) as isize,
        );
        let c2rust_fresh13 = out;
        out = out.offset(1);
        *c2rust_fresh13 =
            *trans_ptr.offset((0 as ::core::ffi::c_int & 0o77 as ::core::ffi::c_int) as isize);
    }
    return out.offset_from(start) as size_t;
}
unsafe extern "C" fn encode() {
    let mut finishing: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut buf: [::core::ffi::c_uchar; 45] = [0; 45];
        let mut buf_out: [::core::ffi::c_char; 64] = [0; 64];
        let mut rdct: ::core::ffi::c_int = (if 0 != 0
            && 0 != 0
            && (1 as ::core::ffi::c_int as size_t)
                .wrapping_mul(::core::mem::size_of::<[::core::ffi::c_uchar; 45]>())
                <= 8 as size_t
            && 1 as ::core::ffi::c_int as size_t != 0 as size_t
        {
            ({
                let mut __ptr: *mut ::core::ffi::c_char =
                    &raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char;
                let mut __stream: *mut FILE = stdin;
                let mut __cnt: size_t = 0;
                __cnt = (1 as ::core::ffi::c_int as size_t)
                    .wrapping_mul(::core::mem::size_of::<[::core::ffi::c_uchar; 45]>());
                while __cnt > 0 as size_t {
                    let mut __c: ::core::ffi::c_int = getc_unlocked(__stream);
                    if __c == EOF {
                        break;
                    }
                    let c2rust_fresh3 = __ptr;
                    __ptr = __ptr.offset(1);
                    *c2rust_fresh3 = __c as ::core::ffi::c_char;
                    __cnt = __cnt.wrapping_sub(1);
                }
                (1 as ::core::ffi::c_int as size_t)
                    .wrapping_mul(::core::mem::size_of::<[::core::ffi::c_uchar; 45]>())
                    .wrapping_sub(__cnt)
                    .wrapping_div(1 as ::core::ffi::c_int as size_t)
            })
        } else if 0 != 0 && 1 as ::core::ffi::c_int as size_t == 0 as size_t
            || 0 != 0 && ::core::mem::size_of::<[::core::ffi::c_uchar; 45]>() == 0 as size_t
        {
            0 as ::core::ffi::c_int as size_t
        } else {
            fread_unlocked(
                &raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void,
                1 as size_t,
                ::core::mem::size_of::<[::core::ffi::c_uchar; 45]>(),
                stdin,
            )
        }) as ::core::ffi::c_int;
        let mut wrct: size_t = ::core::mem::size_of::<[::core::ffi::c_char; 64]>();
        if rdct <= 0 as ::core::ffi::c_int {
            if ferror_unlocked(stdin) != 0 {
                fserr(
                    C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
                    b"fread\0".as_ptr() as *const ::core::ffi::c_char,
                    input_name,
                );
            }
            break;
        } else {
            if rdct < 45 as ::core::ffi::c_int {
                if feof_unlocked(stdin) == 0 {
                    fserr(
                        C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
                        b"fread\0".as_ptr() as *const ::core::ffi::c_char,
                        input_name,
                    );
                }
                finishing = 1 as ::core::ffi::c_int;
            }
            if (*uuencodeOptions
                .pOptDesc
                .offset(C2Rust_Unnamed::INDEX_OPT_BASE64.0 as ::core::ffi::c_int as isize))
            .fOptState
                & OPTST_SET_MASK as opt_state_mask_t
                == 0 as opt_state_mask_t
            {
                try_putchar(
                    *trans_ptr.offset(
                        (rdct as ::core::ffi::c_uint & 0o77 as ::core::ffi::c_uint) as isize,
                    ) as ::core::ffi::c_int,
                );
                wrct = encode_block(
                    &raw mut buf_out as *mut ::core::ffi::c_char,
                    &raw mut buf as *mut ::core::ffi::c_uchar,
                    rdct as size_t,
                );
            } else {
                base64_encode(
                    &raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
                    rdct as size_t,
                    &raw mut buf_out as *mut ::core::ffi::c_char,
                    wrct,
                );
                wrct = strlen(&raw mut buf_out as *mut ::core::ffi::c_char);
            }
            let c2rust_fresh4 = wrct;
            wrct = wrct.wrapping_add(1);
            buf_out[c2rust_fresh4] = '\n' as ::core::ffi::c_char;
            if if 0 != 0
                && 0 != 0
                && (1 as ::core::ffi::c_int as size_t).wrapping_mul(wrct) <= 8 as size_t
                && 1 as ::core::ffi::c_int as size_t != 0 as size_t
            {
                ({
                    let mut __ptr: *const ::core::ffi::c_char =
                        &raw mut buf_out as *mut ::core::ffi::c_char as *const ::core::ffi::c_char;
                    let mut __stream: *mut FILE = stdout;
                    let mut __cnt: size_t = 0;
                    __cnt = (1 as ::core::ffi::c_int as size_t).wrapping_mul(wrct);
                    while __cnt > 0 as size_t {
                        let c2rust_fresh5 = __ptr;
                        __ptr = __ptr.offset(1);
                        if putc_unlocked(*c2rust_fresh5 as ::core::ffi::c_int, __stream) == EOF {
                            break;
                        }
                        __cnt = __cnt.wrapping_sub(1);
                    }
                    (1 as ::core::ffi::c_int as size_t)
                        .wrapping_mul(wrct)
                        .wrapping_sub(__cnt)
                        .wrapping_div(1 as ::core::ffi::c_int as size_t)
                })
            } else if 0 != 0 && 1 as ::core::ffi::c_int as size_t == 0 as size_t
                || 0 != 0 && wrct == 0 as size_t
            {
                0 as ::core::ffi::c_int as size_t
            } else {
                fwrite_unlocked(
                    &raw mut buf_out as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                    1 as size_t,
                    wrct,
                    stdout,
                )
            } != wrct
            {
                fserr(
                    C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
                    b"fwrite\0".as_ptr() as *const ::core::ffi::c_char,
                    aoGetsText(b"standard output\0".as_ptr() as *const ::core::ffi::c_char),
                );
            }
            if finishing != 0 {
                break;
            }
        }
    }
    if fclose(stdin) != 0 as ::core::ffi::c_int {
        fserr(
            C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
            b"fclose\0".as_ptr() as *const ::core::ffi::c_char,
            input_name,
        );
    }
    if (*uuencodeOptions
        .pOptDesc
        .offset(C2Rust_Unnamed::INDEX_OPT_BASE64.0 as ::core::ffi::c_int as isize))
    .fOptState
        & OPTST_SET_MASK as opt_state_mask_t
        == 0 as opt_state_mask_t
    {
        try_putchar(
            *trans_ptr.offset(('\0' as ::core::ffi::c_int & 0o77 as ::core::ffi::c_int) as isize)
                as ::core::ffi::c_int,
        );
        try_putchar('\n' as ::core::ffi::c_int);
    }
}
unsafe extern "C" fn process_opts(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut mode: *mut ::core::ffi::c_int,
) {
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    input_name = aoGetsText(b"standard input\0".as_ptr() as *const ::core::ffi::c_char);
    let mut ct: ::core::ffi::c_int = optionProcess(&raw mut uuencodeOptions, argc, argv);
    argc -= ct;
    argv = argv.offset(ct as isize);
    match argc {
        2 => {
            let mut sb: stat = stat {
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
            let mut fp: *mut FILE =
                freopen(*argv, b"r\0".as_ptr() as *const ::core::ffi::c_char, stdin);
            input_name = *argv;
            if fp != stdin {
                fserr(
                    C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
                    aoGetsText(b"freopen of stdin\0".as_ptr() as *const ::core::ffi::c_char),
                    input_name,
                );
            }
            if fstat(fileno(stdin), &raw mut sb) != 0 as ::core::ffi::c_int {
                fserr(
                    C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
                    b"fstat\0".as_ptr() as *const ::core::ffi::c_char,
                    input_name,
                );
            }
            *mode = (sb.st_mode & (S_IRWXU | S_IRWXG | S_IRWXO) as __mode_t) as ::core::ffi::c_int;
            output_name = *argv.offset(1isize);
        }
        1 => {
            *mode =
                (IRWALL_MODE as __mode_t & !umask(IRWALL_MODE as __mode_t)) as ::core::ffi::c_int;
            output_name = *argv;
        }
        0 | _ => {
            Some(
                uuencodeOptions
                    .pUsageProc
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                &raw mut uuencodeOptions,
                C2Rust_Unnamed_0::UUENCODE_EXIT_USAGE_ERROR.0 as ::core::ffi::c_int,
            );
        }
    }
    if !((*uuencodeOptions
        .pOptDesc
        .offset(C2Rust_Unnamed::INDEX_OPT_ENCODE_FILE_NAME.0 as ::core::ffi::c_int as isize))
    .fOptState
        & OPTST_SET_MASK as opt_state_mask_t
        == 0 as opt_state_mask_t)
    {
        let mut nmlen: size_t = strlen(output_name);
        let mut bfsz: size_t = nmlen
            .wrapping_add(nmlen.wrapping_div(3 as size_t))
            .wrapping_add(4 as size_t);
        let mut p: *mut ::core::ffi::c_char = malloc(bfsz) as *mut ::core::ffi::c_char;
        if p.is_null() {
            fserr(
                C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
                b"malloc\0".as_ptr() as *const ::core::ffi::c_char,
                aoGetsText(b"file name\0".as_ptr() as *const ::core::ffi::c_char),
            );
        }
        base64_encode(output_name, nmlen, p, bfsz);
        output_name = p;
    }
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_uuencode(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut mode: ::core::ffi::c_int = 0;
    process_opts(argc, argv, &raw mut mode);
    if printf(
        b"begin%s%s %o %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if !((*uuencodeOptions
            .pOptDesc
            .offset(C2Rust_Unnamed::INDEX_OPT_BASE64.0 as ::core::ffi::c_int as isize))
        .fOptState
            & OPTST_SET_MASK as opt_state_mask_t
            == 0 as opt_state_mask_t)
        {
            b"-base64\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        if !((*uuencodeOptions
            .pOptDesc
            .offset(C2Rust_Unnamed::INDEX_OPT_ENCODE_FILE_NAME.0 as ::core::ffi::c_int as isize))
        .fOptState
            & OPTST_SET_MASK as opt_state_mask_t
            == 0 as opt_state_mask_t)
        {
            b"-encoded\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
        mode,
        output_name,
    ) < 0 as ::core::ffi::c_int
    {
        fserr(
            C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
            b"printf\0".as_ptr() as *const ::core::ffi::c_char,
            aoGetsText(b"standard output\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
    encode();
    if ferror_unlocked(stdout) != 0 {
        fserr(
            C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
            b"ferror\0".as_ptr() as *const ::core::ffi::c_char,
            aoGetsText(b"standard output\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
    if puts(
        if !((*uuencodeOptions
            .pOptDesc
            .offset(C2Rust_Unnamed::INDEX_OPT_BASE64.0 as ::core::ffi::c_int as isize))
        .fOptState
            & OPTST_SET_MASK as opt_state_mask_t
            == 0 as opt_state_mask_t)
        {
            b"====\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"end\0".as_ptr() as *const ::core::ffi::c_char
        },
    ) == EOF
    {
        fserr(
            C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
            b"puts\0".as_ptr() as *const ::core::ffi::c_char,
            aoGetsText(b"standard output\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
    if fclose(stdout) != 0 as ::core::ffi::c_int {
        fserr(
            C2Rust_Unnamed_0::UUENCODE_EXIT_FAILURE.0 as ::core::ffi::c_int,
            b"fclose\0".as_ptr() as *const ::core::ffi::c_char,
            aoGetsText(b"standard output\0".as_ptr() as *const ::core::ffi::c_char),
        );
    }
    exit(C2Rust_Unnamed_0::UUENCODE_EXIT_SUCCESS.0 as ::core::ffi::c_int);
}
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/sharutils/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"sharutils\0") };
