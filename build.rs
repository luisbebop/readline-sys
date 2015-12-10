extern crate vergen;

use std::env;
use std::fs;
use std::path::{Path,PathBuf};
use std::process::{Command,Stdio};
use vergen::*;

fn main() {
    let mut flags = Flags::all();
    flags.toggle(NOW);
    vergen(flags);
    
    let target = match env::var("TARGET") {
        Ok(t)  => t,
        Err(e) => panic!("Unable to read TARGET env: {}", e),
    };
    let mingw = target.contains("windows-gnu");

    let libpath = if mingw {
        Path::new("C:/mingw64/bin")
    } else {
        Path::new("/usr/lib")
    };

    let dll = match fs::metadata(libpath.join("libreadline6.dll")) {
        Ok(meta) => meta.is_file(),
        Err(_)   => false,
    };

    let statik = match fs::metadata(libpath.join("libreadline.a")) {
        Ok(meta) => meta.is_file(),
        Err(_)   => false,
    };

    let dyn = match fs::metadata(libpath.join("libreadline.so")) {
        Ok(meta) => meta.is_file(),
        Err(_)   => false,
    };

    if mingw && dll {
        println!("cargo:rustc-flags=-l readline6");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else if !mingw && statik {
        println!("cargo:rustc-flags=-l static=readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else if !mingw && dyn {
        println!("cargo:rustc-flags=-l readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else {
        let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
            Some(d) => d,
            None    => panic!("Unable to read manifest dir"),
        };
        let out_dir = match env::var_os("OUT_DIR") {
            Some(d) => d,
            None    => panic!("Unable to read output dir"),
        };
        let src = PathBuf::from(&manifest_dir).join("readline");
        let dst = PathBuf::from(&out_dir).join("build");
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
