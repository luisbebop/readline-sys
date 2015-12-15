//! Readline API
use std::ffi::{CStr, CString};
use std::str;

mod ext_readline {
    use libc::c_char;

    extern "C" {
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}

/// Wraps the libreadline readline function.  The argument is the prompt to use.
///
/// If readline encounters an `EOF` while reading the line, and the line is empty at that point,
/// then `Ok(None)` is returned. Otherwise, the line is ended just as if a newline has been typed.
///
/// # Examples
///
/// ```
/// use rl_sys::readline;
///
/// loop {
///     match readline::readline("$ ") {
///         Ok(Some(s)) => println!("{}", s),
///         Ok(None) => break,
///         Err(e) => {
///             println!("{}", e);
///             break;
///        },
///     }
/// }
/// ```
pub fn readline(prompt: &str) -> Result<Option<String>, ::ReadlineError> {
    let cprmt = try!(CString::new(prompt.as_bytes()));

    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {
            // user pressed Ctrl-D
            Ok(None)
        } else {
            let slice = CStr::from_ptr(ret);
            let res = try!(str::from_utf8(slice.to_bytes()));
            Ok(Some(res.to_owned()))
        }
    }
}
