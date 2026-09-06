use std::{env, fs, path::PathBuf};

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let inputs = root.join("build/rust-link-inputs.txt");
    println!("cargo:rerun-if-changed={}", inputs.display());
    for input in fs::read_to_string(&inputs).expect("run scripts/assemble-coreutils.py first").lines() {
        println!("cargo:rustc-link-arg={input}");
        if !input.starts_with('-') { println!("cargo:rerun-if-changed={input}"); }
    }
}
