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
        let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap())
            .join("readline");
        let dst = Path::new(os::getenv("OUT_DIR").unwrap()).join("build");
        let _ = fs::mkdir(&dst, io::USER_DIR);

        let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());
        let target = os::getenv("TARGET").unwrap();
        let mingw = target.contains("windows-gnu");

        cflags.push_str(" -ffunction-sections -fdata-sections");

        if target.contains("i686") {
            cflags.push_str(" -m32");
        } else if target.as_slice().contains("x86_64") {
            cflags.push_str(" -m64");
        }
        if !target.contains("i686") {
            cflags.push_str(" -fPIC");
        }

        run(Command::new("./configure").cwd(&src));
        run(Command::new("make").env("CFLAGS", cflags.as_slice()).cwd(&src));

        let shlib = src.join("shlib");
        let _ = fs::copy(&shlib.join("libreadline.so.6.3"),
                         &dst.join("libreadline.so.6.3"));
        let _ = fs::copy(&shlib.join("libhistory.so.6.3"),
                         &dst.join("libhistory.so.6.3"));

        println!("cargo:rustc-flags=-l readline");
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
