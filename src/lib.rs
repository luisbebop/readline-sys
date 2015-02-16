#![feature(core,io,libc,path,std_misc)]
extern crate libc;

use std::ffi::{c_str_to_bytes,CString};
use std::old_io::{
    Append,
    BufferedReader,
    File,
    LineBufferedWriter,
    Truncate,
    Write
};
use std::old_io::fs::PathExtensions;
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
        let cline = CString::from_slice(line.as_bytes().as_slice());
        ext_readline::add_history(cline.as_ptr());
    }
}

pub fn readline(prompt: String) -> Option<String> {
    let cprmt = CString::from_slice(prompt.as_bytes().as_slice());
    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {  // user pressed Ctrl-D
            None
        } else {
            let slice = c_str_to_bytes(&ret);
            let res = str::from_utf8(slice).ok().expect("Failed to parse utf-8");
            Some(res.to_string())
        }
    }
}

pub fn preload_history(file: &Path) {
    if file.exists() {
        let mut file = BufferedReader::new(File::open(file));
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
    let mut write = LineBufferedWriter::new(
        if file.exists() {
            File::open_mode(file, Append, Write)
        } else {
            File::open_mode(file, Truncate, Write)
        });

    // Only add the line to the history file if it doesn't already
    // contain the line to add.
    let mut read = BufferedReader::new(File::open(file));
    let cmds: Vec<String> = read.lines().map(|l| l.unwrap()).collect();
    let mut trimmed: Vec<&str> = cmds.iter().map(|c| c.trim_right()).collect();
    trimmed.dedup();

    if !trimmed.contains(&line.trim_right()) {
        let _ = write.write_line(line.as_slice());
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
