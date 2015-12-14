//! This library provides native bindings for the [GNU readline library][1].
//!
//! [1]: https://cnswww.cns.cwru.edu/php/chet/readline/rltop.html
//!
//! The GNU Readline library provides a set of functions for use by applications
//! that allow users to edit command lines as they are typed in. Both Emacs and
//! vi editing modes are available. The Readline library includes additional
//! functions to maintain a list of previously-entered command lines, to recall
//! and perhaps reedit those lines, and perform csh-like history expansion on
//! previous commands.
//!
//! # Examples
//!
//! ```
//! use rl_sys::readline;
//! use rl_sys::history::listmgmt;
//!
//! loop {
//!     let input = match readline("$ ") {
//!         Ok(Some(s)) => match &*s {
//!             "clear" => {
//!                 listmgmt::clear_history();
//!                 continue;
//!             }
//!             _ => s
//!         },
//!         Ok(None) => break,  // EOF encountered
//!         Err(e) => {
//!             println!("{}", e);
//!             continue;
//!         }
//!     };
//!     println!("{}", input);
//!
//!     // Add input to history.
//!     let _ = listmgmt::add_history(&input);
//! }
//! ```
extern crate libc;
#[macro_use]
extern crate log;
#[cfg(test)]
extern crate sodium_sys;
extern crate time;

pub use error::{HistoryError, ReadlineError};
use std::ffi::{CStr, CString};
use std::str;

pub use version::version;

mod error;
mod ext_readline {
    use libc::c_char;

    extern "C" {
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}
pub mod history;
mod version;

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
///     match readline("$ ") {
///         Ok(Some(s)) => println!("{}", s),
///         Ok(None) => break,
///         Err(e) => {
///             println!("{}", e);
///             break;
///        },
///     }
/// }
/// ```
pub fn readline(prompt: &str) -> Result<Option<String>, ReadlineError> {
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
