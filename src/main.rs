// GNU command bodies retain their original C ABI. Borrow the operating
// system's argv directly, including non-UTF-8 bytes, without creating a
// std::env argument allocation that GNU's exit paths would bypass.
#![no_main]
#![feature(c_variadic)]
#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals,
         unused_assignments, unused_mut, unused_variables, clippy::all)]

use core::ffi::{c_char, c_int};
use core::ffi::CStr;

include!("registry.rs");

unsafe fn write_all(fd: c_int, mut bytes: &[u8]) -> bool {
    while !bytes.is_empty() {
        let count = libc::write(fd, bytes.as_ptr().cast(), bytes.len());
        if count < 0 {
            if *libc::__errno_location() == libc::EINTR { continue; }
            return false;
        }
        if count == 0 { return false; }
        bytes = &bytes[count as usize..];
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn main(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    if argc < 1 || argv.is_null() || (*argv).is_null() { return 1; }
    let path = CStr::from_ptr(*argv).to_bytes();
    let mut name = path.rsplit(|byte| *byte == b'/').next().unwrap_or(path);
    let rbox_invocation = name == b"rboxc" || name == b"rbox";
    if rbox_invocation {
        if argc < 2 {
            write_all(2, b"Usage: rboxc COMMAND [ARGUMENTS...]\n       rboxc --list\n");
            return 1;
        }
        name = CStr::from_ptr(*argv.add(1)).to_bytes();
        if name == b"--list" {
            for (command, _) in APPLETS {
                if !write_all(1, command) || !write_all(1, b"\n") { return 1; }
            }
            return 0;
        }
        if name == b"--help" {
            return if write_all(1, b"Usage: rboxc COMMAND [ARGUMENTS...]\n       rboxc --list\n") { 0 } else { 1 };
        }
        argc -= 1;
        argv = argv.add(1);
    }
    for (command, entry) in APPLETS {
        if name == *command { return entry(argc, argv); }
    }
    // GNU updatedb's private encoder is callable by its symlink, but is not
    // an additional public inventory command.
    if name == b"frcode" { return applet_frcode::single_binary_main_frcode(argc, argv); }
    if !rbox_invocation {
        // GNU owns alternate executable names, including ginstall, prefixed
        // coreutils names, and the diagnostic for an unknown symlink name.
        return applet_coreutils::single_binary_main_coreutils(argc, argv);
    }
    write_all(2, b"rboxc: unknown program\n");
    127
}
