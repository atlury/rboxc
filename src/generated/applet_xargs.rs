// Generated from pinned GNU Findutils 4.11.0 by scripts/translate-findutils.py.
// Source SHA-256: 7bd9374c030fdc420885a9b7ab30e4c70e4ee0a09c835cba0dd87369c887503e
/* xargs -- build and execute command lines from standard input
   Copyright (C) 1990-2026 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn __assert_single_arg(_: bool) -> bool;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_xargs_rpl_fcntl"]
    fn rpl_fcntl(fd: ::core::ffi::c_int, action: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn sigemptyset(__set: *mut sigset_t) -> ::core::ffi::c_int;
    fn sigaction(
        __sig: ::core::ffi::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn dup2(__fd: ::core::ffi::c_int, __fd2: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn getpid() -> __pid_t;
    fn fork() -> __pid_t;
    fn isatty(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fdopen(__fd: ::core::ffi::c_int, __modes: *const ::core::ffi::c_char) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn getc(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn strtoul(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn atexit(__func: Option<unsafe extern "C" fn() -> ()>) -> ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn setenv(
        __name: *const ::core::ffi::c_char,
        __value: *const ::core::ffi::c_char,
        __replace: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn unsetenv(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    #[link_name = "rboxc_findutils_xargs_close_stdin"]
    fn close_stdin();
    #[link_name = "rboxc_findutils_xargs_open_safer"]
    fn open_safer(_: *const ::core::ffi::c_char, _: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_xargs_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_xargs_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    #[link_name = "rboxc_findutils_xargs_quotearg_n_style"]
    fn quotearg_n_style(
        n: ::core::ffi::c_int,
        s: quoting_style,
        arg: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_findutils_xargs_safe_read"]
    fn safe_read(fd: ::core::ffi::c_int, buf: *mut ::core::ffi::c_void, count: idx_t) -> ptrdiff_t;
    #[link_name = "rboxc_findutils_xargs_pipe_safer"]
    fn pipe_safer(_: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_xargs_xmalloc"]
    fn xmalloc(s: size_t) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_findutils_xargs_x2nrealloc"]
    fn x2nrealloc(
        p: *mut ::core::ffi::c_void,
        pn: *mut size_t,
        s: size_t,
    ) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_findutils_xargs_bc_size_of_environment"]
    fn bc_size_of_environment() -> size_t;
    #[link_name = "rboxc_findutils_xargs_bc_do_insert"]
    fn bc_do_insert(
        ctl: *mut buildcmd_control,
        state: *mut buildcmd_state,
        arg: *mut ::core::ffi::c_char,
        arglen: size_t,
        prefix: *const ::core::ffi::c_char,
        pfxlen: size_t,
        linebuf_0: *const ::core::ffi::c_char,
        lblen: size_t,
        initial_args_0: ::core::ffi::c_int,
    );
    #[link_name = "rboxc_findutils_xargs_bc_do_exec"]
    fn bc_do_exec(ctl: *mut buildcmd_control, state: *mut buildcmd_state);
    #[link_name = "rboxc_findutils_xargs_bc_push_arg"]
    fn bc_push_arg(
        ctl: *mut buildcmd_control,
        state: *mut buildcmd_state,
        arg: *const ::core::ffi::c_char,
        len: size_t,
        prefix: *const ::core::ffi::c_char,
        pfxlen: size_t,
        initial_args_0: ::core::ffi::c_int,
    );
    #[link_name = "rboxc_findutils_xargs_bc_init_controlinfo"]
    fn bc_init_controlinfo(ctl: *mut buildcmd_control, arglen_headroom: size_t) -> BC_INIT_STATUS;
    #[link_name = "rboxc_findutils_xargs_bc_use_sensible_arg_max"]
    fn bc_use_sensible_arg_max(ctl: *mut buildcmd_control);
    #[link_name = "rboxc_findutils_xargs_bc_clear_args"]
    fn bc_clear_args(ctl: *const buildcmd_control, state: *mut buildcmd_state);
    #[link_name = "rboxc_findutils_xargs_bc_args_exceed_testing_limit"]
    fn bc_args_exceed_testing_limit(argv: *mut *mut ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_findutils_xargs_explain_how_to_report_bugs"]
    fn explain_how_to_report_bugs(
        f: *mut FILE,
        program_name_0: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_xargs_remember_non_cloexec_fds"]
    fn remember_non_cloexec_fds();
    #[link_name = "rboxc_findutils_xargs_complain_about_leaky_fds"]
    fn complain_about_leaky_fds();
    #[link_name = "rboxc_findutils_xargs_fd_leak_check_is_enabled"]
    fn fd_leak_check_is_enabled() -> bool;
    #[link_name = "rboxc_findutils_xargs_open_cloexec"]
    fn open_cloexec(
        path: *const ::core::ffi::c_char,
        flags: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_findutils_xargs_display_findutils_version"]
    fn display_findutils_version(official_name: *const ::core::ffi::c_char);
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
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __uid_t = ::core::ffi::c_uint;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __clock_t = ::core::ffi::c_long;
pub type __sig_atomic_t = ::core::ffi::c_int;
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
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type size_t = usize;
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type sig_atomic_t = __sig_atomic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: ::core::ffi::c_int,
    pub sival_ptr: *mut ::core::ffi::c_void,
}
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::core::ffi::c_int,
    pub si_errno: ::core::ffi::c_int,
    pub si_code: ::core::ffi::c_int,
    pub __pad0: ::core::ffi::c_int,
    pub _sifields: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub _pad: [::core::ffi::c_int; 28],
    pub _kill: C2Rust_Unnamed_9,
    pub _timer: C2Rust_Unnamed_8,
    pub _rt: C2Rust_Unnamed_7,
    pub _sigchld: C2Rust_Unnamed_6,
    pub _sigfault: C2Rust_Unnamed_3,
    pub _sigpoll: C2Rust_Unnamed_2,
    pub _sigsys: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub _call_addr: *mut ::core::ffi::c_void,
    pub _syscall: ::core::ffi::c_int,
    pub _arch: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub si_band: ::core::ffi::c_long,
    pub si_fd: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub si_addr: *mut ::core::ffi::c_void,
    pub si_addr_lsb: ::core::ffi::c_short,
    pub _bounds: C2Rust_Unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub _addr_bnd: C2Rust_Unnamed_5,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub _lower: *mut ::core::ffi::c_void,
    pub _upper: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: ::core::ffi::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub si_tid: ::core::ffi::c_int,
    pub si_overrun: ::core::ffi::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2Rust_Unnamed_10,
    pub sa_mask: __sigset_t,
    pub sa_flags: ::core::ffi::c_int,
    pub sa_restorer: Option<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option<
        unsafe extern "C" fn(::core::ffi::c_int, *mut siginfo_t, *mut ::core::ffi::c_void) -> (),
    >,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_11(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_11 {
    pub const _SC_ARG_MAX: Self = Self(0);
    pub const _SC_CHILD_MAX: Self = Self(1);
    pub const _SC_CLK_TCK: Self = Self(2);
    pub const _SC_NGROUPS_MAX: Self = Self(3);
    pub const _SC_OPEN_MAX: Self = Self(4);
    pub const _SC_STREAM_MAX: Self = Self(5);
    pub const _SC_TZNAME_MAX: Self = Self(6);
    pub const _SC_JOB_CONTROL: Self = Self(7);
    pub const _SC_SAVED_IDS: Self = Self(8);
    pub const _SC_REALTIME_SIGNALS: Self = Self(9);
    pub const _SC_PRIORITY_SCHEDULING: Self = Self(10);
    pub const _SC_TIMERS: Self = Self(11);
    pub const _SC_ASYNCHRONOUS_IO: Self = Self(12);
    pub const _SC_PRIORITIZED_IO: Self = Self(13);
    pub const _SC_SYNCHRONIZED_IO: Self = Self(14);
    pub const _SC_FSYNC: Self = Self(15);
    pub const _SC_MAPPED_FILES: Self = Self(16);
    pub const _SC_MEMLOCK: Self = Self(17);
    pub const _SC_MEMLOCK_RANGE: Self = Self(18);
    pub const _SC_MEMORY_PROTECTION: Self = Self(19);
    pub const _SC_MESSAGE_PASSING: Self = Self(20);
    pub const _SC_SEMAPHORES: Self = Self(21);
    pub const _SC_SHARED_MEMORY_OBJECTS: Self = Self(22);
    pub const _SC_AIO_LISTIO_MAX: Self = Self(23);
    pub const _SC_AIO_MAX: Self = Self(24);
    pub const _SC_AIO_PRIO_DELTA_MAX: Self = Self(25);
    pub const _SC_DELAYTIMER_MAX: Self = Self(26);
    pub const _SC_MQ_OPEN_MAX: Self = Self(27);
    pub const _SC_MQ_PRIO_MAX: Self = Self(28);
    pub const _SC_VERSION: Self = Self(29);
    pub const _SC_PAGESIZE: Self = Self(30);
    pub const _SC_RTSIG_MAX: Self = Self(31);
    pub const _SC_SEM_NSEMS_MAX: Self = Self(32);
    pub const _SC_SEM_VALUE_MAX: Self = Self(33);
    pub const _SC_SIGQUEUE_MAX: Self = Self(34);
    pub const _SC_TIMER_MAX: Self = Self(35);
    pub const _SC_BC_BASE_MAX: Self = Self(36);
    pub const _SC_BC_DIM_MAX: Self = Self(37);
    pub const _SC_BC_SCALE_MAX: Self = Self(38);
    pub const _SC_BC_STRING_MAX: Self = Self(39);
    pub const _SC_COLL_WEIGHTS_MAX: Self = Self(40);
    pub const _SC_EQUIV_CLASS_MAX: Self = Self(41);
    pub const _SC_EXPR_NEST_MAX: Self = Self(42);
    pub const _SC_LINE_MAX: Self = Self(43);
    pub const _SC_RE_DUP_MAX: Self = Self(44);
    pub const _SC_CHARCLASS_NAME_MAX: Self = Self(45);
    pub const _SC_2_VERSION: Self = Self(46);
    pub const _SC_2_C_BIND: Self = Self(47);
    pub const _SC_2_C_DEV: Self = Self(48);
    pub const _SC_2_FORT_DEV: Self = Self(49);
    pub const _SC_2_FORT_RUN: Self = Self(50);
    pub const _SC_2_SW_DEV: Self = Self(51);
    pub const _SC_2_LOCALEDEF: Self = Self(52);
    pub const _SC_PII: Self = Self(53);
    pub const _SC_PII_XTI: Self = Self(54);
    pub const _SC_PII_SOCKET: Self = Self(55);
    pub const _SC_PII_INTERNET: Self = Self(56);
    pub const _SC_PII_OSI: Self = Self(57);
    pub const _SC_POLL: Self = Self(58);
    pub const _SC_SELECT: Self = Self(59);
    pub const _SC_UIO_MAXIOV: Self = Self(60);
    pub const _SC_IOV_MAX: Self = Self(60);
    pub const _SC_PII_INTERNET_STREAM: Self = Self(61);
    pub const _SC_PII_INTERNET_DGRAM: Self = Self(62);
    pub const _SC_PII_OSI_COTS: Self = Self(63);
    pub const _SC_PII_OSI_CLTS: Self = Self(64);
    pub const _SC_PII_OSI_M: Self = Self(65);
    pub const _SC_T_IOV_MAX: Self = Self(66);
    pub const _SC_THREADS: Self = Self(67);
    pub const _SC_THREAD_SAFE_FUNCTIONS: Self = Self(68);
    pub const _SC_GETGR_R_SIZE_MAX: Self = Self(69);
    pub const _SC_GETPW_R_SIZE_MAX: Self = Self(70);
    pub const _SC_LOGIN_NAME_MAX: Self = Self(71);
    pub const _SC_TTY_NAME_MAX: Self = Self(72);
    pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: Self = Self(73);
    pub const _SC_THREAD_KEYS_MAX: Self = Self(74);
    pub const _SC_THREAD_STACK_MIN: Self = Self(75);
    pub const _SC_THREAD_THREADS_MAX: Self = Self(76);
    pub const _SC_THREAD_ATTR_STACKADDR: Self = Self(77);
    pub const _SC_THREAD_ATTR_STACKSIZE: Self = Self(78);
    pub const _SC_THREAD_PRIORITY_SCHEDULING: Self = Self(79);
    pub const _SC_THREAD_PRIO_INHERIT: Self = Self(80);
    pub const _SC_THREAD_PRIO_PROTECT: Self = Self(81);
    pub const _SC_THREAD_PROCESS_SHARED: Self = Self(82);
    pub const _SC_NPROCESSORS_CONF: Self = Self(83);
    pub const _SC_NPROCESSORS_ONLN: Self = Self(84);
    pub const _SC_PHYS_PAGES: Self = Self(85);
    pub const _SC_AVPHYS_PAGES: Self = Self(86);
    pub const _SC_ATEXIT_MAX: Self = Self(87);
    pub const _SC_PASS_MAX: Self = Self(88);
    pub const _SC_XOPEN_VERSION: Self = Self(89);
    pub const _SC_XOPEN_XCU_VERSION: Self = Self(90);
    pub const _SC_XOPEN_UNIX: Self = Self(91);
    pub const _SC_XOPEN_CRYPT: Self = Self(92);
    pub const _SC_XOPEN_ENH_I18N: Self = Self(93);
    pub const _SC_XOPEN_SHM: Self = Self(94);
    pub const _SC_2_CHAR_TERM: Self = Self(95);
    pub const _SC_2_C_VERSION: Self = Self(96);
    pub const _SC_2_UPE: Self = Self(97);
    pub const _SC_XOPEN_XPG2: Self = Self(98);
    pub const _SC_XOPEN_XPG3: Self = Self(99);
    pub const _SC_XOPEN_XPG4: Self = Self(100);
    pub const _SC_CHAR_BIT: Self = Self(101);
    pub const _SC_CHAR_MAX: Self = Self(102);
    pub const _SC_CHAR_MIN: Self = Self(103);
    pub const _SC_INT_MAX: Self = Self(104);
    pub const _SC_INT_MIN: Self = Self(105);
    pub const _SC_LONG_BIT: Self = Self(106);
    pub const _SC_WORD_BIT: Self = Self(107);
    pub const _SC_MB_LEN_MAX: Self = Self(108);
    pub const _SC_NZERO: Self = Self(109);
    pub const _SC_SSIZE_MAX: Self = Self(110);
    pub const _SC_SCHAR_MAX: Self = Self(111);
    pub const _SC_SCHAR_MIN: Self = Self(112);
    pub const _SC_SHRT_MAX: Self = Self(113);
    pub const _SC_SHRT_MIN: Self = Self(114);
    pub const _SC_UCHAR_MAX: Self = Self(115);
    pub const _SC_UINT_MAX: Self = Self(116);
    pub const _SC_ULONG_MAX: Self = Self(117);
    pub const _SC_USHRT_MAX: Self = Self(118);
    pub const _SC_NL_ARGMAX: Self = Self(119);
    pub const _SC_NL_LANGMAX: Self = Self(120);
    pub const _SC_NL_MSGMAX: Self = Self(121);
    pub const _SC_NL_NMAX: Self = Self(122);
    pub const _SC_NL_SETMAX: Self = Self(123);
    pub const _SC_NL_TEXTMAX: Self = Self(124);
    pub const _SC_XBS5_ILP32_OFF32: Self = Self(125);
    pub const _SC_XBS5_ILP32_OFFBIG: Self = Self(126);
    pub const _SC_XBS5_LP64_OFF64: Self = Self(127);
    pub const _SC_XBS5_LPBIG_OFFBIG: Self = Self(128);
    pub const _SC_XOPEN_LEGACY: Self = Self(129);
    pub const _SC_XOPEN_REALTIME: Self = Self(130);
    pub const _SC_XOPEN_REALTIME_THREADS: Self = Self(131);
    pub const _SC_ADVISORY_INFO: Self = Self(132);
    pub const _SC_BARRIERS: Self = Self(133);
    pub const _SC_BASE: Self = Self(134);
    pub const _SC_C_LANG_SUPPORT: Self = Self(135);
    pub const _SC_C_LANG_SUPPORT_R: Self = Self(136);
    pub const _SC_CLOCK_SELECTION: Self = Self(137);
    pub const _SC_CPUTIME: Self = Self(138);
    pub const _SC_THREAD_CPUTIME: Self = Self(139);
    pub const _SC_DEVICE_IO: Self = Self(140);
    pub const _SC_DEVICE_SPECIFIC: Self = Self(141);
    pub const _SC_DEVICE_SPECIFIC_R: Self = Self(142);
    pub const _SC_FD_MGMT: Self = Self(143);
    pub const _SC_FIFO: Self = Self(144);
    pub const _SC_PIPE: Self = Self(145);
    pub const _SC_FILE_ATTRIBUTES: Self = Self(146);
    pub const _SC_FILE_LOCKING: Self = Self(147);
    pub const _SC_FILE_SYSTEM: Self = Self(148);
    pub const _SC_MONOTONIC_CLOCK: Self = Self(149);
    pub const _SC_MULTI_PROCESS: Self = Self(150);
    pub const _SC_SINGLE_PROCESS: Self = Self(151);
    pub const _SC_NETWORKING: Self = Self(152);
    pub const _SC_READER_WRITER_LOCKS: Self = Self(153);
    pub const _SC_SPIN_LOCKS: Self = Self(154);
    pub const _SC_REGEXP: Self = Self(155);
    pub const _SC_REGEX_VERSION: Self = Self(156);
    pub const _SC_SHELL: Self = Self(157);
    pub const _SC_SIGNALS: Self = Self(158);
    pub const _SC_SPAWN: Self = Self(159);
    pub const _SC_SPORADIC_SERVER: Self = Self(160);
    pub const _SC_THREAD_SPORADIC_SERVER: Self = Self(161);
    pub const _SC_SYSTEM_DATABASE: Self = Self(162);
    pub const _SC_SYSTEM_DATABASE_R: Self = Self(163);
    pub const _SC_TIMEOUTS: Self = Self(164);
    pub const _SC_TYPED_MEMORY_OBJECTS: Self = Self(165);
    pub const _SC_USER_GROUPS: Self = Self(166);
    pub const _SC_USER_GROUPS_R: Self = Self(167);
    pub const _SC_2_PBS: Self = Self(168);
    pub const _SC_2_PBS_ACCOUNTING: Self = Self(169);
    pub const _SC_2_PBS_LOCATE: Self = Self(170);
    pub const _SC_2_PBS_MESSAGE: Self = Self(171);
    pub const _SC_2_PBS_TRACK: Self = Self(172);
    pub const _SC_SYMLOOP_MAX: Self = Self(173);
    pub const _SC_STREAMS: Self = Self(174);
    pub const _SC_2_PBS_CHECKPOINT: Self = Self(175);
    pub const _SC_V6_ILP32_OFF32: Self = Self(176);
    pub const _SC_V6_ILP32_OFFBIG: Self = Self(177);
    pub const _SC_V6_LP64_OFF64: Self = Self(178);
    pub const _SC_V6_LPBIG_OFFBIG: Self = Self(179);
    pub const _SC_HOST_NAME_MAX: Self = Self(180);
    pub const _SC_TRACE: Self = Self(181);
    pub const _SC_TRACE_EVENT_FILTER: Self = Self(182);
    pub const _SC_TRACE_INHERIT: Self = Self(183);
    pub const _SC_TRACE_LOG: Self = Self(184);
    pub const _SC_LEVEL1_ICACHE_SIZE: Self = Self(185);
    pub const _SC_LEVEL1_ICACHE_ASSOC: Self = Self(186);
    pub const _SC_LEVEL1_ICACHE_LINESIZE: Self = Self(187);
    pub const _SC_LEVEL1_DCACHE_SIZE: Self = Self(188);
    pub const _SC_LEVEL1_DCACHE_ASSOC: Self = Self(189);
    pub const _SC_LEVEL1_DCACHE_LINESIZE: Self = Self(190);
    pub const _SC_LEVEL2_CACHE_SIZE: Self = Self(191);
    pub const _SC_LEVEL2_CACHE_ASSOC: Self = Self(192);
    pub const _SC_LEVEL2_CACHE_LINESIZE: Self = Self(193);
    pub const _SC_LEVEL3_CACHE_SIZE: Self = Self(194);
    pub const _SC_LEVEL3_CACHE_ASSOC: Self = Self(195);
    pub const _SC_LEVEL3_CACHE_LINESIZE: Self = Self(196);
    pub const _SC_LEVEL4_CACHE_SIZE: Self = Self(197);
    pub const _SC_LEVEL4_CACHE_ASSOC: Self = Self(198);
    pub const _SC_LEVEL4_CACHE_LINESIZE: Self = Self(199);
    pub const _SC_IPV6: Self = Self(235);
    pub const _SC_RAW_SOCKETS: Self = Self(236);
    pub const _SC_V7_ILP32_OFF32: Self = Self(237);
    pub const _SC_V7_ILP32_OFFBIG: Self = Self(238);
    pub const _SC_V7_LP64_OFF64: Self = Self(239);
    pub const _SC_V7_LPBIG_OFFBIG: Self = Self(240);
    pub const _SC_SS_REPL_MAX: Self = Self(241);
    pub const _SC_TRACE_EVENT_NAME_MAX: Self = Self(242);
    pub const _SC_TRACE_NAME_MAX: Self = Self(243);
    pub const _SC_TRACE_SYS_MAX: Self = Self(244);
    pub const _SC_TRACE_USER_EVENT_MAX: Self = Self(245);
    pub const _SC_XOPEN_STREAMS: Self = Self(246);
    pub const _SC_THREAD_ROBUST_PRIO_INHERIT: Self = Self(247);
    pub const _SC_THREAD_ROBUST_PRIO_PROTECT: Self = Self(248);
    pub const _SC_MINSIGSTKSZ: Self = Self(249);
    pub const _SC_SIGSTKSZ: Self = Self(250);
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
pub type ptrdiff_t = isize;
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
pub type idx_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_state {
    pub cmd_argc: size_t,
    pub cmd_argv: *mut *mut ::core::ffi::c_char,
    pub cmd_argv_alloc: size_t,
    pub argbuf: *mut ::core::ffi::c_char,
    pub cmd_argv_chars: size_t,
    pub cmd_initial_argv_chars: size_t,
    pub usercontext: *mut ::core::ffi::c_void,
    pub todo: ::core::ffi::c_int,
    pub dir_fd: ::core::ffi::c_int,
    pub largest_successful_arg_count: size_t,
    pub smallest_failed_arg_count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buildcmd_control {
    pub exit_if_size_exceeded: ::core::ffi::c_int,
    pub posix_arg_size_max: size_t,
    pub posix_arg_size_min: size_t,
    pub arg_max: size_t,
    pub max_arg_count: size_t,
    pub rplen: size_t,
    pub replace_pat: *const ::core::ffi::c_char,
    pub initial_argc: size_t,
    pub exec_callback: Option<
        unsafe extern "C" fn(
            *mut buildcmd_control,
            *mut ::core::ffi::c_void,
            ::core::ffi::c_int,
            *mut *mut ::core::ffi::c_char,
        ) -> ::core::ffi::c_int,
    >,
    pub lines_per_exec: ::core::ffi::c_ulong,
    pub args_per_exec: size_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct BC_INIT_STATUS(pub ::core::ffi::c_uint);
impl BC_INIT_STATUS {
    pub const BC_INIT_OK: Self = Self(0);
    pub const BC_INIT_ENV_TOO_BIG: Self = Self(1);
    pub const BC_INIT_CANNOT_ACCOMODATE_HEADROOM: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct LongOptionIdentifier(pub ::core::ffi::c_uint);
impl LongOptionIdentifier {
    pub const PROCESS_SLOT_VAR: Self = Self(128);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct XargsStatusValues(pub ::core::ffi::c_uint);
impl XargsStatusValues {
    pub const XARGS_EXIT_CLIENT_EXIT_NONZERO: Self = Self(123);
    pub const XARGS_EXIT_CLIENT_EXIT_255: Self = Self(124);
    pub const XARGS_EXIT_CLIENT_FATAL_SIG: Self = Self(125);
    pub const XARGS_EXIT_COMMAND_CANNOT_BE_RUN: Self = Self(126);
    pub const XARGS_EXIT_COMMAND_NOT_FOUND: Self = Self(127);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ClientStatusValues(pub ::core::ffi::c_uint);
impl ClientStatusValues {
    pub const CHILD_EXIT_PLEASE_STOP_IMMEDIATELY: Self = Self(255);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct read_line_state(pub ::core::ffi::c_uint);
impl read_line_state {
    pub const NORM: Self = Self(0);
    pub const SPACE: Self = Self(1);
    pub const QUOTE: Self = Self(2);
    pub const BACKSLASH: Self = Self(3);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_12(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_12 {
    pub const XARGS_POSIX_HEADROOM: Self = Self(2048);
}
pub const __STDC_LIMIT_MACROS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const E2BIG: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const ERANGE: ::core::ffi::c_int = 34 as ::core::ffi::c_int;
pub const UCHAR_MAX: ::core::ffi::c_int =
    __SCHAR_MAX__ * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int;
pub const ULONG_MAX: ::core::ffi::c_ulong = (__LONG_MAX__ as ::core::ffi::c_ulong)
    .wrapping_mul(2 as ::core::ffi::c_ulong)
    .wrapping_add(1 as ::core::ffi::c_ulong);
pub const SIG_ATOMIC_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const F_SETFD: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const FD_CLOEXEC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const optional_argument: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGUSR2: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const SA_RESTART: ::core::ffi::c_int = 0x10000000 as ::core::ffi::c_int;
pub const STDIN_FILENO: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const EOF: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const WNOHANG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
static mut input_stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
static mut linebuf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut keep_stdin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut lineno: size_t = 0 as size_t;
static mut bc_state: buildcmd_state = buildcmd_state {
    cmd_argc: 0,
    cmd_argv: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    cmd_argv_alloc: 0,
    argbuf: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    cmd_argv_chars: 0,
    cmd_initial_argv_chars: 0,
    usercontext: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    todo: 0,
    dir_fd: 0,
    largest_successful_arg_count: 0,
    smallest_failed_arg_count: 0,
};
static mut bc_ctl: buildcmd_control = buildcmd_control {
    exit_if_size_exceeded: 0,
    posix_arg_size_max: 0,
    posix_arg_size_min: 0,
    arg_max: 0,
    max_arg_count: 0,
    rplen: 0,
    replace_pat: ::core::ptr::null::<::core::ffi::c_char>(),
    initial_argc: 0,
    exec_callback: None,
    lines_per_exec: 0,
    args_per_exec: 0,
};
static mut nullwarning_given: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut eof_str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut initial_args: bool = r#true != 0;
pub const MAX_PROC_MAX: ::core::ffi::c_int = SIG_ATOMIC_MAX;
static mut proc_max: sig_atomic_t = 1 as sig_atomic_t;
static mut procs_executed: bool = r#false != 0;
static mut procs_executing: ::core::ffi::c_ulong = 0 as ::core::ffi::c_ulong;
static mut pids: *mut pid_t = ::core::ptr::null_mut::<pid_t>();
static mut pids_alloc: size_t = 0 as size_t;
static mut parent: pid_t = 0;
static mut stop_waiting: sig_atomic_t = 0 as sig_atomic_t;
static mut child_error: ::core::ffi::c_int = EXIT_SUCCESS;
static mut original_exit_value: ::core::ffi::c_int = 0;
static mut open_tty: bool = r#false != 0;
static mut print_command: bool = r#false != 0;
static mut query_before_executing: bool = r#false != 0;
static mut input_delimiter: ::core::ffi::c_char = '\0' as ::core::ffi::c_char;
static mut slot_var_name: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
static mut longopts: [option; 19] = [
    option {
        name: b"null\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: '0' as ::core::ffi::c_int,
    },
    option {
        name: b"arg-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'a' as ::core::ffi::c_int,
    },
    option {
        name: b"delimiter\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'd' as ::core::ffi::c_int,
    },
    option {
        name: b"eof\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'e' as ::core::ffi::c_int,
    },
    option {
        name: b"replace\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'I' as ::core::ffi::c_int,
    },
    option {
        name: b"max-lines\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: optional_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'l' as ::core::ffi::c_int,
    },
    option {
        name: b"max-args\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'n' as ::core::ffi::c_int,
    },
    option {
        name: b"open-tty\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'o' as ::core::ffi::c_int,
    },
    option {
        name: b"interactive\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"no-run-if-empty\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'r' as ::core::ffi::c_int,
    },
    option {
        name: b"max-chars\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 's' as ::core::ffi::c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 't' as ::core::ffi::c_int,
    },
    option {
        name: b"show-limits\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'S' as ::core::ffi::c_int,
    },
    option {
        name: b"exit\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'x' as ::core::ffi::c_int,
    },
    option {
        name: b"max-procs\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'P' as ::core::ffi::c_int,
    },
    option {
        name: b"process-slot-var\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: LongOptionIdentifier::PROCESS_SLOT_VAR.0 as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'h' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn get_char_oct_or_hex_escape(
    mut s: *const ::core::ffi::c_char,
) -> ::core::ffi::c_char {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut base: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
    let mut val: ::core::ffi::c_ulong = 0;
    let mut endp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    '_c2rust_label: {
        if '\\' as ::core::ffi::c_int == *s.offset(0isize) as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"'\\\\' == s[0]\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr() as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_uint,
                b"char get_char_oct_or_hex_escape(const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if 'x' as ::core::ffi::c_int == *s.offset(1isize) as ::core::ffi::c_int {
        p = s.offset(2 as ::core::ffi::c_int as isize);
        base = 16 as ::core::ffi::c_int;
    } else if *(*__ctype_b_loc())
        .offset(*s.offset(1isize) as ::core::ffi::c_uchar as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & C2Rust_Unnamed::_ISdigit.0 as ::core::ffi::c_int as ::core::ffi::c_ushort
            as ::core::ffi::c_int
        != 0
    {
        p = s.offset(1 as ::core::ffi::c_int as isize);
        base = 8 as ::core::ffi::c_int;
    } else {
        p = ::core::ptr::null::<::core::ffi::c_char>();
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Invalid escape sequence %s in input delimiter specification.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                s,
            );
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid escape sequence %s in input delimiter specification.\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    *__errno_location() = 0 as ::core::ffi::c_int;
    endp = ::core::ptr::null_mut::<::core::ffi::c_char>();
    val = strtoul(p, &raw mut endp, base);
    if ULONG_MAX == val && ERANGE == *__errno_location() || val > UCHAR_MAX as ::core::ffi::c_ulong
    {
        if 16 as ::core::ffi::c_int == base {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lx.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                    (127 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lx.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        s,
                        (127 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        } else {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lo.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                    (127 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                        + 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Invalid escape sequence %s in input delimiter specification; character values must not exceed %lo.\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        s,
                        (127 as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int) as ::core::ffi::c_ulong,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if 0 as ::core::ffi::c_int != *endp as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Invalid escape sequence %s in input delimiter specification; trailing characters %s not recognised.\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                s,
                endp,
            );
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid escape sequence %s in input delimiter specification; trailing characters %s not recognised.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                    endp,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    return val as ::core::ffi::c_char;
}
unsafe extern "C" fn get_input_delimiter(mut s: *const ::core::ffi::c_char) -> ::core::ffi::c_char {
    if 1 as size_t == strlen(s) {
        return *s.offset(0isize);
    } else if '\\' as ::core::ffi::c_int == *s.offset(0isize) as ::core::ffi::c_int {
        match *s.offset(1isize) as ::core::ffi::c_int {
            97 => return '\u{7}' as ::core::ffi::c_char,
            98 => return '\u{8}' as ::core::ffi::c_char,
            102 => return '\u{c}' as ::core::ffi::c_char,
            110 => return '\n' as ::core::ffi::c_char,
            114 => return '\r' as ::core::ffi::c_char,
            116 => return '\t' as ::core::ffi::c_char,
            118 => return '\u{b}' as ::core::ffi::c_char,
            92 => return '\\' as ::core::ffi::c_char,
            _ => return get_char_oct_or_hex_escape(s),
        }
    } else {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Invalid input delimiter specification %s: the delimiter must be either a single character or an escape sequence starting with \\.\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                s,
            );
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    0 as ::core::ffi::c_int,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Invalid input delimiter specification %s: the delimiter must be either a single character or an escape sequence starting with \\.\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    s,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        return 0 as ::core::ffi::c_char;
    };
}
unsafe extern "C" fn noop() {}
unsafe extern "C" fn fail_due_to_env_size() {
    if 0 != 0 {
        error(
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"environment is too large for exec\0".as_ptr() as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            ),
        );
        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            unreachable!();
        } else {
        };
    } else {
        ({
            let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            error(
                __errstatus,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"environment is too large for exec\0".as_ptr() as *const ::core::ffi::c_char,
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
unsafe extern "C" fn smaller_of(mut a: size_t, mut b: size_t) -> size_t {
    if a < b {
        return a;
    } else {
        return b;
    };
}
unsafe extern "C" fn fopen_cloexec_for_read_only(
    mut file_name: *const ::core::ffi::c_char,
) -> *mut FILE {
    let mut fd: ::core::ffi::c_int = open_cloexec(file_name, O_RDONLY);
    if fd < 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<FILE>();
    } else {
        let mut result: *mut FILE = fdopen(fd, b"r\0".as_ptr() as *const ::core::ffi::c_char);
        if result.is_null() {
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            close(fd);
            *__errno_location() = saved_errno;
            return ::core::ptr::null_mut::<FILE>();
        }
        return result;
    };
}
unsafe extern "C" fn warn_mutually_exclusive(
    mut option: *const ::core::ffi::c_char,
    mut offending: *const ::core::ffi::c_char,
) {
    if 0 != 0 {
        error(
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"warning: options %s and %s are mutually exclusive, ignoring previous %s value\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                5 as ::core::ffi::c_int,
            ),
            offending,
            option,
            offending,
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
                    b"warning: options %s and %s are mutually exclusive, ignoring previous %s value\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
                offending,
                option,
                offending,
            );
            if __errstatus != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        });
    };
}
unsafe extern "C" fn rboxc_findutils_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut optc: ::core::ffi::c_int = 0;
    let mut option_index: ::core::ffi::c_int = 0;
    let mut show_limits: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut always_run_command: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut input_file: *const ::core::ffi::c_char = b"-\0".as_ptr() as *const ::core::ffi::c_char;
    let mut default_cmd: [::core::ffi::c_char; 5] =
        ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"echo\0");
    let mut default_arglist: [*mut ::core::ffi::c_char; 1] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 1];
    let mut read_args: Option<unsafe extern "C" fn() -> ::core::ffi::c_int> =
        Some(read_line as unsafe extern "C" fn() -> ::core::ffi::c_int);
    let mut act_on_init_result: Option<unsafe extern "C" fn() -> ()> =
        Some(noop as unsafe extern "C" fn() -> ());
    let mut bcstatus: BC_INIT_STATUS = BC_INIT_STATUS::BC_INIT_OK;
    let mut catch_usr_signals: bool = r#false != 0;
    if !(*argv.offset(0isize)).is_null() {
        set_program_name(*argv.offset(0isize));
    } else {
        set_program_name(b"xargs\0".as_ptr() as *const ::core::ffi::c_char);
    }
    remember_non_cloexec_fds();
    parent = getpid() as pid_t;
    ::core::ptr::write_volatile(&raw mut original_exit_value, EXIT_SUCCESS);
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(PACKAGE.as_ptr(), LOCALEDIR.as_ptr());
    textdomain(PACKAGE.as_ptr());
    if atexit(Some(close_stdin as unsafe extern "C" fn() -> ())) != 0
        || atexit(Some(wait_for_proc_all as unsafe extern "C" fn() -> ())) != 0
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"The atexit library function failed\0".as_ptr() as *const ::core::ffi::c_char,
                    5 as ::core::ffi::c_int,
                ),
            );
            if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"The atexit library function failed\0".as_ptr()
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
    if libc::atexit(rboxc_release_xargs_input) != 0 { return 1; }
    bcstatus = bc_init_controlinfo(
        &raw mut bc_ctl,
        C2Rust_Unnamed_12::XARGS_POSIX_HEADROOM.0 as ::core::ffi::c_int as size_t,
    );
    if BC_INIT_STATUS::BC_INIT_ENV_TOO_BIG.0 == bcstatus.0 {
        act_on_init_result = Some(fail_due_to_env_size as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
    } else if BC_INIT_STATUS::BC_INIT_CANNOT_ACCOMODATE_HEADROOM.0 == bcstatus.0 {
        act_on_init_result = Some(fail_due_to_env_size as unsafe extern "C" fn() -> ())
            as Option<unsafe extern "C" fn() -> ()>;
    } else {
        let mut val: ::core::ffi::c_long = 0;
        val = sysconf(C2Rust_Unnamed_11::_SC_ARG_MAX.0 as ::core::ffi::c_int);
        if val > 0 as ::core::ffi::c_long {
            '_c2rust_label_0: {
                if val
                    > C2Rust_Unnamed_12::XARGS_POSIX_HEADROOM.0 as ::core::ffi::c_int
                        as ::core::ffi::c_long
                {
                } else {
                    __assert_fail(
                        b"val > XARGS_POSIX_HEADROOM\0".as_ptr() as *const ::core::ffi::c_char,
                        b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        482 as ::core::ffi::c_uint,
                        b"int single_binary_main_xargs(int, char **)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
            };
            bc_ctl.arg_max = smaller_of(
                bc_ctl.arg_max,
                (val as size_t).wrapping_sub(
                    C2Rust_Unnamed_12::XARGS_POSIX_HEADROOM.0 as ::core::ffi::c_int as size_t,
                ),
            );
        }
        '_c2rust_label_1: {
            if bc_ctl.arg_max >= 2048 as size_t {
            } else {
                __assert_fail(
                    b"bc_ctl.arg_max >= LINE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                    b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    510 as ::core::ffi::c_uint,
                    b"int single_binary_main_xargs(int, char **)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        bc_ctl.exec_callback = Some(
            xargs_do_exec
                as unsafe extern "C" fn(
                    *mut buildcmd_control,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut buildcmd_control,
                    *mut ::core::ffi::c_void,
                    ::core::ffi::c_int,
                    *mut *mut ::core::ffi::c_char,
                ) -> ::core::ffi::c_int,
            >;
        bc_use_sensible_arg_max(&raw mut bc_ctl);
    }
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"+0a:E:e::i::I:l::L:n:oprs:txP:d:\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            &raw mut option_index,
        );
        if optc == -1 as ::core::ffi::c_int {
            break;
        }
        match optc {
            48 => {
                read_args = Some(read_string as unsafe extern "C" fn() -> ::core::ffi::c_int)
                    as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
                input_delimiter = '\0' as ::core::ffi::c_char;
            }
            100 => {
                read_args = Some(read_string as unsafe extern "C" fn() -> ::core::ffi::c_int)
                    as Option<unsafe extern "C" fn() -> ::core::ffi::c_int>;
                input_delimiter = get_input_delimiter(optarg);
            }
            69 | 101 => {
                if !optarg.is_null() && strlen(optarg) > 0 as size_t {
                    eof_str = optarg;
                } else {
                    eof_str = ::core::ptr::null_mut::<::core::ffi::c_char>();
                }
            }
            104 => {
                usage(EXIT_SUCCESS);
            }
            73 | 105 => {
                if !optarg.is_null() {
                    bc_ctl.replace_pat = optarg;
                } else {
                    bc_ctl.replace_pat = b"{}\0".as_ptr() as *const ::core::ffi::c_char;
                }
                if bc_ctl.args_per_exec != 0 as size_t {
                    warn_mutually_exclusive(
                        b"--replace/-I/-i\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--max-args\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.args_per_exec = 0 as size_t;
                }
                if bc_ctl.lines_per_exec != 0 as ::core::ffi::c_ulong {
                    warn_mutually_exclusive(
                        b"--replace/-I/-i\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--max-lines\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.lines_per_exec = 0 as ::core::ffi::c_ulong;
                }
            }
            76 => {
                bc_ctl.lines_per_exec = parse_num(
                    optarg,
                    'L' as ::core::ffi::c_int,
                    1 as ::core::ffi::c_long,
                    -1 as ::core::ffi::c_long,
                    1 as ::core::ffi::c_int,
                ) as ::core::ffi::c_ulong;
                if bc_ctl.args_per_exec != 0 as size_t {
                    warn_mutually_exclusive(
                        b"-L\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--max-args\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.args_per_exec = 0 as size_t;
                }
                if !bc_ctl.replace_pat.is_null() {
                    warn_mutually_exclusive(
                        b"-L\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--replace\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.replace_pat = ::core::ptr::null::<::core::ffi::c_char>();
                }
            }
            108 => {
                if !optarg.is_null() {
                    bc_ctl.lines_per_exec = parse_num(
                        optarg,
                        'l' as ::core::ffi::c_int,
                        1 as ::core::ffi::c_long,
                        -1 as ::core::ffi::c_long,
                        1 as ::core::ffi::c_int,
                    ) as ::core::ffi::c_ulong;
                } else {
                    bc_ctl.lines_per_exec = 1 as ::core::ffi::c_ulong;
                }
                if bc_ctl.args_per_exec != 0 as size_t {
                    warn_mutually_exclusive(
                        b"--max-lines/-l\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--max-args\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.args_per_exec = 0 as size_t;
                }
                if !bc_ctl.replace_pat.is_null() {
                    warn_mutually_exclusive(
                        b"--max-lines/-l\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--replace\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.replace_pat = ::core::ptr::null::<::core::ffi::c_char>();
                }
            }
            110 => {
                bc_ctl.args_per_exec = parse_num(
                    optarg,
                    'n' as ::core::ffi::c_int,
                    1 as ::core::ffi::c_long,
                    -1 as ::core::ffi::c_long,
                    1 as ::core::ffi::c_int,
                ) as size_t;
                if bc_ctl.lines_per_exec != 0 as ::core::ffi::c_ulong {
                    warn_mutually_exclusive(
                        b"--max-args/-n\0".as_ptr() as *const ::core::ffi::c_char,
                        b"--max-lines\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bc_ctl.lines_per_exec = 0 as ::core::ffi::c_ulong;
                }
                if !bc_ctl.replace_pat.is_null() {
                    if bc_ctl.args_per_exec == 1 as size_t {
                        bc_ctl.args_per_exec = 0 as size_t;
                    } else {
                        warn_mutually_exclusive(
                            b"--max-args/-n\0".as_ptr() as *const ::core::ffi::c_char,
                            b"--replace\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        bc_ctl.replace_pat = ::core::ptr::null::<::core::ffi::c_char>();
                    }
                }
            }
            115 => {
                let mut arg_size: size_t = 0;
                act_on_init_result.expect("non-null function pointer")();
                arg_size = parse_num(
                    optarg,
                    's' as ::core::ffi::c_int,
                    1 as ::core::ffi::c_long,
                    bc_ctl.posix_arg_size_max as ::core::ffi::c_long,
                    0 as ::core::ffi::c_int,
                ) as size_t;
                if arg_size > bc_ctl.posix_arg_size_max {
                    if 0 != 0 {
                        error(
                            0 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"warning: value %ld for -s option is too large, using %ld instead\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            arg_size as ::core::ffi::c_long,
                            bc_ctl.posix_arg_size_max as ::core::ffi::c_long,
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
                                    b"warning: value %ld for -s option is too large, using %ld instead\0"
                                        .as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                arg_size as ::core::ffi::c_long,
                                bc_ctl.posix_arg_size_max as ::core::ffi::c_long,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                    arg_size = bc_ctl.posix_arg_size_max;
                }
                bc_ctl.arg_max = arg_size;
            }
            83 => {
                show_limits = r#true;
            }
            116 => {
                print_command = r#true != 0;
            }
            120 => {
                bc_ctl.exit_if_size_exceeded = r#true;
            }
            111 => {
                open_tty = r#true != 0;
            }
            112 => {
                query_before_executing = r#true != 0;
                print_command = r#true != 0;
            }
            114 => {
                always_run_command = 0 as ::core::ffi::c_int;
            }
            80 => {
                ::core::ptr::write_volatile(
                    &raw mut proc_max,
                    parse_num(
                        optarg,
                        'P' as ::core::ffi::c_int,
                        0 as ::core::ffi::c_long,
                        MAX_PROC_MAX as ::core::ffi::c_long,
                        1 as ::core::ffi::c_int,
                    ) as sig_atomic_t,
                );
                catch_usr_signals = r#true != 0;
            }
            97 => {
                input_file = optarg;
            }
            118 => {
                display_findutils_version(b"xargs\0".as_ptr() as *const ::core::ffi::c_char);
                return 0 as ::core::ffi::c_int;
            }
            128 => {
                if !strchr(optarg, '=' as ::core::ffi::c_int).is_null() {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"option --%s may not be set to a value which includes `='\0"
                                    .as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            longopts[option_index as usize].name,
                        );
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                0 as ::core::ffi::c_int,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"option --%s may not be set to a value which includes `='\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                longopts[option_index as usize].name,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
                slot_var_name = optarg;
                if 0 as ::core::ffi::c_int != unsetenv(slot_var_name) {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            *__errno_location(),
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"failed to unset environment variable %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            slot_var_name,
                        );
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"failed to unset environment variable %s\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                ),
                                slot_var_name,
                            );
                            if __errstatus != 0 as ::core::ffi::c_int {
                                unreachable!();
                            } else {
                            };
                        });
                    };
                }
            }
            _ => {
                usage(EXIT_FAILURE);
            }
        }
    }
    if !eof_str.is_null()
        && read_args == Some(read_string as unsafe extern "C" fn() -> ::core::ffi::c_int)
    {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"warning: the -E option has no effect if -0 or -d is used.\n\0".as_ptr()
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
                        b"warning: the -E option has no effect if -0 or -d is used.\n\0".as_ptr()
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
    act_on_init_result.expect("non-null function pointer")();
    '_c2rust_label_6: {
        if BC_INIT_STATUS::BC_INIT_OK.0 == bcstatus.0 {
        } else {
            __assert_fail(
                b"BC_INIT_OK == bcstatus\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr() as *const ::core::ffi::c_char,
                729 as ::core::ffi::c_uint,
                b"int single_binary_main_xargs(int, char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if catch_usr_signals {
        let mut sigact: sigaction = sigaction {
            __sigaction_handler: C2Rust_Unnamed_10 { sa_handler: None },
            sa_mask: __sigset_t { __val: [0; 16] },
            sa_flags: 0,
            sa_restorer: None,
        };
        sigact.__sigaction_handler.sa_handler =
            Some(increment_proc_max as unsafe extern "C" fn(::core::ffi::c_int) -> ())
                as __sighandler_t;
        sigemptyset(&raw mut sigact.sa_mask);
        sigact.sa_flags = SA_RESTART;
        if 0 as ::core::ffi::c_int != sigaction(SIGUSR1, &raw mut sigact, NULL as *mut sigaction) {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot set SIGUSR1 signal handler\0".as_ptr()
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
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot set SIGUSR1 signal handler\0".as_ptr()
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
        sigact.__sigaction_handler.sa_handler =
            Some(decrement_proc_max as unsafe extern "C" fn(::core::ffi::c_int) -> ())
                as __sighandler_t;
        sigemptyset(&raw mut sigact.sa_mask);
        sigact.sa_flags = SA_RESTART;
        if 0 as ::core::ffi::c_int != sigaction(SIGUSR2, &raw mut sigact, NULL as *mut sigaction) {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot set SIGUSR2 signal handler\0".as_ptr()
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
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot set SIGUSR2 signal handler\0".as_ptr()
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
    }
    if 0 as ::core::ffi::c_int == strcmp(input_file, b"-\0".as_ptr() as *const ::core::ffi::c_char)
    {
        input_stream = stdin;
    } else {
        keep_stdin = 1 as ::core::ffi::c_int;
        input_stream = fopen_cloexec_for_read_only(input_file);
        if input_stream.is_null() {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Cannot open input file %s\0".as_ptr() as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    quotearg_n_style(
                        0 as ::core::ffi::c_int,
                        quoting_style::locale_quoting_style,
                        input_file,
                    ),
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Cannot open input file %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            input_file,
                        ),
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if !bc_ctl.replace_pat.is_null() || bc_ctl.lines_per_exec != 0 {
        bc_ctl.exit_if_size_exceeded = r#true;
    }
    if optind == argc {
        optind = 0 as ::core::ffi::c_int;
        argc = 1 as ::core::ffi::c_int;
        default_arglist[0usize] = &raw mut default_cmd as *mut ::core::ffi::c_char;
        argv = &raw mut default_arglist as *mut *mut ::core::ffi::c_char;
    }
    if show_limits != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Your environment variables take up %lu bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            bc_size_of_environment() as uintmax_t,
        );
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"POSIX upper limit on argument length (this system): %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            bc_ctl.posix_arg_size_max as uintmax_t,
        );
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"POSIX smallest allowable upper limit on argument length (all systems): %lu\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            bc_ctl.posix_arg_size_min as uintmax_t,
        );
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Maximum length of command we could actually use: %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            bc_ctl
                .posix_arg_size_max
                .wrapping_sub(bc_size_of_environment()) as uintmax_t,
        );
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Size of command buffer we are actually using: %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            bc_ctl.arg_max as uintmax_t,
        );
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Maximum parallelism (--max-procs must be no greater): %lu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            MAX_PROC_MAX as uintmax_t,
        );
        if isatty(STDIN_FILENO) != 0 {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"\nExecution of xargs will continue now, and it will try to read its input and run commands; if this is not what you wanted to happen, please type the end-of-file keystroke.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            if always_run_command != 0 {
                fprintf(
                    stderr,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Warning: %s will be run at least once.  If you do not want that to happen, then press the interrupt keystroke.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    *argv.offset(optind as isize),
                );
            }
        }
    }
    linebuf = xmalloc(bc_ctl.arg_max.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    bc_state.argbuf = xmalloc(bc_ctl.arg_max.wrapping_add(1 as size_t)) as *mut ::core::ffi::c_char;
    signal(SIGCHLD, SIG_DFL);
    if bc_ctl.replace_pat.is_null() {
        while optind < argc {
            bc_push_arg(
                &raw mut bc_ctl,
                &raw mut bc_state,
                *argv.offset(optind as isize),
                strlen(*argv.offset(optind as isize)).wrapping_add(1 as size_t),
                ::core::ptr::null::<::core::ffi::c_char>(),
                0 as size_t,
                initial_args as ::core::ffi::c_int,
            );
            optind += 1;
        }
        initial_args = r#false != 0;
        bc_ctl.initial_argc = bc_state.cmd_argc;
        bc_state.cmd_initial_argv_chars = bc_state.cmd_argv_chars;
        bc_ctl.initial_argc = bc_state.cmd_argc;
        while Some(read_args.expect("non-null function pointer"))
            .expect("non-null function pointer")()
            != -1 as ::core::ffi::c_int
        {
            if bc_ctl.lines_per_exec != 0 && lineno >= bc_ctl.lines_per_exec as size_t {
                bc_do_exec(&raw mut bc_ctl, &raw mut bc_state);
                lineno = 0 as size_t;
            }
        }
        if bc_state.cmd_argc != bc_ctl.initial_argc
            || always_run_command != 0
                && procs_executed as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            bc_do_exec(&raw mut bc_ctl, &raw mut bc_state);
        }
    } else {
        let mut i: ::core::ffi::c_int = 0;
        let mut args: ::core::ffi::c_int = 0;
        RBOXC_ARGLEN =
            xmalloc(::core::mem::size_of::<size_t>().wrapping_mul(argc as size_t)) as *mut size_t;
        i = optind;
        while i < argc {
            *RBOXC_ARGLEN.offset(i as isize) = strlen(*argv.offset(i as isize));
            i += 1;
        }
        bc_ctl.rplen = strlen(bc_ctl.replace_pat);
        loop {
            args = Some(read_args.expect("non-null function pointer"))
                .expect("non-null function pointer")();
            if args == -1 as ::core::ffi::c_int {
                break;
            }
            let mut len: size_t = args as size_t;
            bc_clear_args(&raw mut bc_ctl, &raw mut bc_state);
            bc_state.cmd_argv_chars = 0 as size_t;
            bc_push_arg(
                &raw mut bc_ctl,
                &raw mut bc_state,
                *argv.offset(optind as isize),
                (*RBOXC_ARGLEN.offset(optind as isize)).wrapping_add(1 as size_t),
                ::core::ptr::null::<::core::ffi::c_char>(),
                0 as size_t,
                initial_args as ::core::ffi::c_int,
            );
            len = len.wrapping_sub(1);
            initial_args = r#false != 0;
            i = optind + 1 as ::core::ffi::c_int;
            while i < argc {
                bc_do_insert(
                    &raw mut bc_ctl,
                    &raw mut bc_state,
                    *argv.offset(i as isize),
                    *RBOXC_ARGLEN.offset(i as isize),
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as size_t,
                    linebuf,
                    len,
                    initial_args as ::core::ffi::c_int,
                );
                i += 1;
            }
            bc_do_exec(&raw mut bc_ctl, &raw mut bc_state);
        }
    }
    ::core::ptr::write_volatile(
        &raw mut original_exit_value,
        ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const child_error),
    );
    return ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const child_error);
}
unsafe extern "C" fn read_line() -> ::core::ffi::c_int {
    static mut eof: bool = r#false != 0;
    let mut state: read_line_state = read_line_state::SPACE;
    let mut prevc: ::core::ffi::c_int = 0;
    let mut quotc: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut c: ::core::ffi::c_int = EOF;
    let mut first: bool = r#true != 0;
    let mut seen_arg: bool = r#false != 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = linebuf;
    let mut endbuf: *mut ::core::ffi::c_char = linebuf
        .offset(bc_ctl.arg_max as isize)
        .offset(-(bc_state.cmd_initial_argv_chars as isize))
        .offset(-(1 as ::core::ffi::c_int as isize));
    if eof {
        return -1 as ::core::ffi::c_int;
    }
    's_33: loop {
        prevc = c;
        c = getc(input_stream);
        if c == EOF {
            if EINTR == *__errno_location() {
                continue;
            }
            eof = r#true != 0;
            if p == linebuf {
                return -1 as ::core::ffi::c_int;
            }
            let c2rust_fresh0 = p;
            p = p.offset(1);
            *c2rust_fresh0 = '\0' as ::core::ffi::c_char;
            len = p.offset_from(linebuf) as ::core::ffi::c_int;
            if state.0 == read_line_state::QUOTE.0 {
                exec_if_possible();
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                .as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        if quotc == '"' as ::core::ffi::c_int {
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"double\0".as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            )
                        } else {
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"single\0".as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            )
                        },
                    );
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                    .as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            if quotc == '"' as ::core::ffi::c_int {
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"double\0".as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                )
                            } else {
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"single\0".as_ptr() as *const ::core::ffi::c_char,
                                    5 as ::core::ffi::c_int,
                                )
                            },
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            if first as ::core::ffi::c_int != 0
                && (!eof_str.is_null()
                    && *eof_str as ::core::ffi::c_int == *linebuf as ::core::ffi::c_int
                    && strcmp(eof_str, linebuf) == 0)
            {
                return -1 as ::core::ffi::c_int;
            }
            if bc_ctl.replace_pat.is_null() {
                bc_push_arg(
                    &raw mut bc_ctl,
                    &raw mut bc_state,
                    linebuf,
                    len as size_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as size_t,
                    initial_args as ::core::ffi::c_int,
                );
            }
            return len;
        } else {
            's_289: {
                match state {
                    read_line_state::SPACE => {
                        if c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
                            && *(*__ctype_b_loc()).offset(c as isize) as ::core::ffi::c_int
                                & C2Rust_Unnamed::_ISblank.0 as ::core::ffi::c_int
                                    as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0
                            || c == '\n' as ::core::ffi::c_int
                            || c == '\r' as ::core::ffi::c_int
                            || c == '\u{c}' as ::core::ffi::c_int
                            || c == '\u{b}' as ::core::ffi::c_int
                        {
                            continue 's_33;
                        }
                        state = read_line_state::NORM;
                    }
                    read_line_state::NORM => {}
                    read_line_state::QUOTE => {
                        if c == '\n' as ::core::ffi::c_int {
                            exec_if_possible();
                            if 0 != 0 {
                                error(
                                    1 as ::core::ffi::c_int,
                                    0 as ::core::ffi::c_int,
                                    dcgettext(
                                        ::core::ptr::null::<::core::ffi::c_char>(),
                                        b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                            .as_ptr() as *const ::core::ffi::c_char,
                                        5 as ::core::ffi::c_int,
                                    ),
                                    if quotc == '"' as ::core::ffi::c_int {
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"double\0".as_ptr() as *const ::core::ffi::c_char,
                                            5 as ::core::ffi::c_int,
                                        )
                                    } else {
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"single\0".as_ptr() as *const ::core::ffi::c_char,
                                            5 as ::core::ffi::c_int,
                                        )
                                    },
                                );
                                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                                    unreachable!();
                                } else {
                                };
                            } else {
                                ({
                                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                                    error(
                                        __errstatus,
                                        0 as ::core::ffi::c_int,
                                        dcgettext(
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"unmatched %s quote; by default quotes are special to xargs unless you use the -0 option\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            5 as ::core::ffi::c_int,
                                        ),
                                        if quotc == '"' as ::core::ffi::c_int {
                                            dcgettext(
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                b"double\0".as_ptr() as *const ::core::ffi::c_char,
                                                5 as ::core::ffi::c_int,
                                            )
                                        } else {
                                            dcgettext(
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                b"single\0".as_ptr() as *const ::core::ffi::c_char,
                                                5 as ::core::ffi::c_int,
                                            )
                                        },
                                    );
                                    if __errstatus != 0 as ::core::ffi::c_int {
                                        unreachable!();
                                    } else {
                                    };
                                });
                            };
                        }
                        if c == quotc {
                            state = read_line_state::NORM;
                            seen_arg = r#true != 0;
                            continue 's_33;
                        } else {
                            break 's_289;
                        }
                    }
                    read_line_state::BACKSLASH => {
                        state = read_line_state::NORM;
                        break 's_289;
                    }
                    _ => {
                        break 's_289;
                    }
                }
                if c == '\n' as ::core::ffi::c_int {
                    if !(prevc & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
                        && *(*__ctype_b_loc()).offset(prevc as isize) as ::core::ffi::c_int
                            & C2Rust_Unnamed::_ISblank.0 as ::core::ffi::c_int
                                as ::core::ffi::c_ushort
                                as ::core::ffi::c_int
                            != 0)
                    {
                        lineno = lineno.wrapping_add(1);
                    }
                    if p == linebuf {
                        if !seen_arg {
                            state = read_line_state::SPACE;
                            continue 's_33;
                        }
                    }
                    let c2rust_fresh1 = p;
                    p = p.offset(1);
                    *c2rust_fresh1 = '\0' as ::core::ffi::c_char;
                    len = p.offset_from(linebuf) as ::core::ffi::c_int;
                    if !eof_str.is_null()
                        && *eof_str as ::core::ffi::c_int == *linebuf as ::core::ffi::c_int
                        && strcmp(eof_str, linebuf) == 0
                    {
                        eof = r#true != 0;
                        return if first as ::core::ffi::c_int != 0 {
                            -1 as ::core::ffi::c_int
                        } else {
                            len
                        };
                    }
                    if bc_ctl.replace_pat.is_null() {
                        bc_push_arg(
                            &raw mut bc_ctl,
                            &raw mut bc_state,
                            linebuf,
                            len as size_t,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            0 as size_t,
                            initial_args as ::core::ffi::c_int,
                        );
                    }
                    return len;
                } else {
                    seen_arg = r#true != 0;
                    if bc_ctl.replace_pat.is_null()
                        && (c & !(0x7f as ::core::ffi::c_int) == 0 as ::core::ffi::c_int
                            && *(*__ctype_b_loc()).offset(c as isize) as ::core::ffi::c_int
                                & C2Rust_Unnamed::_ISblank.0 as ::core::ffi::c_int
                                    as ::core::ffi::c_ushort
                                    as ::core::ffi::c_int
                                != 0)
                    {
                        let c2rust_fresh2 = p;
                        p = p.offset(1);
                        *c2rust_fresh2 = '\0' as ::core::ffi::c_char;
                        len = p.offset_from(linebuf) as ::core::ffi::c_int;
                        if !eof_str.is_null()
                            && *eof_str as ::core::ffi::c_int == *linebuf as ::core::ffi::c_int
                            && strcmp(eof_str, linebuf) == 0
                        {
                            eof = r#true != 0;
                            return if first as ::core::ffi::c_int != 0 {
                                -1 as ::core::ffi::c_int
                            } else {
                                len
                            };
                        }
                        bc_push_arg(
                            &raw mut bc_ctl,
                            &raw mut bc_state,
                            linebuf,
                            len as size_t,
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            0 as size_t,
                            initial_args as ::core::ffi::c_int,
                        );
                        p = linebuf;
                        state = read_line_state::SPACE;
                        first = r#false != 0;
                        continue 's_33;
                    } else {
                        match c {
                            92 => {
                                state = read_line_state::BACKSLASH;
                                continue 's_33;
                            }
                            39 | 34 => {
                                state = read_line_state::QUOTE;
                                quotc = c;
                                continue 's_33;
                            }
                            _ => {}
                        }
                    }
                }
            }
            if 0 as ::core::ffi::c_int == c && nullwarning_given == 0 {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"WARNING: a NUL character occurred in the input.  It cannot be passed through in the argument list.  Did you mean to use the --null option?\0"
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
                                b"WARNING: a NUL character occurred in the input.  It cannot be passed through in the argument list.  Did you mean to use the --null option?\0"
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
                nullwarning_given = 1 as ::core::ffi::c_int;
            }
            if p >= endbuf {
                exec_if_possible();
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"argument line too long\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"argument line too long\0".as_ptr() as *const ::core::ffi::c_char,
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
            let c2rust_fresh3 = p;
            p = p.offset(1);
            *c2rust_fresh3 = c as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn read_string() -> ::core::ffi::c_int {
    static mut eof: bool = r#false != 0;
    let mut len: ::core::ffi::c_int = 0;
    let mut p: *mut ::core::ffi::c_char = linebuf;
    let mut endbuf: *mut ::core::ffi::c_char = linebuf
        .offset(bc_ctl.arg_max as isize)
        .offset(-(bc_state.cmd_initial_argv_chars as isize))
        .offset(-(1 as ::core::ffi::c_int as isize));
    if eof {
        return -1 as ::core::ffi::c_int;
    }
    loop {
        let mut c: ::core::ffi::c_int = getc(input_stream);
        if c == EOF {
            if EINTR == *__errno_location() {
                continue;
            }
            eof = r#true != 0;
            if p == linebuf {
                return -1 as ::core::ffi::c_int;
            }
            let c2rust_fresh4 = p;
            p = p.offset(1);
            *c2rust_fresh4 = '\0' as ::core::ffi::c_char;
            len = p.offset_from(linebuf) as ::core::ffi::c_int;
            if bc_ctl.replace_pat.is_null() {
                bc_push_arg(
                    &raw mut bc_ctl,
                    &raw mut bc_state,
                    linebuf,
                    len as size_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    0 as size_t,
                    initial_args as ::core::ffi::c_int,
                );
            }
            return len;
        } else {
            if c == input_delimiter as ::core::ffi::c_int {
                lineno = lineno.wrapping_add(1);
                let c2rust_fresh5 = p;
                p = p.offset(1);
                *c2rust_fresh5 = '\0' as ::core::ffi::c_char;
                len = p.offset_from(linebuf) as ::core::ffi::c_int;
                if bc_ctl.replace_pat.is_null() {
                    bc_push_arg(
                        &raw mut bc_ctl,
                        &raw mut bc_state,
                        linebuf,
                        len as size_t,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        0 as size_t,
                        initial_args as ::core::ffi::c_int,
                    );
                }
                return len;
            }
            if p >= endbuf {
                exec_if_possible();
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"argument line too long\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            0 as ::core::ffi::c_int,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"argument line too long\0".as_ptr() as *const ::core::ffi::c_char,
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
            let c2rust_fresh6 = p;
            p = p.offset(1);
            *c2rust_fresh6 = c as ::core::ffi::c_char;
        }
    }
}
unsafe extern "C" fn print_args(mut ask: bool) -> bool {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < bc_state.cmd_argc.wrapping_sub(1 as size_t) {
        if fprintf(
            stderr,
            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            if i == 0 as size_t {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b" \0".as_ptr() as *const ::core::ffi::c_char
            },
            quotearg_n_style(
                0 as ::core::ffi::c_int,
                quoting_style::shell_escape_quoting_style,
                *bc_state.cmd_argv.offset(i as isize),
            ),
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Failed to write to standard error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Failed to write to standard error\0".as_ptr()
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
        i = i.wrapping_add(1);
    }
    if ask {
        static mut tty_stream: *mut FILE = ::core::ptr::null_mut::<FILE>();
        let mut c: ::core::ffi::c_int = 0;
        let mut savec: ::core::ffi::c_int = 0;
        if tty_stream.is_null() {
            tty_stream =
                fopen_cloexec_for_read_only(b"/dev/tty\0".as_ptr() as *const ::core::ffi::c_char);
            if tty_stream.is_null() {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"failed to open /dev/tty for reading\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"failed to open /dev/tty for reading\0".as_ptr()
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
        }
        fputs(b"?...\0".as_ptr() as *const ::core::ffi::c_char, stderr);
        if fflush(stderr) != 0 as ::core::ffi::c_int {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Failed to write to standard error\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Failed to write to standard error\0".as_ptr()
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
        savec = getc(tty_stream);
        c = savec;
        while c != EOF && c != '\n' as ::core::ffi::c_int {
            c = getc(tty_stream);
        }
        if EOF == c {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Failed to read from standard input\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"Failed to read from standard input\0".as_ptr()
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
        if savec == 'y' as ::core::ffi::c_int || savec == 'Y' as ::core::ffi::c_int {
            return r#true != 0;
        }
    } else {
        putc('\n' as ::core::ffi::c_int, stderr);
    }
    return r#false != 0;
}
unsafe extern "C" fn set_slot_var(mut n: ::core::ffi::c_uint) {
    let mut buf: [::core::ffi::c_char; 20] = [0; 20];
    '_c2rust_label: {
        if snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1 as size_t),
            b"%u\0".as_ptr() as *const ::core::ffi::c_char,
            n,
        ) as usize
            <= ::core::mem::size_of::<[::core::ffi::c_char; 20]>().wrapping_sub(1usize)
        {
        } else {
            __assert_fail(
                b"snprintf (buf, sizeof buf - 1, \"%u\", n) <= sizeof buf - 1\0".as_ptr()
                    as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr() as *const ::core::ffi::c_char,
                1220 as ::core::ffi::c_uint,
                b"void set_slot_var(unsigned int)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if !slot_var_name.is_null() {
        if setenv(
            slot_var_name,
            &raw mut buf as *mut ::core::ffi::c_char,
            1 as ::core::ffi::c_int,
        ) < 0 as ::core::ffi::c_int
        {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"failed to set environment variable %s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                    slot_var_name,
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
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"failed to set environment variable %s\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        slot_var_name,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
}
unsafe extern "C" fn prep_child_for_exec() {
    if fd_leak_check_is_enabled() {
        complain_about_leaky_fds();
    }
    let mut slot: ::core::ffi::c_uint = add_proc(0 as pid_t);
    set_slot_var(slot);
    if keep_stdin == 0 || open_tty as ::core::ffi::c_int != 0 {
        let mut fd: ::core::ffi::c_int = 0;
        let mut inputfile: *const ::core::ffi::c_char = if open_tty as ::core::ffi::c_int != 0 {
            b"/dev/tty\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char
        };
        close(0 as ::core::ffi::c_int);
        fd = open_safer(inputfile, O_RDONLY);
        if fd < 0 as ::core::ffi::c_int {
            if open_tty {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            inputfile,
                        ),
                    );
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                inputfile,
                            ),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            } else {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            inputfile,
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
                            *__errno_location(),
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                inputfile,
                            ),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
        }
        if STDIN_FILENO < fd {
            if dup2(fd, STDIN_FILENO) != 0 as ::core::ffi::c_int {
                if 0 != 0 {
                    error(
                        1 as ::core::ffi::c_int,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"failed to redirect standard input of the child process\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                    );
                    if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                } else {
                    ({
                        let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                        error(
                            __errstatus,
                            *__errno_location(),
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"failed to redirect standard input of the child process\0".as_ptr()
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
            close(fd);
        }
    }
}
unsafe extern "C" fn xargs_do_exec(
    mut ctl: *mut buildcmd_control,
    mut usercontext: *mut ::core::ffi::c_void,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut child: pid_t = 0;
    let mut fd: [::core::ffi::c_int; 2] = [0; 2];
    let mut buf: ::core::ffi::c_int = 0;
    let mut r: ptrdiff_t = 0;
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const proc_max) != 0 {
        while procs_executing
            >= ::core::ptr::read_volatile::<sig_atomic_t>(&raw const proc_max)
                as ::core::ffi::c_ulong
        {
            wait_for_proc(r#false != 0, 1 as ::core::ffi::c_uint);
        }
    }
    if !query_before_executing || print_args(r#true != 0) as ::core::ffi::c_int != 0 {
        if !query_before_executing && print_command as ::core::ffi::c_int != 0 {
            print_args(r#false != 0);
        }
        wait_for_proc(r#false != 0, 0 as ::core::ffi::c_uint);
        if pipe_safer(&raw mut fd as *mut ::core::ffi::c_int) != 0 {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"could not create pipe before fork\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        5 as ::core::ffi::c_int,
                    ),
                );
                if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"could not create pipe before fork\0".as_ptr()
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
        rpl_fcntl(fd[1usize], F_SETFD, FD_CLOEXEC);
        loop {
            child = fork() as pid_t;
            if !(child < 0 as ::core::ffi::c_int
                && *__errno_location() == EAGAIN
                && procs_executing != 0)
            {
                break;
            }
            wait_for_proc(r#false != 0, 1 as ::core::ffi::c_uint);
        }
        's_159: {
            match child {
                -1 => {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            *__errno_location(),
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"cannot fork\0".as_ptr() as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                        );
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"cannot fork\0".as_ptr() as *const ::core::ffi::c_char,
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
                0 => {}
                _ => {
                    close(fd[1usize]);
                    break 's_159;
                }
            }
            close(fd[0usize]);
            ::core::ptr::write_volatile(&raw mut child_error, EXIT_SUCCESS);
            prep_child_for_exec();
            if bc_args_exceed_testing_limit(argv) {
                *__errno_location() = E2BIG;
            } else {
                execvp(
                    *argv.offset(0isize),
                    argv as *const *mut ::core::ffi::c_char,
                );
            }
            let mut saved_errno: ::core::ffi::c_int = *__errno_location();
            if saved_errno != 0 {
                write(
                    fd[1usize],
                    &raw mut saved_errno as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<::core::ffi::c_int>(),
                );
            }
            close(fd[1usize]);
            if E2BIG != saved_errno {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        saved_errno,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"failed to run command %s\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        quotearg_n_style(
                            0 as ::core::ffi::c_int,
                            quoting_style::locale_quoting_style,
                            *argv.offset(0isize),
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
                            saved_errno,
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"failed to run command %s\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            quotearg_n_style(
                                0 as ::core::ffi::c_int,
                                quoting_style::locale_quoting_style,
                                *argv.offset(0isize),
                            ),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            // Only the failed-exec child owns this replacement for stdin.
            rboxc_release_xargs_input();
            if keep_stdin == 0 || open_tty { libc::close(0); }
            _exit(if saved_errno == ENOENT {
                XargsStatusValues::XARGS_EXIT_COMMAND_NOT_FOUND.0 as ::core::ffi::c_int
            } else {
                XargsStatusValues::XARGS_EXIT_COMMAND_CANNOT_BE_RUN.0 as ::core::ffi::c_int
            });
        }
        r = safe_read(
            fd[0usize],
            &raw mut buf as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as idx_t,
        );
        close(fd[0usize]);
        if r < 0 as ptrdiff_t {
            if 0 != 0 {
                error(
                    0 as ::core::ffi::c_int,
                    *__errno_location(),
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"errno-buffer safe_read failed in xargs_do_exec (this is probably a bug, please report it)\0"
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
                        *__errno_location(),
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"errno-buffer safe_read failed in xargs_do_exec (this is probably a bug, please report it)\0"
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
        } else if (0 as ptrdiff_t) < r {
            let mut childstatus: ::core::ffi::c_int = 0;
            waitpid(child, &raw mut childstatus, 0 as ::core::ffi::c_int);
            if E2BIG == buf {
                return 0 as ::core::ffi::c_int;
            } else if ENOENT == buf {
                exit(XargsStatusValues::XARGS_EXIT_COMMAND_NOT_FOUND.0 as ::core::ffi::c_int);
            } else {
                exit(XargsStatusValues::XARGS_EXIT_COMMAND_CANNOT_BE_RUN.0 as ::core::ffi::c_int);
            }
        } else {
            add_proc(child);
        }
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn exec_if_possible() {
    if !bc_ctl.replace_pat.is_null()
        || initial_args as ::core::ffi::c_int != 0
        || bc_state.cmd_argc == bc_ctl.initial_argc
        || bc_ctl.exit_if_size_exceeded != 0
    {
        return;
    }
    bc_do_exec(&raw mut bc_ctl, &raw mut bc_state);
}
unsafe extern "C" fn add_proc(mut pid: pid_t) -> ::core::ffi::c_uint {
    let mut i: ::core::ffi::c_uint = 0;
    let mut j: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while (i as size_t) < pids_alloc && *pids.offset(i as isize) != 0 {
        i = i.wrapping_add(1);
    }
    if i as size_t == pids_alloc {
        pids = x2nrealloc(
            pids as *mut ::core::ffi::c_void,
            &raw mut pids_alloc,
            ::core::mem::size_of::<pid_t>(),
        ) as *mut pid_t;
        j = i;
        while (j as size_t) < pids_alloc {
            *pids.offset(j as isize) = 0 as ::core::ffi::c_int;
            j = j.wrapping_add(1);
        }
    }
    '_c2rust_label: {
        if 0 as ::core::ffi::c_int == *pids.offset(i as isize) {
        } else {
            __assert_fail(
                b"0 == pids[i]\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr() as *const ::core::ffi::c_char,
                1514 as ::core::ffi::c_uint,
                b"unsigned int add_proc(pid_t)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    *pids.offset(i as isize) = pid;
    procs_executing = procs_executing.wrapping_add(1);
    procs_executed = r#true != 0;
    return i;
}
unsafe extern "C" fn wait_for_proc(mut all: bool, mut minreap: ::core::ffi::c_uint) {
    let mut reaped: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut deferred_exit_status: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while procs_executing != 0 {
        let mut i: ::core::ffi::c_uint = 0;
        let mut status: ::core::ffi::c_int = 0;
        let mut pid: pid_t = 0;
        let mut wflags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !all {
            if reaped >= minreap {
                wflags = WNOHANG;
            }
        }
        ::core::ptr::write_volatile(
            &raw mut stop_waiting,
            0 as ::core::ffi::c_int as sig_atomic_t,
        );
        loop {
            loop {
                pid = waitpid(-1 as __pid_t, &raw mut status, wflags) as pid_t;
                if pid != -1 as ::core::ffi::c_int {
                    break;
                }
                if *__errno_location() != EINTR {
                    if 0 != 0 {
                        error(
                            1 as ::core::ffi::c_int,
                            *__errno_location(),
                            dcgettext(
                                ::core::ptr::null::<::core::ffi::c_char>(),
                                b"error waiting for child process\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                        );
                        if 1 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    } else {
                        ({
                            let __errstatus: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                            error(
                                __errstatus,
                                *__errno_location(),
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"error waiting for child process\0".as_ptr()
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
                if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const stop_waiting) != 0 && !all
                {
                    wflags = WNOHANG;
                }
            }
            if pid != 0 {
                i = 0 as ::core::ffi::c_uint;
                while (i as size_t) < pids_alloc && pid != *pids.offset(i as isize) {
                    i = i.wrapping_add(1);
                }
            }
            if !(pid != 0 && i as size_t == pids_alloc) {
                break;
            }
        }
        if pid == 0 {
            if wflags & WNOHANG == 0 {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"WARNING: Lost track of %lu child processes\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        procs_executing,
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
                                b"WARNING: Lost track of %lu child processes\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            procs_executing,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
            }
            break;
        } else {
            *pids.offset(i as isize) = 0 as ::core::ffi::c_int as pid_t;
            procs_executing = procs_executing.wrapping_sub(1);
            reaped = reaped.wrapping_add(1);
            if (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
                == ClientStatusValues::CHILD_EXIT_PLEASE_STOP_IMMEDIATELY.0 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s: exited with status 255; aborting\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        *bc_state.cmd_argv.offset(0isize),
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
                                b"%s: exited with status 255; aborting\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            *bc_state.cmd_argv.offset(0isize),
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                if deferred_exit_status
                    < XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_255.0 as ::core::ffi::c_int
                {
                    deferred_exit_status =
                        XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_255.0 as ::core::ffi::c_int;
                }
            }
            if status & 0xff as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s: stopped by signal %d\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        *bc_state.cmd_argv.offset(0isize),
                        (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int,
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
                                b"%s: stopped by signal %d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            *bc_state.cmd_argv.offset(0isize),
                            (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                if deferred_exit_status
                    < XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG.0 as ::core::ffi::c_int
                {
                    deferred_exit_status =
                        XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG.0 as ::core::ffi::c_int;
                }
            }
            if ((status & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
                as ::core::ffi::c_schar as ::core::ffi::c_int
                >> 1 as ::core::ffi::c_int
                > 0 as ::core::ffi::c_int
            {
                if 0 != 0 {
                    error(
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        dcgettext(
                            ::core::ptr::null::<::core::ffi::c_char>(),
                            b"%s: terminated by signal %d\0".as_ptr() as *const ::core::ffi::c_char,
                            5 as ::core::ffi::c_int,
                        ),
                        *bc_state.cmd_argv.offset(0isize),
                        status & 0x7f as ::core::ffi::c_int,
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
                                b"%s: terminated by signal %d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                5 as ::core::ffi::c_int,
                            ),
                            *bc_state.cmd_argv.offset(0isize),
                            status & 0x7f as ::core::ffi::c_int,
                        );
                        if __errstatus != 0 as ::core::ffi::c_int {
                            unreachable!();
                        } else {
                        };
                    });
                };
                if deferred_exit_status
                    < XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG.0 as ::core::ffi::c_int
                {
                    deferred_exit_status =
                        XargsStatusValues::XARGS_EXIT_CLIENT_FATAL_SIG.0 as ::core::ffi::c_int;
                }
            }
            if (status & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                ::core::ptr::write_volatile(
                    &raw mut child_error,
                    XargsStatusValues::XARGS_EXIT_CLIENT_EXIT_NONZERO.0 as ::core::ffi::c_int,
                );
            }
            if deferred_exit_status != 0 && !all {
                break;
            }
        }
    }
    if deferred_exit_status != 0 {
        ::core::ptr::write_volatile(
            &raw mut child_error,
            if deferred_exit_status
                > ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const child_error)
            {
                deferred_exit_status
            } else {
                ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const child_error)
            },
        );
        exit(::core::ptr::read_volatile::<::core::ffi::c_int>(
            &raw const child_error,
        ));
    }
}
unsafe extern "C" fn wait_for_proc_all() {
    static mut waiting: bool = r#false != 0;
    '_c2rust_label: {
        if getpid() == parent {
        } else {
            __assert_fail(
                b"getpid () == parent\0".as_ptr() as *const ::core::ffi::c_char,
                b"/opt/src/findutils-4.11.0/xargs/xargs.c\0".as_ptr() as *const ::core::ffi::c_char,
                1670 as ::core::ffi::c_uint,
                b"void wait_for_proc_all(void)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if waiting {
        return;
    }
    waiting = r#true != 0;
    wait_for_proc(r#true != 0, 0 as ::core::ffi::c_uint);
    waiting = r#false != 0;
    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const original_exit_value)
        != ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const child_error)
    {
        _exit(::core::ptr::read_volatile::<::core::ffi::c_int>(
            &raw const child_error,
        ));
    }
}
unsafe extern "C" fn increment_proc_max(mut ignore: ::core::ffi::c_int) {
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const proc_max) < MAX_PROC_MAX {
        ::core::ptr::write_volatile(
            &raw mut proc_max,
            ::core::ptr::read_volatile::<sig_atomic_t>(&raw const proc_max) + 1,
        );
    }
    ::core::ptr::write_volatile(
        &raw mut stop_waiting,
        1 as ::core::ffi::c_int as sig_atomic_t,
    );
}
unsafe extern "C" fn decrement_proc_max(mut ignore: ::core::ffi::c_int) {
    if ::core::ptr::read_volatile::<sig_atomic_t>(&raw const proc_max) > 1 as ::core::ffi::c_int {
        ::core::ptr::write_volatile(
            &raw mut proc_max,
            ::core::ptr::read_volatile::<sig_atomic_t>(&raw const proc_max) - 1,
        );
    }
}
unsafe extern "C" fn parse_num(
    mut str: *mut ::core::ffi::c_char,
    mut option: ::core::ffi::c_int,
    mut min: ::core::ffi::c_long,
    mut max: ::core::ffi::c_long,
    mut fatal: ::core::ffi::c_int,
) -> ::core::ffi::c_long {
    let mut eptr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut val: ::core::ffi::c_long = 0;
    val = strtol(str, &raw mut eptr, 10 as ::core::ffi::c_int);
    if eptr == str || *eptr as ::core::ffi::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: invalid number \"%s\" for -%c option\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
            str,
            option,
        );
        usage(EXIT_FAILURE);
    } else if val < min {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: value %s for -%c option should be >= %ld\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
            str,
            option,
            min,
        );
        if fatal != 0 {
            usage(EXIT_FAILURE);
        }
        val = min;
    } else if max >= 0 as ::core::ffi::c_long && val > max {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: value %s for -%c option should be <= %ld\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
            str,
            option,
            max,
        );
        if fatal != 0 {
            usage(EXIT_FAILURE);
        }
        val = max;
    }
    return val;
}
unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) -> ! {
    if status != EXIT_SUCCESS {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            program_name,
        );
        exit(status);
    }
    fprintf(
        stdout,
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... [COMMAND [INITIAL-ARGS]...]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        program_name,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Run COMMAND with arguments INITIAL-ARGS and more arguments read from input.\n\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Mandatory and optional arguments to long options are also\nmandatory or optional for the corresponding short option.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -0, --null                   items are separated by a null, not whitespace;\n                                 disables quote and backslash processing and\n                                 logical EOF processing\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -a, --arg-file=FILE          read arguments from FILE, not standard input\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -d, --delimiter=CHARACTER    items in input stream are separated by CHARACTER,\n                                 not by whitespace; disables quote and backslash\n                                 processing and logical EOF processing\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -E END                       set logical EOF string; if END occurs as a line\n                                 of input, the rest of the input is ignored\n                                 (ignored if -0 or -d was specified)\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -e, --eof[=END]              equivalent to -E END if END is specified;\n                                 otherwise, there is no end-of-file string\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -I R                         same as --replace=R\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -i, --replace[=R]            replace R in INITIAL-ARGS with names read\n                                 from standard input, split at newlines;\n                                 if R is unspecified, assume {}\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -L, --max-lines=MAX-LINES    use at most MAX-LINES non-blank input lines per\n                                 command line\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -l[MAX-LINES]                similar to -L but defaults to at most one non-\n                                 blank input line if MAX-LINES is not specified\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -n, --max-args=MAX-ARGS      use at most MAX-ARGS arguments per command line\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -o, --open-tty               Reopen standard input as /dev/tty in the child\n                                 process before executing the command; useful to\n                                 run an interactive application.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -P, --max-procs=MAX-PROCS    run at most MAX-PROCS processes at a time\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -p, --interactive            prompt before running commands\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --process-slot-var=VAR   set environment variable VAR in child processes\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -r, --no-run-if-empty        if there are no arguments, then do not run COMMAND;\n                                 if this option is not given, COMMAND will be\n                                 run at least once\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -s, --max-chars=MAX-CHARS    limit length of command line to MAX-CHARS\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --show-limits            show limits on command-line length\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -t, --verbose                print commands before executing them\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"  -x, --exit                   exit if the size (see -s) is exceeded\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --help                   display this help and exit\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"      --version                output version information and exit\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"If COMMAND is omitted, the default is 'echo'.\n\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            5 as ::core::ffi::c_int,
        ),
        stdout,
    );
    explain_how_to_report_bugs(stdout, program_name);
    exit(status);
}
pub const LOCALEDIR: [::core::ffi::c_char; 48] = unsafe {
    ::core::mem::transmute::<[u8; 48], [::core::ffi::c_char; 48]>(
        *b"/root/rboxc/build/oracle/findutils/share/locale\0",
    )
};
pub const __SCHAR_MAX__: ::core::ffi::c_int = 127 as ::core::ffi::c_int;
pub const __LONG_MAX__: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const PACKAGE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"findutils\0") };

extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
}
static mut RBOXC_INVOCATION: *const ::core::ffi::c_char = ::core::ptr::null();
unsafe extern "C" fn rboxc_findutils_error_prefix() {
    libc::fprintf(stderr.cast(), b"%s: \0".as_ptr().cast(), RBOXC_INVOCATION);
}

static mut RBOXC_ARGLEN: *mut size_t = ::core::ptr::null_mut();
extern "C" fn rboxc_release_xargs_input() {
    unsafe {
        let saved_errno = *libc::__errno_location();
        libc::free(RBOXC_ARGLEN.cast());
        RBOXC_ARGLEN = ::core::ptr::null_mut();
        if !input_stream.is_null() && input_stream != stdin {
            let owned = input_stream;
            input_stream = ::core::ptr::null_mut();
            libc::fclose(owned.cast());
        }
        *libc::__errno_location() = saved_errno;
    }
}

#[no_mangle]
pub unsafe extern "C" fn single_binary_main_xargs(
    argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    RBOXC_INVOCATION = if argv.is_null() || (*argv).is_null() {
        b"xargs\0".as_ptr().cast()
    } else { *argv };
    if error_print_progname.is_none() {
        error_print_progname = Some(rboxc_findutils_error_prefix);
    }
    rboxc_findutils_main_inner(argc, argv)
}
