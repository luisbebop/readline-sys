#![allow(unstable)]
extern crate "pkg-config" as pkg_config;

fn main() {
    let mut opts = pkg_config::default_options("readline");
    opts.atleast_version = Some("6.3".to_string());
    match pkg_config::find_library_opts("readline", &opts) {
        Ok(()) => return,
        Err(..) => {}
    }
}
