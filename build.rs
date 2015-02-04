#![feature(core,env,io,path)]
use std::env;
use std::old_io::{self, fs, Command};
use std::old_io::fs::PathExtensions;
use std::old_io::process::InheritFd;

fn main() {
    let target = env::var_string("TARGET").unwrap();

    let mingw = target.contains("windows-gnu");

    let libpath = if mingw {
        Path::new("C:/mingw64/bin")
    } else {
        Path::new("/usr/lib")
    };

    if mingw && libpath.join("libreadline6.dll").exists() {
        println!("cargo:rustc-flags=-l readline6");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else if !mingw && libpath.join("libreadline.a").exists() {
        println!("cargo:rustc-flags=-l static=readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else if !mingw && libpath.join("libreadline.so").exists() {
        println!("cargo:rustc-flags=-l readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else {
        let src = Path::new(env::var_string("CARGO_MANIFEST_DIR").unwrap())
            .join("readline");
        let dst = Path::new(env::var_string("OUT_DIR").unwrap()).join("build");
        let _ = fs::mkdir(&dst, old_io::USER_DIR);

        let cflags = env::var_string("CFLAGS").unwrap_or(String::new());

        if mingw {
            run(Command::new("sh")
                .arg("-c")
                .arg("configure")
                .cwd(&src));
        } else {
            run(Command::new("./configure").cwd(&src));
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
        if mingw {
            let _ = fs::copy(&shlib.join("libreadline6.dll"),
                             &dst.join("libreadline.dll"));
            let _ = fs::copy(&shlib.join("libhistory6.dll"),
                             &dst.join("libhistory.dll"));
            let _ = fs::copy(&shlib.join("libreadline6.dll.a"),
                             &dst.join("libreadline.dll.a"));
            let _ = fs::copy(&shlib.join("libhistory6.dll.a"),
                             &dst.join("libhistory.dll.a"));
        } else {
            let _ = fs::copy(&shlib.join("libreadline.so.6.3"),
                             &dst.join("libreadline.so.6.3"));
            let _ = fs::copy(&shlib.join("libhistory.so.6.3"),
                             &dst.join("libhistory.so.6.3"));
        }

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
