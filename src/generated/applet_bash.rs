// Generated from pinned GNU bash 5.3 by scripts/translate-entry-provider.py.
// Source SHA-256: 5f3d539f15e19c032a09ece500efecc9c63d050d408e985b6f385c64329bf73d
/* shell.c -- GNU's idea of the POSIX shell specification. */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
use ::c2rust_bitfields;
extern "C" {
    fn abort() -> !;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    #[link_name = "rboxc_bash_dollar_vars"]
    static mut dollar_vars: [*mut ::core::ffi::c_char; 0];
    #[link_name = "rboxc_bash_shell_environment"]
    static mut shell_environment: *mut *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_command_execution_string"]
    static mut command_execution_string: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_debugging_mode"]
    static mut debugging_mode: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_executing"]
    static mut executing: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_login_shell"]
    static mut login_shell: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_su_shell"]
    static mut su_shell: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_interactive_shell"]
    static mut interactive_shell: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_interactive"]
    static mut interactive: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_startup_state"]
    static mut startup_state: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_reading_shell_script"]
    static mut reading_shell_script: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_ssh_reading_startup_files"]
    static mut ssh_reading_startup_files: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_shell_initialized"]
    static mut shell_initialized: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_running_under_emacs"]
    static mut running_under_emacs: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_pretty_print_mode"]
    static mut pretty_print_mode: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_posixly_correct"]
    static mut posixly_correct: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_no_line_editing"]
    static mut no_line_editing: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_shell_name"]
    static mut shell_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_subshell_argc"]
    static mut subshell_argc: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_subshell_argv"]
    static mut subshell_argv: *mut *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_subshell_envp"]
    static mut subshell_envp: *mut *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_default_buffered_input"]
    static mut default_buffered_input: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_exit_immediately_on_error"]
    static mut exit_immediately_on_error: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_read_but_dont_execute"]
    static mut read_but_dont_execute: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_echo_input_at_read"]
    static mut echo_input_at_read: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_verbose_flag"]
    static mut verbose_flag: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_forced_interactive"]
    static mut forced_interactive: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_privileged_mode"]
    static mut privileged_mode: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_want_pending_command"]
    static mut want_pending_command: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_read_from_stdin"]
    static mut read_from_stdin: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_restricted"]
    static mut restricted: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_restricted_shell"]
    static mut restricted_shell: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_last_command_exit_value"]
    static mut last_command_exit_value: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_this_command_name"]
    static mut this_command_name: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_history_lines_this_session"]
    static mut history_lines_this_session: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_gnu_error_format"]
    static mut gnu_error_format: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_debugging_login_shell"]
    static mut debugging_login_shell: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_shell_start_time"]
    static mut shell_start_time: time_t;
    #[link_name = "rboxc_bash_shellstart"]
    static mut shellstart: timeval;
    #[link_name = "rboxc_bash_act_like_sh"]
    static mut act_like_sh: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_sourced_env"]
    static mut sourced_env: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_running_setuid"]
    static mut running_setuid: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_do_version"]
    static mut do_version: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_make_login_shell"]
    static mut make_login_shell: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_want_initial_help"]
    static mut want_initial_help: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_dump_translatable_strings"]
    static mut dump_translatable_strings: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_dump_po_strings"]
    static mut dump_po_strings: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_wordexp_only"]
    static mut wordexp_only: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_exec_argv0"]
    static mut exec_argv0: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_shell_script_filename"]
    static mut shell_script_filename: *mut ::core::ffi::c_char;
    #[link_name = "rboxc_bash_shell_reinitialized"]
    static mut shell_reinitialized: ::core::ffi::c_int;
    #[link_name = "rboxc_bash_default_input"]
    static mut default_input: *mut FILE;
    #[link_name = "rboxc_bash_shopt_alist"]
    static mut shopt_alist: *mut STRING_INT_ALIST;
    #[link_name = "rboxc_bash_rboxc_gate_begin"]
    fn rboxc_gate_begin(g: *mut rboxc_bash_gate);
    #[link_name = "rboxc_bash_rboxc_gate_checkpoint"]
    fn rboxc_gate_checkpoint(
        g: *mut rboxc_bash_gate,
        id: ::core::ffi::c_int,
        sub: ::core::ffi::c_int,
        save: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_bash_rboxc_gate_call_0"]
    fn rboxc_gate_call_0(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_0;
    #[link_name = "rboxc_bash_rboxc_gate_call_1"]
    fn rboxc_gate_call_1(g: *mut rboxc_bash_gate) -> rboxc_gate_result_1;
    #[link_name = "rboxc_bash_rboxc_gate_call_2"]
    fn rboxc_gate_call_2(g: *mut rboxc_bash_gate) -> rboxc_gate_result_2;
    #[link_name = "rboxc_bash_rboxc_gate_call_3"]
    fn rboxc_gate_call_3(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_uint) -> rboxc_gate_result_3;
    #[link_name = "rboxc_bash_rboxc_gate_call_4"]
    fn rboxc_gate_call_4(g: *mut rboxc_bash_gate) -> rboxc_gate_result_4;
    #[link_name = "rboxc_bash_rboxc_gate_call_5"]
    fn rboxc_gate_call_5(g: *mut rboxc_bash_gate) -> rboxc_gate_result_5;
    #[link_name = "rboxc_bash_rboxc_gate_call_6"]
    fn rboxc_gate_call_6(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_6;
    #[link_name = "rboxc_bash_rboxc_gate_call_7"]
    fn rboxc_gate_call_7(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_7;
    #[link_name = "rboxc_bash_rboxc_gate_call_8"]
    fn rboxc_gate_call_8(g: *mut rboxc_bash_gate) -> rboxc_gate_result_8;
    #[link_name = "rboxc_bash_rboxc_gate_call_9"]
    fn rboxc_gate_call_9(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_9;
    #[link_name = "rboxc_bash_rboxc_gate_call_10"]
    fn rboxc_gate_call_10(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
    ) -> rboxc_gate_result_10;
    #[link_name = "rboxc_bash_rboxc_gate_call_11"]
    fn rboxc_gate_call_11(
        g: *mut rboxc_bash_gate,
        a0: *mut timeval,
        a1: *mut ::core::ffi::c_void,
    ) -> rboxc_gate_result_11;
    #[link_name = "rboxc_bash_rboxc_gate_call_12"]
    fn rboxc_gate_call_12(
        g: *mut rboxc_bash_gate,
        a0: *mut *mut ::core::ffi::c_char,
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_12;
    #[link_name = "rboxc_bash_rboxc_gate_call_13"]
    fn rboxc_gate_call_13(
        g: *mut rboxc_bash_gate,
        a0: *mut FILE,
        a1: ::core::ffi::c_int,
    ) -> rboxc_gate_result_13;
    #[link_name = "rboxc_bash_rboxc_gate_call_14"]
    fn rboxc_gate_call_14(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_14;
    #[link_name = "rboxc_bash_rboxc_gate_call_15"]
    fn rboxc_gate_call_15(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_15;
    #[link_name = "rboxc_bash_rboxc_gate_call_16"]
    fn rboxc_gate_call_16(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_16;
    #[link_name = "rboxc_bash_rboxc_gate_call_17"]
    fn rboxc_gate_call_17(
        g: *mut rboxc_bash_gate,
        a0: *mut *mut ::core::ffi::c_char,
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_17;
    #[link_name = "rboxc_bash_rboxc_gate_call_18"]
    fn rboxc_gate_call_18(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
        a1: ::core::ffi::c_int,
    ) -> rboxc_gate_result_18;
    #[link_name = "rboxc_bash_rboxc_gate_call_19"]
    fn rboxc_gate_call_19(g: *mut rboxc_bash_gate) -> rboxc_gate_result_19;
    #[link_name = "rboxc_bash_rboxc_gate_call_20"]
    fn rboxc_gate_call_20(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_20;
    #[link_name = "rboxc_bash_rboxc_gate_call_21"]
    fn rboxc_gate_call_21(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *mut ::core::ffi::c_char,
    ) -> rboxc_gate_result_21;
    #[link_name = "rboxc_bash_rboxc_gate_call_22"]
    fn rboxc_gate_call_22(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_22;
    #[link_name = "rboxc_bash_rboxc_gate_call_23"]
    fn rboxc_gate_call_23(g: *mut rboxc_bash_gate, a0: *mut FILE) -> rboxc_gate_result_23;
    #[link_name = "rboxc_bash_rboxc_gate_call_24"]
    fn rboxc_gate_call_24(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_24;
    #[link_name = "rboxc_bash_rboxc_gate_call_25"]
    fn rboxc_gate_call_25(g: *mut rboxc_bash_gate, a0: *mut FILE) -> rboxc_gate_result_25;
    #[link_name = "rboxc_bash_rboxc_gate_call_26"]
    fn rboxc_gate_call_26(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_26;
    #[link_name = "rboxc_bash_rboxc_gate_call_27"]
    fn rboxc_gate_call_27(g: *mut rboxc_bash_gate) -> rboxc_gate_result_27;
    #[link_name = "rboxc_bash_rboxc_gate_call_28"]
    fn rboxc_gate_call_28(g: *mut rboxc_bash_gate) -> rboxc_gate_result_28;
    #[link_name = "rboxc_bash_rboxc_gate_call_29"]
    fn rboxc_gate_call_29(
        g: *mut rboxc_bash_gate,
        a0: ::core::ffi::c_int,
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_29;
    #[link_name = "rboxc_bash_rboxc_gate_call_30"]
    fn rboxc_gate_call_30(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_30;
    #[link_name = "rboxc_bash_rboxc_gate_call_31"]
    fn rboxc_gate_call_31(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_31;
    #[link_name = "rboxc_bash_rboxc_gate_call_32"]
    fn rboxc_gate_call_32(g: *mut rboxc_bash_gate) -> rboxc_gate_result_32;
    #[link_name = "rboxc_bash_rboxc_gate_call_33"]
    fn rboxc_gate_call_33(g: *mut rboxc_bash_gate) -> rboxc_gate_result_33;
    #[link_name = "rboxc_bash_rboxc_gate_call_34"]
    fn rboxc_gate_call_34(g: *mut rboxc_bash_gate) -> rboxc_gate_result_34;
    #[link_name = "rboxc_bash_rboxc_gate_call_35"]
    fn rboxc_gate_call_35(g: *mut rboxc_bash_gate) -> rboxc_gate_result_35;
    #[link_name = "rboxc_bash_rboxc_gate_call_36"]
    fn rboxc_gate_call_36(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_36;
    #[link_name = "rboxc_bash_rboxc_gate_call_37"]
    fn rboxc_gate_call_37(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_37;
    #[link_name = "rboxc_bash_rboxc_gate_call_38"]
    fn rboxc_gate_call_38(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_38;
    #[link_name = "rboxc_bash_rboxc_gate_call_39"]
    fn rboxc_gate_call_39(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_39;
    #[link_name = "rboxc_bash_rboxc_gate_call_40"]
    fn rboxc_gate_call_40(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_40;
    #[link_name = "rboxc_bash_rboxc_gate_call_41"]
    fn rboxc_gate_call_41(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_41;
    #[link_name = "rboxc_bash_rboxc_gate_call_42"]
    fn rboxc_gate_call_42(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_42;
    #[link_name = "rboxc_bash_rboxc_gate_call_43"]
    fn rboxc_gate_call_43(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_43;
    #[link_name = "rboxc_bash_rboxc_gate_call_44"]
    fn rboxc_gate_call_44(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
        a2: size_t,
    ) -> rboxc_gate_result_44;
    #[link_name = "rboxc_bash_rboxc_gate_call_45"]
    fn rboxc_gate_call_45(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
        a2: size_t,
    ) -> rboxc_gate_result_45;
    #[link_name = "rboxc_bash_rboxc_gate_call_46"]
    fn rboxc_gate_call_46(g: *mut rboxc_bash_gate) -> rboxc_gate_result_46;
    #[link_name = "rboxc_bash_rboxc_gate_call_47"]
    fn rboxc_gate_call_47(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_47;
    #[link_name = "rboxc_bash_rboxc_gate_call_48"]
    fn rboxc_gate_call_48(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_48;
    #[link_name = "rboxc_bash_rboxc_gate_call_49"]
    fn rboxc_gate_call_49(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_49;
    #[link_name = "rboxc_bash_rboxc_gate_call_50"]
    fn rboxc_gate_call_50(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_50;
    #[link_name = "rboxc_bash_rboxc_gate_call_51"]
    fn rboxc_gate_call_51(
        g: *mut rboxc_bash_gate,
        a0: ::core::ffi::c_int,
        a1: ::core::ffi::c_int,
    ) -> rboxc_gate_result_51;
    #[link_name = "rboxc_bash_rboxc_gate_call_52"]
    fn rboxc_gate_call_52(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
    ) -> rboxc_gate_result_52;
    #[link_name = "rboxc_bash_rboxc_gate_call_53"]
    fn rboxc_gate_call_53(
        g: *mut rboxc_bash_gate,
        a0: *mut *mut ::core::ffi::c_char,
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
        a3: ::core::ffi::c_int,
    ) -> rboxc_gate_result_53;
    #[link_name = "rboxc_bash_rboxc_gate_call_54"]
    fn rboxc_gate_call_54(
        g: *mut rboxc_bash_gate,
        a0: *mut *mut ::core::ffi::c_char,
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
        a3: ::core::ffi::c_int,
    ) -> rboxc_gate_result_54;
    #[link_name = "rboxc_bash_rboxc_gate_call_55"]
    fn rboxc_gate_call_55(
        g: *mut rboxc_bash_gate,
        a0: *mut *mut ::core::ffi::c_char,
        a1: ::core::ffi::c_int,
        a2: ::core::ffi::c_int,
        a3: ::core::ffi::c_int,
    ) -> rboxc_gate_result_55;
    #[link_name = "rboxc_bash_rboxc_gate_call_56"]
    fn rboxc_gate_call_56(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_56;
    #[link_name = "rboxc_bash_rboxc_gate_call_57"]
    fn rboxc_gate_call_57(
        g: *mut rboxc_bash_gate,
        a0: ::core::ffi::c_ulong,
    ) -> rboxc_gate_result_57;
    #[link_name = "rboxc_bash_rboxc_gate_call_58"]
    fn rboxc_gate_call_58(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_58;
    #[link_name = "rboxc_bash_rboxc_gate_call_59"]
    fn rboxc_gate_call_59(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_59;
    #[link_name = "rboxc_bash_rboxc_gate_call_60"]
    fn rboxc_gate_call_60(
        g: *mut rboxc_bash_gate,
        a0: ::core::ffi::c_ulong,
    ) -> rboxc_gate_result_60;
    #[link_name = "rboxc_bash_rboxc_gate_call_61"]
    fn rboxc_gate_call_61(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_61;
    #[link_name = "rboxc_bash_rboxc_gate_call_62"]
    fn rboxc_gate_call_62(g: *mut rboxc_bash_gate) -> rboxc_gate_result_62;
    #[link_name = "rboxc_bash_rboxc_gate_call_63"]
    fn rboxc_gate_call_63(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_void,
    ) -> rboxc_gate_result_63;
    #[link_name = "rboxc_bash_rboxc_gate_call_64"]
    fn rboxc_gate_call_64(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_64;
    #[link_name = "rboxc_bash_rboxc_gate_call_65"]
    fn rboxc_gate_call_65(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_65;
    #[link_name = "rboxc_bash_rboxc_gate_call_66"]
    fn rboxc_gate_call_66(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
    ) -> rboxc_gate_result_66;
    #[link_name = "rboxc_bash_rboxc_gate_call_67"]
    fn rboxc_gate_call_67(g: *mut rboxc_bash_gate) -> rboxc_gate_result_67;
    #[link_name = "rboxc_bash_rboxc_gate_call_68"]
    fn rboxc_gate_call_68(g: *mut rboxc_bash_gate) -> rboxc_gate_result_68;
    #[link_name = "rboxc_bash_rboxc_gate_call_69"]
    fn rboxc_gate_call_69(g: *mut rboxc_bash_gate) -> rboxc_gate_result_69;
    #[link_name = "rboxc_bash_rboxc_gate_call_70"]
    fn rboxc_gate_call_70(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
    ) -> rboxc_gate_result_70;
    #[link_name = "rboxc_bash_rboxc_gate_call_71"]
    fn rboxc_gate_call_71(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_71;
    #[link_name = "rboxc_bash_rboxc_gate_call_72"]
    fn rboxc_gate_call_72(
        g: *mut rboxc_bash_gate,
        a0: *mut ::core::ffi::c_char,
    ) -> rboxc_gate_result_72;
    #[link_name = "rboxc_bash_rboxc_gate_call_73"]
    fn rboxc_gate_call_73(g: *mut rboxc_bash_gate, a0: *mut FILE) -> rboxc_gate_result_73;
    #[link_name = "rboxc_bash_rboxc_gate_call_74"]
    fn rboxc_gate_call_74(g: *mut rboxc_bash_gate) -> rboxc_gate_result_74;
    #[link_name = "rboxc_bash_rboxc_gate_call_75"]
    fn rboxc_gate_call_75(g: *mut rboxc_bash_gate) -> rboxc_gate_result_75;
    #[link_name = "rboxc_bash_rboxc_gate_call_76"]
    fn rboxc_gate_call_76(g: *mut rboxc_bash_gate) -> rboxc_gate_result_76;
    #[link_name = "rboxc_bash_rboxc_gate_call_77"]
    fn rboxc_gate_call_77(g: *mut rboxc_bash_gate) -> rboxc_gate_result_77;
    #[link_name = "rboxc_bash_rboxc_gate_call_78"]
    fn rboxc_gate_call_78(g: *mut rboxc_bash_gate) -> rboxc_gate_result_78;
    #[link_name = "rboxc_bash_rboxc_gate_call_79"]
    fn rboxc_gate_call_79(g: *mut rboxc_bash_gate) -> rboxc_gate_result_79;
    #[link_name = "rboxc_bash_rboxc_gate_call_80"]
    fn rboxc_gate_call_80(g: *mut rboxc_bash_gate) -> rboxc_gate_result_80;
    #[link_name = "rboxc_bash_rboxc_gate_call_81"]
    fn rboxc_gate_call_81(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
        a1: *const ::core::ffi::c_char,
        a2: ::core::ffi::c_int,
    ) -> rboxc_gate_result_81;
    #[link_name = "rboxc_bash_rboxc_gate_call_82"]
    fn rboxc_gate_call_82(
        g: *mut rboxc_bash_gate,
        a0: *const ::core::ffi::c_char,
    ) -> rboxc_gate_result_82;
    #[link_name = "rboxc_bash_rboxc_gate_call_83"]
    fn rboxc_gate_call_83(g: *mut rboxc_bash_gate) -> rboxc_gate_result_83;
    #[link_name = "rboxc_bash_rboxc_gate_call_84"]
    fn rboxc_gate_call_84(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_84;
    #[link_name = "rboxc_bash_rboxc_gate_call_85"]
    fn rboxc_gate_call_85(g: *mut rboxc_bash_gate) -> rboxc_gate_result_85;
    #[link_name = "rboxc_bash_rboxc_gate_call_86"]
    fn rboxc_gate_call_86(g: *mut rboxc_bash_gate, a0: ::core::ffi::c_int) -> rboxc_gate_result_86;
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
pub type __uint64_t = u64;
pub type __off64_t = ::core::ffi::c_long;
pub type _IO_lock_t = ();
pub type __off_t = ::core::ffi::c_long;
pub type FILE = _IO_FILE;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type intmax_t = ::libc::intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct STRING_INT_ALIST {
    pub word: *mut ::core::ffi::c_char,
    pub token: ::core::ffi::c_int,
}
pub type arrayind_t = intmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable {
    pub name: *mut ::core::ffi::c_char,
    pub value: *mut ::core::ffi::c_char,
    pub exportstr: *mut ::core::ffi::c_char,
    pub dynamic_value: Option<sh_var_value_func_t>,
    pub assign_func: Option<sh_var_assign_func_t>,
    pub attributes: ::core::ffi::c_int,
    pub context: ::core::ffi::c_int,
}
pub type sh_var_assign_func_t = unsafe extern "C" fn(
    *mut variable,
    *mut ::core::ffi::c_char,
    arrayind_t,
    *mut ::core::ffi::c_char,
) -> *mut variable;
pub type sh_var_value_func_t = unsafe extern "C" fn(*mut variable) -> *mut variable;
pub type SHELL_VAR = variable;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_bash_gate {
    pub allowed: sigset_t,
    pub top_mask: sigset_t,
    pub sub_mask: sigset_t,
    pub top: ::core::ffi::c_int,
    pub sub: ::core::ffi::c_int,
    pub top_save: ::core::ffi::c_int,
    pub sub_save: ::core::ffi::c_int,
    pub pending: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_0 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_1 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_2 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_3 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_4 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_5 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_6 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_7 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_8 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_9 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_10 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_11 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_12 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_13 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_14 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_15 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_16 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_17 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_18 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_19 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_20 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_21 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_22 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_23 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_24 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_25 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_26 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_27 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_28 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_29 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_30 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut SHELL_VAR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_31 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_32 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_33 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_34 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_35 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_36 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_37 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_38 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_39 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_40 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_41 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_42 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_43 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_44 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_45 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_46 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_47 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_48 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_49 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_50 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_51 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_52 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_53 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_54 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_55 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_56 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_57 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_58 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_59 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_60 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_61 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_62 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_63 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_64 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut SHELL_VAR,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_65 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_66 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_67 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_68 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_69 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_70 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_71 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_72 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_73 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_74 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_75 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_76 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_77 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_78 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_79 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_80 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_81 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_82 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_83 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_84 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_85 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
    pub value: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rboxc_gate_result_86 {
    pub jumped: ::core::ffi::c_int,
    pub target: ::core::ffi::c_int,
    pub code: ::core::ffi::c_int,
}
unsafe extern "C" fn rboxc_bash_main_inner(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
    mut env: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut rboxc_result_1: rboxc_gate_result_1 = rboxc_gate_result_1 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_2: rboxc_gate_result_2 = rboxc_gate_result_2 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_4: rboxc_gate_result_4 = rboxc_gate_result_4 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_5: rboxc_gate_result_5 = rboxc_gate_result_5 {
        jumped: 0,
        target: 0,
        code: 0,
        value: 0,
    };
    let mut rboxc_result_6: rboxc_gate_result_6 = rboxc_gate_result_6 {
        jumped: 0,
        target: 0,
        code: 0,
        value: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    };
    let mut rboxc_expr_1: ::core::ffi::c_int = 0;
    let mut rboxc_expr_2: ::core::ffi::c_int = 0;
    let mut rboxc_result_10: rboxc_gate_result_10 = rboxc_gate_result_10 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_11: rboxc_gate_result_11 = rboxc_gate_result_11 {
        jumped: 0,
        target: 0,
        code: 0,
        value: 0,
    };
    let mut rboxc_result_12: rboxc_gate_result_12 = rboxc_gate_result_12 {
        jumped: 0,
        target: 0,
        code: 0,
        value: 0,
    };
    let mut rboxc_result_17: rboxc_gate_result_17 = rboxc_gate_result_17 {
        jumped: 0,
        target: 0,
        code: 0,
        value: 0,
    };
    let mut rboxc_result_18: rboxc_gate_result_18 = rboxc_gate_result_18 {
        jumped: 0,
        target: 0,
        code: 0,
        value: 0,
    };
    let mut rboxc_expr_3: ::core::ffi::c_int = 0;
    let mut rboxc_expr_9: ::core::ffi::c_int = 0;
    let mut rboxc_expr_10: ::core::ffi::c_int = 0;
    let mut rboxc_result_33: rboxc_gate_result_33 = rboxc_gate_result_33 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_34: rboxc_gate_result_34 = rboxc_gate_result_34 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_35: rboxc_gate_result_35 = rboxc_gate_result_35 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    let mut rboxc_result_46: rboxc_gate_result_46 = rboxc_gate_result_46 {
        jumped: 0,
        target: 0,
        code: 0,
        value: 0,
    };
    let mut rboxc_result_8: rboxc_gate_result_8 = rboxc_gate_result_8 {
        jumped: 0,
        target: 0,
        code: 0,
    };
    enum C2Rust_Block {
        S_396,
        S_528,
        S_2396,
    }
    let mut c2rust_current_block: C2Rust_Block;
    let mut rboxc_gate: rboxc_bash_gate = rboxc_bash_gate {
        allowed: __sigset_t {
            __val: [
                0 as ::core::ffi::c_ulong,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
            ],
        },
        top_mask: sigset_t { __val: [0; 16] },
        sub_mask: sigset_t { __val: [0; 16] },
        top: 0,
        sub: 0,
        top_save: 0,
        sub_save: 0,
        pending: 0,
        target: 0,
    };
    rboxc_gate_begin(&raw mut rboxc_gate);
    let mut i: ::core::ffi::c_int = 0;
    let mut code: ::core::ffi::c_int = 0;
    let mut old_errexit_flag: ::core::ffi::c_int = 0;
    let mut saverst: ::core::ffi::c_int = 0;
    let mut locally_skip_execution: ::core::ffi::c_int = 0;
    let mut arg_index: ::core::ffi::c_int = 0;
    let mut top_level_arg_index: ::core::ffi::c_int = 0;
    &raw mut argc;
    &raw mut argv;
    &raw mut env;
    &raw mut code;
    &raw mut old_errexit_flag;
    &raw mut saverst;
    's_38: loop {
        code = rboxc_gate_checkpoint(
            &raw mut rboxc_gate,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        's_2396: {
            if code != 0 {
                let mut rboxc_result_0: rboxc_gate_result_0 =
                    rboxc_gate_call_0(&raw mut rboxc_gate, 2 as ::core::ffi::c_int);
                if rboxc_result_0.jumped != 0 {
                    rboxc_gate.pending = rboxc_result_0.code;
                    rboxc_gate.target = rboxc_result_0.target;
                    match rboxc_result_0.target {
                        1 => {
                            continue 's_38;
                        }
                        2 => {
                            c2rust_current_block = C2Rust_Block::S_396;
                            break 's_2396;
                        }
                        3 => {
                            c2rust_current_block = C2Rust_Block::S_528;
                            break 's_2396;
                        }
                        4 => {
                            c2rust_current_block = C2Rust_Block::S_2396;
                            break 's_2396;
                        }
                        _ => {
                            abort();
                        }
                    }
                }
            }
            rboxc_result_1 = rboxc_gate_call_1(&raw mut rboxc_gate);
            if rboxc_result_1.jumped != 0 {
                rboxc_gate.pending = rboxc_result_1.code;
                rboxc_gate.target = rboxc_result_1.target;
                match rboxc_result_1.target {
                    1 => {
                        continue 's_38;
                    }
                    2 => {
                        c2rust_current_block = C2Rust_Block::S_396;
                    }
                    3 => {
                        c2rust_current_block = C2Rust_Block::S_528;
                    }
                    4 => {
                        c2rust_current_block = C2Rust_Block::S_2396;
                    }
                    _ => {
                        abort();
                    }
                }
            } else {
                rboxc_result_2 = rboxc_gate_call_2(&raw mut rboxc_gate);
                if rboxc_result_2.jumped != 0 {
                    rboxc_gate.pending = rboxc_result_2.code;
                    rboxc_gate.target = rboxc_result_2.target;
                    match rboxc_result_2.target {
                        1 => {
                            continue 's_38;
                        }
                        2 => {
                            c2rust_current_block = C2Rust_Block::S_396;
                        }
                        3 => {
                            c2rust_current_block = C2Rust_Block::S_528;
                        }
                        4 => {
                            c2rust_current_block = C2Rust_Block::S_2396;
                        }
                        _ => {
                            abort();
                        }
                    }
                } else {
                    while debugging_login_shell != 0 {
                        let mut rboxc_result_3: rboxc_gate_result_3 =
                            rboxc_gate_call_3(&raw mut rboxc_gate, 3 as ::core::ffi::c_uint);
                        if rboxc_result_3.jumped == 0 {
                            continue;
                        }
                        rboxc_gate.pending = rboxc_result_3.code;
                        rboxc_gate.target = rboxc_result_3.target;
                        match rboxc_result_3.target {
                            1 => {
                                continue 's_38;
                            }
                            2 => {
                                c2rust_current_block = C2Rust_Block::S_396;
                                break 's_2396;
                            }
                            3 => {
                                c2rust_current_block = C2Rust_Block::S_528;
                                break 's_2396;
                            }
                            4 => {
                                c2rust_current_block = C2Rust_Block::S_2396;
                                break 's_2396;
                            }
                            _ => {}
                        }
                        abort();
                    }
                    rboxc_result_4 = rboxc_gate_call_4(&raw mut rboxc_gate);
                    if rboxc_result_4.jumped != 0 {
                        rboxc_gate.pending = rboxc_result_4.code;
                        rboxc_gate.target = rboxc_result_4.target;
                        match rboxc_result_4.target {
                            1 => {
                                continue 's_38;
                            }
                            2 => {
                                c2rust_current_block = C2Rust_Block::S_396;
                            }
                            3 => {
                                c2rust_current_block = C2Rust_Block::S_528;
                            }
                            4 => {
                                c2rust_current_block = C2Rust_Block::S_2396;
                            }
                            _ => {
                                abort();
                            }
                        }
                    } else {
                        rboxc_result_5 = rboxc_gate_call_5(&raw mut rboxc_gate);
                        if rboxc_result_5.jumped != 0 {
                            rboxc_gate.pending = rboxc_result_5.code;
                            rboxc_gate.target = rboxc_result_5.target;
                            match rboxc_result_5.target {
                                1 => {
                                    continue 's_38;
                                }
                                2 => {
                                    c2rust_current_block = C2Rust_Block::S_396;
                                }
                                3 => {
                                    c2rust_current_block = C2Rust_Block::S_528;
                                }
                                4 => {
                                    c2rust_current_block = C2Rust_Block::S_2396;
                                }
                                _ => {
                                    abort();
                                }
                            }
                        } else {
                            running_setuid = rboxc_result_5.value;
                            rboxc_result_6 = rboxc_gate_call_6(
                                &raw mut rboxc_gate,
                                b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            if rboxc_result_6.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_6.code;
                                rboxc_gate.target = rboxc_result_6.target;
                                match rboxc_result_6.target {
                                    1 => {
                                        continue 's_38;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                    }
                                    _ => {
                                        abort();
                                    }
                                }
                            } else {
                                rboxc_expr_1 =
                                    !rboxc_result_6.value.is_null() as ::core::ffi::c_int;
                                if rboxc_expr_1 == 0 {
                                    let mut rboxc_result_7: rboxc_gate_result_7 = rboxc_gate_call_7(
                                        &raw mut rboxc_gate,
                                        b"POSIX_PEDANTIC\0".as_ptr() as *const ::core::ffi::c_char,
                                    );
                                    if rboxc_result_7.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_7.code;
                                        rboxc_gate.target = rboxc_result_7.target;
                                        match rboxc_result_7.target {
                                            1 => {
                                                continue 's_38;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                break 's_2396;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                break 's_2396;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                break 's_2396;
                                            }
                                            _ => {
                                                abort();
                                            }
                                        }
                                    } else {
                                        rboxc_expr_1 =
                                            !rboxc_result_7.value.is_null() as ::core::ffi::c_int;
                                    }
                                }
                                if rboxc_expr_1 != 0 {
                                    posixly_correct = 1 as ::core::ffi::c_int;
                                }
                                c2rust_current_block = C2Rust_Block::S_396;
                            }
                        }
                    }
                }
            }
        }
        's_396: loop {
            match c2rust_current_block {
                C2Rust_Block::S_2396 => {
                    code = rboxc_gate_checkpoint(
                        &raw mut rboxc_gate,
                        4 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    );
                    if code != 0 {
                        let mut rboxc_expr_15: ::core::ffi::c_int =
                            (code == 3 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        if rboxc_expr_15 == 0 {
                            rboxc_expr_15 = (code == 4 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        }
                        let mut rboxc_expr_16: ::core::ffi::c_int =
                            (rboxc_expr_15 != 0) as ::core::ffi::c_int;
                        if rboxc_expr_16 == 0 {
                            rboxc_expr_16 = (code == 6 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        }
                        if rboxc_expr_16 != 0 {
                            let mut rboxc_result_47: rboxc_gate_result_47 = rboxc_gate_call_47(
                                &raw mut rboxc_gate,
                                ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                    &raw const last_command_exit_value,
                                ),
                            );
                            if rboxc_result_47.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_47.code;
                                rboxc_gate.target = rboxc_result_47.target;
                                match rboxc_result_47.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            }
                        } else {
                            let mut rboxc_result_48: rboxc_gate_result_48 =
                                rboxc_gate_call_48(&raw mut rboxc_gate, interactive_shell);
                            if rboxc_result_48.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_48.code;
                                rboxc_gate.target = rboxc_result_48.target;
                                match rboxc_result_48.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                exit_immediately_on_error += old_errexit_flag;
                                ::core::ptr::write_volatile(
                                    &raw mut locally_skip_execution,
                                    ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                        &raw const locally_skip_execution,
                                    ) + 1,
                                );
                            }
                        }
                    }
                    ::core::ptr::write_volatile(
                        &raw mut arg_index,
                        ::core::ptr::read_volatile::<::core::ffi::c_int>(
                            &raw const top_level_arg_index,
                        ),
                    );
                    if interactive_shell == 0 as ::core::ffi::c_int {
                        let mut rboxc_result_49: rboxc_gate_result_49 = rboxc_gate_call_49(
                            &raw mut rboxc_gate,
                            b"PS1\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        if rboxc_result_49.jumped != 0 {
                            rboxc_gate.pending = rboxc_result_49.code;
                            rboxc_gate.target = rboxc_result_49.target;
                            match rboxc_result_49.target {
                                1 => {
                                    break;
                                }
                                2 => {
                                    c2rust_current_block = C2Rust_Block::S_396;
                                    continue;
                                }
                                3 => {
                                    c2rust_current_block = C2Rust_Block::S_528;
                                    continue;
                                }
                                4 => {
                                    c2rust_current_block = C2Rust_Block::S_2396;
                                    continue;
                                }
                                _ => {}
                            }
                            abort();
                        } else {
                            let mut rboxc_result_50: rboxc_gate_result_50 = rboxc_gate_call_50(
                                &raw mut rboxc_gate,
                                b"PS2\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                            if rboxc_result_50.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_50.code;
                                rboxc_gate.target = rboxc_result_50.target;
                                match rboxc_result_50.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                interactive = 0 as ::core::ffi::c_int;
                            }
                        }
                    } else {
                        let mut rboxc_result_51: rboxc_gate_result_51 = rboxc_gate_call_51(
                            &raw mut rboxc_gate,
                            'i' as ::core::ffi::c_int,
                            '-' as ::core::ffi::c_int,
                        );
                        if rboxc_result_51.jumped != 0 {
                            rboxc_gate.pending = rboxc_result_51.code;
                            rboxc_gate.target = rboxc_result_51.target;
                            match rboxc_result_51.target {
                                1 => {
                                    break;
                                }
                                2 => {
                                    c2rust_current_block = C2Rust_Block::S_396;
                                    continue;
                                }
                                3 => {
                                    c2rust_current_block = C2Rust_Block::S_528;
                                    continue;
                                }
                                4 => {
                                    c2rust_current_block = C2Rust_Block::S_2396;
                                    continue;
                                }
                                _ => {}
                            }
                            abort();
                        } else {
                            interactive = 1 as ::core::ffi::c_int;
                            if forced_interactive == 0 as ::core::ffi::c_int {
                                read_but_dont_execute = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                    let mut rboxc_result_52: rboxc_gate_result_52 =
                        rboxc_gate_call_52(&raw mut rboxc_gate, shell_name);
                    if rboxc_result_52.jumped != 0 {
                        rboxc_gate.pending = rboxc_result_52.code;
                        rboxc_gate.target = rboxc_result_52.target;
                        match rboxc_result_52.target {
                            1 => {
                                break;
                            }
                            2 => {
                                c2rust_current_block = C2Rust_Block::S_396;
                                continue;
                            }
                            3 => {
                                c2rust_current_block = C2Rust_Block::S_528;
                                continue;
                            }
                            4 => {
                                c2rust_current_block = C2Rust_Block::S_2396;
                                continue;
                            }
                            _ => {}
                        }
                        abort();
                    } else {
                        restricted_shell = rboxc_result_52.value;
                        saverst = restricted;
                        restricted = 0 as ::core::ffi::c_int;
                        if wordexp_only == 0 {
                            if !command_execution_string.is_null() {
                                let mut rboxc_result_53: rboxc_gate_result_53 = rboxc_gate_call_53(
                                    &raw mut rboxc_gate,
                                    argv,
                                    ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                        &raw const arg_index,
                                    ),
                                    argc,
                                    0 as ::core::ffi::c_int,
                                );
                                if rboxc_result_53.jumped != 0 {
                                    rboxc_gate.pending = rboxc_result_53.code;
                                    rboxc_gate.target = rboxc_result_53.target;
                                    match rboxc_result_53.target {
                                        1 => {
                                            break;
                                        }
                                        2 => {
                                            c2rust_current_block = C2Rust_Block::S_396;
                                            continue;
                                        }
                                        3 => {
                                            c2rust_current_block = C2Rust_Block::S_528;
                                            continue;
                                        }
                                        4 => {
                                            c2rust_current_block = C2Rust_Block::S_2396;
                                            continue;
                                        }
                                        _ => {}
                                    }
                                    abort();
                                } else {
                                    ::core::ptr::write_volatile(
                                        &raw mut arg_index,
                                        rboxc_result_53.value,
                                    );
                                }
                            } else {
                                let mut rboxc_expr_17: ::core::ffi::c_int =
                                    (::core::ptr::read_volatile::<::core::ffi::c_int>(
                                        &raw const arg_index,
                                    ) != argc)
                                        as ::core::ffi::c_int;
                                if rboxc_expr_17 != 0 {
                                    rboxc_expr_17 = (read_from_stdin == 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int;
                                }
                                if rboxc_expr_17 != 0 {
                                    let c2rust_fresh0 =
                                        ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                            &raw const arg_index,
                                        );
                                    ::core::ptr::write_volatile(
                                        &raw mut arg_index,
                                        ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                            &raw const arg_index,
                                        ) + 1,
                                    );
                                    shell_script_filename = *argv.offset(c2rust_fresh0 as isize);
                                    let mut rboxc_result_54: rboxc_gate_result_54 =
                                        rboxc_gate_call_54(
                                            &raw mut rboxc_gate,
                                            argv,
                                            ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                &raw const arg_index,
                                            ),
                                            argc,
                                            1 as ::core::ffi::c_int,
                                        );
                                    if rboxc_result_54.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_54.code;
                                        rboxc_gate.target = rboxc_result_54.target;
                                        match rboxc_result_54.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        ::core::ptr::write_volatile(
                                            &raw mut arg_index,
                                            rboxc_result_54.value,
                                        );
                                    }
                                } else {
                                    let mut rboxc_result_55: rboxc_gate_result_55 =
                                        rboxc_gate_call_55(
                                            &raw mut rboxc_gate,
                                            argv,
                                            ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                &raw const arg_index,
                                            ),
                                            argc,
                                            1 as ::core::ffi::c_int,
                                        );
                                    if rboxc_result_55.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_55.code;
                                        rboxc_gate.target = rboxc_result_55.target;
                                        match rboxc_result_55.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        ::core::ptr::write_volatile(
                                            &raw mut arg_index,
                                            rboxc_result_55.value,
                                        );
                                    }
                                }
                            }
                        }
                        ssh_reading_startup_files = 0 as ::core::ffi::c_int;
                        let mut rboxc_expr_18: ::core::ffi::c_int =
                            (::core::ptr::read_volatile::<::core::ffi::c_int>(
                                &raw const locally_skip_execution,
                            ) == 0 as ::core::ffi::c_int)
                                as ::core::ffi::c_int;
                        if rboxc_expr_18 != 0 {
                            rboxc_expr_18 =
                                (running_setuid == 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
                        }
                        if rboxc_expr_18 != 0 {
                            let mut t: *mut ::core::ffi::c_char =
                                ::core::ptr::null_mut::<::core::ffi::c_char>();
                            old_errexit_flag = exit_immediately_on_error;
                            exit_immediately_on_error = 0 as ::core::ffi::c_int;
                            if !shell_script_filename.is_null() {
                                t = *(&raw mut dollar_vars as *mut *mut ::core::ffi::c_char)
                                    .offset(0isize);
                                let mut rboxc_expr_19: *mut ::core::ffi::c_char =
                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                if !exec_argv0.is_null() {
                                    let mut rboxc_result_56: rboxc_gate_result_56 =
                                        rboxc_gate_call_56(&raw mut rboxc_gate, exec_argv0);
                                    if rboxc_result_56.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_56.code;
                                        rboxc_gate.target = rboxc_result_56.target;
                                        match rboxc_result_56.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        let mut rboxc_result_57: rboxc_gate_result_57 =
                                            rboxc_gate_call_57(
                                                &raw mut rboxc_gate,
                                                (1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(rboxc_result_56.value),
                                            );
                                        if rboxc_result_57.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_57.code;
                                            rboxc_gate.target = rboxc_result_57.target;
                                            match rboxc_result_57.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_58: rboxc_gate_result_58 =
                                                rboxc_gate_call_58(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_57.value
                                                        as *mut ::core::ffi::c_char,
                                                    exec_argv0,
                                                );
                                            if rboxc_result_58.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_58.code;
                                                rboxc_gate.target = rboxc_result_58.target;
                                                match rboxc_result_58.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                rboxc_expr_19 = rboxc_result_58.value;
                                            }
                                        }
                                    }
                                } else {
                                    let mut rboxc_result_59: rboxc_gate_result_59 =
                                        rboxc_gate_call_59(
                                            &raw mut rboxc_gate,
                                            shell_script_filename,
                                        );
                                    if rboxc_result_59.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_59.code;
                                        rboxc_gate.target = rboxc_result_59.target;
                                        match rboxc_result_59.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        let mut rboxc_result_60: rboxc_gate_result_60 =
                                            rboxc_gate_call_60(
                                                &raw mut rboxc_gate,
                                                (1 as ::core::ffi::c_ulong)
                                                    .wrapping_add(rboxc_result_59.value),
                                            );
                                        if rboxc_result_60.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_60.code;
                                            rboxc_gate.target = rboxc_result_60.target;
                                            match rboxc_result_60.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_61: rboxc_gate_result_61 =
                                                rboxc_gate_call_61(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_60.value
                                                        as *mut ::core::ffi::c_char,
                                                    shell_script_filename,
                                                );
                                            if rboxc_result_61.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_61.code;
                                                rboxc_gate.target = rboxc_result_61.target;
                                                match rboxc_result_61.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                rboxc_expr_19 = rboxc_result_61.value;
                                            }
                                        }
                                    }
                                }
                                *(&raw mut dollar_vars as *mut *mut ::core::ffi::c_char)
                                    .offset(0isize) = rboxc_expr_19;
                            }
                            let mut rboxc_result_62: rboxc_gate_result_62 =
                                rboxc_gate_call_62(&raw mut rboxc_gate);
                            if rboxc_result_62.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_62.code;
                                rboxc_gate.target = rboxc_result_62.target;
                                match rboxc_result_62.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                if !shell_script_filename.is_null() {
                                    let mut rboxc_result_63: rboxc_gate_result_63 =
                                        rboxc_gate_call_63(
                                            &raw mut rboxc_gate,
                                            *(&raw mut dollar_vars as *mut *mut ::core::ffi::c_char)
                                                .offset(0isize)
                                                as *mut ::core::ffi::c_void,
                                        );
                                    if rboxc_result_63.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_63.code;
                                        rboxc_gate.target = rboxc_result_63.target;
                                        match rboxc_result_63.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        *(&raw mut dollar_vars as *mut *mut ::core::ffi::c_char)
                                            .offset(0isize) = t;
                                    }
                                }
                                exit_immediately_on_error += old_errexit_flag;
                            }
                        }
                        if act_like_sh != 0 {
                            let mut rboxc_result_64: rboxc_gate_result_64 = rboxc_gate_call_64(
                                &raw mut rboxc_gate,
                                b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char,
                                b"y\0".as_ptr() as *const ::core::ffi::c_char,
                                0 as ::core::ffi::c_int,
                            );
                            if rboxc_result_64.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_64.code;
                                rboxc_gate.target = rboxc_result_64.target;
                                match rboxc_result_64.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                let mut rboxc_result_65: rboxc_gate_result_65 = rboxc_gate_call_65(
                                    &raw mut rboxc_gate,
                                    b"POSIXLY_CORRECT\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                                if rboxc_result_65.jumped != 0 {
                                    rboxc_gate.pending = rboxc_result_65.code;
                                    rboxc_gate.target = rboxc_result_65.target;
                                    match rboxc_result_65.target {
                                        1 => {
                                            break;
                                        }
                                        2 => {
                                            c2rust_current_block = C2Rust_Block::S_396;
                                            continue;
                                        }
                                        3 => {
                                            c2rust_current_block = C2Rust_Block::S_528;
                                            continue;
                                        }
                                        4 => {
                                            c2rust_current_block = C2Rust_Block::S_2396;
                                            continue;
                                        }
                                        _ => {}
                                    }
                                    abort();
                                }
                            }
                        }
                        let mut rboxc_expr_20: ::core::ffi::c_int =
                            (saverst != 0) as ::core::ffi::c_int;
                        if rboxc_expr_20 == 0 {
                            rboxc_expr_20 = (restricted != 0) as ::core::ffi::c_int;
                        }
                        restricted = rboxc_expr_20;
                        if shell_reinitialized == 0 as ::core::ffi::c_int {
                            let mut rboxc_result_66: rboxc_gate_result_66 =
                                rboxc_gate_call_66(&raw mut rboxc_gate, shell_name);
                            if rboxc_result_66.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_66.code;
                                rboxc_gate.target = rboxc_result_66.target;
                                match rboxc_result_66.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            }
                        }
                        let mut rboxc_result_67: rboxc_gate_result_67 =
                            rboxc_gate_call_67(&raw mut rboxc_gate);
                        if rboxc_result_67.jumped != 0 {
                            rboxc_gate.pending = rboxc_result_67.code;
                            rboxc_gate.target = rboxc_result_67.target;
                            match rboxc_result_67.target {
                                1 => {
                                    break;
                                }
                                2 => {
                                    c2rust_current_block = C2Rust_Block::S_396;
                                    continue;
                                }
                                3 => {
                                    c2rust_current_block = C2Rust_Block::S_528;
                                    continue;
                                }
                                4 => {
                                    c2rust_current_block = C2Rust_Block::S_2396;
                                    continue;
                                }
                                _ => {}
                            }
                            abort();
                        } else {
                            let mut rboxc_result_68: rboxc_gate_result_68 =
                                rboxc_gate_call_68(&raw mut rboxc_gate);
                            if rboxc_result_68.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_68.code;
                                rboxc_gate.target = rboxc_result_68.target;
                                match rboxc_result_68.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                if !command_execution_string.is_null() {
                                    startup_state = 2 as ::core::ffi::c_int;
                                    if debugging_mode != 0 {
                                        let mut rboxc_result_69: rboxc_gate_result_69 =
                                            rboxc_gate_call_69(&raw mut rboxc_gate);
                                        if rboxc_result_69.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_69.code;
                                            rboxc_gate.target = rboxc_result_69.target;
                                            match rboxc_result_69.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        }
                                    }
                                    shell_initialized = 1 as ::core::ffi::c_int;
                                    executing = shell_initialized;
                                    let mut rboxc_result_70: rboxc_gate_result_70 =
                                        rboxc_gate_call_70(
                                            &raw mut rboxc_gate,
                                            command_execution_string,
                                        );
                                    if rboxc_result_70.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_70.code;
                                        rboxc_gate.target = rboxc_result_70.target;
                                        match rboxc_result_70.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        let mut rboxc_result_71: rboxc_gate_result_71 =
                                            rboxc_gate_call_71(
                                                &raw mut rboxc_gate,
                                                ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                    &raw const last_command_exit_value,
                                                ),
                                            );
                                        if rboxc_result_71.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_71.code;
                                            rboxc_gate.target = rboxc_result_71.target;
                                            match rboxc_result_71.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        }
                                    }
                                }
                                if !shell_script_filename.is_null() {
                                    let mut rboxc_result_72: rboxc_gate_result_72 =
                                        rboxc_gate_call_72(
                                            &raw mut rboxc_gate,
                                            shell_script_filename,
                                        );
                                    if rboxc_result_72.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_72.code;
                                        rboxc_gate.target = rboxc_result_72.target;
                                        match rboxc_result_72.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    }
                                } else if interactive == 0 as ::core::ffi::c_int {
                                    let mut rboxc_result_73: rboxc_gate_result_73 =
                                        rboxc_gate_call_73(&raw mut rboxc_gate, stdin);
                                    if rboxc_result_73.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_73.code;
                                        rboxc_gate.target = rboxc_result_73.target;
                                        match rboxc_result_73.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        default_buffered_input = rboxc_result_73.value;
                                        read_from_stdin = 1 as ::core::ffi::c_int;
                                    }
                                } else if ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                    &raw const top_level_arg_index,
                                ) == argc
                                {
                                    read_from_stdin = 1 as ::core::ffi::c_int;
                                }
                                let mut rboxc_result_74: rboxc_gate_result_74 =
                                    rboxc_gate_call_74(&raw mut rboxc_gate);
                                if rboxc_result_74.jumped != 0 {
                                    rboxc_gate.pending = rboxc_result_74.code;
                                    rboxc_gate.target = rboxc_result_74.target;
                                    match rboxc_result_74.target {
                                        1 => {
                                            break;
                                        }
                                        2 => {
                                            c2rust_current_block = C2Rust_Block::S_396;
                                            continue;
                                        }
                                        3 => {
                                            c2rust_current_block = C2Rust_Block::S_528;
                                            continue;
                                        }
                                        4 => {
                                            c2rust_current_block = C2Rust_Block::S_2396;
                                            continue;
                                        }
                                        _ => {}
                                    }
                                    abort();
                                } else {
                                    let mut rboxc_expr_21: ::core::ffi::c_int =
                                        (debugging_mode != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_21 != 0 {
                                        rboxc_expr_21 =
                                            (::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                &raw const locally_skip_execution,
                                            ) == 0 as ::core::ffi::c_int)
                                                as ::core::ffi::c_int;
                                    }
                                    let mut rboxc_expr_22: ::core::ffi::c_int =
                                        (rboxc_expr_21 != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_22 != 0 {
                                        rboxc_expr_22 = (running_setuid == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int;
                                    }
                                    let mut rboxc_expr_24: ::core::ffi::c_int =
                                        (rboxc_expr_22 != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_24 != 0 {
                                        let mut rboxc_expr_23: ::core::ffi::c_int =
                                            (reading_shell_script != 0) as ::core::ffi::c_int;
                                        if rboxc_expr_23 == 0 {
                                            rboxc_expr_23 = (interactive_shell
                                                == 0 as ::core::ffi::c_int)
                                                as ::core::ffi::c_int;
                                        }
                                        rboxc_expr_24 = (rboxc_expr_23 != 0) as ::core::ffi::c_int;
                                    }
                                    if rboxc_expr_24 != 0 {
                                        let mut rboxc_result_75: rboxc_gate_result_75 =
                                            rboxc_gate_call_75(&raw mut rboxc_gate);
                                        if rboxc_result_75.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_75.code;
                                            rboxc_gate.target = rboxc_result_75.target;
                                            match rboxc_result_75.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        }
                                    }
                                    if interactive_shell != 0 {
                                        let mut rboxc_result_76: rboxc_gate_result_76 =
                                            rboxc_gate_call_76(&raw mut rboxc_gate);
                                        if rboxc_result_76.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_76.code;
                                            rboxc_gate.target = rboxc_result_76.target;
                                            match rboxc_result_76.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_77: rboxc_gate_result_77 =
                                                rboxc_gate_call_77(&raw mut rboxc_gate);
                                            if rboxc_result_77.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_77.code;
                                                rboxc_gate.target = rboxc_result_77.target;
                                                match rboxc_result_77.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                let mut rboxc_result_78: rboxc_gate_result_78 =
                                                    rboxc_gate_call_78(&raw mut rboxc_gate);
                                                if rboxc_result_78.jumped != 0 {
                                                    rboxc_gate.pending = rboxc_result_78.code;
                                                    rboxc_gate.target = rboxc_result_78.target;
                                                    match rboxc_result_78.target {
                                                        1 => {
                                                            break;
                                                        }
                                                        2 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_396;
                                                            continue;
                                                        }
                                                        3 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_528;
                                                            continue;
                                                        }
                                                        4 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_2396;
                                                            continue;
                                                        }
                                                        _ => {}
                                                    }
                                                    abort();
                                                } else {
                                                    let mut rboxc_expr_25: ::core::ffi::c_int =
                                                        (shell_initialized
                                                            == 0 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_int;
                                                    if rboxc_expr_25 != 0 {
                                                        rboxc_expr_25 = (history_lines_this_session
                                                            == 0 as ::core::ffi::c_int)
                                                            as ::core::ffi::c_int;
                                                    }
                                                    if rboxc_expr_25 != 0 {
                                                        let mut rboxc_result_79: rboxc_gate_result_79 = rboxc_gate_call_79(
                                                            &raw mut rboxc_gate,
                                                        );
                                                        if rboxc_result_79.jumped != 0 {
                                                            rboxc_gate.pending =
                                                                rboxc_result_79.code;
                                                            rboxc_gate.target =
                                                                rboxc_result_79.target;
                                                            match rboxc_result_79.target {
                                                                1 => {
                                                                    break;
                                                                }
                                                                2 => {
                                                                    c2rust_current_block =
                                                                        C2Rust_Block::S_396;
                                                                    continue;
                                                                }
                                                                3 => {
                                                                    c2rust_current_block =
                                                                        C2Rust_Block::S_528;
                                                                    continue;
                                                                }
                                                                4 => {
                                                                    c2rust_current_block =
                                                                        C2Rust_Block::S_2396;
                                                                    continue;
                                                                }
                                                                _ => {}
                                                            }
                                                            abort();
                                                        }
                                                    }
                                                    let mut rboxc_result_80: rboxc_gate_result_80 =
                                                        rboxc_gate_call_80(&raw mut rboxc_gate);
                                                    if rboxc_result_80.jumped != 0 {
                                                        rboxc_gate.pending = rboxc_result_80.code;
                                                        rboxc_gate.target = rboxc_result_80.target;
                                                        match rboxc_result_80.target {
                                                            1 => {
                                                                break;
                                                            }
                                                            2 => {
                                                                c2rust_current_block =
                                                                    C2Rust_Block::S_396;
                                                                continue;
                                                            }
                                                            3 => {
                                                                c2rust_current_block =
                                                                    C2Rust_Block::S_528;
                                                                continue;
                                                            }
                                                            4 => {
                                                                c2rust_current_block =
                                                                    C2Rust_Block::S_2396;
                                                                continue;
                                                            }
                                                            _ => {}
                                                        }
                                                        abort();
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    shell_initialized = 1 as ::core::ffi::c_int;
                                    let mut rboxc_expr_26: ::core::ffi::c_int =
                                        (pretty_print_mode != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_26 != 0 {
                                        rboxc_expr_26 =
                                            (interactive_shell != 0) as ::core::ffi::c_int;
                                    }
                                    if rboxc_expr_26 != 0 {
                                        let mut rboxc_result_81: rboxc_gate_result_81 = rboxc_gate_call_81(
                                            &raw mut rboxc_gate,
                                            ::core::ptr::null::<::core::ffi::c_char>(),
                                            b"pretty-printing mode ignored in interactive shells\0"
                                                .as_ptr() as *const ::core::ffi::c_char,
                                            5 as ::core::ffi::c_int,
                                        );
                                        if rboxc_result_81.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_81.code;
                                            rboxc_gate.target = rboxc_result_81.target;
                                            match rboxc_result_81.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_82: rboxc_gate_result_82 =
                                                rboxc_gate_call_82(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_81.value,
                                                );
                                            if rboxc_result_82.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_82.code;
                                                rboxc_gate.target = rboxc_result_82.target;
                                                match rboxc_result_82.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                pretty_print_mode = 0 as ::core::ffi::c_int;
                                            }
                                        }
                                    }
                                    if pretty_print_mode != 0 {
                                        let mut rboxc_result_83: rboxc_gate_result_83 =
                                            rboxc_gate_call_83(&raw mut rboxc_gate);
                                        if rboxc_result_83.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_83.code;
                                            rboxc_gate.target = rboxc_result_83.target;
                                            match rboxc_result_83.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_84: rboxc_gate_result_84 =
                                                rboxc_gate_call_84(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_83.value,
                                                );
                                            if rboxc_result_84.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_84.code;
                                                rboxc_gate.target = rboxc_result_84.target;
                                                match rboxc_result_84.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            }
                                        }
                                    }
                                    let mut rboxc_result_85: rboxc_gate_result_85 =
                                        rboxc_gate_call_85(&raw mut rboxc_gate);
                                    if rboxc_result_85.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_85.code;
                                        rboxc_gate.target = rboxc_result_85.target;
                                        match rboxc_result_85.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        let mut rboxc_result_86: rboxc_gate_result_86 =
                                            rboxc_gate_call_86(
                                                &raw mut rboxc_gate,
                                                ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                    &raw const last_command_exit_value,
                                                ),
                                            );
                                        if rboxc_result_86.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_86.code;
                                            rboxc_gate.target = rboxc_result_86.target;
                                            match rboxc_result_86.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            panic!(
                                                "Reached end of non-void function without returning"
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                C2Rust_Block::S_528 => {
                    if rboxc_gate_checkpoint(
                        &raw mut rboxc_gate,
                        3 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ) != 0
                    {
                        let mut rboxc_result_9: rboxc_gate_result_9 =
                            rboxc_gate_call_9(&raw mut rboxc_gate, 2 as ::core::ffi::c_int);
                        if rboxc_result_9.jumped != 0 {
                            rboxc_gate.pending = rboxc_result_9.code;
                            rboxc_gate.target = rboxc_result_9.target;
                            match rboxc_result_9.target {
                                1 => {
                                    break;
                                }
                                2 => {
                                    c2rust_current_block = C2Rust_Block::S_396;
                                    continue;
                                }
                                3 => {
                                    c2rust_current_block = C2Rust_Block::S_528;
                                    continue;
                                }
                                4 => {
                                    c2rust_current_block = C2Rust_Block::S_2396;
                                    continue;
                                }
                                _ => {}
                            }
                            abort();
                        }
                    }
                }
                C2Rust_Block::S_396 => {
                    if rboxc_gate_checkpoint(
                        &raw mut rboxc_gate,
                        2 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    ) != 0
                    {
                        argc = subshell_argc;
                        argv = subshell_argv;
                        env = subshell_envp;
                        sourced_env = 0 as ::core::ffi::c_int;
                    }
                    shell_reinitialized = 0 as ::core::ffi::c_int;
                    ::core::ptr::write_volatile(&raw mut arg_index, 1 as ::core::ffi::c_int);
                    if ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const arg_index) > argc
                    {
                        ::core::ptr::write_volatile(&raw mut arg_index, argc);
                    }
                    shell_script_filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
                    command_execution_string = shell_script_filename;
                    read_from_stdin = 0 as ::core::ffi::c_int;
                    ::core::ptr::write_volatile(&raw mut locally_skip_execution, read_from_stdin);
                    want_pending_command = ::core::ptr::read_volatile::<::core::ffi::c_int>(
                        &raw const locally_skip_execution,
                    );
                    default_input = stdin;
                    default_buffered_input = -1 as ::core::ffi::c_int;
                    su_shell = 0 as ::core::ffi::c_int;
                    make_login_shell = su_shell;
                    login_shell = make_login_shell;
                    rboxc_expr_2 = (shell_initialized != 0) as ::core::ffi::c_int;
                    if rboxc_expr_2 == 0 {
                        rboxc_expr_2 = !shell_name.is_null() as ::core::ffi::c_int;
                    }
                    if rboxc_expr_2 != 0 {
                        if *shell_name as ::core::ffi::c_int == '-' as ::core::ffi::c_int {
                            shell_name = shell_name.offset(1);
                        }
                        rboxc_result_8 = rboxc_gate_call_8(&raw mut rboxc_gate);
                        if rboxc_result_8.jumped == 0 {
                            c2rust_current_block = C2Rust_Block::S_528;
                            continue;
                        }
                        rboxc_gate.pending = rboxc_result_8.code;
                        rboxc_gate.target = rboxc_result_8.target;
                        match rboxc_result_8.target {
                            1 => {
                                break;
                            }
                            2 => {
                                c2rust_current_block = C2Rust_Block::S_396;
                                continue;
                            }
                            3 => {
                                c2rust_current_block = C2Rust_Block::S_528;
                                continue;
                            }
                            4 => {
                                c2rust_current_block = C2Rust_Block::S_2396;
                                continue;
                            }
                            _ => {}
                        }
                        abort();
                    }
                }
            }
            shell_environment = env;
            rboxc_result_10 = rboxc_gate_call_10(&raw mut rboxc_gate, *argv.offset(0isize));
            if rboxc_result_10.jumped != 0 {
                rboxc_gate.pending = rboxc_result_10.code;
                rboxc_gate.target = rboxc_result_10.target;
                match rboxc_result_10.target {
                    1 => {
                        break;
                    }
                    2 => {
                        c2rust_current_block = C2Rust_Block::S_396;
                        continue;
                    }
                    3 => {
                        c2rust_current_block = C2Rust_Block::S_528;
                        continue;
                    }
                    4 => {
                        c2rust_current_block = C2Rust_Block::S_2396;
                        continue;
                    }
                    _ => {}
                }
                abort();
            } else {
                rboxc_result_11 = rboxc_gate_call_11(
                    &raw mut rboxc_gate,
                    &raw mut shellstart,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                if rboxc_result_11.jumped != 0 {
                    rboxc_gate.pending = rboxc_result_11.code;
                    rboxc_gate.target = rboxc_result_11.target;
                    match rboxc_result_11.target {
                        1 => {
                            break;
                        }
                        2 => {
                            c2rust_current_block = C2Rust_Block::S_396;
                            continue;
                        }
                        3 => {
                            c2rust_current_block = C2Rust_Block::S_528;
                            continue;
                        }
                        4 => {
                            c2rust_current_block = C2Rust_Block::S_2396;
                            continue;
                        }
                        _ => {}
                    }
                    abort();
                } else {
                    shell_start_time = shellstart.tv_sec as time_t;
                    rboxc_result_12 = rboxc_gate_call_12(
                        &raw mut rboxc_gate,
                        argv,
                        ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const arg_index),
                        argc,
                    );
                    if rboxc_result_12.jumped != 0 {
                        rboxc_gate.pending = rboxc_result_12.code;
                        rboxc_gate.target = rboxc_result_12.target;
                        match rboxc_result_12.target {
                            1 => {
                                break;
                            }
                            2 => {
                                c2rust_current_block = C2Rust_Block::S_396;
                                continue;
                            }
                            3 => {
                                c2rust_current_block = C2Rust_Block::S_528;
                                continue;
                            }
                            4 => {
                                c2rust_current_block = C2Rust_Block::S_2396;
                                continue;
                            }
                            _ => {}
                        }
                        abort();
                    } else {
                        ::core::ptr::write_volatile(&raw mut arg_index, rboxc_result_12.value);
                        if want_initial_help != 0 {
                            let mut rboxc_result_13: rboxc_gate_result_13 = rboxc_gate_call_13(
                                &raw mut rboxc_gate,
                                stdout,
                                1 as ::core::ffi::c_int,
                            );
                            if rboxc_result_13.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_13.code;
                                rboxc_gate.target = rboxc_result_13.target;
                                match rboxc_result_13.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                let mut rboxc_result_14: rboxc_gate_result_14 = rboxc_gate_call_14(
                                    &raw mut rboxc_gate,
                                    0 as ::core::ffi::c_int,
                                );
                                if rboxc_result_14.jumped != 0 {
                                    rboxc_gate.pending = rboxc_result_14.code;
                                    rboxc_gate.target = rboxc_result_14.target;
                                    match rboxc_result_14.target {
                                        1 => {
                                            break;
                                        }
                                        2 => {
                                            c2rust_current_block = C2Rust_Block::S_396;
                                            continue;
                                        }
                                        3 => {
                                            c2rust_current_block = C2Rust_Block::S_528;
                                            continue;
                                        }
                                        4 => {
                                            c2rust_current_block = C2Rust_Block::S_2396;
                                            continue;
                                        }
                                        _ => {}
                                    }
                                    abort();
                                }
                            }
                        }
                        if do_version != 0 {
                            let mut rboxc_result_15: rboxc_gate_result_15 =
                                rboxc_gate_call_15(&raw mut rboxc_gate, 1 as ::core::ffi::c_int);
                            if rboxc_result_15.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_15.code;
                                rboxc_gate.target = rboxc_result_15.target;
                                match rboxc_result_15.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                let mut rboxc_result_16: rboxc_gate_result_16 = rboxc_gate_call_16(
                                    &raw mut rboxc_gate,
                                    0 as ::core::ffi::c_int,
                                );
                                if rboxc_result_16.jumped != 0 {
                                    rboxc_gate.pending = rboxc_result_16.code;
                                    rboxc_gate.target = rboxc_result_16.target;
                                    match rboxc_result_16.target {
                                        1 => {
                                            break;
                                        }
                                        2 => {
                                            c2rust_current_block = C2Rust_Block::S_396;
                                            continue;
                                        }
                                        3 => {
                                            c2rust_current_block = C2Rust_Block::S_528;
                                            continue;
                                        }
                                        4 => {
                                            c2rust_current_block = C2Rust_Block::S_2396;
                                            continue;
                                        }
                                        _ => {}
                                    }
                                    abort();
                                }
                            }
                        }
                        echo_input_at_read = verbose_flag;
                        this_command_name = shell_name;
                        rboxc_result_17 = rboxc_gate_call_17(
                            &raw mut rboxc_gate,
                            argv,
                            ::core::ptr::read_volatile::<::core::ffi::c_int>(&raw const arg_index),
                            argc,
                        );
                        if rboxc_result_17.jumped != 0 {
                            rboxc_gate.pending = rboxc_result_17.code;
                            rboxc_gate.target = rboxc_result_17.target;
                            match rboxc_result_17.target {
                                1 => {
                                    break;
                                }
                                2 => {
                                    c2rust_current_block = C2Rust_Block::S_396;
                                    continue;
                                }
                                3 => {
                                    c2rust_current_block = C2Rust_Block::S_528;
                                    continue;
                                }
                                4 => {
                                    c2rust_current_block = C2Rust_Block::S_2396;
                                    continue;
                                }
                                _ => {}
                            }
                            abort();
                        } else {
                            ::core::ptr::write_volatile(&raw mut arg_index, rboxc_result_17.value);
                            if make_login_shell != 0 {
                                login_shell += 1;
                                login_shell = -login_shell;
                            }
                            rboxc_result_18 = rboxc_gate_call_18(
                                &raw mut rboxc_gate,
                                b"login_shell\0".as_ptr() as *const ::core::ffi::c_char
                                    as *mut ::core::ffi::c_char,
                                (login_shell != 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                            );
                            if rboxc_result_18.jumped != 0 {
                                rboxc_gate.pending = rboxc_result_18.code;
                                rboxc_gate.target = rboxc_result_18.target;
                                match rboxc_result_18.target {
                                    1 => {
                                        break;
                                    }
                                    2 => {
                                        c2rust_current_block = C2Rust_Block::S_396;
                                        continue;
                                    }
                                    3 => {
                                        c2rust_current_block = C2Rust_Block::S_528;
                                        continue;
                                    }
                                    4 => {
                                        c2rust_current_block = C2Rust_Block::S_2396;
                                        continue;
                                    }
                                    _ => {}
                                }
                                abort();
                            } else {
                                if dump_po_strings != 0 {
                                    dump_translatable_strings = 1 as ::core::ffi::c_int;
                                }
                                if dump_translatable_strings != 0 {
                                    read_but_dont_execute = 1 as ::core::ffi::c_int;
                                }
                                rboxc_expr_3 = (running_setuid != 0) as ::core::ffi::c_int;
                                if rboxc_expr_3 != 0 {
                                    rboxc_expr_3 = (privileged_mode == 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int;
                                }
                                if rboxc_expr_3 != 0 {
                                    let mut rboxc_result_19: rboxc_gate_result_19 =
                                        rboxc_gate_call_19(&raw mut rboxc_gate);
                                    if rboxc_result_19.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_19.code;
                                        rboxc_gate.target = rboxc_result_19.target;
                                        match rboxc_result_19.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    }
                                }
                                if want_pending_command != 0 {
                                    command_execution_string = *argv.offset(
                                        ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                            &raw const arg_index,
                                        ) as isize,
                                    );
                                    if command_execution_string.is_null() {
                                        let mut rboxc_result_20: rboxc_gate_result_20 =
                                            rboxc_gate_call_20(
                                                &raw mut rboxc_gate,
                                                ::core::ptr::null::<::core::ffi::c_char>(),
                                                b"%s: option requires an argument\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                5 as ::core::ffi::c_int,
                                            );
                                        if rboxc_result_20.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_20.code;
                                            rboxc_gate.target = rboxc_result_20.target;
                                            match rboxc_result_20.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_21: rboxc_gate_result_21 =
                                                rboxc_gate_call_21(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_20.value,
                                                    b"-c\0".as_ptr() as *const ::core::ffi::c_char
                                                        as *mut ::core::ffi::c_char,
                                                );
                                            if rboxc_result_21.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_21.code;
                                                rboxc_gate.target = rboxc_result_21.target;
                                                match rboxc_result_21.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                let mut rboxc_result_22: rboxc_gate_result_22 =
                                                    rboxc_gate_call_22(
                                                        &raw mut rboxc_gate,
                                                        2 as ::core::ffi::c_int,
                                                    );
                                                if rboxc_result_22.jumped != 0 {
                                                    rboxc_gate.pending = rboxc_result_22.code;
                                                    rboxc_gate.target = rboxc_result_22.target;
                                                    match rboxc_result_22.target {
                                                        1 => {
                                                            break;
                                                        }
                                                        2 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_396;
                                                            continue;
                                                        }
                                                        3 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_528;
                                                            continue;
                                                        }
                                                        4 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_2396;
                                                            continue;
                                                        }
                                                        _ => {}
                                                    }
                                                    abort();
                                                }
                                            }
                                        }
                                    }
                                    ::core::ptr::write_volatile(
                                        &raw mut arg_index,
                                        ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                            &raw const arg_index,
                                        ) + 1,
                                    );
                                }
                                this_command_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
                                rboxc_expr_9 = (forced_interactive != 0) as ::core::ffi::c_int;
                                if rboxc_expr_9 == 0 {
                                    let mut rboxc_expr_4: ::core::ffi::c_int =
                                        command_execution_string.is_null() as ::core::ffi::c_int;
                                    if rboxc_expr_4 != 0 {
                                        rboxc_expr_4 = (wordexp_only == 0 as ::core::ffi::c_int)
                                            as ::core::ffi::c_int;
                                    }
                                    let mut rboxc_expr_6: ::core::ffi::c_int =
                                        (rboxc_expr_4 != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_6 != 0 {
                                        let mut rboxc_expr_5: ::core::ffi::c_int =
                                            (::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                &raw const arg_index,
                                            ) == argc)
                                                as ::core::ffi::c_int;
                                        if rboxc_expr_5 == 0 {
                                            rboxc_expr_5 =
                                                (read_from_stdin != 0) as ::core::ffi::c_int;
                                        }
                                        rboxc_expr_6 = (rboxc_expr_5 != 0) as ::core::ffi::c_int;
                                    }
                                    let mut rboxc_expr_7: ::core::ffi::c_int =
                                        (rboxc_expr_6 != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_7 != 0 {
                                        let mut rboxc_result_23: rboxc_gate_result_23 =
                                            rboxc_gate_call_23(&raw mut rboxc_gate, stdin);
                                        if rboxc_result_23.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_23.code;
                                            rboxc_gate.target = rboxc_result_23.target;
                                            match rboxc_result_23.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_24: rboxc_gate_result_24 =
                                                rboxc_gate_call_24(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_23.value,
                                                );
                                            if rboxc_result_24.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_24.code;
                                                rboxc_gate.target = rboxc_result_24.target;
                                                match rboxc_result_24.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                rboxc_expr_7 = (rboxc_result_24.value != 0)
                                                    as ::core::ffi::c_int;
                                            }
                                        }
                                    }
                                    let mut rboxc_expr_8: ::core::ffi::c_int =
                                        (rboxc_expr_7 != 0) as ::core::ffi::c_int;
                                    if rboxc_expr_8 != 0 {
                                        let mut rboxc_result_25: rboxc_gate_result_25 =
                                            rboxc_gate_call_25(&raw mut rboxc_gate, stderr);
                                        if rboxc_result_25.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_25.code;
                                            rboxc_gate.target = rboxc_result_25.target;
                                            match rboxc_result_25.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            let mut rboxc_result_26: rboxc_gate_result_26 =
                                                rboxc_gate_call_26(
                                                    &raw mut rboxc_gate,
                                                    rboxc_result_25.value,
                                                );
                                            if rboxc_result_26.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_26.code;
                                                rboxc_gate.target = rboxc_result_26.target;
                                                match rboxc_result_26.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                rboxc_expr_8 = (rboxc_result_26.value != 0)
                                                    as ::core::ffi::c_int;
                                            }
                                        }
                                    }
                                    rboxc_expr_9 = (rboxc_expr_8 != 0) as ::core::ffi::c_int;
                                }
                                if rboxc_expr_9 != 0 {
                                    let mut rboxc_result_27: rboxc_gate_result_27 =
                                        rboxc_gate_call_27(&raw mut rboxc_gate);
                                    if rboxc_result_27.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_27.code;
                                        rboxc_gate.target = rboxc_result_27.target;
                                        match rboxc_result_27.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    }
                                } else {
                                    let mut rboxc_result_28: rboxc_gate_result_28 =
                                        rboxc_gate_call_28(&raw mut rboxc_gate);
                                    if rboxc_result_28.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_28.code;
                                        rboxc_gate.target = rboxc_result_28.target;
                                        match rboxc_result_28.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    }
                                }
                                rboxc_expr_10 = (login_shell != 0) as ::core::ffi::c_int;
                                if rboxc_expr_10 != 0 {
                                    rboxc_expr_10 = (interactive_shell != 0) as ::core::ffi::c_int;
                                }
                                's_1596: {
                                    if rboxc_expr_10 != 0 {
                                        i = 3 as ::core::ffi::c_int;
                                        loop {
                                            if i >= 20 as ::core::ffi::c_int {
                                                break 's_1596;
                                            }
                                            let mut rboxc_result_29: rboxc_gate_result_29 =
                                                rboxc_gate_call_29(
                                                    &raw mut rboxc_gate,
                                                    i,
                                                    2 as ::core::ffi::c_int,
                                                    1 as ::core::ffi::c_int,
                                                );
                                            if rboxc_result_29.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_29.code;
                                                rboxc_gate.target = rboxc_result_29.target;
                                                match rboxc_result_29.target {
                                                    1 => {
                                                        continue 's_38;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue 's_396;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue 's_396;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue 's_396;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                i += 1;
                                            }
                                        }
                                    }
                                }
                                if posixly_correct != 0 {
                                    let mut rboxc_result_30: rboxc_gate_result_30 =
                                        rboxc_gate_call_30(
                                            &raw mut rboxc_gate,
                                            b"POSIXLY_CORRECT\0".as_ptr()
                                                as *const ::core::ffi::c_char,
                                            b"y\0".as_ptr() as *const ::core::ffi::c_char,
                                            0 as ::core::ffi::c_int,
                                        );
                                    if rboxc_result_30.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_30.code;
                                        rboxc_gate.target = rboxc_result_30.target;
                                        match rboxc_result_30.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        let mut rboxc_result_31: rboxc_gate_result_31 =
                                            rboxc_gate_call_31(
                                                &raw mut rboxc_gate,
                                                b"POSIXLY_CORRECT\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                            );
                                        if rboxc_result_31.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_31.code;
                                            rboxc_gate.target = rboxc_result_31.target;
                                            match rboxc_result_31.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        }
                                    }
                                }
                                if !shopt_alist.is_null() {
                                    let mut rboxc_result_32: rboxc_gate_result_32 =
                                        rboxc_gate_call_32(&raw mut rboxc_gate);
                                    if rboxc_result_32.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_32.code;
                                        rboxc_gate.target = rboxc_result_32.target;
                                        match rboxc_result_32.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    }
                                }
                                rboxc_result_33 = rboxc_gate_call_33(&raw mut rboxc_gate);
                                if rboxc_result_33.jumped != 0 {
                                    rboxc_gate.pending = rboxc_result_33.code;
                                    rboxc_gate.target = rboxc_result_33.target;
                                    match rboxc_result_33.target {
                                        1 => {
                                            break;
                                        }
                                        2 => {
                                            c2rust_current_block = C2Rust_Block::S_396;
                                            continue;
                                        }
                                        3 => {
                                            c2rust_current_block = C2Rust_Block::S_528;
                                            continue;
                                        }
                                        4 => {
                                            c2rust_current_block = C2Rust_Block::S_2396;
                                            continue;
                                        }
                                        _ => {}
                                    }
                                    abort();
                                } else {
                                    rboxc_result_34 = rboxc_gate_call_34(&raw mut rboxc_gate);
                                    if rboxc_result_34.jumped != 0 {
                                        rboxc_gate.pending = rboxc_result_34.code;
                                        rboxc_gate.target = rboxc_result_34.target;
                                        match rboxc_result_34.target {
                                            1 => {
                                                break;
                                            }
                                            2 => {
                                                c2rust_current_block = C2Rust_Block::S_396;
                                                continue;
                                            }
                                            3 => {
                                                c2rust_current_block = C2Rust_Block::S_528;
                                                continue;
                                            }
                                            4 => {
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                continue;
                                            }
                                            _ => {}
                                        }
                                        abort();
                                    } else {
                                        rboxc_result_35 = rboxc_gate_call_35(&raw mut rboxc_gate);
                                        if rboxc_result_35.jumped != 0 {
                                            rboxc_gate.pending = rboxc_result_35.code;
                                            rboxc_gate.target = rboxc_result_35.target;
                                            match rboxc_result_35.target {
                                                1 => {
                                                    break;
                                                }
                                                2 => {
                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                    continue;
                                                }
                                                3 => {
                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                    continue;
                                                }
                                                4 => {
                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                    continue;
                                                }
                                                _ => {}
                                            }
                                            abort();
                                        } else {
                                            if interactive_shell != 0 {
                                                let mut term: *mut ::core::ffi::c_char =
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                let mut emacs: *mut ::core::ffi::c_char =
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                let mut inside_emacs: *mut ::core::ffi::c_char =
                                                    ::core::ptr::null_mut::<::core::ffi::c_char>();
                                                let mut emacs_term: ::core::ffi::c_int = 0;
                                                let mut in_emacs: ::core::ffi::c_int = 0;
                                                let mut rboxc_result_36: rboxc_gate_result_36 =
                                                    rboxc_gate_call_36(
                                                        &raw mut rboxc_gate,
                                                        b"TERM\0".as_ptr()
                                                            as *const ::core::ffi::c_char,
                                                    );
                                                if rboxc_result_36.jumped != 0 {
                                                    rboxc_gate.pending = rboxc_result_36.code;
                                                    rboxc_gate.target = rboxc_result_36.target;
                                                    match rboxc_result_36.target {
                                                        1 => {
                                                            break;
                                                        }
                                                        2 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_396;
                                                            continue;
                                                        }
                                                        3 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_528;
                                                            continue;
                                                        }
                                                        4 => {
                                                            c2rust_current_block =
                                                                C2Rust_Block::S_2396;
                                                            continue;
                                                        }
                                                        _ => {}
                                                    }
                                                    abort();
                                                } else {
                                                    term = rboxc_result_36.value;
                                                    let mut rboxc_result_37: rboxc_gate_result_37 =
                                                        rboxc_gate_call_37(
                                                            &raw mut rboxc_gate,
                                                            b"EMACS\0".as_ptr()
                                                                as *const ::core::ffi::c_char,
                                                        );
                                                    if rboxc_result_37.jumped != 0 {
                                                        rboxc_gate.pending = rboxc_result_37.code;
                                                        rboxc_gate.target = rboxc_result_37.target;
                                                        match rboxc_result_37.target {
                                                            1 => {
                                                                break;
                                                            }
                                                            2 => {
                                                                c2rust_current_block =
                                                                    C2Rust_Block::S_396;
                                                                continue;
                                                            }
                                                            3 => {
                                                                c2rust_current_block =
                                                                    C2Rust_Block::S_528;
                                                                continue;
                                                            }
                                                            4 => {
                                                                c2rust_current_block =
                                                                    C2Rust_Block::S_2396;
                                                                continue;
                                                            }
                                                            _ => {}
                                                        }
                                                        abort();
                                                    } else {
                                                        emacs = rboxc_result_37.value;
                                                        let mut rboxc_result_38: rboxc_gate_result_38 = rboxc_gate_call_38(
                                                            &raw mut rboxc_gate,
                                                            b"INSIDE_EMACS\0".as_ptr() as *const ::core::ffi::c_char,
                                                        );
                                                        if rboxc_result_38.jumped != 0 {
                                                            rboxc_gate.pending =
                                                                rboxc_result_38.code;
                                                            rboxc_gate.target =
                                                                rboxc_result_38.target;
                                                            match rboxc_result_38.target {
                                                                1 => {
                                                                    break;
                                                                }
                                                                2 => {
                                                                    c2rust_current_block =
                                                                        C2Rust_Block::S_396;
                                                                    continue;
                                                                }
                                                                3 => {
                                                                    c2rust_current_block =
                                                                        C2Rust_Block::S_528;
                                                                    continue;
                                                                }
                                                                4 => {
                                                                    c2rust_current_block =
                                                                        C2Rust_Block::S_2396;
                                                                    continue;
                                                                }
                                                                _ => {}
                                                            }
                                                            abort();
                                                        } else {
                                                            inside_emacs = rboxc_result_38.value;
                                                            if !inside_emacs.is_null() {
                                                                let mut rboxc_result_39: rboxc_gate_result_39 = rboxc_gate_call_39(
                                                                    &raw mut rboxc_gate,
                                                                    inside_emacs,
                                                                    b",term:\0".as_ptr() as *const ::core::ffi::c_char,
                                                                );
                                                                if rboxc_result_39.jumped != 0 {
                                                                    rboxc_gate.pending =
                                                                        rboxc_result_39.code;
                                                                    rboxc_gate.target =
                                                                        rboxc_result_39.target;
                                                                    match rboxc_result_39.target {
                                                                        1 => {
                                                                            break;
                                                                        }
                                                                        2 => {
                                                                            c2rust_current_block =
                                                                                C2Rust_Block::S_396;
                                                                            continue;
                                                                        }
                                                                        3 => {
                                                                            c2rust_current_block =
                                                                                C2Rust_Block::S_528;
                                                                            continue;
                                                                        }
                                                                        4 => {
                                                                            c2rust_current_block = C2Rust_Block::S_2396;
                                                                            continue;
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                    abort();
                                                                } else {
                                                                    emacs_term = !rboxc_result_39
                                                                        .value
                                                                        .is_null()
                                                                        as ::core::ffi::c_int;
                                                                    in_emacs =
                                                                        1 as ::core::ffi::c_int;
                                                                }
                                                            } else if !emacs.is_null() {
                                                                let mut rboxc_result_40: rboxc_gate_result_40 = rboxc_gate_call_40(
                                                                    &raw mut rboxc_gate,
                                                                    emacs,
                                                                    b" (term:\0".as_ptr() as *const ::core::ffi::c_char,
                                                                );
                                                                if rboxc_result_40.jumped != 0 {
                                                                    rboxc_gate.pending =
                                                                        rboxc_result_40.code;
                                                                    rboxc_gate.target =
                                                                        rboxc_result_40.target;
                                                                    match rboxc_result_40.target {
                                                                        1 => {
                                                                            break;
                                                                        }
                                                                        2 => {
                                                                            c2rust_current_block =
                                                                                C2Rust_Block::S_396;
                                                                            continue;
                                                                        }
                                                                        3 => {
                                                                            c2rust_current_block =
                                                                                C2Rust_Block::S_528;
                                                                            continue;
                                                                        }
                                                                        4 => {
                                                                            c2rust_current_block = C2Rust_Block::S_2396;
                                                                            continue;
                                                                        }
                                                                        _ => {}
                                                                    }
                                                                    abort();
                                                                } else {
                                                                    emacs_term = !rboxc_result_40
                                                                        .value
                                                                        .is_null()
                                                                        as ::core::ffi::c_int;
                                                                    let mut rboxc_expr_11: ::core::ffi::c_int = (emacs_term
                                                                        != 0) as ::core::ffi::c_int;
                                                                    if rboxc_expr_11 == 0 {
                                                                        let mut rboxc_result_41: rboxc_gate_result_41 = rboxc_gate_call_41(
                                                                            &raw mut rboxc_gate,
                                                                            emacs,
                                                                            b"t\0".as_ptr() as *const ::core::ffi::c_char,
                                                                        );
                                                                        if rboxc_result_41.jumped
                                                                            != 0
                                                                        {
                                                                            rboxc_gate.pending =
                                                                                rboxc_result_41
                                                                                    .code;
                                                                            rboxc_gate.target =
                                                                                rboxc_result_41
                                                                                    .target;
                                                                            match rboxc_result_41
                                                                                .target
                                                                            {
                                                                                1 => {
                                                                                    break;
                                                                                }
                                                                                2 => {
                                                                                    c2rust_current_block = C2Rust_Block::S_396;
                                                                                    continue;
                                                                                }
                                                                                3 => {
                                                                                    c2rust_current_block = C2Rust_Block::S_528;
                                                                                    continue;
                                                                                }
                                                                                4 => {
                                                                                    c2rust_current_block = C2Rust_Block::S_2396;
                                                                                    continue;
                                                                                }
                                                                                _ => {}
                                                                            }
                                                                            abort();
                                                                        } else {
                                                                            rboxc_expr_11 = (rboxc_result_41.value != 0)
                                                                                as ::core::ffi::c_int;
                                                                        }
                                                                    }
                                                                    in_emacs = rboxc_expr_11;
                                                                }
                                                            } else {
                                                                emacs_term =
                                                                    0 as ::core::ffi::c_int;
                                                                in_emacs = emacs_term;
                                                            }
                                                            let mut rboxc_result_42: rboxc_gate_result_42 = rboxc_gate_call_42(
                                                                &raw mut rboxc_gate,
                                                                term,
                                                                b"emacs\0".as_ptr() as *const ::core::ffi::c_char,
                                                            );
                                                            if rboxc_result_42.jumped != 0 {
                                                                rboxc_gate.pending =
                                                                    rboxc_result_42.code;
                                                                rboxc_gate.target =
                                                                    rboxc_result_42.target;
                                                                match rboxc_result_42.target {
                                                                    1 => {
                                                                        break;
                                                                    }
                                                                    2 => {
                                                                        c2rust_current_block =
                                                                            C2Rust_Block::S_396;
                                                                        continue;
                                                                    }
                                                                    3 => {
                                                                        c2rust_current_block =
                                                                            C2Rust_Block::S_528;
                                                                        continue;
                                                                    }
                                                                    4 => {
                                                                        c2rust_current_block =
                                                                            C2Rust_Block::S_2396;
                                                                        continue;
                                                                    }
                                                                    _ => {}
                                                                }
                                                                abort();
                                                            } else {
                                                                no_line_editing |=
                                                                    rboxc_result_42.value;
                                                                let mut rboxc_expr_12: ::core::ffi::c_int = (in_emacs != 0)
                                                                    as ::core::ffi::c_int;
                                                                if rboxc_expr_12 != 0 {
                                                                    let mut rboxc_result_43: rboxc_gate_result_43 = rboxc_gate_call_43(
                                                                        &raw mut rboxc_gate,
                                                                        term,
                                                                        b"dumb\0".as_ptr() as *const ::core::ffi::c_char,
                                                                    );
                                                                    if rboxc_result_43.jumped != 0 {
                                                                        rboxc_gate.pending =
                                                                            rboxc_result_43.code;
                                                                        rboxc_gate.target =
                                                                            rboxc_result_43.target;
                                                                        match rboxc_result_43.target
                                                                        {
                                                                            1 => {
                                                                                break;
                                                                            }
                                                                            2 => {
                                                                                c2rust_current_block = C2Rust_Block::S_396;
                                                                                continue;
                                                                            }
                                                                            3 => {
                                                                                c2rust_current_block = C2Rust_Block::S_528;
                                                                                continue;
                                                                            }
                                                                            4 => {
                                                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                                                continue;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        abort();
                                                                    } else {
                                                                        rboxc_expr_12 = (rboxc_result_43.value != 0)
                                                                            as ::core::ffi::c_int;
                                                                    }
                                                                }
                                                                no_line_editing |= rboxc_expr_12;
                                                                let mut rboxc_expr_13: ::core::ffi::c_int = (in_emacs != 0)
                                                                    as ::core::ffi::c_int;
                                                                if rboxc_expr_13 == 0 {
                                                                    let mut rboxc_result_44: rboxc_gate_result_44 = rboxc_gate_call_44(
                                                                        &raw mut rboxc_gate,
                                                                        term,
                                                                        b"emacs\0".as_ptr() as *const ::core::ffi::c_char,
                                                                        5 as size_t,
                                                                    );
                                                                    if rboxc_result_44.jumped != 0 {
                                                                        rboxc_gate.pending =
                                                                            rboxc_result_44.code;
                                                                        rboxc_gate.target =
                                                                            rboxc_result_44.target;
                                                                        match rboxc_result_44.target
                                                                        {
                                                                            1 => {
                                                                                break;
                                                                            }
                                                                            2 => {
                                                                                c2rust_current_block = C2Rust_Block::S_396;
                                                                                continue;
                                                                            }
                                                                            3 => {
                                                                                c2rust_current_block = C2Rust_Block::S_528;
                                                                                continue;
                                                                            }
                                                                            4 => {
                                                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                                                continue;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        abort();
                                                                    } else {
                                                                        rboxc_expr_13 = (rboxc_result_44.value != 0)
                                                                            as ::core::ffi::c_int;
                                                                    }
                                                                }
                                                                running_under_emacs = rboxc_expr_13;
                                                                let mut rboxc_expr_14: ::core::ffi::c_int = (emacs_term
                                                                    != 0) as ::core::ffi::c_int;
                                                                if rboxc_expr_14 != 0 {
                                                                    let mut rboxc_result_45: rboxc_gate_result_45 = rboxc_gate_call_45(
                                                                        &raw mut rboxc_gate,
                                                                        term,
                                                                        b"eterm\0".as_ptr() as *const ::core::ffi::c_char,
                                                                        5 as size_t,
                                                                    );
                                                                    if rboxc_result_45.jumped != 0 {
                                                                        rboxc_gate.pending =
                                                                            rboxc_result_45.code;
                                                                        rboxc_gate.target =
                                                                            rboxc_result_45.target;
                                                                        match rboxc_result_45.target
                                                                        {
                                                                            1 => {
                                                                                break;
                                                                            }
                                                                            2 => {
                                                                                c2rust_current_block = C2Rust_Block::S_396;
                                                                                continue;
                                                                            }
                                                                            3 => {
                                                                                c2rust_current_block = C2Rust_Block::S_528;
                                                                                continue;
                                                                            }
                                                                            4 => {
                                                                                c2rust_current_block = C2Rust_Block::S_2396;
                                                                                continue;
                                                                            }
                                                                            _ => {}
                                                                        }
                                                                        abort();
                                                                    } else {
                                                                        rboxc_expr_14 = (rboxc_result_45.value != 0)
                                                                            as ::core::ffi::c_int;
                                                                    }
                                                                }
                                                                running_under_emacs +=
                                                                    rboxc_expr_14;
                                                                if running_under_emacs != 0 {
                                                                    gnu_error_format =
                                                                        1 as ::core::ffi::c_int;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            rboxc_result_46 =
                                                rboxc_gate_call_46(&raw mut rboxc_gate);
                                            if rboxc_result_46.jumped != 0 {
                                                rboxc_gate.pending = rboxc_result_46.code;
                                                rboxc_gate.target = rboxc_result_46.target;
                                                match rboxc_result_46.target {
                                                    1 => {
                                                        break;
                                                    }
                                                    2 => {
                                                        c2rust_current_block = C2Rust_Block::S_396;
                                                        continue;
                                                    }
                                                    3 => {
                                                        c2rust_current_block = C2Rust_Block::S_528;
                                                        continue;
                                                    }
                                                    4 => {
                                                        c2rust_current_block = C2Rust_Block::S_2396;
                                                        continue;
                                                    }
                                                    _ => {}
                                                }
                                                abort();
                                            } else {
                                                ::core::ptr::write_volatile(
                                                    &raw mut top_level_arg_index,
                                                    ::core::ptr::read_volatile::<::core::ffi::c_int>(
                                                        &raw const arg_index,
                                                    ),
                                                );
                                                old_errexit_flag = exit_immediately_on_error;
                                                c2rust_current_block = C2Rust_Block::S_2396;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

extern "C" { #[link_name = "environ"] static mut RBOXC_BASH_ENVIRON: *mut *mut ::core::ffi::c_char; }
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_bash(argc: ::core::ffi::c_int, argv: *mut *mut ::core::ffi::c_char) -> ::core::ffi::c_int {
    rboxc_bash_main_inner(argc, argv, RBOXC_BASH_ENVIRON)
}
