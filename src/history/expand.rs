//! [2.3.7 History Expansion](https://goo.gl/OHS0L3)
//!
//! These functions implement history expansion.
use libc::c_char;
use std::ffi::{CStr, CString};

mod ext_expand {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn history_expand(s: *mut c_char, out: *mut *mut c_char) -> c_int;
    // pub fn get_history_event(arg1: *const c_char,
    //                          arg2: *mut c_int, arg3: c_int) -> *mut c_char;
    // pub fn history_tokenize(arg1: *const c_char) -> *mut *mut c_char;
    // pub fn history_arg_extract(arg1: c_int, arg2: c_int,
    //                            arg3: *const c_char) -> *mut c_char;
    }
}

/// Expand string, placing the result into `output`, a pointer to a string
/// (see section [1.1 History Expansion](https://goo.gl/aR8VB3)).
///
/// Returns:
///
/// * 0  - If no expansions took place (or, if the only change in the text was the removal of
/// escape characters preceding the history expansion character).
/// * 1  - If expansions did take place.
/// * -1 - If there was an error in expansion.
/// * 2 - If the returned line should be displayed, but not executed, as with the :p modifier (see
/// section [1.1.3 Modifiers](https://goo.gl/9HFpWN)).
///
/// If an error occurred in expansion, then output contains a descriptive error message.
///
/// # Examples
///
/// ```
///
/// ```
pub fn expand(s: &str) -> Result<(isize, String), ::HistoryError> {
    ::history::mgmt::init();

    unsafe {
        let ptr = try!(CString::new(s)).into_raw();
        let mut output_ptr: *mut c_char = try!(CString::new("")).into_raw();
        let res = ext_expand::history_expand(&mut *ptr, &mut output_ptr);
        let out = CStr::from_ptr(&mut *output_ptr);
        Ok((res as isize, out.to_string_lossy().into_owned()))
    }
}

pub fn get_event() {}

pub fn tokenize() {}

pub fn arg_extract() {}

#[cfg(test)]
mod test {
    use history::listmgmt;
    use super::*;

    #[test]
    fn test_expand() {
        ::history::mgmt::init();
        assert!(listmgmt::add("ls -al").is_ok());
        let (res, out) = expand("!ls").unwrap();
        assert!(res == 1);
        assert!(out == "ls -al");
    }
}
