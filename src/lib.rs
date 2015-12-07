extern crate libc;
#[macro_use] extern crate log;

use std::ffi::{CStr, CString};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::str;

#[derive(Debug)]
pub struct ReadlineError {
    desc: String,
    detail: String,
}

impl ReadlineError {
    pub fn new<T>(desc: &str, detail: T) -> ReadlineError where T: fmt::Debug {
        ReadlineError {
            desc: String::from(desc),
            detail: format!("{:?}", detail),
        }
    }
}

impl From<std::ffi::NulError> for ReadlineError {
    fn from(e: std::ffi::NulError) -> ReadlineError {
        ReadlineError::new("NulError", e)
    }
}

impl From<std::str::Utf8Error> for ReadlineError {
    fn from(e: std::str::Utf8Error) -> ReadlineError {
        ReadlineError::new("FromUtf8Error", e)
    }
}

impl From<std::io::Error> for ReadlineError {
    fn from(e: std::io::Error) -> ReadlineError {
        ReadlineError::new("I/O Error", e)
    }
}

impl fmt::Display for ReadlineError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.desc, self.detail)
    }
}

mod ext_readline {
    use libc::c_char;

    extern {
        pub fn add_history(line: *const c_char);
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

pub fn add_history(line: String) -> Result<(), ReadlineError> {
    unsafe {
        let cline = try!(CString::new(&(line.as_bytes())[..]));
        ext_readline::add_history(cline.as_ptr());
        Ok(())
    }
}

pub fn readline(prompt: String) -> Result<Option<String>, ReadlineError> {
    let cprmt = try!(CString::new(&(prompt.as_bytes())[..]));

    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {  // user pressed Ctrl-D
            Ok(None)
        } else {
            let slice = CStr::from_ptr(ret);
            let res = try!(str::from_utf8(slice.to_bytes()));
            Ok(Some(res.to_string()))
        }
    }
}

pub fn preload_history(file: &Path) -> Result<(), ReadlineError> {
    let exists = match fs::metadata(file) {
        Ok(meta) => meta.is_file(),
        Err(e)   => {
            error!("{:?}", e);
            false
        },
    };

    if exists {
        let file = BufReader::new(File::open(file).unwrap());
        for opt in file.lines() {
            match opt {
                Ok(o) => try!(add_history(o)),
                Err(e) => {
                    error!("{:?}", e);
                    return Err(ReadlineError::new(
                        "ReadlineError",
                        "Unable to preload history!"
                    ))
                },
            }
        }
    }

    Ok(())
}

pub fn add_history_persist(
    line: String,
    file: &Path
) -> Result<(), ReadlineError> {
    let exists = match fs::metadata(file) {
        Ok(meta) => meta.is_file(),
        Err(e)   => {
            error!("{:?}", e);
            false
        },
    };

    let mut write = LineWriter::new(if exists {
        let mut oo = OpenOptions::new();
        oo.append(true);
        oo.write(true);
        try!(oo.open(file))
    } else {
        try!(File::create(file))
    });

    // Only add the line to the history file if it doesn't already
    // contain the line to add.
    let read = BufReader::new(try!(File::open(file)));
    // The lines method returns strings without the trailing '\n'
    let mut cmds: Vec<String> = Vec::new();

    for line in read.lines() {
        match line {
            Ok(l)  => cmds.push(l),
            Err(e) => {
                error!("{:?}", e);
                return Err(ReadlineError {
                    desc: String::from("ReadlineError"),
                    detail: String::from("Unable to parse history file!"),
                })
            },
        }
    }

    let trimmed = line.trim_right().to_string();

    // Only add the line to history if it doesn't exist already and isn't empty.
    if !cmds.contains(&trimmed) && !trimmed.is_empty() {
        // Write the line with the trailing '\n' to the file.
        try!(write.write(line.as_bytes()));
    }

    // Add the line witout the trailing '\n' to the readline history.
    try!(add_history(trimmed));
    Ok(())
}

#[cfg(test)]
mod test {
    use super::add_history;

    #[test]
    fn test_readline() {
        assert!(add_history("test".to_string()).is_ok());
    }
}
