use std::env;
use std::fs;
use std::path::{Path,PathBuf};
use std::process::{Command,Stdio};

fn main() {
    let libpath = Path::new("/usr/lib");

    let statik = match fs::metadata(libpath.join("libreadline.a")) {
        Ok(meta) => meta.is_file(),
        Err(_)   => false,
    };

    let dyn = match fs::metadata(libpath.join("libreadline.so")) {
        Ok(meta) => meta.is_file(),
        Err(_)   => false,
    };

    if statik {
        println!("cargo:rustc-flags=-l static=readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else if dyn {
        println!("cargo:rustc-flags=-l readline");
        println!("cargo:rustc-flags=-L {}", libpath.display());
    } else {
        let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
            Ok(md) => md,
            Err(e) => panic!("manifest dir not defined! {}", e),
        };

        let out_dir = match env::var("OUT_DIR") {
            Ok(od) => od,
            Err(e) => panic!("manifest dir not defined! {}", e),
        };
        let src = PathBuf::from(manifest_dir).join("readline");
        let dst = PathBuf::from(out_dir).join("build");
        let _ = fs::create_dir(&dst);

        let cflags = env::var("CFLAGS").unwrap_or(String::new());

        run(Command::new("./configure").current_dir(&src));
        run(Command::new("make")
            .env("CFLAGS", &cflags[..])
            .current_dir(&src));

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
    assert!(cmd.stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status()
            .unwrap()
            .success());
}
