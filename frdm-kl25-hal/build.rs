#![allow(missing_docs)]

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Put `linker.x` in our output directory and ensure it's
    // on the linker search path.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("linker.x"))
        .unwrap()
        .write_all(include_bytes!("linker.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}
