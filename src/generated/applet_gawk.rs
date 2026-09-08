// Generated from pinned GNU gawk 5.4.1 by scripts/translate-entry-provider.py.
// Source SHA-256: 0542fce6748ae8b476b8c39952ab749e65525dd983f88cd7ff8a096aef929c16
/*
 * main.c -- Code generator and main program for gawk.
 */
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
#[repr(C)]
pub struct break_point { _opaque: [u8; 0] }
#[repr(C)]
pub struct instruction_block { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn __ctype_get_mb_cur_max() -> size_t;
    fn abort() -> !;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_pma_version"]
    static pma_version: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_gawk_pma_errno"]
    static mut pma_errno: ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_pma_init"]
    fn pma_init(
        verbose: ::core::ffi::c_int,
        file: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_pma_malloc"]
    fn pma_malloc(size: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_gawk_pma_calloc"]
    fn pma_calloc(nmemb: size_t, size: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_gawk_pma_realloc"]
    fn pma_realloc(ptr: *mut ::core::ffi::c_void, size: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_gawk_pma_free"]
    fn pma_free(ptr: *mut ::core::ffi::c_void);
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn sscanf(
        __s: *const ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn ferror(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn localeconv() -> *mut lconv;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strsignal(__sig: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getpgrp() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn getgroups(__size: ::core::ffi::c_int, __list: *mut __gid_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_optarg"]
    static mut optarg: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_optind"]
    static mut optind: ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_opterr"]
    static mut opterr: ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_optopt"]
    static mut optopt: ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_sourceline"]
    static mut sourceline: ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_source"]
    static mut source: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_errcount"]
    static mut errcount: ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_version_string"]
    static mut version_string: *const ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_interpret"]
    static mut interpret: Option<unsafe extern "C" fn(*mut INSTRUCTION) -> ::core::ffi::c_int>;
    #[link_name = "rboxc_gawk_make_number"]
    static mut make_number: Option<unsafe extern "C" fn(::core::ffi::c_double) -> *mut NODE>;
    #[link_name = "rboxc_gawk_str2number"]
    static mut str2number: Option<unsafe extern "C" fn(*mut NODE) -> *mut NODE>;
    #[link_name = "rboxc_gawk_str_array_func"]
    static str_array_func: array_funcs_t;
    #[link_name = "rboxc_gawk_quote"]
    static quote: ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_defpath"]
    static mut defpath: *const ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_deflibpath"]
    static mut deflibpath: *const ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_awk_namespace"]
    static awk_namespace: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_gawk_current_namespace"]
    static mut current_namespace: *const ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_r_unref"]
    fn r_unref(tmp: *mut NODE);
    #[link_name = "rboxc_gawk_r_getblock"]
    fn r_getblock(id: ::core::ffi::c_int) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_gawk_make_array"]
    fn make_array() -> *mut NODE;
    #[link_name = "rboxc_gawk_null_array"]
    fn null_array(symbol: *mut NODE);
    #[link_name = "rboxc_gawk_array_init"]
    fn array_init();
    #[link_name = "rboxc_gawk_set_SUBSEP"]
    fn set_SUBSEP();
    #[link_name = "rboxc_gawk_init_env_array"]
    fn init_env_array(env_node: *mut NODE);
    #[link_name = "rboxc_gawk_init_argv_array"]
    fn init_argv_array(argv_node: *mut NODE, shadow_node: *mut NODE);
    #[link_name = "rboxc_gawk_variable"]
    fn variable(
        location: ::core::ffi::c_int,
        name: *mut ::core::ffi::c_char,
        r#type: NODETYPE,
    ) -> *mut NODE;
    #[link_name = "rboxc_gawk_parse_program"]
    fn parse_program(pcode: *mut *mut INSTRUCTION, from_eval: bool) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_dump_funcs"]
    fn dump_funcs();
    #[link_name = "rboxc_gawk_dump_vars"]
    fn dump_vars(fname: *const ::core::ffi::c_char);
    #[link_name = "rboxc_gawk_shadow_funcs"]
    fn shadow_funcs();
    #[link_name = "rboxc_gawk_check_special"]
    fn check_special(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_add_srcfile"]
    fn add_srcfile(
        stype: srctype,
        src: *mut ::core::ffi::c_char,
        curr: *mut SRCFILE,
        already_included: *mut bool,
        errcode: *mut ::core::ffi::c_int,
    ) -> *mut SRCFILE;
    #[link_name = "rboxc_gawk_install_builtins"]
    fn install_builtins();
    #[link_name = "rboxc_gawk_is_letter"]
    fn is_letter(c: ::core::ffi::c_int) -> bool;
    #[link_name = "rboxc_gawk_is_identchar"]
    fn is_identchar(c: ::core::ffi::c_int) -> bool;
    #[link_name = "rboxc_gawk_validate_qualified_name"]
    fn validate_qualified_name(token: *mut ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_gawk_init_debug"]
    fn init_debug();
    #[link_name = "rboxc_gawk_debug_prog"]
    fn debug_prog(pc: *mut INSTRUCTION) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_init_interpret"]
    fn init_interpret();
    #[link_name = "rboxc_gawk_set_IGNORECASE"]
    fn set_IGNORECASE();
    #[link_name = "rboxc_gawk_set_OFS"]
    fn set_OFS();
    #[link_name = "rboxc_gawk_set_ORS"]
    fn set_ORS();
    #[link_name = "rboxc_gawk_set_OFMT"]
    fn set_OFMT();
    #[link_name = "rboxc_gawk_set_CONVFMT"]
    fn set_CONVFMT();
    #[link_name = "rboxc_gawk_set_BINMODE"]
    fn set_BINMODE();
    #[link_name = "rboxc_gawk_set_LINT"]
    fn set_LINT();
    #[link_name = "rboxc_gawk_set_TEXTDOMAIN"]
    fn set_TEXTDOMAIN();
    #[link_name = "rboxc_gawk_update_NR"]
    fn update_NR();
    #[link_name = "rboxc_gawk_update_NF"]
    fn update_NF();
    #[link_name = "rboxc_gawk_update_FNR"]
    fn update_FNR();
    #[link_name = "rboxc_gawk_load_casetable"]
    fn load_casetable();
    #[link_name = "rboxc_gawk_r_get_lhs"]
    fn r_get_lhs(n: *mut NODE, reference: bool) -> *mut *mut NODE;
    #[link_name = "rboxc_gawk_elem_new_reset"]
    fn elem_new_reset(n: *mut NODE);
    #[link_name = "rboxc_gawk_load_ext"]
    fn load_ext(name: *const ::core::ffi::c_char, lib_name: *const ::core::ffi::c_char);
    #[link_name = "rboxc_gawk_init_fields"]
    fn init_fields();
    #[link_name = "rboxc_gawk_init_csv_fields"]
    fn init_csv_fields();
    #[link_name = "rboxc_gawk_set_NF"]
    fn set_NF();
    #[link_name = "rboxc_gawk_set_FS"]
    fn set_FS();
    #[link_name = "rboxc_gawk_set_RS"]
    fn set_RS();
    #[link_name = "rboxc_gawk_set_FIELDWIDTHS"]
    fn set_FIELDWIDTHS();
    #[link_name = "rboxc_gawk_set_FPAT"]
    fn set_FPAT();
    #[link_name = "rboxc_gawk_update_PROCINFO_str"]
    fn update_PROCINFO_str(subscript: *const ::core::ffi::c_char, str: *const ::core::ffi::c_char);
    #[link_name = "rboxc_gawk_update_PROCINFO_num"]
    fn update_PROCINFO_num(subscript: *const ::core::ffi::c_char, val: ::core::ffi::c_double);
    #[link_name = "rboxc_gawk_current_field_sep_str"]
    fn current_field_sep_str() -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_init_ext_api"]
    fn init_ext_api();
    #[link_name = "rboxc_gawk_print_ext_versions"]
    fn print_ext_versions();
    #[link_name = "rboxc_gawk_gawk_name"]
    fn gawk_name(filespec: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_gawk_os_arg_fixup"]
    fn os_arg_fixup(argcp: *mut ::core::ffi::c_int, argvp: *mut *mut *mut ::core::ffi::c_char);
    #[link_name = "rboxc_gawk_os_isatty"]
    fn os_isatty(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_os_is_setuid"]
    fn os_is_setuid() -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_os_setbinmode"]
    fn os_setbinmode(fd: ::core::ffi::c_int, mode: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_os_maybe_set_errno"]
    fn os_maybe_set_errno();
    #[link_name = "rboxc_gawk_os_disable_aslr"]
    fn os_disable_aslr(
        persist_file_0: *const ::core::ffi::c_char,
        argv: *mut *mut ::core::ffi::c_char,
    );
    #[link_name = "rboxc_gawk_init_io"]
    fn init_io();
    #[link_name = "rboxc_gawk_init_csv_records"]
    fn init_csv_records();
    #[link_name = "rboxc_gawk_set_FNR"]
    fn set_FNR();
    #[link_name = "rboxc_gawk_set_NR"]
    fn set_NR();
    #[link_name = "rboxc_gawk_devopen"]
    fn devopen(
        name: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_gawk_set_PREC"]
    fn set_PREC();
    #[link_name = "rboxc_gawk_set_ROUNDMODE"]
    fn set_ROUNDMODE();
    #[link_name = "rboxc_gawk_final_exit"]
    fn final_exit(status: ::core::ffi::c_int) -> !;
    #[link_name = "rboxc_gawk_msg"]
    fn msg(mesg: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_gawk_r_warning"]
    fn r_warning(mesg: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_gawk_set_loc"]
    fn set_loc(file: *const ::core::ffi::c_char, line: ::core::ffi::c_int);
    #[link_name = "rboxc_gawk_r_fatal"]
    fn r_fatal(mesg: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_gawk_init_profiling_signals"]
    fn init_profiling_signals();
    #[link_name = "rboxc_gawk_set_prof_file"]
    fn set_prof_file(filename: *const ::core::ffi::c_char);
    #[link_name = "rboxc_gawk_close_prof_file"]
    fn close_prof_file();
    #[link_name = "rboxc_gawk_dump_prog"]
    fn dump_prog(code: *mut INSTRUCTION);
    #[link_name = "rboxc_gawk_make_str_node"]
    fn make_str_node(
        s: *const ::core::ffi::c_char,
        len: size_t,
        flags: ::core::ffi::c_int,
    ) -> *mut NODE;
    #[link_name = "rboxc_gawk_make_typed_regex"]
    fn make_typed_regex(re: *const ::core::ffi::c_char, len: size_t) -> *mut NODE;
    #[link_name = "rboxc_gawk_init_btowc_cache"]
    fn init_btowc_cache();
    #[link_name = "rboxc_gawk_resetup"]
    fn resetup();
    #[link_name = "rboxc_gawk_load_symbols"]
    fn load_symbols();
    #[link_name = "rboxc_gawk_init_symbol_table"]
    fn init_symbol_table();
    #[link_name = "rboxc_gawk_install_symbol"]
    fn install_symbol(name: *const ::core::ffi::c_char, r#type: NODETYPE) -> *mut NODE;
    #[link_name = "rboxc_gawk_lookup"]
    fn lookup(name: *const ::core::ffi::c_char) -> *mut NODE;
    #[link_name = "rboxc_gawk_release_all_vars"]
    fn release_all_vars();
    #[link_name = "rboxc_gawk_new_context"]
    fn new_context() -> *mut AWK_CONTEXT;
    #[link_name = "rboxc_gawk_push_context"]
    fn push_context(ctxt: *mut AWK_CONTEXT);
    #[link_name = "rboxc_gawk_pma_mpfr_check"]
    fn pma_mpfr_check();
    #[link_name = "rboxc_gawk_pma_save_free_lists"]
    fn pma_save_free_lists();
    #[link_name = "rboxc_gawk_getopt_long"]
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn mtrace();
}
pub type size_t = usize;
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
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type gid_t = __gid_t;
pub type time_t = __time_t;
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
pub struct lconv {
    pub decimal_point: *mut ::core::ffi::c_char,
    pub thousands_sep: *mut ::core::ffi::c_char,
    pub grouping: *mut ::core::ffi::c_char,
    pub int_curr_symbol: *mut ::core::ffi::c_char,
    pub currency_symbol: *mut ::core::ffi::c_char,
    pub mon_decimal_point: *mut ::core::ffi::c_char,
    pub mon_thousands_sep: *mut ::core::ffi::c_char,
    pub mon_grouping: *mut ::core::ffi::c_char,
    pub positive_sign: *mut ::core::ffi::c_char,
    pub negative_sign: *mut ::core::ffi::c_char,
    pub int_frac_digits: ::core::ffi::c_char,
    pub frac_digits: ::core::ffi::c_char,
    pub p_cs_precedes: ::core::ffi::c_char,
    pub p_sep_by_space: ::core::ffi::c_char,
    pub n_cs_precedes: ::core::ffi::c_char,
    pub n_sep_by_space: ::core::ffi::c_char,
    pub p_sign_posn: ::core::ffi::c_char,
    pub n_sign_posn: ::core::ffi::c_char,
    pub int_p_cs_precedes: ::core::ffi::c_char,
    pub int_p_sep_by_space: ::core::ffi::c_char,
    pub int_n_cs_precedes: ::core::ffi::c_char,
    pub int_n_sep_by_space: ::core::ffi::c_char,
    pub int_p_sign_posn: ::core::ffi::c_char,
    pub int_n_sign_posn: ::core::ffi::c_char,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type char32_t = __uint_least32_t;
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
pub type __re_size_t = ::core::ffi::c_uint;
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
pub type regoff_t = ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct re_registers {
    pub num_regs: __re_size_t,
    pub start: *mut regoff_t,
    pub end: *mut regoff_t,
}
pub type ptrdiff_t = isize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct minrx_regcomp_flags_t(pub ::core::ffi::c_uint);
impl minrx_regcomp_flags_t {
    pub const MINRX_REG_EXTENDED: Self = Self(1);
    pub const MINRX_REG_ICASE: Self = Self(2);
    pub const MINRX_REG_MINIMAL: Self = Self(4);
    pub const MINRX_REG_NEWLINE: Self = Self(8);
    pub const MINRX_REG_NOSUB: Self = Self(16);
    pub const MINRX_REG_BRACE_COMPAT: Self = Self(32);
    pub const MINRX_REG_BRACK_ESCAPE: Self = Self(64);
    pub const MINRX_REG_EXTENSIONS_BSD: Self = Self(128);
    pub const MINRX_REG_EXTENSIONS_GNU: Self = Self(256);
    pub const MINRX_REG_NATIVE1B: Self = Self(512);
    pub const MINRX_REG_MINDISABLE: Self = Self(1024);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minrx_regex_t {
    pub re_regexp: *mut ::core::ffi::c_void,
    pub re_nsub: size_t,
    pub re_compflags: minrx_regcomp_flags_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct minrx_regmatch_t {
    pub rm_so: ptrdiff_t,
    pub rm_eo: ptrdiff_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Regexp {
    pub pat: re_pattern_buffer,
    pub regs: re_registers,
    pub dfareg: *mut dfa,
    pub mre_pat: minrx_regex_t,
    pub mre_regs: *mut minrx_regmatch_t,
    pub has_meta: bool,
    pub maybe_long: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct awk_bool(pub ::core::ffi::c_uint);
impl awk_bool {
    pub const awk_false: Self = Self(0);
    pub const awk_true: Self = Self(1);
}
pub type awk_bool_t = awk_bool;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const GAWK_API_MAJOR_VERSION: Self = Self(4);
    pub const GAWK_API_MINOR_VERSION: Self = Self(1);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_string {
    pub str: *mut ::core::ffi::c_char,
    pub len: size_t,
}
pub type awk_string_t = awk_string;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct AWK_NUMBER_TYPE(pub ::core::ffi::c_uint);
impl AWK_NUMBER_TYPE {
    pub const AWK_NUMBER_TYPE_DOUBLE: Self = Self(0);
    pub const AWK_NUMBER_TYPE_MPFR: Self = Self(1);
    pub const AWK_NUMBER_TYPE_MPZ: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_number {
    pub d: ::core::ffi::c_double,
    pub r#type: AWK_NUMBER_TYPE,
    pub ptr: *mut ::core::ffi::c_void,
}
pub type awk_number_t = awk_number;
pub type awk_array_t = *mut ::core::ffi::c_void;
pub type awk_scalar_t = *mut ::core::ffi::c_void;
pub type awk_value_cookie_t = *mut ::core::ffi::c_void;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct awk_valtype_t(pub ::core::ffi::c_uint);
impl awk_valtype_t {
    pub const AWK_UNDEFINED: Self = Self(0);
    pub const AWK_NUMBER: Self = Self(1);
    pub const AWK_STRING: Self = Self(2);
    pub const AWK_REGEX: Self = Self(3);
    pub const AWK_STRNUM: Self = Self(4);
    pub const AWK_ARRAY: Self = Self(5);
    pub const AWK_SCALAR: Self = Self(6);
    pub const AWK_VALUE_COOKIE: Self = Self(7);
    pub const AWK_BOOL: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_value {
    pub val_type: awk_valtype_t,
    pub u: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub s: awk_string_t,
    pub n: awk_number_t,
    pub a: awk_array_t,
    pub scl: awk_scalar_t,
    pub vc: awk_value_cookie_t,
    pub b: awk_bool_t,
}
pub type awk_value_t = awk_value;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct awk_ext_func {
    pub name: *const ::core::ffi::c_char,
    pub function: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub max_expected_args: size_t,
    pub min_required_args: size_t,
    pub suppress_lint: awk_bool_t,
    pub data: *mut ::core::ffi::c_void,
}
pub type awk_ext_func_t = awk_ext_func;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct nodevals(pub ::core::ffi::c_uint);
impl nodevals {
    pub const Node_illegal: Self = Self(0);
    pub const Node_val: Self = Self(1);
    pub const Node_regex: Self = Self(2);
    pub const Node_dynregex: Self = Self(3);
    pub const Node_var: Self = Self(4);
    pub const Node_var_array: Self = Self(5);
    pub const Node_var_new: Self = Self(6);
    pub const Node_elem_new: Self = Self(7);
    pub const Node_param_list: Self = Self(8);
    pub const Node_func: Self = Self(9);
    pub const Node_ext_func: Self = Self(10);
    pub const Node_builtin_func: Self = Self(11);
    pub const Node_array_ref: Self = Self(12);
    pub const Node_array_tree: Self = Self(13);
    pub const Node_array_leaf: Self = Self(14);
    pub const Node_dump_array: Self = Self(15);
    pub const Node_arrayfor: Self = Self(16);
    pub const Node_frame: Self = Self(17);
    pub const Node_instruction: Self = Self(18);
    pub const Node_final: Self = Self(19);
}
pub type NODETYPE = nodevals;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_node {
    pub sub: C2Rust_Unnamed_2,
    pub r#type: NODETYPE,
    pub flags: flagvals,
    pub valref: ::core::ffi::c_long,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct flagvals(pub ::core::ffi::c_uint);
impl flagvals {
    pub const MALLOC: Self = Self(1);
    pub const STRING: Self = Self(2);
    pub const STRCUR: Self = Self(4);
    pub const NUMCUR: Self = Self(8);
    pub const NUMBER: Self = Self(16);
    pub const USER_INPUT: Self = Self(32);
    pub const BOOLVAL: Self = Self(64);
    pub const INTLSTR: Self = Self(128);
    pub const NUMINT: Self = Self(256);
    pub const INTIND: Self = Self(512);
    pub const WSTRCUR: Self = Self(1024);
    pub const MPFN: Self = Self(2048);
    pub const MPZN: Self = Self(4096);
    pub const NO_EXT_SET: Self = Self(8192);
    pub const NULL_FIELD: Self = Self(16384);
    pub const ARRAYMAXED: Self = Self(32768);
    pub const HALFHAT: Self = Self(65536);
    pub const XARRAY: Self = Self(131072);
    pub const NUMCONSTSTR: Self = Self(262144);
    pub const REGEX: Self = Self(524288);
    pub const CONVFMT_FMT: Self = Self(1048576);
    pub const OFMT_FMT: Self = Self(2097152);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub nodep: C2Rust_Unnamed_5,
    pub val: C2Rust_Unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub fltnum: ::core::ffi::c_double,
    pub for_alignment_only: ::core::ffi::c_int,
    pub sp: *mut ::core::ffi::c_char,
    pub slen: size_t,
    pub idx: ::core::ffi::c_int,
    pub z: C2Rust_Unnamed_4,
    pub wslen: size_t,
    pub typre: *mut exp_node,
    pub comtype: commenttype,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct commenttype(pub ::core::ffi::c_uint);
impl commenttype {
    pub const EOL_COMMENT: Self = Self(1);
    pub const BLOCK_COMMENT: Self = Self(2);
    pub const FOR_COMMENT: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub wsp: *mut char32_t,
    pub vn: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub l: C2Rust_Unnamed_12,
    pub r: C2Rust_Unnamed_7,
    pub x: C2Rust_Unnamed_6,
    pub name: *mut ::core::ffi::c_char,
    pub reserved: size_t,
    pub rn: *mut exp_node,
    pub cnt: ::core::ffi::c_ulong,
    pub reflags: reflagvals,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct reflagvals(pub ::core::ffi::c_uint);
impl reflagvals {
    pub const CONSTANT: Self = Self(1);
    pub const FS_DFLT: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub extra: *mut exp_node,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xl: ::core::ffi::c_long,
    pub cmnt: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub rptr: *mut exp_node,
    pub preg: [*mut Regexp; 2],
    pub av: *mut *mut exp_node,
    pub bv: *mut *mut BUCKET,
    pub uptr: Option<unsafe extern "C" fn() -> ()>,
    pub iptr: *mut exp_instruction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exp_instruction {
    pub nexti: *mut exp_instruction,
    pub d: C2Rust_Unnamed_9,
    pub x: C2Rust_Unnamed_8,
    pub comment: *mut exp_instruction,
    pub source_line: ::core::ffi::c_short,
    pub pool_size: ::core::ffi::c_short,
    pub opcode: OPCODE,
}
pub type OPCODE = opcodeval;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct opcodeval(pub ::core::ffi::c_uint);
impl opcodeval {
    pub const Op_illegal: Self = Self(0);
    pub const Op_times: Self = Self(1);
    pub const Op_times_i: Self = Self(2);
    pub const Op_quotient: Self = Self(3);
    pub const Op_quotient_i: Self = Self(4);
    pub const Op_mod: Self = Self(5);
    pub const Op_mod_i: Self = Self(6);
    pub const Op_plus: Self = Self(7);
    pub const Op_plus_i: Self = Self(8);
    pub const Op_minus: Self = Self(9);
    pub const Op_minus_i: Self = Self(10);
    pub const Op_exp: Self = Self(11);
    pub const Op_exp_i: Self = Self(12);
    pub const Op_concat: Self = Self(13);
    pub const Op_line_range: Self = Self(14);
    pub const Op_cond_pair: Self = Self(15);
    pub const Op_subscript: Self = Self(16);
    pub const Op_sub_array: Self = Self(17);
    pub const Op_preincrement: Self = Self(18);
    pub const Op_predecrement: Self = Self(19);
    pub const Op_postincrement: Self = Self(20);
    pub const Op_postdecrement: Self = Self(21);
    pub const Op_unary_minus: Self = Self(22);
    pub const Op_unary_plus: Self = Self(23);
    pub const Op_field_spec: Self = Self(24);
    pub const Op_not: Self = Self(25);
    pub const Op_assign: Self = Self(26);
    pub const Op_store_var: Self = Self(27);
    pub const Op_store_sub: Self = Self(28);
    pub const Op_store_field: Self = Self(29);
    pub const Op_store_field_exp: Self = Self(30);
    pub const Op_assign_times: Self = Self(31);
    pub const Op_assign_quotient: Self = Self(32);
    pub const Op_assign_mod: Self = Self(33);
    pub const Op_assign_plus: Self = Self(34);
    pub const Op_assign_minus: Self = Self(35);
    pub const Op_assign_exp: Self = Self(36);
    pub const Op_assign_concat: Self = Self(37);
    pub const Op_and: Self = Self(38);
    pub const Op_and_final: Self = Self(39);
    pub const Op_or: Self = Self(40);
    pub const Op_or_final: Self = Self(41);
    pub const Op_equal: Self = Self(42);
    pub const Op_notequal: Self = Self(43);
    pub const Op_less: Self = Self(44);
    pub const Op_greater: Self = Self(45);
    pub const Op_leq: Self = Self(46);
    pub const Op_geq: Self = Self(47);
    pub const Op_match: Self = Self(48);
    pub const Op_match_rec: Self = Self(49);
    pub const Op_nomatch: Self = Self(50);
    pub const Op_rule: Self = Self(51);
    pub const Op_K_case: Self = Self(52);
    pub const Op_K_default: Self = Self(53);
    pub const Op_K_break: Self = Self(54);
    pub const Op_K_continue: Self = Self(55);
    pub const Op_K_print: Self = Self(56);
    pub const Op_K_print_rec: Self = Self(57);
    pub const Op_K_printf: Self = Self(58);
    pub const Op_K_next: Self = Self(59);
    pub const Op_K_exit: Self = Self(60);
    pub const Op_K_return: Self = Self(61);
    pub const Op_K_return_from_eval: Self = Self(62);
    pub const Op_K_delete: Self = Self(63);
    pub const Op_K_delete_loop: Self = Self(64);
    pub const Op_K_getline_redir: Self = Self(65);
    pub const Op_K_getline: Self = Self(66);
    pub const Op_K_nextfile: Self = Self(67);
    pub const Op_K_namespace: Self = Self(68);
    pub const Op_builtin: Self = Self(69);
    pub const Op_sub_builtin: Self = Self(70);
    pub const Op_ext_builtin: Self = Self(71);
    pub const Op_in_array: Self = Self(72);
    pub const Op_func_call: Self = Self(73);
    pub const Op_indirect_func_call: Self = Self(74);
    pub const Op_push: Self = Self(75);
    pub const Op_push_arg: Self = Self(76);
    pub const Op_push_arg_untyped: Self = Self(77);
    pub const Op_push_i: Self = Self(78);
    pub const Op_push_re: Self = Self(79);
    pub const Op_push_array: Self = Self(80);
    pub const Op_push_param: Self = Self(81);
    pub const Op_push_lhs: Self = Self(82);
    pub const Op_subscript_lhs: Self = Self(83);
    pub const Op_field_spec_lhs: Self = Self(84);
    pub const Op_no_op: Self = Self(85);
    pub const Op_pop: Self = Self(86);
    pub const Op_jmp: Self = Self(87);
    pub const Op_jmp_true: Self = Self(88);
    pub const Op_jmp_false: Self = Self(89);
    pub const Op_get_record: Self = Self(90);
    pub const Op_newfile: Self = Self(91);
    pub const Op_arrayfor_init: Self = Self(92);
    pub const Op_arrayfor_incr: Self = Self(93);
    pub const Op_arrayfor_final: Self = Self(94);
    pub const Op_var_update: Self = Self(95);
    pub const Op_var_assign: Self = Self(96);
    pub const Op_field_assign: Self = Self(97);
    pub const Op_subscript_assign: Self = Self(98);
    pub const Op_after_beginfile: Self = Self(99);
    pub const Op_after_endfile: Self = Self(100);
    pub const Op_func: Self = Self(101);
    pub const Op_comment: Self = Self(102);
    pub const Op_exec_count: Self = Self(103);
    pub const Op_breakpoint: Self = Self(104);
    pub const Op_lint: Self = Self(105);
    pub const Op_atexit: Self = Self(106);
    pub const Op_stop: Self = Self(107);
    pub const Op_token: Self = Self(108);
    pub const Op_symbol: Self = Self(109);
    pub const Op_list: Self = Self(110);
    pub const Op_K_do: Self = Self(111);
    pub const Op_K_for: Self = Self(112);
    pub const Op_K_arrayfor: Self = Self(113);
    pub const Op_K_while: Self = Self(114);
    pub const Op_K_switch: Self = Self(115);
    pub const Op_K_if: Self = Self(116);
    pub const Op_K_else: Self = Self(117);
    pub const Op_K_function: Self = Self(118);
    pub const Op_cond_exp: Self = Self(119);
    pub const Op_parens: Self = Self(120);
    pub const Op_final: Self = Self(121);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub xl: ::core::ffi::c_long,
    pub xn: *mut NODE,
    pub aptr: Option<unsafe extern "C" fn() -> ()>,
    pub xi: *mut exp_instruction,
    pub bpt: *mut break_point,
    pub exf: *mut awk_ext_func_t,
}
pub type NODE = exp_node;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub dn: *mut NODE,
    pub di: *mut exp_instruction,
    pub fptr: Option<unsafe extern "C" fn(::core::ffi::c_int) -> *mut NODE>,
    pub efptr: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            *mut awk_value_t,
            *mut awk_ext_func,
        ) -> *mut awk_value_t,
    >,
    pub dl: ::core::ffi::c_long,
    pub ldl: exec_count_t,
    pub name: *mut ::core::ffi::c_char,
}
pub type exec_count_t = ::core::ffi::c_ulonglong;
pub type BUCKET = bucket_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub union bucket_item {
    pub hs: C2Rust_Unnamed_11,
    pub hi: C2Rust_Unnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub next: *mut bucket_item,
    pub li: [::core::ffi::c_long; 2],
    pub val: [*mut exp_node; 2],
    pub cnt: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub next: *mut bucket_item,
    pub str: *mut ::core::ffi::c_char,
    pub len: size_t,
    pub code: size_t,
    pub name: *mut exp_node,
    pub val: *mut exp_node,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub lptr: *mut exp_node,
    pub li: *mut exp_instruction,
    pub ll: ::core::ffi::c_long,
    pub lp: *const array_funcs_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct array_funcs_t {
    pub name: *const ::core::ffi::c_char,
    pub init: afunc_t,
    pub type_of: afunc_t,
    pub lookup: afunc_t,
    pub exists: afunc_t,
    pub clear: afunc_t,
    pub remove: afunc_t,
    pub list: afunc_t,
    pub copy: afunc_t,
    pub dump: afunc_t,
    pub store: afunc_t,
}
pub type afunc_t = Option<unsafe extern "C" fn(*mut exp_node, *mut exp_node) -> *mut *mut exp_node>;
pub type INSTRUCTION = exp_instruction;
pub type Func_ptr = Option<unsafe extern "C" fn() -> ()>;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct binmode_values(pub ::core::ffi::c_uint);
impl binmode_values {
    pub const TEXT_TRANSLATE: Self = Self(0);
    pub const BINMODE_INPUT: Self = Self(1);
    pub const BINMODE_OUTPUT: Self = Self(2);
    pub const BINMODE_BOTH: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct srcfile {
    pub next: *mut srcfile,
    pub prev: *mut srcfile,
    pub stype: srctype,
    pub src: *mut ::core::ffi::c_char,
    pub fullpath: *mut ::core::ffi::c_char,
    pub mtime: time_t,
    pub sbuf: stat,
    pub srclines: ::core::ffi::c_int,
    pub bufsize: size_t,
    pub buf: *mut ::core::ffi::c_char,
    pub line_offset: *mut ::core::ffi::c_int,
    pub fd: ::core::ffi::c_int,
    pub maxlen: ::core::ffi::c_int,
    pub fini_func: Option<unsafe extern "C" fn() -> ()>,
    pub lexptr: *mut ::core::ffi::c_char,
    pub lexend: *mut ::core::ffi::c_char,
    pub lexeme: *mut ::core::ffi::c_char,
    pub lexptr_begin: *mut ::core::ffi::c_char,
    pub lasttok: ::core::ffi::c_int,
    pub comment: *mut INSTRUCTION,
    pub namespace: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct srctype(pub ::core::ffi::c_uint);
impl srctype {
    pub const SRC_CMDLINE: Self = Self(1);
    pub const SRC_STDIN: Self = Self(2);
    pub const SRC_FILE: Self = Self(3);
    pub const SRC_INC: Self = Self(4);
    pub const SRC_NSINC: Self = Self(5);
    pub const SRC_EXTLIB: Self = Self(6);
}
pub type SRCFILE = srcfile;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_pool {
    pub pool: [instruction_mem_pool; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct instruction_mem_pool {
    pub block_list: *mut instruction_block,
    pub free_space: *mut INSTRUCTION,
    pub free_list: *mut INSTRUCTION,
}
pub type INSTRUCTION_POOL = instruction_pool;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct context {
    pub pools: INSTRUCTION_POOL,
    pub symbols: NODE,
    pub rule_list: INSTRUCTION,
    pub srcfiles: SRCFILE,
    pub sourceline: ::core::ffi::c_int,
    pub source: *mut ::core::ffi::c_char,
    pub install_func: Option<unsafe extern "C" fn(*mut NODE) -> ()>,
    pub prev: *mut context,
}
pub type AWK_CONTEXT = context;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct block_id(pub ::core::ffi::c_uint);
impl block_id {
    pub const BLOCK_NODE: Self = Self(0);
    pub const BLOCK_BUCKET: Self = Self(1);
    pub const BLOCK_MAX: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct do_flag_values(pub ::core::ffi::c_uint);
impl do_flag_values {
    pub const DO_FLAG_NONE: Self = Self(0);
    pub const DO_LINT_INVALID: Self = Self(1);
    pub const DO_LINT_EXTENSIONS: Self = Self(2);
    pub const DO_LINT_ALL: Self = Self(4);
    pub const DO_LINT_OLD: Self = Self(8);
    pub const DO_TRADITIONAL: Self = Self(16);
    pub const DO_POSIX: Self = Self(32);
    pub const DO_INTL: Self = Self(64);
    pub const DO_NON_DEC_DATA: Self = Self(128);
    pub const DO_INTERVALS: Self = Self(256);
    pub const DO_PRETTY_PRINT: Self = Self(512);
    pub const DO_DUMP_VARS: Self = Self(1024);
    pub const DO_TIDY_MEM: Self = Self(2048);
    pub const DO_SANDBOX: Self = Self(4096);
    pub const DO_PROFILE: Self = Self(8192);
    pub const DO_DEBUG: Self = Self(16384);
    pub const DO_MPFR: Self = Self(32768);
    pub const DO_CSV: Self = Self(65536);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct varinit {
    pub spec: *mut *mut NODE,
    pub name: *const ::core::ffi::c_char,
    pub strval: *const ::core::ffi::c_char,
    pub numval: ::core::ffi::c_double,
    pub update: Func_ptr,
    pub assign: Func_ptr,
    pub do_assign: bool,
    pub flags: ::core::ffi::c_int,
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
pub struct pre_assign {
    pub r#type: assign_type,
    pub val: *mut ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct assign_type(pub ::core::ffi::c_uint);
impl assign_type {
    pub const PRE_ASSIGN: Self = Self(1);
    pub const PRE_ASSIGN_FS: Self = Self(2);
}
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const __ASSERT_FUNCTION: [::core::ffi::c_char; 19] = unsafe {
    ::core::mem::transmute::<[u8; 19], [::core::ffi::c_char; 19]>(*b"void unref(NODE *)\0")
};
pub const LC_CTYPE: ::core::ffi::c_int = __LC_CTYPE;
pub const LC_NUMERIC: ::core::ffi::c_int = __LC_NUMERIC;
pub const LC_TIME: ::core::ffi::c_int = __LC_TIME;
pub const LC_COLLATE: ::core::ffi::c_int = __LC_COLLATE;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
pub const O_BINARY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SCAN: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_FATAL: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn unref(mut r: *mut NODE) {
    '_c2rust_label: {
        if r.is_null() || (*r).valref > 0 as ::core::ffi::c_long {
        } else {
            __assert_fail(
                b"r == NULL || r->valref > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
                2076 as ::core::ffi::c_uint,
                __ASSERT_FUNCTION.as_ptr(),
            );
        }
    };
    if !r.is_null() && {
        (*r).valref -= 1;
        (*r).valref <= 0 as ::core::ffi::c_long
    } {
        r_unref(r);
    }
}
#[inline]
unsafe extern "C" fn force_number(mut n: *mut NODE) -> *mut NODE {
    if (*n).r#type.0 == nodevals::Node_elem_new.0 {
        elem_new_reset(n);
        (*n).r#type = nodevals::Node_val;
        return n;
    }
    return if (*n).flags.0 & flagvals::NUMCUR.0 != 0 as ::core::ffi::c_uint {
        n
    } else {
        str2number.expect("non-null function pointer")(n)
    };
}
#[inline]
unsafe extern "C" fn emalloc_real(
    mut count: size_t,
    mut r#where: *const ::core::ffi::c_char,
    mut var: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if count == 0 as size_t {
        set_loc(
            b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
            2154 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            b"%s:%d: emalloc called with zero bytes\0".as_ptr() as *const ::core::ffi::c_char,
            file,
            line,
        );
    }
    ret = pma_malloc(count);
    if ret.is_null() {
        set_loc(
            b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
            2158 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s:%d:%s: %s: cannot allocate %zu bytes of memory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            file,
            line,
            r#where,
            var,
            count,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn ezalloc_real(
    mut count: size_t,
    mut r#where: *const ::core::ffi::c_char,
    mut var: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if count == 0 as size_t {
        set_loc(
            b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
            2172 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            b"%s:%d: ezalloc called with zero bytes\0".as_ptr() as *const ::core::ffi::c_char,
            file,
            line,
        );
    }
    ret = pma_calloc(1 as size_t, count);
    if ret.is_null() {
        set_loc(
            b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
            2176 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s:%d:%s: %s: cannot allocate %zu bytes of memory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            file,
            line,
            r#where,
            var,
            count,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn erealloc_real(
    mut ptr: *mut ::core::ffi::c_void,
    mut count: size_t,
    mut r#where: *const ::core::ffi::c_char,
    mut var: *const ::core::ffi::c_char,
    mut file: *const ::core::ffi::c_char,
    mut line: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut ret: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    if count == 0 as size_t {
        set_loc(
            b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
            2190 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            b"%s:%d: erealloc called with zero bytes\0".as_ptr() as *const ::core::ffi::c_char,
            file,
            line,
        );
    }
    ret = pma_realloc(ptr, count);
    if ret.is_null() {
        set_loc(
            b"/opt/src/gawk-5.4.1/awk.h\0".as_ptr() as *const ::core::ffi::c_char,
            2194 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s:%d:%s: %s: cannot reallocate %zu bytes of memory: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            file,
            line,
            r#where,
            var,
            count,
            strerror(*__errno_location()),
        );
    }
    return ret;
}
#[inline]
unsafe extern "C" fn assoc_set(mut array: *mut NODE, mut sub: *mut NODE, mut value: *mut NODE) {
    let mut lhs: *mut *mut NODE = (*(*array).sub.nodep.l.lp)
        .lookup
        .expect("non-null function pointer")(array, sub);
    unref(*lhs);
    *lhs = value;
    if (*(*array).sub.nodep.l.lp).store.is_some() {
        Some(
            (*(*array).sub.nodep.l.lp)
                .store
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(array, sub);
    }
    unref(sub);
}
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const optional_argument: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PACKAGE: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"gawk\0") };
pub const PACKAGE_VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"5.4.1\0") };
pub const VERSION: [::core::ffi::c_char; 6] =
    unsafe { ::core::mem::transmute::<[u8; 6], [::core::ffi::c_char; 6]>(*b"5.4.1\0") };
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const UPDATE_YEAR: ::core::ffi::c_int = 2026 as ::core::ffi::c_int;
pub const DEFAULT_PROFILE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"awkprof.out\0") };
pub const DEFAULT_VARFILE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"awkvars.out\0") };
pub const DEFAULT_PREC: ::core::ffi::c_int = 53 as ::core::ffi::c_int;
pub const DEFAULT_ROUNDMODE: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"N\0") };
static mut varfile: *const ::core::ffi::c_char = DEFAULT_VARFILE.as_ptr();
#[export_name = "rboxc_gawk_command_file"]
pub static mut command_file: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_CONVFMT_node"]
pub static mut CONVFMT_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_ARGC_node"]
pub static mut ARGC_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_BINMODE_node"]
pub static mut BINMODE_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_ARGV_node"]
pub static mut ARGV_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_ARGIND_node"]
pub static mut ARGIND_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
static mut ENVIRON_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_ERRNO_node"]
pub static mut ERRNO_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_FILENAME_node"]
pub static mut FILENAME_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_FIELDWIDTHS_node"]
pub static mut FIELDWIDTHS_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_FNR_node"]
pub static mut FNR_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_FPAT_node"]
pub static mut FPAT_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_LINT_node"]
pub static mut LINT_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_IGNORECASE_node"]
pub static mut IGNORECASE_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_FS_node"]
pub static mut FS_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_OFS_node"]
pub static mut OFS_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_ORS_node"]
pub static mut ORS_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_PROCINFO_node"]
pub static mut PROCINFO_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_NR_node"]
pub static mut NR_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_OFMT_node"]
pub static mut OFMT_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_NF_node"]
pub static mut NF_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_RT_node"]
pub static mut RT_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_RSTART_node"]
pub static mut RSTART_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_RLENGTH_node"]
pub static mut RLENGTH_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_RS_node"]
pub static mut RS_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_SUBSEP_node"]
pub static mut SUBSEP_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_ROUNDMODE_node"]
pub static mut ROUNDMODE_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_PREC_node"]
pub static mut PREC_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_TEXTDOMAIN_node"]
pub static mut TEXTDOMAIN_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_NF"]
pub static mut NF: ::core::ffi::c_long = 0;
#[export_name = "rboxc_gawk_NR"]
pub static mut NR: ::core::ffi::c_long = 0;
#[export_name = "rboxc_gawk_FNR"]
pub static mut FNR: ::core::ffi::c_long = 0;
#[export_name = "rboxc_gawk_BINMODE"]
pub static mut BINMODE: ::core::ffi::c_int = 0;
#[export_name = "rboxc_gawk_IGNORECASE"]
pub static mut IGNORECASE: bool = false;
#[export_name = "rboxc_gawk_OFS"]
pub static mut OFS: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_ORS"]
pub static mut ORS: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_OFMT"]
pub static mut OFMT: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_TEXTDOMAIN"]
pub static mut TEXTDOMAIN: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_CONVFMT"]
pub static mut CONVFMT: *const ::core::ffi::c_char =
    b"%.6g\0".as_ptr() as *const ::core::ffi::c_char;
#[export_name = "rboxc_gawk_Nnull_string"]
pub static mut Nnull_string: *mut NODE = ::core::ptr::null_mut::<NODE>();
#[export_name = "rboxc_gawk_loc"]
pub static mut loc: lconv = lconv {
    decimal_point: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    thousands_sep: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    grouping: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    int_curr_symbol: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    currency_symbol: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mon_decimal_point: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mon_thousands_sep: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    mon_grouping: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    positive_sign: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    negative_sign: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    int_frac_digits: 0,
    frac_digits: 0,
    p_cs_precedes: 0,
    p_sep_by_space: 0,
    n_cs_precedes: 0,
    n_sep_by_space: 0,
    p_sign_posn: 0,
    n_sign_posn: 0,
    int_p_cs_precedes: 0,
    int_p_sep_by_space: 0,
    int_n_cs_precedes: 0,
    int_n_sep_by_space: 0,
    int_p_sign_posn: 0,
    int_n_sign_posn: 0,
};
#[export_name = "rboxc_gawk_myname"]
pub static mut myname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_code_block"]
pub static mut code_block: *mut INSTRUCTION = ::core::ptr::null_mut::<INSTRUCTION>();
#[export_name = "rboxc_gawk_d_argv"]
pub static mut d_argv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
#[export_name = "rboxc_gawk_rule_list"]
pub static mut rule_list: *mut INSTRUCTION = ::core::ptr::null_mut::<INSTRUCTION>();
#[export_name = "rboxc_gawk_exit_val"]
pub static mut exit_val: ::core::ffi::c_int = EXIT_SUCCESS;
#[export_name = "rboxc_gawk_srcfiles"]
pub static mut srcfiles: *mut SRCFILE = ::core::ptr::null_mut::<SRCFILE>();
static mut preassigns: *mut pre_assign = ::core::ptr::null_mut::<pre_assign>();
static mut numassigns: ::core::ffi::c_long = -1 as ::core::ffi::c_long;
static mut disallow_var_assigns: bool = r#false != 0;
static mut stopped_early: bool = r#false != 0;
#[export_name = "rboxc_gawk_using_persistent_malloc"]
pub static mut using_persistent_malloc: bool = r#false != 0;
#[export_name = "rboxc_gawk_persist_file"]
pub static mut persist_file: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_gawk_do_flags"]
pub static mut do_flags: do_flag_values = do_flag_values::DO_FLAG_NONE;
#[export_name = "rboxc_gawk_do_itrace"]
pub static mut do_itrace: bool = r#false != 0;
#[export_name = "rboxc_gawk_do_optimize"]
pub static mut do_optimize: bool = r#true != 0;
static mut do_binary: ::core::ffi::c_int = r#false;
static mut do_version: ::core::ffi::c_int = r#false;
static mut locale: *const ::core::ffi::c_char = b"\0".as_ptr() as *const ::core::ffi::c_char;
static mut locale_dir: *const ::core::ffi::c_char = LOCALEDIR.as_ptr();
#[export_name = "rboxc_gawk_use_gnu_matchers"]
pub static mut use_gnu_matchers: bool = r#false != 0;
#[export_name = "rboxc_gawk_use_lc_numeric"]
pub static mut use_lc_numeric: ::core::ffi::c_int = r#false;
#[export_name = "rboxc_gawk_gawk_mb_cur_max"]
pub static mut gawk_mb_cur_max: ::core::ffi::c_int = 0;
#[export_name = "rboxc_gawk_output_fp"]
pub static mut output_fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
#[export_name = "rboxc_gawk_output_is_tty"]
pub static mut output_is_tty: bool = r#false != 0;
#[export_name = "rboxc_gawk_def_strftime_format"]
pub static mut def_strftime_format: [::core::ffi::c_char; 24] = unsafe {
    ::core::mem::transmute::<[u8; 24], [::core::ffi::c_char; 24]>(*b"%a %b %e %H:%M:%S %Z %Y\0")
};
#[export_name = "rboxc_gawk_groupset"]
pub static mut groupset: *mut gid_t = ::core::ptr::null_mut::<gid_t>();
#[export_name = "rboxc_gawk_ngroups"]
pub static mut ngroups: ::core::ffi::c_int = 0;
#[export_name = "rboxc_gawk_lintfunc"]
pub static mut lintfunc: Option<unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ()> =
    Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ());
static mut optab: [option; 31] = unsafe {
    [
        option {
            name: b"assign\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'v' as ::core::ffi::c_int,
        },
        option {
            name: b"bignum\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'M' as ::core::ffi::c_int,
        },
        option {
            name: b"characters-as-bytes\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: &raw const do_binary as *mut ::core::ffi::c_int,
            val: 'b' as ::core::ffi::c_int,
        },
        option {
            name: b"copyright\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'C' as ::core::ffi::c_int,
        },
        option {
            name: b"csv\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'k' as ::core::ffi::c_int,
        },
        option {
            name: b"debug\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'D' as ::core::ffi::c_int,
        },
        option {
            name: b"dump-variables\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'd' as ::core::ffi::c_int,
        },
        option {
            name: b"exec\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'E' as ::core::ffi::c_int,
        },
        option {
            name: b"field-separator\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'F' as ::core::ffi::c_int,
        },
        option {
            name: b"file\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'f' as ::core::ffi::c_int,
        },
        option {
            name: b"gen-pot\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'g' as ::core::ffi::c_int,
        },
        option {
            name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'h' as ::core::ffi::c_int,
        },
        option {
            name: b"include\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'i' as ::core::ffi::c_int,
        },
        option {
            name: b"lint\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'L' as ::core::ffi::c_int,
        },
        option {
            name: b"lint-old\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 't' as ::core::ffi::c_int,
        },
        option {
            name: b"load\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'l' as ::core::ffi::c_int,
        },
        option {
            name: b"non-decimal-data\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'n' as ::core::ffi::c_int,
        },
        option {
            name: b"no-optimize\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 's' as ::core::ffi::c_int,
        },
        option {
            name: b"optimize\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'O' as ::core::ffi::c_int,
        },
        option {
            name: b"persist\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'T' as ::core::ffi::c_int,
        },
        option {
            name: b"posix\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'P' as ::core::ffi::c_int,
        },
        option {
            name: b"pretty-print\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'o' as ::core::ffi::c_int,
        },
        option {
            name: b"profile\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: optional_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'p' as ::core::ffi::c_int,
        },
        option {
            name: b"re-interval\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'r' as ::core::ffi::c_int,
        },
        option {
            name: b"sandbox\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'S' as ::core::ffi::c_int,
        },
        option {
            name: b"source\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: required_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'e' as ::core::ffi::c_int,
        },
        option {
            name: b"trace\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'I' as ::core::ffi::c_int,
        },
        option {
            name: b"traditional\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: 'c' as ::core::ffi::c_int,
        },
        option {
            name: b"use-lc-numeric\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: &raw const use_lc_numeric as *mut ::core::ffi::c_int,
            val: 1 as ::core::ffi::c_int,
        },
        option {
            name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
            has_arg: no_argument,
            flag: &raw const do_version as *mut ::core::ffi::c_int,
            val: 'V' as ::core::ffi::c_int,
        },
        option {
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            has_arg: 0 as ::core::ffi::c_int,
            flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
            val: '\0' as ::core::ffi::c_int,
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_gawk(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut i: ::core::ffi::c_int = 0;
    let mut have_srcfile: bool = r#false != 0;
    let mut s: *mut SRCFILE = ::core::ptr::null_mut::<SRCFILE>();
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    myname = gawk_name(*argv.offset(0isize));
    using_persistent_malloc = enable_pma(argv);
    if !getenv(b"TIDYMEM\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_TIDY_MEM.0);
    }
    if !using_persistent_malloc && do_flags.0 & do_flag_values::DO_TIDY_MEM.0 != 0 {
        mtrace();
    }
    os_arg_fixup(&raw mut argc, &raw mut argv);
    if argc < 2 as ::core::ffi::c_int {
        usage(EXIT_FAILURE, stderr);
    }
    cp = getenv(b"GAWK_LOCALE_DIR\0".as_ptr() as *const ::core::ffi::c_char);
    if !cp.is_null() {
        locale_dir = cp;
    }
    set_locale_stuff();
    signal(
        SIGSEGV,
        Some(catchsig as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGFPE,
        Some(catchsig as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGBUS,
        Some(catchsig as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGPIPE,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    Nnull_string = make_str_node(
        b"\0".as_ptr() as *const ::core::ffi::c_char,
        0 as size_t,
        0 as ::core::ffi::c_int,
    );
    init_fds();
    array_init();
    init_symbol_table();
    output_fp = stdout;
    push_context(new_context());
    parse_args(argc, argv);
    if !getenv(b"GAWK_GNU_MATCHERS\0".as_ptr() as *const ::core::ffi::c_char).is_null() {
        use_gnu_matchers = r#true != 0;
    }
    gawk_mb_cur_max = __ctype_get_mb_cur_max() as ::core::ffi::c_int;
    init_btowc_cache();
    if gawk_mb_cur_max == 1 as ::core::ffi::c_int {
        load_casetable();
    }
    if do_flags.0 & do_flag_values::DO_POSIX.0 == 0
        && !getenv(b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char).is_null()
    {
        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_POSIX.0);
        if do_flags.0
            & (do_flag_values::DO_LINT_INVALID.0 as ::core::ffi::c_int
                | do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != 0
        {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                326 as ::core::ffi::c_int,
            );
            Some(lintfunc.expect("non-null function pointer")).expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"environment variable `POSIXLY_CORRECT' set: turning on `--posix'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        }
    }
    if do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
        use_lc_numeric = r#true;
        if do_flags.0 & do_flag_values::DO_TRADITIONAL.0 != 0 {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                334 as ::core::ffi::c_int,
            );
            Some(
                Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"`--posix' overrides `--traditional'\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ));
        } else {
            do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_TRADITIONAL.0);
        }
    }
    if do_flags.0 & do_flag_values::DO_TRADITIONAL.0 != 0
        && do_flags.0 & do_flag_values::DO_NON_DEC_DATA.0 != 0
    {
        do_flags = do_flag_values(
            do_flags.0
                & !(do_flag_values::DO_NON_DEC_DATA.0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        );
        set_loc(
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            345 as ::core::ffi::c_int,
        );
        Some(
            Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"`--posix'/`--traditional' overrides `--non-decimal-data'\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
    if do_binary != 0 {
        if do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                350 as ::core::ffi::c_int,
            );
            Some(
                Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"`--posix' overrides `--characters-as-bytes'\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ));
        } else {
            gawk_mb_cur_max = 1 as ::core::ffi::c_int;
            setlocale(LC_ALL, b"C\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    if do_flags.0 & do_flag_values::DO_CSV.0 != 0 && do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
        set_loc(
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            363 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"`--posix' and `--csv' conflict\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    }
    if do_flags.0
        & (do_flag_values::DO_LINT_INVALID.0 as ::core::ffi::c_int
            | do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        if os_is_setuid() != 0 {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                367 as ::core::ffi::c_int,
            );
            Some(lintfunc.expect("non-null function pointer")).expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"running %s setuid root may be a security problem\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                myname,
            );
        }
        if do_flags.0 & do_flag_values::DO_INTERVALS.0 != 0 {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                369 as ::core::ffi::c_int,
            );
            Some(lintfunc.expect("non-null function pointer")).expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"The -r/--re-interval options no longer have any effect\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        }
    }
    if do_flags.0 & do_flag_values::DO_DEBUG.0 != 0 {
        init_debug();
    }
    init_groupset();
    (*Nnull_string).sub.val.fltnum = 0.0f64;
    (*Nnull_string).flags = flagvals(
        (flagvals::MALLOC.0 as ::core::ffi::c_int
            | flagvals::STRCUR.0 as ::core::ffi::c_int
            | flagvals::STRING.0 as ::core::ffi::c_int
            | flagvals::NUMCUR.0 as ::core::ffi::c_int
            | flagvals::NUMBER.0 as ::core::ffi::c_int) as ::core::ffi::c_uint,
    );
    resetup();
    init_vars();
    init_csv_records();
    init_csv_fields();
    init_fields();
    let mut dash_v_errs: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i as ::core::ffi::c_long <= numassigns {
        if (*preassigns.offset(i as isize)).r#type.0 == assign_type::PRE_ASSIGN.0 {
            dash_v_errs += (arg_assign((*preassigns.offset(i as isize)).val, r#true != 0)
                == r#false) as ::core::ffi::c_int;
        } else {
            cmdline_fs((*preassigns.offset(i as isize)).val);
        }
        pma_free((*preassigns.offset(i as isize)).val as *mut ::core::ffi::c_void);
        i += 1;
    }
    if !preassigns.is_null() {
        pma_free(preassigns as *mut ::core::ffi::c_void);
    }
    if BINMODE & binmode_values::BINMODE_INPUT.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if os_setbinmode(fileno(stdin), O_BINARY) == -1 as ::core::ffi::c_int {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                427 as ::core::ffi::c_int,
            );
            Some(
                Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"cannot set binary mode on stdin: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                strerror(*__errno_location()),
            );
        }
    }
    if BINMODE & binmode_values::BINMODE_OUTPUT.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        if os_setbinmode(fileno(stdout), O_BINARY) == -1 as ::core::ffi::c_int {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                430 as ::core::ffi::c_int,
            );
            Some(
                Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"cannot set binary mode on stdout: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                strerror(*__errno_location()),
            );
        }
        if os_setbinmode(fileno(stderr), O_BINARY) == -1 as ::core::ffi::c_int {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                432 as ::core::ffi::c_int,
            );
            Some(
                Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"cannot set binary mode on stderr: %s\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                strerror(*__errno_location()),
            );
        }
    }
    if os_isatty(fileno(stdout)) != 0 {
        output_is_tty = r#true != 0;
    }
    atexit(Some(pma_save_free_lists as unsafe extern "C" fn() -> ()));
    init_ext_api();
    s = (*srcfiles).next as *mut SRCFILE;
    while s != srcfiles {
        if (*s).stype.0 == srctype::SRC_EXTLIB.0 {
            load_ext((*s).src, (*s).fullpath);
        } else if (*s).stype.0 != srctype::SRC_INC.0 && (*s).stype.0 != srctype::SRC_NSINC.0 {
            have_srcfile = r#true != 0;
        }
        s = (*s).next as *mut SRCFILE;
    }
    if do_version != 0 {
        version();
    }
    if !have_srcfile {
        if optind > argc - 1 as ::core::ffi::c_int || stopped_early as ::core::ffi::c_int != 0 {
            usage(EXIT_FAILURE, stderr);
        }
        add_srcfile(
            srctype::SRC_CMDLINE,
            *argv.offset(optind as isize),
            srcfiles,
            ::core::ptr::null_mut::<bool>(),
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        optind += 1;
    }
    init_interpret();
    init_args(
        optind,
        argc,
        if do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
            *argv.offset(0isize) as *const ::core::ffi::c_char
        } else {
            myname
        },
        argv,
    );
    setlocale(LC_NUMERIC, b"C\0".as_ptr() as *const ::core::ffi::c_char);
    if parse_program(&raw mut code_block, r#false != 0) != 0 as ::core::ffi::c_int
        || dash_v_errs > 0 as ::core::ffi::c_int
    {
        exit(EXIT_FAILURE);
    }
    if do_flags.0 & do_flag_values::DO_INTL.0 != 0 {
        exit(EXIT_SUCCESS);
    }
    set_current_namespace(&raw const awk_namespace as *const ::core::ffi::c_char);
    install_builtins();
    if do_flags.0
        & (do_flag_values::DO_LINT_INVALID.0 as ::core::ffi::c_int
            | do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
    {
        shadow_funcs();
    }
    if do_flags.0
        & (do_flag_values::DO_LINT_INVALID.0 as ::core::ffi::c_int
            | do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int) as ::core::ffi::c_uint
        != 0
        && (*(*code_block).nexti).opcode.0 == opcodeval::Op_atexit.0
    {
        set_loc(
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            497 as ::core::ffi::c_int,
        );
        Some(lintfunc.expect("non-null function pointer")).expect("non-null function pointer")(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"no program text at all!\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
    }
    load_symbols();
    if do_flags.0 & do_flag_values::DO_PROFILE.0 != 0 {
        init_profiling_signals();
    }
    if use_lc_numeric != 0 {
        setlocale(LC_NUMERIC, locale);
    }
    init_io();
    output_fp = stdout;
    if do_flags.0 & do_flag_values::DO_DEBUG.0 != 0 {
        debug_prog(code_block);
    } else if !(do_flags.0 & do_flag_values::DO_PRETTY_PRINT.0 != 0
        && do_flags.0 & do_flag_values::DO_PROFILE.0 == 0)
    {
        interpret.expect("non-null function pointer")(code_block);
    }
    if do_flags.0 & do_flag_values::DO_PRETTY_PRINT.0 != 0 {
        set_current_namespace(&raw const awk_namespace as *const ::core::ffi::c_char);
        dump_prog(code_block);
        dump_funcs();
        close_prof_file();
    }
    if do_flags.0 & do_flag_values::DO_DUMP_VARS.0 != 0 {
        dump_vars(varfile);
    }
    if do_flags.0 & do_flag_values::DO_TIDY_MEM.0 != 0 {
        release_all_vars();
    }
    final_exit(exit_val);
}
unsafe extern "C" fn add_preassign(mut r#type: assign_type, mut val: *mut ::core::ffi::c_char) {
    static mut alloc_assigns: ::core::ffi::c_long = 0;
    numassigns += 1;
    if preassigns.is_null() {
        preassigns = emalloc_real(
            4usize.wrapping_mul(::core::mem::size_of::<pre_assign>()),
            b"add_preassign\0".as_ptr() as *const ::core::ffi::c_char,
            b"preassigns\0".as_ptr() as *const ::core::ffi::c_char,
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            568 as ::core::ffi::c_int,
        ) as *mut pre_assign;
        alloc_assigns = INIT_SRC as ::core::ffi::c_long;
    } else if numassigns >= alloc_assigns {
        alloc_assigns *= 2 as ::core::ffi::c_long;
        preassigns = erealloc_real(
            preassigns as *mut ::core::ffi::c_void,
            (alloc_assigns as usize).wrapping_mul(::core::mem::size_of::<pre_assign>()),
            b"add_preassign\0".as_ptr() as *const ::core::ffi::c_char,
            b"preassigns\0".as_ptr() as *const ::core::ffi::c_char,
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            572 as ::core::ffi::c_int,
        ) as *mut pre_assign;
    }
    (*preassigns.offset(numassigns as isize)).r#type = r#type as assign_type;
    (*preassigns.offset(numassigns as isize)).val = estrdup(val, strlen(val));
}
pub const INIT_SRC: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn usage(mut exitval: ::core::ffi::c_int, mut fp: *mut FILE) -> ! {
    static mut gnu_url: [::core::ffi::c_char; 30] = unsafe {
        ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
            *b"https://ftp.gnu.org/gnu/gawk/\0",
        )
    };
    static mut beta_url: [::core::ffi::c_char; 29] = unsafe {
        ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
            *b"https://www.skeeve.com/gawk/\0",
        )
    };
    let mut major_version: ::core::ffi::c_int = 0;
    let mut minor_version: ::core::ffi::c_int = 0;
    let mut patchlevel: ::core::ffi::c_int = 0;
    patchlevel = 0 as ::core::ffi::c_int;
    minor_version = patchlevel;
    major_version = minor_version;
    sscanf(
        PACKAGE_VERSION.as_ptr(),
        b"%d.%d.%d\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut major_version,
        &raw mut minor_version,
        &raw mut patchlevel,
    );
    fprintf(
        fp,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [POSIX or GNU style options] -f progfile [--] file ...\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        myname,
    );
    fprintf(
        fp,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [POSIX or GNU style options] [--] %cprogram%c file ...\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        myname,
        quote as ::core::ffi::c_int,
        quote as ::core::ffi::c_int,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"POSIX options:\t\tGNU long options: (standard)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-f progfile\t\t--file=progfile\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-F fs\t\t\t--field-separator=fs\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-v var=val\t\t--assign=var=val\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Short options:\t\tGNU long options: (extensions)\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-b\t\t\t--characters-as-bytes\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-c\t\t\t--traditional\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-C\t\t\t--copyright\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-d[file]\t\t--dump-variables[=file]\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-D[file]\t\t--debug[=file]\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-e 'program-text'\t--source='program-text'\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-E file\t\t\t--exec=file\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-g\t\t\t--gen-pot\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-h\t\t\t--help\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-i includefile\t\t--include=includefile\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-I\t\t\t--trace\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-k\t\t\t--csv\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-l library\t\t--load=library\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-L[fatal|invalid|no-ext]\t--lint[=fatal|invalid|no-ext]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-M\t\t\t--bignum\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-N\t\t\t--use-lc-numeric\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-n\t\t\t--non-decimal-data\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-o[file]\t\t--pretty-print[=file]\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-O\t\t\t--optimize\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-p[file]\t\t--profile[=file]\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-P\t\t\t--posix\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-r\t\t\t--re-interval\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-s\t\t\t--no-optimize\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-S\t\t\t--sandbox\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-t\t\t\t--lint-old\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\t-V\t\t\t--version\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\nTo report bugs, use the `gawkbug' program.\nFor full instructions, see the node `Bugs' in `gawk.info'\nwhich is section `Reporting Problems and Bugs' in the\nprinted version.  This same information may be found at\nhttps://www.gnu.org/software/gawk/manual/html_node/Bugs.html.\nPLEASE do NOT try to report bugs by posting in comp.lang.awk,\nor by using a web forum such as Stack Overflow.\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fprintf(
        fp,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Source code for gawk formal releases may be obtained from\n%s.\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        &raw const gnu_url as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Source code for gawk beta releases may be obtained from\n%s.\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        &raw const beta_url as *const ::core::ffi::c_char,
    );
    fprintf(
        fp,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Alpha and beta releases have a version number >= 60 or that end in a letter.\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"gawk is a pattern scanning and processing language.\nBy default it reads standard input and writes standard output.\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        fp,
    );
    fprintf(
        fp,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Examples:\n\t%s '{ sum += $1 }; END { print sum }' file\n\t%s -F: '{ print $1 }' /etc/passwd\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        myname,
        myname,
    );
    fflush(fp);
    if ferror(fp) != 0 {
        os_maybe_set_errno();
        if *__errno_location() == EPIPE {
            signal(SIGPIPE, SIG_DFL);
            kill(getpid(), SIGPIPE);
        }
        if fp == stdout {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                676 as ::core::ffi::c_int,
            );
            Some(
                Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"error writing standard output: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                strerror(*__errno_location()),
            );
        } else if fp == stderr {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                678 as ::core::ffi::c_int,
            );
            Some(
                Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"error writing standard error: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                strerror(*__errno_location()),
            );
        }
        exit(EXIT_FAILURE);
    }
    exit(exitval);
}
unsafe extern "C" fn copyleft() -> ! {
    static mut blurb_part1: [::core::ffi::c_char; 297] = unsafe {
        ::core::mem::transmute::<
            [u8; 297],
            [::core::ffi::c_char; 297],
        >(
            *b"Copyright (C) 1989, 1991-%d Free Software Foundation.\n\nThis program is free software; you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation; either version 3 of the License, or\n(at your option) any later version.\n\n\0",
        )
    };
    static mut blurb_part2: [::core::ffi::c_char; 236] = unsafe {
        ::core::mem::transmute::<
            [u8; 236],
            [::core::ffi::c_char; 236],
        >(
            *b"This program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\n\n\0",
        )
    };
    static mut blurb_part3: [::core::ffi::c_char; 134] = unsafe {
        ::core::mem::transmute::<
            [u8; 134],
            [::core::ffi::c_char; 134],
        >(
            *b"You should have received a copy of the GNU General Public License\nalong with this program. If not, see http://www.gnu.org/licenses/.\n\0",
        )
    };
    printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw const blurb_part1 as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        UPDATE_YEAR,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw const blurb_part2 as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw const blurb_part3 as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        stdout,
    );
    fflush(stdout);
    if ferror(stdout) != 0 {
        os_maybe_set_errno();
        if *__errno_location() != EPIPE {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                721 as ::core::ffi::c_int,
            );
            Some(
                Some(r_warning as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"error writing standard output: %s\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                strerror(*__errno_location()),
            );
        }
        exit(EXIT_FAILURE);
    }
    exit(EXIT_SUCCESS);
}
unsafe extern "C" fn cmdline_fs(mut str: *mut ::core::ffi::c_char) {
    let mut tmp: *mut *mut NODE = ::core::ptr::null_mut::<*mut NODE>();
    tmp = &raw mut (*FS_node).sub.nodep.l.lptr as *mut *mut NODE;
    unref(*tmp);
    if *str.offset(0isize) as ::core::ffi::c_int == 't' as ::core::ffi::c_int
        && *str.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        if do_flags.0
            & (do_flag_values::DO_LINT_INVALID.0 as ::core::ffi::c_int
                | do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != 0
        {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                746 as ::core::ffi::c_int,
            );
            Some(lintfunc.expect("non-null function pointer")).expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"-Ft does not set FS to tab in POSIX awk\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        }
        if do_flags.0 & do_flag_values::DO_TRADITIONAL.0 != 0
            && do_flags.0 & do_flag_values::DO_POSIX.0 == 0
        {
            *str.offset(0isize) = '\t' as ::core::ffi::c_char;
        }
    }
    *tmp = make_str_node(str, strlen(str), SCAN);
    set_FS();
}
unsafe extern "C" fn init_args(
    mut argc0: ::core::ffi::c_int,
    mut argc: ::core::ffi::c_int,
    mut argv0: *const ::core::ffi::c_char,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    let mut sub: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut val: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut shadow_node: *mut NODE = ::core::ptr::null_mut::<NODE>();
    ARGV_node = install_symbol(
        estrdup(
            b"ARGV\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ),
        nodevals::Node_var_array,
    );
    sub = make_number.expect("non-null function pointer")(0.0f64);
    val = make_str_node(argv0, strlen(argv0), 0 as ::core::ffi::c_int);
    (*val).flags = flagvals((*val).flags.0 | flagvals::USER_INPUT.0);
    assoc_set(ARGV_node, sub, val);
    if do_flags.0 & do_flag_values::DO_SANDBOX.0 != 0 {
        shadow_node = make_array();
        sub = make_str_node(argv0, strlen(argv0), 0 as ::core::ffi::c_int);
        val = make_number.expect("non-null function pointer")(0.0f64);
        assoc_set(shadow_node, sub, val);
    }
    i = argc0;
    j = 1 as ::core::ffi::c_int;
    while i < argc {
        sub = make_number.expect("non-null function pointer")(j as ::core::ffi::c_double);
        val = make_str_node(
            *argv.offset(i as isize),
            strlen(*argv.offset(i as isize)),
            0 as ::core::ffi::c_int,
        );
        (*val).flags = flagvals((*val).flags.0 | flagvals::USER_INPUT.0);
        assoc_set(ARGV_node, sub, val);
        if do_flags.0 & do_flag_values::DO_SANDBOX.0 != 0 {
            sub = make_str_node(
                *argv.offset(i as isize),
                strlen(*argv.offset(i as isize)),
                0 as ::core::ffi::c_int,
            );
            val = make_number.expect("non-null function pointer")(0.0f64);
            assoc_set(shadow_node, sub, val);
        }
        i += 1;
        j += 1;
    }
    ARGC_node = install_symbol(
        estrdup(
            b"ARGC\0".as_ptr() as *const ::core::ffi::c_char,
            4 as size_t,
        ),
        nodevals::Node_var,
    );
    (*ARGC_node).sub.nodep.l.lptr =
        make_number.expect("non-null function pointer")(j as ::core::ffi::c_double)
            as *mut exp_node;
    if do_flags.0 & do_flag_values::DO_SANDBOX.0 != 0 {
        init_argv_array(ARGV_node, shadow_node);
    }
}
pub const NO_INSTALL: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NON_STANDARD: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const NOT_OFF_LIMITS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
static mut varinit: [varinit; 29] = unsafe {
    [
        varinit {
            spec: ::core::ptr::null_mut::<*mut NODE>(),
            name: b"ARGC\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NO_INSTALL,
        },
        varinit {
            spec: &raw const ARGIND_node as *mut *mut NODE,
            name: b"ARGIND\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: ::core::ptr::null_mut::<*mut NODE>(),
            name: b"ARGV\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NO_INSTALL,
        },
        varinit {
            spec: &raw const BINMODE_node as *mut *mut NODE,
            name: b"BINMODE\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_BINMODE as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const CONVFMT_node as *mut *mut NODE,
            name: b"CONVFMT\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"%.6g\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_CONVFMT as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: ::core::ptr::null_mut::<*mut NODE>(),
            name: b"ENVIRON\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NO_INSTALL,
        },
        varinit {
            spec: &raw const ERRNO_node as *mut *mut NODE,
            name: b"ERRNO\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const FIELDWIDTHS_node as *mut *mut NODE,
            name: b"FIELDWIDTHS\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_FIELDWIDTHS as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const FILENAME_node as *mut *mut NODE,
            name: b"FILENAME\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const FNR_node as *mut *mut NODE,
            name: b"FNR\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: Some(update_FNR as unsafe extern "C" fn() -> ()),
            assign: Some(set_FNR as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const FS_node as *mut *mut NODE,
            name: b"FS\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b" \0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_FS as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const FPAT_node as *mut *mut NODE,
            name: b"FPAT\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"[^[:space:]]+\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_FPAT as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const IGNORECASE_node as *mut *mut NODE,
            name: b"IGNORECASE\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_IGNORECASE as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const LINT_node as *mut *mut NODE,
            name: b"LINT\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_LINT as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const PREC_node as *mut *mut NODE,
            name: b"PREC\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: DEFAULT_PREC as ::core::ffi::c_double,
            update: None,
            assign: Some(set_PREC as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const NF_node as *mut *mut NODE,
            name: b"NF\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: -1 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: Some(update_NF as unsafe extern "C" fn() -> ()),
            assign: Some(set_NF as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const NR_node as *mut *mut NODE,
            name: b"NR\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: Some(update_NR as unsafe extern "C" fn() -> ()),
            assign: Some(set_NR as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const OFMT_node as *mut *mut NODE,
            name: b"OFMT\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"%.6g\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_OFMT as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const OFS_node as *mut *mut NODE,
            name: b"OFS\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b" \0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_OFS as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const ORS_node as *mut *mut NODE,
            name: b"ORS\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\n\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_ORS as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: ::core::ptr::null_mut::<*mut NODE>(),
            name: b"PROCINFO\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NO_INSTALL | NON_STANDARD | NOT_OFF_LIMITS,
        },
        varinit {
            spec: &raw const RLENGTH_node as *mut *mut NODE,
            name: b"RLENGTH\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const ROUNDMODE_node as *mut *mut NODE,
            name: b"ROUNDMODE\0".as_ptr() as *const ::core::ffi::c_char,
            strval: DEFAULT_ROUNDMODE.as_ptr(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_ROUNDMODE as unsafe extern "C" fn() -> ()),
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const RS_node as *mut *mut NODE,
            name: b"RS\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\n\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_RS as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const RSTART_node as *mut *mut NODE,
            name: b"RSTART\0".as_ptr() as *const ::core::ffi::c_char,
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const RT_node as *mut *mut NODE,
            name: b"RT\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: &raw const SUBSEP_node as *mut *mut NODE,
            name: b"SUBSEP\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"\x1C\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_SUBSEP as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: 0 as ::core::ffi::c_int,
        },
        varinit {
            spec: &raw const TEXTDOMAIN_node as *mut *mut NODE,
            name: b"TEXTDOMAIN\0".as_ptr() as *const ::core::ffi::c_char,
            strval: b"messages\0".as_ptr() as *const ::core::ffi::c_char,
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: Some(set_TEXTDOMAIN as unsafe extern "C" fn() -> ()),
            do_assign: r#true != 0,
            flags: NON_STANDARD,
        },
        varinit {
            spec: ::core::ptr::null_mut::<*mut NODE>(),
            name: ::core::ptr::null::<::core::ffi::c_char>(),
            strval: ::core::ptr::null::<::core::ffi::c_char>(),
            numval: 0 as ::core::ffi::c_int as ::core::ffi::c_double,
            update: None,
            assign: None,
            do_assign: r#false != 0,
            flags: 0 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn init_vars() {
    let mut vp: *const varinit = ::core::ptr::null::<varinit>();
    let mut n: *mut NODE = ::core::ptr::null_mut::<NODE>();
    vp = &raw const varinit as *const varinit;
    while !(*vp).name.is_null() {
        if !((*vp).flags & NO_INSTALL != 0 as ::core::ffi::c_int
            || do_flags.0 & do_flag_values::DO_TRADITIONAL.0 != 0
                && (*vp).flags & NON_STANDARD != 0 as ::core::ffi::c_int)
        {
            *(*vp).spec =
                install_symbol(estrdup((*vp).name, strlen((*vp).name)), nodevals::Node_var);
            n = *(*vp).spec;
            if !(*vp).strval.is_null() {
                (*n).sub.nodep.l.lptr =
                    make_str_node((*vp).strval, strlen((*vp).strval), 0 as ::core::ffi::c_int)
                        as *mut exp_node;
            } else {
                (*n).sub.nodep.l.lptr =
                    make_number.expect("non-null function pointer")((*vp).numval) as *mut exp_node;
            }
            (*n).sub.nodep.x.aptr = (*vp).assign as Option<unsafe extern "C" fn() -> ()>;
            (*n).sub.nodep.r.uptr = (*vp).update as Option<unsafe extern "C" fn() -> ()>;
            if (*vp).do_assign {
                Some((*vp).assign.expect("non-null function pointer"))
                    .expect("non-null function pointer")();
            }
        }
        vp = vp.offset(1);
    }
    if do_flags.0 & do_flag_values::DO_TRADITIONAL.0 == 0 {
        load_procinfo();
    }
    load_environ();
}
unsafe extern "C" fn path_environ(
    mut pname: *const ::core::ffi::c_char,
    mut dflt: *const ::core::ffi::c_char,
) {
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut aptr: *mut *mut NODE = ::core::ptr::null_mut::<*mut NODE>();
    let mut tmp: *mut NODE = ::core::ptr::null_mut::<NODE>();
    tmp = make_str_node(pname, strlen(pname), 0 as ::core::ffi::c_int);
    val = getenv(pname);
    if val.is_null() || *val as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        val = dflt;
    }
    aptr = (*(*ENVIRON_node).sub.nodep.l.lp)
        .lookup
        .expect("non-null function pointer")(ENVIRON_node, tmp) as *mut *mut NODE;
    if (**aptr).sub.val.slen == 0 as size_t {
        unref(*aptr);
        *aptr = make_str_node(val, strlen(val), 0 as ::core::ffi::c_int);
    }
    unref(tmp);
}
unsafe extern "C" fn load_environ() -> *mut NODE {
    extern "C" {
        #[link_name = "environ"]
        static mut environ_0: *mut *mut ::core::ffi::c_char;
    }
    let mut var: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut sub: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut newval: *mut NODE = ::core::ptr::null_mut::<NODE>();
    static mut been_here: bool = r#false != 0;
    if been_here {
        return ENVIRON_node;
    }
    been_here = r#true != 0;
    ENVIRON_node = install_symbol(
        estrdup(
            b"ENVIRON\0".as_ptr() as *const ::core::ffi::c_char,
            7 as size_t,
        ),
        nodevals::Node_var_array,
    );
    (*ENVIRON_node).sub.nodep.l.lp = &raw const str_array_func;
    i = 0 as ::core::ffi::c_int;
    while !(*environ.offset(i as isize)).is_null() {
        static mut nullstr: [::core::ffi::c_char; 1] =
            unsafe { ::core::mem::transmute::<[u8; 1], [::core::ffi::c_char; 1]>(*b"\0") };
        var = *environ.offset(i as isize);
        val = strchr(var, '=' as ::core::ffi::c_int);
        if !val.is_null() {
            let c2rust_fresh1 = val;
            val = val.offset(1);
            *c2rust_fresh1 = '\0' as ::core::ffi::c_char;
        } else {
            val = &raw mut nullstr as *mut ::core::ffi::c_char;
        }
        sub = make_str_node(var, strlen(var), 0 as ::core::ffi::c_int);
        newval = make_str_node(val, strlen(val), 0 as ::core::ffi::c_int);
        (*newval).flags = flagvals((*newval).flags.0 | flagvals::USER_INPUT.0);
        assoc_set(ENVIRON_node, sub, newval);
        if val != &raw mut nullstr as *mut ::core::ffi::c_char {
            val = val.offset(-1);
            *val = '=' as ::core::ffi::c_char;
        }
        i += 1;
    }
    path_environ(b"AWKPATH\0".as_ptr() as *const ::core::ffi::c_char, defpath);
    path_environ(
        b"AWKLIBPATH\0".as_ptr() as *const ::core::ffi::c_char,
        deflibpath,
    );
    init_env_array(ENVIRON_node);
    return ENVIRON_node;
}
unsafe extern "C" fn load_procinfo_argv() {
    let mut sub: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut val: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut argv_array: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut i: ::core::ffi::c_int = 0;
    argv_array = r_getblock(block_id::BLOCK_NODE.0 as ::core::ffi::c_int) as *mut NODE;
    memset(
        argv_array as *mut ::core::ffi::c_void,
        '\0' as ::core::ffi::c_int,
        ::core::mem::size_of::<NODE>(),
    );
    null_array(argv_array);
    (*argv_array).sub.nodep.x.extra = PROCINFO_node as *mut exp_node;
    (*argv_array).sub.nodep.name = estrdup(
        b"argv\0".as_ptr() as *const ::core::ffi::c_char,
        4 as size_t,
    );
    i = 0 as ::core::ffi::c_int;
    while !(*d_argv.offset(i as isize)).is_null() {
        sub = make_number.expect("non-null function pointer")(i as ::core::ffi::c_double);
        val = make_str_node(
            *d_argv.offset(i as isize),
            strlen(*d_argv.offset(i as isize)),
            0 as ::core::ffi::c_int,
        );
        assoc_set(argv_array, sub, val);
        i += 1;
    }
    sub = make_str_node(
        b"argv\0".as_ptr() as *const ::core::ffi::c_char,
        4 as size_t,
        0 as ::core::ffi::c_int,
    );
    assoc_set(PROCINFO_node, sub, argv_array);
}
unsafe extern "C" fn load_procinfo() -> *mut NODE {
    let mut i: ::core::ffi::c_int = 0;
    let mut name: [::core::ffi::c_char; 100] = [0; 100];
    let mut value: ::core::ffi::c_double = 0.;
    static mut been_here: bool = r#false != 0;
    if been_here {
        return PROCINFO_node;
    }
    been_here = r#true != 0;
    PROCINFO_node = install_symbol(
        estrdup(
            b"PROCINFO\0".as_ptr() as *const ::core::ffi::c_char,
            8 as size_t,
        ),
        nodevals::Node_var_array,
    );
    update_PROCINFO_str(
        b"version\0".as_ptr() as *const ::core::ffi::c_char,
        VERSION.as_ptr(),
    );
    update_PROCINFO_str(
        b"strftime\0".as_ptr() as *const ::core::ffi::c_char,
        &raw const def_strftime_format as *const ::core::ffi::c_char,
    );
    update_PROCINFO_str(
        b"platform\0".as_ptr() as *const ::core::ffi::c_char,
        platform_name(),
    );
    update_PROCINFO_num(
        b"api_major\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::GAWK_API_MAJOR_VERSION.0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    update_PROCINFO_num(
        b"api_minor\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::GAWK_API_MINOR_VERSION.0 as ::core::ffi::c_int as ::core::ffi::c_double,
    );
    value = getpgrp() as ::core::ffi::c_double;
    update_PROCINFO_num(b"pgrpid\0".as_ptr() as *const ::core::ffi::c_char, value);
    value = getpid() as ::core::ffi::c_double;
    update_PROCINFO_num(b"pid\0".as_ptr() as *const ::core::ffi::c_char, value);
    value = getppid() as ::core::ffi::c_double;
    update_PROCINFO_num(b"ppid\0".as_ptr() as *const ::core::ffi::c_char, value);
    value = getuid() as ::core::ffi::c_double;
    update_PROCINFO_num(b"uid\0".as_ptr() as *const ::core::ffi::c_char, value);
    value = geteuid() as ::core::ffi::c_double;
    update_PROCINFO_num(b"euid\0".as_ptr() as *const ::core::ffi::c_char, value);
    value = getgid() as ::core::ffi::c_double;
    update_PROCINFO_num(b"gid\0".as_ptr() as *const ::core::ffi::c_char, value);
    value = getegid() as ::core::ffi::c_double;
    update_PROCINFO_num(b"egid\0".as_ptr() as *const ::core::ffi::c_char, value);
    update_PROCINFO_str(
        b"FS\0".as_ptr() as *const ::core::ffi::c_char,
        current_field_sep_str(),
    );
    i = 0 as ::core::ffi::c_int;
    while i < ngroups {
        sprintf(
            &raw mut name as *mut ::core::ffi::c_char,
            b"group%d\0".as_ptr() as *const ::core::ffi::c_char,
            i + 1 as ::core::ffi::c_int,
        );
        value = *groupset.offset(i as isize) as ::core::ffi::c_double;
        update_PROCINFO_num(&raw mut name as *mut ::core::ffi::c_char, value);
        i += 1;
    }
    if !groupset.is_null() {
        pma_free(groupset as *mut ::core::ffi::c_void);
        groupset = ::core::ptr::null_mut::<gid_t>();
    }
    update_PROCINFO_str(
        b"pma\0".as_ptr() as *const ::core::ffi::c_char,
        get_pma_version(),
    );
    if do_flags.0 & do_flag_values::DO_CSV.0 != 0 {
        update_PROCINFO_num(
            b"CSV\0".as_ptr() as *const ::core::ffi::c_char,
            1 as ::core::ffi::c_int as ::core::ffi::c_double,
        );
    }
    load_procinfo_argv();
    return PROCINFO_node;
}
#[export_name = "rboxc_gawk_is_std_var"]
pub unsafe extern "C" fn is_std_var(mut var: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut vp: *const varinit = ::core::ptr::null::<varinit>();
    vp = &raw const varinit as *const varinit;
    while !(*vp).name.is_null() {
        if strcmp((*vp).name, var) == 0 as ::core::ffi::c_int {
            if do_flags.0 & do_flag_values::DO_TRADITIONAL.0 != 0
                && (*vp).flags & NON_STANDARD != 0 as ::core::ffi::c_int
            {
                return r#false;
            }
            return r#true;
        }
        vp = vp.offset(1);
    }
    return r#false;
}
#[export_name = "rboxc_gawk_is_off_limits_var"]
pub unsafe extern "C" fn is_off_limits_var(
    mut var: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut vp: *const varinit = ::core::ptr::null::<varinit>();
    vp = &raw const varinit as *const varinit;
    while !(*vp).name.is_null() {
        if strcmp((*vp).name, var) == 0 as ::core::ffi::c_int {
            return ((*vp).flags & NOT_OFF_LIMITS == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        }
        vp = vp.offset(1);
    }
    return r#false;
}
#[export_name = "rboxc_gawk_get_spec_varname"]
pub unsafe extern "C" fn get_spec_varname(mut fptr: Func_ptr) -> *const ::core::ffi::c_char {
    let mut vp: *const varinit = ::core::ptr::null::<varinit>();
    if fptr.is_none() {
        return ::core::ptr::null::<::core::ffi::c_char>();
    }
    vp = &raw const varinit as *const varinit;
    while !(*vp).name.is_null() {
        if (*vp).assign == fptr || (*vp).update == fptr {
            return (*vp).name;
        }
        vp = vp.offset(1);
    }
    return ::core::ptr::null::<::core::ffi::c_char>();
}
#[export_name = "rboxc_gawk_arg_assign"]
pub unsafe extern "C" fn arg_assign(
    mut arg: *mut ::core::ffi::c_char,
    mut initing: bool,
) -> ::core::ffi::c_int {
    let mut cp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cp2: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut badvar: bool = false;
    let mut var: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut it: *mut NODE = ::core::ptr::null_mut::<NODE>();
    let mut lhs: *mut *mut NODE = ::core::ptr::null_mut::<*mut NODE>();
    let mut save_FNR: ::core::ffi::c_long = 0;
    if !initing && disallow_var_assigns as ::core::ffi::c_int != 0 {
        return r#false;
    }
    cp = strchr(arg, '=' as ::core::ffi::c_int);
    if cp.is_null() {
        if !initing {
            return r#false;
        }
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: `%s' argument to `-v' not in `var=value' form\n\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            myname,
            arg,
        );
        usage(EXIT_FAILURE, stderr);
    }
    let c2rust_fresh0 = cp;
    cp = cp.offset(1);
    *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
    source = ::core::ptr::null_mut::<::core::ffi::c_char>();
    sourceline = 0 as ::core::ffi::c_int;
    save_FNR = FNR;
    FNR = 0 as ::core::ffi::c_long;
    badvar = r#false != 0;
    if !is_letter(*arg.offset(0isize) as ::core::ffi::c_uchar as ::core::ffi::c_int) {
        badvar = r#true != 0;
    } else {
        cp2 = arg.offset(1 as ::core::ffi::c_int as isize);
        while *cp2 != 0 {
            if !is_identchar(*cp2 as ::core::ffi::c_uchar as ::core::ffi::c_int)
                && *cp2 as ::core::ffi::c_int != ':' as ::core::ffi::c_int
            {
                badvar = r#true != 0;
                break;
            } else {
                cp2 = cp2.offset(1);
            }
        }
    }
    if badvar {
        if initing {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                1201 as ::core::ffi::c_int,
            );
            Some(
                Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"`%s' is not a legal variable name\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                arg,
            );
        }
        if do_flags.0
            & (do_flag_values::DO_LINT_INVALID.0 as ::core::ffi::c_int
                | do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            != 0
        {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                1204 as ::core::ffi::c_int,
            );
            Some(lintfunc.expect("non-null function pointer")).expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"`%s' is not a variable name, looking for file `%s=%s'\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                arg,
                arg,
                cp,
            );
        }
    } else if !validate_qualified_name(arg) {
        badvar = r#true != 0;
    } else {
        if check_special(arg) >= 0 as ::core::ffi::c_int {
            set_loc(
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                1218 as ::core::ffi::c_int,
            );
            Some(
                Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"cannot use gawk builtin `%s' as variable name\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                arg,
            );
        }
        if !initing {
            var = lookup(arg);
            if !var.is_null() && (*var).r#type.0 == nodevals::Node_func.0 {
                set_loc(
                    b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1223 as ::core::ffi::c_int,
                );
                Some(
                    Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"cannot use function `%s' as variable name\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    arg,
                );
            }
        }
        cp2 = cp
            .offset(strlen(cp) as isize)
            .offset(-(1 as ::core::ffi::c_int as isize));
        if do_flags.0 & do_flag_values::DO_TRADITIONAL.0 == 0
            && strlen(cp) >= 3 as size_t
            && *cp.offset(0isize) as ::core::ffi::c_int == '@' as ::core::ffi::c_int
            && *cp.offset(1isize) as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            && *cp2 as ::core::ffi::c_int == '/' as ::core::ffi::c_int
        {
            let mut len: size_t = strlen(cp).wrapping_sub(3 as size_t);
            cp2 = ezalloc_real(
                len.wrapping_add(1 as size_t),
                b"arg_assign\0".as_ptr() as *const ::core::ffi::c_char,
                b"cp2\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                1233 as ::core::ffi::c_int,
            ) as *mut ::core::ffi::c_char;
            memcpy(
                cp2 as *mut ::core::ffi::c_void,
                cp.offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                len,
            );
            it = make_typed_regex(cp2, len);
        } else {
            if do_flags.0 & do_flag_values::DO_POSIX.0 != 0
                && !strchr(cp, '\n' as ::core::ffi::c_int).is_null()
            {
                set_loc(
                    b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1245 as ::core::ffi::c_int,
                );
                Some(
                    Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")(dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"POSIX does not allow physical newlines in string values\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ));
            }
            it = make_str_node(cp, strlen(cp), SCAN);
            (*it).flags = flagvals((*it).flags.0 | flagvals::USER_INPUT.0);
            if do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
                setlocale(LC_NUMERIC, b"C\0".as_ptr() as *const ::core::ffi::c_char);
            }
            force_number(it);
            if do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
                setlocale(LC_NUMERIC, locale);
            }
        }
        cp2 = estrdup(arg, cp.offset_from(arg) as size_t);
        var = variable(0 as ::core::ffi::c_int, cp2, nodevals::Node_var);
        if var.is_null() {
            final_exit(EXIT_FATAL);
        }
        if (*var).r#type.0 == nodevals::Node_var.0 && (*var).sub.nodep.r.uptr.is_some() {
            (*var).sub.nodep.r.uptr.expect("non-null function pointer")();
        }
        lhs = (if (*var).r#type.0 == nodevals::Node_var.0
            && !((*var).sub.nodep.l.lptr == Nnull_string)
        {
            &raw mut (*var).sub.nodep.l.lptr
        } else {
            r_get_lhs(var, false)
        }) as *mut *mut NODE;
        unref(*lhs);
        *lhs = it;
        if (*var).r#type.0 == nodevals::Node_var.0 && (*var).sub.nodep.x.aptr.is_some() {
            (*var).sub.nodep.x.aptr.expect("non-null function pointer")();
        }
    }
    if !initing {
        cp = cp.offset(-1);
        *cp = '=' as ::core::ffi::c_char;
    }
    FNR = save_FNR;
    return !badvar as ::core::ffi::c_int;
}
unsafe extern "C" fn catchsig(mut sig: ::core::ffi::c_int) {
    if sig == SIGFPE {
        set_loc(
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            1302 as ::core::ffi::c_int,
        );
        Some(
            Some(r_fatal as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> ())
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"floating point exception\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
    } else if sig == SIGSEGV || sig == SIGBUS {
        if errcount > 0 as ::core::ffi::c_int {
            exit(EXIT_FATAL);
        }
        set_loc(
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            1311 as ::core::ffi::c_int,
        );
        msg(dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"fatal error: internal error\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ));
        fflush(::core::ptr::null_mut::<FILE>());
        abort();
    } else {
        r_fatal(
            b"internal error: file %s, line %d: unexpected signal, number %d (%s)\0".as_ptr()
                as *const ::core::ffi::c_char,
            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
            1318 as ::core::ffi::c_int,
            sig,
            strsignal(sig),
        );
    };
}
#[export_name = "rboxc_gawk_get_pma_version"]
pub unsafe extern "C" fn get_pma_version() -> *const ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 200] = [0; 200];
    let mut open: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut close: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut out: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut r#in: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    open = strchr(
        &raw const pma_version as *const ::core::ffi::c_char,
        '(' as ::core::ffi::c_int,
    ) as *const ::core::ffi::c_char;
    if open.is_null() {
        return &raw const pma_version as *const ::core::ffi::c_char;
    }
    open = open.offset(1);
    close = strchr(open, ')' as ::core::ffi::c_int) as *const ::core::ffi::c_char;
    if close.is_null() {
        return &raw const pma_version as *const ::core::ffi::c_char;
    }
    out = &raw mut buf as *mut ::core::ffi::c_char;
    r#in = open;
    while r#in < close {
        let c2rust_fresh2 = r#in;
        r#in = r#in.offset(1);
        let c2rust_fresh3 = out;
        out = out.offset(1);
        *c2rust_fresh3 = *c2rust_fresh2;
    }
    let c2rust_fresh4 = out;
    out = out.offset(1);
    *c2rust_fresh4 = '\0' as ::core::ffi::c_char;
    return &raw mut buf as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn version() -> ! {
    printf(
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        version_string,
    );
    printf(
        b", API %d.%d\0".as_ptr() as *const ::core::ffi::c_char,
        C2Rust_Unnamed_0::GAWK_API_MAJOR_VERSION.0 as ::core::ffi::c_int,
        C2Rust_Unnamed_0::GAWK_API_MINOR_VERSION.0 as ::core::ffi::c_int,
    );
    printf(
        b", PMA %s\0".as_ptr() as *const ::core::ffi::c_char,
        get_pma_version(),
    );
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    print_ext_versions();
    copyleft();
}
unsafe extern "C" fn init_fds() {
    let mut sbuf: stat = stat {
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
    let mut fd: ::core::ffi::c_int = 0;
    let mut newfd: ::core::ffi::c_int = 0;
    let opposite_mode: [*const ::core::ffi::c_char; 3] = [
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
    ];
    fd = 0 as ::core::ffi::c_int;
    while fd <= 2 as ::core::ffi::c_int {
        if fstat(fd, &raw mut sbuf) < 0 as ::core::ffi::c_int {
            newfd = devopen(
                b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
                opposite_mode[fd as usize],
            );
            newfd = newfd + 0 as ::core::ffi::c_int;
        }
        fd += 1;
    }
}
unsafe extern "C" fn init_groupset() {
    ngroups = getgroups(0 as ::core::ffi::c_int, ::core::ptr::null_mut::<__gid_t>());
    if ngroups <= 0 as ::core::ffi::c_int {
        return;
    }
    groupset = emalloc_real(
        (ngroups as usize).wrapping_mul(::core::mem::size_of::<gid_t>()),
        b"init_groupset\0".as_ptr() as *const ::core::ffi::c_char,
        b"groupset\0".as_ptr() as *const ::core::ffi::c_char,
        b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        1433 as ::core::ffi::c_int,
    ) as *mut gid_t;
    ngroups = getgroups(ngroups, groupset);
    if ngroups == -1 as ::core::ffi::c_int {
        pma_free(groupset as *mut ::core::ffi::c_void);
        ngroups = 0 as ::core::ffi::c_int;
        groupset = ::core::ptr::null_mut::<gid_t>();
    }
}
#[export_name = "rboxc_gawk_estrdup"]
pub unsafe extern "C" fn estrdup(
    mut str: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = emalloc_real(
        len.wrapping_add(1 as size_t),
        b"estrdup\0".as_ptr() as *const ::core::ffi::c_char,
        b"s\0".as_ptr() as *const ::core::ffi::c_char,
        b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        1451 as ::core::ffi::c_int,
    ) as *mut ::core::ffi::c_char;
    memcpy(
        s as *mut ::core::ffi::c_void,
        str as *const ::core::ffi::c_void,
        len,
    );
    *s.offset(len as isize) = '\0' as ::core::ffi::c_char;
    return s;
}
unsafe extern "C" fn init_locale(mut l: *mut lconv) {
    let mut t: *mut lconv = ::core::ptr::null_mut::<lconv>();
    t = localeconv();
    *l = *t;
    (*l).thousands_sep = estrdup((*t).thousands_sep, strlen((*t).thousands_sep));
    (*l).decimal_point = estrdup((*t).decimal_point, strlen((*t).decimal_point));
    (*l).grouping = estrdup((*t).grouping, strlen((*t).grouping));
    (*l).int_curr_symbol = estrdup((*t).int_curr_symbol, strlen((*t).int_curr_symbol));
    (*l).currency_symbol = estrdup((*t).currency_symbol, strlen((*t).currency_symbol));
    (*l).mon_decimal_point = estrdup((*t).mon_decimal_point, strlen((*t).mon_decimal_point));
    (*l).mon_thousands_sep = estrdup((*t).mon_thousands_sep, strlen((*t).mon_thousands_sep));
    (*l).mon_grouping = estrdup((*t).mon_grouping, strlen((*t).mon_grouping));
    (*l).positive_sign = estrdup((*t).positive_sign, strlen((*t).positive_sign));
    (*l).negative_sign = estrdup((*t).negative_sign, strlen((*t).negative_sign));
}
unsafe extern "C" fn save_argv(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut i: ::core::ffi::c_int = 0;
    d_argv = emalloc_real(
        ((argc + 1 as ::core::ffi::c_int) as usize)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
        b"save_argv\0".as_ptr() as *const ::core::ffi::c_char,
        b"d_argv\0".as_ptr() as *const ::core::ffi::c_char,
        b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
        1496 as ::core::ffi::c_int,
    ) as *mut *mut ::core::ffi::c_char;
    i = 0 as ::core::ffi::c_int;
    while i < argc {
        *d_argv.offset(i as isize) =
            estrdup(*argv.offset(i as isize), strlen(*argv.offset(i as isize)));
        i += 1;
    }
    *d_argv.offset(argc as isize) = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[export_name = "rboxc_gawk_update_global_values"]
pub unsafe extern "C" fn update_global_values() {
    let mut vp: *const varinit = ::core::ptr::null::<varinit>();
    vp = &raw const varinit as *const varinit;
    while !(*vp).name.is_null() {
        if (*vp).update.is_some() {
            (*vp).update.expect("non-null function pointer")();
        }
        vp = vp.offset(1);
    }
}
#[export_name = "rboxc_gawk_getenv_long"]
pub unsafe extern "C" fn getenv_long(mut name: *const ::core::ffi::c_char) -> ::core::ffi::c_long {
    let mut val: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut newval: ::core::ffi::c_long = 0;
    val = getenv(name);
    if !val.is_null()
        && *(*__ctype_b_loc()).offset(*val as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                as ::core::ffi::c_int
            != 0
    {
        newval = 0 as ::core::ffi::c_long;
        while *val as ::core::ffi::c_int != 0
            && *(*__ctype_b_loc())
                .offset(*val as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                    as ::core::ffi::c_int
                != 0
        {
            newval = newval * 10 as ::core::ffi::c_long + *val as ::core::ffi::c_long
                - '0' as ::core::ffi::c_long;
            val = val.offset(1);
        }
        return newval;
    }
    return -1 as ::core::ffi::c_long;
}
unsafe extern "C" fn parse_args(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) {
    let mut optlist: *const ::core::ffi::c_char =
        b"+F:f:v:W;bcCd::D::e:E:ghi:kIl:L::nNo::Op::MPrSstVYZ:G\0".as_ptr()
            as *const ::core::ffi::c_char;
    let mut old_optind: ::core::ffi::c_int = 0;
    let mut c: ::core::ffi::c_int = 0;
    let mut scan: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut src: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    opterr = r#false;
    save_argv(argc, argv);
    optopt = 0 as ::core::ffi::c_int;
    old_optind = 1 as ::core::ffi::c_int;
    '_out: loop {
        c = getopt_long(
            argc,
            argv,
            optlist,
            &raw const optab as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if c == EOF {
            break;
        }
        if do_flags.0 & do_flag_values::DO_POSIX.0 != 0 {
            opterr = r#true;
        }
        's_423: {
            'c_20973: {
                match c {
                    70 => {
                        add_preassign(assign_type::PRE_ASSIGN_FS, optarg);
                        break 's_423;
                    }
                    69 => {
                        disallow_var_assigns = r#true != 0;
                        break 'c_20973;
                    }
                    102 => {
                        break 'c_20973;
                    }
                    118 => {
                        add_preassign(assign_type::PRE_ASSIGN, optarg);
                        break 's_423;
                    }
                    98 => {
                        do_binary = r#true;
                        break 's_423;
                    }
                    99 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_TRADITIONAL.0);
                        break 's_423;
                    }
                    67 => {
                        copyleft();
                    }
                    100 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_DUMP_VARS.0);
                        if !optarg.is_null()
                            && *optarg.offset(0isize) as ::core::ffi::c_int
                                != '\0' as ::core::ffi::c_int
                        {
                            varfile = optarg;
                        }
                        break 's_423;
                    }
                    68 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_DEBUG.0);
                        if !optarg.is_null()
                            && *optarg.offset(0isize) as ::core::ffi::c_int
                                != '\0' as ::core::ffi::c_int
                        {
                            command_file = optarg;
                        }
                        break 's_423;
                    }
                    101 => {
                        if *optarg.offset(0isize) as ::core::ffi::c_int
                            == '\0' as ::core::ffi::c_int
                        {
                            set_loc(
                                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1621 as ::core::ffi::c_int,
                            );
                            Some(
                                Some(
                                    r_warning
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_char,
                                            ...
                                        )
                                            -> (),
                                )
                                .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"empty argument to `-e/--source' ignored\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    LC_MESSAGES,
                                ),
                            );
                        } else {
                            add_srcfile(
                                srctype::SRC_CMDLINE,
                                optarg,
                                srcfiles,
                                ::core::ptr::null_mut::<bool>(),
                                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                            );
                        }
                        break 's_423;
                    }
                    103 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_INTL.0);
                        break 's_423;
                    }
                    71 => {
                        use_gnu_matchers = r#true != 0;
                        break 's_423;
                    }
                    104 => {
                        usage(EXIT_SUCCESS, stdout);
                    }
                    105 => {
                        add_srcfile(
                            srctype::SRC_INC,
                            optarg,
                            srcfiles,
                            ::core::ptr::null_mut::<bool>(),
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        );
                        break 's_423;
                    }
                    73 => {
                        do_itrace = r#true != 0;
                        break 's_423;
                    }
                    107 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_CSV.0);
                        break 's_423;
                    }
                    108 => {
                        add_srcfile(
                            srctype::SRC_EXTLIB,
                            optarg,
                            srcfiles,
                            ::core::ptr::null_mut::<bool>(),
                            ::core::ptr::null_mut::<::core::ffi::c_int>(),
                        );
                        break 's_423;
                    }
                    76 => {
                        do_flags = do_flag_values(
                            do_flags.0
                                | (do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int
                                    | do_flag_values::DO_LINT_EXTENSIONS.0 as ::core::ffi::c_int)
                                    as ::core::ffi::c_uint,
                        );
                        if !optarg.is_null() {
                            if strcmp(optarg, b"fatal\0".as_ptr() as *const ::core::ffi::c_char)
                                == 0 as ::core::ffi::c_int
                            {
                                lintfunc = Some(
                                    r_fatal
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_char,
                                            ...
                                        )
                                            -> (),
                                )
                                    as Option<
                                        unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
                                    >;
                            } else if strcmp(
                                optarg,
                                b"invalid\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                do_flags = do_flag_values(
                                    do_flags.0
                                        & !(do_flag_values::DO_LINT_ALL.0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint,
                                );
                                do_flags =
                                    do_flag_values(do_flags.0 | do_flag_values::DO_LINT_INVALID.0);
                            } else if strcmp(
                                optarg,
                                b"no-ext\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0 as ::core::ffi::c_int
                            {
                                do_flags = do_flag_values(
                                    do_flags.0
                                        & !(do_flag_values::DO_LINT_EXTENSIONS.0
                                            as ::core::ffi::c_int)
                                            as ::core::ffi::c_uint,
                                );
                            }
                        }
                        break 's_423;
                    }
                    116 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_LINT_OLD.0);
                        break 's_423;
                    }
                    110 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_NON_DEC_DATA.0);
                        break 's_423;
                    }
                    78 => {
                        use_lc_numeric = r#true;
                        break 's_423;
                    }
                    79 => {
                        do_optimize = r#true != 0;
                        break 's_423;
                    }
                    112 => {
                        if do_flags.0 & do_flag_values::DO_PRETTY_PRINT.0 != 0
                            && do_flags.0 & do_flag_values::DO_PROFILE.0 == 0
                        {
                            set_loc(
                                b"/opt/src/gawk-5.4.1/main.c\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                1694 as ::core::ffi::c_int,
                            );
                            Some(
                                Some(
                                    r_warning
                                        as unsafe extern "C" fn(
                                            *const ::core::ffi::c_char,
                                            ...
                                        )
                                            -> (),
                                )
                                .expect("non-null function pointer"),
                            )
                            .expect("non-null function pointer")(
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"`--profile' overrides `--pretty-print'\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    LC_MESSAGES,
                                ),
                            );
                        }
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_PROFILE.0);
                    }
                    111 => {}
                    77 => {
                        set_loc(
                            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1711 as ::core::ffi::c_int,
                        );
                        Some(
                            Some(
                                r_warning
                                    as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
                            )
                            .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"-M ignored: MPFR/GMP support not compiled in\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ));
                        break 's_423;
                    }
                    80 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_POSIX.0);
                        break 's_423;
                    }
                    114 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_INTERVALS.0);
                        break 's_423;
                    }
                    115 => {
                        do_optimize = r#false != 0;
                        break 's_423;
                    }
                    83 => {
                        do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_SANDBOX.0);
                        break 's_423;
                    }
                    84 => {
                        if optarg.is_null() {
                            optarg = b"/some/file\0".as_ptr() as *const ::core::ffi::c_char
                                as *mut ::core::ffi::c_char;
                        }
                        set_loc(
                            b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                            1737 as ::core::ffi::c_int,
                        );
                        Some(
                            Some(
                                r_fatal
                                    as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
                            )
                            .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"Use `GAWK_PERSIST_FILE=%s gawk ...' instead of --persist.\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ),
                            optarg,
                        );
                        break 's_423;
                    }
                    86 => {
                        do_version = r#true;
                        break 's_423;
                    }
                    87 => {
                        fprintf(
                            stderr,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"%s: option `-W %s' unrecognized, ignored\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ),
                            *argv.offset(0isize),
                            optarg,
                        );
                        break 's_423;
                    }
                    0 => {
                        break 's_423;
                    }
                    89 | 90 | 63 | _ => {
                        if do_flags.0 & do_flag_values::DO_POSIX.0 == 0
                            && (optopt == '\0' as ::core::ffi::c_int
                                || (strchr(optlist, optopt) as *const ::core::ffi::c_char)
                                    .is_null())
                        {
                            optind = old_optind;
                            stopped_early = r#true != 0;
                            break '_out;
                        } else {
                            if optopt != '\0' as ::core::ffi::c_int {
                                fprintf(
                                    stderr,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"%s: option requires an argument -- %c\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        LC_MESSAGES,
                                    ),
                                    myname,
                                    optopt,
                                );
                                usage(EXIT_FAILURE, stderr);
                            }
                            break 's_423;
                        }
                    }
                }
                if c == 'o' as ::core::ffi::c_int && do_flags.0 & do_flag_values::DO_PROFILE.0 != 0
                {
                    set_loc(
                        b"/opt/src/gawk-5.4.1/main.c\0".as_ptr() as *const ::core::ffi::c_char,
                        1699 as ::core::ffi::c_int,
                    );
                    Some(
                        Some(
                            r_warning
                                as unsafe extern "C" fn(*const ::core::ffi::c_char, ...) -> (),
                        )
                        .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"`--profile' overrides `--pretty-print'\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ));
                }
                do_flags = do_flag_values(do_flags.0 | do_flag_values::DO_PRETTY_PRINT.0);
                if !optarg.is_null() {
                    set_prof_file(optarg);
                } else {
                    set_prof_file(DEFAULT_PROFILE.as_ptr());
                }
                break 's_423;
            }
            scan = optarg;
            if *argv.offset((optind - 1 as ::core::ffi::c_int) as isize) != optarg {
                while *(*__ctype_b_loc())
                    .offset(*scan as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int
                    & C2Rust_Unnamed::_ISspace.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
                        as ::core::ffi::c_int
                    != 0
                {
                    scan = scan.offset(1);
                }
            }
            src = if *scan as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
                let c2rust_fresh5 = optind;
                optind += 1;
                *argv.offset(c2rust_fresh5 as isize)
            } else {
                optarg
            };
            add_srcfile(
                srctype(
                    (if !src.is_null()
                        && *src.offset(0isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                        && *src.offset(1isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
                    {
                        srctype::SRC_STDIN.0 as ::core::ffi::c_int
                    } else {
                        srctype::SRC_FILE.0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint,
                ),
                src,
                srcfiles,
                ::core::ptr::null_mut::<bool>(),
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
            );
        }
        if c == 'E' as ::core::ffi::c_int {
            break;
        }
        optopt = 0 as ::core::ffi::c_int;
        old_optind = optind;
    }
    do_optimize = do_optimize as ::core::ffi::c_int != 0
        && do_flags.0 & do_flag_values::DO_PRETTY_PRINT.0 == 0;
    pma_mpfr_check();
}
unsafe extern "C" fn set_locale_stuff() {
    setlocale(LC_CTYPE, locale);
    setlocale(LC_COLLATE, locale);
    setlocale(LC_MESSAGES, locale);
    setlocale(LC_NUMERIC, locale);
    init_locale(&raw mut loc);
    setlocale(LC_NUMERIC, b"C\0".as_ptr() as *const ::core::ffi::c_char);
    setlocale(LC_TIME, locale);
    bindtextdomain(PACKAGE.as_ptr(), locale_dir);
    textdomain(PACKAGE.as_ptr());
}
unsafe extern "C" fn platform_name() -> *const ::core::ffi::c_char {
    return b"posix\0".as_ptr() as *const ::core::ffi::c_char;
}
#[export_name = "rboxc_gawk_set_current_namespace"]
pub unsafe extern "C" fn set_current_namespace(mut new_namespace: *const ::core::ffi::c_char) {
    if current_namespace != &raw const awk_namespace as *const ::core::ffi::c_char {
        pma_free(current_namespace as *mut ::core::ffi::c_void);
    }
    current_namespace = new_namespace;
}
unsafe extern "C" fn check_pma_security(mut pma_file: *const ::core::ffi::c_char) {
    let mut sbuf: stat = stat {
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
    let mut euid: ::core::ffi::c_int = geteuid() as ::core::ffi::c_int;
    if pma_file.is_null() {
        return;
    } else if stat(pma_file, &raw mut sbuf) < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: fatal: cannot stat %s: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            myname,
            pma_file,
            strerror(*__errno_location()),
        );
        exit(EXIT_FATAL);
    } else if euid == 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: fatal: using persistent memory is not allowed when running as root.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            myname,
        );
        exit(EXIT_FATAL);
    } else if sbuf.st_uid != euid as __uid_t {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: warning: %s is not owned by euid %d.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            myname,
            pma_file,
            euid,
        );
    }
}
unsafe extern "C" fn enable_pma(mut argv: *mut *mut ::core::ffi::c_char) -> bool {
    persist_file = getenv(b"GAWK_PERSIST_FILE\0".as_ptr() as *const ::core::ffi::c_char);
    os_disable_aslr(persist_file, argv);
    check_pma_security(persist_file);
    let mut pma_result: ::core::ffi::c_int = pma_init(1 as ::core::ffi::c_int, persist_file);
    if pma_result != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: fatal: persistent memory allocator failed to initialize: return value %d, pma.c line: %d.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            myname,
            pma_result,
            pma_errno,
        );
        exit(EXIT_FATAL);
    }
    return !persist_file.is_null();
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __LC_CTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_NUMERIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const __LC_TIME: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const __LC_COLLATE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGFPE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const SIGBUS: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EPIPE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const LOCALEDIR: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"/root/rboxc/build/oracle/gawk/share/locale\0",
    )
};
