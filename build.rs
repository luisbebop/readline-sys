extern crate pkg_config;
extern crate vergen;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use vergen::*;

fn main() {
    let mut flags = Flags::all();
    flags.toggle(NOW);
    vergen(flags);

    let latest = env::var("CARGO_FEATURE_LATEST").is_ok();
    if latest {
        build_readline();
    } else {
        let has_pkgconfig = Command::new("pkg-config").output().is_ok();

        if has_pkgconfig && pkg_config::find_library("libreadline").is_ok() {
            return;
        } else {
            build_readline();
        }
    }
}

fn build_readline() {
    let manifest_dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(d) => d,
        None => panic!("Unable to read manifest dir"),
    };
    let out_dir = match env::var_os("OUT_DIR") {
        Some(d) => d,
        None => panic!("Unable to read output dir"),
    };
    let src = PathBuf::from(&manifest_dir).join("readline");
    let dst = PathBuf::from(&out_dir).join("build");
    let _ = fs::create_dir(&dst);

    run(Command::new("./configure").current_dir(&src));
    run(Command::new("make").current_dir(&src));
    let shlib = &src.join("shlib");
    let _ = fs::copy(&shlib.join("libreadline.so.6.3"),
                     &dst.join("libreadline.so"));

    println!("cargo:rustc-link-lib=readline");
    println!("cargo:rustc-link-lib=curses");
    println!("cargo:rustc-flags=-L {}", dst.display());
}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
