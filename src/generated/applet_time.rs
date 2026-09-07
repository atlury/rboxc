// Generated from pinned GNU Time 1.10 by scripts/translate-time.py.
// Source SHA-256: 64f6e009e1ae32a77244e2781322f78fb8043af15cf8ac973afbaea2369fee44
/* time - display the resource usage of a process.
   Copyright (C) 1990-2021, 2026 Free Software Foundation, Inc.

   Originally written by David Keppel <pardo@cs.washington.edu>.
   Heavily modified by David MacKenzie <djm@gnu.ai.mit.edu>.
   Heavily modified (again) by Assaf Gordon <assafgordon@gmail.com>.

   This file is part of GNU Time.

   GNU Time is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   GNU Time is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with GNU Time.  If not, see <http://www.gnu.org/licenses/>.
*/
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn execvp(
        __file: *const ::core::ffi::c_char,
        __argv: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn fork() -> __pid_t;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getpagesize() -> ::core::ffi::c_int;
    #[link_name = "rboxc_time_sig2str"]
    fn sig2str(signo: ::core::ffi::c_int, str: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn getrusage(__who: __rusage_who_t, __usage: *mut rusage) -> ::core::ffi::c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush_unlocked(__stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn fputs_unlocked(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn __overflow(_: *mut FILE, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_time_program_name"]
    static mut program_name: *const ::core::ffi::c_char;
    #[link_name = "rboxc_time_set_program_name"]
    fn set_program_name(argv0: *const ::core::ffi::c_char);
    fn error(
        __status: ::core::ffi::c_int,
        __errnum: ::core::ffi::c_int,
        __format: *const ::core::ffi::c_char,
        ...
    );
    #[link_name = "rboxc_time_timespec_sub"]
    fn timespec_sub(_: timespec, _: timespec) -> timespec;
    #[link_name = "rboxc_time_current_timespec"]
    fn current_timespec() -> timespec;
    #[link_name = "rboxc_time_Version"]
    static mut Version: *const ::core::ffi::c_char;
    #[link_name = "rboxc_time_version_etc"]
    fn version_etc(
        stream: *mut FILE,
        command_name: *const ::core::ffi::c_char,
        package: *const ::core::ffi::c_char,
        version: *const ::core::ffi::c_char,
        ...
    );
}
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type pid_t = __pid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type sighandler_t = __sighandler_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rusage {
    pub ru_utime: timeval,
    pub ru_stime: timeval,
    pub c2rust_unnamed: C2Rust_Unnamed_12,
    pub c2rust_unnamed_0: C2Rust_Unnamed_11,
    pub c2rust_unnamed_1: C2Rust_Unnamed_10,
    pub c2rust_unnamed_2: C2Rust_Unnamed_9,
    pub c2rust_unnamed_3: C2Rust_Unnamed_8,
    pub c2rust_unnamed_4: C2Rust_Unnamed_7,
    pub c2rust_unnamed_5: C2Rust_Unnamed_6,
    pub c2rust_unnamed_6: C2Rust_Unnamed_5,
    pub c2rust_unnamed_7: C2Rust_Unnamed_4,
    pub c2rust_unnamed_8: C2Rust_Unnamed_3,
    pub c2rust_unnamed_9: C2Rust_Unnamed_2,
    pub c2rust_unnamed_10: C2Rust_Unnamed_1,
    pub c2rust_unnamed_11: C2Rust_Unnamed_0,
    pub c2rust_unnamed_12: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub ru_nivcsw: ::core::ffi::c_long,
    pub __ru_nivcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub ru_nvcsw: ::core::ffi::c_long,
    pub __ru_nvcsw_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_1 {
    pub ru_nsignals: ::core::ffi::c_long,
    pub __ru_nsignals_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_2 {
    pub ru_msgrcv: ::core::ffi::c_long,
    pub __ru_msgrcv_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_3 {
    pub ru_msgsnd: ::core::ffi::c_long,
    pub __ru_msgsnd_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_4 {
    pub ru_oublock: ::core::ffi::c_long,
    pub __ru_oublock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_5 {
    pub ru_inblock: ::core::ffi::c_long,
    pub __ru_inblock_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_6 {
    pub ru_nswap: ::core::ffi::c_long,
    pub __ru_nswap_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_7 {
    pub ru_majflt: ::core::ffi::c_long,
    pub __ru_majflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_8 {
    pub ru_minflt: ::core::ffi::c_long,
    pub __ru_minflt_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_9 {
    pub ru_isrss: ::core::ffi::c_long,
    pub __ru_isrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_10 {
    pub ru_idrss: ::core::ffi::c_long,
    pub __ru_idrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_11 {
    pub ru_ixrss: ::core::ffi::c_long,
    pub __ru_ixrss_word: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub ru_maxrss: ::core::ffi::c_long,
    pub __ru_maxrss_word: __syscall_slong_t,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct __rusage_who(pub ::core::ffi::c_int);
impl __rusage_who {
    pub const RUSAGE_SELF: Self = Self(0);
    pub const RUSAGE_CHILDREN: Self = Self(-1);
    pub const RUSAGE_THREAD: Self = Self(1);
}
pub type __rusage_who_t = __rusage_who;
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
pub struct option {
    pub name: *const ::core::ffi::c_char,
    pub has_arg: ::core::ffi::c_int,
    pub flag: *mut ::core::ffi::c_int,
    pub val: ::core::ffi::c_int,
}
pub type uintmax_t = ::libc::uintmax_t;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_13(pub ::core::ffi::c_int);
impl C2Rust_Unnamed_13 {
    pub const GETOPT_HELP_CHAR: Self = Self(-130);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RESUSE {
    pub waitstatus: ::core::ffi::c_int,
    pub ru: rusage,
    pub start_time: timespec,
    pub end_time: timespec,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_14(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_14 {
    pub const EXIT_CANCELED: Self = Self(125);
    pub const EXIT_CANNOT_INVOKE: Self = Self(126);
    pub const EXIT_ENOENT: Self = Self(127);
}
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const _IO_ERR_SEEN: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn fputc_unlocked(
    mut __c: ::core::ffi::c_int,
    mut __stream: *mut FILE,
) -> ::core::ffi::c_int {
    return if ((*__stream)._IO_write_ptr >= (*__stream)._IO_write_end) as ::core::ffi::c_int
        as ::core::ffi::c_long
        != 0
    {
        __overflow(__stream, __c as ::core::ffi::c_uchar as ::core::ffi::c_int)
    } else {
        let c2rust_fresh0 = (*__stream)._IO_write_ptr;
        (*__stream)._IO_write_ptr = (*__stream)._IO_write_ptr.offset(1);
        let c2rust_lvalue_ptr = &raw mut *c2rust_fresh0;
        *c2rust_lvalue_ptr = __c as ::core::ffi::c_char;
        *c2rust_lvalue_ptr as ::core::ffi::c_uchar as ::core::ffi::c_int
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
unsafe extern "C" fn ferror_unlocked(mut __stream: *mut FILE) -> ::core::ffi::c_int {
    return ((*__stream)._flags & _IO_ERR_SEEN != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const ENOENT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EXIT_SUCCESS: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const USAGE_BUILTIN_WARNING: [::core::ffi::c_char; 185] = unsafe {
    ::core::mem::transmute::<
        [u8; 185],
        [::core::ffi::c_char; 185],
    >(
        *b"\nYour shell may have its own version of %s, which usually supersedes\nthe version described here.  Please refer to your shell's documentation\nfor details about the options it supports.\n\0",
    )
};
#[inline]
unsafe extern "C" fn get_rusage_maxrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed.ru_maxrss as uintmax_t;
}
#[inline]
unsafe extern "C" fn get_rusage_ixrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed_0.ru_ixrss as uintmax_t;
}
#[inline]
unsafe extern "C" fn get_rusage_idrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed_1.ru_idrss as uintmax_t;
}
#[inline]
unsafe extern "C" fn get_rusage_isrss_kb(mut ru: *const rusage) -> uintmax_t {
    return (*ru).c2rust_unnamed_2.ru_isrss as uintmax_t;
}
pub const SIGNALLED_OFFSET: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const TICKS_PER_SEC: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const MSEC_PER_TICK: ::core::ffi::c_int = 1000 as ::core::ffi::c_int / TICKS_PER_SEC;
static mut default_format: *const ::core::ffi::c_char = b"%Uuser %Ssystem %Eelapsed %PCPU (%Xavgtext+%Davgdata %Mmaxresident)k\n%Iinputs+%Ooutputs (%Fmajor+%Rminor)pagefaults %Wswaps\0"
    .as_ptr() as *const ::core::ffi::c_char;
static mut posix_format: *const ::core::ffi::c_char =
    b"real %e\nuser %U\nsys %S\0".as_ptr() as *const ::core::ffi::c_char;
static mut verbose_format: *const ::core::ffi::c_char = b"\tCommand being timed: \"%C\"\n\tUser time (seconds): %U\n\tSystem time (seconds): %S\n\tPercent of CPU this job got: %P\n\tElapsed (wall clock) time (h:mm:ss or m:ss): %E\n\tAverage shared text size (kbytes): %X\n\tAverage unshared data size (kbytes): %D\n\tAverage stack size (kbytes): %p\n\tAverage total size (kbytes): %K\n\tMaximum resident set size (kbytes): %M\n\tAverage resident set size (kbytes): %t\n\tMajor (requiring I/O) page faults: %F\n\tMinor (reclaiming a frame) page faults: %R\n\tVoluntary context switches: %w\n\tInvoluntary context switches: %c\n\tSwaps: %W\n\tFile system inputs: %I\n\tFile system outputs: %O\n\tSocket messages sent: %s\n\tSocket messages received: %r\n\tSignals delivered: %k\n\tPage size (bytes): %Z\n\tExit status: %x\0"
    .as_ptr() as *const ::core::ffi::c_char;
static mut verbose: bool = r#false != 0;
static mut outfile: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut outfp: *mut FILE = ::core::ptr::null_mut::<FILE>();
static mut append: bool = r#false != 0;
static mut output_format: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
static mut quiet: bool = r#false != 0;
static mut longopts: [option; 9] = [
    option {
        name: b"append\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'a' as ::core::ffi::c_int,
    },
    option {
        name: b"format\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'f' as ::core::ffi::c_int,
    },
    option {
        name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: C2Rust_Unnamed_13::GETOPT_HELP_CHAR.0,
    },
    option {
        name: b"output-file\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: required_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'o' as ::core::ffi::c_int,
    },
    option {
        name: b"portability\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'p' as ::core::ffi::c_int,
    },
    option {
        name: b"quiet\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'q' as ::core::ffi::c_int,
    },
    option {
        name: b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'v' as ::core::ffi::c_int,
    },
    option {
        name: b"version\0".as_ptr() as *const ::core::ffi::c_char,
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 'V' as ::core::ffi::c_int,
    },
    option {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        has_arg: no_argument,
        flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
        val: 0 as ::core::ffi::c_int,
    },
];
pub const PROGRAM_NAME: [::core::ffi::c_char; 5] =
    unsafe { ::core::mem::transmute::<[u8; 5], [::core::ffi::c_char; 5]>(*b"time\0") };
unsafe extern "C" fn usage(mut status: ::core::ffi::c_int) {
    if status != EXIT_SUCCESS {
        fprintf(
            stderr,
            b"Try '%s --help' for more information.\n\0".as_ptr() as *const ::core::ffi::c_char,
            program_name,
        );
        exit(status);
    }
    printf(
        b"Usage: %s [OPTIONS] COMMAND [ARG]...\n\0".as_ptr() as *const ::core::ffi::c_char,
        program_name,
    );
    fputs_unlocked(
        b"Run COMMAND, then print system resource usage.\n\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -a, --append              with -o FILE, append instead of overwriting\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -f, --format=FORMAT       use the specified FORMAT instead of the default\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -o, --output=FILE         write to FILE instead of STDERR\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -p, --portability         print POSIX standard 1003.2 conformant string:\n                              real %e\n                              user %U\n                              sys %S\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -q, --quiet               do not print information about abnormal program\n                            termination (non-zero exit codes or signals)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -v, --verbose             print all resource usage information instead of\n                            the default format\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"       --help               display this help and exit\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  -V,  --version            output version information and exit\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"\nCommonly usaged format sequences for -f/--format:\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"(see documentation for full list)\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %%   a literal '%'\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %C   command line and arguments\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %c   involuntary context switches\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %E   elapsed real time (wall clock) in [hour:]min:sec\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %e   elapsed real time (wall clock) in seconds\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %F   major page faults\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %M   maximum resident set size in KB\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %P   percent of CPU this job got\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %R   minor page faults\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %S   system (kernel) time in seconds\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %U   user time in seconds\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %w   voluntary context switches\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %x   exit status of command\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %Tt  exit type (normal/signalled)\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %Tx  numeric exit code IF exited normally\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %Tn  numeric signal code IF signalled\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %Ts  signal name IF signalled\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"  %To  'ok' IF exited normally with code zero\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(
        b"\nDefault output format:\n\0".as_ptr() as *const ::core::ffi::c_char,
        stdout,
    );
    fputs_unlocked(default_format, stdout);
    fputc_unlocked('\n' as ::core::ffi::c_int, stdout);
    printf(USAGE_BUILTIN_WARNING.as_ptr(), PROGRAM_NAME.as_ptr());
    printf(
        b"\n%s website: <%s>\n\0".as_ptr() as *const ::core::ffi::c_char,
        PACKAGE_NAME.as_ptr(),
        PACKAGE_URL.as_ptr(),
    );
    printf(
        b"Full documentation at: <%smanual>\n\0".as_ptr() as *const ::core::ffi::c_char,
        PACKAGE_URL.as_ptr(),
    );
    printf(
        b"E-mail bug reports to: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        PACKAGE_BUGREPORT.as_ptr(),
    );
    exit(EXIT_SUCCESS);
}
unsafe extern "C" fn fprintargv(
    mut fp: *mut FILE,
    mut argv: *const *const ::core::ffi::c_char,
    mut filler: *const ::core::ffi::c_char,
) {
    let mut av: *const *const ::core::ffi::c_char =
        ::core::ptr::null::<*const ::core::ffi::c_char>();
    av = argv;
    fputs_unlocked(*av, fp);
    loop {
        av = av.offset(1);
        if (*av).is_null() {
            break;
        }
        fputs_unlocked(filler, fp);
        fputs_unlocked(*av, fp);
    }
    if ferror_unlocked(fp) != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"write error\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
unsafe extern "C" fn summarize(
    mut fp: *mut FILE,
    mut fmt: *const ::core::ffi::c_char,
    mut command: *mut *const ::core::ffi::c_char,
    mut resp: *mut RESUSE,
) {
    let mut r: ::core::ffi::c_ulong = 0;
    let mut v: ::core::ffi::c_ulong = 0;
    let mut us_r: ::core::ffi::c_ulong = 0;
    let mut us_v: ::core::ffi::c_ulong = 0;
    if !quiet && output_format != posix_format {
        if (*resp).waitstatus & 0xff as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int {
            fprintf(
                fp,
                b"Command stopped by signal %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                ((*resp).waitstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int,
            );
        } else if (((*resp).waitstatus & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
            as ::core::ffi::c_schar as ::core::ffi::c_int
            >> 1 as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
        {
            fprintf(
                fp,
                b"Command terminated by signal %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*resp).waitstatus & 0x7f as ::core::ffi::c_int,
            );
        } else if (*resp).waitstatus & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int
            && ((*resp).waitstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int != 0
        {
            fprintf(
                fp,
                b"Command exited with non-zero status %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ((*resp).waitstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int,
            );
        }
    }
    let mut elapsed_time: timespec = timespec_sub((*resp).end_time, (*resp).start_time);
    r = (elapsed_time.tv_sec * 1000 as __syscall_slong_t
        + elapsed_time.tv_nsec / 1000000 as __syscall_slong_t) as ::core::ffi::c_ulong;
    v = ((*resp).ru.ru_utime.tv_sec * 1000 as __suseconds_t
        + (*resp).ru.ru_utime.tv_usec / 1000 as __suseconds_t
        + (*resp).ru.ru_stime.tv_sec * 1000 as __suseconds_t
        + (*resp).ru.ru_stime.tv_usec / 1000 as __suseconds_t) as ::core::ffi::c_ulong;
    us_r = (elapsed_time.tv_nsec / 1000 as __syscall_slong_t) as ::core::ffi::c_ulong;
    us_v = ((*resp).ru.ru_utime.tv_usec + (*resp).ru.ru_stime.tv_usec) as ::core::ffi::c_ulong;
    while *fmt != 0 {
        match *fmt as ::core::ffi::c_int {
            37 => {
                fmt = fmt.offset(1);
                match *fmt as ::core::ffi::c_int {
                    37 => {
                        putc_unlocked('%' as ::core::ffi::c_int, fp);
                    }
                    67 => {
                        fprintargv(fp, command, b" \0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    68 => {
                        fprintf(
                            fp,
                            b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            if v.wrapping_div(MSEC_PER_TICK as ::core::ffi::c_ulong)
                                == 0 as ::core::ffi::c_ulong
                            {
                                0 as uintmax_t
                            } else {
                                get_rusage_idrss_kb(&raw mut (*resp).ru)
                                    .wrapping_div(
                                        (v as uintmax_t).wrapping_div(MSEC_PER_TICK as uintmax_t),
                                    )
                                    .wrapping_add(
                                        get_rusage_isrss_kb(&raw mut (*resp).ru).wrapping_div(
                                            (v as uintmax_t)
                                                .wrapping_div(MSEC_PER_TICK as uintmax_t),
                                        ),
                                    )
                            },
                        );
                    }
                    69 => {
                        if elapsed_time.tv_sec >= 3600 as __time_t {
                            fprintf(
                                fp,
                                b"%ld:%02ld:%02ld\0".as_ptr() as *const ::core::ffi::c_char,
                                elapsed_time.tv_sec / 3600 as __time_t,
                                elapsed_time.tv_sec % 3600 as __time_t / 60 as __time_t,
                                elapsed_time.tv_sec % 60 as __time_t,
                            );
                        } else {
                            fprintf(
                                fp,
                                b"%ld:%02ld.%02ld\0".as_ptr() as *const ::core::ffi::c_char,
                                elapsed_time.tv_sec / 60 as __time_t,
                                elapsed_time.tv_sec % 60 as __time_t,
                                elapsed_time.tv_nsec / 10000000 as __syscall_slong_t,
                            );
                        }
                    }
                    70 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_4.ru_majflt,
                        );
                    }
                    73 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_6.ru_inblock,
                        );
                    }
                    75 => {
                        fprintf(
                            fp,
                            b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            if v.wrapping_div(MSEC_PER_TICK as ::core::ffi::c_ulong)
                                == 0 as ::core::ffi::c_ulong
                            {
                                0 as ::core::ffi::c_ulong
                            } else {
                                get_rusage_idrss_kb(&raw mut (*resp).ru)
                                    .wrapping_div(
                                        (v as uintmax_t).wrapping_div(MSEC_PER_TICK as uintmax_t),
                                    )
                                    .wrapping_add(
                                        get_rusage_isrss_kb(&raw mut (*resp).ru).wrapping_div(
                                            (v as uintmax_t)
                                                .wrapping_div(MSEC_PER_TICK as uintmax_t),
                                        ),
                                    )
                                    .wrapping_add(
                                        get_rusage_ixrss_kb(&raw mut (*resp).ru).wrapping_div(
                                            (v as uintmax_t)
                                                .wrapping_div(MSEC_PER_TICK as uintmax_t),
                                        ),
                                    ) as ::core::ffi::c_ulong
                            },
                        );
                    }
                    77 => {
                        fprintf(
                            fp,
                            b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            get_rusage_maxrss_kb(&raw mut (*resp).ru),
                        );
                    }
                    79 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_7.ru_oublock,
                        );
                    }
                    80 => {
                        if r > 0 as ::core::ffi::c_ulong {
                            fprintf(
                                fp,
                                b"%lu%%\0".as_ptr() as *const ::core::ffi::c_char,
                                v.wrapping_mul(100 as ::core::ffi::c_ulong).wrapping_div(r),
                            );
                        } else if us_r > 0 as ::core::ffi::c_ulong {
                            fprintf(
                                fp,
                                b"%lu%%\0".as_ptr() as *const ::core::ffi::c_char,
                                us_v.wrapping_mul(100 as ::core::ffi::c_ulong)
                                    .wrapping_div(us_r),
                            );
                        } else {
                            fprintf(fp, b"?%%\0".as_ptr() as *const ::core::ffi::c_char);
                        }
                    }
                    82 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_3.ru_minflt,
                        );
                    }
                    83 => {
                        fprintf(
                            fp,
                            b"%ld.%02ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.ru_stime.tv_sec,
                            (*resp).ru.ru_stime.tv_usec
                                / 1000 as __suseconds_t
                                / 10 as __suseconds_t,
                        );
                    }
                    84 => {
                        fmt = fmt.offset(1);
                        match *fmt as ::core::ffi::c_int {
                            116 => {
                                fputs_unlocked(
                                    if (*resp).waitstatus & 0xff as ::core::ffi::c_int
                                        == 0x7f as ::core::ffi::c_int
                                    {
                                        b"stopped\0".as_ptr() as *const ::core::ffi::c_char
                                    } else if (((*resp).waitstatus & 0x7f as ::core::ffi::c_int)
                                        + 1 as ::core::ffi::c_int)
                                        as ::core::ffi::c_schar
                                        as ::core::ffi::c_int
                                        >> 1 as ::core::ffi::c_int
                                        > 0 as ::core::ffi::c_int
                                    {
                                        b"signalled\0".as_ptr() as *const ::core::ffi::c_char
                                    } else {
                                        b"normal\0".as_ptr() as *const ::core::ffi::c_char
                                    },
                                    fp,
                                );
                            }
                            110 => {
                                if (((*resp).waitstatus & 0x7f as ::core::ffi::c_int)
                                    + 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_schar
                                    as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int
                                    > 0 as ::core::ffi::c_int
                                {
                                    fprintf(
                                        fp,
                                        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                                        (*resp).waitstatus & 0x7f as ::core::ffi::c_int,
                                    );
                                }
                            }
                            115 => {
                                if (((*resp).waitstatus & 0x7f as ::core::ffi::c_int)
                                    + 1 as ::core::ffi::c_int)
                                    as ::core::ffi::c_schar
                                    as ::core::ffi::c_int
                                    >> 1 as ::core::ffi::c_int
                                    > 0 as ::core::ffi::c_int
                                {
                                    let mut buf: [::core::ffi::c_char; 20] = [0; 20];
                                    let mut i: ::core::ffi::c_int = sig2str(
                                        (*resp).waitstatus & 0x7f as ::core::ffi::c_int,
                                        &raw mut buf as *mut ::core::ffi::c_char,
                                    );
                                    if i == -1 as ::core::ffi::c_int {
                                        fprintf(
                                            fp,
                                            b"(%d)\0".as_ptr() as *const ::core::ffi::c_char,
                                            (*resp).waitstatus & 0x7f as ::core::ffi::c_int,
                                        );
                                    } else {
                                        fputs_unlocked(
                                            &raw mut buf as *mut ::core::ffi::c_char,
                                            fp,
                                        );
                                    }
                                }
                            }
                            120 => {
                                if (*resp).waitstatus & 0x7f as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    fprintf(
                                        fp,
                                        b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                                        ((*resp).waitstatus & 0xff00 as ::core::ffi::c_int)
                                            >> 8 as ::core::ffi::c_int,
                                    );
                                }
                            }
                            111 => {
                                if (*resp).waitstatus & 0x7f as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                    && ((*resp).waitstatus & 0xff00 as ::core::ffi::c_int)
                                        >> 8 as ::core::ffi::c_int
                                        == 0 as ::core::ffi::c_int
                                {
                                    fputs_unlocked(
                                        b"ok\0".as_ptr() as *const ::core::ffi::c_char,
                                        fp,
                                    );
                                }
                            }
                            0 => {
                                fputs_unlocked(
                                    b"T=missing letter\0".as_ptr() as *const ::core::ffi::c_char,
                                    fp,
                                );
                            }
                            _ => {
                                fputs_unlocked(
                                    b"T?=unknown\0".as_ptr() as *const ::core::ffi::c_char,
                                    fp,
                                );
                            }
                        }
                    }
                    85 => {
                        fprintf(
                            fp,
                            b"%ld.%02ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.ru_utime.tv_sec,
                            (*resp).ru.ru_utime.tv_usec
                                / 1000 as __suseconds_t
                                / 10 as __suseconds_t,
                        );
                    }
                    87 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_5.ru_nswap,
                        );
                    }
                    88 => {
                        fprintf(
                            fp,
                            b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            if v.wrapping_div(MSEC_PER_TICK as ::core::ffi::c_ulong)
                                == 0 as ::core::ffi::c_ulong
                            {
                                0 as uintmax_t
                            } else {
                                get_rusage_ixrss_kb(&raw mut (*resp).ru).wrapping_div(
                                    (v as uintmax_t).wrapping_div(MSEC_PER_TICK as uintmax_t),
                                )
                            },
                        );
                    }
                    90 => {
                        fprintf(
                            fp,
                            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                            getpagesize(),
                        );
                    }
                    99 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_12.ru_nivcsw,
                        );
                    }
                    101 => {
                        fprintf(
                            fp,
                            b"%ld.%02ld\0".as_ptr() as *const ::core::ffi::c_char,
                            elapsed_time.tv_sec,
                            elapsed_time.tv_nsec / 10000000 as __syscall_slong_t,
                        );
                    }
                    107 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_10.ru_nsignals,
                        );
                    }
                    112 => {
                        fprintf(
                            fp,
                            b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            if v.wrapping_div(MSEC_PER_TICK as ::core::ffi::c_ulong)
                                == 0 as ::core::ffi::c_ulong
                            {
                                0 as uintmax_t
                            } else {
                                get_rusage_isrss_kb(&raw mut (*resp).ru).wrapping_div(
                                    (v as uintmax_t).wrapping_div(MSEC_PER_TICK as uintmax_t),
                                )
                            },
                        );
                    }
                    114 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_9.ru_msgrcv,
                        );
                    }
                    115 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_8.ru_msgsnd,
                        );
                    }
                    116 => {
                        fprintf(
                            fp,
                            b"%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            if v.wrapping_div(MSEC_PER_TICK as ::core::ffi::c_ulong)
                                == 0 as ::core::ffi::c_ulong
                            {
                                0 as uintmax_t
                            } else {
                                get_rusage_idrss_kb(&raw mut (*resp).ru).wrapping_div(
                                    (v as uintmax_t).wrapping_div(MSEC_PER_TICK as uintmax_t),
                                )
                            },
                        );
                    }
                    119 => {
                        fprintf(
                            fp,
                            b"%ld\0".as_ptr() as *const ::core::ffi::c_char,
                            (*resp).ru.c2rust_unnamed_11.ru_nvcsw,
                        );
                    }
                    120 => {
                        fprintf(
                            fp,
                            b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                            ((*resp).waitstatus & 0xff00 as ::core::ffi::c_int)
                                >> 8 as ::core::ffi::c_int,
                        );
                    }
                    0 => {
                        putc_unlocked('?' as ::core::ffi::c_int, fp);
                    }
                    _ => {
                        putc_unlocked('?' as ::core::ffi::c_int, fp);
                        putc_unlocked(*fmt as ::core::ffi::c_int, fp);
                    }
                }
                if *fmt as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    fmt = fmt.offset(1);
                }
            }
            92 => {
                fmt = fmt.offset(1);
                match *fmt as ::core::ffi::c_int {
                    116 => {
                        putc_unlocked('\t' as ::core::ffi::c_int, fp);
                    }
                    110 => {
                        putc_unlocked('\n' as ::core::ffi::c_int, fp);
                    }
                    92 => {
                        putc_unlocked('\\' as ::core::ffi::c_int, fp);
                    }
                    0 => {
                        putc_unlocked('?' as ::core::ffi::c_int, fp);
                        putc_unlocked('\\' as ::core::ffi::c_int, fp);
                    }
                    _ => {
                        putc_unlocked('?' as ::core::ffi::c_int, fp);
                        putc_unlocked('\\' as ::core::ffi::c_int, fp);
                        putc_unlocked(*fmt as ::core::ffi::c_int, fp);
                    }
                }
                if *fmt as ::core::ffi::c_int != '\0' as ::core::ffi::c_int {
                    fmt = fmt.offset(1);
                }
            }
            _ => {
                let c2rust_fresh2 = fmt;
                fmt = fmt.offset(1);
                putc_unlocked(*c2rust_fresh2 as ::core::ffi::c_int, fp);
            }
        }
        if ferror_unlocked(fp) != 0 {
            if 0 != 0 {
                error(
                    1 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"write error\0".as_ptr() as *const ::core::ffi::c_char,
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
                        b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    putc_unlocked('\n' as ::core::ffi::c_int, fp);
    if ferror_unlocked(fp) != 0 {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"write error\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"write error\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
}
unsafe extern "C" fn getargs(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> *mut *const ::core::ffi::c_char {
    let mut optc: ::core::ffi::c_int = 0;
    loop {
        optc = getopt_long(
            argc,
            argv,
            b"+af:o:pqvV\0".as_ptr() as *const ::core::ffi::c_char,
            &raw const longopts as *const option,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
        );
        if optc == -1 as ::core::ffi::c_int {
            break;
        }
        match optc {
            97 => {
                append = r#true != 0;
            }
            102 => {
                output_format = optarg;
            }
            111 => {
                outfile = optarg;
            }
            112 => {
                output_format = posix_format;
            }
            113 => {
                quiet = r#true != 0;
            }
            118 => {
                verbose = r#true != 0;
            }
            86 => {
                version_etc(
                    stdout,
                    PROGRAM_NAME.as_ptr(),
                    PACKAGE_NAME.as_ptr(),
                    Version,
                    b"David Keppel\0".as_ptr() as *const ::core::ffi::c_char,
                    b"David MacKenzie\0".as_ptr() as *const ::core::ffi::c_char,
                    b"Assaf Gordon\0".as_ptr() as *const ::core::ffi::c_char,
                    NULL as *mut ::core::ffi::c_char,
                );
                exit(EXIT_SUCCESS);
            }
            -130 => {
                usage(EXIT_SUCCESS);
            }
            _ => {
                usage(C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int);
            }
        }
    }
    if optind == argc {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"missing program to run\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"missing program to run\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        usage(C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int);
    }
    if outfile.is_null() {
        outfp = stderr;
    } else {
        outfp = fopen(
            outfile,
            if append as ::core::ffi::c_int != 0 {
                b"ae\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"we\0".as_ptr() as *const ::core::ffi::c_char
            },
        ) as *mut FILE;
        if outfp.is_null() {
            if 0 != 0 {
                error(
                    C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int,
                    *__errno_location(),
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    outfile,
                );
                if C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    unreachable!();
                } else {
                };
            } else {
                ({
                    let __errstatus: ::core::ffi::c_int =
                        C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int;
                    error(
                        __errstatus,
                        *__errno_location(),
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        outfile,
                    );
                    if __errstatus != 0 as ::core::ffi::c_int {
                        unreachable!();
                    } else {
                    };
                });
            };
        }
    }
    if verbose {
        output_format = verbose_format;
    } else if output_format.is_null() {
        let mut env_format: *const ::core::ffi::c_char =
            getenv(b"TIME\0".as_ptr() as *const ::core::ffi::c_char);
        output_format = if !env_format.is_null() {
            env_format
        } else {
            default_format
        };
    }
    return argv.offset(optind as isize) as *mut *const ::core::ffi::c_char;
}
unsafe extern "C" fn run_command(mut cmd: *mut *const ::core::ffi::c_char, mut resp: *mut RESUSE) {
    let mut pid: pid_t = 0;
    let mut interrupt_signal: sighandler_t = None;
    let mut quit_signal: sighandler_t = None;
    let mut saved_errno: ::core::ffi::c_int = 0;
    (*resp).start_time = current_timespec();
    pid = fork() as pid_t;
    if pid < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot fork\0".as_ptr() as *const ::core::ffi::c_char,
            );
            if C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                unreachable!();
            } else {
            };
        } else {
            ({
                let __errstatus: ::core::ffi::c_int =
                    C2Rust_Unnamed_14::EXIT_CANCELED.0 as ::core::ffi::c_int;
                error(
                    __errstatus,
                    *__errno_location(),
                    b"cannot fork\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    } else if pid == 0 as ::core::ffi::c_int {
        execvp(*cmd.offset(0isize), cmd as *const *mut ::core::ffi::c_char);
        saved_errno = *__errno_location();
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                *__errno_location(),
                b"cannot run %s\0".as_ptr() as *const ::core::ffi::c_char,
                *cmd.offset(0isize),
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
                    b"cannot run %s\0".as_ptr() as *const ::core::ffi::c_char,
                    *cmd.offset(0isize),
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        rboxc_time_close_output();
        _exit(if saved_errno == ENOENT {
            C2Rust_Unnamed_14::EXIT_ENOENT.0 as ::core::ffi::c_int
        } else {
            C2Rust_Unnamed_14::EXIT_CANNOT_INVOKE.0 as ::core::ffi::c_int
        });
    }
    interrupt_signal = signal(
        SIGINT,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    ) as sighandler_t;
    quit_signal = signal(
        SIGQUIT,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    ) as sighandler_t;
    if waitpid(pid, &raw mut (*resp).waitstatus, 0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int
    {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"error waiting for child process\0".as_ptr() as *const ::core::ffi::c_char,
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
                    b"error waiting for child process\0".as_ptr() as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    (*resp).end_time = current_timespec();
    if getrusage(__rusage_who::RUSAGE_CHILDREN, &raw mut (*resp).ru) < 0 as ::core::ffi::c_int {
        if 0 != 0 {
            error(
                1 as ::core::ffi::c_int,
                *__errno_location(),
                b"error getting resource usage for child process\0".as_ptr()
                    as *const ::core::ffi::c_char,
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
                    b"error getting resource usage for child process\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
    }
    signal(SIGINT, interrupt_signal);
    signal(SIGQUIT, quit_signal);
}
extern "C" {
    static mut error_print_progname: Option<unsafe extern "C" fn()>;
    fn fclose(stream: *mut FILE) -> ::core::ffi::c_int;
    fn atexit(callback: unsafe extern "C" fn()) -> ::core::ffi::c_int;
}
unsafe extern "C" fn rboxc_time_error_prefix() {
    fprintf(stderr, b"%s: \0".as_ptr().cast(), program_name);
}
unsafe extern "C" fn rboxc_time_close_output() {
    if !outfile.is_null() && !outfp.is_null() {
        let saved_errno = *__errno_location();
        let owned = outfp;
        outfp = ::core::ptr::null_mut();
        fclose(owned);
        *__errno_location() = saved_errno;
    }
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_time(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut command_line: *mut *const ::core::ffi::c_char =
        ::core::ptr::null_mut::<*const ::core::ffi::c_char>();
    let mut res: RESUSE = RESUSE {
        waitstatus: 0,
        ru: rusage {
            ru_utime: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            ru_stime: timeval {
                tv_sec: 0,
                tv_usec: 0,
            },
            c2rust_unnamed: C2Rust_Unnamed_12 { ru_maxrss: 0 },
            c2rust_unnamed_0: C2Rust_Unnamed_11 { ru_ixrss: 0 },
            c2rust_unnamed_1: C2Rust_Unnamed_10 { ru_idrss: 0 },
            c2rust_unnamed_2: C2Rust_Unnamed_9 { ru_isrss: 0 },
            c2rust_unnamed_3: C2Rust_Unnamed_8 { ru_minflt: 0 },
            c2rust_unnamed_4: C2Rust_Unnamed_7 { ru_majflt: 0 },
            c2rust_unnamed_5: C2Rust_Unnamed_6 { ru_nswap: 0 },
            c2rust_unnamed_6: C2Rust_Unnamed_5 { ru_inblock: 0 },
            c2rust_unnamed_7: C2Rust_Unnamed_4 { ru_oublock: 0 },
            c2rust_unnamed_8: C2Rust_Unnamed_3 { ru_msgsnd: 0 },
            c2rust_unnamed_9: C2Rust_Unnamed_2 { ru_msgrcv: 0 },
            c2rust_unnamed_10: C2Rust_Unnamed_1 { ru_nsignals: 0 },
            c2rust_unnamed_11: C2Rust_Unnamed_0 { ru_nvcsw: 0 },
            c2rust_unnamed_12: C2Rust_Unnamed { ru_nivcsw: 0 },
        },
        start_time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        end_time: timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
    };
    let mut status: ::core::ffi::c_int = 0;
    set_program_name(*argv.offset(0isize));
    let prior_error_prefix = error_print_progname;
    if prior_error_prefix.is_none() {
        error_print_progname = Some(rboxc_time_error_prefix);
    }
    atexit(rboxc_time_close_output);
    command_line = getargs(argc, argv);
    run_command(command_line, &raw mut res);
    summarize(outfp, output_format, command_line, &raw mut res);
    fflush_unlocked(outfp);
    rboxc_time_close_output();
    if res.waitstatus & 0xff as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int {
        status = ((res.waitstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int)
            + SIGNALLED_OFFSET;
    } else if ((res.waitstatus & 0x7f as ::core::ffi::c_int) + 1 as ::core::ffi::c_int)
        as ::core::ffi::c_schar as ::core::ffi::c_int
        >> 1 as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        status = (res.waitstatus & 0x7f as ::core::ffi::c_int) + SIGNALLED_OFFSET;
    } else if res.waitstatus & 0x7f as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        status = (res.waitstatus & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int;
    } else {
        if 0 != 0 {
            error(
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                b"unknown status from command (%d)\0".as_ptr() as *const ::core::ffi::c_char,
                res.waitstatus,
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
                    b"unknown status from command (%d)\0".as_ptr() as *const ::core::ffi::c_char,
                    res.waitstatus,
                );
                if __errstatus != 0 as ::core::ffi::c_int {
                    unreachable!();
                } else {
                };
            });
        };
        status = EXIT_FAILURE;
    }
    return status;
}
pub const PACKAGE_BUGREPORT: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"bug-time@gnu.org\0")
};
pub const PACKAGE_NAME: [::core::ffi::c_char; 9] =
    unsafe { ::core::mem::transmute::<[u8; 9], [::core::ffi::c_char; 9]>(*b"GNU Time\0") };
pub const PACKAGE_URL: [::core::ffi::c_char; 35] = unsafe {
    ::core::mem::transmute::<[u8; 35], [::core::ffi::c_char; 35]>(
        *b"https://www.gnu.org/software/time/\0",
    )
};
