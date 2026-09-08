// Generated from pinned GNU getconf 2.43 by scripts/translate-entry-provider.py.
// Source SHA-256: 7e73c2905c1c32e9f2908cb80dbb2e59675896f8967046bab19b6592df253386
/* Copyright (C) 1991-2026 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published
   by the Free Software Foundation; version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, see <https://www.gnu.org/licenses/>.  */
#[repr(C)]
pub struct _IO_jump_t { _opaque: [u8; 0] }
#[repr(C)]
pub struct __gconv_loaded_object { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn pathconf(
        __path: *const ::core::ffi::c_char,
        __name: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn sysconf(__name: ::core::ffi::c_int) -> ::core::ffi::c_long;
    fn confstr(
        __name: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
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
    fn __dcgettext(
        __domainname: *const ::core::ffi::c_char,
        __msgid: *const ::core::ffi::c_char,
        __category: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    fn textdomain(__domainname: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    static _libc_intl_domainname: [::core::ffi::c_char; 0];
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn puts(__s: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut __progname: *const ::core::ffi::c_char;
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const _PC_LINK_MAX: Self = Self(0);
    pub const _PC_MAX_CANON: Self = Self(1);
    pub const _PC_MAX_INPUT: Self = Self(2);
    pub const _PC_NAME_MAX: Self = Self(3);
    pub const _PC_PATH_MAX: Self = Self(4);
    pub const _PC_PIPE_BUF: Self = Self(5);
    pub const _PC_CHOWN_RESTRICTED: Self = Self(6);
    pub const _PC_NO_TRUNC: Self = Self(7);
    pub const _PC_VDISABLE: Self = Self(8);
    pub const _PC_SYNC_IO: Self = Self(9);
    pub const _PC_ASYNC_IO: Self = Self(10);
    pub const _PC_PRIO_IO: Self = Self(11);
    pub const _PC_SOCK_MAXBUF: Self = Self(12);
    pub const _PC_FILESIZEBITS: Self = Self(13);
    pub const _PC_REC_INCR_XFER_SIZE: Self = Self(14);
    pub const _PC_REC_MAX_XFER_SIZE: Self = Self(15);
    pub const _PC_REC_MIN_XFER_SIZE: Self = Self(16);
    pub const _PC_REC_XFER_ALIGN: Self = Self(17);
    pub const _PC_ALLOC_SIZE_MIN: Self = Self(18);
    pub const _PC_SYMLINK_MAX: Self = Self(19);
    pub const _PC_2_SYMLINKS: Self = Self(20);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
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
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const _CS_PATH: Self = Self(0);
    pub const _CS_V6_WIDTH_RESTRICTED_ENVS: Self = Self(1);
    pub const _CS_GNU_LIBC_VERSION: Self = Self(2);
    pub const _CS_GNU_LIBPTHREAD_VERSION: Self = Self(3);
    pub const _CS_V5_WIDTH_RESTRICTED_ENVS: Self = Self(4);
    pub const _CS_V7_WIDTH_RESTRICTED_ENVS: Self = Self(5);
    pub const _CS_LFS_CFLAGS: Self = Self(1000);
    pub const _CS_LFS_LDFLAGS: Self = Self(1001);
    pub const _CS_LFS_LIBS: Self = Self(1002);
    pub const _CS_LFS_LINTFLAGS: Self = Self(1003);
    pub const _CS_LFS64_CFLAGS: Self = Self(1004);
    pub const _CS_LFS64_LDFLAGS: Self = Self(1005);
    pub const _CS_LFS64_LIBS: Self = Self(1006);
    pub const _CS_LFS64_LINTFLAGS: Self = Self(1007);
    pub const _CS_XBS5_ILP32_OFF32_CFLAGS: Self = Self(1100);
    pub const _CS_XBS5_ILP32_OFF32_LDFLAGS: Self = Self(1101);
    pub const _CS_XBS5_ILP32_OFF32_LIBS: Self = Self(1102);
    pub const _CS_XBS5_ILP32_OFF32_LINTFLAGS: Self = Self(1103);
    pub const _CS_XBS5_ILP32_OFFBIG_CFLAGS: Self = Self(1104);
    pub const _CS_XBS5_ILP32_OFFBIG_LDFLAGS: Self = Self(1105);
    pub const _CS_XBS5_ILP32_OFFBIG_LIBS: Self = Self(1106);
    pub const _CS_XBS5_ILP32_OFFBIG_LINTFLAGS: Self = Self(1107);
    pub const _CS_XBS5_LP64_OFF64_CFLAGS: Self = Self(1108);
    pub const _CS_XBS5_LP64_OFF64_LDFLAGS: Self = Self(1109);
    pub const _CS_XBS5_LP64_OFF64_LIBS: Self = Self(1110);
    pub const _CS_XBS5_LP64_OFF64_LINTFLAGS: Self = Self(1111);
    pub const _CS_XBS5_LPBIG_OFFBIG_CFLAGS: Self = Self(1112);
    pub const _CS_XBS5_LPBIG_OFFBIG_LDFLAGS: Self = Self(1113);
    pub const _CS_XBS5_LPBIG_OFFBIG_LIBS: Self = Self(1114);
    pub const _CS_XBS5_LPBIG_OFFBIG_LINTFLAGS: Self = Self(1115);
    pub const _CS_POSIX_V6_ILP32_OFF32_CFLAGS: Self = Self(1116);
    pub const _CS_POSIX_V6_ILP32_OFF32_LDFLAGS: Self = Self(1117);
    pub const _CS_POSIX_V6_ILP32_OFF32_LIBS: Self = Self(1118);
    pub const _CS_POSIX_V6_ILP32_OFF32_LINTFLAGS: Self = Self(1119);
    pub const _CS_POSIX_V6_ILP32_OFFBIG_CFLAGS: Self = Self(1120);
    pub const _CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS: Self = Self(1121);
    pub const _CS_POSIX_V6_ILP32_OFFBIG_LIBS: Self = Self(1122);
    pub const _CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS: Self = Self(1123);
    pub const _CS_POSIX_V6_LP64_OFF64_CFLAGS: Self = Self(1124);
    pub const _CS_POSIX_V6_LP64_OFF64_LDFLAGS: Self = Self(1125);
    pub const _CS_POSIX_V6_LP64_OFF64_LIBS: Self = Self(1126);
    pub const _CS_POSIX_V6_LP64_OFF64_LINTFLAGS: Self = Self(1127);
    pub const _CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS: Self = Self(1128);
    pub const _CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS: Self = Self(1129);
    pub const _CS_POSIX_V6_LPBIG_OFFBIG_LIBS: Self = Self(1130);
    pub const _CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS: Self = Self(1131);
    pub const _CS_POSIX_V7_ILP32_OFF32_CFLAGS: Self = Self(1132);
    pub const _CS_POSIX_V7_ILP32_OFF32_LDFLAGS: Self = Self(1133);
    pub const _CS_POSIX_V7_ILP32_OFF32_LIBS: Self = Self(1134);
    pub const _CS_POSIX_V7_ILP32_OFF32_LINTFLAGS: Self = Self(1135);
    pub const _CS_POSIX_V7_ILP32_OFFBIG_CFLAGS: Self = Self(1136);
    pub const _CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS: Self = Self(1137);
    pub const _CS_POSIX_V7_ILP32_OFFBIG_LIBS: Self = Self(1138);
    pub const _CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS: Self = Self(1139);
    pub const _CS_POSIX_V7_LP64_OFF64_CFLAGS: Self = Self(1140);
    pub const _CS_POSIX_V7_LP64_OFF64_LDFLAGS: Self = Self(1141);
    pub const _CS_POSIX_V7_LP64_OFF64_LIBS: Self = Self(1142);
    pub const _CS_POSIX_V7_LP64_OFF64_LINTFLAGS: Self = Self(1143);
    pub const _CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS: Self = Self(1144);
    pub const _CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS: Self = Self(1145);
    pub const _CS_POSIX_V7_LPBIG_OFFBIG_LIBS: Self = Self(1146);
    pub const _CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS: Self = Self(1147);
    pub const _CS_V6_ENV: Self = Self(1148);
    pub const _CS_V7_ENV: Self = Self(1149);
}
pub type wchar_t = ::libc::wchar_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: ::core::ffi::c_int,
    pub __value: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub __wch: ::core::ffi::c_uint,
    pub __wchb: [::core::ffi::c_char; 4],
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_wide_data {
    pub _IO_read_ptr: *mut wchar_t,
    pub _IO_read_end: *mut wchar_t,
    pub _IO_read_base: *mut wchar_t,
    pub _IO_write_base: *mut wchar_t,
    pub _IO_write_ptr: *mut wchar_t,
    pub _IO_write_end: *mut wchar_t,
    pub _IO_buf_base: *mut wchar_t,
    pub _IO_buf_end: *mut wchar_t,
    pub _IO_save_base: *mut wchar_t,
    pub _IO_backup_base: *mut wchar_t,
    pub _IO_save_end: *mut wchar_t,
    pub _IO_state: __mbstate_t,
    pub _IO_last_state: __mbstate_t,
    pub _codecvt: _IO_codecvt,
    pub _shortbuf: [wchar_t; 1],
    pub _wide_vtable: *const _IO_jump_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_codecvt {
    pub __cd_in: _IO_iconv_t,
    pub __cd_out: _IO_iconv_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_iconv_t {
    pub step: *mut __gconv_step,
    pub step_data: __gconv_step_data,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gconv_step_data {
    pub __outbuf: *mut ::core::ffi::c_uchar,
    pub __outbufend: *mut ::core::ffi::c_uchar,
    pub __flags: ::core::ffi::c_int,
    pub __invocation_counter: ::core::ffi::c_int,
    pub __internal_use: ::core::ffi::c_int,
    pub __statep: *mut __mbstate_t,
    pub __state: __mbstate_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __gconv_step {
    pub __shlib_handle: *mut __gconv_loaded_object,
    pub __modname: *const ::core::ffi::c_char,
    pub __counter: ::core::ffi::c_int,
    pub __from_name: *mut ::core::ffi::c_char,
    pub __to_name: *mut ::core::ffi::c_char,
    pub __fct: __gconv_fct,
    pub __btowc_fct: __gconv_btowc_fct,
    pub __init_fct: __gconv_init_fct,
    pub __end_fct: __gconv_end_fct,
    pub __min_needed_from: ::core::ffi::c_int,
    pub __max_needed_from: ::core::ffi::c_int,
    pub __min_needed_to: ::core::ffi::c_int,
    pub __max_needed_to: ::core::ffi::c_int,
    pub __stateful: ::core::ffi::c_int,
    pub __data: *mut ::core::ffi::c_void,
}
pub type __gconv_end_fct = Option<unsafe extern "C" fn(*mut __gconv_step) -> ()>;
pub type __gconv_init_fct = Option<unsafe extern "C" fn(*mut __gconv_step) -> ::core::ffi::c_int>;
pub type __gconv_btowc_fct =
    Option<unsafe extern "C" fn(*mut __gconv_step, ::core::ffi::c_uchar) -> wint_t>;
pub type wint_t = ::core::ffi::c_uint;
pub type __gconv_fct = Option<
    unsafe extern "C" fn(
        *mut __gconv_step,
        *mut __gconv_step_data,
        *mut *const ::core::ffi::c_uchar,
        *const ::core::ffi::c_uchar,
        *mut *mut ::core::ffi::c_uchar,
        *mut size_t,
        ::core::ffi::c_int,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut FILE,
    pub _pos: ::core::ffi::c_int,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct conf {
    pub name: *const ::core::ffi::c_char,
    pub call_name: ::core::ffi::c_long,
    pub call: C2Rust_Unnamed_3,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const SYSCONF: Self = Self(0);
    pub const CONFSTR: Self = Self(1);
    pub const PATHCONF: Self = Self(2);
    pub const LIMITS_H: Self = Self(3);
}
pub const __LC_CTYPE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const LC_CTYPE: ::core::ffi::c_int = __LC_CTYPE;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const VERSION: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"2.43\0") };
static mut vars: [conf; 356] = [
    conf {
        name: b"LINK_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_LINK_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"MAX_CANON\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_MAX_CANON.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"MAX_INPUT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_MAX_INPUT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_NAME_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"PATH_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_PATH_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"PIPE_BUF\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_PIPE_BUF.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"SOCK_MAXBUF\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_SOCK_MAXBUF.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"_POSIX_ASYNC_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_ASYNC_IO.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"_POSIX_CHOWN_RESTRICTED\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_CHOWN_RESTRICTED.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"_POSIX_NO_TRUNC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_NO_TRUNC.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"_POSIX_PRIO_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_PRIO_IO.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"_POSIX_SYNC_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_SYNC_IO.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"_POSIX_VDISABLE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_VDISABLE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"ARG_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_ARG_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"ATEXIT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_ATEXIT_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"CHAR_BIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CHAR_BIT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"CHAR_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CHAR_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"CHAR_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CHAR_MIN.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"CHILD_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CHILD_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"CLK_TCK\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CLK_TCK.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"INT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_INT_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"INT_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_INT_MIN.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"IOV_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_UIO_MAXIOV.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LOGNAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LOGIN_NAME_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LONG_BIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LONG_BIT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"MB_LEN_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MB_LEN_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NGROUPS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NGROUPS_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NL_ARGMAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NL_ARGMAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NL_LANGMAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NL_LANGMAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NL_MSGMAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NL_MSGMAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NL_NMAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NL_NMAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NL_SETMAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NL_SETMAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NL_TEXTMAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NL_TEXTMAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NSS_BUFLEN_GROUP\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_GETGR_R_SIZE_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NSS_BUFLEN_PASSWD\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_GETPW_R_SIZE_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NZERO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NZERO.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"OPEN_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_OPEN_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PAGESIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PAGESIZE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PAGE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PAGESIZE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PASS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PASS_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PTHREAD_DESTRUCTOR_ITERATIONS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_DESTRUCTOR_ITERATIONS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PTHREAD_KEYS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_KEYS_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PTHREAD_STACK_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_STACK_MIN.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PTHREAD_THREADS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_THREADS_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SCHAR_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SCHAR_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SCHAR_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SCHAR_MIN.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SHRT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SHRT_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SHRT_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SHRT_MIN.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SSIZE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SSIZE_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"TTY_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TTY_NAME_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"TZNAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TZNAME_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"UCHAR_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_UCHAR_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"UINT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_UINT_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"UIO_MAXIOV\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_UIO_MAXIOV.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"ULONG_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_ULONG_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"USHRT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_USHRT_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"WORD_BIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_WORD_BIT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_AVPHYS_PAGES\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_AVPHYS_PAGES.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_NPROCESSORS_CONF\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NPROCESSORS_CONF.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NPROCESSORS_CONF\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NPROCESSORS_CONF.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_NPROCESSORS_ONLN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NPROCESSORS_ONLN.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"NPROCESSORS_ONLN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NPROCESSORS_ONLN.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_PHYS_PAGES\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PHYS_PAGES.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_ASYNCHRONOUS_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_ASYNCHRONOUS_IO.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_FSYNC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_FSYNC.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_JOB_CONTROL\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_JOB_CONTROL.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MAPPED_FILES\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MAPPED_FILES.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MEMLOCK\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MEMLOCK.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MEMLOCK_RANGE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MEMLOCK_RANGE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MEMORY_PROTECTION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MEMORY_PROTECTION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MESSAGE_PASSING\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MESSAGE_PASSING.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_INTERNET\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_INTERNET.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_INTERNET_DGRAM\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_INTERNET_DGRAM.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_INTERNET_STREAM\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_INTERNET_STREAM.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_OSI\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_OSI.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_OSI_CLTS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_OSI_CLTS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_OSI_COTS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_OSI_COTS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_OSI_M\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_OSI_M.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_SOCKET\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_SOCKET.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PII_XTI\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PII_XTI.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_POLL\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_POLL.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PRIORITIZED_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PRIORITIZED_IO.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PRIORITY_SCHEDULING\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PRIORITY_SCHEDULING.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_REALTIME_SIGNALS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_REALTIME_SIGNALS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SAVED_IDS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SAVED_IDS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SELECT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SELECT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SEMAPHORES\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SEMAPHORES.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SHARED_MEMORY_OBJECTS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SHARED_MEMORY_OBJECTS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SYNCHRONIZED_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SYNCHRONIZED_IO.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREADS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREADS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_ATTR_STACKADDR\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_ATTR_STACKADDR.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_ATTR_STACKSIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_ATTR_STACKSIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_PRIORITY_SCHEDULING\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_PRIORITY_SCHEDULING.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_PRIO_INHERIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_PRIO_INHERIT.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_PRIO_PROTECT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_PRIO_PROTECT.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_ROBUST_PRIO_INHERIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_ROBUST_PRIO_INHERIT.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_ROBUST_PRIO_PROTECT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_ROBUST_PRIO_PROTECT.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_PROCESS_SHARED\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_PROCESS_SHARED.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_SAFE_FUNCTIONS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_SAFE_FUNCTIONS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TIMERS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TIMERS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"TIMER_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TIMER_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_VERSION.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_T_IOV_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_T_IOV_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_CRYPT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_CRYPT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_ENH_I18N\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_ENH_I18N.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_LEGACY\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_LEGACY.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_REALTIME\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_REALTIME.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_REALTIME_THREADS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_REALTIME_THREADS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_SHM\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_SHM.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_UNIX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_UNIX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_VERSION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_XCU_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_XCU_VERSION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_XPG2\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_XPG2.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_XPG3\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_XPG3.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_XOPEN_XPG4\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XOPEN_XPG4.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"BC_BASE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_BASE_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"BC_DIM_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_DIM_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"BC_SCALE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_SCALE_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"BC_STRING_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_STRING_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"CHARCLASS_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CHARCLASS_NAME_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"COLL_WEIGHTS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_COLL_WEIGHTS_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"EQUIV_CLASS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_EQUIV_CLASS_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"EXPR_NEST_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_EXPR_NEST_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LINE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LINE_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_BC_BASE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_BASE_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_BC_DIM_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_DIM_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_BC_SCALE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_SCALE_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_BC_STRING_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BC_STRING_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_CHAR_TERM\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_CHAR_TERM.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_COLL_WEIGHTS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_COLL_WEIGHTS_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_C_BIND\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_C_BIND.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_C_DEV\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_C_DEV.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_C_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_C_VERSION.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_EXPR_NEST_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_EXPR_NEST_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_FORT_DEV\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_FORT_DEV.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_FORT_RUN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_FORT_RUN.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_LINE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LINE_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_LOCALEDEF\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_LOCALEDEF.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_RE_DUP_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_RE_DUP_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_SW_DEV\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_SW_DEV.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_UPE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_UPE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_VERSION.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"RE_DUP_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_RE_DUP_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"PATH\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_PATH.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"CS_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_PATH.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS_CFLAGS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS_LDFLAGS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS_LIBS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS64_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS64_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS64_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS64_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS64_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS64_LIBS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"LFS64_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_LFS64_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_XBS5_WIDTH_RESTRICTED_ENVS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_V5_WIDTH_RESTRICTED_ENVS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_WIDTH_RESTRICTED_ENVS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_V5_WIDTH_RESTRICTED_ENVS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_XBS5_ILP32_OFF32\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XBS5_ILP32_OFF32.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"XBS5_ILP32_OFF32_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFF32_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_ILP32_OFF32_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFF32_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_ILP32_OFF32_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFF32_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_ILP32_OFF32_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFF32_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_XBS5_ILP32_OFFBIG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XBS5_ILP32_OFFBIG.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"XBS5_ILP32_OFFBIG_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFFBIG_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_ILP32_OFFBIG_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFFBIG_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_ILP32_OFFBIG_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFFBIG_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_ILP32_OFFBIG_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_ILP32_OFFBIG_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_XBS5_LP64_OFF64\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XBS5_LP64_OFF64.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"XBS5_LP64_OFF64_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LP64_OFF64_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_LP64_OFF64_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LP64_OFF64_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_LP64_OFF64_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LP64_OFF64_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_LP64_OFF64_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LP64_OFF64_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_XBS5_LPBIG_OFFBIG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_XBS5_LPBIG_OFFBIG.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"XBS5_LPBIG_OFFBIG_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LPBIG_OFFBIG_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_LPBIG_OFFBIG_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LPBIG_OFFBIG_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_LPBIG_OFFBIG_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LPBIG_OFFBIG_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"XBS5_LPBIG_OFFBIG_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_XBS5_LPBIG_OFFBIG_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V6_ILP32_OFF32\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V6_ILP32_OFF32.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFF32_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFF32_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFF32_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFF32_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFF32_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFF32_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFF32_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFF32_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V6_WIDTH_RESTRICTED_ENVS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_V6_WIDTH_RESTRICTED_ENVS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_WIDTH_RESTRICTED_ENVS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_V6_WIDTH_RESTRICTED_ENVS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V6_ILP32_OFFBIG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V6_ILP32_OFFBIG.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFFBIG_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFFBIG_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFFBIG_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFFBIG_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFFBIG_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFFBIG_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_ILP32_OFFBIG_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_ILP32_OFFBIG_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V6_LP64_OFF64\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V6_LP64_OFF64.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V6_LP64_OFF64_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LP64_OFF64_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_LP64_OFF64_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LP64_OFF64_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_LP64_OFF64_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LP64_OFF64_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_LP64_OFF64_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LP64_OFF64_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V6_LPBIG_OFFBIG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V6_LPBIG_OFFBIG.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V6_LPBIG_OFFBIG_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LPBIG_OFFBIG_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_LPBIG_OFFBIG_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LPBIG_OFFBIG_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_LPBIG_OFFBIG_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LPBIG_OFFBIG_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V6_LPBIG_OFFBIG_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V6_LPBIG_OFFBIG_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V7_ILP32_OFF32\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V7_ILP32_OFF32.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFF32_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFF32_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFF32_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFF32_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFF32_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFF32_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFF32_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFF32_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V7_WIDTH_RESTRICTED_ENVS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_V7_WIDTH_RESTRICTED_ENVS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_WIDTH_RESTRICTED_ENVS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_V7_WIDTH_RESTRICTED_ENVS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V7_ILP32_OFFBIG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V7_ILP32_OFFBIG.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFFBIG_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFFBIG_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFFBIG_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFFBIG_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFFBIG_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFFBIG_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_ILP32_OFFBIG_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_ILP32_OFFBIG_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V7_LP64_OFF64\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V7_LP64_OFF64.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V7_LP64_OFF64_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LP64_OFF64_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_LP64_OFF64_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LP64_OFF64_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_LP64_OFF64_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LP64_OFF64_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_LP64_OFF64_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LP64_OFF64_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_V7_LPBIG_OFFBIG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_V7_LPBIG_OFFBIG.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX_V7_LPBIG_OFFBIG_CFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LPBIG_OFFBIG_CFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_LPBIG_OFFBIG_LDFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LPBIG_OFFBIG_LDFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_LPBIG_OFFBIG_LIBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LPBIG_OFFBIG_LIBS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX_V7_LPBIG_OFFBIG_LINTFLAGS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_POSIX_V7_LPBIG_OFFBIG_LINTFLAGS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"_POSIX_ADVISORY_INFO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_ADVISORY_INFO.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_BARRIERS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BARRIERS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_BASE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_BASE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_C_LANG_SUPPORT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_C_LANG_SUPPORT.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_C_LANG_SUPPORT_R\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_C_LANG_SUPPORT_R.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_CLOCK_SELECTION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CLOCK_SELECTION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_CPUTIME\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_CPUTIME.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_CPUTIME\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_CPUTIME.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_DEVICE_SPECIFIC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_DEVICE_SPECIFIC.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_DEVICE_SPECIFIC_R\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_DEVICE_SPECIFIC_R.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_FD_MGMT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_FD_MGMT.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_FIFO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_FIFO.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_PIPE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_PIPE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_FILE_ATTRIBUTES\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_FILE_ATTRIBUTES.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_FILE_LOCKING\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_FILE_LOCKING.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_FILE_SYSTEM\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_FILE_SYSTEM.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MONOTONIC_CLOCK\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MONOTONIC_CLOCK.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_MULTI_PROCESS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MULTI_PROCESS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SINGLE_PROCESS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SINGLE_PROCESS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_NETWORKING\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_NETWORKING.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_READER_WRITER_LOCKS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_READER_WRITER_LOCKS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SPIN_LOCKS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SPIN_LOCKS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_REGEXP\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_REGEXP.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_REGEX_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_REGEX_VERSION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SHELL\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SHELL.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SIGNALS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SIGNALS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SPAWN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SPAWN.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SPORADIC_SERVER\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SPORADIC_SERVER.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_THREAD_SPORADIC_SERVER\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_THREAD_SPORADIC_SERVER.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SYSTEM_DATABASE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SYSTEM_DATABASE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_SYSTEM_DATABASE_R\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SYSTEM_DATABASE_R.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TIMEOUTS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TIMEOUTS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TYPED_MEMORY_OBJECTS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TYPED_MEMORY_OBJECTS.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_USER_GROUPS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_USER_GROUPS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_USER_GROUPS_R\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_USER_GROUPS_R.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_PBS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_PBS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_PBS_ACCOUNTING\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_PBS_ACCOUNTING.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_PBS_LOCATE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_PBS_LOCATE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_PBS_TRACK\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_PBS_TRACK.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"POSIX2_PBS_MESSAGE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_2_PBS_MESSAGE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SYMLOOP_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SYMLOOP_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"STREAM_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_STREAM_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"AIO_LISTIO_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_AIO_LISTIO_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"AIO_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_AIO_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"AIO_PRIO_DELTA_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_AIO_PRIO_DELTA_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"DELAYTIMER_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_DELAYTIMER_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"HOST_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_HOST_NAME_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LOGIN_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LOGIN_NAME_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"MQ_OPEN_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MQ_OPEN_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"MQ_PRIO_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_MQ_PRIO_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_DEVICE_IO\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_DEVICE_IO.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TRACE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TRACE.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TRACE_EVENT_FILTER\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TRACE_EVENT_FILTER.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TRACE_INHERIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TRACE_INHERIT.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_TRACE_LOG\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_TRACE_LOG.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"RTSIG_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_RTSIG_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SEM_NSEMS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SEM_NSEMS_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SEM_VALUE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SEM_VALUE_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"SIGQUEUE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_SIGQUEUE_MAX.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"FILESIZEBITS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_FILESIZEBITS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"POSIX_ALLOC_SIZE_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_ALLOC_SIZE_MIN.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"POSIX_REC_INCR_XFER_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_REC_INCR_XFER_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"POSIX_REC_MAX_XFER_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_REC_MAX_XFER_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"POSIX_REC_MIN_XFER_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_REC_MIN_XFER_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"POSIX_REC_XFER_ALIGN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_REC_XFER_ALIGN.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"SYMLINK_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_SYMLINK_MAX.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"GNU_LIBC_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_GNU_LIBC_VERSION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"GNU_LIBPTHREAD_VERSION\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_1::_CS_GNU_LIBPTHREAD_VERSION.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::CONFSTR,
    },
    conf {
        name: b"POSIX2_SYMLINKS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed::_PC_2_SYMLINKS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::PATHCONF,
    },
    conf {
        name: b"LEVEL1_ICACHE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL1_ICACHE_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL1_ICACHE_ASSOC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL1_ICACHE_ASSOC.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL1_ICACHE_LINESIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL1_ICACHE_LINESIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL1_DCACHE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL1_DCACHE_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL1_DCACHE_ASSOC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL1_DCACHE_ASSOC.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL1_DCACHE_LINESIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL1_DCACHE_LINESIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL2_CACHE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL2_CACHE_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL2_CACHE_ASSOC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL2_CACHE_ASSOC.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL2_CACHE_LINESIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL2_CACHE_LINESIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL3_CACHE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL3_CACHE_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL3_CACHE_ASSOC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL3_CACHE_ASSOC.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL3_CACHE_LINESIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL3_CACHE_LINESIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL4_CACHE_SIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL4_CACHE_SIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL4_CACHE_ASSOC\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL4_CACHE_ASSOC.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"LEVEL4_CACHE_LINESIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_LEVEL4_CACHE_LINESIZE.0 as ::core::ffi::c_int
            as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"IPV6\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_IPV6.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"RAW_SOCKETS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_RAW_SOCKETS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_IPV6\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_IPV6.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_RAW_SOCKETS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: C2Rust_Unnamed_0::_SC_RAW_SOCKETS.0 as ::core::ffi::c_int as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
    conf {
        name: b"_POSIX_CLOCKRES_MIN\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 20000000 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_AIO_LISTIO_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 2 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_AIO_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 1 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_ARG_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 4096 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_CHILD_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 25 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_DELAYTIMER_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_HOST_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 255 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_LINK_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 8 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_LOGIN_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 9 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_MAX_CANON\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 255 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_MAX_INPUT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 255 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_MQ_OPEN_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 8 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_MQ_PRIO_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 14 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_NGROUPS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 8 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_OPEN_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 20 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_PATH_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 256 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_PIPE_BUF\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 512 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_RE_DUP_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 255 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_RTSIG_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 8 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_SEM_NSEMS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 256 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_SEM_VALUE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32767 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_SIGQUEUE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_SSIZE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32767 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_STREAM_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 8 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_SYMLINK_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 255 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_SYMLOOP_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 8 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_THREAD_DESTRUCTOR_ITERATIONS\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 4 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_THREAD_KEYS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 128 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_THREAD_THREADS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 64 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_TIMER_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_TTY_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 9 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_TZNAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 6 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_BC_BASE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 99 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_BC_DIM_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 2048 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_BC_SCALE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 99 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_BC_STRING_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 1000 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_CHARCLASS_NAME_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 14 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_COLL_WEIGHTS_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 2 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_EXPR_NEST_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 32 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_LINE_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 2048 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX2_RE_DUP_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 255 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_XOPEN_IOV_MAX\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 16 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_FD_SETSIZE\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 20 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_HIWAT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 512 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_QLIMIT\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 1 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: b"_POSIX_UIO_MAXIOV\0".as_ptr() as *const ::core::ffi::c_char,
        call_name: 16 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::LIMITS_H,
    },
    conf {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        call_name: 0 as ::core::ffi::c_long,
        call: C2Rust_Unnamed_3::SYSCONF,
    },
];
unsafe extern "C" fn usage() {
    fprintf(
        stderr,
        __dcgettext(
            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
            b"Usage: %s [-v specification] variable_name [pathname]\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        __progname,
    );
    fprintf(
        stderr,
        __dcgettext(
            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
            b"       %s -a [pathname]\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        __progname,
    );
    exit(2 as ::core::ffi::c_int);
}
unsafe extern "C" fn print_all(mut path: *const ::core::ffi::c_char) {
    let mut c: *const conf = ::core::ptr::null::<conf>();
    let mut clen: size_t = 0;
    let mut value: ::core::ffi::c_long = 0;
    let mut cvalue: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    c = &raw const vars as *const conf;
    while !(*c).name.is_null() {
        printf(b"%-36s\0".as_ptr() as *const ::core::ffi::c_char, (*c).name);
        match (*c).call {
            C2Rust_Unnamed_3::PATHCONF => {
                value = pathconf(path, (*c).call_name as ::core::ffi::c_int);
                if value != -1 as ::core::ffi::c_long {
                    printf(b"%ld\0".as_ptr() as *const ::core::ffi::c_char, value);
                }
                printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            C2Rust_Unnamed_3::SYSCONF => {
                value = sysconf((*c).call_name as ::core::ffi::c_int);
                if value == -1 as ::core::ffi::c_long {
                    if (*c).call_name
                        == C2Rust_Unnamed_0::_SC_UINT_MAX.0 as ::core::ffi::c_int
                            as ::core::ffi::c_long
                        || (*c).call_name
                            == C2Rust_Unnamed_0::_SC_ULONG_MAX.0 as ::core::ffi::c_int
                                as ::core::ffi::c_long
                    {
                        printf(b"%lu\0".as_ptr() as *const ::core::ffi::c_char, value);
                    }
                } else {
                    printf(b"%ld\0".as_ptr() as *const ::core::ffi::c_char, value);
                }
                printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
            C2Rust_Unnamed_3::CONFSTR => {
                clen = confstr(
                    (*c).call_name as ::core::ffi::c_int,
                    NULL as *mut ::core::ffi::c_char,
                    0 as size_t,
                );
                cvalue = malloc(clen) as *mut ::core::ffi::c_char;
                if cvalue.is_null() {
                    error(
                        3 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        __dcgettext(
                            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                            b"memory exhausted\0".as_ptr() as *const ::core::ffi::c_char,
                            LC_MESSAGES,
                        ),
                    );
                }
                if confstr((*c).call_name as ::core::ffi::c_int, cvalue, clen) != clen {
                    error(
                        3 as ::core::ffi::c_int,
                        *__errno_location(),
                        b"confstr\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
                printf(
                    b"%.*s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    clen as ::core::ffi::c_int,
                    cvalue,
                );
                free(cvalue as *mut ::core::ffi::c_void);
            }
            C2Rust_Unnamed_3::LIMITS_H => {
                printf(
                    b"%ld\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*c).call_name,
                );
            }
            _ => {}
        }
        c = c.offset(1);
    }
    exit(0 as ::core::ffi::c_int);
}
unsafe extern "C" fn rboxc_getconf_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut c: *const conf = ::core::ptr::null::<conf>();
    setlocale(LC_CTYPE, b"\0".as_ptr() as *const ::core::ffi::c_char);
    setlocale(LC_MESSAGES, b"\0".as_ptr() as *const ::core::ffi::c_char);
    textdomain(&raw const _libc_intl_domainname as *const ::core::ffi::c_char);
    if argc > 1 as ::core::ffi::c_int
        && strcmp(
            *argv.offset(1isize),
            b"--version\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        printf(
            b"getconf %s%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            PKGVERSION.as_ptr(),
            VERSION.as_ptr(),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Copyright (C) %s Free Software Foundation, Inc.\nThis is free software; see the source for copying conditions.  There is NO\nwarranty; not even for MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            b"2024\0".as_ptr() as *const ::core::ffi::c_char,
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Written by %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            b"Roland McGrath\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 0 as ::core::ffi::c_int;
    }
    if argc > 1 as ::core::ffi::c_int
        && strcmp(
            *argv.offset(1isize),
            b"--help\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Usage: getconf [-v SPEC] VAR\n  or:  getconf [-v SPEC] PATH_VAR PATH\n\nGet the configuration value for variable VAR, or for variable PATH_VAR\nfor path PATH.  If SPEC is given, give values for compilation\nenvironment SPEC.\n\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"For bug reporting instructions, please see:\n%s.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            REPORT_BUGS_TO.as_ptr(),
        );
        return 0 as ::core::ffi::c_int;
    }
    if argc > 1 as ::core::ffi::c_int
        && strncmp(
            *argv.offset(1isize),
            b"-v\0".as_ptr() as *const ::core::ffi::c_char,
            2 as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if *(*argv.offset(1isize)).offset(2isize) as ::core::ffi::c_int
            == '\0' as ::core::ffi::c_int
        {
            if argc < 3 as ::core::ffi::c_int {
                usage();
            }
            argv = argv.offset(2 as ::core::ffi::c_int as isize);
            argc -= 2 as ::core::ffi::c_int;
        } else {
            argv = argv.offset(1 as ::core::ffi::c_int as isize);
            argc -= 1 as ::core::ffi::c_int;
        }
    }
    if argc > 1 as ::core::ffi::c_int
        && strcmp(
            *argv.offset(1isize),
            b"-a\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        if argc == 2 as ::core::ffi::c_int {
            print_all(b"/\0".as_ptr() as *const ::core::ffi::c_char);
        } else if argc == 3 as ::core::ffi::c_int {
            print_all(*argv.offset(2isize));
        } else {
            usage();
        }
    }
    let mut ai: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if argc > ai
        && strcmp(
            *argv.offset(ai as isize),
            b"--\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        ai += 1;
    }
    if argc - ai < 1 as ::core::ffi::c_int || argc - ai > 2 as ::core::ffi::c_int {
        usage();
    }
    c = &raw const vars as *const conf;
    while !(*c).name.is_null() {
        if strcmp((*c).name, *argv.offset(ai as isize)) == 0 as ::core::ffi::c_int
            || strncmp(
                (*c).name,
                b"_POSIX_\0".as_ptr() as *const ::core::ffi::c_char,
                7 as size_t,
            ) == 0 as ::core::ffi::c_int
                && strcmp(
                    (*c).name.offset(7 as ::core::ffi::c_int as isize),
                    *argv.offset(ai as isize),
                ) == 0 as ::core::ffi::c_int
        {
            let mut value: ::core::ffi::c_long = 0;
            let mut clen: size_t = 0;
            let mut cvalue: *mut ::core::ffi::c_char =
                ::core::ptr::null_mut::<::core::ffi::c_char>();
            match (*c).call {
                C2Rust_Unnamed_3::PATHCONF => {
                    if argc - ai < 2 as ::core::ffi::c_int {
                        usage();
                    }
                    *__errno_location() = 0 as ::core::ffi::c_int;
                    value = pathconf(
                        *argv.offset((ai + 1 as ::core::ffi::c_int) as isize),
                        (*c).call_name as ::core::ffi::c_int,
                    );
                    if value == -1 as ::core::ffi::c_long {
                        if *__errno_location() != 0 {
                            error(
                                3 as ::core::ffi::c_int,
                                *__errno_location(),
                                b"pathconf: %s\0".as_ptr() as *const ::core::ffi::c_char,
                                *argv.offset((ai + 1 as ::core::ffi::c_int) as isize),
                            );
                        } else {
                            puts(__dcgettext(
                                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                b"undefined\0".as_ptr() as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ));
                        }
                    } else {
                        printf(b"%ld\n\0".as_ptr() as *const ::core::ffi::c_char, value);
                    }
                    exit(0 as ::core::ffi::c_int);
                }
                C2Rust_Unnamed_3::SYSCONF => {
                    if argc - ai > 1 as ::core::ffi::c_int {
                        usage();
                    }
                    value = sysconf((*c).call_name as ::core::ffi::c_int);
                    if value == -1 as ::core::ffi::c_long {
                        if (*c).call_name
                            == C2Rust_Unnamed_0::_SC_UINT_MAX.0 as ::core::ffi::c_int
                                as ::core::ffi::c_long
                            || (*c).call_name
                                == C2Rust_Unnamed_0::_SC_ULONG_MAX.0 as ::core::ffi::c_int
                                    as ::core::ffi::c_long
                        {
                            printf(b"%lu\n\0".as_ptr() as *const ::core::ffi::c_char, value);
                        } else {
                            puts(__dcgettext(
                                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                b"undefined\0".as_ptr() as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ));
                        }
                    } else {
                        printf(b"%ld\n\0".as_ptr() as *const ::core::ffi::c_char, value);
                    }
                    exit(0 as ::core::ffi::c_int);
                }
                C2Rust_Unnamed_3::CONFSTR => {
                    if argc - ai > 1 as ::core::ffi::c_int {
                        usage();
                    }
                    clen = confstr(
                        (*c).call_name as ::core::ffi::c_int,
                        NULL as *mut ::core::ffi::c_char,
                        0 as size_t,
                    );
                    cvalue = malloc(clen) as *mut ::core::ffi::c_char;
                    if cvalue.is_null() {
                        error(
                            3 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            __dcgettext(
                                &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
                                b"memory exhausted\0".as_ptr() as *const ::core::ffi::c_char,
                                LC_MESSAGES,
                            ),
                        );
                    }
                    if confstr((*c).call_name as ::core::ffi::c_int, cvalue, clen) != clen {
                        error(
                            3 as ::core::ffi::c_int,
                            *__errno_location(),
                            b"confstr\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                    printf(
                        b"%.*s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        clen as ::core::ffi::c_int,
                        cvalue,
                    );
                    exit(0 as ::core::ffi::c_int);
                }
                C2Rust_Unnamed_3::LIMITS_H => {
                    if argc - ai > 1 as ::core::ffi::c_int {
                        usage();
                    }
                    printf(
                        b"%ld\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (*c).call_name,
                    );
                    exit(0 as ::core::ffi::c_int);
                }
                _ => {}
            }
        }
        c = c.offset(1);
    }
    error(
        2 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
        __dcgettext(
            &raw const _libc_intl_domainname as *const ::core::ffi::c_char,
            b"Unrecognized variable `%s'\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        *argv.offset(ai as isize),
    );
    return 2 as ::core::ffi::c_int;
}
pub const PKGVERSION: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"(GNU libc) \0") };
pub const REPORT_BUGS_TO: [::core::ffi::c_char; 46] = unsafe {
    ::core::mem::transmute::<[u8; 46], [::core::ffi::c_char; 46]>(
        *b"<https://www.gnu.org/software/libc/bugs.html>\0",
    )
};

extern "C" {
    #[link_name = "program_invocation_name"]
    static mut RBOXC_LIBC_INVOCATION: *mut ::core::ffi::c_char;
    #[link_name = "program_invocation_short_name"]
    static mut RBOXC_LIBC_SHORT_INVOCATION: *mut ::core::ffi::c_char;
    #[link_name = "error_print_progname"]
    static mut RBOXC_ERROR_PRINT_PROGNAME: Option<unsafe extern "C" fn()>;
    #[link_name = "stderr"]
    static mut RBOXC_ERROR_STDERR: *mut libc::FILE;
}
unsafe extern "C" fn rboxc_glibc_error_prefix() {
    libc::fprintf(RBOXC_ERROR_STDERR, b"%s: \0".as_ptr().cast(), RBOXC_LIBC_INVOCATION);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_getconf(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    RBOXC_LIBC_INVOCATION = *argv;
    let bytes = ::core::ffi::CStr::from_ptr(*argv).to_bytes();
    let offset = bytes.iter().rposition(|b| *b == b'/').map_or(0, |i| i+1);
    RBOXC_LIBC_SHORT_INVOCATION = (*argv).add(offset);
    RBOXC_ERROR_PRINT_PROGNAME = Some(rboxc_glibc_error_prefix);
    rboxc_getconf_main_inner(argc, argv)
}
