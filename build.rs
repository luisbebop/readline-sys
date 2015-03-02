// build.rs
#![feature(vergen)]
extern crate vergen;

use vergen::vergen;

fn main() {
    vergen();
}
