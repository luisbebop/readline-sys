#![allow(unstable)]
use std::io::{self, fs, Command};
use std::io::fs::PathExtensions;
use std::io::process::InheritFd;
use std::os;

fn main() {
    let target = os::getenv("TARGET").unwrap();
    let mingw = target.contains("windows-gnu");

    let libpath = if mingw {
        Path::new("C:/msys64/usr/lib")
    } else {
        Path::new("/usr/lib")
    };

    if !mingw && libpath.join("libreadline.a").exists() {
        println!("cargo:rustc-flags=-l static=readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else if !mingw && libpath.join("libreadline.so").exists() {
        println!("cargo:rustc-flags=-l readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else {
        let src = Path::new(os::getenv("CARGO_MANIFEST_DIR").unwrap())
            .join("readline");
        let dst = Path::new(os::getenv("OUT_DIR").unwrap()).join("build");
        let _ = fs::mkdir(&dst, io::USER_DIR);

        let mut cflags = os::getenv("CFLAGS").unwrap_or(String::new());

        cflags.push_str(" -ffunction-sections -fdata-sections");

        if target.contains("i686") {
            cflags.push_str(" -m32");
        } else if target.as_slice().contains("x86_64") {
            cflags.push_str(" -m64");
        }
        if !target.contains("i686") {
            cflags.push_str(" -fPIC");
        }

        if mingw {
            run(Command::new("sh")
                .arg("-c")
                .arg("configure")
                .env("CFLAGS", "-D_POSIX")
                .cwd(&src));
        } else {
            run(Command::new("configure").cwd(&src));
        }

        if mingw {
            run(Command::new("sh")
                .arg("-c")
                .arg("make")
                .env("CFLAGS", cflags.as_slice())
                .cwd(&src));
        } else {
            run(Command::new("make")
                .env("CFLAGS", cflags.as_slice())
                .cwd(&src));
        }

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
