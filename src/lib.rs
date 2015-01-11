#![allow(unstable)]
extern crate libc;

use std::ffi::{c_str_to_bytes,CString};
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

#[cfg(test)]
mod test {
    use super::readline;

    #[test]
    fn test_readline() {
        let blah = readline("rh >".to_string()).unwrap();
        println!("test {}", blah);
    }
}
