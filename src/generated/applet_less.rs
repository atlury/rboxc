// Generated from pinned GNU less 704 by scripts/translate-entry-provider.py.
// Source SHA-256: eb38294b99c8483d2d3cb747051884a2854c89e3e7b34ceb49286d2ab6f5fe56
/*
 * Copyright (C) 1984-2026  Mark Nudelman
 *
 * You may distribute under the terms of either the GNU General Public
 * License or the Less License, as specified in the README file.
 *
 * For more information, see the README file.
 */
extern "C" {
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn calloc(__nmemb: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_less_xbuf_init"]
    fn xbuf_init(xbuf: *mut xbuffer);
    #[link_name = "rboxc_less_xbuf_deinit"]
    fn xbuf_deinit(xbuf: *mut xbuffer);
    #[link_name = "rboxc_less_xbuf_add_data"]
    fn xbuf_add_data(xbuf: *mut xbuffer, data: *const ::core::ffi::c_void, len: size_t);
    #[link_name = "rboxc_less_raw_mode"]
    fn raw_mode(on: ::core::ffi::c_int);
    #[link_name = "rboxc_less_get_term"]
    fn get_term();
    #[link_name = "rboxc_less_term_deinit"]
    fn term_deinit();
    #[link_name = "rboxc_less_interactive"]
    fn interactive() -> lbool;
    #[link_name = "rboxc_less_clear_bot"]
    fn clear_bot();
    #[link_name = "rboxc_less_init_charset"]
    fn init_charset();
    #[link_name = "rboxc_less_init_cmdhist"]
    fn init_cmdhist();
    #[link_name = "rboxc_less_save_cmdhist"]
    fn save_cmdhist();
    #[link_name = "rboxc_less_commands"]
    fn commands();
    #[link_name = "rboxc_less_expand_cmd_tables"]
    fn expand_cmd_tables();
    #[link_name = "rboxc_less_init_cmds"]
    fn init_cmds();
    #[link_name = "rboxc_less_parse_csl_bitmap"]
    fn parse_csl_bitmap(
        str: *const ::core::ffi::c_char,
        defs: *mut csl_bitmap_def,
        num_defs: ::core::ffi::c_int,
        pfx: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_lgetenv"]
    fn lgetenv(var: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_less_isnullenv"]
    fn isnullenv(s: *const ::core::ffi::c_char) -> lbool;
    #[link_name = "rboxc_less_check_altpipe_error"]
    fn check_altpipe_error();
    #[link_name = "rboxc_less_edit"]
    fn edit(filename: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_edit_first"]
    fn edit_first() -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_edit_next"]
    fn edit_next(n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_cat_file"]
    fn cat_file();
    #[link_name = "rboxc_less_last_component"]
    fn last_component(name: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_less_get_one_screen"]
    fn get_one_screen() -> lbool;
    #[link_name = "rboxc_less_prev_ifile"]
    fn prev_ifile(h: *mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_less_nifile"]
    fn nifile() -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_get_ifile"]
    fn get_ifile(
        filename: *const ::core::ffi::c_char,
        prev: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_less_repaint"]
    fn repaint();
    #[link_name = "rboxc_less_init_line"]
    fn init_line();
    #[link_name = "rboxc_less_init_mark"]
    fn init_mark();
    #[link_name = "rboxc_less_opt_header"]
    fn opt_header(r#type: ::core::ffi::c_int, s: *const ::core::ffi::c_char);
    #[link_name = "rboxc_less_scan_option"]
    fn scan_option(s: *const ::core::ffi::c_char, is_env: lbool);
    #[link_name = "rboxc_less_isoptpending"]
    fn isoptpending() -> lbool;
    #[link_name = "rboxc_less_nopendopt"]
    fn nopendopt();
    #[link_name = "rboxc_less_init_unsupport"]
    fn init_unsupport();
    #[link_name = "rboxc_less_init_option"]
    fn init_option();
    #[link_name = "rboxc_less_init_poll"]
    fn init_poll();
    #[link_name = "rboxc_less_get_time"]
    fn get_time() -> time_t;
    #[link_name = "rboxc_less_flush"]
    fn flush();
    #[link_name = "rboxc_less_set_output"]
    fn set_output(fd: ::core::ffi::c_int, no_term_init: lbool);
    #[link_name = "rboxc_less_putchr"]
    fn putchr(ch: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_less_printf"]
    fn less_printf(fmt: *const ::core::ffi::c_char, parg: *const PARG) -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_get_return"]
    fn get_return();
    #[link_name = "rboxc_less_error"]
    fn error(fmt: *const ::core::ffi::c_char, parg: *const PARG);
    #[link_name = "rboxc_less_init_prompt"]
    fn init_prompt();
    #[link_name = "rboxc_less_init_search"]
    fn init_search();
    #[link_name = "rboxc_less_init_signals"]
    fn init_signals(on: ::core::ffi::c_int);
    #[link_name = "rboxc_less_findtag"]
    fn findtag(tag: *const ::core::ffi::c_char);
    #[link_name = "rboxc_less_tagsearch"]
    fn tagsearch() -> POSITION;
    #[link_name = "rboxc_less_edit_tagfile"]
    fn edit_tagfile() -> ::core::ffi::c_int;
    #[link_name = "rboxc_less_open_getchr"]
    fn open_getchr();
    #[link_name = "rboxc_less_close_getchr"]
    fn close_getchr();
    #[link_name = "rboxc_less_tags"]
    static mut tags: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_less_tagoption"]
    static mut tagoption: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_less_jump_sline"]
    static mut jump_sline: ::core::ffi::c_int;
    #[link_name = "rboxc_less_less_is_more"]
    static mut less_is_more: ::core::ffi::c_int;
    #[link_name = "rboxc_less_missing_cap"]
    static mut missing_cap: lbool;
    #[link_name = "rboxc_less_know_dumb"]
    static mut know_dumb: ::core::ffi::c_int;
    #[link_name = "rboxc_less_quit_if_one_screen"]
    static mut quit_if_one_screen: ::core::ffi::c_int;
    #[link_name = "rboxc_less_no_init"]
    static mut no_init: ::core::ffi::c_int;
    #[link_name = "rboxc_less_errmsgs"]
    static mut errmsgs: ::core::ffi::c_int;
    #[link_name = "rboxc_less_redraw_on_quit"]
    static mut redraw_on_quit: ::core::ffi::c_int;
    #[link_name = "rboxc_less_term_addrs"]
    static mut term_addrs: ::core::ffi::c_int;
    #[link_name = "rboxc_less_first_time"]
    static mut first_time: lbool;
}
pub type __off_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct lbool(pub ::core::ffi::c_uint);
impl lbool {
    pub const LFALSE: Self = Self(0);
    pub const LTRUE: Self = Self(1);
}
pub type less_off_t = off_t;
pub type POSITION = less_off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *const ::core::ffi::c_char,
    pub p_int: ::core::ffi::c_int,
    pub p_linenum: LINENUM,
    pub p_char: ::core::ffi::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csl_bitmap_def {
    pub bit_name: *const ::core::ffi::c_char,
    pub bit_value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xbuffer {
    pub data: *mut ::core::ffi::c_uchar,
    pub end: size_t,
    pub size: size_t,
    pub init_size: size_t,
}
pub const EDIT_PGM: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"vi\0") };
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const NULL_POSITION: POSITION = -1 as ::core::ffi::c_int as POSITION;
pub const NULL_IFILE: *mut ::core::ffi::c_void = NULL;
pub const NULL_PARG: *mut PARG = NULL as *mut PARG;
pub const QUIT_OK: ::core::ffi::c_int = EXIT_SUCCESS;
pub const QUIT_ERROR: ::core::ffi::c_int = EXIT_FAILURE;
pub const FAKE_HELPFILE: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"@/\\less/\\help/\\file/\\@\0")
};
pub const SF_EDIT: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int;
pub const SF_EXAMINE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const SF_GLOB: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int;
pub const SF_HISTORY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int;
pub const SF_LESSKEY: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int;
pub const SF_LESSOPEN: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int;
pub const SF_LOGFILE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int;
pub const SF_PIPE: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 8 as ::core::ffi::c_int;
pub const SF_SHELL: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 9 as ::core::ffi::c_int;
pub const SF_STOP: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 10 as ::core::ffi::c_int;
pub const SF_TAGS: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 11 as ::core::ffi::c_int;
pub const SF_OSC8_OPEN: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 12 as ::core::ffi::c_int;
pub const TOGGLE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[export_name = "rboxc_less_every_first_cmd"]
pub static mut every_first_cmd: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_less_new_file"]
pub static mut new_file: lbool = lbool::LFALSE;
#[export_name = "rboxc_less_is_tty"]
pub static mut is_tty: lbool = lbool::LFALSE;
#[export_name = "rboxc_less_curr_ifile"]
pub static mut curr_ifile: *mut ::core::ffi::c_void = NULL_IFILE;
#[export_name = "rboxc_less_old_ifile"]
pub static mut old_ifile: *mut ::core::ffi::c_void = NULL_IFILE;
#[export_name = "rboxc_less_initial_scrpos"]
pub static mut initial_scrpos: scrpos = scrpos { pos: 0, ln: 0 };
#[export_name = "rboxc_less_start_attnpos"]
pub static mut start_attnpos: POSITION = NULL_POSITION;
#[export_name = "rboxc_less_end_attnpos"]
pub static mut end_attnpos: POSITION = NULL_POSITION;
#[export_name = "rboxc_less_wscroll"]
pub static mut wscroll: ::core::ffi::c_int = 0;
#[export_name = "rboxc_less_progname"]
pub static mut progname: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_less_quitting"]
pub static mut quitting: lbool = lbool::LFALSE;
#[export_name = "rboxc_less_dohelp"]
pub static mut dohelp: lbool = lbool::LFALSE;
#[export_name = "rboxc_less_init_header"]
pub static mut init_header: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_less_no_config"]
pub static mut no_config: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut secure_allow_features: ::core::ffi::c_int = 0;
#[export_name = "rboxc_less_logfile"]
pub static mut logfile: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_less_force_logfile"]
pub static mut force_logfile: lbool = lbool::LFALSE;
#[export_name = "rboxc_less_namelogfile"]
pub static mut namelogfile: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_less_editor"]
pub static mut editor: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_less_editproto"]
pub static mut editproto: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_less_less_start_time"]
pub static mut less_start_time: time_t = 0;
#[export_name = "rboxc_less_one_screen"]
pub static mut one_screen: lbool = lbool::LFALSE;
unsafe extern "C" fn init_secure() {
    static mut security_features: [csl_bitmap_def; 12] = [
        csl_bitmap_def {
            bit_name: b"edit\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_EDIT,
        },
        csl_bitmap_def {
            bit_name: b"examine\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_EXAMINE,
        },
        csl_bitmap_def {
            bit_name: b"glob\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_GLOB,
        },
        csl_bitmap_def {
            bit_name: b"history\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_HISTORY,
        },
        csl_bitmap_def {
            bit_name: b"lesskey\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_LESSKEY,
        },
        csl_bitmap_def {
            bit_name: b"lessopen\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_LESSOPEN,
        },
        csl_bitmap_def {
            bit_name: b"logfile\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_LOGFILE,
        },
        csl_bitmap_def {
            bit_name: b"osc8\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_OSC8_OPEN,
        },
        csl_bitmap_def {
            bit_name: b"pipe\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_PIPE,
        },
        csl_bitmap_def {
            bit_name: b"shell\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_SHELL,
        },
        csl_bitmap_def {
            bit_name: b"stop\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_STOP,
        },
        csl_bitmap_def {
            bit_name: b"tags\0".as_ptr() as *const ::core::ffi::c_char,
            bit_value: SF_TAGS,
        },
    ];
    let mut str: *const ::core::ffi::c_char =
        lgetenv(b"LESSSECURE\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(str).0 != 0 {
        secure_allow_features = !(0 as ::core::ffi::c_int);
    } else {
        secure_allow_features = 0 as ::core::ffi::c_int;
    }
    str = lgetenv(b"LESSSECURE_ALLOW\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(str).0 == 0 {
        secure_allow_features = parse_csl_bitmap(
            str,
            &raw mut security_features as *mut csl_bitmap_def,
            ::core::mem::size_of::<[csl_bitmap_def; 12]>()
                .wrapping_div(::core::mem::size_of::<csl_bitmap_def>())
                as ::core::ffi::c_int,
            b"LESSSECURE_ALLOW\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn rboxc_less_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ifile: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    let mut s: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut xfiles: xbuffer = xbuffer {
        data: ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        end: 0,
        size: 0,
        init_size: 0,
    };
    let mut files: *const ::core::ffi::c_int = ::core::ptr::null::<::core::ffi::c_int>();
    let mut num_files: size_t = 0;
    let mut f: size_t = 0;
    let mut end_opts: lbool = lbool::LFALSE;
    let mut posixly_correct: lbool = lbool::LFALSE;
    no_config = getenv(b"LESSNOCONFIG\0".as_ptr() as *const ::core::ffi::c_char);
    let c2rust_fresh0 = argv;
    argv = argv.offset(1);
    progname = *c2rust_fresh0;
    argc -= 1;
    init_secure();
    is_tty = lbool(
        (isatty(1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
            as ::core::ffi::c_uint,
    );
    init_mark();
    init_cmds();
    init_poll();
    init_charset();
    init_line();
    init_cmdhist();
    init_option();
    init_search();
    if strcmp(
        last_component(progname),
        b"more\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
        && isnullenv(lgetenv(
            b"LESS_IS_MORE\0".as_ptr() as *const ::core::ffi::c_char
        ))
        .0 != 0
    {
        less_is_more = 1 as ::core::ffi::c_int;
    }
    init_prompt();
    init_unsupport();
    s = lgetenv(if less_is_more != 0 {
        b"MORE\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        b"LESS\0".as_ptr() as *const ::core::ffi::c_char
    });
    if !s.is_null() {
        scan_option(s, lbool::LTRUE);
    }
    xbuf_init(&raw mut xfiles);
    posixly_correct = lbool(
        !lgetenv(b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char).is_null()
            as ::core::ffi::c_int as ::core::ffi::c_uint,
    );
    i = 0 as ::core::ffi::c_int;
    while i < argc {
        if strcmp(
            *argv.offset(i as isize),
            b"--\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            end_opts = lbool::LTRUE;
        } else if end_opts.0 == 0
            && ((*(*argv.offset(i as isize)).offset(0isize) as ::core::ffi::c_int
                == '-' as ::core::ffi::c_int
                || *(*argv.offset(i as isize)).offset(0isize) as ::core::ffi::c_int
                    == '+' as ::core::ffi::c_int)
                && *(*argv.offset(i as isize)).offset(1isize) as ::core::ffi::c_int
                    != '\0' as ::core::ffi::c_int
                || isoptpending().0 != 0)
        {
            scan_option(*argv.offset(i as isize), lbool::LFALSE);
        } else {
            if posixly_correct.0 != 0 {
                end_opts = lbool::LTRUE;
            }
            xbuf_add_data(
                &raw mut xfiles,
                &raw mut i as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>(),
            );
        }
        i += 1;
    }
    if isoptpending().0 != 0 {
        nopendopt();
        quit(QUIT_OK);
    }
    if is_tty.0 != 0 {
        get_term();
    }
    expand_cmd_tables();
    editor = lgetenv(b"VISUAL\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(editor).0 != 0 {
        editor = lgetenv(b"EDITOR\0".as_ptr() as *const ::core::ffi::c_char);
        if isnullenv(editor).0 != 0 {
            editor = EDIT_PGM.as_ptr();
        }
    }
    editproto = lgetenv(b"LESSEDIT\0".as_ptr() as *const ::core::ffi::c_char);
    if isnullenv(editproto).0 != 0 {
        editproto = b"%E ?lm+%lm. %g\0".as_ptr() as *const ::core::ffi::c_char;
    }
    ifile = NULL_IFILE;
    if dohelp.0 != 0 {
        ifile = get_ifile(FAKE_HELPFILE.as_ptr(), ifile);
    }
    files = xfiles.data as *const ::core::ffi::c_int;
    num_files = xfiles
        .end
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>());
    f = 0 as size_t;
    while f < num_files {
        get_ifile(*argv.offset(*files.offset(f as isize) as isize), ifile);
        ifile = prev_ifile(NULL_IFILE);
        f = f.wrapping_add(1);
    }
    xbuf_deinit(&raw mut xfiles);
    if is_tty.0 == 0 {
        if edit_first() == 0 as ::core::ffi::c_int {
            set_output(1 as ::core::ffi::c_int, lbool::LTRUE);
            loop {
                cat_file();
                if edit_next(1 as ::core::ffi::c_int) != 0 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        quit(QUIT_OK);
    }
    if missing_cap.0 != 0 && know_dumb == 0 {
        error(
            b"WARNING: terminal is not fully functional\0".as_ptr() as *const ::core::ffi::c_char,
            NULL_PARG,
        );
    }
    open_getchr();
    raw_mode(1 as ::core::ffi::c_int);
    init_signals(1 as ::core::ffi::c_int);
    less_start_time = get_time();
    if !tagoption.is_null()
        || strcmp(tags, b"-\0".as_ptr() as *const ::core::ffi::c_char) == 0 as ::core::ffi::c_int
    {
        if nifile() > 0 as ::core::ffi::c_int {
            error(
                b"No filenames allowed with -t option\0".as_ptr() as *const ::core::ffi::c_char,
                NULL_PARG,
            );
            quit(QUIT_ERROR);
        }
        findtag(tagoption);
        if edit_tagfile() != 0 {
            quit(QUIT_ERROR);
        }
        initial_scrpos.pos = tagsearch();
        if initial_scrpos.pos == NULL_POSITION {
            quit(QUIT_ERROR);
        }
        initial_scrpos.ln = jump_sline;
    } else {
        if edit_first() != 0 {
            quit(QUIT_ERROR);
        }
        if quit_if_one_screen != 0 {
            if nifile() > 1 as ::core::ffi::c_int {
                quit_if_one_screen = lbool::LFALSE.0 as ::core::ffi::c_int;
            } else if no_init == 0 {
                one_screen = get_one_screen();
            }
        }
    }
    if !init_header.is_null() {
        opt_header(TOGGLE, init_header);
        free(init_header as *mut ::core::ffi::c_void);
        init_header = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if errmsgs > 0 as ::core::ffi::c_int {
        less_printf(
            b"Press RETURN to continue \0".as_ptr() as *const ::core::ffi::c_char,
            NULL_PARG,
        );
        get_return();
        putchr('\n' as ::core::ffi::c_int);
    }
    set_output(1 as ::core::ffi::c_int, lbool::LFALSE);
    commands();
    quit(QUIT_OK);
    return 0 as ::core::ffi::c_int;
}
#[export_name = "rboxc_less_saven"]
pub unsafe extern "C" fn saven(
    mut s: *const ::core::ffi::c_char,
    mut n: size_t,
) -> *mut ::core::ffi::c_char {
    let mut p: *mut ::core::ffi::c_char = ecalloc(
        n.wrapping_add(1 as size_t),
        ::core::mem::size_of::<::core::ffi::c_char>(),
    ) as *mut ::core::ffi::c_char;
    strncpy(p, s, n);
    *p.offset(n as isize) = '\0' as ::core::ffi::c_char;
    return p;
}
#[export_name = "rboxc_less_save"]
pub unsafe extern "C" fn save(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    return saven(s, strlen(s));
}
#[export_name = "rboxc_less_out_of_memory"]
pub unsafe extern "C" fn out_of_memory() {
    error(
        b"Cannot allocate memory\0".as_ptr() as *const ::core::ffi::c_char,
        NULL_PARG,
    );
    quit(QUIT_ERROR);
}
#[export_name = "rboxc_less_ecalloc"]
pub unsafe extern "C" fn ecalloc(mut count: size_t, mut size: size_t) -> *mut ::core::ffi::c_void {
    let mut p: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
    p = calloc(count, size);
    if p.is_null() {
        out_of_memory();
    }
    return p;
}
#[export_name = "rboxc_less_skipsp"]
pub unsafe extern "C" fn skipsp(mut s: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    while *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *s as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        s = s.offset(1);
    }
    return s;
}
#[export_name = "rboxc_less_skipspc"]
pub unsafe extern "C" fn skipspc(mut s: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char {
    while *s as ::core::ffi::c_int == ' ' as ::core::ffi::c_int
        || *s as ::core::ffi::c_int == '\t' as ::core::ffi::c_int
    {
        s = s.offset(1);
    }
    return s;
}
#[export_name = "rboxc_less_sprefix"]
pub unsafe extern "C" fn sprefix(
    mut ps: *const ::core::ffi::c_char,
    mut s: *const ::core::ffi::c_char,
    mut uppercase: ::core::ffi::c_int,
) -> size_t {
    let mut c: ::core::ffi::c_char = 0;
    let mut sc: ::core::ffi::c_char = 0;
    let mut len: size_t = 0 as size_t;
    while *s as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
        c = *ps;
        if uppercase != 0 {
            if len == 0 as size_t
                && (c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                    && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int)
            {
                return 0 as size_t;
            }
            if c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int
            {
                c = (c as ::core::ffi::c_int - 'A' as ::core::ffi::c_int
                    + 'a' as ::core::ffi::c_int) as ::core::ffi::c_char;
            }
        }
        sc = *s;
        if len > 0 as size_t
            && (sc as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && sc as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int)
        {
            sc = (sc as ::core::ffi::c_int - 'A' as ::core::ffi::c_int + 'a' as ::core::ffi::c_int)
                as ::core::ffi::c_char;
        }
        if c as ::core::ffi::c_int != sc as ::core::ffi::c_int {
            break;
        }
        len = len.wrapping_add(1);
        s = s.offset(1);
        ps = ps.offset(1);
    }
    return len;
}
#[export_name = "rboxc_less_quit"]
pub unsafe extern "C" fn quit(mut status: ::core::ffi::c_int) {
    static mut save_status: ::core::ffi::c_int = 0;
    if status < 0 as ::core::ffi::c_int {
        status = save_status;
    } else {
        save_status = status;
    }
    quitting = lbool::LTRUE;
    check_altpipe_error();
    if interactive().0 != 0 {
        clear_bot();
    }
    term_deinit();
    flush();
    if redraw_on_quit != 0 && term_addrs != 0 {
        first_time = lbool::LTRUE;
        repaint();
        flush();
    }
    edit(NULL as *mut ::core::ffi::c_char);
    save_cmdhist();
    raw_mode(0 as ::core::ffi::c_int);
    close_getchr();
    exit(status);
}
#[export_name = "rboxc_less_secure_allow"]
pub unsafe extern "C" fn secure_allow(mut features: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return (secure_allow_features & features == features) as ::core::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn single_binary_main_less(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_less_main_inner(argc, argv.cast())
}
