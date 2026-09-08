use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let helper = root.join("src/shell_arguments.c");
    let object = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("shell_arguments.o");
    println!("cargo:rerun-if-changed={}", helper.display());
    assert!(std::process::Command::new("cc").args(["-O2", "-fPIC", "-Wall", "-Wextra", "-Werror", "-c"]).arg(&helper).arg("-o").arg(&object).status().unwrap().success());
    println!("cargo:rustc-link-arg={}", object.display());
    let inputs = root.join("build/rust-link-inputs.txt");
    println!("cargo:rerun-if-changed={}", inputs.display());
    for input in fs::read_to_string(&inputs).expect("run scripts/assemble-coreutils.py first").lines() {
        println!("cargo:rustc-link-arg={input}");
        if !input.starts_with('-') { println!("cargo:rerun-if-changed={input}"); }
    }
}
