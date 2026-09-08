// Generated from pinned GNU wget 1.25.0 by scripts/translate-entry-provider.py.
// Source SHA-256: d5805682944ae9fe0f914441b7a3b3937b1babb89e921f8fa09e512dd92dcf2e
/* Command line parsing.
   Copyright (C) 1996-2015, 2018-2024 Free Software Foundation, Inc.

This file is part of GNU Wget.

GNU Wget is free software; you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation; either version 3 of the License, or
(at your option) any later version.

GNU Wget is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Wget.  If not, see <http://www.gnu.org/licenses/>.

Additional permission under GNU GPL version 3 section 7

If you modify this program, or any covered work, by linking or
combining it with the OpenSSL project's OpenSSL library (or a
modified version of that library), containing parts covered by the
terms of the OpenSSL or SSLeay licenses, the Free Software Foundation
grants you additional permission to convey the resulting work.
Corresponding Source for a non-source form of such a combination
shall include the source code for the parts of OpenSSL used as well
as that of the covered work.  */
#[repr(C)]
pub struct _IO_wide_data { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_codecvt { _opaque: [u8; 0] }
#[repr(C)]
pub struct _IO_marker { _opaque: [u8; 0] }
#[repr(C)]
pub struct __spawn_action { _opaque: [u8; 0] }
#[repr(C)]
pub struct hsts_store { _opaque: [u8; 0] }
#[repr(C)]
pub struct ptimer { _opaque: [u8; 0] }
use ::c2rust_bitfields;
use ::libc;
extern "C" {
    fn time(__timer: *mut time_t) -> time_t;
    fn fstat(__fd: ::core::ffi::c_int, __buf: *mut stat) -> ::core::ffi::c_int;
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
    fn setlocale(
        __category: ::core::ffi::c_int,
        __locale: *const ::core::ffi::c_char,
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
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strpbrk(
        __s: *const ::core::ffi::c_char,
        __accept: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strtok(
        __s: *mut ::core::ffi::c_char,
        __delim: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    #[link_name = "rboxc_wget_xstrdup"]
    fn xstrdup(str: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
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
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn fileno(__stream: *mut FILE) -> ::core::ffi::c_int;
    #[link_name = "rboxc_wget_logprintf"]
    fn logprintf(_: log_options, _: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_wget_debug_logprintf"]
    fn debug_logprintf(_: *const ::core::ffi::c_char, ...);
    #[link_name = "rboxc_wget_log_init"]
    fn log_init(_: *const ::core::ffi::c_char, _: bool);
    #[link_name = "rboxc_wget_redirect_output"]
    fn redirect_output(_: bool, _: *const ::core::ffi::c_char);
    #[link_name = "rboxc_wget_quote"]
    fn quote(arg: *const ::core::ffi::c_char) -> *const ::core::ffi::c_char;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn pipe(__pipedes: *mut ::core::ffi::c_int) -> ::core::ffi::c_int;
    static mut environ: *mut *mut ::core::ffi::c_char;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    static mut opterr: ::core::ffi::c_int;
    fn getpass(__prompt: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn posix_spawnp(
        __pid: *mut pid_t,
        __file: *const ::core::ffi::c_char,
        __file_actions: *const posix_spawn_file_actions_t,
        __attrp: *const posix_spawnattr_t,
        __argv: *const *mut ::core::ffi::c_char,
        __envp: *const *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_init(
        __file_actions: *mut posix_spawn_file_actions_t,
    ) -> ::core::ffi::c_int;
    fn posix_spawn_file_actions_adddup2(
        __file_actions: *mut posix_spawn_file_actions_t,
        __fd: ::core::ffi::c_int,
        __newfd: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    #[link_name = "rboxc_wget_inform_exit_status"]
    fn inform_exit_status(err: uerr_t);
    #[link_name = "rboxc_wget_get_exit_status"]
    fn get_exit_status() -> ::core::ffi::c_int;
    #[link_name = "rboxc_wget_datetime_str"]
    fn datetime_str(_: time_t) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_fork_to_background"]
    fn fork_to_background() -> bool;
    #[link_name = "rboxc_wget_file_exists_p"]
    fn file_exists_p(_: *const ::core::ffi::c_char, _: *mut file_stats_t) -> bool;
    #[link_name = "rboxc_wget_human_readable"]
    fn human_readable(
        _: wgint,
        _: ::core::ffi::c_int,
        _: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_compile_pcre2_regex"]
    fn compile_pcre2_regex(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_wget_match_pcre2_regex"]
    fn match_pcre2_regex(_: *const ::core::ffi::c_void, _: *const ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_wget_compile_posix_regex"]
    fn compile_posix_regex(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_void;
    #[link_name = "rboxc_wget_match_posix_regex"]
    fn match_posix_regex(_: *const ::core::ffi::c_void, _: *const ::core::ffi::c_char) -> bool;
    #[link_name = "rboxc_wget_print_decimal"]
    fn print_decimal(_: ::core::ffi::c_double) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_ajoin_dir_file"]
    fn ajoin_dir_file(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_wgetrc_env_file_name"]
    fn wgetrc_env_file_name() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_wgetrc_user_file_name"]
    fn wgetrc_user_file_name() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_initialize"]
    fn initialize() -> ::core::ffi::c_int;
    #[link_name = "rboxc_wget_run_command"]
    fn run_command(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_wget_setoptval"]
    fn setoptval(
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
    );
    #[link_name = "rboxc_wget_home_dir"]
    fn home_dir() -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_cleanup"]
    fn cleanup();
    #[link_name = "rboxc_wget_defaults"]
    fn defaults();
    #[link_name = "rboxc_wget_run_wgetrc"]
    fn run_wgetrc(file: *const ::core::ffi::c_char, _: *mut file_stats_t) -> bool;
    #[link_name = "rboxc_wget_url_parse"]
    fn url_parse(
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        iri: *mut iri,
        percent_encode: bool,
    ) -> *mut url;
    #[link_name = "rboxc_wget_url_error"]
    fn url_error(_: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_url_free"]
    fn url_free(_: *mut url);
    #[link_name = "rboxc_wget_url_scheme"]
    fn url_scheme_0(_: *const ::core::ffi::c_char) -> url_scheme;
    #[link_name = "rboxc_wget_scheme_leading_string"]
    fn scheme_leading_string(_: url_scheme) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_maybe_prepend_scheme"]
    fn maybe_prepend_scheme(_: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_total_downloaded_bytes"]
    static mut total_downloaded_bytes: wgint;
    #[link_name = "rboxc_wget_total_download_time"]
    static mut total_download_time: ::core::ffi::c_double;
    #[link_name = "rboxc_wget_output_stream"]
    static mut output_stream: *mut FILE;
    #[link_name = "rboxc_wget_output_stream_regular"]
    static mut output_stream_regular: bool;
    #[link_name = "rboxc_wget_retrieve_url"]
    fn retrieve_url(
        _: *mut url,
        _: *const ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: *mut *mut ::core::ffi::c_char,
        _: *const ::core::ffi::c_char,
        _: *mut ::core::ffi::c_int,
        _: bool,
        _: *mut iri,
        _: bool,
    ) -> uerr_t;
    #[link_name = "rboxc_wget_retrieve_from_file"]
    fn retrieve_from_file(
        _: *const ::core::ffi::c_char,
        _: bool,
        _: *mut ::core::ffi::c_int,
    ) -> uerr_t;
    #[link_name = "rboxc_wget_retr_rate"]
    fn retr_rate(_: wgint, _: ::core::ffi::c_double) -> *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_url_uses_proxy"]
    fn url_uses_proxy(_: *mut url) -> bool;
    #[link_name = "rboxc_wget_retrieve_tree"]
    fn retrieve_tree(_: *mut url, _: *mut iri) -> uerr_t;
    #[link_name = "rboxc_wget_set_progress_implementation"]
    fn set_progress_implementation(_: *const ::core::ffi::c_char);
    #[link_name = "rboxc_wget_progress_schedule_redirect"]
    fn progress_schedule_redirect();
    #[link_name = "rboxc_wget_progress_handle_sigwinch"]
    fn progress_handle_sigwinch(_: ::core::ffi::c_int);
    #[link_name = "rboxc_wget_convert_all_links"]
    fn convert_all_links();
    #[link_name = "rboxc_wget_print_broken_links"]
    fn print_broken_links();
    #[link_name = "rboxc_wget_hsts_store_open"]
    fn hsts_store_open(_: *const ::core::ffi::c_char) -> hsts_store_t;
    #[link_name = "rboxc_wget_hsts_store_save"]
    fn hsts_store_save(_: hsts_store_t, _: *const ::core::ffi::c_char);
    #[link_name = "rboxc_wget_hsts_store_close"]
    fn hsts_store_close(_: hsts_store_t);
    #[link_name = "rboxc_wget_hsts_store_has_changed"]
    fn hsts_store_has_changed(_: hsts_store_t) -> bool;
    #[link_name = "rboxc_wget_save_cookies"]
    fn save_cookies();
    #[link_name = "rboxc_wget_ptimer_new"]
    fn ptimer_new() -> *mut ptimer;
    #[link_name = "rboxc_wget_ptimer_destroy"]
    fn ptimer_destroy(_: *mut ptimer);
    #[link_name = "rboxc_wget_ptimer_measure"]
    fn ptimer_measure(_: *mut ptimer) -> ::core::ffi::c_double;
    #[link_name = "rboxc_wget_warc_init"]
    fn warc_init();
    #[link_name = "rboxc_wget_version_string"]
    static mut version_string: *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_compilation_string"]
    static mut compilation_string: *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_link_string"]
    static mut link_string: *const ::core::ffi::c_char;
    #[link_name = "rboxc_wget_compiled_features"]
    static mut compiled_features: [*const ::core::ffi::c_char; 0];
    #[link_name = "rboxc_wget_c_strcasecmp"]
    fn c_strcasecmp(
        s1: *const ::core::ffi::c_char,
        s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    #[link_name = "rboxc_wget_base_name"]
    fn base_name(file: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    #[link_name = "rboxc_wget_xmemdup0"]
    fn xmemdup0(p: *const ::core::ffi::c_void, s: size_t) -> *mut ::core::ffi::c_char;
    fn getopt_long(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
        __longopts: *const option,
        __longind: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
pub type __pid_t = ::core::ffi::c_int;
pub type __time_t = ::core::ffi::c_long;
pub type __blksize_t = ::core::ffi::c_long;
pub type __blkcnt_t = ::core::ffi::c_long;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type ino_t = __ino_t;
pub type dev_t = __dev_t;
pub type pid_t = __pid_t;
pub type ssize_t = isize;
pub type time_t = __time_t;
pub type size_t = usize;
pub type int64_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [::core::ffi::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type wgint = int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct options {
    pub verbose: ::core::ffi::c_int,
    pub quiet: bool,
    pub ntry: ::core::ffi::c_int,
    pub retry_connrefused: bool,
    pub retry_on_host_error: bool,
    pub retry_on_http_error: *mut ::core::ffi::c_char,
    pub background: bool,
    pub ignore_length: bool,
    pub recursive: bool,
    pub spanhost: bool,
    pub max_redirect: ::core::ffi::c_int,
    pub relative_only: bool,
    pub no_parent: bool,
    pub reclevel: ::core::ffi::c_int,
    pub dirstruct: bool,
    pub no_dirstruct: bool,
    pub cut_dirs: ::core::ffi::c_int,
    pub add_hostdir: bool,
    pub protocol_directories: bool,
    pub noclobber: bool,
    pub unlink_requested: bool,
    pub dir_prefix: *mut ::core::ffi::c_char,
    pub lfilename: *mut ::core::ffi::c_char,
    pub input_filename: *mut ::core::ffi::c_char,
    pub choose_config: *mut ::core::ffi::c_char,
    pub noconfig: bool,
    pub force_html: bool,
    pub default_page: *mut ::core::ffi::c_char,
    pub spider: bool,
    pub accepts: *mut *mut ::core::ffi::c_char,
    pub rejects: *mut *mut ::core::ffi::c_char,
    pub excludes: *mut *const ::core::ffi::c_char,
    pub includes: *mut *const ::core::ffi::c_char,
    pub ignore_case: bool,
    pub acceptregex_s: *mut ::core::ffi::c_char,
    pub rejectregex_s: *mut ::core::ffi::c_char,
    pub acceptregex: *mut ::core::ffi::c_void,
    pub rejectregex: *mut ::core::ffi::c_void,
    pub regex_type: C2Rust_Unnamed_3,
    pub regex_compile_fun:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void>,
    pub regex_match_fun: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void, *const ::core::ffi::c_char) -> bool,
    >,
    pub domains: *mut *mut ::core::ffi::c_char,
    pub exclude_domains: *mut *mut ::core::ffi::c_char,
    pub dns_cache: bool,
    pub follow_tags: *mut *mut ::core::ffi::c_char,
    pub ignore_tags: *mut *mut ::core::ffi::c_char,
    pub follow_ftp: bool,
    pub retr_symlinks: bool,
    pub output_document: *mut ::core::ffi::c_char,
    pub warc_filename: *mut ::core::ffi::c_char,
    pub warc_tempdir: *mut ::core::ffi::c_char,
    pub warc_cdx_dedup_filename: *mut ::core::ffi::c_char,
    pub warc_maxsize: wgint,
    pub warc_compression_enabled: bool,
    pub warc_digests_enabled: bool,
    pub warc_cdx_enabled: bool,
    pub warc_keep_log: bool,
    pub warc_user_headers: *mut *mut ::core::ffi::c_char,
    pub enable_xattr: bool,
    pub user: *mut ::core::ffi::c_char,
    pub passwd: *mut ::core::ffi::c_char,
    pub ask_passwd: bool,
    pub use_askpass: *mut ::core::ffi::c_char,
    pub always_rest: bool,
    pub start_pos: wgint,
    pub ftp_user: *mut ::core::ffi::c_char,
    pub ftp_passwd: *mut ::core::ffi::c_char,
    pub netrc: bool,
    pub ftp_glob: bool,
    pub ftp_pasv: bool,
    pub http_user: *mut ::core::ffi::c_char,
    pub http_passwd: *mut ::core::ffi::c_char,
    pub user_headers: *mut *mut ::core::ffi::c_char,
    pub http_keep_alive: bool,
    pub use_proxy: bool,
    pub allow_cache: bool,
    pub http_proxy: *mut ::core::ffi::c_char,
    pub ftp_proxy: *mut ::core::ffi::c_char,
    pub https_proxy: *mut ::core::ffi::c_char,
    pub no_proxy: *mut *mut ::core::ffi::c_char,
    pub base_href: *mut ::core::ffi::c_char,
    pub progress_type: *mut ::core::ffi::c_char,
    pub show_progress: ::core::ffi::c_int,
    pub noscroll: bool,
    pub proxy_user: *mut ::core::ffi::c_char,
    pub proxy_passwd: *mut ::core::ffi::c_char,
    pub read_timeout: ::core::ffi::c_double,
    pub dns_timeout: ::core::ffi::c_double,
    pub connect_timeout: ::core::ffi::c_double,
    pub random_wait: bool,
    pub wait: ::core::ffi::c_double,
    pub waitretry: ::core::ffi::c_double,
    pub use_robots: bool,
    pub limit_rate: wgint,
    pub quota: wgint,
    pub server_response: bool,
    pub save_headers: bool,
    pub content_on_error: bool,
    pub debug: bool,
    pub timestamping: bool,
    pub if_modified_since: bool,
    pub backup_converted: bool,
    pub backups: ::core::ffi::c_int,
    pub useragent: *mut ::core::ffi::c_char,
    pub referer: *mut ::core::ffi::c_char,
    pub convert_links: bool,
    pub convert_file_only: bool,
    pub remove_listing: bool,
    pub htmlify: bool,
    pub dot_style: *mut ::core::ffi::c_char,
    pub dot_bytes: wgint,
    pub dots_in_line: ::core::ffi::c_int,
    pub dot_spacing: ::core::ffi::c_int,
    pub delete_after: bool,
    pub adjust_extension: bool,
    pub page_requisites: bool,
    pub bind_address: *mut ::core::ffi::c_char,
    pub secure_protocol: C2Rust_Unnamed_2,
    pub secure_protocol_name: [::core::ffi::c_char; 8],
    pub check_cert: ::core::ffi::c_int,
    pub cert_file: *mut ::core::ffi::c_char,
    pub private_key: *mut ::core::ffi::c_char,
    pub cert_type: keyfile_type,
    pub private_key_type: keyfile_type,
    pub ca_directory: *mut ::core::ffi::c_char,
    pub ca_cert: *mut ::core::ffi::c_char,
    pub crl_file: *mut ::core::ffi::c_char,
    pub pinnedpubkey: *mut ::core::ffi::c_char,
    pub random_file: *mut ::core::ffi::c_char,
    pub egd_file: *mut ::core::ffi::c_char,
    pub https_only: bool,
    pub ftps_resume_ssl: bool,
    pub ftps_fallback_to_ftp: bool,
    pub ftps_implicit: bool,
    pub ftps_clear_data_connection: bool,
    pub tls_ciphers_string: *mut ::core::ffi::c_char,
    pub cookies: bool,
    pub cookies_input: *mut ::core::ffi::c_char,
    pub cookies_output: *mut ::core::ffi::c_char,
    pub keep_badhash: bool,
    pub keep_session_cookies: bool,
    pub post_data: *mut ::core::ffi::c_char,
    pub post_file_name: *mut ::core::ffi::c_char,
    pub method: *mut ::core::ffi::c_char,
    pub body_data: *mut ::core::ffi::c_char,
    pub body_file: *mut ::core::ffi::c_char,
    pub restrict_files_os: C2Rust_Unnamed_1,
    pub restrict_files_ctrl: bool,
    pub restrict_files_nonascii: bool,
    pub restrict_files_case: C2Rust_Unnamed_0,
    pub strict_comments: bool,
    pub preserve_perm: bool,
    pub ipv4_only: bool,
    pub ipv6_only: bool,
    pub prefer_family: C2Rust_Unnamed,
    pub content_disposition: bool,
    pub auth_without_challenge: bool,
    pub enable_iri: bool,
    pub encoding_remote: *mut ::core::ffi::c_char,
    pub locale: *const ::core::ffi::c_char,
    pub trustservernames: bool,
    pub useservertimestamps: bool,
    pub show_all_dns_entries: bool,
    pub report_bps: bool,
    pub compression: compression_options,
    pub rejected_log: *mut ::core::ffi::c_char,
    pub hsts: bool,
    pub hsts_file: *mut ::core::ffi::c_char,
    pub homedir: *const ::core::ffi::c_char,
    pub wgetrcfile: *const ::core::ffi::c_char,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct compression_options(pub ::core::ffi::c_uint);
impl compression_options {
    pub const compression_auto: Self = Self(0);
    pub const compression_gzip: Self = Self(1);
    pub const compression_none: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed {
    pub const prefer_ipv4: Self = Self(0);
    pub const prefer_ipv6: Self = Self(1);
    pub const prefer_none: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_0(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_0 {
    pub const restrict_no_case_restriction: Self = Self(0);
    pub const restrict_lowercase: Self = Self(1);
    pub const restrict_uppercase: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_1(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_1 {
    pub const restrict_unix: Self = Self(0);
    pub const restrict_vms: Self = Self(1);
    pub const restrict_windows: Self = Self(2);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct keyfile_type(pub ::core::ffi::c_uint);
impl keyfile_type {
    pub const keyfile_pem: Self = Self(0);
    pub const keyfile_asn1: Self = Self(1);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_2(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_2 {
    pub const secure_protocol_auto: Self = Self(0);
    pub const secure_protocol_sslv2: Self = Self(1);
    pub const secure_protocol_sslv3: Self = Self(2);
    pub const secure_protocol_tlsv1: Self = Self(3);
    pub const secure_protocol_tlsv1_1: Self = Self(4);
    pub const secure_protocol_tlsv1_2: Self = Self(5);
    pub const secure_protocol_tlsv1_3: Self = Self(6);
    pub const secure_protocol_pfs: Self = Self(7);
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_3(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_3 {
    pub const regex_type_pcre: Self = Self(0);
    pub const regex_type_posix: Self = Self(1);
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
pub struct log_options(pub ::core::ffi::c_uint);
impl log_options {
    pub const LOG_VERBOSE: Self = Self(0);
    pub const LOG_NOTQUIET: Self = Self(1);
    pub const LOG_NONVERBOSE: Self = Self(2);
    pub const LOG_ALWAYS: Self = Self(3);
    pub const LOG_PROGRESS: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iri {
    pub uri_encoding: *mut ::core::ffi::c_char,
    pub content_encoding: *mut ::core::ffi::c_char,
    pub orig_url: *mut ::core::ffi::c_char,
    pub utf8_encode: bool,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct uerr_t(pub ::core::ffi::c_uint);
impl uerr_t {
    pub const NOCONERROR: Self = Self(0);
    pub const HOSTERR: Self = Self(1);
    pub const CONSOCKERR: Self = Self(2);
    pub const CONERROR: Self = Self(3);
    pub const CONSSLERR: Self = Self(4);
    pub const CONIMPOSSIBLE: Self = Self(5);
    pub const NEWLOCATION: Self = Self(6);
    pub const FTPOK: Self = Self(7);
    pub const FTPLOGINC: Self = Self(8);
    pub const FTPLOGREFUSED: Self = Self(9);
    pub const FTPPORTERR: Self = Self(10);
    pub const FTPSYSERR: Self = Self(11);
    pub const FTPNSFOD: Self = Self(12);
    pub const FTPUNKNOWNTYPE: Self = Self(13);
    pub const FTPRERR: Self = Self(14);
    pub const FTPSRVERR: Self = Self(15);
    pub const FTPRETRINT: Self = Self(16);
    pub const FTPRESTFAIL: Self = Self(17);
    pub const URLERROR: Self = Self(18);
    pub const FOPENERR: Self = Self(19);
    pub const FOPEN_EXCL_ERR: Self = Self(20);
    pub const FWRITEERR: Self = Self(21);
    pub const HEOF: Self = Self(22);
    pub const GATEWAYTIMEOUT: Self = Self(23);
    pub const HERR: Self = Self(24);
    pub const RETROK: Self = Self(25);
    pub const RECLEVELEXC: Self = Self(26);
    pub const WRONGCODE: Self = Self(27);
    pub const FTPINVPASV: Self = Self(28);
    pub const FTPNOPASV: Self = Self(29);
    pub const FTPNOPBSZ: Self = Self(30);
    pub const FTPNOPROT: Self = Self(31);
    pub const FTPNOAUTH: Self = Self(32);
    pub const CONTNOTSUPPORTED: Self = Self(33);
    pub const RETRUNNEEDED: Self = Self(34);
    pub const RETRFINISHED: Self = Self(35);
    pub const READERR: Self = Self(36);
    pub const TRYLIMEXC: Self = Self(37);
    pub const FILEBADFILE: Self = Self(38);
    pub const RANGEERR: Self = Self(39);
    pub const RETRBADPATTERN: Self = Self(40);
    pub const PROXERR: Self = Self(41);
    pub const AUTHFAILED: Self = Self(42);
    pub const QUOTEXC: Self = Self(43);
    pub const WRITEFAILED: Self = Self(44);
    pub const SSLINITFAILED: Self = Self(45);
    pub const VERIFCERTERR: Self = Self(46);
    pub const UNLINKERR: Self = Self(47);
    pub const NEWLOCATION_KEEP_POST: Self = Self(48);
    pub const CLOSEFAILED: Self = Self(49);
    pub const ATTRMISSING: Self = Self(50);
    pub const UNKNOWNATTR: Self = Self(51);
    pub const WARC_ERR: Self = Self(52);
    pub const WARC_TMP_FOPENERR: Self = Self(53);
    pub const WARC_TMP_FWRITEERR: Self = Self(54);
    pub const TIMECONV_ERR: Self = Self(55);
    pub const METALINK_PARSE_ERROR: Self = Self(56);
    pub const METALINK_RETR_ERROR: Self = Self(57);
    pub const METALINK_CHKSUM_ERROR: Self = Self(58);
    pub const METALINK_SIG_ERROR: Self = Self(59);
    pub const METALINK_MISSING_RESOURCE: Self = Self(60);
    pub const RETR_WITH_METALINK: Self = Self(61);
    pub const METALINK_SIZE_ERROR: Self = Self(62);
}
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sched_param {
    pub sched_priority: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawnattr_t {
    pub __flags: ::core::ffi::c_short,
    pub __pgrp: pid_t,
    pub __sd: sigset_t,
    pub __ss: sigset_t,
    pub __sp: sched_param,
    pub __policy: ::core::ffi::c_int,
    pub __cgroup: ::core::ffi::c_int,
    pub __pad: [::core::ffi::c_int; 15],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_spawn_file_actions_t {
    pub __allocated: ::core::ffi::c_int,
    pub __used: ::core::ffi::c_int,
    pub __actions: *mut __spawn_action,
    pub __pad: [::core::ffi::c_int; 16],
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_4(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_4 {
    pub const WGET_EXIT_SUCCESS: Self = Self(0);
    pub const WGET_EXIT_GENERIC_ERROR: Self = Self(1);
    pub const WGET_EXIT_PARSE_ERROR: Self = Self(2);
    pub const WGET_EXIT_IO_FAIL: Self = Self(3);
    pub const WGET_EXIT_NETWORK_FAIL: Self = Self(4);
    pub const WGET_EXIT_SSL_AUTH_FAIL: Self = Self(5);
    pub const WGET_EXIT_SERVER_AUTH_FAIL: Self = Self(6);
    pub const WGET_EXIT_PROTOCOL_ERROR: Self = Self(7);
    pub const WGET_EXIT_SERVER_ERROR: Self = Self(8);
    pub const WGET_EXIT_UNKNOWN: Self = Self(9);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_stat_s {
    pub access_err: ::core::ffi::c_int,
    pub st_ino: ino_t,
    pub st_dev: dev_t,
}
pub type file_stats_t = file_stat_s;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct url_scheme(pub ::core::ffi::c_uint);
impl url_scheme {
    pub const SCHEME_HTTP: Self = Self(0);
    pub const SCHEME_HTTPS: Self = Self(1);
    pub const SCHEME_FTP: Self = Self(2);
    pub const SCHEME_FTPS: Self = Self(3);
    pub const SCHEME_INVALID: Self = Self(4);
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct url {
    pub url: *mut ::core::ffi::c_char,
    pub scheme: url_scheme,
    pub host: *mut ::core::ffi::c_char,
    pub port: ::core::ffi::c_int,
    pub path: *mut ::core::ffi::c_char,
    pub params: *mut ::core::ffi::c_char,
    pub query: *mut ::core::ffi::c_char,
    pub fragment: *mut ::core::ffi::c_char,
    pub dir: *mut ::core::ffi::c_char,
    pub file: *mut ::core::ffi::c_char,
    pub user: *mut ::core::ffi::c_char,
    pub passwd: *mut ::core::ffi::c_char,
}
pub type hsts_store_t = *mut hsts_store;
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
pub struct cmdline_option {
    pub long_name: [::core::ffi::c_char; 26],
    pub short_name: ::core::ffi::c_char,
    pub r#type: C2Rust_Unnamed_5,
    pub data: *const ::core::ffi::c_void,
    pub argtype: ::core::ffi::c_int,
}
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct C2Rust_Unnamed_5(pub ::core::ffi::c_uint);
impl C2Rust_Unnamed_5 {
    pub const OPT_VALUE: Self = Self(0);
    pub const OPT_BOOLEAN: Self = Self(1);
    pub const OPT_FUNCALL: Self = Self(2);
    pub const OPT__APPEND_OUTPUT: Self = Self(3);
    pub const OPT__CLOBBER: Self = Self(4);
    pub const OPT__DONT_REMOVE_LISTING: Self = Self(5);
    pub const OPT__EXECUTE: Self = Self(6);
    pub const OPT__NO: Self = Self(7);
    pub const OPT__PARENT: Self = Self(8);
}
pub const NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const __S_IFMT: ::core::ffi::c_int = 0o170000 as ::core::ffi::c_int;
pub const __LC_MESSAGES: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const __LC_ALL: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const SIGHUP: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGPIPE: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const LC_MESSAGES: ::core::ffi::c_int = __LC_MESSAGES;
pub const LC_ALL: ::core::ffi::c_int = __LC_ALL;
#[inline]
unsafe extern "C" fn c_tolower(mut c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    match c {
        65 | 66 | 67 | 68 | 69 | 70 | 71 | 72 | 73 | 74 | 75 | 76 | 77 | 78 | 79 | 80 | 81 | 82
        | 83 | 84 | 85 | 86 | 87 | 88 | 89 | 90 => {
            return c - 'A' as ::core::ffi::c_int + 'a' as ::core::ffi::c_int;
        }
        _ => return c,
    };
}
pub const STDOUT_FILENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SIGUSR1: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const SIGWINCH: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const INFINITE_RECURSION: ::core::ffi::c_int = -1 as ::core::ffi::c_int;
pub const no_argument: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const required_argument: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const optional_argument: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[export_name = "rboxc_wget_dummy_iri"]
pub static mut dummy_iri: iri = iri {
    uri_encoding: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    content_encoding: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    orig_url: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    utf8_encode: false,
};
#[export_name = "rboxc_wget_ares"]
pub static mut ares: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
#[export_name = "rboxc_wget_opt"]
pub static mut opt: options = options {
    verbose: 0,
    quiet: false,
    ntry: 0,
    retry_connrefused: false,
    retry_on_host_error: false,
    retry_on_http_error: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    background: false,
    ignore_length: false,
    recursive: false,
    spanhost: false,
    max_redirect: 0,
    relative_only: false,
    no_parent: false,
    reclevel: 0,
    dirstruct: false,
    no_dirstruct: false,
    cut_dirs: 0,
    add_hostdir: false,
    protocol_directories: false,
    noclobber: false,
    unlink_requested: false,
    dir_prefix: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    lfilename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    input_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    choose_config: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    noconfig: false,
    force_html: false,
    default_page: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    spider: false,
    accepts: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    rejects: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    excludes: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    includes: ::core::ptr::null_mut::<*const ::core::ffi::c_char>(),
    ignore_case: false,
    acceptregex_s: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    rejectregex_s: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    acceptregex: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    rejectregex: ::core::ptr::null_mut::<::core::ffi::c_void>(),
    regex_type: C2Rust_Unnamed_3::regex_type_pcre,
    regex_compile_fun: None,
    regex_match_fun: None,
    domains: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    exclude_domains: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    dns_cache: false,
    follow_tags: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    ignore_tags: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    follow_ftp: false,
    retr_symlinks: false,
    output_document: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    warc_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    warc_tempdir: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    warc_cdx_dedup_filename: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    warc_maxsize: 0,
    warc_compression_enabled: false,
    warc_digests_enabled: false,
    warc_cdx_enabled: false,
    warc_keep_log: false,
    warc_user_headers: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    enable_xattr: false,
    user: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ask_passwd: false,
    use_askpass: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    always_rest: false,
    start_pos: 0,
    ftp_user: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ftp_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    netrc: false,
    ftp_glob: false,
    ftp_pasv: false,
    http_user: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    http_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    user_headers: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    http_keep_alive: false,
    use_proxy: false,
    allow_cache: false,
    http_proxy: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ftp_proxy: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    https_proxy: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    no_proxy: ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    base_href: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    progress_type: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    show_progress: 0,
    noscroll: false,
    proxy_user: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    proxy_passwd: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    read_timeout: 0.,
    dns_timeout: 0.,
    connect_timeout: 0.,
    random_wait: false,
    wait: 0.,
    waitretry: 0.,
    use_robots: false,
    limit_rate: 0,
    quota: 0,
    server_response: false,
    save_headers: false,
    content_on_error: false,
    debug: false,
    timestamping: false,
    if_modified_since: false,
    backup_converted: false,
    backups: 0,
    useragent: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    referer: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    convert_links: false,
    convert_file_only: false,
    remove_listing: false,
    htmlify: false,
    dot_style: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    dot_bytes: 0,
    dots_in_line: 0,
    dot_spacing: 0,
    delete_after: false,
    adjust_extension: false,
    page_requisites: false,
    bind_address: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    secure_protocol: C2Rust_Unnamed_2::secure_protocol_auto,
    secure_protocol_name: [0; 8],
    check_cert: 0,
    cert_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    private_key: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    cert_type: keyfile_type::keyfile_pem,
    private_key_type: keyfile_type::keyfile_pem,
    ca_directory: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    ca_cert: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    crl_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    pinnedpubkey: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    random_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    egd_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    https_only: false,
    ftps_resume_ssl: false,
    ftps_fallback_to_ftp: false,
    ftps_implicit: false,
    ftps_clear_data_connection: false,
    tls_ciphers_string: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    cookies: false,
    cookies_input: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    cookies_output: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    keep_badhash: false,
    keep_session_cookies: false,
    post_data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    post_file_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    method: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    body_data: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    body_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    restrict_files_os: C2Rust_Unnamed_1::restrict_unix,
    restrict_files_ctrl: false,
    restrict_files_nonascii: false,
    restrict_files_case: C2Rust_Unnamed_0::restrict_no_case_restriction,
    strict_comments: false,
    preserve_perm: false,
    ipv4_only: false,
    ipv6_only: false,
    prefer_family: C2Rust_Unnamed::prefer_ipv4,
    content_disposition: false,
    auth_without_challenge: false,
    enable_iri: false,
    encoding_remote: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    locale: ::core::ptr::null::<::core::ffi::c_char>(),
    trustservernames: false,
    useservertimestamps: false,
    show_all_dns_entries: false,
    report_bps: false,
    compression: compression_options::compression_auto,
    rejected_log: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    hsts: false,
    hsts_file: ::core::ptr::null_mut::<::core::ffi::c_char>(),
    homedir: ::core::ptr::null::<::core::ffi::c_char>(),
    wgetrcfile: ::core::ptr::null::<::core::ffi::c_char>(),
};
pub const MAX_CHARS_PER_LINE: ::core::ffi::c_int = 72 as ::core::ffi::c_int;
pub const TABULATION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[export_name = "rboxc_wget_exec_name"]
pub static mut exec_name: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_wget_numurls"]
pub static mut numurls: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn redirect_output_signal(mut sig: ::core::ffi::c_int) {
    let mut signal_name: *const ::core::ffi::c_char =
        b"WTF?!\0".as_ptr() as *const ::core::ffi::c_char;
    if sig == SIGHUP {
        signal_name = b"SIGHUP\0".as_ptr() as *const ::core::ffi::c_char;
    }
    if sig == SIGUSR1 {
        signal_name = b"SIGUSR1\0".as_ptr() as *const ::core::ffi::c_char;
    }
    redirect_output(r#true != 0, signal_name);
    progress_schedule_redirect();
    signal(
        sig,
        Some(redirect_output_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
}
unsafe extern "C" fn i18n_initialize() {
    setlocale(LC_ALL, b"\0".as_ptr() as *const ::core::ffi::c_char);
    bindtextdomain(
        b"wget\0".as_ptr() as *const ::core::ffi::c_char,
        LOCALEDIR.as_ptr(),
    );
    bindtextdomain(
        b"wget-gnulib\0".as_ptr() as *const ::core::ffi::c_char,
        LOCALEDIR.as_ptr(),
    );
    textdomain(b"wget\0".as_ptr() as *const ::core::ffi::c_char);
}
#[export_name = "rboxc_wget_hsts_store"]
pub static mut hsts_store: hsts_store_t = ::core::ptr::null_mut::<hsts_store>();
unsafe extern "C" fn get_hsts_database() -> *mut ::core::ffi::c_char {
    if !opt.hsts_file.is_null() {
        return xstrdup(opt.hsts_file);
    }
    if !opt.homedir.is_null() {
        let mut dir: *mut ::core::ffi::c_char = ajoin_dir_file(
            opt.homedir,
            b".wget-hsts\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return dir;
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
unsafe extern "C" fn load_hsts() {
    if hsts_store.is_null() {
        let mut filename: *mut ::core::ffi::c_char = get_hsts_database();
        if !filename.is_null() {
            if opt.debug as ::core::ffi::c_long != 0 {
                debug_logprintf(
                    b"Reading HSTS entries from %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                );
            }
            hsts_store = hsts_store_open(filename);
            if hsts_store.is_null() {
                logprintf(
                    log_options::LOG_NOTQUIET,
                    b"ERROR: could not open HSTS store at '%s'. HSTS will be disabled.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    filename,
                );
            }
        } else {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"ERROR: could not open HSTS store. HSTS will be disabled.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
        free(filename as *mut ::core::ffi::c_void);
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
unsafe extern "C" fn save_hsts() {
    if !hsts_store.is_null() {
        let mut filename: *mut ::core::ffi::c_char = get_hsts_database();
        if !filename.is_null() && hsts_store_has_changed(hsts_store) as ::core::ffi::c_int != 0 {
            if opt.debug as ::core::ffi::c_long != 0 {
                debug_logprintf(
                    b"Saving HSTS entries to %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    filename,
                );
            }
            hsts_store_save(hsts_store, filename);
        }
        hsts_store_close(hsts_store);
        free(hsts_store as *mut ::core::ffi::c_void);
        hsts_store = ::core::ptr::null_mut::<hsts_store>();
        free(filename as *mut ::core::ffi::c_void);
        filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
}
static mut option_data: [cmdline_option; 164] = unsafe {
    [
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"accept\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'A' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"accept\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"accept-regex\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"acceptregex\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"adjust-extension\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'E' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"adjustextension\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"append-output\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'a' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT__APPEND_OUTPUT,
            data: ::core::ptr::null::<::core::ffi::c_void>(),
            argtype: required_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ask-password\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"askpassword\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"auth-no-challenge\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"authnochallenge\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"background\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'b' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"background\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"backup-converted\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'K' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"backupconverted\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"backups\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"backups\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"base\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'B' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"base\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"bind-address\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"bindaddress\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"body-data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"bodydata\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"body-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"bodyfile\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ca-certificate\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"cacertificate\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ca-directory\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"cadirectory\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"cache\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"certificate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"certificate\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"certificate-type\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"certificatetype\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"check-certificate\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"checkcertificate\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT__CLOBBER,
            data: ::core::ptr::null::<::core::ffi::c_void>(),
            argtype: optional_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"compression\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"compression\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"chooseconfig\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"connect-timeout\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"connecttimeout\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"continue\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'c' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"continue\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"convert-file-only\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"convertfileonly\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"convert-links\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'k' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"convertlinks\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"content-disposition\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"contentdisposition\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"content-on-error\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"contentonerror\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"cookies\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"crl-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"crlfile\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"cut-dirs\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"cutdirs\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"debug\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'd' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"debug\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"default-page\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"defaultpage\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"delete-after\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"deleteafter\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"directories\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"dirstruct\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"directory-prefix\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'P' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"dirprefix\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"dns-cache\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"dnscache\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"dns-timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"dnstimeout\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"domains\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'D' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"domains\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"dont-remove-listing\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT__DONT_REMOVE_LISTING,
            data: ::core::ptr::null::<::core::ffi::c_void>(),
            argtype: no_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"dot-style\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"dotstyle\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"egd-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"egdfile\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"exclude-directories\0\0\0\0\0\0\0",
            ),
            short_name: 'X' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"excludedirectories\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"exclude-domains\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"excludedomains\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"execute\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'e' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT__EXECUTE,
            data: ::core::ptr::null::<::core::ffi::c_void>(),
            argtype: required_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"follow-ftp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"followftp\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"follow-tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"followtags\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"force-directories\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'x' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"dirstruct\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"force-html\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'F' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"forcehtml\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ftp-password\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"ftppassword\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ftp-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"ftpuser\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ftps-clear-data-connection",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ftpscleardataconnection\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ftps-fallback-to-ftp\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ftpsfallbacktoftp\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ftps-implicit\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ftpsimplicit\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ftps-resume-ssl\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ftpsresumessl\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"glob\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"glob\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"header\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"help\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'h' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_FUNCALL,
            data: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                *mut ::core::ffi::c_void,
            >(Some(print_help as unsafe extern "C" fn() -> ())),
            argtype: no_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"host-directories\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"addhostdir\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"hsts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"hsts\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"hsts-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"hstsfile\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"html-extension\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'E' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"adjustextension\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"htmlify\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"htmlify\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"http-keep-alive\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"httpkeepalive\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"http-passwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"httppassword\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"http-password\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"httppassword\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"http-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"httpuser\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"https-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"httpsonly\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ignore-case\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ignorecase\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ignore-length\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ignorelength\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ignore-tags\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"ignoretags\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"include-directories\0\0\0\0\0\0\0",
            ),
            short_name: 'I' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"includedirectories\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"inet4-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: '4' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"inet4only\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"inet6-only\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: '6' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"inet6only\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"input-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'i' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"input\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"iri\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"iri\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"keep-badhash\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"keepbadhash\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"keep-session-cookies\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"keepsessioncookies\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"level\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'l' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"reclevel\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"limit-rate\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"limitrate\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"load-cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"loadcookies\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"local-encoding\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"localencoding\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"rejected-log\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"rejectedlog\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"max-redirect\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"maxredirect\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"method\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"method\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"mirror\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'm' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"mirror\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"netrc\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"netrc\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"no\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'n' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT__NO,
            data: ::core::ptr::null::<::core::ffi::c_void>(),
            argtype: required_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"no-clobber\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"noclobber\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"no-config\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"noconfig\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"no-parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"noparent\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"output-document\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'O' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"outputdocument\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"output-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'o' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"logfile\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"page-requisites\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'p' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"pagerequisites\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"parent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT__PARENT,
            data: ::core::ptr::null::<::core::ffi::c_void>(),
            argtype: optional_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"passive-ftp\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"passiveftp\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"password\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"password\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"pinnedpubkey\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"pinnedpubkey\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"post-data\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"postdata\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"post-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"postfile\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"prefer-family\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"preferfamily\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"preserve-permissions\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"preservepermissions\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"ciphers\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"ciphers\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"private-key\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"privatekey\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"private-key-type\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"privatekeytype\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"progress\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"progress\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"show-progress\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"showprogress\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"protocol-directories\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"protocoldirectories\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"proxy\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"useproxy\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"proxy__compat\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'Y' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"useproxy\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"proxy-passwd\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"proxypassword\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"proxy-password\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"proxypassword\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"proxy-user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"proxyuser\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"quiet\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'q' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"quiet\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"quota\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'Q' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"quota\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"random-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"randomfile\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"random-wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"randomwait\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"read-timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"readtimeout\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"recursive\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'r' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"recursive\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"referer\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"referer\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"regex-type\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"regextype\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"reject\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'R' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"reject\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"reject-regex\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"rejectregex\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"relative\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'L' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"relativeonly\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"remote-encoding\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"remoteencoding\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"remove-listing\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"removelisting\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"report-speed\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"reportspeed\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"restrict-file-names\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"restrictfilenames\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"retr-symlinks\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"retrsymlinks\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"retry-connrefused\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"retryconnrefused\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"retry-on-host-error\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"retryonhosterror\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"retry-on-http-error\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"retryonhttperror\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"save-cookies\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"savecookies\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"save-headers\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"saveheaders\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"secure-protocol\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"secureprotocol\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"server-response\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'S' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"serverresponse\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"span-hosts\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'H' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"spanhosts\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"spider\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"spider\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"start-pos\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"startpos\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"strict-comments\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"strictcomments\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"timeout\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'T' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"timeout\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"timestamping\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'N' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"timestamping\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"if-modified-since\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"ifmodifiedsince\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"tries\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 't' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"tries\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"unlink\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"unlink\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"trust-server-names\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"trustservernames\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"use-askpass\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"useaskpass\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"use-server-timestamps\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"useservertimestamps\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"user\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"user\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"user-agent\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'U' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"useragent\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"verbose\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'v' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"verbose\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"version\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'V' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_FUNCALL,
            data: ::core::mem::transmute::<
                Option<unsafe extern "C" fn() -> ()>,
                *mut ::core::ffi::c_void,
            >(Some(print_version as unsafe extern "C" fn() -> ())),
            argtype: no_argument,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"wait\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 'w' as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"wait\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"waitretry\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"waitretry\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-cdx\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"warccdx\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-compression\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"warccompression\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-dedup\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"warccdxdedup\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-digests\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"warcdigests\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-file\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"warcfile\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-header\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"warcheader\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-keep-log\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"warckeeplog\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-max-size\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"warcmaxsize\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"warc-tempdir\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_VALUE,
            data: b"warctempdir\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
        cmdline_option {
            long_name: ::core::mem::transmute::<[u8; 26], [::core::ffi::c_char; 26]>(
                *b"xattr\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
            ),
            short_name: 0 as ::core::ffi::c_char,
            r#type: C2Rust_Unnamed_5::OPT_BOOLEAN,
            data: b"xattr\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            argtype: -1 as ::core::ffi::c_int,
        },
    ]
};
unsafe extern "C" fn no_prefix(mut s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char {
    static mut buffer: [::core::ffi::c_char; 2048] = [0; 2048];
    static mut p: *mut ::core::ffi::c_char =
        unsafe { &raw const buffer as *mut ::core::ffi::c_char };
    let mut cp: *mut ::core::ffi::c_char = p;
    let mut size: ::core::ffi::c_int = (3 as size_t)
        .wrapping_add(strlen(s))
        .wrapping_add(1 as size_t) as ::core::ffi::c_int;
    *cp.offset(0isize) = 'n' as ::core::ffi::c_char;
    *cp.offset(1isize) = 'o' as ::core::ffi::c_char;
    *cp.offset(2isize) = '-' as ::core::ffi::c_char;
    strcpy(cp.offset(3 as ::core::ffi::c_int as isize), s);
    p = p.offset(size as isize);
    return cp;
}
static mut long_options: [option; 329] = [option {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    has_arg: 0,
    flag: ::core::ptr::null_mut::<::core::ffi::c_int>(),
    val: 0,
}; 329];
static mut short_options: [::core::ffi::c_char; 128] = [0; 128];
static mut optmap: [::core::ffi::c_uchar; 96] = [0; 96];
pub const BOOLEAN_NEG_MARKER: ::core::ffi::c_int = 1024 as ::core::ffi::c_int;
unsafe extern "C" fn init_switches() {
    static mut initialized: bool = false;
    let mut p: *mut ::core::ffi::c_char = &raw mut short_options as *mut ::core::ffi::c_char;
    let mut i: size_t = 0;
    let mut o: size_t = 0 as size_t;
    if initialized {
        return;
    }
    initialized = true;
    i = 0 as size_t;
    while i < ::core::mem::size_of::<[cmdline_option; 164]>()
        .wrapping_div(::core::mem::size_of::<cmdline_option>())
    {
        let mut cmdopt: *mut cmdline_option =
            (&raw mut option_data as *mut cmdline_option).offset(i as isize);
        let mut longopt: *mut option = ::core::ptr::null_mut::<option>();
        let c2rust_fresh0 = o;
        o = o.wrapping_add(1);
        longopt = (&raw mut long_options as *mut option).offset(c2rust_fresh0 as isize);
        (*longopt).name = &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char;
        (*longopt).val = i as ::core::ffi::c_int;
        if (*cmdopt).short_name != 0 {
            let c2rust_fresh1 = p;
            p = p.offset(1);
            *c2rust_fresh1 = (*cmdopt).short_name;
            optmap[((*cmdopt).short_name as ::core::ffi::c_int - 32 as ::core::ffi::c_int)
                as usize] =
                longopt.offset_from(&raw mut long_options as *mut option) as ::core::ffi::c_uchar;
        }
        match (*cmdopt).r#type {
            C2Rust_Unnamed_5::OPT_VALUE => {
                (*longopt).has_arg = required_argument;
                if (*cmdopt).short_name != 0 {
                    let c2rust_fresh2 = p;
                    p = p.offset(1);
                    *c2rust_fresh2 = ':' as ::core::ffi::c_char;
                }
            }
            C2Rust_Unnamed_5::OPT_BOOLEAN => {
                (*longopt).has_arg = optional_argument;
                let c2rust_fresh3 = o;
                o = o.wrapping_add(1);
                longopt = (&raw mut long_options as *mut option).offset(c2rust_fresh3 as isize);
                (*longopt).name =
                    no_prefix(&raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char);
                (*longopt).has_arg = no_argument;
                (*longopt).val = (i | BOOLEAN_NEG_MARKER as size_t) as ::core::ffi::c_int;
            }
            _ => {
                (*longopt).has_arg = (*cmdopt).argtype;
                if (*cmdopt).short_name != 0 {
                    if (*longopt).has_arg == required_argument {
                        let c2rust_fresh4 = p;
                        p = p.offset(1);
                        *c2rust_fresh4 = ':' as ::core::ffi::c_char;
                    }
                }
            }
        }
        i = i.wrapping_add(1);
    }
    *p = '\0' as ::core::ffi::c_char;
}
unsafe extern "C" fn print_usage(mut error: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return fprintf(
        if error != 0 { stderr } else { stdout },
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Usage: %s [OPTION]... [URL]...\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        exec_name,
    );
}
unsafe extern "C" fn print_help() {
    static mut help: [*const ::core::ffi::c_char; 179] = [
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Mandatory arguments to long options are mandatory for short options too.\n\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"Startup:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -V,  --version                   display the version of Wget and exit\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -h,  --help                      print this help\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -b,  --background                go to background after startup\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -e,  --execute=COMMAND           execute a `.wgetrc'-style command\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Logging and input file:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -o,  --output-file=FILE          log messages to FILE\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -a,  --append-output=FILE        append messages to FILE\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -d,  --debug                     print lots of debugging information\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -q,  --quiet                     quiet (no output)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -v,  --verbose                   be verbose (this is the default)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -nv, --no-verbose                turn off verboseness, without being quiet\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --report-speed=TYPE         output bandwidth as TYPE.  TYPE can be bits\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -i,  --input-file=FILE           download URLs found in local or external FILE\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -F,  --force-html                treat input file as HTML\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -B,  --base=URL                  resolves HTML input-file links (-i -F)\n                                     relative to URL\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --config=FILE               specify config file to use\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-config                 do not read any config file\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --rejected-log=FILE         log reasons for URL rejection to FILE\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Download:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -t,  --tries=NUMBER              set number of retries to NUMBER (0 unlimits)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --retry-connrefused         retry even if connection is refused\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --retry-on-host-error       consider host errors as non-fatal, transient errors\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --retry-on-http-error=ERRORS    comma-separated list of HTTP errors to retry\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -O,  --output-document=FILE      write documents to FILE\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -nc, --no-clobber                skip downloads that would download to\n                                     existing files (overwriting them)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-netrc                  don't try to obtain credentials from .netrc\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -c,  --continue                  resume getting a partially-downloaded file\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --start-pos=OFFSET          start downloading from zero-based position OFFSET\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --progress=TYPE             select progress gauge type\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --show-progress             display the progress bar in any verbosity mode\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -N,  --timestamping              don't re-retrieve files unless newer than\n                                     local\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-if-modified-since      don't use conditional if-modified-since get\n                                     requests in timestamping mode\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-use-server-timestamps  don't set the local file's timestamp by\n                                     the one on the server\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -S,  --server-response           print server response\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --spider                    don't download anything\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -T,  --timeout=SECONDS           set all timeout values to SECONDS\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --dns-timeout=SECS          set the DNS lookup timeout to SECS\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --connect-timeout=SECS      set the connect timeout to SECS\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --read-timeout=SECS         set the read timeout to SECS\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -w,  --wait=SECONDS              wait SECONDS between retrievals\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --waitretry=SECONDS         wait 1..SECONDS between retries of a retrieval\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --random-wait               wait from 0.5*WAIT...1.5*WAIT secs between retrievals\n                                     (applies if more then 1 URL is to be retrieved)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-proxy                  explicitly turn off proxy\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -Q,  --quota=NUMBER              set retrieval quota to NUMBER\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --bind-address=ADDRESS      bind to ADDRESS (hostname or IP) on local host\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --limit-rate=RATE           limit download rate to RATE\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-dns-cache              disable caching DNS lookups\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --restrict-file-names=OS    restrict chars in file names to ones OS allows\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ignore-case               ignore case when matching files/directories\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -4,  --inet4-only                connect only to IPv4 addresses\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -6,  --inet6-only                connect only to IPv6 addresses\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --prefer-family=FAMILY      connect first to addresses of specified family,\n                                     one of IPv6, IPv4, or none\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --user=USER                 set both ftp and http user to USER\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --password=PASS             set both ftp and http password to PASS\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ask-password              prompt for passwords\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --use-askpass=COMMAND       specify credential handler for requesting \n                                     username and password.  If no COMMAND is \n                                     specified the WGET_ASKPASS or the SSH_ASKPASS \n                                     environment variable is used.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-iri                    turn off IRI support\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --local-encoding=ENC        use ENC as the local encoding for IRIs\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --remote-encoding=ENC       use ENC as the default remote encoding\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --unlink                    remove file before clobber\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --xattr                     turn on storage of metadata in extended file attributes\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Directories:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -nd, --no-directories            don't create directories\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -x,  --force-directories         force creation of directories\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -nH, --no-host-directories       don't create host directories\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --protocol-directories      use protocol name in directories\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -P,  --directory-prefix=PREFIX   save files to PREFIX/..\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --cut-dirs=NUMBER           ignore NUMBER remote directory components\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"HTTP options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --http-user=USER            set http user to USER\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --http-password=PASS        set http password to PASS\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-cache                  disallow server-cached data\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --default-page=NAME         change the default page name (normally\n                                     this is 'index.html'.)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -E,  --adjust-extension          save HTML/CSS documents with proper extensions\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ignore-length             ignore 'Content-Length' header field\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --header=STRING             insert STRING among the headers\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --compression=TYPE          choose compression, one of auto, gzip and none. (default: none)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --max-redirect              maximum redirections allowed per page\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --proxy-user=USER           set USER as proxy username\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --proxy-password=PASS       set PASS as proxy password\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --referer=URL               include 'Referer: URL' header in HTTP request\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --save-headers              save the HTTP headers to file\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -U,  --user-agent=AGENT          identify as AGENT instead of Wget/VERSION\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-http-keep-alive        disable HTTP keep-alive (persistent connections)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-cookies                don't use cookies\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --load-cookies=FILE         load cookies from FILE before session\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --save-cookies=FILE         save cookies to FILE after session\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --keep-session-cookies      load and save session (non-permanent) cookies\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --post-data=STRING          use the POST method; send STRING as the data\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --post-file=FILE            use the POST method; send contents of FILE\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --method=HTTPMethod         use method \"HTTPMethod\" in the request\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --body-data=STRING          send STRING as data. --method MUST be set\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --body-file=FILE            send contents of FILE. --method MUST be set\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --content-disposition       honor the Content-Disposition header when\n                                     choosing local file names (EXPERIMENTAL)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --content-on-error          output the received content on server errors\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --auth-no-challenge         send Basic HTTP authentication information\n                                     without first waiting for the server's\n                                     challenge\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"HTTPS (SSL/TLS) options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --secure-protocol=PR        choose secure protocol, one of auto, SSLv2,\n                                     SSLv3, TLSv1, TLSv1_1, TLSv1_2, TLSv1_3 and PFS\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --https-only                only follow secure HTTPS links\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-check-certificate      don't validate the server's certificate\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --certificate=FILE          client certificate file\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --certificate-type=TYPE     client certificate type, PEM or DER\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --private-key=FILE          private key file\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --private-key-type=TYPE     private key type, PEM or DER\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --ca-certificate=FILE       file with the bundle of CAs\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --ca-directory=DIR          directory where hash list of CAs is stored\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --crl-file=FILE             file with bundle of CRLs\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --pinnedpubkey=FILE/HASHES  Public key (PEM/DER) file, or any number\n                                   of base64 encoded sha256 hashes preceded by\n                                   'sha256//' and separated by ';', to verify\n                                   peer against\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --random-file=FILE          file with random data for seeding the SSL PRNG\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --ciphers=STR           Set the priority string (GnuTLS) or cipher list string (OpenSSL) directly.\n                                   Use with care. This option overrides --secure-protocol.\n                                   The format and syntax of this string depend on the specific SSL/TLS engine.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"HSTS options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --no-hsts                   disable HSTS\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --hsts-file                 path of HSTS database (will override default)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"FTP options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --ftp-user=USER             set ftp user to USER\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --ftp-password=PASS         set ftp password to PASS\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-remove-listing         don't remove '.listing' files\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-glob                   turn off FTP file name globbing\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-passive-ftp            disable the \"passive\" transfer mode\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --preserve-permissions      preserve remote file permissions\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --retr-symlinks             when recursing, get linked-to files (not dir)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"FTPS options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --ftps-implicit                 use implicit FTPS (default port is 990)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ftps-resume-ssl               resume the SSL/TLS session started in the control connection when\n                                         opening a data connection\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ftps-clear-data-connection    cipher the control channel only; all the data will be in plaintext\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ftps-fallback-to-ftp          fall back to FTP if FTPS is not supported in the target server\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"WARC options:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"       --warc-file=FILENAME        save request/response data to a .warc.gz file\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --warc-header=STRING        insert STRING into the warcinfo record\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --warc-max-size=NUMBER      set maximum size of WARC files to NUMBER\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --warc-cdx                  write CDX index files\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --warc-dedup=FILENAME       do not store records listed in this CDX file\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-warc-compression       do not compress WARC files with GZIP\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --no-warc-digests           do not calculate SHA1 digests\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --no-warc-keep-log          do not store the log file in a WARC record\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --warc-tempdir=DIRECTORY    location for temporary files created by the\n                                     WARC writer\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Recursive download:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -r,  --recursive                 specify recursive download\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -l,  --level=NUMBER              maximum recursion depth (inf or 0 for infinite)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --delete-after              delete files locally after downloading them\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -k,  --convert-links             make links in downloaded HTML or CSS point to\n                                     local files\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --convert-file-only         convert the file part of the URLs only (usually known as the basename)\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --backups=N                 before writing file X, rotate up to N backup files\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -K,  --backup-converted          before converting file X, back up as X.orig\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -m,  --mirror                    shortcut for -N -r -l inf --no-remove-listing\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -p,  --page-requisites           get all images, etc. needed to display HTML page\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --strict-comments           turn on strict (SGML) handling of HTML comments\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Recursive accept/reject:\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"  -A,  --accept=LIST               comma-separated list of accepted extensions\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -R,  --reject=LIST               comma-separated list of rejected extensions\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --accept-regex=REGEX        regex matching accepted URLs\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --reject-regex=REGEX        regex matching rejected URLs\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --regex-type=TYPE           regex type (posix|pcre)\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -D,  --domains=LIST              comma-separated list of accepted domains\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --exclude-domains=LIST      comma-separated list of rejected domains\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --follow-ftp                follow FTP links from HTML documents\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --follow-tags=LIST          comma-separated list of followed HTML tags\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"       --ignore-tags=LIST          comma-separated list of ignored HTML tags\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -H,  --span-hosts                go to foreign hosts when recursive\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -L,  --relative                  follow relative links only\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -I,  --include-directories=LIST  list of allowed directories\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"       --trust-server-names        use the name specified by the redirection\n                                     URL's last component\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"  -X,  --exclude-directories=LIST  list of excluded directories\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        b"  -np, --no-parent                 don't ascend to the parent directory\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        b"Email bug reports, questions, discussions to <bug-wget@gnu.org>\nand/or open issues at https://savannah.gnu.org/bugs/?func=additem&group=wget.\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    ];
    let mut i: size_t = 0;
    if printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"GNU Wget %s, a non-interactive network retriever.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        version_string,
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if print_usage(0 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    i = 0 as size_t;
    while i < ::core::mem::size_of::<[*const ::core::ffi::c_char; 179]>()
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>())
    {
        if fputs(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                help[i],
                LC_MESSAGES,
            ),
            stdout,
        ) < 0 as ::core::ffi::c_int
        {
            exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
        }
        i = i.wrapping_add(1);
    }
    exit(C2Rust_Unnamed_4::WGET_EXIT_SUCCESS.0 as ::core::ffi::c_int);
}
unsafe extern "C" fn secs_to_human_time(
    mut interval: ::core::ffi::c_double,
) -> *mut ::core::ffi::c_char {
    static mut buf: [::core::ffi::c_char; 32] = [0; 32];
    let mut secs: ::core::ffi::c_int = (interval + 0.5f64) as ::core::ffi::c_int;
    let mut hours: ::core::ffi::c_int = 0;
    let mut mins: ::core::ffi::c_int = 0;
    let mut days: ::core::ffi::c_int = 0;
    days = secs / 86400 as ::core::ffi::c_int;
    secs %= 86400 as ::core::ffi::c_int;
    hours = secs / 3600 as ::core::ffi::c_int;
    secs %= 3600 as ::core::ffi::c_int;
    mins = secs / 60 as ::core::ffi::c_int;
    secs %= 60 as ::core::ffi::c_int;
    if days != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%dd %dh %dm %ds\0".as_ptr() as *const ::core::ffi::c_char,
            days,
            hours,
            mins,
            secs,
        );
    } else if hours != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%dh %dm %ds\0".as_ptr() as *const ::core::ffi::c_char,
            hours,
            mins,
            secs,
        );
    } else if mins != 0 {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%dm %ds\0".as_ptr() as *const ::core::ffi::c_char,
            mins,
            secs,
        );
    } else {
        snprintf(
            &raw mut buf as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 32]>(),
            b"%ss\0".as_ptr() as *const ::core::ffi::c_char,
            print_decimal(interval),
        );
    }
    return &raw mut buf as *mut ::core::ffi::c_char;
}
unsafe extern "C" fn prompt_for_password() -> *mut ::core::ffi::c_char {
    if !opt.user.is_null() {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Password for user %s: \0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            quote(opt.user),
        );
    } else {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Password: \0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
    }
    return getpass(b"\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn run_use_askpass(
    mut question: *mut ::core::ffi::c_char,
    mut answer: *mut *mut ::core::ffi::c_char,
) {
    let mut tmp: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut pid: pid_t = 0;
    let mut status: ::core::ffi::c_int = 0;
    let mut com: [::core::ffi::c_int; 2] = [0; 2];
    let mut bytes: ssize_t = 0 as ssize_t;
    let mut argv: [*mut ::core::ffi::c_char; 3] =
        [::core::ptr::null_mut::<::core::ffi::c_char>(); 3];
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut fa: posix_spawn_file_actions_t = posix_spawn_file_actions_t {
        __allocated: 0,
        __used: 0,
        __actions: ::core::ptr::null_mut::<__spawn_action>(),
        __pad: [0; 16],
    };
    if pipe(&raw mut com as *mut ::core::ffi::c_int) == -1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Cannot create pipe\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    status = posix_spawn_file_actions_init(&raw mut fa);
    if status != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Error initializing spawn file actions for use-askpass: %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            status,
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    status = posix_spawn_file_actions_adddup2(&raw mut fa, com[1usize], STDOUT_FILENO);
    if status != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Error setting spawn file actions for use-askpass: %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            status,
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    argv[0usize] = opt.use_askpass;
    argv[1usize] = question;
    argv[2usize] = ::core::ptr::null_mut::<::core::ffi::c_char>();
    status = posix_spawnp(
        &raw mut pid,
        opt.use_askpass,
        &raw mut fa,
        ::core::ptr::null::<posix_spawnattr_t>(),
        &raw mut argv as *mut *mut ::core::ffi::c_char as *const *mut ::core::ffi::c_char,
        environ as *const *mut ::core::ffi::c_char,
    );
    if status != 0 {
        fprintf(
            stderr,
            b"Error spawning %s: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            opt.use_askpass,
            status,
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    close(com[1usize]);
    bytes = read(
        com[0usize],
        &raw mut tmp as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[::core::ffi::c_char; 1024]>().wrapping_sub(1 as size_t),
    );
    if bytes <= 0 as ssize_t {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Error reading response from command \"%s %s\": %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            opt.use_askpass,
            question,
            strerror(*__errno_location()),
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    tmp[bytes as usize] = '\0' as ::core::ffi::c_char;
    p = strpbrk(
        &raw mut tmp as *mut ::core::ffi::c_char,
        b"\r\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if !p.is_null() {
        bytes = p.offset_from(&raw mut tmp as *mut ::core::ffi::c_char) as ssize_t;
    }
    *answer = xmemdup0(
        &raw mut tmp as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        bytes as size_t,
    );
}
unsafe extern "C" fn use_askpass(mut u: *mut url) {
    static mut question: [::core::ffi::c_char; 1024] = [0; 1024];
    if (*u).user.is_null()
        || *(*u).user.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        snprintf(
            &raw mut question as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Username for '%s%s': \0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            scheme_leading_string((*u).scheme),
            (*u).host,
        );
        run_use_askpass(
            &raw mut question as *mut ::core::ffi::c_char,
            &raw mut (*u).user,
        );
        if opt.recursive {
            opt.user = xstrdup((*u).user);
        }
    }
    if (*u).passwd.is_null()
        || *(*u).passwd.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
    {
        snprintf(
            &raw mut question as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 1024]>(),
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Password for '%s%s@%s': \0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            scheme_leading_string((*u).scheme),
            (*u).user,
            (*u).host,
        );
        run_use_askpass(
            &raw mut question as *mut ::core::ffi::c_char,
            &raw mut (*u).passwd,
        );
        if opt.recursive {
            opt.passwd = xstrdup((*u).passwd);
        }
    }
}
unsafe extern "C" fn format_and_print_line(
    mut prefix: *const ::core::ffi::c_char,
    mut line: *const ::core::ffi::c_char,
    mut line_length: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut remaining_chars: ::core::ffi::c_int = 0;
    let mut line_dup: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut token: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    line_dup = xstrdup(line);
    if printf(b"%s\0".as_ptr() as *const ::core::ffi::c_char, prefix) < 0 as ::core::ffi::c_int {
        free(line_dup as *mut ::core::ffi::c_void);
        line_dup = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return -1 as ::core::ffi::c_int;
    }
    remaining_chars = 0 as ::core::ffi::c_int;
    token = strtok(line_dup, b" \0".as_ptr() as *const ::core::ffi::c_char);
    while !token.is_null() {
        if remaining_chars <= strlen(token) as ::core::ffi::c_int {
            if printf(
                b"\n%*c\0".as_ptr() as *const ::core::ffi::c_char,
                TABULATION,
                ' ' as ::core::ffi::c_int,
            ) < 0 as ::core::ffi::c_int
            {
                free(line_dup as *mut ::core::ffi::c_void);
                line_dup = ::core::ptr::null_mut::<::core::ffi::c_char>();
                return -1 as ::core::ffi::c_int;
            }
            remaining_chars = line_length - TABULATION;
        }
        if printf(b"%s \0".as_ptr() as *const ::core::ffi::c_char, token) < 0 as ::core::ffi::c_int
        {
            free(line_dup as *mut ::core::ffi::c_void);
            line_dup = ::core::ptr::null_mut::<::core::ffi::c_char>();
            return -1 as ::core::ffi::c_int;
        }
        remaining_chars = (remaining_chars as size_t)
            .wrapping_sub(strlen(token).wrapping_add(1 as size_t))
            as ::core::ffi::c_int;
        token = strtok(
            ::core::ptr::null_mut::<::core::ffi::c_char>(),
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
        free(line_dup as *mut ::core::ffi::c_void);
        line_dup = ::core::ptr::null_mut::<::core::ffi::c_char>();
        return -1 as ::core::ffi::c_int;
    }
    free(line_dup as *mut ::core::ffi::c_void);
    line_dup = ::core::ptr::null_mut::<::core::ffi::c_char>();
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn print_version() {
    let mut wgetrc_title: *const ::core::ffi::c_char = dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Wgetrc: \0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    );
    let mut locale_title: *const ::core::ffi::c_char = dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Locale: \0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    );
    let mut compile_title: *const ::core::ffi::c_char = dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Compile: \0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    );
    let mut link_title: *const ::core::ffi::c_char = dcgettext(
        ::core::ptr::null::<::core::ffi::c_char>(),
        b"Link: \0".as_ptr() as *const ::core::ffi::c_char,
        LC_MESSAGES,
    );
    let mut env_wgetrc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut user_wgetrc: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    if printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"GNU Wget %s built on %s.\n\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        version_string,
        OS_TYPE.as_ptr(),
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    i = 0 as ::core::ffi::c_int;
    while !(*(&raw mut compiled_features as *mut *const ::core::ffi::c_char).offset(i as isize))
        .is_null()
    {
        let mut line_length: ::core::ffi::c_int = MAX_CHARS_PER_LINE;
        while line_length > 0 as ::core::ffi::c_int
            && !(*(&raw mut compiled_features as *mut *const ::core::ffi::c_char)
                .offset(i as isize))
            .is_null()
        {
            if printf(
                b"%s \0".as_ptr() as *const ::core::ffi::c_char,
                *(&raw mut compiled_features as *mut *const ::core::ffi::c_char).offset(i as isize),
            ) < 0 as ::core::ffi::c_int
            {
                exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
            }
            line_length -= strlen(
                *(&raw mut compiled_features as *mut *const ::core::ffi::c_char).offset(i as isize),
            ) as ::core::ffi::c_int
                + 2 as ::core::ffi::c_int;
            i += 1;
        }
        if printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
            exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
        }
    }
    if printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if printf(
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        wgetrc_title,
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    env_wgetrc = wgetrc_env_file_name();
    if !env_wgetrc.is_null() && *env_wgetrc as ::core::ffi::c_int != 0 {
        if printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"    %s (env)\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            env_wgetrc,
        ) < 0 as ::core::ffi::c_int
        {
            exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
        }
        free(env_wgetrc as *mut ::core::ffi::c_void);
        env_wgetrc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    user_wgetrc = wgetrc_user_file_name();
    if !user_wgetrc.is_null() {
        if printf(
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"    %s (user)\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            user_wgetrc,
        ) < 0 as ::core::ffi::c_int
        {
            exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
        }
        free(user_wgetrc as *mut ::core::ffi::c_void);
        user_wgetrc = ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    if printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"    %s (system)\n\0".as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        SYSTEM_WGETRC.as_ptr(),
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if format_and_print_line(locale_title, LOCALEDIR.as_ptr(), MAX_CHARS_PER_LINE)
        < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if !compilation_string.is_null() {
        if format_and_print_line(compile_title, compilation_string, MAX_CHARS_PER_LINE)
            < 0 as ::core::ffi::c_int
        {
            exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
        }
    }
    if !link_string.is_null() {
        if format_and_print_line(link_title, link_string, MAX_CHARS_PER_LINE)
            < 0 as ::core::ffi::c_int
        {
            exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
        }
    }
    if printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char) < 0 as ::core::ffi::c_int {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if printf(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Copyright (C) %s Free Software Foundation, Inc.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        b"2015\0".as_ptr() as *const ::core::ffi::c_char,
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"License GPLv3+: GNU GPL version 3 or later\n<http://www.gnu.org/licenses/gpl.html>.\nThis is free software: you are free to change and redistribute it.\nThere is NO WARRANTY, to the extent permitted by law.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        stdout,
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"\nOriginally written by Hrvoje Niksic <hniksic@xemacs.org>.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        stdout,
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    if fputs(
        dcgettext(
            ::core::ptr::null::<::core::ffi::c_char>(),
            b"Please send bug reports and questions to <bug-wget@gnu.org>.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            LC_MESSAGES,
        ),
        stdout,
    ) < 0 as ::core::ffi::c_int
    {
        exit(C2Rust_Unnamed_4::WGET_EXIT_IO_FAIL.0 as ::core::ffi::c_int);
    }
    exit(C2Rust_Unnamed_4::WGET_EXIT_SUCCESS.0 as ::core::ffi::c_int);
}
#[export_name = "rboxc_wget_program_name"]
pub static mut program_name: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_wget_program_argstring"]
pub static mut program_argstring: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[export_name = "rboxc_wget_timer"]
pub static mut timer: *mut ptimer = ::core::ptr::null_mut::<ptimer>();
#[export_name = "rboxc_wget_cleaned_up"]
pub static mut cleaned_up: ::core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn single_binary_main_wget(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut p: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut i: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    let mut longindex: ::core::ffi::c_int = 0;
    let mut nurls: ::core::ffi::c_int = 0;
    let mut argstring_length: ::core::ffi::c_int = 0;
    let mut use_userconfig: bool = r#false != 0;
    let mut noconfig: bool = r#false != 0;
    let mut append_to_log: bool = r#false != 0;
    cleaned_up = 0 as ::core::ffi::c_int;
    timer = ptimer_new();
    let mut start_time: ::core::ffi::c_double = ptimer_measure(timer);
    total_downloaded_bytes = 0 as wgint;
    program_name = *argv.offset(0isize);
    i18n_initialize();
    exec_name = base_name(*argv.offset(0isize));
    argstring_length = 1 as ::core::ffi::c_int;
    i = 1 as ::core::ffi::c_int;
    while i < argc {
        argstring_length = (argstring_length as size_t).wrapping_add(
            strlen(*argv.offset(i as isize))
                .wrapping_add(3 as size_t)
                .wrapping_add(1 as size_t),
        ) as ::core::ffi::c_int;
        i += 1;
    }
    p = malloc(argstring_length as size_t) as *mut ::core::ffi::c_char;
    program_argstring = p;
    if p.is_null() {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Memory allocation problem\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_PARSE_ERROR.0 as ::core::ffi::c_int);
    }
    i = 1 as ::core::ffi::c_int;
    while i < argc {
        let mut arglen: ::core::ffi::c_int = 0;
        let c2rust_fresh5 = p;
        p = p.offset(1);
        *c2rust_fresh5 = '"' as ::core::ffi::c_char;
        arglen = strlen(*argv.offset(i as isize)) as ::core::ffi::c_int;
        memcpy(
            p as *mut ::core::ffi::c_void,
            *argv.offset(i as isize) as *const ::core::ffi::c_void,
            arglen as size_t,
        );
        p = p.offset(arglen as isize);
        let c2rust_fresh6 = p;
        p = p.offset(1);
        *c2rust_fresh6 = '"' as ::core::ffi::c_char;
        let c2rust_fresh7 = p;
        p = p.offset(1);
        *c2rust_fresh7 = ' ' as ::core::ffi::c_char;
        i += 1;
    }
    *p = '\0' as ::core::ffi::c_char;
    defaults();
    opt.homedir = home_dir();
    init_switches();
    longindex = -1 as ::core::ffi::c_int;
    while getopt_long(
        argc,
        argv,
        &raw mut short_options as *mut ::core::ffi::c_char,
        &raw mut long_options as *mut option,
        &raw mut longindex,
    ) != -1 as ::core::ffi::c_int
    {
        let mut confval: ::core::ffi::c_int = 0;
        let mut config_opt: *mut cmdline_option = ::core::ptr::null_mut::<cmdline_option>();
        if longindex < 0 as ::core::ffi::c_int {
            continue;
        }
        confval = long_options[longindex as usize].val;
        config_opt = (&raw mut option_data as *mut cmdline_option)
            .offset((confval & !BOOLEAN_NEG_MARKER) as isize);
        if strcmp(
            &raw mut (*config_opt).long_name as *mut ::core::ffi::c_char,
            b"no-config\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            noconfig = r#true != 0;
            break;
        } else {
            if strcmp(
                &raw mut (*config_opt).long_name as *mut ::core::ffi::c_char,
                b"config\0".as_ptr() as *const ::core::ffi::c_char,
            ) != 0 as ::core::ffi::c_int
            {
                continue;
            }
            let mut flstats: file_stats_t = file_stats_t {
                access_err: 0,
                st_ino: 0,
                st_dev: 0,
            };
            use_userconfig = r#true != 0;
            memset(
                &raw mut flstats as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<file_stats_t>(),
            );
            if file_exists_p(optarg, &raw mut flstats) as ::core::ffi::c_int != 0
                && run_wgetrc(optarg, &raw mut flstats) as ::core::ffi::c_int != 0
            {
                break;
            }
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Exiting due to error in %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                optarg,
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_PARSE_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if noconfig as ::core::ffi::c_int == r#false && use_userconfig as ::core::ffi::c_int == r#false
    {
        ret = initialize();
        if ret != 0 {
            return ret;
        }
    }
    opterr = 0 as ::core::ffi::c_int;
    optind = 0 as ::core::ffi::c_int;
    longindex = -1 as ::core::ffi::c_int;
    loop {
        ret = getopt_long(
            argc,
            argv,
            &raw mut short_options as *mut ::core::ffi::c_char,
            &raw mut long_options as *mut option,
            &raw mut longindex,
        );
        if ret == -1 as ::core::ffi::c_int {
            break;
        }
        let mut val: ::core::ffi::c_int = 0;
        let mut cmdopt: *mut cmdline_option = ::core::ptr::null_mut::<cmdline_option>();
        if longindex == -1 as ::core::ffi::c_int {
            if ret == '?' as ::core::ffi::c_int {
                print_usage(1 as ::core::ffi::c_int);
                fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                fprintf(
                    stderr,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Try `%s --help' for more options.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    exec_name,
                );
                exit(C2Rust_Unnamed_4::WGET_EXIT_PARSE_ERROR.0 as ::core::ffi::c_int);
            }
            longindex = optmap[(ret - 32 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int;
        }
        val = long_options[longindex as usize].val;
        cmdopt = (&raw mut option_data as *mut cmdline_option)
            .offset((val & !BOOLEAN_NEG_MARKER) as isize);
        match (*cmdopt).r#type {
            C2Rust_Unnamed_5::OPT_VALUE => {
                setoptval(
                    (*cmdopt).data as *const ::core::ffi::c_char,
                    optarg,
                    &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                );
            }
            C2Rust_Unnamed_5::OPT_BOOLEAN => {
                if !optarg.is_null() {
                    setoptval(
                        (*cmdopt).data as *const ::core::ffi::c_char,
                        optarg,
                        &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                    );
                } else {
                    let mut neg: bool = val & BOOLEAN_NEG_MARKER != 0;
                    setoptval(
                        (*cmdopt).data as *const ::core::ffi::c_char,
                        if neg as ::core::ffi::c_int != 0 {
                            b"0\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b"1\0".as_ptr() as *const ::core::ffi::c_char
                        },
                        &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                    );
                }
            }
            C2Rust_Unnamed_5::OPT_FUNCALL => {
                let mut func: Option<unsafe extern "C" fn() -> ()> =
                    ::core::mem::transmute::<
                        *const ::core::ffi::c_void,
                        Option<unsafe extern "C" fn() -> ()>,
                    >((*cmdopt).data);
                func.expect("non-null function pointer")();
            }
            C2Rust_Unnamed_5::OPT__APPEND_OUTPUT => {
                setoptval(
                    b"logfile\0".as_ptr() as *const ::core::ffi::c_char,
                    optarg,
                    &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                );
                append_to_log = r#true != 0;
            }
            C2Rust_Unnamed_5::OPT__EXECUTE => {
                if !optarg.is_null() {
                    run_command(optarg);
                }
            }
            C2Rust_Unnamed_5::OPT__NO => {
                p = optarg;
                while !p.is_null() && *p as ::core::ffi::c_int != 0 {
                    match *p as ::core::ffi::c_int {
                        118 => {
                            setoptval(
                                b"verbose\0".as_ptr() as *const ::core::ffi::c_char,
                                b"0\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                            );
                        }
                        72 => {
                            setoptval(
                                b"addhostdir\0".as_ptr() as *const ::core::ffi::c_char,
                                b"0\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                            );
                        }
                        100 => {
                            setoptval(
                                b"dirstruct\0".as_ptr() as *const ::core::ffi::c_char,
                                b"0\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                            );
                        }
                        99 => {
                            setoptval(
                                b"noclobber\0".as_ptr() as *const ::core::ffi::c_char,
                                b"1\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                            );
                        }
                        112 => {
                            setoptval(
                                b"noparent\0".as_ptr() as *const ::core::ffi::c_char,
                                b"1\0".as_ptr() as *const ::core::ffi::c_char,
                                &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                            );
                        }
                        _ => {
                            fprintf(
                                stderr,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"%s: illegal option -- `-n%c'\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    LC_MESSAGES,
                                ),
                                exec_name,
                                *p as ::core::ffi::c_int,
                            );
                            print_usage(1 as ::core::ffi::c_int);
                            fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                            fprintf(
                                stderr,
                                dcgettext(
                                    ::core::ptr::null::<::core::ffi::c_char>(),
                                    b"Try `%s --help' for more options.\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    LC_MESSAGES,
                                ),
                                exec_name,
                            );
                            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
                        }
                    }
                    p = p.offset(1);
                }
            }
            C2Rust_Unnamed_5::OPT__PARENT | C2Rust_Unnamed_5::OPT__CLOBBER => {
                let mut flag: bool = r#true != 0;
                if !optarg.is_null() {
                    flag = *optarg as ::core::ffi::c_int == '1' as ::core::ffi::c_int
                        || c_tolower(*optarg as ::core::ffi::c_int) == 'y' as ::core::ffi::c_int
                        || c_tolower(*optarg.offset(0isize) as ::core::ffi::c_int)
                            == 'o' as ::core::ffi::c_int
                            && c_tolower(*optarg.offset(1isize) as ::core::ffi::c_int)
                                == 'n' as ::core::ffi::c_int;
                }
                setoptval(
                    if (*cmdopt).r#type.0 == C2Rust_Unnamed_5::OPT__PARENT.0 {
                        b"noparent\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"noclobber\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    if flag as ::core::ffi::c_int != 0 {
                        b"0\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"1\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                );
            }
            C2Rust_Unnamed_5::OPT__DONT_REMOVE_LISTING => {
                setoptval(
                    b"removelisting\0".as_ptr() as *const ::core::ffi::c_char,
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut (*cmdopt).long_name as *mut ::core::ffi::c_char,
                );
            }
            _ => {}
        }
        longindex = -1 as ::core::ffi::c_int;
    }
    nurls = argc - optind;
    log_init(opt.lfilename, append_to_log);
    if opt.noclobber as ::core::ffi::c_int != 0
        && (opt.convert_links as ::core::ffi::c_int != 0
            || opt.convert_file_only as ::core::ffi::c_int != 0)
    {
        fprintf(
            stderr,
            if opt.convert_links as ::core::ffi::c_int != 0 {
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Both --no-clobber and --convert-links were specified, only --convert-links will be used.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                )
            } else {
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Both --no-clobber and --convert-file-only were specified, only --convert-file-only will be used.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                )
            },
        );
        opt.noclobber = r#false != 0;
    }
    if opt.reclevel == 0 as ::core::ffi::c_int {
        opt.reclevel = INFINITE_RECURSION;
    }
    if opt.spider as ::core::ffi::c_int != 0 || opt.delete_after as ::core::ffi::c_int != 0 {
        opt.no_dirstruct = r#true != 0;
    }
    if opt.page_requisites as ::core::ffi::c_int != 0 && !opt.recursive {
        opt.reclevel = 0 as ::core::ffi::c_int;
        if !opt.no_dirstruct {
            opt.dirstruct = true;
        }
    }
    if opt.verbose == -1 as ::core::ffi::c_int {
        opt.verbose = !opt.quiet as ::core::ffi::c_int;
    }
    if opt.verbose == 0 && opt.show_progress == -1 as ::core::ffi::c_int {
        opt.show_progress = r#false;
    }
    if opt.quiet as ::core::ffi::c_int != 0 && opt.show_progress == -1 as ::core::ffi::c_int {
        opt.show_progress = r#false;
    }
    if opt.verbose != 0 && opt.quiet as ::core::ffi::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Can't be verbose and quiet at the same time.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        print_usage(1 as ::core::ffi::c_int);
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    if opt.timestamping as ::core::ffi::c_int != 0 && opt.noclobber as ::core::ffi::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Can't timestamp and not clobber old files at the same time.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        print_usage(1 as ::core::ffi::c_int);
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    if opt.ipv4_only as ::core::ffi::c_int != 0 && opt.ipv6_only as ::core::ffi::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Cannot specify both --inet4-only and --inet6-only.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        print_usage(1 as ::core::ffi::c_int);
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    if !opt.output_document.is_null() {
        if (opt.convert_links as ::core::ffi::c_int != 0
            || opt.convert_file_only as ::core::ffi::c_int != 0)
            && (nurls > 1 as ::core::ffi::c_int
                || opt.page_requisites as ::core::ffi::c_int != 0
                || opt.recursive as ::core::ffi::c_int != 0)
        {
            fputs(
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Cannot specify both -k or --convert-file-only and -O if multiple URLs are given, or in combination\nwith -p or -r. See the manual for details.\n\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                stderr,
            );
            print_usage(1 as ::core::ffi::c_int);
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
        if opt.page_requisites as ::core::ffi::c_int != 0
            || opt.recursive as ::core::ffi::c_int != 0
        {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"WARNING: combining -O with -r or -p will mean that all downloaded content\nwill be placed in the single file you specified.\n\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        }
        if opt.timestamping {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"WARNING: timestamping does nothing in combination with -O. See the manual\nfor details.\n\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            opt.timestamping = r#false != 0;
        }
        if opt.noclobber as ::core::ffi::c_int != 0
            && file_exists_p(opt.output_document, ::core::ptr::null_mut::<file_stats_t>())
                as ::core::ffi::c_int
                != 0
        {
            logprintf(
                log_options::LOG_VERBOSE,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"File %s already there; not retrieving.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                quote(opt.output_document),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if !opt.warc_filename.is_null() {
        if opt.noclobber {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"WARC output does not work with --no-clobber, --no-clobber will be disabled.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            opt.noclobber = r#false != 0;
        }
        if opt.timestamping {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"WARC output does not work with timestamping, timestamping will be disabled.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            opt.timestamping = r#false != 0;
        }
        if opt.spider {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"WARC output does not work with --spider.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
        if opt.always_rest as ::core::ffi::c_int != 0 || opt.start_pos >= 0 as wgint {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"WARC output does not work with --continue or --start-pos, they will be disabled.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            opt.always_rest = r#false != 0;
            opt.start_pos = -1 as wgint;
        }
        if !opt.warc_cdx_dedup_filename.is_null() && !opt.warc_digests_enabled {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Digests are disabled; WARC deduplication will not find duplicate records.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
        }
        if opt.warc_keep_log {
            opt.progress_type = xstrdup(b"dot\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    if opt.always_rest as ::core::ffi::c_int != 0 || opt.start_pos >= 0 as wgint {
        if opt.compression.0 == compression_options::compression_auto.0 {
            opt.compression = compression_options::compression_none;
        } else if opt.compression.0 != compression_options::compression_none.0 {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Compression does not work with --continue or --start-pos, they will be disabled.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            opt.always_rest = r#false != 0;
            opt.start_pos = -1 as wgint;
        }
    }
    if opt.ask_passwd as ::core::ffi::c_int != 0 && !opt.passwd.is_null() {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Cannot specify both --ask-password and --password.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        print_usage(1 as ::core::ffi::c_int);
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    if opt.ask_passwd as ::core::ffi::c_int != 0
        && !(!opt.user.is_null() || !opt.http_user.is_null() || !opt.ftp_user.is_null())
    {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"WARNING: No username set with --ask-password. This is usually not what you want.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
    }
    if opt.start_pos >= 0 as wgint && opt.always_rest as ::core::ffi::c_int != 0 {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Specifying both --start-pos and --continue is not recommended; --continue will be disabled.\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        opt.always_rest = r#false != 0;
    }
    if nurls == 0 && opt.input_filename.is_null() {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"%s: missing URL\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            exec_name,
        );
        print_usage(1 as ::core::ffi::c_int);
        fprintf(stderr, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"Try `%s --help' for more options.\n\0".as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            exec_name,
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    match opt.regex_type {
        C2Rust_Unnamed_3::regex_type_pcre => {
            opt.regex_compile_fun = Some(
                compile_pcre2_regex
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
            )
                as Option<
                    unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
                >;
            opt.regex_match_fun = Some(
                match_pcre2_regex
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> bool,
                >;
        }
        C2Rust_Unnamed_3::regex_type_posix | _ => {
            opt.regex_compile_fun = Some(
                compile_posix_regex
                    as unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
            )
                as Option<
                    unsafe extern "C" fn(*const ::core::ffi::c_char) -> *mut ::core::ffi::c_void,
                >;
            opt.regex_match_fun = Some(
                match_posix_regex
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> bool,
            )
                as Option<
                    unsafe extern "C" fn(
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_char,
                    ) -> bool,
                >;
        }
    }
    if !opt.acceptregex_s.is_null() {
        opt.acceptregex =
            opt.regex_compile_fun.expect("non-null function pointer")(opt.acceptregex_s);
        if opt.acceptregex.is_null() {
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if !opt.rejectregex_s.is_null() {
        opt.rejectregex =
            opt.regex_compile_fun.expect("non-null function pointer")(opt.rejectregex_s);
        if opt.rejectregex.is_null() {
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if !opt.post_data.is_null() || !opt.post_file_name.is_null() {
        if !opt.post_data.is_null() && !opt.post_file_name.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"You cannot specify both --post-data and --post-file.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        } else if !opt.method.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"You cannot use --post-data or --post-file along with --method. --method expects data through --body-data and --body-file options\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if !opt.body_data.is_null() || !opt.body_file.is_null() {
        if opt.method.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"You must specify a method through --method=HTTPMethod to use with --body-data or --body-file.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        } else if !opt.body_data.is_null() && !opt.body_file.is_null() {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"You cannot specify both --body-data and --body-file.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if !opt.method.is_null()
        && c_strcasecmp(opt.method, b"HEAD\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
    {
        setoptval(
            b"spider\0".as_ptr() as *const ::core::ffi::c_char,
            b"1\0".as_ptr() as *const ::core::ffi::c_char,
            b"spider\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !opt.post_data.is_null() || !opt.post_file_name.is_null() {
        setoptval(
            b"method\0".as_ptr() as *const ::core::ffi::c_char,
            b"POST\0".as_ptr() as *const ::core::ffi::c_char,
            b"method\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !opt.post_data.is_null() {
            setoptval(
                b"bodydata\0".as_ptr() as *const ::core::ffi::c_char,
                opt.post_data,
                b"body-data\0".as_ptr() as *const ::core::ffi::c_char,
            );
            free(opt.post_data as *mut ::core::ffi::c_void);
            opt.post_data = ::core::ptr::null_mut::<::core::ffi::c_char>();
        } else {
            setoptval(
                b"bodyfile\0".as_ptr() as *const ::core::ffi::c_char,
                opt.post_file_name,
                b"body-file\0".as_ptr() as *const ::core::ffi::c_char,
            );
            free(opt.post_file_name as *mut ::core::ffi::c_void);
            opt.post_file_name = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
    }
    memset(
        &raw mut dummy_iri as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<iri>(),
    );
    if opt.enable_iri as ::core::ffi::c_int != 0
        || !opt.locale.is_null()
        || !opt.encoding_remote.is_null()
    {
        fprintf(
            stderr,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"This version does not have support for IRIs\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
        );
        exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
    }
    if opt.ask_passwd {
        opt.passwd = prompt_for_password();
        if opt.passwd.is_null()
            || *opt.passwd.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int
        {
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if !opt.use_askpass.is_null() {
        if *opt.use_askpass.offset(0isize) as ::core::ffi::c_int == '\0' as ::core::ffi::c_int {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"use-askpass requires a string or either environment variable WGET_ASKPASS or SSH_ASKPASS to be set.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if opt.background {
        let mut logfile_changed: bool = fork_to_background();
        if logfile_changed {
            log_init(opt.lfilename, append_to_log);
        }
    }
    if opt.show_progress != 0 {
        set_progress_implementation(opt.progress_type);
    }
    if !opt.warc_filename.is_null() {
        warc_init();
    }
    if opt.debug as ::core::ffi::c_long != 0 {
        debug_logprintf(
            b"DEBUG output created by Wget %s on %s.\n\n\0".as_ptr() as *const ::core::ffi::c_char,
            version_string,
            b"linux-gnu\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if !opt.output_document.is_null() {
        if *opt.output_document as ::core::ffi::c_int == '-' as ::core::ffi::c_int
            && *opt.output_document.offset(1 as ::core::ffi::c_int as isize) == 0
        {
            output_stream = stdout;
        } else {
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
            if opt.unlink_requested {
                unlink(opt.output_document);
            }
            output_stream = fopen(
                opt.output_document,
                if opt.always_rest as ::core::ffi::c_int != 0 {
                    b"ab\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"wb\0".as_ptr() as *const ::core::ffi::c_char
                },
            ) as *mut FILE;
            if output_stream.is_null() {
                perror(opt.output_document);
                exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
            }
            if fstat(fileno(output_stream), &raw mut st) == 0 as ::core::ffi::c_int
                && st.st_mode & __S_IFMT as __mode_t == 0o100000 as __mode_t
            {
                output_stream_regular = r#true != 0;
            }
        }
        if !output_stream_regular
            && (opt.convert_links as ::core::ffi::c_int != 0
                || opt.recursive as ::core::ffi::c_int != 0)
        {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"-k or -r can be used together with -O only if outputting to a regular file.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
        if !output_stream_regular
            && (opt.convert_links as ::core::ffi::c_int != 0
                || opt.convert_file_only as ::core::ffi::c_int != 0)
        {
            fprintf(
                stderr,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"--convert-links or --convert-file-only can be used together only if outputting to a regular file.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
            );
            exit(C2Rust_Unnamed_4::WGET_EXIT_GENERIC_ERROR.0 as ::core::ffi::c_int);
        }
    }
    if signal(
        SIGHUP,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    ) != ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
        1 as ::core::ffi::c_int as ::libc::intptr_t,
    ) {
        signal(
            SIGHUP,
            Some(redirect_output_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
        );
    }
    signal(
        SIGUSR1,
        Some(redirect_output_signal as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    signal(
        SIGPIPE,
        ::core::mem::transmute::<::libc::intptr_t, __sighandler_t>(
            1 as ::core::ffi::c_int as ::libc::intptr_t,
        ),
    );
    signal(
        SIGWINCH,
        Some(progress_handle_sigwinch as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    if opt.hsts {
        load_hsts();
    }
    i = 0 as ::core::ffi::c_int;
    while i < nurls {
        let mut t: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut filename: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut redirected_URL: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        let mut dt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut url_err: ::core::ffi::c_int = 0;
        let mut iri: *mut iri = &raw mut dummy_iri;
        let mut url_parsed: *mut url = ::core::ptr::null_mut::<url>();
        t = maybe_prepend_scheme(*argv.offset(optind as isize));
        if t.is_null() {
            t = *argv.offset(optind as isize);
        }
        url_parsed = url_parse(t, &raw mut url_err, iri, r#true != 0);
        if url_parsed.is_null() {
            logprintf(
                log_options::LOG_NOTQUIET,
                b"%s: %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                t,
                url_error(url_err),
            );
            inform_exit_status(uerr_t::URLERROR);
        } else {
            if !opt.use_askpass.is_null() {
                use_askpass(url_parsed);
            }
            if (opt.recursive as ::core::ffi::c_int != 0
                || opt.page_requisites as ::core::ffi::c_int != 0)
                && (url_scheme_0(t).0 != url_scheme::SCHEME_FTP.0
                    && url_scheme_0(t).0 != url_scheme::SCHEME_FTPS.0
                    || url_uses_proxy(url_parsed) as ::core::ffi::c_int != 0)
            {
                let mut old_follow_ftp: ::core::ffi::c_int = opt.follow_ftp as ::core::ffi::c_int;
                if url_scheme_0(t).0 == url_scheme::SCHEME_FTP.0
                    || url_scheme_0(t).0 == url_scheme::SCHEME_FTPS.0
                {
                    opt.follow_ftp = true;
                }
                retrieve_tree(url_parsed, ::core::ptr::null_mut::<iri>());
                opt.follow_ftp = old_follow_ftp != 0;
            } else {
                retrieve_url(
                    url_parsed,
                    t,
                    &raw mut filename,
                    &raw mut redirected_URL,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    &raw mut dt,
                    opt.recursive,
                    iri,
                    r#true != 0,
                );
            }
            if opt.delete_after as ::core::ffi::c_int != 0
                && !filename.is_null()
                && file_exists_p(filename, ::core::ptr::null_mut::<file_stats_t>())
                    as ::core::ffi::c_int
                    != 0
            {
                if opt.debug as ::core::ffi::c_long != 0 {
                    debug_logprintf(
                        b"Removing file due to --delete-after in main():\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                }
                logprintf(
                    log_options::LOG_VERBOSE,
                    dcgettext(
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        b"Removing %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                        LC_MESSAGES,
                    ),
                    filename,
                );
                if unlink(filename) != 0 {
                    logprintf(
                        log_options::LOG_NOTQUIET,
                        b"unlink: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        strerror(*__errno_location()),
                    );
                }
            }
            free(redirected_URL as *mut ::core::ffi::c_void);
            redirected_URL = ::core::ptr::null_mut::<::core::ffi::c_char>();
            free(filename as *mut ::core::ffi::c_void);
            filename = ::core::ptr::null_mut::<::core::ffi::c_char>();
            url_free(url_parsed);
        }
        if t != *argv.offset(optind as isize) {
            free(t as *mut ::core::ffi::c_void);
            t = ::core::ptr::null_mut::<::core::ffi::c_char>();
        }
        i += 1;
        optind += 1;
    }
    if !opt.input_filename.is_null() {
        let mut count: ::core::ffi::c_int = 0;
        let mut status: ::core::ffi::c_int = 0;
        status = retrieve_from_file(opt.input_filename, opt.force_html, &raw mut count).0
            as ::core::ffi::c_int;
        inform_exit_status(uerr_t(status as ::core::ffi::c_uint));
        if count == 0 {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"No URLs found in %s.\n\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                opt.input_filename,
            );
        }
    }
    if opt.recursive as ::core::ffi::c_int != 0 && opt.spider as ::core::ffi::c_int != 0 {
        print_broken_links();
    }
    if (opt.recursive as ::core::ffi::c_int != 0
        || opt.page_requisites as ::core::ffi::c_int != 0
        || nurls > 1 as ::core::ffi::c_int
        || !opt.input_filename.is_null() && total_downloaded_bytes != 0 as wgint)
        && total_downloaded_bytes != 0 as wgint
    {
        let mut end_time: ::core::ffi::c_double = ptimer_measure(timer);
        let mut wall_time: *mut ::core::ffi::c_char =
            xstrdup(secs_to_human_time(end_time - start_time));
        let mut download_time: *mut ::core::ffi::c_char =
            xstrdup(secs_to_human_time(total_download_time));
        ptimer_destroy(timer);
        timer = ::core::ptr::null_mut::<ptimer>();
        logprintf(
            log_options::LOG_NOTQUIET,
            dcgettext(
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"FINISHED --%s--\nTotal wall clock time: %s\nDownloaded: %d files, %s in %s (%s)\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                LC_MESSAGES,
            ),
            datetime_str(time(::core::ptr::null_mut::<time_t>())),
            wall_time,
            numurls,
            human_readable(
                total_downloaded_bytes,
                10 as ::core::ffi::c_int,
                1 as ::core::ffi::c_int,
            ),
            download_time,
            retr_rate(total_downloaded_bytes, total_download_time),
        );
        free(wall_time as *mut ::core::ffi::c_void);
        wall_time = ::core::ptr::null_mut::<::core::ffi::c_char>();
        free(download_time as *mut ::core::ffi::c_void);
        download_time = ::core::ptr::null_mut::<::core::ffi::c_char>();
        if opt.quota != 0 && total_downloaded_bytes > opt.quota {
            logprintf(
                log_options::LOG_NOTQUIET,
                dcgettext(
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    b"Download quota of %s EXCEEDED!\n\0".as_ptr() as *const ::core::ffi::c_char,
                    LC_MESSAGES,
                ),
                human_readable(opt.quota, 10 as ::core::ffi::c_int, 1 as ::core::ffi::c_int),
            );
        }
    }
    if !opt.cookies_output.is_null() {
        save_cookies();
    }
    if opt.hsts as ::core::ffi::c_int != 0 && !hsts_store.is_null() {
        save_hsts();
    }
    if (opt.convert_links as ::core::ffi::c_int != 0
        || opt.convert_file_only as ::core::ffi::c_int != 0)
        && !opt.delete_after
    {
        convert_all_links();
    }
    cleanup();
    exit(get_exit_status());
}
pub const OS_TYPE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"linux-gnu\0") };
pub const SYSTEM_WGETRC: [::core::ffi::c_char; 41] = unsafe {
    ::core::mem::transmute::<[u8; 41], [::core::ffi::c_char; 41]>(
        *b"/root/rboxc/build/oracle/wget/etc/wgetrc\0",
    )
};
pub const LOCALEDIR: [::core::ffi::c_char; 43] = unsafe {
    ::core::mem::transmute::<[u8; 43], [::core::ffi::c_char; 43]>(
        *b"/root/rboxc/build/oracle/wget/share/locale\0",
    )
};
pub const r#true: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const r#false: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
