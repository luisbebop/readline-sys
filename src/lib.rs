#![feature(core,io,libc,path,path_ext)]
extern crate libc;

use std::ffi::{CStr,CString};
use std::io::prelude::*;
use std::fs::File;
use std::io::{BufReader,LineWriter};
use std::path::Path;
use std::str;

mod ext_readline {
    use libc::c_char;

    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn add_history(line: String) {
    unsafe {
        let cline = CString::new(line.as_bytes().as_slice()).unwrap();
        ext_readline::add_history(cline.as_ptr());
    }
}

pub fn readline(prompt: String) -> Option<String> {
    let cprmt = CString::new(prompt.as_bytes().as_slice()).unwrap();
    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {  // user pressed Ctrl-D
            None
        } else {
            let slice = CStr::from_ptr(ret);
            let res = str::from_utf8(slice.to_bytes())
                .ok().expect("Failed to parse utf-8");
            Some(res.to_string())
        }
    }
}

pub fn preload_history(file: &Path) {
    if file.exists() {
        let file = BufReader::new(File::open(file).unwrap());
        let mut lines = Vec::new();
        for opt in file.lines() {
            lines.push(opt.unwrap());
        }

        lines.dedup();

        for line in lines.iter() {
            let trimmed = line.trim_right();
            add_history(trimmed.to_string());
        }
    }
}

pub fn add_history_persist(line: String, file: &Path) {
    let mut write = LineWriter::new(File::create(file).unwrap());

    // Only add the line to the history file if it doesn't already
    // contain the line to add.
    let read = BufReader::new(File::open(file).unwrap());
    let cmds: Vec<String> = read.lines().map(|l| l.unwrap()).collect();
    let mut trimmed: Vec<&str> = cmds.iter().map(|c| c.trim_right()).collect();
    trimmed.dedup();

    if !trimmed.contains(&line.trim_right()) {
        let _ = write.write(line.as_bytes());
        add_history(line);
    }
}

#[cfg(test)]
mod test {
    use super::add_history;

    #[test]
    fn test_readline() {
        add_history("test".to_string());
    }
}
