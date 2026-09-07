// Generated from pinned GNU Which 2.25 by scripts/translate-which.py.
// Source SHA-256: 59ce0c3f631b838a31f9466d28f525736a224a5f011b7d1bfe514e2920f7ee14
/*
 * which v2.x -- print full path of executables
 * Copyright (C) 1999, 2003, 2007, 2008, 2024, 2026  Carlo Wood <carlo@gnu.org>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcat(
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
    fn strrchr(
        __s: *const ::core::ffi::c_char,
        __c: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn getcwd(__buf: *mut ::core::ffi::c_char, __size: size_t) -> *mut ::core::ffi::c_char;
    fn geteuid() -> __uid_t;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut optind: ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn realloc(__ptr: *mut ::core::ffi::c_void, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn getopt_long(
        argc: ::core::ffi::c_int,
        argv: *const *mut ::core::ffi::c_char,
        shortopts: *const ::core::ffi::c_char,
        longopts: *const option,
        longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_which_file_status"]
    fn file_status(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_which_contains_separator"]
    fn contains_separator(string: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_which_get_next_path_element"]
    fn get_next_path_element(
        path_list: *const ::core::ffi::c_char,
        path_index_pointer: *mut ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_which_make_full_pathname"]
    fn make_full_pathname(
        path: *const ::core::ffi::c_char,
        name: *const ::core::ffi::c_char,
        name_len: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_which_uidget"]
    fn uidget() -> ::core::ffi::c_int;
    #[link_name = "rboxc_which_sh_get_home_dir"]
    fn sh_get_home_dir() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_which_tilde_expand"]
    fn tilde_expand(string: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
}
pub type size_t = usize;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type uid_t = __uid_t;
pub type FILE = _IO_FILE;
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
pub struct function_st {
    pub name: *mut ::core::ffi::c_char,
    pub len: size_t,
    pub lines: *mut *mut ::core::ffi::c_char,
    pub line_count: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct opts(pub ::core::ffi::c_uint);
impl opts {
    pub const opt_version: Self = Self(0);
    pub const opt_skip_dot: Self = Self(1);
    pub const opt_skip_tilde: Self = Self(2);
    pub const opt_skip_alias: Self = Self(3);
    pub const opt_read_functions: Self = Self(4);
    pub const opt_skip_functions: Self = Self(5);
    pub const opt_show_dot: Self = Self(6);
    pub const opt_show_tilde: Self = Self(7);
    pub const opt_tty_only: Self = Self(8);
    pub const opt_help: Self = Self(9);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const DIR_SEPARATOR: ::core::ffi::c_int = '/' as ::core::ffi::c_int;
pub const FS_EXISTS: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const FS_EXECABLE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
unsafe extern "C" fn print_usage(mut out: *mut FILE) {
    fprintf(
        out,
        b"Usage: %s [options] [--] COMMAND [...]\n\0".as_ptr() as *const ::core::ffi::c_char,
        progname,
    );
    fprintf(
        out,
        b"Write the full path of COMMAND(s) to standard output.\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --version, -[vV] Print version and exit successfully.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --help,          Print this help and exit successfully.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --skip-dot       Skip directories in PATH that start with a dot.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --skip-tilde     Skip directories in PATH that start with a tilde.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --show-dot       Don't expand a dot to current directory in output.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --show-tilde     Output a tilde for HOME directory for non-root.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --tty-only       Stop processing options on the right if not on tty.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --all, -a        Print all matches in PATH, not just the first\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --read-alias, -i Read list of aliases from stdin.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --skip-alias     Ignore option --read-alias; don't read stdin.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --read-functions Read shell functions from stdin.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"  --skip-functions Ignore option --read-functions; don't read stdin.\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"Recommended use is to write the output of (alias; declare -f) to standard\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"input, so that which can show aliases and shell functions. See which(1) for\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"examples.\n\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"If the options --read-alias and/or --read-functions are specified then the\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"output can be a full alias or function definition, optionally followed by\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"the full path of each command used inside of those.\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        out,
        b"Report bugs to <which-bugs@gnu.org>.\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn print_version() {
    fprintf(
        stdout,
        b"GNU which v2.25, Copyright (C) 1999 - 2026 Carlo Wood.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"GNU which comes with ABSOLUTELY NO WARRANTY;\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"This program is free software; your freedom to use, change\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        stdout,
        b"and distribute this program is protected by the GPL.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
}
unsafe extern "C" fn print_fail(
    mut name: *const ::core::ffi::c_char,
    mut path_list: *const ::core::ffi::c_char,
) {
    fprintf(
        stderr,
        b"%s: no %s in (%s)\n\0".as_ptr() as *const ::core::ffi::c_char,
        progname,
        name,
        path_list,
    );
}
static mut home: [::core::ffi::c_char; 4096] = [0; 4096];
static mut homelen: size_t = 0 as size_t;
static mut path_containing_separator_given: ::core::ffi::c_int = 0;
static mut found_path_starts_with_dot: ::core::ffi::c_int = 0;
static mut abs_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut skip_dot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut read_alias: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut skip_alias: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut skip_tilde: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut show_dot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut show_tilde: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut show_all: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut tty_only: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut skip_functions: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut read_functions: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn find_command_in_path(
    mut name: *const ::core::ffi::c_char,
    mut path_list: *const ::core::ffi::c_char,
    mut path_index: *mut ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    let mut found: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut full_path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut status: ::core::ffi::c_int = 0;
    let mut name_len: ::core::ffi::c_int = 0;
    name_len = strlen(name) as ::core::ffi::c_int;
    if contains_separator(name) == 0 {
        path_containing_separator_given = 0 as ::core::ffi::c_int;
    } else {
        let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        path_containing_separator_given = 1 as ::core::ffi::c_int;
        if !abs_path.is_null() {
            free(abs_path as *mut ::core::ffi::c_void);
        }
        if *name as ::core::ffi::c_int != '.' as ::core::ffi::c_int
            && !(*name.offset(0isize) as ::core::ffi::c_int == DIR_SEPARATOR)
            && *name as ::core::ffi::c_int != '~' as ::core::ffi::c_int
        {
            abs_path =
                xmalloc((3 as ::core::ffi::c_int + name_len) as size_t) as *mut ::core::ffi::c_char;
            strcpy(abs_path, b"./\0".as_ptr() as *const ::core::ffi::c_char);
            *abs_path.offset(1isize) = DIR_SEPARATOR as ::core::ffi::c_char;
            strcat(abs_path, name);
        } else {
            abs_path =
                xmalloc((1 as ::core::ffi::c_int + name_len) as size_t) as *mut ::core::ffi::c_char;
            strcpy(abs_path, name);
        }
        path_list = abs_path;
        p = strrchr(abs_path, DIR_SEPARATOR);
        let c2rust_fresh0 = p;
        p = p.offset(1);
        *c2rust_fresh0 = 0 as ::core::ffi::c_char;
        name = p;
    }
    while !path_list.is_null() && *path_list.offset(*path_index as isize) as ::core::ffi::c_int != 0
    {
        let mut path: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if path_containing_separator_given != 0 {
            path = strcpy(
                xmalloc((1 as size_t).wrapping_add(strlen(path_list))) as *mut ::core::ffi::c_char,
                path_list,
            );
            *path_index = strlen(path) as ::core::ffi::c_int;
        } else {
            path = get_next_path_element(path_list, path_index);
        }
        if path.is_null() {
            break;
        }
        if *path as ::core::ffi::c_int == '~' as ::core::ffi::c_int {
            let mut t: *mut ::core::ffi::c_char = tilde_expand(path);
            free(path as *mut ::core::ffi::c_void);
            path = t;
            if skip_tilde != 0 {
                free(path as *mut ::core::ffi::c_void);
                continue;
            }
        }
        if skip_dot != 0 && !(*path.offset(0isize) as ::core::ffi::c_int == DIR_SEPARATOR) {
            free(path as *mut ::core::ffi::c_void);
        } else {
            found_path_starts_with_dot =
                (*path as ::core::ffi::c_int == '.' as ::core::ffi::c_int) as ::core::ffi::c_int;
            full_path = make_full_pathname(path, name, name_len);
            status = file_status(full_path);
            if status & FS_EXISTS != 0 && status & FS_EXECABLE != 0 {
                found = full_path;
                free(path as *mut ::core::ffi::c_void);
                break;
            } else {
                free(path as *mut ::core::ffi::c_void);
                if !found.is_null() {
                    break;
                }
                free(full_path as *mut ::core::ffi::c_void);
            }
        }
    }
    return found;
}
static mut cwd: [::core::ffi::c_char; 4097] = [0; 4097];
static mut cwdlen: size_t = 0;
unsafe extern "C" fn get_current_working_directory() {
    if cwdlen != 0 {
        return;
    }
    if getcwd(
        &raw mut cwd as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 4097]>().wrapping_sub(1 as size_t),
    )
    .is_null()
    {
        let mut pwd: *const ::core::ffi::c_char =
            getenv(b"PWD\0".as_ptr() as *const ::core::ffi::c_char);
        if !pwd.is_null()
            && strlen(pwd)
                < ::core::mem::size_of::<[::core::ffi::c_char; 4097]>().wrapping_sub(1usize)
        {
            strcpy(&raw mut cwd as *mut ::core::ffi::c_char, pwd);
        } else {
            cwd[0usize] = '\0' as ::core::ffi::c_char;
        }
    }
    if !(cwd[0usize] as ::core::ffi::c_int == DIR_SEPARATOR) {
        fprintf(
            stderr,
            b"Can't get current working directory\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(-1 as ::core::ffi::c_int);
    }
    cwdlen = strlen(&raw mut cwd as *mut ::core::ffi::c_char);
    if !(cwd[cwdlen.wrapping_sub(1 as size_t)] as ::core::ffi::c_int == DIR_SEPARATOR) {
        let c2rust_fresh1 = cwdlen;
        cwdlen = cwdlen.wrapping_add(1);
        cwd[c2rust_fresh1] = DIR_SEPARATOR as ::core::ffi::c_char;
        cwd[cwdlen] = 0 as ::core::ffi::c_char;
    }
}
unsafe extern "C" fn path_clean_up(
    mut path: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    static mut result: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut p1: *const ::core::ffi::c_char = path;
    let mut p2: *mut ::core::ffi::c_char = &raw mut result as *mut ::core::ffi::c_char;
    let mut saw_slash: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut saw_slash_dot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut saw_slash_dot_dot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*p1.offset(0isize) as ::core::ffi::c_int == DIR_SEPARATOR) {
        get_current_working_directory();
        strcpy(
            &raw mut result as *mut ::core::ffi::c_char,
            &raw mut cwd as *mut ::core::ffi::c_char,
        );
        saw_slash = 1 as ::core::ffi::c_int;
        p2 = (&raw mut result as *mut ::core::ffi::c_char).offset(cwdlen as isize);
    }
    loop {
        if saw_slash == 0
            || !(*p1 as ::core::ffi::c_int == DIR_SEPARATOR)
            || p1 == path.offset(1 as ::core::ffi::c_int as isize)
                && !(*p1.offset(1isize) as ::core::ffi::c_int == DIR_SEPARATOR)
        {
            if p2
                >= (&raw mut result as *mut ::core::ffi::c_char)
                    .offset(::core::mem::size_of::<[::core::ffi::c_char; 4097]>() as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize))
            {
                fprintf(
                    stderr,
                    b"Can't create full path\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
                exit(-1 as ::core::ffi::c_int);
            }
            let c2rust_fresh2 = p2;
            p2 = p2.offset(1);
            *c2rust_fresh2 = *p1;
        }
        if saw_slash_dot != 0 && *p1 as ::core::ffi::c_int == DIR_SEPARATOR {
            p2 = p2.offset(-(2 as ::core::ffi::c_int as isize));
        }
        if saw_slash_dot_dot != 0 && *p1 as ::core::ffi::c_int == DIR_SEPARATOR {
            let mut cnt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            loop {
                p2 = p2.offset(-1);
                if p2 < &raw mut result as *mut ::core::ffi::c_char {
                    strcpy(&raw mut result as *mut ::core::ffi::c_char, path);
                    return &raw mut result as *mut ::core::ffi::c_char;
                }
                if *p2 as ::core::ffi::c_int == DIR_SEPARATOR {
                    cnt += 1;
                }
                if cnt == 3 as ::core::ffi::c_int {
                    break;
                }
            }
            p2 = p2.offset(1);
        }
        saw_slash_dot_dot = (saw_slash_dot != 0
            && *p1 as ::core::ffi::c_int == '.' as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        saw_slash_dot = (saw_slash != 0 && *p1 as ::core::ffi::c_int == '.' as ::core::ffi::c_int)
            as ::core::ffi::c_int;
        saw_slash = (*p1 as ::core::ffi::c_int == DIR_SEPARATOR) as ::core::ffi::c_int;
        let c2rust_fresh3 = p1;
        p1 = p1.offset(1);
        if *c2rust_fresh3 == 0 {
            break;
        }
    }
    return &raw mut result as *mut ::core::ffi::c_char;
}
static mut functions: *mut function_st = ::core::ptr::null_mut::<function_st>();
static mut func_count: ::core::ffi::c_int = 0;
static mut max_func_count: ::core::ffi::c_int = 0;
static mut aliases: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
static mut alias_count: ::core::ffi::c_int = 0;
static mut max_alias_count: ::core::ffi::c_int = 0;
#[export_name = "rboxc_which_func_search"]
pub unsafe extern "C" fn func_search(
    mut indent: ::core::ffi::c_int,
    mut cmd: *const ::core::ffi::c_char,
    mut func_list: *mut function_st,
    mut function_start_type: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < func_count {
        if strcmp((*functions.offset(i as isize)).name, cmd) == 0 {
            let mut j: ::core::ffi::c_int = 0;
            if indent != 0 {
                fputc('\t' as ::core::ffi::c_int, stdout);
            }
            if function_start_type == 1 as ::core::ffi::c_int {
                fprintf(
                    stdout,
                    b"%s () {\n\0".as_ptr() as *const ::core::ffi::c_char,
                    cmd,
                );
            } else {
                fprintf(
                    stdout,
                    b"%s ()\n\0".as_ptr() as *const ::core::ffi::c_char,
                    cmd,
                );
            }
            j = 0 as ::core::ffi::c_int;
            while j < (*functions.offset(i as isize)).line_count {
                if indent != 0 {
                    fputc('\t' as ::core::ffi::c_int, stdout);
                }
                fputs(
                    *(*functions.offset(i as isize)).lines.offset(j as isize),
                    stdout,
                );
                j += 1;
            }
            return 1 as ::core::ffi::c_int;
        }
        i += 1;
    }
    return 0 as ::core::ffi::c_int;
}
#[export_name = "rboxc_which_path_search"]
pub unsafe extern "C" fn path_search(
    mut indent: ::core::ffi::c_int,
    mut cmd: *const ::core::ffi::c_char,
    mut path_list: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut result: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut found_something: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !path_list.is_null() && *path_list as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        let mut next: ::core::ffi::c_int = 0;
        let mut path_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        loop {
            next = show_all;
            result = find_command_in_path(cmd, path_list, &raw mut path_index);
            if result.is_null() {
                break;
            }
            let mut full_path: *const ::core::ffi::c_char = path_clean_up(result);
            let mut in_home: ::core::ffi::c_int = ((show_tilde != 0 || skip_tilde != 0)
                && strncmp(
                    full_path,
                    &raw mut home as *mut ::core::ffi::c_char,
                    homelen,
                ) == 0) as ::core::ffi::c_int;
            if indent != 0 {
                fprintf(stdout, b"\t\0".as_ptr() as *const ::core::ffi::c_char);
            }
            's_13: {
                if !(skip_tilde != 0 && in_home != 0)
                    && show_dot != 0
                    && found_path_starts_with_dot != 0
                    && strncmp(full_path, &raw mut cwd as *mut ::core::ffi::c_char, cwdlen) == 0
                {
                    full_path = full_path.offset(cwdlen as isize);
                    fprintf(
                        stdout,
                        b".%c\0".as_ptr() as *const ::core::ffi::c_char,
                        DIR_SEPARATOR,
                    );
                } else if in_home != 0 {
                    if skip_tilde != 0 {
                        next = 1 as ::core::ffi::c_int;
                        free(result as *mut ::core::ffi::c_void);
                        break 's_13;
                    } else if show_tilde != 0 {
                        full_path = full_path.offset(homelen as isize);
                        fprintf(
                            stdout,
                            b"~%c\0".as_ptr() as *const ::core::ffi::c_char,
                            DIR_SEPARATOR,
                        );
                    }
                }
                fprintf(
                    stdout,
                    b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    full_path,
                );
                free(result as *mut ::core::ffi::c_void);
                found_something = 1 as ::core::ffi::c_int;
            }
            if next == 0 {
                break;
            }
        }
    }
    return found_something;
}
#[export_name = "rboxc_which_process_alias"]
pub unsafe extern "C" fn process_alias(
    mut str: *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut path_list: *const ::core::ffi::c_char,
    mut function_start_type: ::core::ffi::c_int,
) {
    let mut p: *const ::core::ffi::c_char = str;
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    if strncmp(
        b"alias\0".as_ptr() as *const ::core::ffi::c_char,
        p,
        5 as size_t,
    ) == 0
    {
        p = p.offset(5 as ::core::ffi::c_int as isize);
    }
    while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        p = p.offset(1);
    }
    while *p as ::core::ffi::c_int != 0
        && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
        && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
        && *p as ::core::ffi::c_int != '=' as ::core::ffi::c_int
    {
        p = p.offset(1);
        len += 1;
    }
    while argc > 0 as ::core::ffi::c_int {
        let mut q: ::core::ffi::c_char = 0 as ::core::ffi::c_char;
        let mut cmd: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if (*argv).is_null()
            || len as size_t != strlen(*argv)
            || strncmp(*argv, p.offset(-len as isize), len as size_t) != 0
        {
            argc -= 1;
            argv = argv.offset(1);
        } else {
            fputs(str, stdout);
            if show_all == 0 {
                *argv = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
            while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == '=' as ::core::ffi::c_int {
                p = p.offset(1);
            }
            while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
            {
                p = p.offset(1);
            }
            if *p as ::core::ffi::c_int == '"' as ::core::ffi::c_int
                || *p as ::core::ffi::c_int == '\'' as ::core::ffi::c_int
            {
                q = *p;
                p = p.offset(1);
            }
            loop {
                let mut found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while *p as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    || *p as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
                {
                    p = p.offset(1);
                }
                len = 0 as ::core::ffi::c_int;
                while *p as ::core::ffi::c_int != 0
                    && *p as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int != '\t' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int != q as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int != '|' as ::core::ffi::c_int
                    && *p as ::core::ffi::c_int != '&' as ::core::ffi::c_int
                {
                    p = p.offset(1);
                    len += 1;
                }
                cmd =
                    xmalloc((len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
                strncpy(cmd, p.offset(-len as isize), len as size_t);
                *cmd.offset(len as isize) = 0 as ::core::ffi::c_char;
                if !(*argv).is_null() && strcmp(cmd, *argv) == 0 {
                    *argv = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
                if read_functions != 0 && strchr(cmd, DIR_SEPARATOR).is_null() {
                    found =
                        func_search(1 as ::core::ffi::c_int, cmd, functions, function_start_type);
                }
                if show_all != 0 || found == 0 {
                    path_search(1 as ::core::ffi::c_int, cmd, path_list);
                }
                free(cmd as *mut ::core::ffi::c_void);
                while *p as ::core::ffi::c_int != 0
                    && (*p as ::core::ffi::c_int != '|' as ::core::ffi::c_int
                        || *p.offset(1isize) as ::core::ffi::c_int == '|' as ::core::ffi::c_int)
                    && (*p as ::core::ffi::c_int != '&' as ::core::ffi::c_int
                        || *p.offset(1isize) as ::core::ffi::c_int == '&' as ::core::ffi::c_int)
                {
                    p = p.offset(1);
                }
                if *p == 0 {
                    break;
                }
                p = p.offset(1);
            }
            break;
        }
    }
}
static mut superuser: uid_t = 0 as uid_t;
unsafe extern "C" fn rboxc_which_release_owned() {
    for i in 0..alias_count {
        free((*aliases.offset(i as isize)).cast());
    }
    free(aliases.cast());
    aliases = ::core::ptr::null_mut();
    alias_count = 0;
    for i in 0..func_count {
        let function = functions.offset(i as isize);
        free((*function).name.cast());
        for j in 0..(*function).line_count {
            free((*(*function).lines.offset(j as isize)).cast());
        }
        free((*function).lines.cast());
    }
    free(functions.cast());
    functions = ::core::ptr::null_mut();
    func_count = 0;
    free(abs_path.cast());
    abs_path = ::core::ptr::null_mut();
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_which(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut path_list: *mut ::core::ffi::c_char =
        getenv(b"PATH\0".as_ptr() as *const ::core::ffi::c_char);
    let mut short_option: ::core::ffi::c_int = 0;
    let mut fail_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    static mut long_option: ::core::ffi::c_int = 0;
    let mut longopts: [option; 13] = [
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_help.0 as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_version.0 as ::core::ffi::c_int,
        },
        option {
            name: b"skip-dot\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_skip_dot.0 as ::core::ffi::c_int,
        },
        option {
            name: b"skip-tilde\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_skip_tilde.0 as ::core::ffi::c_int,
        },
        option {
            name: b"show-dot\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_show_dot.0 as ::core::ffi::c_int,
        },
        option {
            name: b"show-tilde\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_show_tilde.0 as ::core::ffi::c_int,
        },
        option {
            name: b"tty-only\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_tty_only.0 as ::core::ffi::c_int,
        },
        option {
            name: b"all\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'a' as ::core::ffi::c_int,
        },
        option {
            name: b"read-alias\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'i' as ::core::ffi::c_int,
        },
        option {
            name: b"skip-alias\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_skip_alias.0 as ::core::ffi::c_int,
        },
        option {
            name: b"read-functions\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_read_functions.0 as ::core::ffi::c_int,
        },
        option {
            name: b"skip-functions\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: &raw mut long_option,
            val: opts::opt_skip_functions.0 as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    progname = *argv.offset(0isize);
    loop {
        short_option = getopt_long(
            argc,
            argv as *const *mut ::core::ffi::c_char,
            b"aivV\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut longopts as *mut option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if short_option == -1 as ::core::ffi::c_int {
            break;
        }
        match short_option {
            0 => match long_option {
                9 => {
                    print_usage(stdout);
                    return 0 as ::core::ffi::c_int;
                }
                0 => {
                    print_version();
                    return 0 as ::core::ffi::c_int;
                }
                1 => {
                    skip_dot = (tty_only == 0) as ::core::ffi::c_int;
                }
                2 => {
                    skip_tilde = (tty_only == 0) as ::core::ffi::c_int;
                }
                3 => {
                    skip_alias = 1 as ::core::ffi::c_int;
                }
                6 => {
                    show_dot = (tty_only == 0) as ::core::ffi::c_int;
                }
                7 => {
                    show_tilde = (tty_only == 0 && geteuid() != superuser) as ::core::ffi::c_int;
                }
                8 => {
                    tty_only = (isatty(1 as ::core::ffi::c_int) == 0) as ::core::ffi::c_int;
                }
                4 => {
                    read_functions = 1 as ::core::ffi::c_int;
                }
                5 => {
                    skip_functions = 1 as ::core::ffi::c_int;
                }
                _ => {}
            },
            97 => {
                show_all = 1 as ::core::ffi::c_int;
            }
            105 => {
                read_alias = 1 as ::core::ffi::c_int;
            }
            118 | 86 => {
                print_version();
                return 0 as ::core::ffi::c_int;
            }
            _ => {}
        }
    }
    uidget();
    if show_dot != 0 {
        get_current_working_directory();
    }
    if show_tilde != 0 || skip_tilde != 0 {
        let mut h: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        h = getenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
        if h.is_null() {
            h = sh_get_home_dir();
        }
        strncpy(
            &raw mut home as *mut ::core::ffi::c_char,
            h,
            ::core::mem::size_of::<[::core::ffi::c_char; 4096]>(),
        );
        home[::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize)] =
            0 as ::core::ffi::c_char;
        homelen = strlen(&raw mut home as *mut ::core::ffi::c_char);
        if homelen == 0 as size_t
            || !(home[homelen.wrapping_sub(1 as size_t)] as ::core::ffi::c_int == DIR_SEPARATOR)
                && homelen
                    < ::core::mem::size_of::<[::core::ffi::c_char; 4096]>().wrapping_sub(1usize)
        {
            home[homelen] = DIR_SEPARATOR as ::core::ffi::c_char;
            home[homelen.wrapping_add(1 as size_t)] = '\0' as ::core::ffi::c_char;
            homelen = homelen.wrapping_add(1);
        }
    }
    if skip_alias != 0 {
        read_alias = 0 as ::core::ffi::c_int;
    }
    if skip_functions != 0 {
        read_functions = 0 as ::core::ffi::c_int;
    }
    argv = argv.offset(optind as isize);
    argc -= optind;
    if argc == 0 as ::core::ffi::c_int {
        print_usage(stderr);
        return -1 as ::core::ffi::c_int;
    }
    let mut function_start_type: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if read_alias != 0 || read_functions != 0 {
        let mut buf: [::core::ffi::c_char; 1024] = [0; 1024];
        let mut processing_aliases: ::core::ffi::c_int = read_alias;
        if isatty(0 as ::core::ffi::c_int) != 0 {
            fprintf(
                stderr,
                b"%s: %s: Warning: stdin is a tty.\n\0".as_ptr() as *const ::core::ffi::c_char,
                progname,
                if read_functions != 0 {
                    if read_alias != 0 {
                        b"--read-functions, --read-alias, -i\0".as_ptr()
                            as *const ::core::ffi::c_char
                    } else {
                        b"--read-functions\0".as_ptr() as *const ::core::ffi::c_char
                    }
                } else {
                    b"--read-alias, -i\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        while !fgets(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
            stdin,
        )
        .is_null()
        {
            let mut looks_like_function_start: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut function_start_has_declare: ::core::ffi::c_int = 0;
            if read_functions != 0 {
                if buf[0usize] as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                    continue;
                }
                let mut p: *mut ::core::ffi::c_char = (&raw mut buf as *mut ::core::ffi::c_char)
                    .offset(strlen(&raw mut buf as *mut ::core::ffi::c_char) as isize)
                    .offset(-(1 as ::core::ffi::c_int as isize));
                while *(*__ctype_b_loc()).offset(*p as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
                    && p > (&raw mut buf as *mut ::core::ffi::c_char)
                        .offset(2 as ::core::ffi::c_int as isize)
                {
                    p = p.offset(-1);
                }
                if *p as ::core::ffi::c_int == ')' as ::core::ffi::c_int
                    && *p.offset(-1isize) as ::core::ffi::c_int == '(' as ::core::ffi::c_int
                    && *p.offset(-2isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                {
                    looks_like_function_start = 1 as ::core::ffi::c_int;
                    function_start_has_declare = (strncmp(
                        b"declare -\0".as_ptr() as *const ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                        9 as size_t,
                    ) == 0 as ::core::ffi::c_int)
                        as ::core::ffi::c_int;
                }
                if p > (&raw mut buf as *mut ::core::ffi::c_char)
                    .offset(4 as ::core::ffi::c_int as isize)
                    && *p as ::core::ffi::c_int == '{' as ::core::ffi::c_int
                    && *p.offset(-1isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                    && *p.offset(-2isize) as ::core::ffi::c_int == ')' as ::core::ffi::c_int
                    && *p.offset(-3isize) as ::core::ffi::c_int == '(' as ::core::ffi::c_int
                    && *p.offset(-4isize) as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
                {
                    looks_like_function_start = 1 as ::core::ffi::c_int;
                    function_start_type = 1 as ::core::ffi::c_int;
                    function_start_has_declare = 0 as ::core::ffi::c_int;
                }
            }
            if processing_aliases != 0 && looks_like_function_start == 0 {
                if strncmp(
                    b"declare -\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut buf as *mut ::core::ffi::c_char,
                    9 as size_t,
                ) == 0
                {
                    continue;
                }
                if alias_count == max_alias_count {
                    max_alias_count += 32 as ::core::ffi::c_int;
                    aliases = xrealloc(
                        aliases as *mut ::core::ffi::c_void,
                        (max_alias_count as size_t)
                            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
                    ) as *mut *mut ::core::ffi::c_char;
                }
                let c2rust_fresh4 = alias_count;
                alias_count += 1;
                *aliases.offset(c2rust_fresh4 as isize) = strcpy(
                    xmalloc(
                        strlen(&raw mut buf as *mut ::core::ffi::c_char).wrapping_add(1 as size_t),
                    ) as *mut ::core::ffi::c_char,
                    &raw mut buf as *mut ::core::ffi::c_char,
                );
            } else if read_functions != 0 && looks_like_function_start != 0 {
                let mut function: *mut function_st = ::core::ptr::null_mut::<function_st>();
                let mut max_line_count: ::core::ffi::c_int = 0;
                let mut p_0: *const ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
                let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                processing_aliases = 0 as ::core::ffi::c_int;
                if function_start_has_declare != 0 {
                    p_0 = p_0.offset(9 as ::core::ffi::c_int as isize);
                    while *p_0 as ::core::ffi::c_int != 0 && {
                        let c2rust_fresh5 = p_0;
                        p_0 = p_0.offset(1);
                        *c2rust_fresh5 as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                    } {}
                }
                while *p_0 as ::core::ffi::c_int != 0
                    && *p_0 as ::core::ffi::c_int != ' ' as ::core::ffi::c_int
                {
                    p_0 = p_0.offset(1);
                    len += 1;
                }
                if func_count == max_func_count {
                    max_func_count += 16 as ::core::ffi::c_int;
                    functions = xrealloc(
                        functions as *mut ::core::ffi::c_void,
                        (max_func_count as size_t)
                            .wrapping_mul(::core::mem::size_of::<function_st>()),
                    ) as *mut function_st;
                }
                let c2rust_fresh6 = func_count;
                func_count += 1;
                function = functions.offset(c2rust_fresh6 as isize);
                (*function).name =
                    xmalloc((len + 1 as ::core::ffi::c_int) as size_t) as *mut ::core::ffi::c_char;
                strncpy((*function).name, p_0.offset(-len as isize), len as size_t);
                *(*function).name.offset(len as isize) = 0 as ::core::ffi::c_char;
                (*function).len = len as size_t;
                max_line_count = 32 as ::core::ffi::c_int;
                (*function).lines = xmalloc(
                    (max_line_count as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
                ) as *mut *mut ::core::ffi::c_char;
                (*function).line_count = 0 as ::core::ffi::c_int;
                while !fgets(
                    &raw mut buf as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as ::core::ffi::c_int,
                    stdin,
                )
                .is_null()
                {
                    let mut blen: size_t = strlen(&raw mut buf as *mut ::core::ffi::c_char);
                    let c2rust_fresh7 = (*function).line_count;
                    (*function).line_count += 1;
                    *(*function).lines.offset(c2rust_fresh7 as isize) = strcpy(
                        xmalloc(blen.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char,
                        &raw mut buf as *mut ::core::ffi::c_char,
                    );
                    if strcmp(
                        &raw mut buf as *mut ::core::ffi::c_char,
                        b"}\n\0".as_ptr() as *const ::core::ffi::c_char,
                    ) == 0
                    {
                        break;
                    }
                    if (*function).line_count == max_line_count {
                        max_line_count += 32 as ::core::ffi::c_int;
                        (*function).lines = xrealloc(
                            (*function).lines as *mut ::core::ffi::c_void,
                            (max_line_count as size_t)
                                .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
                        )
                            as *mut *mut ::core::ffi::c_char;
                    }
                }
            }
        }
        if read_alias != 0 {
            let mut i: ::core::ffi::c_int = 0;
            i = 0 as ::core::ffi::c_int;
            while i < alias_count {
                process_alias(
                    *aliases.offset(i as isize),
                    argc,
                    argv,
                    path_list,
                    function_start_type,
                );
                i += 1;
            }
        }
    }
    while argc > 0 as ::core::ffi::c_int {
        let mut found_something: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !(*argv).is_null() {
            if read_functions != 0 && strchr(*argv, DIR_SEPARATOR).is_null() {
                found_something = func_search(
                    0 as ::core::ffi::c_int,
                    *argv,
                    functions,
                    function_start_type,
                );
            }
            if (show_all != 0 || found_something == 0)
                && path_search(0 as ::core::ffi::c_int, *argv, path_list) == 0
                && found_something == 0
            {
                print_fail(
                    if path_containing_separator_given != 0 {
                        strrchr(*argv, DIR_SEPARATOR).offset(1 as ::core::ffi::c_int as isize)
                    } else {
                        *argv
                    },
                    if path_containing_separator_given != 0 {
                        abs_path
                    } else {
                        path_list
                    },
                );
                fail_count += 1;
            }
        }
        argc -= 1;
        argv = argv.offset(1);
    }
    rboxc_which_release_owned();
    return fail_count;
}
#[export_name = "rboxc_which_xmalloc"]
pub unsafe extern "C" fn xmalloc(mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut ::core::ffi::c_void = malloc(size);
    if ptr.is_null() {
        fprintf(
            stderr,
            b"%s: Out of memory\0".as_ptr() as *const ::core::ffi::c_char,
            progname,
        );
        exit(-1 as ::core::ffi::c_int);
    }
    return ptr;
}
#[export_name = "rboxc_which_xrealloc"]
pub unsafe extern "C" fn xrealloc(
    mut ptr: *mut ::core::ffi::c_void,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    if ptr.is_null() {
        return xmalloc(size);
    }
    ptr = realloc(ptr, size);
    if size > 0 as size_t && ptr.is_null() {
        fprintf(
            stderr,
            b"%s: Out of memory\n\0".as_ptr() as *const ::core::ffi::c_char,
            progname,
        );
        exit(-1 as ::core::ffi::c_int);
    }
    return ptr;
}
