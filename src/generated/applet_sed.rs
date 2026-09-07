// Generated from pinned GNU Sed 4.10 by scripts/translate-sed.py.
// Source SHA-256: 800c10186e55d7475bd57459ef6577c1e8261352599fbe81126c23bf906a8ace
/*  GNU SED, a batch stream editor.
    Copyright (C) 1989-2026 Free Software Foundation, Inc.

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 3, or (at your option)
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; If not, see <https://www.gnu.org/licenses/>. */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct re_dfa_t { _opaque: [u8; 0] }
#[repr(C)]
pub struct dfa { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_sed_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_sed_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_sed_xnmalloc"]
    fn xnmalloc(n: size_t, s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_sed_init_localeinfo"]
    fn init_localeinfo(_: *mut localeinfo);
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_sed_ck_fclose"]
    fn ck_fclose(stream: *mut FILE);
    #[link_name = "rboxc_sed_remove_cleanup_file"]
    fn remove_cleanup_file();
    #[link_name = "rboxc_sed_compile_string"]
    fn compile_string(_: *mut vector, str: *mut ::core::ffi::c_char, len: idx_t) -> *mut vector;
    #[link_name = "rboxc_sed_compile_file"]
    fn compile_file(_: *mut vector, cmdfile: *const ::core::ffi::c_char) -> *mut vector;
    #[link_name = "rboxc_sed_check_final_program"]
    fn check_final_program(_: *mut vector);
    #[link_name = "rboxc_sed_finish_program"]
    fn finish_program(_: *mut vector);
    #[link_name = "rboxc_sed_debug_print_program"]
    fn debug_print_program(program: *const vector);
    #[link_name = "rboxc_sed_process_files"]
    fn process_files(_: *mut vector, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_sed_initialize_mbcs"]
    fn initialize_mbcs();
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn strtoimax(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> intmax_t;
    #[link_name = "rboxc_sed_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_sed_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_sed_Version"]
    static mut Version: *const ::core::ffi::c_char;
    #[link_name = "rboxc_sed_version_etc"]
    fn version_etc(
        stream: *mut FILE,
        command_name: *const ::core::ffi::c_char,
        package: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        ...
    );
}
pub type size_t = usize;
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
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
pub type FILE = _IO_FILE;
pub type ptrdiff_t = isize;
pub type intmax_t = ::libc::intmax_t;
pub type idx_t = ptrdiff_t;
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
pub type regex_t = re_pattern_buffer;
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
pub struct exit_codes(pub ::core::ffi::c_uint);
impl exit_codes {
    pub const EXIT_BAD_USAGE: Self = Self(1);
    pub const EXIT_BAD_INPUT: Self = Self(2);
    pub const EXIT_PANIC: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vector {
    pub v: *mut sed_cmd,
    pub v_allocated: idx_t,
    pub v_length: idx_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sed_cmd {
    pub a1: *mut addr,
    pub a2: *mut addr,
    pub range_state: addr_state,
    pub addr_bang: ::core::ffi::c_char,
    pub cmd: ::core::ffi::c_char,
    pub x: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub cmd_txt: text_buf,
    pub int_arg: intmax_t,
    pub jump_index: idx_t,
    pub readcmd: readcmd,
    pub cmd_subst: *mut subst,
    pub outf: *mut output,
    pub inf: *mut output,
    pub translate: *mut ::core::ffi::c_uchar,
    pub translatemb: *mut *mut ::core::ffi::c_char,
    pub label_name: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct output {
    pub name: *mut ::core::ffi::c_char,
    pub missing_newline: bool,
    pub fp: *mut FILE,
    pub link: *mut output,
}
#[derive(Copy, Clone, ::c2rust_bitfields::BitfieldStruct)]
#[repr(C)]
pub struct subst {
    pub regx: *mut regex,
    pub replacement: *mut replacement,
    pub numb: intmax_t,
    pub outf: *mut output,
    #[bitfield(name = "global", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "print", ty = "::core::ffi::c_uint", bits = "1..=2")]
    #[bitfield(name = "eval", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "max_id", ty = "::core::ffi::c_uint", bits = "4..=7")]
    pub global_print_eval_max_id: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct replacement {
    pub prefix: *mut ::core::ffi::c_char,
    pub prefix_length: idx_t,
    pub subst_id: ::core::ffi::c_int,
    pub repl_type: replacement_types,
    pub next: *mut replacement,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct replacement_types(pub ::core::ffi::c_uint);
impl replacement_types {
    pub const REPL_ASIS: Self = Self(0);
    pub const REPL_UPPERCASE: Self = Self(1);
    pub const REPL_LOWERCASE: Self = Self(2);
    pub const REPL_UPPERCASE_FIRST: Self = Self(4);
    pub const REPL_LOWERCASE_FIRST: Self = Self(8);
    pub const REPL_MODIFIERS: Self = Self(12);
    pub const REPL_UPPERCASE_UPPERCASE: Self = Self(5);
    pub const REPL_UPPERCASE_LOWERCASE: Self = Self(6);
    pub const REPL_LOWERCASE_UPPERCASE: Self = Self(9);
    pub const REPL_LOWERCASE_LOWERCASE: Self = Self(10);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct regex {
    pub pattern: regex_t,
    pub flags: ::core::ffi::c_int,
    pub sz: idx_t,
    pub dfa: *mut dfa,
    pub begline: bool,
    pub endline: bool,
    pub re: [::core::ffi::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct readcmd {
    pub fname: *mut ::core::ffi::c_char,
    pub append: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct text_buf {
    pub text: *mut ::core::ffi::c_char,
    pub text_length: idx_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct addr_state(pub ::core::ffi::c_uint);
impl addr_state {
    pub const RANGE_INACTIVE: Self = Self(0);
    pub const RANGE_ACTIVE: Self = Self(1);
    pub const RANGE_CLOSED: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addr {
    pub addr_type: addr_types,
    pub addr_number: intmax_t,
    pub addr_step: intmax_t,
    pub addr_regex: *mut regex,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct addr_types(pub ::core::ffi::c_uint);
impl addr_types {
    pub const ADDR_IS_NULL: Self = Self(0);
    pub const ADDR_IS_REGEX: Self = Self(1);
    pub const ADDR_IS_NUM: Self = Self(2);
    pub const ADDR_IS_NUM_MOD: Self = Self(3);
    pub const ADDR_IS_STEP: Self = Self(4);
    pub const ADDR_IS_STEP_MOD: Self = Self(5);
    pub const ADDR_IS_LAST: Self = Self(6);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct posixicity_types(pub ::core::ffi::c_uint);
impl posixicity_types {
    pub const POSIXLY_EXTENDED: Self = Self(0);
    pub const POSIXLY_CORRECT: Self = Self(1);
    pub const POSIXLY_BASIC: Self = Self(2);
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
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const SANDBOX_OPTION: Self = Self(128);
    pub const DEBUG_OPTION: Self = Self(129);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const REG_EXTENDED: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn putchar_unlocked(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if ((*stdout)._IO_write_ptr >= (*stdout)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(stdout, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh0 = (*stdout)._IO_write_ptr;
        (*stdout)._IO_write_ptr = (*stdout)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh0;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
    };
}
#[export_name = "rboxc_sed_extended_regexp_flags"]
pub static mut extended_regexp_flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[export_name = "rboxc_sed_buffer_delimiter"]
pub static mut buffer_delimiter: ::core::ffi::c_char = '\n' as ::core::ffi::c_char;
#[export_name = "rboxc_sed_unbuffered"]
pub static mut unbuffered: bool = r#false != 0;
#[export_name = "rboxc_sed_no_default_output"]
pub static mut no_default_output: bool = r#false != 0;
#[export_name = "rboxc_sed_separate_files"]
pub static mut separate_files: bool = r#false != 0;
#[export_name = "rboxc_sed_follow_symlinks"]
pub static mut follow_symlinks: bool = r#false != 0;
#[export_name = "rboxc_sed_sandbox"]
pub static mut sandbox: bool = r#false != 0;
#[export_name = "rboxc_sed_debug"]
pub static mut debug: bool = r#false != 0;
#[export_name = "rboxc_sed_in_place_extension"]
pub static mut in_place_extension: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_sed_read_mode"]
pub static mut read_mode: *const ::core::ffi::c_char =
    b"r\0".as_ptr() as *const ::core::ffi::c_char;
#[export_name = "rboxc_sed_write_mode"]
pub static mut write_mode: *const ::core::ffi::c_char =
    b"w\0".as_ptr() as *const ::core::ffi::c_char;
#[export_name = "rboxc_sed_posixicity"]
pub static mut posixicity: posixicity_types = posixicity_types::POSIXLY_EXTENDED;
#[export_name = "rboxc_sed_lcmd_out_line_len"]
pub static mut lcmd_out_line_len: intmax_t = 70 as intmax_t;
static mut the_program: *mut vector = ::core::ptr::null_mut::<vector>();
#[export_name = "rboxc_sed_localeinfo"]
pub static mut localeinfo: localeinfo = localeinfo {
    multibyte: false,
    simple: false,
    using_utf8: false,
    sbclen: [0; 256],
    sbctowc: [0; 256],
};
unsafe extern "C" fn cleanup() {
    remove_cleanup_file();
}
unsafe extern "C" fn contact(mut errmsg: ::core::ffi::c_int) {
    let mut out: *mut FILE = if errmsg != 0 { stderr } else { stdout };
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"GNU sed home page: <https://www.gnu.org/software/sed/>.\nGeneral help using GNU software: <https://www.gnu.org/gethelp/>.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    if errmsg == 0 {
        fprintf(
            out,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"E-mail bug reports to: <%s>.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            PACKAGE_BUGREPORT.as_ptr(),
        );
    }
}
unsafe extern "C" fn selinux_support() {
    putchar_unlocked('\n' as ::core::ffi::c_int);
    puts(dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"This sed program was built without SELinux support.\0".as_ptr()
            as *const ::core::ffi::c_char,
        LC_MESSAGES,
    ));
    putchar_unlocked('\n' as ::core::ffi::c_int);
}
unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) {
    let mut out: *mut FILE = if status != 0 { stderr } else { stdout };
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... {script-only-if-no-other-script} [input-file]...\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        program_name,
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -n, --quiet, --silent\n                 suppress automatic printing of pattern space\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --debug\n                 annotate program execution\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -e script, --expression=script\n                 add the script to the commands to be executed\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -f script-file, --file=script-file\n                 add the contents of script-file to the commands to be executed\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  --follow-symlinks\n                 follow symlinks when processing in place\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -i[SUFFIX], --in-place[=SUFFIX]\n                 edit files in place (makes backup if SUFFIX supplied)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -l N, --line-length=N\n                 specify the desired line-wrap length for the 'l' command\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  --posix\n                 disable all GNU extensions.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -E, -r, --regexp-extended\n                 use extended regular expressions in the script\n                 (for portability use POSIX -E).\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -s, --separate\n                 consider files as separate rather than as a single,\n                 continuous long stream.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --sandbox\n                 operate in sandbox mode (disable e/r/w commands).\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -u, --unbuffered\n                 load minimal amounts of data from the input files and flush\n                 the output buffers more often\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -z, --null-data\n                 separate lines by NUL characters\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --help     display this help and exit\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --version  output version information and exit\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fprintf(
        out,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\nIf no -e, --expression, -f, or --file option is given, then the first\nnon-option argument is taken as the sed script to interpret.  All\nremaining arguments are names of input files; if no input files are\nspecified, then the standard input is read.\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    contact(status);
    ck_fclose(::core::ptr::null_mut::<FILE>());
    exit(status);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_sed(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    static mut longopts: [option; 19] = [
        option {
            name: b"binary\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'b' as ::core::ffi::c_int,
        },
        option {
            name: b"regexp-extended\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'r' as ::core::ffi::c_int,
        },
        option {
            name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_0::DEBUG_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"expression\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'e' as ::core::ffi::c_int,
        },
        option {
            name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'f' as ::core::ffi::c_int,
        },
        option {
            name: b"in-place\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 2 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'i' as ::core::ffi::c_int,
        },
        option {
            name: b"line-length\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 1 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'l' as ::core::ffi::c_int,
        },
        option {
            name: b"null-data\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'z' as ::core::ffi::c_int,
        },
        option {
            name: b"zero-terminated\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'z' as ::core::ffi::c_int,
        },
        option {
            name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'n' as ::core::ffi::c_int,
        },
        option {
            name: b"posix\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'p' as ::core::ffi::c_int,
        },
        option {
            name: b"silent\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'n' as ::core::ffi::c_int,
        },
        option {
            name: b"sandbox\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: C2Rust_Unnamed_0::SANDBOX_OPTION.0 as ::core::ffi::c_int,
        },
        option {
            name: b"separate\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 's' as ::core::ffi::c_int,
        },
        option {
            name: b"unbuffered\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'u' as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as ::core::ffi::c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"follow-symlinks\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'F' as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 0 as ::core::ffi::c_int,
        },
    ];
    let mut opt: ::core::ffi::c_int = 0;
    let mut return_code: ::core::ffi::c_int = 0;
    let mut cols: *const ::core::ffi::c_char =
        getenv(b"COLS\0".as_ptr() as *const ::core::ffi::c_char);
    set_program_name(*argv.offset(0isize));
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    initialize_mbcs();
    init_localeinfo(&raw mut localeinfo);
    atexit(Some(cleanup as unsafe extern "C" fn() -> ()));
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    bindtextdomain(
        b"gnulib\0".as_ptr() as *const ::core::ffi::c_char,
        GNULIB_LOCALEDIR.as_ptr(),
    );
    textdomain(PACKAGE.as_ptr());
    if !getenv(b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        posixicity = posixicity_types::POSIXLY_CORRECT;
    } else {
        posixicity = posixicity_types::POSIXLY_EXTENDED;
    }
    if !cols.is_null() {
        let mut t: intmax_t = strtoimax(
            cols,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            10 as ::core::ffi::c_int,
        );
        if t > 1 as intmax_t {
            lcmd_out_line_len = t - 1 as intmax_t;
        }
    }
    loop {
        opt = getopt_long(
            argc,
            argv,
            SHORTOPTS.as_ptr(),
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if opt == EOF {
            break;
        }
        's_242: {
            match opt {
                110 => {
                    no_default_output = r#true != 0;
                    break 's_242;
                }
                101 => {
                    the_program = compile_string(the_program, optarg, strlen(optarg) as idx_t);
                    break 's_242;
                }
                102 => {
                    the_program = compile_file(the_program, optarg);
                    break 's_242;
                }
                122 => {
                    buffer_delimiter = 0 as ::core::ffi::c_char;
                    break 's_242;
                }
                70 => {
                    follow_symlinks = r#true != 0;
                    break 's_242;
                }
                105 => {
                    separate_files = r#true != 0;
                    if optarg.is_null() {
                        in_place_extension = xstrdup(b"*\0".as_ptr() as *const ::core::ffi::c_char);
                    } else if !strchr(optarg, '*' as ::core::ffi::c_int).is_null() {
                        in_place_extension = xstrdup(optarg);
                    } else {
                        in_place_extension =
                            (if ::core::mem::size_of::<::core::ffi::c_char>() == 1usize {
                                xmalloc(strlen(optarg).wrapping_add(2 as size_t))
                            } else {
                                xnmalloc(
                                    strlen(optarg).wrapping_add(2 as size_t),
                                    ::core::mem::size_of::<::core::ffi::c_char>(),
                                )
                            }) as *mut ::core::ffi::c_char;
                        *in_place_extension.offset(0isize) = '*' as ::core::ffi::c_char;
                        strcpy(
                            in_place_extension.offset(1 as ::core::ffi::c_int as isize),
                            optarg,
                        );
                    }
                    break 's_242;
                }
                108 => {
                    lcmd_out_line_len = strtoimax(
                        optarg,
                        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                        10 as ::core::ffi::c_int,
                    );
                    break 's_242;
                }
                112 => {
                    posixicity = posixicity_types::POSIXLY_BASIC;
                    break 's_242;
                }
                98 => {
                    read_mode = b"rb\0".as_ptr() as *const ::core::ffi::c_char;
                    write_mode = b"wb\0".as_ptr() as *const ::core::ffi::c_char;
                    break 's_242;
                }
                69 | 114 => {
                    extended_regexp_flags = REG_EXTENDED;
                    break 's_242;
                }
                115 => {
                    separate_files = r#true != 0;
                    break 's_242;
                }
                128 => {
                    sandbox = r#true != 0;
                    break 's_242;
                }
                129 => {
                    debug = r#true != 0;
                    break 's_242;
                }
                117 => {
                    unbuffered = r#true != 0;
                    break 's_242;
                }
                118 => {
                    version_etc(
                        stdout,
                        program_name,
                        PACKAGE_NAME.as_ptr(),
                        Version,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Jay Fenlason\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Tom Lord\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Ken Pizzini\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Paolo Bonzini\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Jim Meyering\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Assaf Gordon\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                        NULL as *mut ::core::ffi::c_char,
                    );
                    selinux_support();
                    contact(r#false);
                    ck_fclose(::core::ptr::null_mut::<FILE>());
                    exit(EXIT_SUCCESS);
                }
                104 => {
                    usage(EXIT_SUCCESS);
                }
                _ => {}
            }
            usage(exit_codes::EXIT_BAD_USAGE.0 as ::core::ffi::c_int);
        }
    }
    if the_program.is_null() {
        if optind < argc {
            let c2rust_fresh1 = optind;
            optind += 1;
            let mut arg: *mut ::core::ffi::c_char = *argv.offset(c2rust_fresh1 as isize);
            the_program = compile_string(the_program, arg, strlen(arg) as idx_t);
        } else {
            usage(exit_codes::EXIT_BAD_USAGE.0 as ::core::ffi::c_int);
        }
    }
    check_final_program(the_program);
    if debug {
        debug_print_program(the_program);
    }
    return_code = process_files(the_program, argv.offset(optind as isize));
    finish_program(the_program);
    ck_fclose(::core::ptr::null_mut::<FILE>());
    return return_code;
}
pub const SHORTOPTS: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"bsnrzuEe:f:l:i::V:\0")
};
pub const LOCALEDIR: [::core::ffi::c_char; 42] = unsafe {
    ::core::mem::transmute::<[u8; 42], [::core::ffi::c_char; 42]>(
        *b"/root/rboxc/build/oracle/sed/share/locale\0",
    )
};
pub const GNULIB_LOCALEDIR: [::core::ffi::c_char; 42] = unsafe {
    ::core::mem::transmute::<[u8; 42], [::core::ffi::c_char; 42]>(
        *b"/root/rboxc/build/oracle/sed/share/locale\0",
    )
};
pub const PACKAGE: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"sed\0") };
pub const PACKAGE_BUGREPORT: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"bug-sed@gnu.org\0") };
pub const PACKAGE_NAME: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"GNU sed\0") };
