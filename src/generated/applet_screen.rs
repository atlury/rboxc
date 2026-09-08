// Generated from pinned GNU screen 5.0.2 by scripts/translate-entry-provider.py.
// Source SHA-256: 36598a0c3381b7fa1dc7f6b439f24ed690d8ca74463086d25ef027c7dd8e6989
/* Copyright (c) 2010
 *      Juergen Weigert (jnweiger@immd4.informatik.uni-erlangen.de)
 *      Sadrul Habib Chowdhury (sadrul@users.sourceforge.net)
 * Copyright (c) 2008, 2009
 *      Juergen Weigert (jnweiger@immd4.informatik.uni-erlangen.de)
 *      Michael Schroeder (mlschroe@immd4.informatik.uni-erlangen.de)
 *      Micah Cowan (micah@cowan.name)
 *      Sadrul Habib Chowdhury (sadrul@users.sourceforge.net)
 * Copyright (c) 1993-2002, 2003, 2005, 2006, 2007
 *      Juergen Weigert (jnweiger@immd4.informatik.uni-erlangen.de)
 *      Michael Schroeder (mlschroe@immd4.informatik.uni-erlangen.de)
 * Copyright (c) 1987 Oliver Laumann
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program (see the file COPYING); if not, see
 * https://www.gnu.org/licenses/, or contact Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02111-1301  USA
 *
 ****************************************************************
 */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct pwdata { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn freopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
        __stream: *mut FILE,
    ) -> *mut FILE;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn kill(__pid: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn killpg(__pgrp: __pid_t, __sig: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn access(__name: *const ::core::ffi::c_char, __type: ::core::ffi::c_int)
        -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    fn sleep(__seconds: ::core::ffi::c_uint) -> ::core::ffi::c_uint;
    fn dup(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn _exit(__status: ::core::ffi::c_int) -> !;
    fn getpid() -> __pid_t;
    fn getppid() -> __pid_t;
    fn getuid() -> __uid_t;
    fn geteuid() -> __uid_t;
    fn getgid() -> __gid_t;
    fn getegid() -> __gid_t;
    fn setuid(__uid: __uid_t) -> ::core::ffi::c_int;
    fn setgid(__gid: __gid_t) -> ::core::ffi::c_int;
    fn fork() -> __pid_t;
    fn ttyname(__fd: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn readlink(
        __path: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn getlogin() -> *mut ::core::ffi::c_char;
    fn gethostname(__name: *mut ::core::ffi::c_char, __len: size_t) -> ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn strtol(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
        __base: ::core::ffi::c_int,
    ) -> ::core::ffi::c_long;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn strncpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
        __n: size_t,
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    #[link_name = "rboxc_screen_visual_bell"]
    static mut visual_bell: bool;
    #[link_name = "rboxc_screen_LGotoPos"]
    fn LGotoPos(_: *mut Layer, _: ::core::ffi::c_int, _: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_LScrollH"]
    fn LScrollH(
        _: *mut Layer,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut mline,
    );
    #[link_name = "rboxc_screen_LScrollV"]
    fn LScrollV(
        _: *mut Layer,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    );
    #[link_name = "rboxc_screen_evenq"]
    fn evenq(_: *mut Event);
    #[link_name = "rboxc_screen_SetTimeout"]
    fn SetTimeout(_: *mut Event, _: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_sched"]
    fn sched() -> !;
    #[link_name = "rboxc_screen_islogfile"]
    fn islogfile(name: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_logfclose"]
    fn logfclose(_: *mut Log) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_logfwrite"]
    fn logfwrite(_: *mut Log, _: *mut ::core::ffi::c_char, _: size_t) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_logfflush"]
    fn logfflush(ifany: *mut Log) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_logreopen_register"]
    fn logreopen_register(
        r#fn: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut Log,
            ) -> ::core::ffi::c_int,
        >,
    );
    #[link_name = "rboxc_screen_lf_move_fd"]
    fn lf_move_fd(fd: ::core::ffi::c_int, wantfd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_SetCanvasWindow"]
    fn SetCanvasWindow(_: *mut Canvas, _: *mut Window);
    #[link_name = "rboxc_screen_MakeDefaultCanvas"]
    fn MakeDefaultCanvas() -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_RethinkViewportOffsets"]
    fn RethinkViewportOffsets(_: *mut Canvas);
    #[link_name = "rboxc_screen_AutosaveLayout"]
    fn AutosaveLayout(_: *mut Layout);
    #[link_name = "rboxc_screen_layout_last"]
    static mut layout_last: *mut Layout;
    #[link_name = "rboxc_screen_UserAdd"]
    fn UserAdd(_: *mut ::core::ffi::c_char, _: *mut *mut acluser) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_DefaultEsc"]
    static mut DefaultEsc: ::core::ffi::c_int;
    #[link_name = "rboxc_screen_DefaultMetaEsc"]
    static mut DefaultMetaEsc: ::core::ffi::c_int;
    #[link_name = "rboxc_screen_MakeDisplay"]
    fn MakeDisplay(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: pid_t,
        _: *mut mode,
    ) -> *mut Display;
    #[link_name = "rboxc_screen_FreeDisplay"]
    fn FreeDisplay();
    #[link_name = "rboxc_screen_InitTerm"]
    fn InitTerm(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_FinitTerm"]
    fn FinitTerm();
    #[link_name = "rboxc_screen_PUTCHARLP"]
    fn PUTCHARLP(_: uint32_t);
    #[link_name = "rboxc_screen_ClearAll"]
    fn ClearAll();
    #[link_name = "rboxc_screen_SetRendition"]
    fn SetRendition(_: *mut mchar);
    #[link_name = "rboxc_screen_MakeStatus"]
    fn MakeStatus(_: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_screen_RemoveStatus"]
    fn RemoveStatus();
    #[link_name = "rboxc_screen_AddStr"]
    fn AddStr(_: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_screen_Flush"]
    fn Flush(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_freetty"]
    fn freetty();
    #[link_name = "rboxc_screen_ClearScrollbackBuffer"]
    fn ClearScrollbackBuffer();
    #[link_name = "rboxc_screen_displays"]
    static mut displays: *mut Display;
    #[link_name = "rboxc_screen_display"]
    static mut display: *mut Display;
    #[link_name = "rboxc_screen_MakeWindow"]
    fn MakeWindow(_: *mut NewWindow) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_FreeWindow"]
    fn FreeWindow(_: *mut Window);
    #[link_name = "rboxc_screen_FreePseudowin"]
    fn FreePseudowin(_: *mut Window);
    #[link_name = "rboxc_screen_nwin_compose"]
    fn nwin_compose(_: *mut NewWindow, _: *mut NewWindow, _: *mut NewWindow);
    #[link_name = "rboxc_screen_ReleaseAutoWritelock"]
    fn ReleaseAutoWritelock(_: *mut Display, _: *mut Window) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_DefaultShell"]
    static mut DefaultShell: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_screen_nwin_options"]
    static mut nwin_options: NewWindow;
    #[link_name = "rboxc_screen_nwin_undef"]
    static mut nwin_undef: NewWindow;
    #[link_name = "rboxc_screen_nwin_default"]
    static mut nwin_default: NewWindow;
    #[link_name = "rboxc_screen_Kill"]
    fn Kill(_: pid_t, _: ::core::ffi::c_int);
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn getpwuid(__uid: __uid_t) -> *mut passwd;
    fn getpwnam(__name: *const ::core::ffi::c_char) -> *mut passwd;
    fn stat(__file: *const ::core::ffi::c_char, __buf: *mut stat) -> ::core::ffi::c_int;
    fn umask(__mask: __mode_t) -> __mode_t;
    fn mkdir(__path: *const ::core::ffi::c_char, __mode: __mode_t) -> ::core::ffi::c_int;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut ::core::ffi::c_int,
        __options: ::core::ffi::c_int,
    ) -> __pid_t;
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn nl_langinfo(__item: nl_item) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_StartRc"]
    fn StartRc(_: *mut ::core::ffi::c_char, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_FinishRc"]
    fn FinishRc(_: *mut ::core::ffi::c_char);
    #[link_name = "rboxc_screen_secopen"]
    fn secopen(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_display_license"]
    fn display_license();
    #[link_name = "rboxc_screen_mark_key_tab"]
    static mut mark_key_tab: [::core::ffi::c_uchar; 0];
    #[link_name = "rboxc_screen_MakeWinMsg"]
    fn MakeWinMsg(
        _: *mut ::core::ffi::c_char,
        _: *mut Window,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_WindowChanged"]
    fn WindowChanged(_: *mut Window, _: WinMsgEscapeChar);
    #[link_name = "rboxc_screen_g_winmsg"]
    static mut g_winmsg: *mut WinMsgBuf;
    #[link_name = "rboxc_screen_Attach"]
    fn Attach(_: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_Attacher"]
    fn Attacher() -> !;
    #[link_name = "rboxc_screen_AttacherFinit"]
    fn AttacherFinit(_: ::core::ffi::c_int) -> !;
    #[link_name = "rboxc_screen_SendCmdMessage"]
    fn SendCmdMessage(
        _: *mut ::core::ffi::c_char,
        _: *mut ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
    );
    #[link_name = "rboxc_screen_InitBuiltinTabs"]
    fn InitBuiltinTabs();
    #[link_name = "rboxc_screen_FindEncoding"]
    fn FindEncoding(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_RecodeBuf"]
    fn RecodeBuf(
        _: *mut ::core::ffi::c_uchar,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_version"]
    static mut version: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_screen_SaveStr"]
    fn SaveStr(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strerror(_: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_stripdev"]
    fn stripdev(_: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_closeallfiles"]
    fn closeallfiles(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_UserContext"]
    fn UserContext() -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_UserReturn"]
    fn UserReturn(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_UserStatus"]
    fn UserStatus() -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_xsignal"]
    fn xsignal(
        _: ::core::ffi::c_int,
        _: Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
    ) -> Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
    #[link_name = "rboxc_screen_xseteuid"]
    fn xseteuid(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_xsetegid"]
    fn xsetegid(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_InitKeytab"]
    fn InitKeytab();
    #[link_name = "rboxc_screen_KillWindow"]
    fn KillWindow(_: *mut Window);
    #[link_name = "rboxc_screen_SetEscape"]
    fn SetEscape(_: *mut acluser, _: ::core::ffi::c_int, _: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_CompileKeys"]
    fn CompileKeys(
        _: *mut ::core::ffi::c_char,
        _: ::core::ffi::c_int,
        _: *mut ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_ApplyAttrColor"]
    fn ApplyAttrColor(_: uint64_t, _: *mut mchar);
    #[link_name = "rboxc_screen_zmodem_recvcmd"]
    static mut zmodem_recvcmd: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_zmodem_sendcmd"]
    static mut zmodem_sendcmd: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_FindSocket"]
    fn FindSocket(
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_int,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_MakeServerSocket"]
    fn MakeServerSocket() -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_RecoverSocket"]
    fn RecoverSocket() -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_chsock"]
    fn chsock() -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_ReceiveMsg"]
    fn ReceiveMsg();
    #[link_name = "rboxc_screen_SendCreateMsg"]
    fn SendCreateMsg(_: *mut ::core::ffi::c_char, _: *mut NewWindow);
    #[link_name = "rboxc_screen_SendErrorMsg"]
    fn SendErrorMsg(_: *mut ::core::ffi::c_char, _: *mut ::core::ffi::c_char)
        -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_InitTermcap"]
    fn InitTermcap(_: ::core::ffi::c_int, _: ::core::ffi::c_int) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_MakeTermcap"]
    fn MakeTermcap(_: bool) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_screen_screenterm"]
    static mut screenterm: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_screen_Term"]
    static mut Term: [::core::ffi::c_char; 0];
    #[link_name = "rboxc_screen_GetTTY"]
    fn GetTTY(_: ::core::ffi::c_int, _: *mut mode);
    #[link_name = "rboxc_screen_SetTTY"]
    fn SetTTY(_: ::core::ffi::c_int, _: *mut mode);
    #[link_name = "rboxc_screen_SetMode"]
    fn SetMode(_: *mut mode, _: *mut mode, _: ::core::ffi::c_int, _: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_TtyGrabConsole"]
    fn TtyGrabConsole(
        _: ::core::ffi::c_int,
        _: bool,
        _: *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_screen_brktty"]
    fn brktty(_: ::core::ffi::c_int);
    #[link_name = "rboxc_screen_CheckTtyname"]
    fn CheckTtyname(_: *mut ::core::ffi::c_char) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __gnuc_va_list = __builtin_va_list;
pub type __uint64_t = u64;
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
pub type __suseconds_t = ::core::ffi::c_long;
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
pub type va_list = __gnuc_va_list;
pub type ssize_t = isize;
pub type gid_t = __gid_t;
pub type mode_t = __mode_t;
pub type uid_t = __uid_t;
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
pub type cc_t = ::core::ffi::c_uchar;
pub type speed_t = ::core::ffi::c_uint;
pub type tcflag_t = ::core::ffi::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c2rust_unnamed: C2Rust_Unnamed_0,
    pub c2rust_unnamed_0: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __ospeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub __ispeed: speed_t,
    pub c_ispeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode {
    pub tio: termios,
}
pub type uint16_t = u16;
pub type uint32_t = u32;
pub type uint64_t = u64;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct state_t(pub ::core::ffi::c_uint);
impl state_t {
    pub const LIT: Self = Self(0);
    pub const ESC: Self = Self(1);
    pub const ASTR: Self = Self(2);
    pub const STRESC: Self = Self(3);
    pub const CSI: Self = Self(4);
    pub const PRIN: Self = Self(5);
    pub const PRINESC: Self = Self(6);
    pub const PRINCSI: Self = Self(7);
    pub const PRIN4: Self = Self(8);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct string_t(pub ::core::ffi::c_uint);
impl string_t {
    pub const NONE: Self = Self(0);
    pub const DCS: Self = Self(1);
    pub const OSC: Self = Self(2);
    pub const APC: Self = Self(3);
    pub const PM: Self = Self(4);
    pub const AKA: Self = Self(5);
    pub const GM: Self = Self(6);
    pub const STATUS: Self = Self(7);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Window {
    pub w_prev: *mut Window,
    pub w_next: *mut Window,
    pub w_prev_mru: *mut Window,
    pub w_type: ::core::ffi::c_int,
    pub w_list_order: bool,
    pub w_list_nested: bool,
    pub w_layer: Layer,
    pub w_savelayer: *mut Layer,
    pub w_blocked: ::core::ffi::c_int,
    pub w_pwin: *mut pseudowin,
    pub w_pdisplay: *mut Display,
    pub w_lastdisp: *mut Display,
    pub w_number: uint16_t,
    pub w_readev: Event,
    pub w_writeev: Event,
    pub w_silenceev: Event,
    pub w_zombieev: Event,
    pub w_poll_zombie_timeout: ::core::ffi::c_int,
    pub w_ptyfd: ::core::ffi::c_int,
    pub w_inbuf: [::core::ffi::c_char; 4096],
    pub w_inlen: ::core::ffi::c_int,
    pub w_outbuf: [::core::ffi::c_char; 4096],
    pub w_outlen: ::core::ffi::c_int,
    pub w_aflag: bool,
    pub w_dynamicaka: bool,
    pub w_title: *mut ::core::ffi::c_char,
    pub w_akachange: *mut ::core::ffi::c_char,
    pub w_akabuf: [::core::ffi::c_char; 768],
    pub w_autoaka: ::core::ffi::c_int,
    pub w_group: *mut Window,
    pub w_intermediate: ::core::ffi::c_int,
    pub w_args: [::core::ffi::c_int; 64],
    pub w_NumArgs: ::core::ffi::c_int,
    pub w_wlock: ::core::ffi::c_int,
    pub w_wlockuser: *mut acluser,
    pub w_userbits: [AclBits; 3],
    pub w_lio_notify: AclBits,
    pub w_mon_notify: AclBits,
    pub w_state: state_t,
    pub w_StringType: string_t,
    pub w_mlines: *mut mline,
    pub w_rend: mchar,
    pub w_FontL: ::core::ffi::c_char,
    pub w_FontR: ::core::ffi::c_char,
    pub w_FontE: ::core::ffi::c_char,
    pub w_Charset: ::core::ffi::c_int,
    pub w_CharsetR: ::core::ffi::c_int,
    pub w_charsets: [::core::ffi::c_int; 4],
    pub w_ss: ::core::ffi::c_int,
    pub w_saved: cursor,
    pub w_top: ::core::ffi::c_int,
    pub w_bot: ::core::ffi::c_int,
    pub w_wrap: bool,
    pub w_origin: ::core::ffi::c_int,
    pub w_insert: bool,
    pub w_keypad: ::core::ffi::c_int,
    pub w_cursorkeys: ::core::ffi::c_int,
    pub w_revvid: bool,
    pub w_curinv: ::core::ffi::c_int,
    pub w_curvvis: ::core::ffi::c_int,
    pub w_autolf: ::core::ffi::c_int,
    pub w_hstatus: *mut ::core::ffi::c_char,
    pub w_gr: ::core::ffi::c_int,
    pub w_c1: bool,
    pub w_decodestate: ::core::ffi::c_int,
    pub w_mbcs: ::core::ffi::c_int,
    pub w_string: [::core::ffi::c_char; 768],
    pub w_stringp: *mut ::core::ffi::c_char,
    pub w_tabs: *mut ::core::ffi::c_char,
    pub w_bell: ::core::ffi::c_int,
    pub w_flow: ::core::ffi::c_int,
    pub w_log: *mut Log,
    pub w_logsilence: ::core::ffi::c_int,
    pub w_monitor: ::core::ffi::c_int,
    pub w_silencewait: ::core::ffi::c_int,
    pub w_silence: ::core::ffi::c_int,
    pub w_norefresh: ::core::ffi::c_char,
    pub w_xtermosc: [[::core::ffi::c_char; 2560]; 5],
    pub w_mouse: ::core::ffi::c_int,
    pub w_extmouse: ::core::ffi::c_int,
    pub w_bracketed: bool,
    pub w_cursorstyle: ::core::ffi::c_int,
    pub w_slowpaste: ::core::ffi::c_int,
    pub w_histheight: ::core::ffi::c_int,
    pub w_histidx: ::core::ffi::c_int,
    pub w_scrollback_height: ::core::ffi::c_int,
    pub w_hlines: *mut mline,
    pub w_paster: paster,
    pub w_pid: pid_t,
    pub w_deadpid: pid_t,
    pub w_cmdargs: [*mut ::core::ffi::c_char; 64],
    pub w_dir: *mut ::core::ffi::c_char,
    pub w_term: *mut ::core::ffi::c_char,
    pub w_tty: [::core::ffi::c_char; 768],
    pub w_zauto: ::core::ffi::c_int,
    pub w_zdisplay: *mut Display,
    pub w_alt: C2Rust_Unnamed_1,
    pub w_destroyev: Event,
    pub w_exitstatus: ::core::ffi::c_int,
    pub w_miflag: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Event {
    pub next: *mut Event,
    pub handler: Option<unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ()>,
    pub data: *mut ::core::ffi::c_void,
    pub fd: ::core::ffi::c_int,
    pub r#type: EventType,
    pub priority: ::core::ffi::c_int,
    pub timeout: ::core::ffi::c_int,
    pub queued: bool,
    pub condpos: *mut ::core::ffi::c_int,
    pub condneg: *mut ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct EventType(pub ::core::ffi::c_uint);
impl EventType {
    pub const EV_TIMEOUT: Self = Self(0);
    pub const EV_READ: Self = Self(1);
    pub const EV_WRITE: Self = Self(2);
    pub const EV_ALWAYS: Self = Self(3);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub on: ::core::ffi::c_int,
    pub mlines: *mut mline,
    pub width: ::core::ffi::c_int,
    pub height: ::core::ffi::c_int,
    pub histheight: ::core::ffi::c_int,
    pub hlines: *mut mline,
    pub histidx: ::core::ffi::c_int,
    pub cursor: cursor,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cursor {
    pub on: ::core::ffi::c_int,
    pub x: ::core::ffi::c_int,
    pub y: ::core::ffi::c_int,
    pub Rend: mchar,
    pub Charset: ::core::ffi::c_int,
    pub CharsetR: ::core::ffi::c_int,
    pub Charsets: [::core::ffi::c_int; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mchar {
    pub image: uint32_t,
    pub attr: uint32_t,
    pub font: uint32_t,
    pub colorbg: uint32_t,
    pub colorfg: uint32_t,
    pub mbcs: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mline {
    pub image: *mut uint32_t,
    pub attr: *mut uint32_t,
    pub font: *mut uint32_t,
    pub colorbg: *mut uint32_t,
    pub colorfg: *mut uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Display {
    pub d_next: *mut Display,
    pub d_user: *mut acluser,
    pub d_canvas: Canvas,
    pub d_cvlist: *mut Canvas,
    pub d_forecv: *mut Canvas,
    pub d_layout: *mut Layout,
    pub d_processinput: Option<unsafe extern "C" fn(*mut ::core::ffi::c_char, size_t) -> ()>,
    pub d_processinputdata: *mut pwdata,
    pub d_vpxmin: ::core::ffi::c_int,
    pub d_vpxmax: ::core::ffi::c_int,
    pub d_fore: *mut Window,
    pub d_other: *mut Window,
    pub d_nonblock: ::core::ffi::c_int,
    pub d_termname: [::core::ffi::c_char; 33],
    pub d_tentry: *mut ::core::ffi::c_char,
    pub d_tcinited: ::core::ffi::c_char,
    pub d_width: ::core::ffi::c_int,
    pub d_height: ::core::ffi::c_int,
    pub d_defwidth: ::core::ffi::c_int,
    pub d_defheight: ::core::ffi::c_int,
    pub d_top: ::core::ffi::c_int,
    pub d_bot: ::core::ffi::c_int,
    pub d_x: ::core::ffi::c_int,
    pub d_y: ::core::ffi::c_int,
    pub d_rend: mchar,
    pub d_atyp: ::core::ffi::c_char,
    pub d_mbcs: ::core::ffi::c_int,
    pub d_encoding: ::core::ffi::c_int,
    pub d_decodestate: ::core::ffi::c_int,
    pub d_realfont: ::core::ffi::c_int,
    pub d_insert: bool,
    pub d_keypad: ::core::ffi::c_int,
    pub d_cursorkeys: ::core::ffi::c_int,
    pub d_revvid: bool,
    pub d_curvis: ::core::ffi::c_int,
    pub d_has_hstatus: HardStatus,
    pub d_hstatus: bool,
    pub d_lp_missing: ::core::ffi::c_int,
    pub d_mouse: ::core::ffi::c_int,
    pub d_extmouse: ::core::ffi::c_int,
    pub d_mouse_parse: mouse_parse,
    pub d_mousetrack: ::core::ffi::c_int,
    pub d_bracketed: ::core::ffi::c_int,
    pub d_cursorstyle: ::core::ffi::c_int,
    pub d_xtermosc: [::core::ffi::c_int; 5],
    pub d_lpchar: mchar,
    pub d_status_time: timeval,
    pub d_status: DisplayStatus,
    pub d_status_bell: ::core::ffi::c_char,
    pub d_status_len: ::core::ffi::c_int,
    pub d_status_lastmsg: *mut ::core::ffi::c_char,
    pub d_status_buflen: ::core::ffi::c_int,
    pub d_status_lastx: ::core::ffi::c_int,
    pub d_status_lasty: ::core::ffi::c_int,
    pub d_status_obuflen: ::core::ffi::c_int,
    pub d_status_obuffree: ::core::ffi::c_int,
    pub d_status_obufpos: ::core::ffi::c_int,
    pub d_statusev: Event,
    pub d_hstatusev: Event,
    pub d_kaablamm: ::core::ffi::c_int,
    pub d_ESCseen: *mut action,
    pub d_userpid: pid_t,
    pub d_usertty: [::core::ffi::c_char; 4096],
    pub d_userfd: ::core::ffi::c_int,
    pub d_readev: Event,
    pub d_writeev: Event,
    pub d_blockedev: Event,
    pub d_OldMode: mode,
    pub d_NewMode: mode,
    pub d_flow: ::core::ffi::c_int,
    pub d_intrc: ::core::ffi::c_int,
    pub d_obuf: *mut ::core::ffi::c_char,
    pub d_obuflen: ::core::ffi::c_int,
    pub d_obufmax: ::core::ffi::c_int,
    pub d_obuflenmax: ::core::ffi::c_int,
    pub d_obufp: *mut ::core::ffi::c_char,
    pub d_obuffree: ::core::ffi::c_int,
    pub d_auto_nuke: bool,
    pub d_nseqs: ::core::ffi::c_int,
    pub d_aseqs: ::core::ffi::c_int,
    pub d_kmaps: *mut ::core::ffi::c_uchar,
    pub d_seqp: *mut ::core::ffi::c_uchar,
    pub d_seql: ::core::ffi::c_int,
    pub d_seqh: *mut ::core::ffi::c_uchar,
    pub d_mapev: Event,
    pub d_dontmap: ::core::ffi::c_int,
    pub d_mapdefault: ::core::ffi::c_int,
    pub d_tcs: [tcu; 204],
    pub d_attrtab: [*mut ::core::ffi::c_char; 7],
    pub d_attrtyp: [::core::ffi::c_char; 7],
    pub d_hascolor: ::core::ffi::c_int,
    pub d_c0_tab: [::core::ffi::c_char; 256],
    pub d_xtable: *mut *mut *mut ::core::ffi::c_char,
    pub d_UPcost: ::core::ffi::c_int,
    pub d_DOcost: ::core::ffi::c_int,
    pub d_LEcost: ::core::ffi::c_int,
    pub d_NDcost: ::core::ffi::c_int,
    pub d_CRcost: ::core::ffi::c_int,
    pub d_IMcost: ::core::ffi::c_int,
    pub d_EIcost: ::core::ffi::c_int,
    pub d_NLcost: ::core::ffi::c_int,
    pub d_printfd: ::core::ffi::c_int,
    pub d_blocked: ::core::ffi::c_int,
    pub d_blocked_fuzz: ::core::ffi::c_int,
    pub d_idleev: Event,
    pub d_blankerpid: pid_t,
    pub d_blankerev: Event,
    pub d_mousetimeoutev: Event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union tcu {
    pub flg: ::core::ffi::c_int,
    pub num: ::core::ffi::c_int,
    pub str: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct action {
    pub nr: ::core::ffi::c_int,
    pub args: *mut *mut ::core::ffi::c_char,
    pub argl: *mut ::core::ffi::c_int,
    pub quiet: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct DisplayStatus(pub ::core::ffi::c_uint);
impl DisplayStatus {
    pub const STATUS_OFF: Self = Self(0);
    pub const STATUS_ON_WIN: Self = Self(1);
    pub const STATUS_ON_HS: Self = Self(2);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mouse_parse {
    pub sgrmode: ::core::ffi::c_char,
    pub state: ::core::ffi::c_int,
    pub params: [::core::ffi::c_int; 3],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct HardStatus(pub ::core::ffi::c_uint);
impl HardStatus {
    pub const HSTATUS_IGNORE: Self = Self(0);
    pub const HSTATUS_LASTLINE: Self = Self(1);
    pub const HSTATUS_MESSAGE: Self = Self(2);
    pub const HSTATUS_HS: Self = Self(3);
    pub const HSTATUS_FIRSTLINE: Self = Self(4);
    pub const HSTATUS_ALWAYS: Self = Self(8);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Layout {
    pub lay_next: *mut Layout,
    pub lay_title: *mut ::core::ffi::c_char,
    pub lay_number: ::core::ffi::c_int,
    pub lay_canvas: Canvas,
    pub lay_forecv: *mut Canvas,
    pub lay_cvlist: *mut Canvas,
    pub lay_autosave: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Canvas {
    pub c_next: *mut Canvas,
    pub c_display: *mut Display,
    pub c_slnext: *mut Canvas,
    pub c_slprev: *mut Canvas,
    pub c_slperp: *mut Canvas,
    pub c_slback: *mut Canvas,
    pub c_slorient: ::core::ffi::c_int,
    pub c_slweight: ::core::ffi::c_int,
    pub c_vplist: *mut Viewport,
    pub c_layer: *mut Layer,
    pub c_lnext: *mut Canvas,
    pub c_blank: Layer,
    pub c_xoff: ::core::ffi::c_int,
    pub c_yoff: ::core::ffi::c_int,
    pub c_xs: ::core::ffi::c_int,
    pub c_xe: ::core::ffi::c_int,
    pub c_ys: ::core::ffi::c_int,
    pub c_ye: ::core::ffi::c_int,
    pub c_captev: Event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Layer {
    pub l_cvlist: *mut Canvas,
    pub l_width: ::core::ffi::c_int,
    pub l_height: ::core::ffi::c_int,
    pub l_x: ::core::ffi::c_int,
    pub l_y: ::core::ffi::c_int,
    pub l_encoding: ::core::ffi::c_int,
    pub l_layfn: *const LayFuncs,
    pub l_data: *mut ::core::ffi::c_void,
    pub l_next: *mut Layer,
    pub l_bottom: *mut Layer,
    pub l_blocking: ::core::ffi::c_int,
    pub l_mode: ::core::ffi::c_int,
    pub l_mouseevent: C2Rust_Unnamed_3,
    pub l_pause: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub d: bool,
    pub left: *mut ::core::ffi::c_int,
    pub right: *mut ::core::ffi::c_int,
    pub top: ::core::ffi::c_int,
    pub bottom: ::core::ffi::c_int,
    pub lines: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub buffer: [::core::ffi::c_uchar; 3],
    pub len: size_t,
    pub start: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayFuncs {
    pub lf_LayProcess:
        Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_char, *mut size_t) -> ()>,
    pub lf_LayAbort: Option<unsafe extern "C" fn() -> ()>,
    pub lf_LayRedisplayLine: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub lf_LayClearLine: Option<
        unsafe extern "C" fn(
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
        ) -> (),
    >,
    pub lf_LayResize:
        Option<unsafe extern "C" fn(::core::ffi::c_int, ::core::ffi::c_int) -> ::core::ffi::c_int>,
    pub lf_LayRestore: Option<unsafe extern "C" fn() -> ()>,
    pub lf_LayFree: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Viewport {
    pub v_next: *mut Viewport,
    pub v_canvas: *mut Canvas,
    pub v_xoff: ::core::ffi::c_int,
    pub v_yoff: ::core::ffi::c_int,
    pub v_xs: ::core::ffi::c_int,
    pub v_xe: ::core::ffi::c_int,
    pub v_ys: ::core::ffi::c_int,
    pub v_ye: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct acluser {
    pub u_next: *mut acluser,
    pub u_name: [::core::ffi::c_char; 257],
    pub u_detachwin: ::core::ffi::c_int,
    pub u_detachotherwin: ::core::ffi::c_int,
    pub u_Esc: ::core::ffi::c_int,
    pub u_MetaEsc: ::core::ffi::c_int,
    pub u_plop: plop,
    pub u_id: ::core::ffi::c_int,
    pub u_umask_w_bits: [AclBits; 3],
    pub u_group: *mut aclusergroup,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aclusergroup {
    pub u: *mut acluser,
    pub next: *mut aclusergroup,
}
pub type AclBits = *mut ::core::ffi::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct plop {
    pub buf: *mut ::core::ffi::c_char,
    pub len: size_t,
    pub enc: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paster {
    pub pa_pastebuf: *mut ::core::ffi::c_char,
    pub pa_pasteptr: *mut ::core::ffi::c_char,
    pub pa_pastelen: size_t,
    pub pa_pastelayer: *mut Layer,
    pub pa_slowev: Event,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Log {
    pub next: *mut Log,
    pub fp: *mut FILE,
    pub name: *mut ::core::ffi::c_char,
    pub opencount: ::core::ffi::c_int,
    pub writecount: ::core::ffi::c_int,
    pub flushcount: ::core::ffi::c_int,
    pub st: *mut stat,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pseudowin {
    pub p_fdpat: ::core::ffi::c_int,
    pub p_pid: pid_t,
    pub p_ptyfd: ::core::ffi::c_int,
    pub p_readev: Event,
    pub p_writeev: Event,
    pub p_cmd: [::core::ffi::c_char; 768],
    pub p_tty: [::core::ffi::c_char; 768],
    pub p_inbuf: [::core::ffi::c_char; 4096],
    pub p_inlen: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct NewWindow {
    pub StartAt: ::core::ffi::c_int,
    pub aka: *mut ::core::ffi::c_char,
    pub args: *mut *mut ::core::ffi::c_char,
    pub dir: *mut ::core::ffi::c_char,
    pub term: *mut ::core::ffi::c_char,
    pub aflag: bool,
    pub dynamicaka: bool,
    pub flowflag: ::core::ffi::c_int,
    pub list_order: bool,
    pub list_nested: bool,
    pub lflag: ::core::ffi::c_int,
    pub histheight: ::core::ffi::c_int,
    pub monitor: ::core::ffi::c_int,
    pub wlock: ::core::ffi::c_int,
    pub silence: ::core::ffi::c_int,
    pub wrap: bool,
    pub Lflag: bool,
    pub slow: ::core::ffi::c_int,
    pub gr: ::core::ffi::c_int,
    pub c1: bool,
    pub bce: ::core::ffi::c_int,
    pub encoding: ::core::ffi::c_int,
    pub hstatus: *mut ::core::ffi::c_char,
    pub charset: *mut ::core::ffi::c_char,
    pub poll_zombie_timeout: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct WinMsgEscapeChar(pub ::core::ffi::c_uint);
impl WinMsgEscapeChar {
    pub const WINESC_HOUR: Self = Self(65);
    pub const WINESC_hour: Self = Self(97);
    pub const WINESC_TIME: Self = Self(67);
    pub const WINESC_time: Self = Self(99);
    pub const WINESC_DAY: Self = Self(68);
    pub const WINESC_day: Self = Self(100);
    pub const WINESC_ESC_SEEN: Self = Self(69);
    pub const WINESC_FOCUS: Self = Self(70);
    pub const WINESC_WFLAGS: Self = Self(102);
    pub const WINESC_WIN_GROUP: Self = Self(103);
    pub const WINESC_HOST: Self = Self(72);
    pub const WINESC_HSTATUS: Self = Self(104);
    pub const WINESC_MONTH: Self = Self(77);
    pub const WINESC_month: Self = Self(109);
    pub const WINESC_WIN_LOGNAME: Self = Self(78);
    pub const WINESC_WIN_NUM: Self = Self(110);
    pub const WINESC_WIN_COUNT: Self = Self(79);
    pub const WINESC_PID: Self = Self(112);
    pub const WINESC_COPY_MODE: Self = Self(80);
    pub const WINESC_SESS_NAME: Self = Self(83);
    pub const WINESC_WIN_SIZE: Self = Self(115);
    pub const WINESC_WIN_TTY: Self = Self(84);
    pub const WINESC_WIN_TITLE: Self = Self(116);
    pub const WINESC_WIN_NAMES_NOCUR: Self = Self(87);
    pub const WINESC_WIN_NAMES: Self = Self(119);
    pub const WINESC_CMD: Self = Self(88);
    pub const WINESC_CMD_ARGS: Self = Self(120);
    pub const WINESC_YEAR: Self = Self(89);
    pub const WINESC_year: Self = Self(121);
    pub const WINESC_REND_START: Self = Self(123);
    pub const WINESC_REND_END: Self = Self(125);
    pub const WINESC_REND_POP: Self = Self(45);
    pub const WINESC_COND: Self = Self(63);
    pub const WINESC_COND_ELSE: Self = Self(58);
    pub const WINESC_BACKTICK: Self = Self(96);
    pub const WINESC_PAD: Self = Self(61);
    pub const WINESC_TRUNC: Self = Self(60);
    pub const WINESC_TRUNC_POS: Self = Self(62);
    pub const WINESC_WIN_CARET: Self = Self(42);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct WinMsgBuf {
    pub buf: *mut ::core::ffi::c_char,
    pub size: size_t,
    pub rend: [uint64_t; 256],
    pub rendpos: [::core::ffi::c_int; 256],
    pub numrend: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::core::ffi::c_char,
    pub pw_passwd: *mut ::core::ffi::c_char,
    pub pw_uid: __uid_t,
    pub pw_gid: __gid_t,
    pub pw_gecos: *mut ::core::ffi::c_char,
    pub pw_dir: *mut ::core::ffi::c_char,
    pub pw_shell: *mut ::core::ffi::c_char,
}
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type nl_item = ::core::ffi::c_int;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const ABDAY_1: Self = Self(131072);
    pub const ABDAY_2: Self = Self(131073);
    pub const ABDAY_3: Self = Self(131074);
    pub const ABDAY_4: Self = Self(131075);
    pub const ABDAY_5: Self = Self(131076);
    pub const ABDAY_6: Self = Self(131077);
    pub const ABDAY_7: Self = Self(131078);
    pub const DAY_1: Self = Self(131079);
    pub const DAY_2: Self = Self(131080);
    pub const DAY_3: Self = Self(131081);
    pub const DAY_4: Self = Self(131082);
    pub const DAY_5: Self = Self(131083);
    pub const DAY_6: Self = Self(131084);
    pub const DAY_7: Self = Self(131085);
    pub const ABMON_1: Self = Self(131086);
    pub const ABMON_2: Self = Self(131087);
    pub const ABMON_3: Self = Self(131088);
    pub const ABMON_4: Self = Self(131089);
    pub const ABMON_5: Self = Self(131090);
    pub const ABMON_6: Self = Self(131091);
    pub const ABMON_7: Self = Self(131092);
    pub const ABMON_8: Self = Self(131093);
    pub const ABMON_9: Self = Self(131094);
    pub const ABMON_10: Self = Self(131095);
    pub const ABMON_11: Self = Self(131096);
    pub const ABMON_12: Self = Self(131097);
    pub const MON_1: Self = Self(131098);
    pub const MON_2: Self = Self(131099);
    pub const MON_3: Self = Self(131100);
    pub const MON_4: Self = Self(131101);
    pub const MON_5: Self = Self(131102);
    pub const MON_6: Self = Self(131103);
    pub const MON_7: Self = Self(131104);
    pub const MON_8: Self = Self(131105);
    pub const MON_9: Self = Self(131106);
    pub const MON_10: Self = Self(131107);
    pub const MON_11: Self = Self(131108);
    pub const MON_12: Self = Self(131109);
    pub const AM_STR: Self = Self(131110);
    pub const PM_STR: Self = Self(131111);
    pub const D_T_FMT: Self = Self(131112);
    pub const D_FMT: Self = Self(131113);
    pub const T_FMT: Self = Self(131114);
    pub const T_FMT_AMPM: Self = Self(131115);
    pub const ERA: Self = Self(131116);
    pub const __ERA_YEAR: Self = Self(131117);
    pub const ERA_D_FMT: Self = Self(131118);
    pub const ALT_DIGITS: Self = Self(131119);
    pub const ERA_D_T_FMT: Self = Self(131120);
    pub const ERA_T_FMT: Self = Self(131121);
    pub const _NL_TIME_ERA_NUM_ENTRIES: Self = Self(131122);
    pub const _NL_TIME_ERA_ENTRIES: Self = Self(131123);
    pub const _NL_WABDAY_1: Self = Self(131124);
    pub const _NL_WABDAY_2: Self = Self(131125);
    pub const _NL_WABDAY_3: Self = Self(131126);
    pub const _NL_WABDAY_4: Self = Self(131127);
    pub const _NL_WABDAY_5: Self = Self(131128);
    pub const _NL_WABDAY_6: Self = Self(131129);
    pub const _NL_WABDAY_7: Self = Self(131130);
    pub const _NL_WDAY_1: Self = Self(131131);
    pub const _NL_WDAY_2: Self = Self(131132);
    pub const _NL_WDAY_3: Self = Self(131133);
    pub const _NL_WDAY_4: Self = Self(131134);
    pub const _NL_WDAY_5: Self = Self(131135);
    pub const _NL_WDAY_6: Self = Self(131136);
    pub const _NL_WDAY_7: Self = Self(131137);
    pub const _NL_WABMON_1: Self = Self(131138);
    pub const _NL_WABMON_2: Self = Self(131139);
    pub const _NL_WABMON_3: Self = Self(131140);
    pub const _NL_WABMON_4: Self = Self(131141);
    pub const _NL_WABMON_5: Self = Self(131142);
    pub const _NL_WABMON_6: Self = Self(131143);
    pub const _NL_WABMON_7: Self = Self(131144);
    pub const _NL_WABMON_8: Self = Self(131145);
    pub const _NL_WABMON_9: Self = Self(131146);
    pub const _NL_WABMON_10: Self = Self(131147);
    pub const _NL_WABMON_11: Self = Self(131148);
    pub const _NL_WABMON_12: Self = Self(131149);
    pub const _NL_WMON_1: Self = Self(131150);
    pub const _NL_WMON_2: Self = Self(131151);
    pub const _NL_WMON_3: Self = Self(131152);
    pub const _NL_WMON_4: Self = Self(131153);
    pub const _NL_WMON_5: Self = Self(131154);
    pub const _NL_WMON_6: Self = Self(131155);
    pub const _NL_WMON_7: Self = Self(131156);
    pub const _NL_WMON_8: Self = Self(131157);
    pub const _NL_WMON_9: Self = Self(131158);
    pub const _NL_WMON_10: Self = Self(131159);
    pub const _NL_WMON_11: Self = Self(131160);
    pub const _NL_WMON_12: Self = Self(131161);
    pub const _NL_WAM_STR: Self = Self(131162);
    pub const _NL_WPM_STR: Self = Self(131163);
    pub const _NL_WD_T_FMT: Self = Self(131164);
    pub const _NL_WD_FMT: Self = Self(131165);
    pub const _NL_WT_FMT: Self = Self(131166);
    pub const _NL_WT_FMT_AMPM: Self = Self(131167);
    pub const _NL_WERA_YEAR: Self = Self(131168);
    pub const _NL_WERA_D_FMT: Self = Self(131169);
    pub const _NL_WALT_DIGITS: Self = Self(131170);
    pub const _NL_WERA_D_T_FMT: Self = Self(131171);
    pub const _NL_WERA_T_FMT: Self = Self(131172);
    pub const _NL_TIME_WEEK_NDAYS: Self = Self(131173);
    pub const _NL_TIME_WEEK_1STDAY: Self = Self(131174);
    pub const _NL_TIME_WEEK_1STWEEK: Self = Self(131175);
    pub const _NL_TIME_FIRST_WEEKDAY: Self = Self(131176);
    pub const _NL_TIME_FIRST_WORKDAY: Self = Self(131177);
    pub const _NL_TIME_CAL_DIRECTION: Self = Self(131178);
    pub const _NL_TIME_TIMEZONE: Self = Self(131179);
    pub const _DATE_FMT: Self = Self(131180);
    pub const _NL_W_DATE_FMT: Self = Self(131181);
    pub const _NL_TIME_CODESET: Self = Self(131182);
    pub const __ALTMON_1: Self = Self(131183);
    pub const __ALTMON_2: Self = Self(131184);
    pub const __ALTMON_3: Self = Self(131185);
    pub const __ALTMON_4: Self = Self(131186);
    pub const __ALTMON_5: Self = Self(131187);
    pub const __ALTMON_6: Self = Self(131188);
    pub const __ALTMON_7: Self = Self(131189);
    pub const __ALTMON_8: Self = Self(131190);
    pub const __ALTMON_9: Self = Self(131191);
    pub const __ALTMON_10: Self = Self(131192);
    pub const __ALTMON_11: Self = Self(131193);
    pub const __ALTMON_12: Self = Self(131194);
    pub const _NL_WALTMON_1: Self = Self(131195);
    pub const _NL_WALTMON_2: Self = Self(131196);
    pub const _NL_WALTMON_3: Self = Self(131197);
    pub const _NL_WALTMON_4: Self = Self(131198);
    pub const _NL_WALTMON_5: Self = Self(131199);
    pub const _NL_WALTMON_6: Self = Self(131200);
    pub const _NL_WALTMON_7: Self = Self(131201);
    pub const _NL_WALTMON_8: Self = Self(131202);
    pub const _NL_WALTMON_9: Self = Self(131203);
    pub const _NL_WALTMON_10: Self = Self(131204);
    pub const _NL_WALTMON_11: Self = Self(131205);
    pub const _NL_WALTMON_12: Self = Self(131206);
    pub const _NL_ABALTMON_1: Self = Self(131207);
    pub const _NL_ABALTMON_2: Self = Self(131208);
    pub const _NL_ABALTMON_3: Self = Self(131209);
    pub const _NL_ABALTMON_4: Self = Self(131210);
    pub const _NL_ABALTMON_5: Self = Self(131211);
    pub const _NL_ABALTMON_6: Self = Self(131212);
    pub const _NL_ABALTMON_7: Self = Self(131213);
    pub const _NL_ABALTMON_8: Self = Self(131214);
    pub const _NL_ABALTMON_9: Self = Self(131215);
    pub const _NL_ABALTMON_10: Self = Self(131216);
    pub const _NL_ABALTMON_11: Self = Self(131217);
    pub const _NL_ABALTMON_12: Self = Self(131218);
    pub const _NL_WABALTMON_1: Self = Self(131219);
    pub const _NL_WABALTMON_2: Self = Self(131220);
    pub const _NL_WABALTMON_3: Self = Self(131221);
    pub const _NL_WABALTMON_4: Self = Self(131222);
    pub const _NL_WABALTMON_5: Self = Self(131223);
    pub const _NL_WABALTMON_6: Self = Self(131224);
    pub const _NL_WABALTMON_7: Self = Self(131225);
    pub const _NL_WABALTMON_8: Self = Self(131226);
    pub const _NL_WABALTMON_9: Self = Self(131227);
    pub const _NL_WABALTMON_10: Self = Self(131228);
    pub const _NL_WABALTMON_11: Self = Self(131229);
    pub const _NL_WABALTMON_12: Self = Self(131230);
    pub const _NL_NUM_LC_TIME: Self = Self(131231);
    pub const _NL_COLLATE_NRULES: Self = Self(196608);
    pub const _NL_COLLATE_RULESETS: Self = Self(196609);
    pub const _NL_COLLATE_TABLEMB: Self = Self(196610);
    pub const _NL_COLLATE_WEIGHTMB: Self = Self(196611);
    pub const _NL_COLLATE_EXTRAMB: Self = Self(196612);
    pub const _NL_COLLATE_INDIRECTMB: Self = Self(196613);
    pub const _NL_COLLATE_GAP1: Self = Self(196614);
    pub const _NL_COLLATE_GAP2: Self = Self(196615);
    pub const _NL_COLLATE_GAP3: Self = Self(196616);
    pub const _NL_COLLATE_TABLEWC: Self = Self(196617);
    pub const _NL_COLLATE_WEIGHTWC: Self = Self(196618);
    pub const _NL_COLLATE_EXTRAWC: Self = Self(196619);
    pub const _NL_COLLATE_INDIRECTWC: Self = Self(196620);
    pub const _NL_COLLATE_SYMB_HASH_SIZEMB: Self = Self(196621);
    pub const _NL_COLLATE_SYMB_TABLEMB: Self = Self(196622);
    pub const _NL_COLLATE_SYMB_EXTRAMB: Self = Self(196623);
    pub const _NL_COLLATE_COLLSEQMB: Self = Self(196624);
    pub const _NL_COLLATE_COLLSEQWC: Self = Self(196625);
    pub const _NL_COLLATE_CODESET: Self = Self(196626);
    pub const _NL_NUM_LC_COLLATE: Self = Self(196627);
    pub const _NL_CTYPE_CLASS: Self = Self(0);
    pub const _NL_CTYPE_TOUPPER: Self = Self(1);
    pub const _NL_CTYPE_GAP1: Self = Self(2);
    pub const _NL_CTYPE_TOLOWER: Self = Self(3);
    pub const _NL_CTYPE_GAP2: Self = Self(4);
    pub const _NL_CTYPE_CLASS32: Self = Self(5);
    pub const _NL_CTYPE_GAP3: Self = Self(6);
    pub const _NL_CTYPE_GAP4: Self = Self(7);
    pub const _NL_CTYPE_GAP5: Self = Self(8);
    pub const _NL_CTYPE_GAP6: Self = Self(9);
    pub const _NL_CTYPE_CLASS_NAMES: Self = Self(10);
    pub const _NL_CTYPE_MAP_NAMES: Self = Self(11);
    pub const _NL_CTYPE_WIDTH: Self = Self(12);
    pub const _NL_CTYPE_MB_CUR_MAX: Self = Self(13);
    pub const _NL_CTYPE_CODESET_NAME: Self = Self(14);
    pub const CODESET: Self = Self(14);
    pub const _NL_CTYPE_TOUPPER32: Self = Self(15);
    pub const _NL_CTYPE_TOLOWER32: Self = Self(16);
    pub const _NL_CTYPE_CLASS_OFFSET: Self = Self(17);
    pub const _NL_CTYPE_MAP_OFFSET: Self = Self(18);
    pub const _NL_CTYPE_INDIGITS_MB_LEN: Self = Self(19);
    pub const _NL_CTYPE_INDIGITS0_MB: Self = Self(20);
    pub const _NL_CTYPE_INDIGITS1_MB: Self = Self(21);
    pub const _NL_CTYPE_INDIGITS2_MB: Self = Self(22);
    pub const _NL_CTYPE_INDIGITS3_MB: Self = Self(23);
    pub const _NL_CTYPE_INDIGITS4_MB: Self = Self(24);
    pub const _NL_CTYPE_INDIGITS5_MB: Self = Self(25);
    pub const _NL_CTYPE_INDIGITS6_MB: Self = Self(26);
    pub const _NL_CTYPE_INDIGITS7_MB: Self = Self(27);
    pub const _NL_CTYPE_INDIGITS8_MB: Self = Self(28);
    pub const _NL_CTYPE_INDIGITS9_MB: Self = Self(29);
    pub const _NL_CTYPE_INDIGITS_WC_LEN: Self = Self(30);
    pub const _NL_CTYPE_INDIGITS0_WC: Self = Self(31);
    pub const _NL_CTYPE_INDIGITS1_WC: Self = Self(32);
    pub const _NL_CTYPE_INDIGITS2_WC: Self = Self(33);
    pub const _NL_CTYPE_INDIGITS3_WC: Self = Self(34);
    pub const _NL_CTYPE_INDIGITS4_WC: Self = Self(35);
    pub const _NL_CTYPE_INDIGITS5_WC: Self = Self(36);
    pub const _NL_CTYPE_INDIGITS6_WC: Self = Self(37);
    pub const _NL_CTYPE_INDIGITS7_WC: Self = Self(38);
    pub const _NL_CTYPE_INDIGITS8_WC: Self = Self(39);
    pub const _NL_CTYPE_INDIGITS9_WC: Self = Self(40);
    pub const _NL_CTYPE_OUTDIGIT0_MB: Self = Self(41);
    pub const _NL_CTYPE_OUTDIGIT1_MB: Self = Self(42);
    pub const _NL_CTYPE_OUTDIGIT2_MB: Self = Self(43);
    pub const _NL_CTYPE_OUTDIGIT3_MB: Self = Self(44);
    pub const _NL_CTYPE_OUTDIGIT4_MB: Self = Self(45);
    pub const _NL_CTYPE_OUTDIGIT5_MB: Self = Self(46);
    pub const _NL_CTYPE_OUTDIGIT6_MB: Self = Self(47);
    pub const _NL_CTYPE_OUTDIGIT7_MB: Self = Self(48);
    pub const _NL_CTYPE_OUTDIGIT8_MB: Self = Self(49);
    pub const _NL_CTYPE_OUTDIGIT9_MB: Self = Self(50);
    pub const _NL_CTYPE_OUTDIGIT0_WC: Self = Self(51);
    pub const _NL_CTYPE_OUTDIGIT1_WC: Self = Self(52);
    pub const _NL_CTYPE_OUTDIGIT2_WC: Self = Self(53);
    pub const _NL_CTYPE_OUTDIGIT3_WC: Self = Self(54);
    pub const _NL_CTYPE_OUTDIGIT4_WC: Self = Self(55);
    pub const _NL_CTYPE_OUTDIGIT5_WC: Self = Self(56);
    pub const _NL_CTYPE_OUTDIGIT6_WC: Self = Self(57);
    pub const _NL_CTYPE_OUTDIGIT7_WC: Self = Self(58);
    pub const _NL_CTYPE_OUTDIGIT8_WC: Self = Self(59);
    pub const _NL_CTYPE_OUTDIGIT9_WC: Self = Self(60);
    pub const _NL_CTYPE_TRANSLIT_TAB_SIZE: Self = Self(61);
    pub const _NL_CTYPE_TRANSLIT_FROM_IDX: Self = Self(62);
    pub const _NL_CTYPE_TRANSLIT_FROM_TBL: Self = Self(63);
    pub const _NL_CTYPE_TRANSLIT_TO_IDX: Self = Self(64);
    pub const _NL_CTYPE_TRANSLIT_TO_TBL: Self = Self(65);
    pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING_LEN: Self = Self(66);
    pub const _NL_CTYPE_TRANSLIT_DEFAULT_MISSING: Self = Self(67);
    pub const _NL_CTYPE_TRANSLIT_IGNORE_LEN: Self = Self(68);
    pub const _NL_CTYPE_TRANSLIT_IGNORE: Self = Self(69);
    pub const _NL_CTYPE_MAP_TO_NONASCII: Self = Self(70);
    pub const _NL_CTYPE_NONASCII_CASE: Self = Self(71);
    pub const _NL_CTYPE_EXTRA_MAP_1: Self = Self(72);
    pub const _NL_CTYPE_EXTRA_MAP_2: Self = Self(73);
    pub const _NL_CTYPE_EXTRA_MAP_3: Self = Self(74);
    pub const _NL_CTYPE_EXTRA_MAP_4: Self = Self(75);
    pub const _NL_CTYPE_EXTRA_MAP_5: Self = Self(76);
    pub const _NL_CTYPE_EXTRA_MAP_6: Self = Self(77);
    pub const _NL_CTYPE_EXTRA_MAP_7: Self = Self(78);
    pub const _NL_CTYPE_EXTRA_MAP_8: Self = Self(79);
    pub const _NL_CTYPE_EXTRA_MAP_9: Self = Self(80);
    pub const _NL_CTYPE_EXTRA_MAP_10: Self = Self(81);
    pub const _NL_CTYPE_EXTRA_MAP_11: Self = Self(82);
    pub const _NL_CTYPE_EXTRA_MAP_12: Self = Self(83);
    pub const _NL_CTYPE_EXTRA_MAP_13: Self = Self(84);
    pub const _NL_CTYPE_EXTRA_MAP_14: Self = Self(85);
    pub const _NL_NUM_LC_CTYPE: Self = Self(86);
    pub const __INT_CURR_SYMBOL: Self = Self(262144);
    pub const __CURRENCY_SYMBOL: Self = Self(262145);
    pub const __MON_DECIMAL_POINT: Self = Self(262146);
    pub const __MON_THOUSANDS_SEP: Self = Self(262147);
    pub const __MON_GROUPING: Self = Self(262148);
    pub const __POSITIVE_SIGN: Self = Self(262149);
    pub const __NEGATIVE_SIGN: Self = Self(262150);
    pub const __INT_FRAC_DIGITS: Self = Self(262151);
    pub const __FRAC_DIGITS: Self = Self(262152);
    pub const __P_CS_PRECEDES: Self = Self(262153);
    pub const __P_SEP_BY_SPACE: Self = Self(262154);
    pub const __N_CS_PRECEDES: Self = Self(262155);
    pub const __N_SEP_BY_SPACE: Self = Self(262156);
    pub const __P_SIGN_POSN: Self = Self(262157);
    pub const __N_SIGN_POSN: Self = Self(262158);
    pub const _NL_MONETARY_CRNCYSTR: Self = Self(262159);
    pub const __INT_P_CS_PRECEDES: Self = Self(262160);
    pub const __INT_P_SEP_BY_SPACE: Self = Self(262161);
    pub const __INT_N_CS_PRECEDES: Self = Self(262162);
    pub const __INT_N_SEP_BY_SPACE: Self = Self(262163);
    pub const __INT_P_SIGN_POSN: Self = Self(262164);
    pub const __INT_N_SIGN_POSN: Self = Self(262165);
    pub const _NL_MONETARY_DUO_INT_CURR_SYMBOL: Self = Self(262166);
    pub const _NL_MONETARY_DUO_CURRENCY_SYMBOL: Self = Self(262167);
    pub const _NL_MONETARY_DUO_INT_FRAC_DIGITS: Self = Self(262168);
    pub const _NL_MONETARY_DUO_FRAC_DIGITS: Self = Self(262169);
    pub const _NL_MONETARY_DUO_P_CS_PRECEDES: Self = Self(262170);
    pub const _NL_MONETARY_DUO_P_SEP_BY_SPACE: Self = Self(262171);
    pub const _NL_MONETARY_DUO_N_CS_PRECEDES: Self = Self(262172);
    pub const _NL_MONETARY_DUO_N_SEP_BY_SPACE: Self = Self(262173);
    pub const _NL_MONETARY_DUO_INT_P_CS_PRECEDES: Self = Self(262174);
    pub const _NL_MONETARY_DUO_INT_P_SEP_BY_SPACE: Self = Self(262175);
    pub const _NL_MONETARY_DUO_INT_N_CS_PRECEDES: Self = Self(262176);
    pub const _NL_MONETARY_DUO_INT_N_SEP_BY_SPACE: Self = Self(262177);
    pub const _NL_MONETARY_DUO_P_SIGN_POSN: Self = Self(262178);
    pub const _NL_MONETARY_DUO_N_SIGN_POSN: Self = Self(262179);
    pub const _NL_MONETARY_DUO_INT_P_SIGN_POSN: Self = Self(262180);
    pub const _NL_MONETARY_DUO_INT_N_SIGN_POSN: Self = Self(262181);
    pub const _NL_MONETARY_UNO_VALID_FROM: Self = Self(262182);
    pub const _NL_MONETARY_UNO_VALID_TO: Self = Self(262183);
    pub const _NL_MONETARY_DUO_VALID_FROM: Self = Self(262184);
    pub const _NL_MONETARY_DUO_VALID_TO: Self = Self(262185);
    pub const _NL_MONETARY_CONVERSION_RATE: Self = Self(262186);
    pub const _NL_MONETARY_DECIMAL_POINT_WC: Self = Self(262187);
    pub const _NL_MONETARY_THOUSANDS_SEP_WC: Self = Self(262188);
    pub const _NL_MONETARY_CODESET: Self = Self(262189);
    pub const _NL_NUM_LC_MONETARY: Self = Self(262190);
    pub const __DECIMAL_POINT: Self = Self(65536);
    pub const RADIXCHAR: Self = Self(65536);
    pub const __THOUSANDS_SEP: Self = Self(65537);
    pub const THOUSEP: Self = Self(65537);
    pub const __GROUPING: Self = Self(65538);
    pub const _NL_NUMERIC_DECIMAL_POINT_WC: Self = Self(65539);
    pub const _NL_NUMERIC_THOUSANDS_SEP_WC: Self = Self(65540);
    pub const _NL_NUMERIC_CODESET: Self = Self(65541);
    pub const _NL_NUM_LC_NUMERIC: Self = Self(65542);
    pub const __YESEXPR: Self = Self(327680);
    pub const __NOEXPR: Self = Self(327681);
    pub const __YESSTR: Self = Self(327682);
    pub const __NOSTR: Self = Self(327683);
    pub const _NL_MESSAGES_CODESET: Self = Self(327684);
    pub const _NL_NUM_LC_MESSAGES: Self = Self(327685);
    pub const _NL_PAPER_HEIGHT: Self = Self(458752);
    pub const _NL_PAPER_WIDTH: Self = Self(458753);
    pub const _NL_PAPER_CODESET: Self = Self(458754);
    pub const _NL_NUM_LC_PAPER: Self = Self(458755);
    pub const _NL_NAME_NAME_FMT: Self = Self(524288);
    pub const _NL_NAME_NAME_GEN: Self = Self(524289);
    pub const _NL_NAME_NAME_MR: Self = Self(524290);
    pub const _NL_NAME_NAME_MRS: Self = Self(524291);
    pub const _NL_NAME_NAME_MISS: Self = Self(524292);
    pub const _NL_NAME_NAME_MS: Self = Self(524293);
    pub const _NL_NAME_CODESET: Self = Self(524294);
    pub const _NL_NUM_LC_NAME: Self = Self(524295);
    pub const _NL_ADDRESS_POSTAL_FMT: Self = Self(589824);
    pub const _NL_ADDRESS_COUNTRY_NAME: Self = Self(589825);
    pub const _NL_ADDRESS_COUNTRY_POST: Self = Self(589826);
    pub const _NL_ADDRESS_COUNTRY_AB2: Self = Self(589827);
    pub const _NL_ADDRESS_COUNTRY_AB3: Self = Self(589828);
    pub const _NL_ADDRESS_COUNTRY_CAR: Self = Self(589829);
    pub const _NL_ADDRESS_COUNTRY_NUM: Self = Self(589830);
    pub const _NL_ADDRESS_COUNTRY_ISBN: Self = Self(589831);
    pub const _NL_ADDRESS_LANG_NAME: Self = Self(589832);
    pub const _NL_ADDRESS_LANG_AB: Self = Self(589833);
    pub const _NL_ADDRESS_LANG_TERM: Self = Self(589834);
    pub const _NL_ADDRESS_LANG_LIB: Self = Self(589835);
    pub const _NL_ADDRESS_CODESET: Self = Self(589836);
    pub const _NL_NUM_LC_ADDRESS: Self = Self(589837);
    pub const _NL_TELEPHONE_TEL_INT_FMT: Self = Self(655360);
    pub const _NL_TELEPHONE_TEL_DOM_FMT: Self = Self(655361);
    pub const _NL_TELEPHONE_INT_SELECT: Self = Self(655362);
    pub const _NL_TELEPHONE_INT_PREFIX: Self = Self(655363);
    pub const _NL_TELEPHONE_CODESET: Self = Self(655364);
    pub const _NL_NUM_LC_TELEPHONE: Self = Self(655365);
    pub const _NL_MEASUREMENT_MEASUREMENT: Self = Self(720896);
    pub const _NL_MEASUREMENT_CODESET: Self = Self(720897);
    pub const _NL_NUM_LC_MEASUREMENT: Self = Self(720898);
    pub const _NL_IDENTIFICATION_TITLE: Self = Self(786432);
    pub const _NL_IDENTIFICATION_SOURCE: Self = Self(786433);
    pub const _NL_IDENTIFICATION_ADDRESS: Self = Self(786434);
    pub const _NL_IDENTIFICATION_CONTACT: Self = Self(786435);
    pub const _NL_IDENTIFICATION_EMAIL: Self = Self(786436);
    pub const _NL_IDENTIFICATION_TEL: Self = Self(786437);
    pub const _NL_IDENTIFICATION_FAX: Self = Self(786438);
    pub const _NL_IDENTIFICATION_LANGUAGE: Self = Self(786439);
    pub const _NL_IDENTIFICATION_TERRITORY: Self = Self(786440);
    pub const _NL_IDENTIFICATION_AUDIENCE: Self = Self(786441);
    pub const _NL_IDENTIFICATION_APPLICATION: Self = Self(786442);
    pub const _NL_IDENTIFICATION_ABBREVIATION: Self = Self(786443);
    pub const _NL_IDENTIFICATION_REVISION: Self = Self(786444);
    pub const _NL_IDENTIFICATION_DATE: Self = Self(786445);
    pub const _NL_IDENTIFICATION_CATEGORY: Self = Self(786446);
    pub const _NL_IDENTIFICATION_CODESET: Self = Self(786447);
    pub const _NL_NUM_LC_IDENTIFICATION: Self = Self(786448);
    pub const _NL_NUM: Self = Self(786449);
}
pub const FILENAME_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const ENODEV: ::core::ffi::c_int = 19 as ::core::ffi::c_int;
pub const MAXPATHLEN: ::core::ffi::c_int = PATH_MAX;
pub const PATH_MAX: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const SIG_DFL: __sighandler_t = None;
pub const SIGINT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SIGSEGV: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGQUIT: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const SIGTSTP: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const SIGCONT: ::core::ffi::c_int = 18 as ::core::ffi::c_int;
pub const SIGCHLD: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const SIGTTIN: ::core::ffi::c_int = 21 as ::core::ffi::c_int;
pub const SIGTTOU: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGUSR2: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const F_OK: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_RDWR: ::core::ffi::c_int = 0o2 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_GETFL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const FNONBLOCK: ::core::ffi::c_int = O_NONBLOCK;
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL_0: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const WNOHANG: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const WUNTRACED: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const ::core::ffi::c_char) -> ::core::ffi::c_int {
    return strtol(
        __nptr,
        NULL as *mut *mut ::core::ffi::c_char,
        10 as ::core::ffi::c_int,
    ) as ::core::ffi::c_int;
}
pub const VINTR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const MAXSTR: ::core::ffi::c_int = 768 as ::core::ffi::c_int;
pub const MSGWAIT: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const MSGMINWAIT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SILENCEWAIT: ::core::ffi::c_int = 30 as ::core::ffi::c_int;
pub const DEFAULT_BUFFERFILE: [::core::ffi::c_char; 21] = unsafe {
    ::core::mem::transmute::<[u8; 21], [::core::ffi::c_char; 21]>(*b"/tmp/screen-exchange\0")
};
pub const UTF8: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const F_UWP: ::core::ffi::c_int = 0x1000 as ::core::ffi::c_int;
pub const FLOW_OFF: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const FLOW_ON: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 0 as ::core::ffi::c_int;
pub const FLOW_AUTOFLAG: ::core::ffi::c_int = (1 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int;
pub const D_DETACH: ::core::ffi::c_int = 0;
pub const D_STOP: ::core::ffi::c_int = 1;
pub const D_REMOTE: ::core::ffi::c_int = 2;
pub const D_POWER: ::core::ffi::c_int = 3;
pub const D_REMOTE_POWER: ::core::ffi::c_int = 4;
pub const D_LOCK: ::core::ffi::c_int = 5;
pub const D_HANGUP: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const MSG_ATTACH: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MSG_DETACH: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const SIG_BYE: ::core::ffi::c_int = SIGHUP;
pub const SIG_POWER_BYE: ::core::ffi::c_int = SIGUSR1;
pub const SIG_LOCK: ::core::ffi::c_int = SIGUSR2;
pub const SIG_STOP: ::core::ffi::c_int = SIGTSTP;
pub const VBELLWAIT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BELL_ON: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const BELL_FOUND: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BELL_DONE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BELL_VISUAL: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const MON_ON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const MON_FOUND: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const MON_DONE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const SILENCE_ON: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SILENCE_FOUND: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const SYSTEM_SCREENRC: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"/etc/screenrc\0") };
pub const VERSION_MAJOR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const VERSION_MINOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const VERSION_REVISION: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[export_name = "rboxc_screen_force_vt"]
pub static mut force_vt: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
#[export_name = "rboxc_screen_VBellWait"]
pub static mut VBellWait: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_MsgWait"]
pub static mut MsgWait: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_MsgMinWait"]
pub static mut MsgMinWait: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_SilenceWait"]
pub static mut SilenceWait: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_ShellProg"]
pub static mut ShellProg: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_ShellArgs"]
pub static mut ShellArgs: [*mut ::core::ffi::c_char; 2] =
    [::core::ptr::null_mut::<::core::ffi::c_char>(); 2];
#[export_name = "rboxc_screen_nversion"]
pub static mut nversion: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_do_auth"]
pub static mut do_auth: bool = r#false != 0;
#[export_name = "rboxc_screen_ppp"]
pub static mut ppp: *mut passwd = ::core::ptr::null_mut::<passwd>();
#[export_name = "rboxc_screen_attach_tty"]
pub static mut attach_tty: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_attach_fd"]
pub static mut attach_fd: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_screen_attach_term"]
pub static mut attach_term: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_LoginName"]
pub static mut LoginName: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_attach_Mode"]
pub static mut attach_Mode: mode = mode {
    tio: termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_line: 0,
        c_cc: [0; 32],
        c2rust_unnamed: C2Rust_Unnamed_0 { __ispeed: 0 },
        c2rust_unnamed_0: C2Rust_Unnamed { __ospeed: 0 },
    },
};
#[export_name = "rboxc_screen_attach_tty_is_in_new_ns"]
pub static mut attach_tty_is_in_new_ns: bool = r#false != 0;
#[export_name = "rboxc_screen_attach_tty_name_in_ns"]
pub static mut attach_tty_name_in_ns: [::core::ffi::c_char; 4096] = [0; 4096];
#[export_name = "rboxc_screen_SocketPath"]
pub static mut SocketPath: [::core::ffi::c_char; 4098] = [0; 4098];
#[export_name = "rboxc_screen_SocketName"]
pub static mut SocketName: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_SocketMatch"]
pub static mut SocketMatch: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_ServerSocket"]
pub static mut ServerSocket: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_screen_serv_read"]
pub static mut serv_read: Event = Event {
    next: ::core::ptr::null_mut::<Event>(),
    handler: None,
    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    fd: 0,
    r#type: EventType::EV_TIMEOUT,
    priority: 0,
    timeout: 0,
    queued: false,
    condpos: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    condneg: ::core::ptr::null_mut::<::core::ffi::c_int>(),
};
#[export_name = "rboxc_screen_serv_select"]
pub static mut serv_select: Event = Event {
    next: ::core::ptr::null_mut::<Event>(),
    handler: None,
    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    fd: 0,
    r#type: EventType::EV_TIMEOUT,
    priority: 0,
    timeout: 0,
    queued: false,
    condpos: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    condneg: ::core::ptr::null_mut::<::core::ffi::c_int>(),
};
#[export_name = "rboxc_screen_logflushev"]
pub static mut logflushev: Event = Event {
    next: ::core::ptr::null_mut::<Event>(),
    handler: None,
    data: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    fd: 0,
    r#type: EventType::EV_TIMEOUT,
    priority: 0,
    timeout: 0,
    queued: false,
    condpos: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    condneg: ::core::ptr::null_mut::<::core::ffi::c_int>(),
};
#[export_name = "rboxc_screen_NewEnv"]
pub static mut NewEnv: *mut *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
#[export_name = "rboxc_screen_RcFileName"]
pub static mut RcFileName: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_home"]
pub static mut home: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_screenlogfile"]
pub static mut screenlogfile: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_log_flush"]
pub static mut log_flush: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
#[export_name = "rboxc_screen_logtstamp_on"]
pub static mut logtstamp_on: bool = r#false != 0;
#[export_name = "rboxc_screen_logtstamp_string"]
pub static mut logtstamp_string: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_logtstamp_after"]
pub static mut logtstamp_after: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
#[export_name = "rboxc_screen_hardcopydir"]
pub static mut hardcopydir: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_BellString"]
pub static mut BellString: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_VisualBellString"]
pub static mut VisualBellString: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_ActivityString"]
pub static mut ActivityString: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_BufferFile"]
pub static mut BufferFile: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_PowDetachString"]
pub static mut PowDetachString: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_hstatusstring"]
pub static mut hstatusstring: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_captionstring"]
pub static mut captionstring: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_wliststr"]
pub static mut wliststr: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_wlisttit"]
pub static mut wlisttit: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_auto_detach"]
pub static mut auto_detach: bool = r#true != 0;
#[export_name = "rboxc_screen_adaptflag"]
pub static mut adaptflag: bool = false;
#[export_name = "rboxc_screen_iflag"]
pub static mut iflag: bool = false;
#[export_name = "rboxc_screen_lsflag"]
pub static mut lsflag: bool = false;
#[export_name = "rboxc_screen_quietflag"]
pub static mut quietflag: bool = false;
#[export_name = "rboxc_screen_wipeflag"]
pub static mut wipeflag: bool = false;
#[export_name = "rboxc_screen_xflag"]
pub static mut xflag: bool = false;
#[export_name = "rboxc_screen_rflag"]
pub static mut rflag: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_dflag"]
pub static mut dflag: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_queryflag"]
pub static mut queryflag: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
#[export_name = "rboxc_screen_hastruecolor"]
pub static mut hastruecolor: bool = r#false != 0;
#[export_name = "rboxc_screen_multi"]
pub static mut multi: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_multiattach"]
pub static mut multiattach: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_HostName"]
pub static mut HostName: [::core::ffi::c_char; 768] = [0; 768];
#[export_name = "rboxc_screen_MasterPid"]
pub static mut MasterPid: pid_t = 0;
#[export_name = "rboxc_screen_PanicPid"]
pub static mut PanicPid: pid_t = 0;
#[export_name = "rboxc_screen_real_uid"]
pub static mut real_uid: uid_t = 0;
#[export_name = "rboxc_screen_eff_uid"]
pub static mut eff_uid: uid_t = 0;
#[export_name = "rboxc_screen_multi_uid"]
pub static mut multi_uid: uid_t = 0;
#[export_name = "rboxc_screen_own_uid"]
pub static mut own_uid: uid_t = 0;
#[export_name = "rboxc_screen_real_gid"]
pub static mut real_gid: gid_t = 0;
#[export_name = "rboxc_screen_eff_gid"]
pub static mut eff_gid: gid_t = 0;
#[export_name = "rboxc_screen_default_startup"]
pub static mut default_startup: bool = false;
#[export_name = "rboxc_screen_ZombieKey_destroy"]
pub static mut ZombieKey_destroy: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_ZombieKey_resurrect"]
pub static mut ZombieKey_resurrect: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_ZombieKey_onerror"]
pub static mut ZombieKey_onerror: ::core::ffi::c_int = 0;
#[export_name = "rboxc_screen_preselect"]
pub static mut preselect: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_screenencodings"]
pub static mut screenencodings: *mut ::core::ffi::c_char =
    ::core::ptr::null_mut::<::core::ffi::c_char>();
#[export_name = "rboxc_screen_cjkwidth"]
pub static mut cjkwidth: bool = false;
#[export_name = "rboxc_screen_flayer"]
pub static mut flayer: *mut Layer = ::core::ptr::null_mut::<Layer>();
#[export_name = "rboxc_screen_fore"]
pub static mut fore: *mut Window = ::core::ptr::null_mut::<Window>();
#[export_name = "rboxc_screen_mru_window"]
pub static mut mru_window: *mut Window = ::core::ptr::null_mut::<Window>();
#[export_name = "rboxc_screen_first_window"]
pub static mut first_window: *mut Window = ::core::ptr::null_mut::<Window>();
#[export_name = "rboxc_screen_last_window"]
pub static mut last_window: *mut Window = ::core::ptr::null_mut::<Window>();
#[export_name = "rboxc_screen_console_window"]
pub static mut console_window: *mut Window = ::core::ptr::null_mut::<Window>();
#[export_name = "rboxc_screen_strnomem"]
pub static mut strnomem: [::core::ffi::c_char; 15] =
    unsafe { ::core::mem::transmute::<[u8; 15], [::core::ffi::c_char; 15]>(*b"Out of memory.\0") };
static mut InterruptPlease: ::core::ffi::c_int = 0;
static mut GotSigChld: ::core::ffi::c_int = 0;
unsafe extern "C" fn lf_secreopen(
    mut name: *mut ::core::ffi::c_char,
    mut wantfd: ::core::ffi::c_int,
    mut l: *mut Log,
) -> ::core::ffi::c_int {
    let mut got_fd: ::core::ffi::c_int = 0;
    close(wantfd);
    got_fd = secopen(
        name,
        O_WRONLY | O_CREAT | O_APPEND,
        0o666 as ::core::ffi::c_int,
    );
    if got_fd < 0 as ::core::ffi::c_int || lf_move_fd(got_fd, wantfd) < 0 as ::core::ffi::c_int {
        logfclose(l);
        return -1 as ::core::ffi::c_int;
    }
    (*(*l).st).st_dev = 0 as __dev_t;
    (*(*l).st).st_ino = (*(*l).st).st_dev as __ino_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn getpwbyname(
    mut name: *mut ::core::ffi::c_char,
    mut ppp_0: *mut passwd,
) -> *mut passwd {
    let mut n: ::core::ffi::c_int = 0;
    if ppp_0.is_null() && {
        ppp_0 = getpwnam(name);
        ppp_0.is_null()
    } {
        return ::core::ptr::null_mut::<passwd>();
    }
    n = 0 as ::core::ffi::c_int;
    if *(*ppp_0).pw_passwd.offset(0isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        && *(*ppp_0).pw_passwd.offset(1isize) as ::core::ffi::c_int == '#' as ::core::ffi::c_int
        && strcmp(
            (*ppp_0).pw_passwd.offset(2 as ::core::ffi::c_int as isize),
            (*ppp_0).pw_name,
        ) == 0 as ::core::ffi::c_int
    {
        n = 13 as ::core::ffi::c_int;
    }
    while n < 13 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_char = *(*ppp_0).pw_passwd.offset(n as isize);
        if !(c as ::core::ffi::c_int == '.' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '/' as ::core::ffi::c_int
            || c as ::core::ffi::c_int == '$' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= '9' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'a' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'z' as ::core::ffi::c_int
            || c as ::core::ffi::c_int >= 'A' as ::core::ffi::c_int
                && c as ::core::ffi::c_int <= 'Z' as ::core::ffi::c_int)
        {
            break;
        }
        n += 1;
    }
    if n < 13 as ::core::ffi::c_int {
        (*ppp_0).pw_passwd = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if !(*ppp_0).pw_passwd.is_null()
        && strlen((*ppp_0).pw_passwd)
            == (13 as ::core::ffi::c_int + 11 as ::core::ffi::c_int) as size_t
    {
        *(*ppp_0).pw_passwd.offset(13isize) = 0 as ::core::ffi::c_char;
    }
    return ppp_0;
}
unsafe extern "C" fn locale_name() -> *mut ::core::ffi::c_char {
    static mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = getenv(b"LC_ALL\0".as_ptr() as *const ::core::ffi::c_char);
    if s.is_null() {
        s = getenv(b"LC_CTYPE\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if s.is_null() {
        s = getenv(b"LANG\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if s.is_null() {
        s = b"C\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    }
    return s;
}
unsafe extern "C" fn exit_with_usage(
    mut myname: *mut ::core::ffi::c_char,
    mut message: *mut ::core::ffi::c_char,
    mut arg: *mut ::core::ffi::c_char,
) {
    printf(
        b"Use: %s [-opts] [cmd [args]]\n\0".as_ptr() as *const ::core::ffi::c_char,
        myname,
    );
    printf(
        b" or: %s -r [host.tty]\n\nOptions:\n\0".as_ptr() as *const ::core::ffi::c_char,
        myname,
    );
    printf(
        b"-a            Force all capabilities into each window's termcap.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-A -[r|R]     Adapt all windows to the new display width & height.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-c file       Read configuration file instead of '.screenrc'.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-d (-r)       Detach the elsewhere running screen (and reattach here).\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-dmS name     Start as daemon: Screen session in detached mode.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-D (-r)       Detach and logout remote (and reattach here).\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-D -RR        Do whatever is needed to get a screen session.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(b"-e xy         Change command characters.\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"-f            Flow control on, -fn = off, -fa = auto.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-h lines      Set the size of the scrollback history buffer.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-i            Interrupt output sooner when flow control is on.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(b"-ls [match]   or\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"-list         Do nothing, just list our SocketDir [on possible matches].\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(b"-L            Turn on output logging.\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(b"-Logfile file Set logfile name.\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"-m            ignore $STY variable, do create a new screen session.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-O            Choose optimal output rather than exact vt100 emulation.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-p window     Preselect the named window if it exists.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-P            Tell screen to enable authentication.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-q            Quiet startup. Exits with non-zero return code if unsuccessful.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    printf(
        b"-Q            Commands will send the response to the stdout of the querying process.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    printf(
        b"-r [session]  Reattach to a detached screen process.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-R            Reattach if possible, otherwise start a new session.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-s shell      Shell to execute rather than $SHELL.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-S sockname   Name this session <pid>.sockname instead of <pid>.<tty>.<host>.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    printf(b"-t title      Set title. (window's name).\n\0".as_ptr() as *const ::core::ffi::c_char);
    printf(
        b"-T term       Use term as $TERM for windows, rather than \"screen\".\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-U            Tell screen to use UTF-8 encoding.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-v            Print \"Screen version %s\".\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut version as *mut ::core::ffi::c_char,
    );
    printf(
        b"-wipe [match] Do nothing, just clean up SocketDir [on possible matches].\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-x            Attach to a not detached screen. (Multi display mode).\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    printf(
        b"-X            Execute <cmd> as a screen command in the specified session.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    if !message.is_null() && *message as ::core::ffi::c_int != 0 {
        printf(b"\nError: \0".as_ptr() as *const ::core::ffi::c_char);
        printf(message, arg);
        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        exit(1 as ::core::ffi::c_int);
    }
    exit(0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_screen(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut n: ::core::ffi::c_int = 0;
    let mut ap: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut av0: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut socknamebuf: [::core::ffi::c_char; 4097] = [0; 4097];
    let mut mflag: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut myname: *mut ::core::ffi::c_char = (if argc == 0 as ::core::ffi::c_int {
        b"screen\0".as_ptr() as *const ::core::ffi::c_char
    } else {
        *argv.offset(0isize) as *const ::core::ffi::c_char
    }) as *mut ::core::ffi::c_char;
    let mut SocketDir: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
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
    let mut oumask: mode_t = 0;
    let mut nwin: NewWindow = NewWindow {
        StartAt: 0,
        aka: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        args: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        dir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        term: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        aflag: false,
        dynamicaka: false,
        flowflag: 0,
        list_order: false,
        list_nested: false,
        lflag: 0,
        histheight: 0,
        monitor: 0,
        wlock: 0,
        silence: 0,
        wrap: false,
        Lflag: false,
        slow: 0,
        gr: 0,
        c1: false,
        bce: 0,
        encoding: 0,
        hstatus: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        charset: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        poll_zombie_timeout: 0,
    };
    let mut detached: bool = r#false != 0;
    let mut sockp: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut sty: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut multi_home: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut cmdflag: bool = false;
    closeallfiles(0 as ::core::ffi::c_int);
    snprintf(
        &raw mut version as *mut ::core::ffi::c_char,
        59 as size_t,
        b"%d.%d.%d (build on %s) \0".as_ptr() as *const ::core::ffi::c_char,
        VERSION_MAJOR,
        VERSION_MINOR,
        VERSION_REVISION,
        BUILD_DATE.as_ptr(),
    );
    nversion = VERSION_MAJOR * 10000 as ::core::ffi::c_int
        + VERSION_MINOR * 100 as ::core::ffi::c_int
        + VERSION_REVISION;
    BellString = SaveStr(b"Bell in window %n\0".as_ptr() as *const ::core::ffi::c_char);
    VisualBellString = SaveStr(b"   Wuff,  Wuff!!  \0".as_ptr() as *const ::core::ffi::c_char);
    ActivityString = SaveStr(b"Activity in window %n\0".as_ptr() as *const ::core::ffi::c_char);
    screenlogfile = SaveStr(b"screenlog.%n\0".as_ptr() as *const ::core::ffi::c_char);
    logtstamp_string =
        SaveStr(b"-- %n:%t -- time-stamp -- %M/%d/%y %c:%s --\n\0".as_ptr()
            as *const ::core::ffi::c_char);
    hstatusstring = SaveStr(b"%h\0".as_ptr() as *const ::core::ffi::c_char);
    captionstring = SaveStr(b"%4n %t\0".as_ptr() as *const ::core::ffi::c_char);
    wlisttit = SaveStr(b" Num Name%=Flags\0".as_ptr() as *const ::core::ffi::c_char);
    wliststr = SaveStr(b"%4n %t%=%f\0".as_ptr() as *const ::core::ffi::c_char);
    BufferFile = SaveStr(DEFAULT_BUFFERFILE.as_ptr());
    ShellProg = ::core::ptr::null_mut::<::core::ffi::c_char>();
    PowDetachString = ::core::ptr::null_mut::<::core::ffi::c_char>();
    default_startup = if argc > 1 as ::core::ffi::c_int {
        r#false
    } else {
        r#true
    } != 0;
    adaptflag = r#false != 0;
    VBellWait = VBELLWAIT * 1000 as ::core::ffi::c_int;
    MsgWait = MSGWAIT * 1000 as ::core::ffi::c_int;
    MsgMinWait = MSGMINWAIT * 1000 as ::core::ffi::c_int;
    SilenceWait = SILENCEWAIT;
    zmodem_sendcmd = SaveStr(b"!!! sz -vv -b \0".as_ptr() as *const ::core::ffi::c_char);
    zmodem_recvcmd = SaveStr(b"!!! rz -vv -b -E\0".as_ptr() as *const ::core::ffi::c_char);
    CompileKeys(
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as ::core::ffi::c_int,
        &raw mut mark_key_tab as *mut ::core::ffi::c_uchar,
    );
    InitBuiltinTabs();
    screenencodings = SaveStr(SCREENENCODINGS.as_ptr());
    cjkwidth = false;
    nwin = nwin_undef;
    nwin_options = nwin_undef;
    strncpy(
        &raw mut screenterm as *mut ::core::ffi::c_char,
        b"screen\0".as_ptr() as *const ::core::ffi::c_char,
        MAXTERMLEN as size_t,
    );
    *(&raw mut screenterm as *mut ::core::ffi::c_char).offset(MAXTERMLEN as isize) =
        '\0' as ::core::ffi::c_char;
    logreopen_register(Some(
        lf_secreopen
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_char,
                ::core::ffi::c_int,
                *mut Log,
            ) -> ::core::ffi::c_int,
    ));
    real_uid = getuid() as uid_t;
    real_gid = getgid() as gid_t;
    eff_uid = geteuid() as uid_t;
    eff_gid = getegid() as gid_t;
    av0 = *argv;
    if *av0 as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
        rflag = 4 as ::core::ffi::c_int;
        xflag = r#true != 0;
        ShellProg = SaveStr(&raw mut DefaultShell as *mut ::core::ffi::c_char);
    }
    while argc > 0 as ::core::ffi::c_int {
        argv = argv.offset(1);
        ap = *argv;
        argc -= 1;
        if !(argc > 0 as ::core::ffi::c_int
            && *ap as ::core::ffi::c_int == '-' as ::core::ffi::c_int)
        {
            break;
        }
        if *ap.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            && *ap.offset(2isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            argv = argv.offset(1);
            argc -= 1;
            break;
        } else {
            if *ap.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                && strncmp(
                    ap,
                    b"--version\0".as_ptr() as *const ::core::ffi::c_char,
                    9 as size_t,
                ) == 0
            {
                printf(
                    b"Screen version %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut version as *mut ::core::ffi::c_char,
                );
                exit(0 as ::core::ffi::c_int);
            }
            if *ap.offset(1isize) as ::core::ffi::c_int == '-' as ::core::ffi::c_int
                && strncmp(
                    ap,
                    b"--help\0".as_ptr() as *const ::core::ffi::c_char,
                    6 as size_t,
                ) == 0
            {
                exit_with_usage(
                    myname,
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                );
            }
            while !ap.is_null() && *ap as ::core::ffi::c_int != 0 && {
                ap = ap.offset(1);
                *ap as ::core::ffi::c_int != 0
            } {
                's_819: {
                    match *ap as ::core::ffi::c_int {
                        97 => {
                            nwin_options.aflag = r#true != 0;
                            break 's_819;
                        }
                        65 => {
                            adaptflag = r#true != 0;
                            break 's_819;
                        }
                        112 => {
                            ap = ap.offset(1);
                            if *ap != 0 {
                                preselect = ap;
                            } else {
                                argc -= 1;
                                if argc == 0 {
                                    exit_with_usage(
                                        myname,
                                        b"Specify a window to preselect with -p\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                }
                                argv = argv.offset(1);
                                preselect = *argv;
                            }
                            ap = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            break 's_819;
                        }
                        80 => {
                            do_auth = r#true != 0;
                            break 's_819;
                        }
                        99 => {
                            ap = ap.offset(1);
                            if *ap != 0 {
                                RcFileName = ap;
                            } else {
                                argc -= 1;
                                if argc == 0 as ::core::ffi::c_int {
                                    exit_with_usage(
                                        myname,
                                        b"Specify an alternate rc-filename with -c\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                }
                                argv = argv.offset(1);
                                RcFileName = *argv;
                            }
                            ap = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            break 's_819;
                        }
                        101 => {
                            ap = ap.offset(1);
                            if *ap == 0 {
                                argc -= 1;
                                if argc == 0 as ::core::ffi::c_int {
                                    exit_with_usage(
                                        myname,
                                        b"Specify command characters with -e\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                }
                                argv = argv.offset(1);
                                ap = *argv;
                            }
                            if ParseEscape(ap) != 0 {
                                Panic(
                                    0 as ::core::ffi::c_int,
                                    b"Two characters are required with -e option, not '%s'.\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    ap,
                                );
                            }
                            ap = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            break 's_819;
                        }
                        102 => {
                            ap = ap.offset(1);
                            's_374: {
                                let c2rust_fresh19 = ap;
                                ap = ap.offset(1);
                                match *c2rust_fresh19 as ::core::ffi::c_int {
                                    110 | 48 => {
                                        nwin_options.flowflag = FLOW_ON;
                                        break 's_374;
                                    }
                                    0 => {
                                        ap = ap.offset(-1);
                                    }
                                    121 | 49 => {}
                                    97 => {
                                        nwin_options.flowflag = FLOW_AUTOFLAG;
                                        break 's_374;
                                    }
                                    _ => {
                                        ap = ap.offset(-1);
                                        exit_with_usage(
                                            myname,
                                            b"Unknown flow option -%s\0".as_ptr()
                                                as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            ap,
                                        );
                                        break 's_374;
                                    }
                                }
                                nwin_options.flowflag = FLOW_OFF;
                            }
                            break 's_819;
                        }
                        104 => {
                            argc -= 1;
                            if argc == 0 as ::core::ffi::c_int {
                                exit_with_usage(
                                    myname,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                            }
                            argv = argv.offset(1);
                            nwin_options.histheight = atoi(*argv);
                            if nwin_options.histheight < 0 as ::core::ffi::c_int {
                                exit_with_usage(
                                    myname,
                                    b"-h: %s: negative scrollback size?\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    *argv,
                                );
                            }
                            break 's_819;
                        }
                        105 => {
                            iflag = r#true != 0;
                            break 's_819;
                        }
                        116 => {
                            argc -= 1;
                            if argc == 0 as ::core::ffi::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify a new window-name with -t\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                            }
                            argv = argv.offset(1);
                            nwin_options.aka = *argv;
                            break 's_819;
                        }
                        108 => {
                            ap = ap.offset(1);
                            's_496: {
                                let c2rust_fresh20 = ap;
                                ap = ap.offset(1);
                                match *c2rust_fresh20 as ::core::ffi::c_int {
                                    110 | 48 => {
                                        nwin_options.lflag = 0 as ::core::ffi::c_int;
                                        break 's_496;
                                    }
                                    0 => {
                                        ap = ap.offset(-1);
                                    }
                                    121 | 49 => {}
                                    97 => {
                                        nwin_options.lflag = 3 as ::core::ffi::c_int;
                                        break 's_496;
                                    }
                                    115 | 105 => {
                                        lsflag = r#true != 0;
                                        if argc > 1 as ::core::ffi::c_int && SocketMatch.is_null() {
                                            argv = argv.offset(1);
                                            SocketMatch = *argv;
                                            argc -= 1;
                                        }
                                        ap = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                        break 's_496;
                                    }
                                    _ => {
                                        ap = ap.offset(-1);
                                        exit_with_usage(
                                            myname,
                                            b"%s: Unknown suboption to -l\0".as_ptr()
                                                as *const ::core::ffi::c_char
                                                as *mut ::core::ffi::c_char,
                                            ap,
                                        );
                                        break 's_496;
                                    }
                                }
                                nwin_options.lflag = 1 as ::core::ffi::c_int;
                            }
                            break 's_819;
                        }
                        119 => {
                            if strcmp(
                                ap.offset(1 as ::core::ffi::c_int as isize),
                                b"ipe\0".as_ptr() as *const ::core::ffi::c_char,
                            ) != 0
                            {
                                ap = ap.offset(-1);
                                exit_with_usage(
                                    myname,
                                    b"Unknown option %s\0".as_ptr() as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    ap,
                                );
                            }
                            lsflag = r#true != 0;
                            wipeflag = r#true != 0;
                            if argc > 1 as ::core::ffi::c_int && SocketMatch.is_null() {
                                argv = argv.offset(1);
                                SocketMatch = *argv;
                                argc -= 1;
                            }
                            break 's_819;
                        }
                        76 => {
                            if strcmp(
                                ap.offset(1 as ::core::ffi::c_int as isize),
                                b"ogfile\0".as_ptr() as *const ::core::ffi::c_char,
                            ) == 0
                            {
                                argc -= 1;
                                if argc == 0 as ::core::ffi::c_int {
                                    exit_with_usage(
                                        myname,
                                        b"Specify logfile path with -Logfile\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                }
                                argv = argv.offset(1);
                                if strlen(*argv) > PATH_MAX as size_t {
                                    Panic(
                                        1 as ::core::ffi::c_int,
                                        b"-Logfile name too long. (max. %d char)\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        PATH_MAX,
                                    );
                                }
                                free(screenlogfile as *mut ::core::ffi::c_void);
                                screenlogfile = SaveStr(*argv);
                                ap = ::core::ptr::null_mut::<::core::ffi::c_char>();
                            } else if strcmp(ap, b"L\0".as_ptr() as *const ::core::ffi::c_char) == 0
                            {
                                nwin_options.Lflag = true;
                            }
                            break 's_819;
                        }
                        109 => {
                            mflag = 1 as ::core::ffi::c_int;
                            break 's_819;
                        }
                        79 => {
                            force_vt = 0 as ::core::ffi::c_int;
                            break 's_819;
                        }
                        84 => {
                            argc -= 1;
                            if argc == 0 as ::core::ffi::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify terminal-type with -T\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                            }
                            argv = argv.offset(1);
                            if strlen(*argv) < MAXTERMLEN as size_t {
                                strncpy(
                                    &raw mut screenterm as *mut ::core::ffi::c_char,
                                    *argv,
                                    MAXTERMLEN as size_t,
                                );
                                *(&raw mut screenterm as *mut ::core::ffi::c_char)
                                    .offset(MAXTERMLEN as isize) = '\0' as ::core::ffi::c_char;
                            } else {
                                Panic(
                                    0 as ::core::ffi::c_int,
                                    b"-T: terminal name too long. (max. %d char)\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    MAXTERMLEN,
                                );
                            }
                            nwin_options.term = &raw mut screenterm as *mut ::core::ffi::c_char;
                            break 's_819;
                        }
                        113 => {
                            quietflag = r#true != 0;
                            break 's_819;
                        }
                        81 => {
                            queryflag = 1 as ::core::ffi::c_int;
                            cmdflag = r#true != 0;
                            break 's_819;
                        }
                        114 | 82 | 120 => {
                            if argc > 1 as ::core::ffi::c_int
                                && **argv.offset(1isize) as ::core::ffi::c_int
                                    != '-' as ::core::ffi::c_int
                                && SocketMatch.is_null()
                            {
                                argv = argv.offset(1);
                                SocketMatch = *argv;
                                argc -= 1;
                            }
                            if *ap as ::core::ffi::c_int == 'x' as ::core::ffi::c_int {
                                xflag = r#true != 0;
                            }
                            if rflag != 0 {
                                rflag = 2 as ::core::ffi::c_int;
                            }
                            rflag += if *ap as ::core::ffi::c_int == 'R' as ::core::ffi::c_int {
                                2 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            };
                            break 's_819;
                        }
                        100 => {
                            dflag = 1 as ::core::ffi::c_int;
                        }
                        68 => {}
                        115 => {
                            argc -= 1;
                            if argc == 0 as ::core::ffi::c_int {
                                exit_with_usage(
                                    myname,
                                    b"Specify shell with -s\0".as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                            }
                            if !ShellProg.is_null() {
                                free(ShellProg as *mut ::core::ffi::c_void);
                            }
                            argv = argv.offset(1);
                            ShellProg = SaveStr(*argv);
                            break 's_819;
                        }
                        83 => {
                            if SocketMatch.is_null() {
                                argc -= 1;
                                if argc == 0 as ::core::ffi::c_int {
                                    exit_with_usage(
                                        myname,
                                        b"Specify session-name with -S\0".as_ptr()
                                            as *const ::core::ffi::c_char
                                            as *mut ::core::ffi::c_char,
                                        ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                    );
                                }
                                argv = argv.offset(1);
                                SocketMatch = *argv;
                            }
                            if *SocketMatch == 0 {
                                exit_with_usage(
                                    myname,
                                    b"Empty session-name?\0".as_ptr() as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                            }
                            if strlen(SocketMatch) > 80 as size_t {
                                exit_with_usage(
                                    myname,
                                    b"Session-name is too long (max length is 80 symbols)\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char
                                        as *mut ::core::ffi::c_char,
                                    ::core::ptr::null_mut::<::core::ffi::c_char>(),
                                );
                            }
                            break 's_819;
                        }
                        88 => {
                            cmdflag = r#true != 0;
                            break 's_819;
                        }
                        118 => {
                            printf(
                                b"Screen version %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut version as *mut ::core::ffi::c_char,
                            );
                            exit(0 as ::core::ffi::c_int);
                        }
                        85 => {
                            nwin_options.encoding = UTF8;
                            break 's_819;
                        }
                        _ => {
                            ap = ap.offset(-1);
                            exit_with_usage(
                                myname,
                                b"Unknown option %s\0".as_ptr() as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                ap,
                            );
                            break 's_819;
                        }
                    }
                    if dflag == 0 {
                        dflag = 2 as ::core::ffi::c_int;
                    }
                    if argc == 2 as ::core::ffi::c_int {
                        if **argv.offset(1isize) as ::core::ffi::c_int != '-' as ::core::ffi::c_int
                            && SocketMatch.is_null()
                        {
                            argv = argv.offset(1);
                            SocketMatch = *argv;
                            argc -= 1;
                        }
                    }
                }
            }
        }
    }
    xsignal(
        SIGSEGV,
        Some(CoreDump as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    if nwin_options.encoding == -1 as ::core::ffi::c_int {
        nwin_options.encoding = FindEncoding(nl_langinfo(C2Rust_Unnamed_4::CODESET.0 as nl_item));
    }
    let mut s: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    s = locale_name();
    if !s.is_null() {
        if strncmp(
            s,
            b"zh_\0".as_ptr() as *const ::core::ffi::c_char,
            3 as size_t,
        ) == 0
            || strncmp(
                s,
                b"ja_\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0
            || strncmp(
                s,
                b"ko_\0".as_ptr() as *const ::core::ffi::c_char,
                3 as size_t,
            ) == 0
        {
            cjkwidth = true;
        }
    }
    if !nwin_options.aka.is_null() {
        if nwin_options.encoding > 0 as ::core::ffi::c_int {
            let mut len: size_t = strlen(nwin_options.aka);
            let mut newsz: size_t = 0;
            let mut newbuf: *mut ::core::ffi::c_char =
                malloc((3 as size_t).wrapping_mul(len)) as *mut ::core::ffi::c_char;
            if newbuf.is_null() {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut strnomem as *mut ::core::ffi::c_char,
                );
            }
            newsz = RecodeBuf(
                nwin_options.aka as *mut ::core::ffi::c_uchar,
                len as ::core::ffi::c_int,
                nwin_options.encoding,
                0 as ::core::ffi::c_int,
                newbuf as *mut ::core::ffi::c_uchar,
            ) as size_t;
            *newbuf.offset(newsz as isize) = '\0' as ::core::ffi::c_char;
            nwin_options.aka = newbuf;
        } else {
            nwin_options.aka = SaveStr(nwin_options.aka);
        }
    }
    if !SocketMatch.is_null() && strlen(SocketMatch) >= MAXSTR as size_t {
        Panic(
            0 as ::core::ffi::c_int,
            b"Ridiculously long socketname - try again.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if cmdflag as ::core::ffi::c_int != 0 && rflag == 0 && dflag == 0 && !xflag {
        xflag = r#true != 0;
    }
    if !cmdflag && dflag != 0 && mflag != 0 && !(rflag != 0 || xflag as ::core::ffi::c_int != 0) {
        detached = r#true != 0;
    }
    nwin = nwin_options;
    nwin.encoding = nwin_undef.encoding;
    if argc != 0 {
        nwin.args = argv;
    }
    if ShellProg.is_null() {
        let mut sh: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        sh = getenv(b"SHELL\0".as_ptr() as *const ::core::ffi::c_char);
        ShellProg = SaveStr(if !sh.is_null() {
            sh
        } else {
            &raw mut DefaultShell as *mut ::core::ffi::c_char
        });
    }
    ShellArgs[0usize] = ShellProg;
    home = getenv(b"HOME\0".as_ptr() as *const ::core::ffi::c_char);
    if mflag == 0 && SocketMatch.is_null() {
        sty = getenv(b"STY\0".as_ptr() as *const ::core::ffi::c_char);
        if !sty.is_null() && *sty as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            sty = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    multi_uid = real_uid;
    own_uid = multi_uid;
    if !SocketMatch.is_null() && {
        sockp = strchr(SocketMatch, '/' as ::core::ffi::c_int);
        !sockp.is_null()
    } {
        *sockp = 0 as ::core::ffi::c_char;
        multi = SocketMatch;
        SocketMatch = sockp.offset(1 as ::core::ffi::c_int as isize);
        if *multi != 0 {
            let mut mppp: *mut passwd = ::core::ptr::null_mut::<passwd>();
            mppp = getpwnam(multi);
            if mppp.is_null() {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"Cannot identify account '%s'.\0".as_ptr() as *const ::core::ffi::c_char,
                    multi,
                );
            }
            multi_uid = (*mppp).pw_uid as uid_t;
            multi_home = SaveStr((*mppp).pw_dir);
            if strlen(multi_home) > (MAXPATHLEN - 10 as ::core::ffi::c_int) as size_t {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"home directory path too long\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if rflag != 0 || lsflag as ::core::ffi::c_int != 0 {
                xflag = true;
            }
            detached = r#false != 0;
            multiattach = 1 as ::core::ffi::c_int;
        }
        if eff_uid != 0 && multi_uid != eff_uid {
            Panic(
                0 as ::core::ffi::c_int,
                b"Must run suid root for multiuser support.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if !SocketMatch.is_null() && *SocketMatch as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        SocketMatch = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    LoginName = getlogin();
    if !LoginName.is_null() {
        ppp = getpwnam(LoginName);
        if !ppp.is_null() {
            if (*ppp).pw_uid != real_uid {
                ppp = ::core::ptr::null_mut::<passwd>();
            }
        }
    }
    if ppp.is_null() {
        ppp = getpwuid(real_uid);
        if ppp.is_null() {
            Panic(
                0 as ::core::ffi::c_int,
                b"getpwuid() can't identify your account!\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        LoginName = (*ppp).pw_name;
    }
    LoginName = SaveStr(LoginName);
    ppp = getpwbyname(LoginName, ppp);
    if !multi.is_null() && multiattach == 0 {
        if !home.is_null() && strcmp(home, (*ppp).pw_dir) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"$HOME must match passwd entry for multiuser screens.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if home.is_null() || *home as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
        home = (*ppp).pw_dir;
    }
    if strlen(LoginName) > MAXLOGINLEN as size_t {
        Panic(
            0 as ::core::ffi::c_int,
            b"LoginName too long - sorry.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !multi.is_null() && strlen(multi) > MAXLOGINLEN as size_t {
        Panic(
            0 as ::core::ffi::c_int,
            b"Screen owner name too long - sorry.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if strlen(home) > MAXPATHLEN as size_t {
        Panic(
            0 as ::core::ffi::c_int,
            b"$HOME too long - sorry.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    attach_tty = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
    if !detached
        && !lsflag
        && !cmdflag
        && !(dflag != 0 && mflag == 0 && rflag == 0 && !xflag)
        && !(!sty.is_null() && SocketMatch.is_null() && mflag == 0 && rflag == 0 && !xflag)
    {
        let mut fl: ::core::ffi::c_int = 0;
        SetTtyname(r#true != 0, &raw mut st);
        fl = fcntl(0 as ::core::ffi::c_int, F_GETFL, 0 as ::core::ffi::c_int);
        if fl != -1 as ::core::ffi::c_int && fl & (O_RDWR | O_RDONLY | O_WRONLY) == O_RDWR {
            attach_fd = 0 as ::core::ffi::c_int;
        }
        if attach_fd == -1 as ::core::ffi::c_int {
            n = secopen(attach_tty, O_RDWR | O_NONBLOCK, 0 as ::core::ffi::c_int);
            if n < 0 as ::core::ffi::c_int {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"Cannot open your terminal '%s' - please check.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    attach_tty,
                );
            }
            if attach_tty_is_in_new_ns {
                attach_fd = n;
            } else {
                close(n);
            }
        }
        attach_term = getenv(b"TERM\0".as_ptr() as *const ::core::ffi::c_char);
        if attach_term.is_null() || *attach_term as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            Panic(
                0 as ::core::ffi::c_int,
                b"Please set a terminal type.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if strlen(attach_term) > MAXTERMLEN as size_t {
            Panic(
                0 as ::core::ffi::c_int,
                b"$TERM too long - sorry.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        GetTTY(0 as ::core::ffi::c_int, &raw mut attach_Mode);
    }
    oumask = umask(0 as __mode_t) as mode_t;
    SocketDir = getenv(b"SCREENDIR\0".as_ptr() as *const ::core::ffi::c_char);
    if !SocketDir.is_null() {
        if strlen(SocketDir) >= (MAXPATHLEN - 1 as ::core::ffi::c_int) as size_t {
            Panic(
                0 as ::core::ffi::c_int,
                b"Ridiculously long $SCREENDIR - try again.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        if !multi.is_null() {
            Panic(
                0 as ::core::ffi::c_int,
                b"No $SCREENDIR with multi screens, please.\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    if multiattach != 0 {
        sprintf(
            &raw mut SocketPath as *mut ::core::ffi::c_char,
            b"%s/.screen\0".as_ptr() as *const ::core::ffi::c_char,
            multi_home,
        );
    } else {
        if SocketDir.is_null() {
            if strlen(home)
                > (MAXPATHLEN - 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as size_t
            {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"$HOME too long - sorry.\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            sprintf(
                &raw mut SocketPath as *mut ::core::ffi::c_char,
                b"%s/.screen\0".as_ptr() as *const ::core::ffi::c_char,
                home,
            );
            SocketDir = &raw mut SocketPath as *mut ::core::ffi::c_char;
        }
        if !SocketDir.is_null() {
            if access(SocketDir, F_OK) != 0 {
                if UserContext() > 0 as ::core::ffi::c_int {
                    if mkdir(SocketDir, 0o700 as __mode_t) != 0 {
                        UserReturn(0 as ::core::ffi::c_int);
                    }
                    UserReturn(1 as ::core::ffi::c_int);
                }
                if UserStatus() <= 0 as ::core::ffi::c_int {
                    Panic(
                        0 as ::core::ffi::c_int,
                        b"Cannot make directory '%s'.\0".as_ptr() as *const ::core::ffi::c_char,
                        SocketDir,
                    );
                }
            }
            if SocketDir != &raw mut SocketPath as *mut ::core::ffi::c_char {
                strncpy(
                    &raw mut SocketPath as *mut ::core::ffi::c_char,
                    SocketDir,
                    ::core::mem::size_of::<[::core::ffi::c_char; 4098]>()
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>()),
                );
            }
        }
    }
    if stat(&raw mut SocketPath as *mut ::core::ffi::c_char, &raw mut st)
        == -1 as ::core::ffi::c_int
    {
        if eff_uid == real_uid {
            Panic(
                *__errno_location(),
                b"Cannot access %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut SocketPath as *mut ::core::ffi::c_char,
            );
        } else {
            Panic(
                0 as ::core::ffi::c_int,
                b"Error accessing %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut SocketPath as *mut ::core::ffi::c_char,
            );
        }
    } else if !(st.st_mode & __S_IFMT as __mode_t == 0o40000 as __mode_t) {
        if eff_uid == real_uid || st.st_uid == real_uid {
            Panic(
                0 as ::core::ffi::c_int,
                b"%s is not a directory.\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut SocketPath as *mut ::core::ffi::c_char,
            );
        } else {
            Panic(
                0 as ::core::ffi::c_int,
                b"Error accessing %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut SocketPath as *mut ::core::ffi::c_char,
            );
        }
    }
    if !multi.is_null() {
        if st.st_uid != multi_uid {
            if eff_uid == real_uid || st.st_uid == real_uid {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"%s is not the owner of %s.\0".as_ptr() as *const ::core::ffi::c_char,
                    multi,
                    &raw mut SocketPath as *mut ::core::ffi::c_char,
                );
            } else {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"Error accessing %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut SocketPath as *mut ::core::ffi::c_char,
                );
            }
        }
    }
    if st.st_mode & 0o777 as __mode_t != 0o700 as __mode_t {
        if eff_uid == real_uid || st.st_uid == real_uid {
            Panic(
                0 as ::core::ffi::c_int,
                b"Directory %s must have mode 700.\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut SocketPath as *mut ::core::ffi::c_char,
            );
        } else {
            Panic(
                0 as ::core::ffi::c_int,
                b"Error accessing %s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut SocketPath as *mut ::core::ffi::c_char,
            );
        }
    }
    if !SocketMatch.is_null() && !strchr(SocketMatch, '/' as ::core::ffi::c_int).is_null() {
        Panic(
            0 as ::core::ffi::c_int,
            b"Bad session name '%s'\0".as_ptr() as *const ::core::ffi::c_char,
            SocketMatch,
        );
    }
    SocketName = (&raw mut SocketPath as *mut ::core::ffi::c_char)
        .offset(strlen(&raw mut SocketPath as *mut ::core::ffi::c_char) as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    *SocketName = 0 as ::core::ffi::c_char;
    umask(oumask);
    gethostname(
        &raw mut HostName as *mut ::core::ffi::c_char,
        MAXSTR as size_t,
    );
    HostName[(MAXSTR - 1 as ::core::ffi::c_int) as usize] = '\0' as ::core::ffi::c_char;
    ap = strchr(
        &raw mut HostName as *mut ::core::ffi::c_char,
        '.' as ::core::ffi::c_int,
    );
    if !ap.is_null() {
        *ap = '\0' as ::core::ffi::c_char;
    }
    if lsflag {
        let mut i: ::core::ffi::c_int = 0;
        let mut fo: ::core::ffi::c_int = 0;
        let mut oth: ::core::ffi::c_int = 0;
        if !multi.is_null() {
            real_uid = multi_uid;
        }
        if setgid(real_gid) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"setgid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if setuid(real_uid) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        eff_uid = real_uid;
        eff_gid = real_gid;
        i = FindSocket(
            NULL_0 as *mut ::core::ffi::c_int,
            &raw mut fo,
            &raw mut oth,
            SocketMatch,
        );
        if quietflag {
            if rflag != 0 {
                exit(10 as ::core::ffi::c_int + i);
            } else {
                exit(
                    9 as ::core::ffi::c_int
                        + if fo != 0 || oth != 0 {
                            1 as ::core::ffi::c_int
                        } else {
                            0 as ::core::ffi::c_int
                        }
                        + fo,
                );
            }
        }
        if fo == 0 as ::core::ffi::c_int {
            if eff_uid == real_uid || st.st_uid == real_uid {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"No Sockets found in %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut SocketPath as *mut ::core::ffi::c_char,
                );
            } else {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"Error accessing %s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut SocketPath as *mut ::core::ffi::c_char,
                );
            }
        }
        Msg(
            0 as ::core::ffi::c_int,
            b"%d Socket%s in %s.\0".as_ptr() as *const ::core::ffi::c_char,
            fo,
            if fo > 1 as ::core::ffi::c_int {
                b"s\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut SocketPath as *mut ::core::ffi::c_char,
        );
        eexit(0 as ::core::ffi::c_int);
    }
    xsignal(
        SIG_BYE,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> !>,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(Some(
            AttacherFinit as unsafe extern "C" fn(::core::ffi::c_int) -> !,
        )),
    );
    if cmdflag {
        if !multi.is_null() {
            real_uid = multi_uid;
        }
        SetTtyname(r#false != 0, &raw mut st);
        if (*argv).is_null() {
            Panic(
                0 as ::core::ffi::c_int,
                b"Please specify a command.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if setgid(real_gid) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"setgid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if setuid(real_uid) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        eff_uid = real_uid;
        eff_gid = real_gid;
        SendCmdMessage(
            sty,
            SocketMatch,
            argv,
            (queryflag >= 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        );
        exit(0 as ::core::ffi::c_int);
    } else if rflag != 0 || xflag as ::core::ffi::c_int != 0 {
        if Attach(MSG_ATTACH) != 0 {
            Attacher();
        }
        if multiattach != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"Can't create sessions of other users.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    } else if dflag != 0 && mflag == 0 {
        SetTtyname(r#false != 0, &raw mut st);
        Attach(MSG_DETACH);
        Msg(
            0 as ::core::ffi::c_int,
            b"[%s %sdetached.]\n\0".as_ptr() as *const ::core::ffi::c_char,
            SocketName,
            if dflag > 1 as ::core::ffi::c_int {
                b"power \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        eexit(0 as ::core::ffi::c_int);
    }
    if SocketMatch.is_null() && mflag == 0 && !sty.is_null() {
        SetTtyname(r#false != 0, &raw mut st);
        if setgid(real_gid) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"setgid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if setuid(real_uid) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        eff_uid = real_uid;
        eff_gid = real_gid;
        nwin_options.args = argv;
        SendCreateMsg(sty, &raw mut nwin);
        exit(0 as ::core::ffi::c_int);
    }
    nwin_compose(
        &raw mut nwin_default,
        &raw mut nwin_options,
        &raw mut nwin_default,
    );
    if !detached || dflag != 2 as ::core::ffi::c_int {
        MasterPid = fork() as pid_t;
    } else {
        MasterPid = 0 as ::core::ffi::c_int as pid_t;
    }
    match MasterPid {
        -1 => {
            Panic(
                *__errno_location(),
                b"fork\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        0 => {}
        _ => {
            if detached {
                exit(0 as ::core::ffi::c_int);
            }
            if !SocketMatch.is_null() {
                sprintf(
                    &raw mut socknamebuf as *mut ::core::ffi::c_char,
                    b"%d.%s\0".as_ptr() as *const ::core::ffi::c_char,
                    MasterPid,
                    SocketMatch,
                );
            } else {
                sprintf(
                    &raw mut socknamebuf as *mut ::core::ffi::c_char,
                    b"%d.%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
                    MasterPid,
                    stripdev(attach_tty),
                    &raw mut HostName as *mut ::core::ffi::c_char,
                );
            }
            ap = &raw mut socknamebuf as *mut ::core::ffi::c_char;
            while *ap != 0 {
                if *ap as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
                    *ap = '-' as ::core::ffi::c_char;
                }
                ap = ap.offset(1);
            }
            if strlen(&raw mut socknamebuf as *mut ::core::ffi::c_char) > FILENAME_MAX as size_t {
                socknamebuf[(FILENAME_MAX - 1 as ::core::ffi::c_int) as usize] =
                    0 as ::core::ffi::c_char;
            }
            snprintf(
                (&raw mut SocketPath as *mut ::core::ffi::c_char)
                    .offset(strlen(&raw mut SocketPath as *mut ::core::ffi::c_char) as isize),
                ::core::mem::size_of::<[::core::ffi::c_char; 4098]>()
                    .wrapping_sub(strlen(&raw mut SocketPath as *mut ::core::ffi::c_char)),
                b"/%s\0".as_ptr() as *const ::core::ffi::c_char,
                &raw mut socknamebuf as *mut ::core::ffi::c_char,
            );
            if setgid(real_gid) != 0 {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"setgid\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            if setuid(real_uid) != 0 {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            eff_uid = real_uid;
            eff_gid = real_gid;
            Attacher();
        }
    }
    if !detached {
        PanicPid = getppid() as pid_t;
    }
    if DefaultEsc == -1 as ::core::ffi::c_int {
        DefaultEsc = 'a' as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int;
    }
    if DefaultMetaEsc == -1 as ::core::ffi::c_int {
        DefaultMetaEsc = 'a' as ::core::ffi::c_int;
    }
    ap = av0
        .offset(strlen(av0) as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    while ap >= av0 {
        if strncmp(
            b"screen\0".as_ptr() as *const ::core::ffi::c_char,
            ap,
            6 as size_t,
        ) == 0
        {
            memcpy(
                ap as *mut ::core::ffi::c_void,
                b"SCREEN\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                6 as size_t,
            );
            break;
        } else {
            ap = ap.offset(-1);
        }
    }
    if ap < av0 {
        *av0 = 'S' as ::core::ffi::c_char;
    }
    if !detached {
        if attach_fd == -1 as ::core::ffi::c_int {
            n = secopen(attach_tty, O_RDWR | O_NONBLOCK, 0 as ::core::ffi::c_int);
            if n < 0 as ::core::ffi::c_int {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"Cannot reopen '%s' - please check.\0".as_ptr() as *const ::core::ffi::c_char,
                    attach_tty,
                );
            }
        } else {
            n = dup(attach_fd);
        }
    } else {
        n = -1 as ::core::ffi::c_int;
    }
    if UserAdd(LoginName, ::core::ptr::null_mut::<*mut acluser>()) < 0 as ::core::ffi::c_int {
        Panic(
            0 as ::core::ffi::c_int,
            b"Could not create user info\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !detached {
        if MakeDisplay(
            LoginName,
            attach_tty,
            attach_term,
            n,
            getppid(),
            &raw mut attach_Mode,
        )
        .is_null()
        {
            Panic(
                0 as ::core::ffi::c_int,
                b"Could not alloc display\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        PanicPid = 0 as ::core::ffi::c_int as pid_t;
        (*display).d_encoding = if nwin_options.encoding > 0 as ::core::ffi::c_int {
            nwin_options.encoding
        } else {
            0 as ::core::ffi::c_int
        };
    }
    if freopen(
        b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
        b"r\0".as_ptr() as *const ::core::ffi::c_char,
        stdin,
    )
    .is_null()
        || freopen(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
            stdout,
        )
        .is_null()
        || freopen(
            b"/dev/null\0".as_ptr() as *const ::core::ffi::c_char,
            b"w\0".as_ptr() as *const ::core::ffi::c_char,
            stderr,
        )
        .is_null()
    {
        Panic(
            0 as ::core::ffi::c_int,
            b"Cannot reassociate std streams\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !SocketMatch.is_null() {
        sprintf(
            &raw mut socknamebuf as *mut ::core::ffi::c_char,
            b"%d.%s\0".as_ptr() as *const ::core::ffi::c_char,
            getpid(),
            SocketMatch,
        );
    } else {
        sprintf(
            &raw mut socknamebuf as *mut ::core::ffi::c_char,
            b"%d.%s.%s\0".as_ptr() as *const ::core::ffi::c_char,
            getpid(),
            stripdev(attach_tty),
            &raw mut HostName as *mut ::core::ffi::c_char,
        );
    }
    ap = &raw mut socknamebuf as *mut ::core::ffi::c_char;
    while *ap != 0 {
        if *ap as ::core::ffi::c_int == '/' as ::core::ffi::c_int {
            *ap = '-' as ::core::ffi::c_char;
        }
        ap = ap.offset(1);
    }
    if strlen(&raw mut socknamebuf as *mut ::core::ffi::c_char) > FILENAME_MAX as size_t {
        socknamebuf[FILENAME_MAX as usize] = 0 as ::core::ffi::c_char;
    }
    snprintf(
        (&raw mut SocketPath as *mut ::core::ffi::c_char)
            .offset(strlen(&raw mut SocketPath as *mut ::core::ffi::c_char) as isize),
        ::core::mem::size_of::<[::core::ffi::c_char; 4098]>()
            .wrapping_sub(strlen(&raw mut SocketPath as *mut ::core::ffi::c_char)),
        b"/%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut socknamebuf as *mut ::core::ffi::c_char,
    );
    ServerSocket = MakeServerSocket();
    StartRc(
        SYSTEM_SCREENRC.as_ptr() as *mut ::core::ffi::c_char,
        0 as ::core::ffi::c_int,
    );
    StartRc(RcFileName, 0 as ::core::ffi::c_int);
    if !display.is_null() {
        if InitTermcap(0 as ::core::ffi::c_int, 0 as ::core::ffi::c_int) != 0 {
            fcntl((*display).d_userfd, F_SETFL, 0 as ::core::ffi::c_int);
            freetty();
            if (*display).d_userpid != 0 {
                Kill((*display).d_userpid, SIG_BYE);
            }
            eexit(1 as ::core::ffi::c_int);
        }
        MakeDefaultCanvas();
        InitTerm(0 as ::core::ffi::c_int);
    } else {
        MakeTermcap(true);
    }
    InitKeytab();
    MakeNewEnv();
    xsignal(
        SIGHUP,
        Some(SigHup as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    xsignal(
        SIGINT,
        Some(FinitHandler as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    xsignal(
        SIGQUIT,
        Some(FinitHandler as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    xsignal(
        SIGTERM,
        Some(FinitHandler as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    xsignal(
        SIGTTIN,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    xsignal(
        SIGTTOU,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    if !display.is_null() {
        brktty((*display).d_userfd);
        SetMode(
            &raw mut (*display).d_OldMode,
            &raw mut (*display).d_NewMode,
            (*display).d_flow,
            iflag as ::core::ffi::c_int,
        );
        SetTTY((*display).d_userfd, &raw mut (*display).d_NewMode);
        if fcntl((*display).d_userfd, F_SETFL, FNBLOCK) != 0 {
            Msg(
                *__errno_location(),
                b"Warning: NBLOCK fcntl failed\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    } else {
        brktty(-1 as ::core::ffi::c_int);
    }
    xsignal(
        SIGCHLD,
        Some(SigChld as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    FinishRc(SYSTEM_SCREENRC.as_ptr() as *mut ::core::ffi::c_char);
    FinishRc(RcFileName);
    if mru_window.is_null() {
        if MakeWindow(&raw mut nwin) == -1 as ::core::ffi::c_int {
            let mut pfd: [pollfd; 1] = [pollfd {
                fd: 0,
                events: 0,
                revents: 0,
            }; 1];
            pfd[0usize].fd = 0 as ::core::ffi::c_int;
            pfd[0usize].events = POLLIN as ::core::ffi::c_short;
            Msg(
                0 as ::core::ffi::c_int,
                b"Sorry, could not find a PTY or TTY.\0".as_ptr() as *const ::core::ffi::c_char,
            );
            poll(
                &raw mut pfd as *mut pollfd,
                (::core::mem::size_of::<[pollfd; 1]>() as nfds_t)
                    .wrapping_div(::core::mem::size_of::<pollfd>() as nfds_t),
                MsgWait,
            );
            Finit(0 as ::core::ffi::c_int);
        }
    } else if argc != 0 {
        MakeWindow(&raw mut nwin);
    }
    if !display.is_null() && default_startup as ::core::ffi::c_int != 0 {
        display_license();
    }
    xsignal(
        SIGINT,
        Some(SigInt as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    if rflag != 0 && rflag & 1 as ::core::ffi::c_int == 0 as ::core::ffi::c_int && !quietflag {
        Msg(
            0 as ::core::ffi::c_int,
            b"New screen...\0".as_ptr() as *const ::core::ffi::c_char,
        );
        rflag = 0 as ::core::ffi::c_int;
    }
    serv_read.r#type = EventType::EV_READ;
    serv_read.fd = ServerSocket;
    serv_read.handler =
        Some(serv_read_fn as unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ()>;
    evenq(&raw mut serv_read);
    serv_select.priority = -10 as ::core::ffi::c_int;
    serv_select.r#type = EventType::EV_ALWAYS;
    serv_select.handler =
        Some(serv_select_fn as unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ()>;
    evenq(&raw mut serv_select);
    logflushev.r#type = EventType::EV_TIMEOUT;
    logflushev.handler =
        Some(logflush_fn as unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ())
            as Option<unsafe extern "C" fn(*mut Event, *mut ::core::ffi::c_void) -> ()>;
    sched();
}
unsafe extern "C" fn SigChldHandler() {
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
    while GotSigChld != 0 {
        GotSigChld = 0 as ::core::ffi::c_int;
        DoWait();
    }
    if stat(&raw mut SocketPath as *mut ::core::ffi::c_char, &raw mut st)
        == -1 as ::core::ffi::c_int
    {
        if RecoverSocket() == 0 {
            Finit(1 as ::core::ffi::c_int);
        }
    }
}
unsafe extern "C" fn SigChld(mut sigsig: ::core::ffi::c_int) {
    GotSigChld = 1 as ::core::ffi::c_int;
}
#[export_name = "rboxc_screen_SigHup"]
pub unsafe extern "C" fn SigHup(mut sigsig: ::core::ffi::c_int) {
    loop {
        display = displays;
        if display.is_null() {
            break;
        }
        Hangup();
    }
}
unsafe extern "C" fn SigInt(mut sigsig: ::core::ffi::c_int) {
    xsignal(
        SIGINT,
        Some(SigInt as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    InterruptPlease = 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn CoreDump(mut sigsig: ::core::ffi::c_int) {
    let mut disp: *mut Display = ::core::ptr::null_mut::<Display>();
    let mut buf: [::core::ffi::c_char; 80] = [0; 80];
    if setgid(getgid()) != 0 {
        Panic(
            0 as ::core::ffi::c_int,
            b"setgid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if setuid(getuid()) != 0 {
        Panic(
            0 as ::core::ffi::c_int,
            b"setuid\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    unlink(b"core\0".as_ptr() as *const ::core::ffi::c_char);
    sprintf(
        &raw mut buf as *mut ::core::ffi::c_char,
        b"\r\n[screen caught a fatal signal. (core dumped)]\r\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    disp = displays;
    while !disp.is_null() {
        if !((*disp).d_nonblock < -1 as ::core::ffi::c_int
            || (*disp).d_nonblock > 1000000 as ::core::ffi::c_int)
        {
            fcntl((*disp).d_userfd, F_SETFL, 0 as ::core::ffi::c_int);
            SetTTY((*disp).d_userfd, &raw mut (*display).d_OldMode);
            write(
                (*disp).d_userfd,
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut buf as *mut ::core::ffi::c_char),
            );
            Kill((*disp).d_userpid, SIG_BYE);
        }
        disp = (*disp).d_next;
    }
    abort();
}
unsafe extern "C" fn DoWait() {
    let mut pid: pid_t = 0;
    let mut win: *mut Window = ::core::ptr::null_mut::<Window>();
    let mut wstat: ::core::ffi::c_int = 0;
    loop {
        pid = waitpid(-1 as __pid_t, &raw mut wstat, WNOHANG | WUNTRACED) as pid_t;
        if pid <= 0 as ::core::ffi::c_int {
            break;
        }
        win = mru_window;
        while !win.is_null() {
            if (*win).w_pid != 0 && pid == (*win).w_pid
                || (*win).w_deadpid != 0 && pid == (*win).w_deadpid
            {
                (*win).w_pid = 0 as ::core::ffi::c_int as pid_t;
                if wstat & 0xff as ::core::ffi::c_int == 0x7f as ::core::ffi::c_int {
                    if (wstat & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int == SIGTTIN
                    {
                        Msg(
                            0 as ::core::ffi::c_int,
                            b"Suspended (tty input)\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    } else if (wstat & 0xff00 as ::core::ffi::c_int) >> 8 as ::core::ffi::c_int
                        == SIGTTOU
                    {
                        Msg(
                            0 as ::core::ffi::c_int,
                            b"Suspended (tty output)\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    } else {
                        Msg(
                            0 as ::core::ffi::c_int,
                            b"Child has been stopped, restarting.\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        if killpg(pid, SIGCONT) != 0 {
                            kill(pid, SIGCONT);
                        }
                        break;
                    }
                } else {
                    (*win).w_destroyev.data =
                        win as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
                    (*win).w_exitstatus = wstat;
                    SetTimeout(
                        &raw mut (*win).w_destroyev,
                        10 as ::core::ffi::c_int * 1000 as ::core::ffi::c_int,
                    );
                    evenq(&raw mut (*win).w_destroyev);
                    break;
                }
            } else if !(*win).w_pwin.is_null() && pid == (*(*win).w_pwin).p_pid {
                FreePseudowin(win);
                break;
            }
            win = (*win).w_prev_mru;
        }
    }
}
unsafe extern "C" fn FinitHandler(mut sigsig: ::core::ffi::c_int) {
    Finit(1 as ::core::ffi::c_int);
}
#[export_name = "rboxc_screen_Finit"]
pub unsafe extern "C" fn Finit(mut i: ::core::ffi::c_int) -> ! {
    xsignal(SIGCHLD, SIG_DFL);
    xsignal(
        SIGHUP,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    while !mru_window.is_null() {
        let mut p: *mut Window = mru_window;
        mru_window = (*mru_window).w_prev_mru;
        FreeWindow(p);
    }
    if ServerSocket != -1 as ::core::ffi::c_int {
        xseteuid(real_uid as ::core::ffi::c_int);
        xsetegid(real_gid as ::core::ffi::c_int);
        unlink(&raw mut SocketPath as *mut ::core::ffi::c_char);
        xseteuid(eff_uid as ::core::ffi::c_int);
        xsetegid(eff_gid as ::core::ffi::c_int);
    }
    display = displays;
    while !display.is_null() {
        if (*display).d_status.0 != 0 {
            RemoveStatus();
        }
        FinitTerm();
        AddStr(
            b"[screen is terminating]\r\n\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char,
        );
        Flush(3 as ::core::ffi::c_int);
        SetTTY((*display).d_userfd, &raw mut (*display).d_OldMode);
        fcntl((*display).d_userfd, F_SETFL, 0 as ::core::ffi::c_int);
        freetty();
        Kill((*display).d_userpid, SIG_BYE);
        display = (*display).d_next;
    }
    exit(i);
}
#[export_name = "rboxc_screen_eexit"]
pub unsafe extern "C" fn eexit(mut e: ::core::ffi::c_int) -> ! {
    if ServerSocket != -1 as ::core::ffi::c_int {
        if setgid(real_gid) != 0 {
            AddStr(
                b"Failed to set gid\r\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        if setuid(real_uid) != 0 {
            AddStr(
                b"Failed to set uid\r\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
        if unlink(&raw mut SocketPath as *mut ::core::ffi::c_char) != 0 {
            AddStr(
                b"Failed to remove socket\r\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char,
            );
        }
    }
    exit(e);
}
#[export_name = "rboxc_screen_Hangup"]
pub unsafe extern "C" fn Hangup() {
    if display.is_null() {
        return;
    }
    if (*display).d_userfd >= 0 as ::core::ffi::c_int {
        close((*display).d_userfd);
        (*display).d_userfd = -1 as ::core::ffi::c_int;
    }
    if auto_detach as ::core::ffi::c_int != 0 || !(*displays).d_next.is_null() {
        Detach(D_HANGUP);
    } else {
        Finit(0 as ::core::ffi::c_int);
    };
}
#[export_name = "rboxc_screen_Detach"]
pub unsafe extern "C" fn Detach(mut mode: ::core::ffi::c_int) {
    let mut sign: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pid: pid_t = 0;
    let mut cv: *mut Canvas = ::core::ptr::null_mut::<Canvas>();
    let mut p: *mut Window = ::core::ptr::null_mut::<Window>();
    if display.is_null() {
        return;
    }
    xsignal(
        SIGHUP,
        ::core::mem::transmute::<
            ::libc::intptr_t,
            Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>,
        >(1 as ::core::ffi::c_int as ::libc::intptr_t),
    );
    if (*display).d_status.0 != 0 {
        RemoveStatus();
    }
    FinitTerm();
    if display.is_null() {
        return;
    }
    match mode {
        D_HANGUP => {
            sign = SIG_BYE;
        }
        D_DETACH => {
            if !SocketName.is_null() {
                AddStr(b"[detached from \0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char);
                AddStr(SocketName);
                AddStr(
                    b"]\r\n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
                );
            } else {
                AddStr(b"[detached]\r\n\0".as_ptr() as *const ::core::ffi::c_char
                    as *mut ::core::ffi::c_char);
            }
            sign = SIG_BYE;
        }
        D_STOP => {
            sign = SIG_STOP;
        }
        D_REMOTE => {
            if !SocketName.is_null() {
                AddStr(
                    b"[remote detached from \0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                AddStr(SocketName);
                AddStr(
                    b"]\r\n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
                );
            } else {
                AddStr(
                    b"[remote detached]\r\n\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            sign = SIG_BYE;
        }
        D_POWER => {
            if !SocketName.is_null() {
                AddStr(
                    b"[power detached from \0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                AddStr(SocketName);
                AddStr(
                    b"]\r\n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
                );
            } else {
                AddStr(
                    b"[power detached]\r\n\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if !PowDetachString.is_null() {
                AddStr(PowDetachString);
                AddStr(b"\r\n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
            }
            sign = SIG_POWER_BYE;
        }
        D_REMOTE_POWER => {
            if !SocketName.is_null() {
                AddStr(
                    b"[remote power detached from \0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
                AddStr(SocketName);
                AddStr(
                    b"]\r\n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char
                );
            } else {
                AddStr(
                    b"[remote power detached]\r\n\0".as_ptr() as *const ::core::ffi::c_char
                        as *mut ::core::ffi::c_char,
                );
            }
            if !PowDetachString.is_null() {
                AddStr(PowDetachString);
                AddStr(b"\r\n\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char);
            }
            sign = SIG_POWER_BYE;
        }
        D_LOCK => {
            ClearAll();
            ClearScrollbackBuffer();
            sign = SIG_LOCK;
        }
        _ => {}
    }
    if (*displays).d_next.is_null() && !console_window.is_null() {
        if TtyGrabConsole(
            (*console_window).w_ptyfd,
            r#false != 0,
            b"detach\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) != 0
        {
            KillWindow(console_window);
            display = displays;
        }
    }
    if !(*display).d_fore.is_null() {
        ReleaseAutoWritelock(display, (*display).d_fore);
        (*(*display).d_user).u_detachwin = (*(*display).d_fore).w_number as ::core::ffi::c_int;
        (*(*display).d_user).u_detachotherwin = if !(*display).d_other.is_null() {
            (*(*display).d_other).w_number as ::core::ffi::c_int
        } else {
            (*(*display).d_fore).w_number as ::core::ffi::c_int
        };
    }
    AutosaveLayout((*display).d_layout);
    layout_last = (*display).d_layout;
    cv = (*display).d_cvlist;
    while !cv.is_null() {
        p = (*(*(*cv).c_layer).l_bottom).l_data as *mut Window;
        SetCanvasWindow(cv, ::core::ptr::null_mut::<Window>());
        if !p.is_null() {
            WindowChanged(
                p,
                WinMsgEscapeChar('u' as ::core::ffi::c_int as ::core::ffi::c_uint),
            );
        }
        cv = (*cv).c_next;
    }
    pid = (*display).d_userpid;
    FreeDisplay();
    if displays.is_null() {
        chsock();
    }
    Kill(pid, sign);
    xsignal(
        SIGHUP,
        Some(SigHup as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
unsafe extern "C" fn IsSymbol(
    mut e: *mut ::core::ffi::c_char,
    mut s: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0;
    l = strlen(s) as ::core::ffi::c_int;
    return (strncmp(e, s, l as size_t) == 0 as ::core::ffi::c_int
        && *e.offset(l as isize) as ::core::ffi::c_int == '=' as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[export_name = "rboxc_screen_MakeNewEnv"]
pub unsafe extern "C" fn MakeNewEnv() {
    let mut op: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    let mut np: *mut *mut ::core::ffi::c_char = ::core::ptr::null_mut::<*mut ::core::ffi::c_char>();
    static mut stybuf: [::core::ffi::c_char; 768] = [0; 768];
    op = environ;
    while !(*op).is_null() {
        op = op.offset(1);
    }
    if !NewEnv.is_null() {
        free(NewEnv as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void);
    }
    np = malloc(
        ((op.offset_from(environ) + 7isize + 1isize) as ::core::ffi::c_uint as size_t)
            .wrapping_mul(::core::mem::size_of::<*mut ::core::ffi::c_char>()),
    ) as *mut *mut ::core::ffi::c_char;
    NewEnv = np;
    if NewEnv.is_null() {
        Panic(
            0 as ::core::ffi::c_int,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut strnomem as *mut ::core::ffi::c_char,
        );
    }
    sprintf(
        &raw mut stybuf as *mut ::core::ffi::c_char,
        b"STY=%s\0".as_ptr() as *const ::core::ffi::c_char,
        if strlen(SocketName) <= (MAXSTR - 5 as ::core::ffi::c_int) as size_t {
            SocketName as *const ::core::ffi::c_char
        } else {
            b"?\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    let c2rust_fresh6 = np;
    np = np.offset(1);
    *c2rust_fresh6 = &raw mut stybuf as *mut ::core::ffi::c_char;
    let c2rust_fresh7 = np;
    np = np.offset(1);
    *c2rust_fresh7 = &raw mut Term as *mut ::core::ffi::c_char;
    np = np.offset(1);
    np = np.offset(2 as ::core::ffi::c_int as isize);
    op = environ;
    while !(*op).is_null() {
        if IsSymbol(
            *op,
            b"TERM\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
        ) == 0
            && IsSymbol(
                *op,
                b"TERMCAP\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"STY\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"WINDOW\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"SCREENCAP\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"SHELL\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"LINES\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
            && IsSymbol(
                *op,
                b"COLUMNS\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char,
            ) == 0
        {
            let c2rust_fresh8 = np;
            np = np.offset(1);
            *c2rust_fresh8 = *op;
        }
        op = op.offset(1);
    }
    *np = ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[export_name = "rboxc_screen_Msg"]
pub unsafe extern "C" fn Msg(
    mut err: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut buf: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    let mut ap: ::core::ffi::VaList;
    ap = c2rust_args.clone();
    vsnprintf(
        p,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(100 as size_t),
        fmt,
        ap,
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let c2rust_fresh0 = p;
        p = p.offset(1);
        *c2rust_fresh0 = ':' as ::core::ffi::c_char;
        let c2rust_fresh1 = p;
        p = p.offset(1);
        *c2rust_fresh1 = ' ' as ::core::ffi::c_char;
        strncpy(
            p,
            strerror(err),
            ((&raw mut buf as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as isize)
                .offset_from(p)
                - 1isize) as size_t,
        );
        buf[::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(1usize)] =
            0 as ::core::ffi::c_char;
    }
    if !display.is_null() && !displays.is_null() {
        MakeStatus(&raw mut buf as *mut ::core::ffi::c_char);
    } else if !displays.is_null() {
        display = displays;
        while !display.is_null() {
            MakeStatus(&raw mut buf as *mut ::core::ffi::c_char);
            display = (*display).d_next;
        }
    } else if !display.is_null() {
        let mut tty: *mut ::core::ffi::c_char =
            &raw mut (*display).d_usertty as *mut ::core::ffi::c_char;
        let mut olddisplay: *mut Display = display;
        display = ::core::ptr::null_mut::<Display>();
        SendErrorMsg(tty, &raw mut buf as *mut ::core::ffi::c_char);
        display = olddisplay;
    } else {
        printf(
            b"%s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
    }
    if queryflag >= 0 as ::core::ffi::c_int {
        write(
            queryflag,
            &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(&raw mut buf as *mut ::core::ffi::c_char),
        );
    }
}
#[export_name = "rboxc_screen_Panic"]
pub unsafe extern "C" fn Panic(
    mut err: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ! {
    let mut buf: [::core::ffi::c_char; 8192] = [0; 8192];
    let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    let mut ap: ::core::ffi::VaList;
    ap = c2rust_args.clone();
    vsnprintf(
        p,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(100 as size_t),
        fmt,
        ap,
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let c2rust_fresh2 = p;
        p = p.offset(1);
        *c2rust_fresh2 = ':' as ::core::ffi::c_char;
        let c2rust_fresh3 = p;
        p = p.offset(1);
        *c2rust_fresh3 = ' ' as ::core::ffi::c_char;
        strncpy(
            p,
            strerror(err),
            ((&raw mut buf as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as isize)
                .offset_from(p)
                - 1isize) as size_t,
        );
        buf[::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(1usize)] =
            0 as ::core::ffi::c_char;
    }
    if displays.is_null() && display.is_null() {
        printf(
            b"%s\r\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut buf as *mut ::core::ffi::c_char,
        );
        if PanicPid != 0 {
            Kill(PanicPid, SIG_BYE);
        }
    } else if displays.is_null() {
        let mut tty: *mut ::core::ffi::c_char =
            &raw mut (*display).d_usertty as *mut ::core::ffi::c_char;
        display = ::core::ptr::null_mut::<Display>();
        SendErrorMsg(tty, &raw mut buf as *mut ::core::ffi::c_char);
        sleep(2 as ::core::ffi::c_uint);
        _exit(1 as ::core::ffi::c_int);
    } else {
        display = displays;
        while !display.is_null() {
            if (*display).d_status.0 != 0 {
                RemoveStatus();
            }
            FinitTerm();
            Flush(3 as ::core::ffi::c_int);
            SetTTY((*display).d_userfd, &raw mut (*display).d_OldMode);
            fcntl((*display).d_userfd, F_SETFL, 0 as ::core::ffi::c_int);
            write(
                (*display).d_userfd,
                &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                strlen(&raw mut buf as *mut ::core::ffi::c_char),
            );
            write(
                (*display).d_userfd,
                b"\n\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                1 as size_t,
            );
            freetty();
            if (*display).d_userpid != 0 {
                Kill((*display).d_userpid, SIG_BYE);
            }
            display = (*display).d_next;
        }
    }
    eexit(1 as ::core::ffi::c_int);
}
#[export_name = "rboxc_screen_QueryMsg"]
pub unsafe extern "C" fn QueryMsg(
    mut err: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut buf: [::core::ffi::c_char; 8192] = [0; 8192];
    if queryflag < 0 as ::core::ffi::c_int {
        return;
    }
    let mut p: *mut ::core::ffi::c_char = &raw mut buf as *mut ::core::ffi::c_char;
    let mut ap: ::core::ffi::VaList;
    ap = c2rust_args.clone();
    vsnprintf(
        p,
        ::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(100 as size_t),
        fmt,
        ap,
    );
    if err != 0 {
        p = p.offset(strlen(p) as isize);
        let c2rust_fresh4 = p;
        p = p.offset(1);
        *c2rust_fresh4 = ':' as ::core::ffi::c_char;
        let c2rust_fresh5 = p;
        p = p.offset(1);
        *c2rust_fresh5 = ' ' as ::core::ffi::c_char;
        strncpy(
            p,
            strerror(err),
            ((&raw mut buf as *mut ::core::ffi::c_char)
                .offset(::core::mem::size_of::<[::core::ffi::c_char; 8192]>() as isize)
                .offset_from(p)
                - 1isize) as size_t,
        );
        buf[::core::mem::size_of::<[::core::ffi::c_char; 8192]>().wrapping_sub(1usize)] =
            0 as ::core::ffi::c_char;
    }
    write(
        queryflag,
        &raw mut buf as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(&raw mut buf as *mut ::core::ffi::c_char),
    );
}
#[export_name = "rboxc_screen_Dummy"]
pub unsafe extern "C" fn Dummy(
    mut err: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
}
#[export_name = "rboxc_screen_PutWinMsg"]
pub unsafe extern "C" fn PutWinMsg(
    mut s: *mut ::core::ffi::c_char,
    mut start: ::core::ffi::c_int,
    mut max: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    let mut p: ::core::ffi::c_int = 0;
    let mut l: ::core::ffi::c_int = 0;
    let mut n: ::core::ffi::c_int = 0;
    let mut r: uint64_t = 0;
    let mut rend: mchar = mchar {
        image: 0,
        attr: 0,
        font: 0,
        colorbg: 0,
        colorfg: 0,
        mbcs: 0,
    };
    let mut rendstack: [mchar; 256] = [mchar {
        image: 0,
        attr: 0,
        font: 0,
        colorbg: 0,
        colorfg: 0,
        mbcs: 0,
    }; 256];
    let mut rendstackn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if s != (*g_winmsg).buf {
        l = strlen(s) as ::core::ffi::c_int;
        if l > max {
            l = max;
        }
        l -= start;
        s = s.offset(start as isize);
        loop {
            let c2rust_fresh9 = l;
            l -= 1;
            if c2rust_fresh9 <= 0 as ::core::ffi::c_int {
                break;
            }
            let c2rust_fresh10 = s;
            s = s.offset(1);
            PUTCHARLP(*c2rust_fresh10 as uint32_t);
        }
        return;
    }
    rend = (*display).d_rend;
    p = 0 as ::core::ffi::c_int;
    l = strlen(s) as ::core::ffi::c_int;
    i = 0 as ::core::ffi::c_int;
    while i < (*g_winmsg).numrend && max > 0 as ::core::ffi::c_int {
        if p > (*g_winmsg).rendpos[i as usize] || (*g_winmsg).rendpos[i as usize] > l {
            break;
        }
        if p < (*g_winmsg).rendpos[i as usize] {
            n = (*g_winmsg).rendpos[i as usize] - p;
            if n > max {
                n = max;
            }
            max -= n;
            p += n;
            loop {
                let c2rust_fresh11 = n;
                n -= 1;
                if c2rust_fresh11 <= 0 as ::core::ffi::c_int {
                    break;
                }
                let c2rust_fresh12 = start;
                start -= 1;
                if c2rust_fresh12 > 0 as ::core::ffi::c_int {
                    s = s.offset(1);
                } else {
                    let c2rust_fresh13 = s;
                    s = s.offset(1);
                    PUTCHARLP(*c2rust_fresh13 as uint32_t);
                }
            }
        }
        r = (*g_winmsg).rend[i as usize];
        if r == 0 as uint64_t {
            if rendstackn > 0 as ::core::ffi::c_int {
                rendstackn -= 1;
                rend = rendstack[rendstackn as usize];
            }
        } else {
            let c2rust_fresh14 = rendstackn;
            rendstackn += 1;
            rendstack[c2rust_fresh14 as usize] = rend;
            ApplyAttrColor(r, &raw mut rend);
        }
        SetRendition(&raw mut rend);
        i += 1;
    }
    if p < l {
        n = l - p;
        if n > max {
            n = max;
        }
        loop {
            let c2rust_fresh15 = n;
            n -= 1;
            if c2rust_fresh15 <= 0 as ::core::ffi::c_int {
                break;
            }
            let c2rust_fresh16 = start;
            start -= 1;
            if c2rust_fresh16 > 0 as ::core::ffi::c_int {
                s = s.offset(1);
            } else {
                let c2rust_fresh17 = s;
                s = s.offset(1);
                PUTCHARLP(*c2rust_fresh17 as uint32_t);
            }
        }
    }
}
unsafe extern "C" fn serv_read_fn(mut event: *mut Event, mut data: *mut ::core::ffi::c_void) {
    ReceiveMsg();
}
unsafe extern "C" fn serv_select_fn(mut event: *mut Event, mut data: *mut ::core::ffi::c_void) {
    let mut p: *mut Window = ::core::ptr::null_mut::<Window>();
    if GotSigChld != 0 {
        SigChldHandler();
    }
    if InterruptPlease != 0 {
        if !fore.is_null() && !displays.is_null() {
            let mut ibuf: ::core::ffi::c_char =
                (*displays).d_OldMode.tio.c_cc[VINTR as usize] as ::core::ffi::c_char;
            write(
                if !(*fore).w_pwin.is_null() && (*(*fore).w_pwin).p_fdpat & F_UWP != 0 {
                    (*(*fore).w_pwin).p_ptyfd
                } else {
                    (*fore).w_ptyfd
                },
                &raw mut ibuf as *const ::core::ffi::c_void,
                1 as size_t,
            );
        }
        InterruptPlease = 0 as ::core::ffi::c_int;
    }
    p = mru_window;
    while !p.is_null() {
        if (*p).w_bell == BELL_FOUND || (*p).w_bell == BELL_VISUAL {
            let mut cv: *mut Canvas = ::core::ptr::null_mut::<Canvas>();
            let mut visual: ::core::ffi::c_int = ((*p).w_bell == BELL_VISUAL
                || visual_bell as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int;
            (*p).w_bell = BELL_ON;
            display = displays;
            while !display.is_null() {
                cv = (*display).d_cvlist;
                while !cv.is_null() {
                    if (*(*cv).c_layer).l_bottom == &raw mut (*p).w_layer {
                        break;
                    }
                    cv = (*cv).c_next;
                }
                if cv.is_null() {
                    (*p).w_bell = BELL_DONE;
                    Msg(
                        0 as ::core::ffi::c_int,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        MakeWinMsg(BellString, p, '%' as ::core::ffi::c_int),
                    );
                } else if visual != 0
                    && (*display).d_tcs[44usize].str.is_null()
                    && ((*display).d_status.0 == 0 || (*display).d_status_bell == 0)
                {
                    Msg(
                        0 as ::core::ffi::c_int,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        VisualBellString,
                    );
                    if (*display).d_status.0 != 0 {
                        (*display).d_status_bell = 1 as ::core::ffi::c_char;
                        SetTimeout(&raw mut (*display).d_statusev, VBellWait);
                    }
                }
                display = (*display).d_next;
            }
            if (*p).w_monitor == MON_FOUND {
                (*p).w_monitor = MON_DONE;
            }
            WindowChanged(p, WinMsgEscapeChar::WINESC_WFLAGS);
        }
        if (*p).w_monitor == MON_FOUND {
            let mut cv_0: *mut Canvas = ::core::ptr::null_mut::<Canvas>();
            (*p).w_monitor = MON_ON;
            display = displays;
            while !display.is_null() {
                cv_0 = (*display).d_cvlist;
                while !cv_0.is_null() {
                    if (*(*cv_0).c_layer).l_bottom == &raw mut (*p).w_layer {
                        break;
                    }
                    cv_0 = (*cv_0).c_next;
                }
                if cv_0.is_null() {
                    if *(*p)
                        .w_mon_notify
                        .offset(((*(*display).d_user).u_id >> 3 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int
                        & 0x80 as ::core::ffi::c_int
                            >> ((*(*display).d_user).u_id & 7 as ::core::ffi::c_int)
                        != 0
                    {
                        Msg(
                            0 as ::core::ffi::c_int,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            MakeWinMsg(ActivityString, p, '%' as ::core::ffi::c_int),
                        );
                        (*p).w_monitor = MON_DONE;
                    }
                }
                display = (*display).d_next;
            }
            WindowChanged(p, WinMsgEscapeChar::WINESC_WFLAGS);
        }
        if (*p).w_silence == SILENCE_FOUND {
            if !(*p).w_layer.l_cvlist.is_null() {
                (*p).w_silence = SILENCE_ON;
                WindowChanged(p, WinMsgEscapeChar::WINESC_WFLAGS);
            }
        }
        p = (*p).w_prev_mru;
    }
    display = displays;
    while !display.is_null() {
        let mut cv_1: *mut Canvas = ::core::ptr::null_mut::<Canvas>();
        if (*display).d_status.0 != DisplayStatus::STATUS_ON_WIN.0 {
            cv_1 = (*display).d_cvlist;
            while !cv_1.is_null() {
                let mut lx: ::core::ffi::c_int = 0;
                let mut ly: ::core::ffi::c_int = 0;
                lx = (*(*cv_1).c_layer).l_x;
                ly = (*(*cv_1).c_layer).l_y;
                if lx == (*(*cv_1).c_layer).l_width {
                    lx -= 1;
                }
                if ly + (*cv_1).c_yoff < (*cv_1).c_ys {
                    let mut i: ::core::ffi::c_int = 0;
                    let mut n: ::core::ffi::c_int = (*cv_1).c_ys - (ly + (*cv_1).c_yoff);
                    (*cv_1).c_yoff = (*cv_1).c_ys - ly;
                    RethinkViewportOffsets(cv_1);
                    if n > (*(*cv_1).c_layer).l_height {
                        n = (*(*cv_1).c_layer).l_height;
                    }
                    let mut olddisplay: *mut Display = display;
                    let mut oldflayer: *mut Layer = flayer;
                    let mut l: *mut Layer = (*cv_1).c_layer;
                    let mut cvlist: *mut Canvas = (*l).l_cvlist;
                    let mut cvlnext: *mut Canvas = (*cv_1).c_lnext;
                    flayer = l;
                    (*l).l_cvlist = cv_1 as *mut Canvas;
                    (*cv_1).c_lnext = ::core::ptr::null_mut::<Canvas>();
                    LScrollV(
                        flayer,
                        -n,
                        0 as ::core::ffi::c_int,
                        (*flayer).l_height - 1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    Some(
                        (*(*flayer).l_layfn)
                            .lf_LayRedisplayLine
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    i = 0 as ::core::ffi::c_int;
                    while i < n {
                        Some(
                            (*(*flayer).l_layfn)
                                .lf_LayRedisplayLine
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            i,
                            0 as ::core::ffi::c_int,
                            (*flayer).l_width - 1 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        i += 1;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer;
                    (*l).l_cvlist = cvlist as *mut Canvas;
                    (*cv_1).c_lnext = cvlnext;
                    display = olddisplay;
                } else if ly + (*cv_1).c_yoff > (*cv_1).c_ye {
                    let mut i_0: ::core::ffi::c_int = 0;
                    let mut n_0: ::core::ffi::c_int = ly + (*cv_1).c_yoff - (*cv_1).c_ye;
                    (*cv_1).c_yoff = (*cv_1).c_ye - ly;
                    RethinkViewportOffsets(cv_1);
                    if n_0 > (*(*cv_1).c_layer).l_height {
                        n_0 = (*(*cv_1).c_layer).l_height;
                    }
                    let mut olddisplay_0: *mut Display = display;
                    let mut oldflayer_0: *mut Layer = flayer;
                    let mut l_0: *mut Layer = (*cv_1).c_layer;
                    let mut cvlist_0: *mut Canvas = (*l_0).l_cvlist;
                    let mut cvlnext_0: *mut Canvas = (*cv_1).c_lnext;
                    flayer = l_0;
                    (*l_0).l_cvlist = cv_1 as *mut Canvas;
                    (*cv_1).c_lnext = ::core::ptr::null_mut::<Canvas>();
                    LScrollV(
                        flayer,
                        n_0,
                        0 as ::core::ffi::c_int,
                        (*(*cv_1).c_layer).l_height - 1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    Some(
                        (*(*flayer).l_layfn)
                            .lf_LayRedisplayLine
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    i_0 = 0 as ::core::ffi::c_int;
                    while i_0 < n_0 {
                        Some(
                            (*(*flayer).l_layfn)
                                .lf_LayRedisplayLine
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            i_0 + (*flayer).l_height - n_0,
                            0 as ::core::ffi::c_int,
                            (*flayer).l_width - 1 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        i_0 += 1;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer_0;
                    (*l_0).l_cvlist = cvlist_0 as *mut Canvas;
                    (*cv_1).c_lnext = cvlnext_0;
                    display = olddisplay_0;
                }
                if lx + (*cv_1).c_xoff < (*cv_1).c_xs {
                    let mut i_1: ::core::ffi::c_int = 0;
                    let mut n_1: ::core::ffi::c_int = (*cv_1).c_xs - (lx + (*cv_1).c_xoff);
                    if n_1
                        < ((*cv_1).c_xe - (*cv_1).c_xs + 1 as ::core::ffi::c_int)
                            / 2 as ::core::ffi::c_int
                    {
                        n_1 = ((*cv_1).c_xe - (*cv_1).c_xs + 1 as ::core::ffi::c_int)
                            / 2 as ::core::ffi::c_int;
                    }
                    if (*cv_1).c_xoff + n_1 > (*cv_1).c_xs {
                        n_1 = (*cv_1).c_xs - (*cv_1).c_xoff;
                    }
                    (*cv_1).c_xoff += n_1;
                    RethinkViewportOffsets(cv_1);
                    if n_1 > (*(*cv_1).c_layer).l_width {
                        n_1 = (*(*cv_1).c_layer).l_width;
                    }
                    let mut olddisplay_1: *mut Display = display;
                    let mut oldflayer_1: *mut Layer = flayer;
                    let mut l_1: *mut Layer = (*cv_1).c_layer;
                    let mut cvlist_1: *mut Canvas = (*l_1).l_cvlist;
                    let mut cvlnext_1: *mut Canvas = (*cv_1).c_lnext;
                    flayer = l_1;
                    (*l_1).l_cvlist = cv_1 as *mut Canvas;
                    (*cv_1).c_lnext = ::core::ptr::null_mut::<Canvas>();
                    Some(
                        (*(*flayer).l_layfn)
                            .lf_LayRedisplayLine
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    i_1 = 0 as ::core::ffi::c_int;
                    while i_1 < (*flayer).l_height {
                        LScrollH(
                            flayer,
                            -n_1,
                            i_1,
                            0 as ::core::ffi::c_int,
                            (*flayer).l_width - 1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            ::core::ptr::null_mut::<mline>(),
                        );
                        Some(
                            (*(*flayer).l_layfn)
                                .lf_LayRedisplayLine
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            i_1,
                            0 as ::core::ffi::c_int,
                            n_1 - 1 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        i_1 += 1;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer_1;
                    (*l_1).l_cvlist = cvlist_1 as *mut Canvas;
                    (*cv_1).c_lnext = cvlnext_1;
                    display = olddisplay_1;
                } else if lx + (*cv_1).c_xoff > (*cv_1).c_xe {
                    let mut i_2: ::core::ffi::c_int = 0;
                    let mut n_2: ::core::ffi::c_int = lx + (*cv_1).c_xoff - (*cv_1).c_xe;
                    if n_2
                        < ((*cv_1).c_xe - (*cv_1).c_xs + 1 as ::core::ffi::c_int)
                            / 2 as ::core::ffi::c_int
                    {
                        n_2 = ((*cv_1).c_xe - (*cv_1).c_xs + 1 as ::core::ffi::c_int)
                            / 2 as ::core::ffi::c_int;
                    }
                    if ((*cv_1).c_xoff - n_2 + (*(*cv_1).c_layer).l_width - 1 as ::core::ffi::c_int)
                        < (*cv_1).c_xe
                    {
                        n_2 = (*cv_1).c_xoff + (*(*cv_1).c_layer).l_width
                            - 1 as ::core::ffi::c_int
                            - (*cv_1).c_xe;
                    }
                    (*cv_1).c_xoff -= n_2;
                    RethinkViewportOffsets(cv_1);
                    if n_2 > (*(*cv_1).c_layer).l_width {
                        n_2 = (*(*cv_1).c_layer).l_width;
                    }
                    let mut olddisplay_2: *mut Display = display;
                    let mut oldflayer_2: *mut Layer = flayer;
                    let mut l_2: *mut Layer = (*cv_1).c_layer;
                    let mut cvlist_2: *mut Canvas = (*l_2).l_cvlist;
                    let mut cvlnext_2: *mut Canvas = (*cv_1).c_lnext;
                    flayer = l_2;
                    (*l_2).l_cvlist = cv_1 as *mut Canvas;
                    (*cv_1).c_lnext = ::core::ptr::null_mut::<Canvas>();
                    Some(
                        (*(*flayer).l_layfn)
                            .lf_LayRedisplayLine
                            .expect("non-null function pointer"),
                    )
                    .expect("non-null function pointer")(
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        -1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    i_2 = 0 as ::core::ffi::c_int;
                    while i_2 < (*flayer).l_height {
                        LScrollH(
                            flayer,
                            n_2,
                            i_2,
                            0 as ::core::ffi::c_int,
                            (*flayer).l_width - 1 as ::core::ffi::c_int,
                            0 as ::core::ffi::c_int,
                            ::core::ptr::null_mut::<mline>(),
                        );
                        Some(
                            (*(*flayer).l_layfn)
                                .lf_LayRedisplayLine
                                .expect("non-null function pointer"),
                        )
                        .expect("non-null function pointer")(
                            i_2,
                            (*flayer).l_width - n_2,
                            (*flayer).l_width - 1 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        i_2 += 1;
                    }
                    if cv_1 == (*(*cv_1).c_display).d_forecv {
                        LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
                    }
                    flayer = oldflayer_2;
                    (*l_2).l_cvlist = cvlist_2 as *mut Canvas;
                    (*cv_1).c_lnext = cvlnext_2;
                    display = olddisplay_2;
                }
                cv_1 = (*cv_1).c_next;
            }
        }
        display = (*display).d_next;
    }
    display = displays;
    while !display.is_null() {
        if !((*display).d_status.0 == DisplayStatus::STATUS_ON_WIN.0
            || (*display).d_cvlist.is_null()
            || (*(*display).d_cvlist).c_next.is_null())
        {
            let mut olddisplay_3: *mut Display = display;
            let mut oldflayer_3: *mut Layer = flayer;
            let mut l_3: *mut Layer = (*(*display).d_forecv).c_layer;
            let mut cvlist_3: *mut Canvas = (*l_3).l_cvlist;
            let mut cvlnext_3: *mut Canvas = (*(*display).d_forecv).c_lnext;
            flayer = l_3;
            (*l_3).l_cvlist = (*display).d_forecv as *mut Canvas;
            (*(*display).d_forecv).c_lnext = ::core::ptr::null_mut::<Canvas>();
            Some(
                (*(*flayer).l_layfn)
                    .lf_LayRestore
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")();
            LGotoPos(flayer, (*flayer).l_x, (*flayer).l_y);
            flayer = oldflayer_3;
            (*l_3).l_cvlist = cvlist_3 as *mut Canvas;
            (*(*display).d_forecv).c_lnext = cvlnext_3;
            display = olddisplay_3;
        }
        display = (*display).d_next;
    }
}
unsafe extern "C" fn logflush_fn(mut event: *mut Event, mut data: *mut ::core::ffi::c_void) {
    let mut p: *mut Window = ::core::ptr::null_mut::<Window>();
    let mut buf: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut n: ::core::ffi::c_int = 0;
    if islogfile(::core::ptr::null_mut::<::core::ffi::c_char>()) == 0 {
        return;
    }
    logfflush(::core::ptr::null_mut::<Log>());
    n = if log_flush != 0 {
        log_flush
    } else {
        (logtstamp_after + 4 as ::core::ffi::c_int) / 5 as ::core::ffi::c_int
    };
    if n != 0 {
        SetTimeout(event, n * 1000 as ::core::ffi::c_int);
        evenq(event);
    }
    if !logtstamp_on {
        return;
    }
    p = mru_window;
    while !p.is_null() {
        if !(*p).w_log.is_null() {
            (*p).w_logsilence += n;
            if (*p).w_logsilence >= logtstamp_after {
                if (*p).w_logsilence - n < logtstamp_after {
                    buf = MakeWinMsg(logtstamp_string, p, '%' as ::core::ffi::c_int);
                    logfwrite((*p).w_log, buf, strlen(buf));
                }
            }
        }
        p = (*p).w_prev_mru;
    }
}
unsafe extern "C" fn ParseChar(
    mut p: *mut ::core::ffi::c_char,
    mut cp: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if *p as ::core::ffi::c_int == '^' as ::core::ffi::c_int
        && *p.offset(1isize) as ::core::ffi::c_int != 0
    {
        p = p.offset(1);
        if *p as ::core::ffi::c_int == '?' as ::core::ffi::c_int {
            *cp = '\u{7f}' as ::core::ffi::c_char;
        } else if *p as ::core::ffi::c_int >= '@' as ::core::ffi::c_int {
            *cp = (*p as ::core::ffi::c_int & 0o37 as ::core::ffi::c_int) as ::core::ffi::c_char;
        } else {
            return ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        p = p.offset(1);
    } else if *p as ::core::ffi::c_int == '\\' as ::core::ffi::c_int
        && {
            p = p.offset(1);
            *p as ::core::ffi::c_int <= '7' as ::core::ffi::c_int
        }
        && *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int
    {
        *cp = 0 as ::core::ffi::c_char;
        loop {
            *cp = (*cp as ::core::ffi::c_int * 8 as ::core::ffi::c_int + *p as ::core::ffi::c_int
                - '0' as ::core::ffi::c_int) as ::core::ffi::c_char;
            p = p.offset(1);
            if !(*p as ::core::ffi::c_int <= '7' as ::core::ffi::c_int
                && *p as ::core::ffi::c_int >= '0' as ::core::ffi::c_int)
            {
                break;
            }
        }
    } else {
        let c2rust_fresh18 = p;
        p = p.offset(1);
        *cp = *c2rust_fresh18;
    }
    return p;
}
unsafe extern "C" fn ParseEscape(mut p: *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    let mut buf: [::core::ffi::c_uchar; 2] = [0; 2];
    if *p as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        SetEscape(
            ::core::ptr::null_mut::<acluser>(),
            -1 as ::core::ffi::c_int,
            -1 as ::core::ffi::c_int,
        );
    } else {
        p = ParseChar(
            p,
            &raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char,
        );
        if p.is_null()
            || {
                p = ParseChar(
                    p,
                    (&raw mut buf as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_char)
                        .offset(1 as ::core::ffi::c_int as isize),
                );
                p.is_null()
            }
            || *p as ::core::ffi::c_int != 0
        {
            return -1 as ::core::ffi::c_int;
        }
        SetEscape(
            ::core::ptr::null_mut::<acluser>(),
            buf[0usize] as ::core::ffi::c_int,
            buf[1usize] as ::core::ffi::c_int,
        );
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn SetTtyname(mut fatal: bool, mut st: *mut stat) {
    let mut ret: ::core::ffi::c_int = 0;
    let mut saved_errno: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    attach_tty_is_in_new_ns = r#false != 0;
    memset(
        &raw mut attach_tty_name_in_ns as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
            .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>()),
    );
    *__errno_location() = 0 as ::core::ffi::c_int;
    attach_tty = ttyname(0 as ::core::ffi::c_int);
    if attach_tty.is_null() {
        if *__errno_location() == ENODEV {
            saved_errno = *__errno_location();
            attach_tty = b"/proc/self/fd/0\0".as_ptr() as *const ::core::ffi::c_char
                as *mut ::core::ffi::c_char;
            attach_tty_is_in_new_ns = r#true != 0;
            ret = readlink(
                attach_tty,
                &raw mut attach_tty_name_in_ns as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                    .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>()),
            ) as ::core::ffi::c_int;
            if ret < 0 as ::core::ffi::c_int
                || ret as size_t
                    >= ::core::mem::size_of::<[::core::ffi::c_char; 4096]>()
                        .wrapping_div(::core::mem::size_of::<::core::ffi::c_char>())
            {
                Panic(
                    0 as ::core::ffi::c_int,
                    b"Bad tty '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                    attach_tty,
                );
            }
        } else if fatal {
            Panic(
                0 as ::core::ffi::c_int,
                b"Must be connected to a terminal.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            attach_tty = b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
    }
    if !attach_tty.is_null()
        && strcmp(attach_tty, b"\0".as_ptr() as *const ::core::ffi::c_char) != 0
    {
        if stat(attach_tty, st) != 0 {
            Panic(
                *__errno_location(),
                b"Cannot access '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                attach_tty,
            );
        }
        if strlen(attach_tty) >= MAXPATHLEN as size_t {
            Panic(
                0 as ::core::ffi::c_int,
                b"TtyName too long - sorry.\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if saved_errno != ENODEV && CheckTtyname(attach_tty) != 0 {
            Panic(
                0 as ::core::ffi::c_int,
                b"Bad tty '%s'\0".as_ptr() as *const ::core::ffi::c_char,
                attach_tty,
            );
        }
    }
}
pub const SCREENENCODINGS: [::core::ffi::c_char; 59] = unsafe {
    ::core::mem::transmute::<[u8; 59], [::core::ffi::c_char; 59]>(
        *b"/root/rboxc/build/oracle/screen/share/screen/utf8encodings\0",
    )
};
pub const FNBLOCK: ::core::ffi::c_int = FNONBLOCK;
pub const MAXTERMLEN: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const MAXLOGINLEN: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const BUILD_DATE: [::core::ffi::c_char; 20] = unsafe {
    ::core::mem::transmute::<[u8; 20], [::core::ffi::c_char; 20]>(*b"2023-11-14 22:13:20\0")
};
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
