// Generated from pinned GNU Ed 1.22.6 by scripts/translate-ed.py.
// Source SHA-256: 4227d00f225724462d82050161de9b45f2bb8add2909dd0e604b6475fd312590
/* GNU ed - The GNU line editor.
   Copyright (C) 2006-2026 Antonio Diaz Diaz.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_ed_ap_init"]
    fn ap_init(
        ap: *mut Arg_parser,
        argc: ::core::ffi::c_int,
        argv: *const *const ::core::ffi::c_char,
        options: *const ap_Option,
        flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_char;
    #[link_name = "rboxc_ed_ap_free"]
    fn ap_free(ap: *mut Arg_parser);
    #[link_name = "rboxc_ed_ap_error"]
    fn ap_error(ap: *const Arg_parser) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_ed_ap_arguments"]
    fn ap_arguments(ap: *const Arg_parser) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ed_ap_code"]
    fn ap_code(ap: *const Arg_parser, i: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ed_ap_argument"]
    fn ap_argument(ap: *const Arg_parser, i: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_ed_init_buffers"]
    fn init_buffers() -> bool;
    #[link_name = "rboxc_ed_last_addr"]
    fn last_addr() -> ::core::ffi::c_int;
    #[link_name = "rboxc_ed_set_current_addr"]
    fn set_current_addr(addr: ::core::ffi::c_int);
    #[link_name = "rboxc_ed_escchar"]
    fn escchar(ch: ::core::ffi::c_uchar) -> ::core::ffi::c_uchar;
    #[link_name = "rboxc_ed_error_msg"]
    fn error_msg() -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_ed_first_e_command"]
    fn first_e_command(filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ed_main_loop"]
    fn main_loop(initial_error: bool, loose: bool) -> ::core::ffi::c_int;
    #[link_name = "rboxc_ed_set_def_filename"]
    fn set_def_filename(s: *const ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_ed_set_error_msg"]
    fn set_error_msg(msg: *const ::core::ffi::c_char);
    #[link_name = "rboxc_ed_set_prompt"]
    fn set_prompt(s: *const ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_ed_set_verbose"]
    fn set_verbose();
    #[link_name = "rboxc_ed_next_matching_node_addr"]
    fn next_matching_node_addr(ibufpp: *mut *const ::core::ffi::c_char) -> ::core::ffi::c_int;
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
pub type size_t = usize;
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ap_Has_arg(pub ::core::ffi::c_uint);
impl ap_Has_arg {
    pub const ap_no: Self = Self(0);
    pub const ap_yes: Self = Self(1);
    pub const ap_maybe: Self = Self(2);
    pub const ap_yesme: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ap_Option {
    pub code: ::core::ffi::c_int,
    pub long_name: *const ::core::ffi::c_char,
    pub has_arg: ap_Has_arg,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ap_Record {
    pub code: ::core::ffi::c_int,
    pub parsed_name: *mut ::core::ffi::c_char,
    pub argument: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Arg_parser {
    pub data: *mut ap_Record,
    pub error: *mut ::core::ffi::c_char,
    pub data_size: ::core::ffi::c_int,
    pub argv_index: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const opt_cr: Self = Self(256);
    pub const opt_un: Self = Self(257);
}
pub const INT_MAX: ::core::ffi::c_int = __INT_MAX__;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
static mut program_name: *const ::core::ffi::c_char =
    b"ed\0".as_ptr() as *const ::core::ffi::c_char;
static mut program_year: *const ::core::ffi::c_char =
    b"2026\0".as_ptr() as *const ::core::ffi::c_char;
static mut invocation_name: *const ::core::ffi::c_char =
    b"ed\0".as_ptr() as *const ::core::ffi::c_char;
static mut extended_regexp_: bool = r#false != 0;
static mut quiet: bool = r#false != 0;
static mut restricted_: bool = r#false != 0;
static mut safe_names: bool = r#true != 0;
static mut scripted_: bool = r#false != 0;
static mut strip_cr_: bool = r#false != 0;
static mut traditional_: bool = r#false != 0;
#[export_name = "rboxc_ed_extended_regexp"]
pub unsafe extern "C" fn extended_regexp() -> bool {
    return extended_regexp_;
}
#[export_name = "rboxc_ed_restricted"]
pub unsafe extern "C" fn restricted() -> bool {
    return restricted_;
}
#[export_name = "rboxc_ed_scripted"]
pub unsafe extern "C" fn scripted() -> bool {
    return scripted_;
}
#[export_name = "rboxc_ed_strip_cr"]
pub unsafe extern "C" fn strip_cr() -> bool {
    return strip_cr_;
}
#[export_name = "rboxc_ed_traditional"]
pub unsafe extern "C" fn traditional() -> bool {
    return traditional_;
}
unsafe extern "C" fn show_help() {
    fputs(
        b"GNU ed is a line-oriented text editor. It is used to create, display,\nmodify and otherwise manipulate text files, both interactively and via\nshell scripts. A restricted version of ed, red, can only edit files in\nthe current directory and cannot execute shell commands. Ed is the\n'standard' text editor in the sense that it is the original editor for\nUnix, and thus widely available. For most purposes, however, it is\nsuperseded by full-screen editors.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    printf(
        b"\nUsage: %s [options] [[+line] file]\n\0".as_ptr() as *const ::core::ffi::c_char,
        invocation_name,
    );
    fputs(
        b"\nThe file name may be preceded by '+line', '+/RE', or '+?RE' to set the\ncurrent line to the line number specified or to the first or last line\nmatching the regular expression 'RE'.\n\nStart edit by reading in 'file' if given.\nIf 'file' begins with a '!', read output of shell command.\n\nThe environment variable LINES can be used to set the initial window size.\n\nOptions:\n  -h, --help                 display this help and exit\n  -V, --version              output version information and exit\n  -E, --extended-regexp      use extended regular expressions\n  -G, --traditional          run in compatibility mode\n  -l, --loose-exit-status    exit with 0 status even if a command fails\n  -p, --prompt=STRING        use STRING as an interactive prompt\n  -q, --quiet, --silent      suppress diagnostics written to stderr\n  -r, --restricted           run in restricted mode\n  -s, --script               suppress byte counts and '!' prompt\n  -v, --verbose              be verbose; equivalent to the 'H' command\n      --strip-trailing-cr    strip carriage returns at end of text lines\n      --unsafe-names         allow control characters in file names\n\n*Exit status*\n0 for a normal exit, 1 for environmental problems (invalid command-line\noptions, memory exhausted, command failed, etc), 2 for problems with the\ninput file (file not found, buffer modified, I/O errors), 3 for an internal\nconsistency error (e.g., bug) which caused ed to panic.\n\n*See also*\nregex(7)\n\nReport bugs to bug-ed@gnu.org\nEd home page: http://www.gnu.org/software/ed/ed.html\nGeneral help using GNU software: http://www.gnu.org/gethelp\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
}
unsafe extern "C" fn show_version() {
    printf(
        b"GNU %s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
        PROGVERSION.as_ptr(),
    );
    printf(
        b"Copyright (C) 1994 Andrew L. Moore.\nCopyright (C) %s Antonio Diaz Diaz.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        program_year,
    );
    fputs(
        b"License GPLv2+: GNU GPL version 2 or later <http://gnu.org/licenses/gpl.html>\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
}
#[export_name = "rboxc_ed_print_escaped"]
pub unsafe extern "C" fn print_escaped(mut p: *const ::core::ffi::c_char, to_stdout: bool) {
    let fp: *mut FILE = if to_stdout as ::core::ffi::c_int != 0 {
        stdout
    } else {
        stderr
    };
    while *p != 0 {
        let ch: ::core::ffi::c_uchar = *p as ::core::ffi::c_uchar;
        if ch as ::core::ffi::c_int >= 32 as ::core::ffi::c_int
            && ch as ::core::ffi::c_int <= 126 as ::core::ffi::c_int
        {
            putc(ch as ::core::ffi::c_int, fp);
        } else {
            putc('\\' as ::core::ffi::c_int, fp);
            let e: ::core::ffi::c_char = escchar(ch) as ::core::ffi::c_char;
            if e != 0 {
                putc(e as ::core::ffi::c_int, fp);
            } else {
                putc(
                    (ch as ::core::ffi::c_int >> 6 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        + '0' as ::core::ffi::c_int,
                    fp,
                );
                putc(
                    (ch as ::core::ffi::c_int >> 3 as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        + '0' as ::core::ffi::c_int,
                    fp,
                );
                putc(
                    (ch as ::core::ffi::c_int & 7 as ::core::ffi::c_int)
                        + '0' as ::core::ffi::c_int,
                    fp,
                );
            }
        }
        p = p.offset(1);
    }
}
#[export_name = "rboxc_ed_show_warning"]
pub unsafe extern "C" fn show_warning(
    filename: *const ::core::ffi::c_char,
    msg: *const ::core::ffi::c_char,
) {
    if !quiet {
        if !filename.is_null() && *filename.offset(0isize) as ::core::ffi::c_int != 0 {
            print_escaped(filename, r#false != 0);
            fputs(b": \0".as_ptr() as *const ::core::ffi::c_char, stderr);
        }
        fprintf(
            stderr,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            msg,
        );
    }
}
#[export_name = "rboxc_ed_show_strerror"]
pub unsafe extern "C" fn show_strerror(
    filename: *const ::core::ffi::c_char,
    errcode: ::core::ffi::c_int,
) {
    if !quiet {
        if !filename.is_null() && *filename.offset(0isize) as ::core::ffi::c_int != 0 {
            print_escaped(filename, r#false != 0);
            fputs(b": \0".as_ptr() as *const ::core::ffi::c_char, stderr);
        }
        fprintf(
            stderr,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            strerror(errcode),
        );
    }
}
unsafe extern "C" fn show_error(
    msg: *const ::core::ffi::c_char,
    errcode: ::core::ffi::c_int,
    help: bool,
) {
    if !msg.is_null() && *msg.offset(0isize) as ::core::ffi::c_int != 0 {
        fprintf(
            stderr,
            b"%s: %s%s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
            msg,
            if errcode > 0 as ::core::ffi::c_int {
                b": \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            if errcode > 0 as ::core::ffi::c_int {
                strerror(errcode) as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if help {
        fprintf(
            stderr,
            b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
            invocation_name,
        );
    }
}
unsafe extern "C" fn parse_addr(arg: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut tail: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    *__errno_location() = 0 as ::core::ffi::c_int;
    let tmp: ::core::ffi::c_long = strtol(arg, &raw mut tail, 10 as ::core::ffi::c_int);
    if *__errno_location() == 0 as ::core::ffi::c_int
        && tail != arg as *mut ::core::ffi::c_char
        && tmp >= 1 as ::core::ffi::c_long
        && tmp <= INT_MAX as ::core::ffi::c_long
    {
        return tmp as ::core::ffi::c_int;
    }
    if !quiet {
        fprintf(
            stderr,
            b"%s: %s: Invalid line number; must be >= 1.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            program_name,
            arg,
        );
    }
    exit(1 as ::core::ffi::c_int);
}
#[export_name = "rboxc_ed_interactive"]
pub unsafe extern "C" fn interactive() -> bool {
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
    return fstat(0 as ::core::ffi::c_int, &raw mut st) == 0 as ::core::ffi::c_int
        && !(st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t);
}
#[export_name = "rboxc_ed_may_access_filename"]
pub unsafe extern "C" fn may_access_filename(name: *const ::core::ffi::c_char) -> bool {
    let len: ::core::ffi::c_int = strlen(name) as ::core::ffi::c_int;
    if len <= 0 as ::core::ffi::c_int
        || *name.offset((len - 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
            == '/' as ::core::ffi::c_int
    {
        set_error_msg(b"Invalid filename\0".as_ptr() as *const ::core::ffi::c_char);
        return r#false != 0;
    }
    if restricted_ {
        if *name.offset(0isize) as ::core::ffi::c_int == '!' as ::core::ffi::c_int {
            set_error_msg(b"Shell access restricted\0".as_ptr() as *const ::core::ffi::c_char);
            return r#false != 0;
        }
        if strcmp(name, b"..\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
            || !strchr(name, '/' as ::core::ffi::c_int).is_null()
        {
            set_error_msg(b"Directory access restricted\0".as_ptr() as *const ::core::ffi::c_char);
            return r#false != 0;
        }
    }
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    p = name;
    while *p != 0 {
        if *p as ::core::ffi::c_int == '\n' as ::core::ffi::c_int {
            set_error_msg(b"Newline character not allowed in file names\0".as_ptr()
                as *const ::core::ffi::c_char);
            return r#false != 0;
        }
        p = p.offset(1);
    }
    if safe_names {
        p = name;
        while *p != 0 {
            if *p as ::core::ffi::c_int <= 13 as ::core::ffi::c_int
                && *p as ::core::ffi::c_int >= 7 as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == 27 as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == 127 as ::core::ffi::c_int
            {
                set_error_msg(b"Control characters not allowed in file names\0".as_ptr()
                    as *const ::core::ffi::c_char);
                return r#false != 0;
            }
            p = p.offset(1);
        }
    }
    return r#true != 0;
}
unsafe extern "C" fn rboxc_ed_main_const(
    argc: ::core::ffi::c_int,
    mut argv: *const *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut initial_error: bool = r#false != 0;
    let mut loose: bool = r#false != 0;
    let options: [ap_Option; 14] = [
        ap_Option {
            code: 'E' as ::core::ffi::c_int,
            long_name: b"extended-regexp\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'G' as ::core::ffi::c_int,
            long_name: b"traditional\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'h' as ::core::ffi::c_int,
            long_name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'l' as ::core::ffi::c_int,
            long_name: b"loose-exit-status\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'p' as ::core::ffi::c_int,
            long_name: b"prompt\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_yes,
        },
        ap_Option {
            code: 'q' as ::core::ffi::c_int,
            long_name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'q' as ::core::ffi::c_int,
            long_name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'r' as ::core::ffi::c_int,
            long_name: b"restricted\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 's' as ::core::ffi::c_int,
            long_name: b"script\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'v' as ::core::ffi::c_int,
            long_name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 'V' as ::core::ffi::c_int,
            long_name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: C2Rust_Unnamed_0::opt_cr.0 as ::core::ffi::c_int,
            long_name: b"strip-trailing-cr\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: C2Rust_Unnamed_0::opt_un.0 as ::core::ffi::c_int,
            long_name: b"unsafe-names\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: ap_Has_arg::ap_no,
        },
        ap_Option {
            code: 0 as ::core::ffi::c_int,
            long_name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: ap_Has_arg::ap_no,
        },
    ];
    let mut parser: Arg_parser = Arg_parser {
        data: ::core::ptr::null_mut::<ap_Record>(),
        error: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        data_size: 0,
        argv_index: 0,
    };
    if argc > 0 as ::core::ffi::c_int {
        invocation_name = *argv.offset(0isize);
    }
    if ap_init(
        &raw mut parser,
        argc,
        argv,
        &raw const options as *const ap_Option,
        0 as ::core::ffi::c_int,
    ) == 0
    {
        show_error(
            b"Memory exhausted.\0".as_ptr() as *const ::core::ffi::c_char,
            0 as ::core::ffi::c_int,
            r#false != 0,
        );
        return 1 as ::core::ffi::c_int;
    }
    if !ap_error(&raw mut parser).is_null() {
        show_error(
            ap_error(&raw mut parser),
            0 as ::core::ffi::c_int,
            r#true != 0,
        );
        return 1 as ::core::ffi::c_int;
    }
    let mut argind: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while argind < ap_arguments(&raw mut parser) {
        let code: ::core::ffi::c_int = ap_code(&raw mut parser, argind);
        if code == 0 {
            break;
        }
        let arg: *const ::core::ffi::c_char = ap_argument(&raw mut parser, argind);
        match code {
            69 => {
                extended_regexp_ = r#true != 0;
            }
            71 => {
                traditional_ = r#true != 0;
            }
            104 => {
                show_help();
                return 0 as ::core::ffi::c_int;
            }
            108 => {
                loose = r#true != 0;
            }
            112 => {
                if !set_prompt(arg) {
                    return 1 as ::core::ffi::c_int;
                }
            }
            113 => {
                quiet = r#true != 0;
            }
            114 => {
                restricted_ = r#true != 0;
            }
            115 => {
                scripted_ = r#true != 0;
            }
            118 => {
                set_verbose();
            }
            86 => {
                show_version();
                return 0 as ::core::ffi::c_int;
            }
            256 => {
                strip_cr_ = r#true != 0;
            }
            257 => {
                safe_names = r#false != 0;
            }
            _ => {
                show_error(
                    b"internal error: uncaught option.\0".as_ptr() as *const ::core::ffi::c_char,
                    0 as ::core::ffi::c_int,
                    r#false != 0,
                );
                return 3 as ::core::ffi::c_int;
            }
        }
        argind += 1;
    }
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    if !init_buffers() {
        return 1 as ::core::ffi::c_int;
    }
    let mut start_re_arg: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut start_addr: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while argind < ap_arguments(&raw mut parser) {
        let arg_0: *const ::core::ffi::c_char = ap_argument(&raw mut parser, argind);
        if strcmp(arg_0, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int {
            scripted_ = r#true != 0;
        } else if *arg_0.offset(0isize) as ::core::ffi::c_int == '+' as ::core::ffi::c_int {
            let ch: ::core::ffi::c_uchar = *arg_0.offset(1isize) as ::core::ffi::c_uchar;
            if ch as ::core::ffi::c_int == '/' as ::core::ffi::c_int
                || ch as ::core::ffi::c_int == '?' as ::core::ffi::c_int
            {
                start_re_arg = arg_0;
            } else if *(*__ctype_b_loc()).offset(ch as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
            {
                start_addr = parse_addr(arg_0.offset(1 as ::core::ffi::c_int as isize));
            } else {
                if !quiet {
                    fprintf(
                        stderr,
                        b"%s: %s: Invalid line number or regular expression.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        program_name,
                        arg_0,
                    );
                }
                return 1 as ::core::ffi::c_int;
            }
        } else {
            if may_access_filename(arg_0) {
                if *arg_0.offset(0isize) as ::core::ffi::c_int != '!' as ::core::ffi::c_int
                    && !set_def_filename(arg_0)
                {
                    return 1 as ::core::ffi::c_int;
                }
                let ret: ::core::ffi::c_int = first_e_command(arg_0);
                if ret < 0 as ::core::ffi::c_int && !interactive() {
                    return 2 as ::core::ffi::c_int;
                }
                if ret == -2 as ::core::ffi::c_int {
                    initial_error = r#true != 0;
                }
                if ret > 0 as ::core::ffi::c_int && start_addr > 0 as ::core::ffi::c_int {
                    if start_addr <= last_addr() {
                        set_current_addr(start_addr);
                    }
                } else if ret > 0 as ::core::ffi::c_int && !start_re_arg.is_null() {
                    set_current_addr(0 as ::core::ffi::c_int);
                    let mut p: *const ::core::ffi::c_char =
                        start_re_arg.offset(1 as ::core::ffi::c_int as isize);
                    let addr: ::core::ffi::c_int = next_matching_node_addr(&raw mut p);
                    if addr > 0 as ::core::ffi::c_int && addr <= last_addr() {
                        set_current_addr(addr);
                    } else {
                        set_current_addr(
                            if *start_re_arg.offset(1isize) as ::core::ffi::c_int
                                == '/' as ::core::ffi::c_int
                            {
                                1 as ::core::ffi::c_int
                            } else {
                                last_addr()
                            },
                        );
                        if !quiet {
                            fprintf(
                                stderr,
                                b"%s: %s: No match found.\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                start_re_arg,
                                arg_0,
                            );
                        }
                        if !interactive() {
                            return 1 as ::core::ffi::c_int;
                        }
                    }
                }
            } else {
                initial_error = r#true != 0;
                if !interactive() {
                    return 2 as ::core::ffi::c_int;
                }
            }
            if initial_error {
                show_warning(arg_0, error_msg());
            }
            break;
        }
        argind += 1;
    }
    ap_free(&raw mut parser);
    return main_loop(initial_error, loose);
}
pub const PROGVERSION: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"1.22.6\0") };
pub const __INT_MAX__: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;

// The multicall ABI supplies mutable argv; GNU Ed only reads it.
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_ed(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    rboxc_ed_main_const(argc, argv as *const *const ::core::ffi::c_char)
}
