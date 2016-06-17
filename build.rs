// Copyright 2015-2016 Jason Ozias
//
// This file is part of rl-sys.
//
// rl-sys is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rl-sys is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rl-sys.  If not, see <http://www.gnu.org/licenses/>.

extern crate pkg_config;
extern crate vergen;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use vergen::*;

fn main() {
    let mut flags = OutputFns::all();
    flags.toggle(NOW);
    assert!(vergen(flags).is_ok());

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
    let dst_file = dst.join("libreadline.a");
    if !dst_file.exists() {
        run(Command::new("./configure")
                .env("CFLAGS", "-fPIC")
                .env("CPPFLAGS", "-fPIC")
                .current_dir(&src));
        run(Command::new("make").current_dir(&src));
        let _ = fs::copy(&src.join("libreadline.a"), &dst_file);
    }
    println!("cargo:rustc-link-lib=static=readline");
    println!("cargo:rustc-link-lib=ncurses");
    println!("cargo:rustc-flags=-L {}", dst.display());
}

fn run(cmd: &mut Command) {
    assert!(cmd.stdout(Stdio::inherit())
               .stderr(Stdio::inherit())
               .status()
               .unwrap()
               .success());
}
