#![feature(path_ext)]
use std::env;
use std::fs::{self,PathExt};
use std::path::{Path,PathBuf};
use std::process::{Command,Stdio};

fn main() {
    let target = env::var("TARGET").unwrap();

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
        let src = PathBuf::from(&env::var_os("CARGO_MANIFEST_DIR").unwrap())
            .join("readline");
        let dst = PathBuf::from(&env::var_os("OUT_DIR").unwrap()).join("build");
        let _ = fs::create_dir(&dst);

        let cflags = env::var("CFLAGS").unwrap_or(String::new());

        if mingw {
            run(Command::new("sh")
                .arg("-c")
                .arg("configure")
                .current_dir(&src));
        } else {
            run(Command::new("./configure").current_dir(&src));
        }

        if mingw {
            run(Command::new("sh")
                .arg("-c")
                .arg("make")
                .env("CFLAGS", &cflags[..])
                .current_dir(&src));
        } else {
            run(Command::new("make")
                .env("CFLAGS", &cflags[..])
                .current_dir(&src));
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
    assert!(cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success());
}
