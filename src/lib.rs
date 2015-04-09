#![feature(libc,path_ext)]
extern crate libc;

use std::ffi::{CStr,CString};
use std::io::prelude::*;
use std::fs::{File,OpenOptions};
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
        let cline = CString::new(&(line.as_bytes())[..]).unwrap();
        ext_readline::add_history(cline.as_ptr());
    }
}

pub fn readline(prompt: String) -> Option<String> {
    let cprmt = CString::new(&(prompt.as_bytes())[..]).unwrap();
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
        for opt in file.lines() {
            add_history(opt.unwrap());
        }
    }
}

pub fn add_history_persist(line: String, file: &Path) {
    let mut write = LineWriter::new(if file.exists() {
        let mut oo = OpenOptions::new();
        oo.append(true);
        oo.write(true);
        oo.open(file).unwrap()
    } else {
        File::create(file).unwrap()
    });

    // Only add the line to the history file if it doesn't already
    // contain the line to add.
    let read = BufReader::new(File::open(file).unwrap());
    // The lines method returns strings without the trailing '\n'
    let cmds: Vec<String> = read.lines().map(|l| l.unwrap()).collect();
    let trimmed = line.trim_right().to_string();

    // Only add the line to history if it doesn't exist already and isn't empty.
    if !cmds.contains(&trimmed) && !trimmed.is_empty() {
        // Write the line with the trailing '\n' to the file.
        let _ = write.write(line.as_bytes());
    }

    // Add the line witout the trailing '\n' to the readline history.
    add_history(trimmed);
}

#[cfg(test)]
mod test {
    use super::add_history;

    #[test]
    fn test_readline() {
        add_history("test".to_string());
    }
}
