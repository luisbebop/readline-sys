#![allow(unstable)]
use std::io::{self, fs, Command};
use std::io::fs::PathExtensions;
use std::os;

fn main() {
    let libpath = Path::new("/usr/lib");

    if libpath.join("libreadline.a").exists() {
        println!("cargo:rustc-flags=-l static=readline");
    } else if libpath.join("libreadline.so").exists() {
        println!("cargo:rustc-flags=-l readline");
    } else {
        let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap())
            .join("readline");
        let dst = Path::new(os::getenv("OUT_DIR").unwrap()).join("build");
        let _ = fs::mkdir(&dst, io::USER_DIR);

        run(Command::new("configure")
            .cwd(&src)
            .arg("--disable-shared"));

        run(Command::new("make")
            .cwd(&src));

        let _ = fs::copy(&src.join("libreadline.a"), &dst.join("libreadline.a"));
        let _ = fs::copy(&src.join("libhistory.a"), &dst.join("libhistory.a"));
        println!("cargo:rustc-flags=-l static=readline");
        println!("cargo:rustc-flags=-L {}", dst.display());
    }
}

fn run(cmd: &mut Command) {
    println!("running: {}", cmd);
    assert!(cmd.status().unwrap().success());
}
