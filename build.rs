extern crate pkg_config;
extern crate vergen;

use std::env;
use std::fs;
// #[cfg(unix)]
// use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};
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

        if has_pkgconfig && pkg_config::find_library("libreadline").is_ok() && pkg_config::find_library("libhistory").is_ok() {
            return
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

    run(Command::new("./configure").env("CFLAGS","-fPIC").env("CPPFLAGS", "-fPIC").current_dir(&src));
    run(Command::new("make").current_dir(&src));

    let _ = fs::copy(&src.join("libreadline.a"),
                     &dst.join("libreadline.a"));
    let _ = fs::copy(&src.join("libhistory.a"),
                     &dst.join("libhistory.a"));

    // create_symlinks(dst.as_path());

    println!("cargo:rustc-link-lib=static=readline");
    println!("cargo:rustc-link-lib=curses");
    println!("cargo:rustc-flags=-L {}", dst.display());
}

// #[cfg(unix)]
// fn create_symlinks(dst: &Path) {
//     let _ = symlink(dst.join("libreadline.so.6.3"), dst.join("libreadline.so"));
//     let _ = symlink(dst.join("libhistory.so.6.3"), dst.join("libhistory.so"));
// }
//
// #[cfg(windows)]
// fn create_symlinks(_dst: &Path) {}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
