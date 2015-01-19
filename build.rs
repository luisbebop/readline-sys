#![allow(unstable)]
use std::io::{self, fs, Command};
use std::io::fs::PathExtensions;
use std::io::process::InheritFd;
use std::os;

fn main() {
    let libpath = Path::new("/usr/lib");

    if libpath.join("libreadline.a").exists() {
        println!("cargo:rustc-flags=-l static=readline");
    } else if libpath.join("libreadline.so").exists() {
        println!("cargo:rustc-flags=-l readline");
    } else {
        let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());
        let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap());
        let dst = Path::new(os::getenv("OUT_DIR").unwrap()).join("build");
        let _ = fs::mkdir(&dst, io::USER_DIR);

        cflags.push_str("-fPIC");

        run(Command::new("./configure")
            .cwd(&src.join("readline"))
            .arg("--disable-shared"));

        run(Command::new("make")
            .env("CFLAGS", cflags.as_slice())
            .cwd(&src.join("readline")));

        let _ = fs::copy(&src.join("readline").join("libreadline.a"), &dst.join("libreadline.a"));
        let _ = fs::copy(&src.join("readline").join("libhistory.a"), &dst.join("libhistory.a"));
        println!("cargo:rustc-flags=-l static=readline");
        println!("cargo:rustc-flags=-L {}", dst.display());
    }
}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(InheritFd(1))
            .stderr(InheritFd(2))
            .status()
            .unwrap()
            .success());
}
