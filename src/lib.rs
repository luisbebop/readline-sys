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
//! ```no_run
//! # extern crate rl_sys;
//! # fn main() {
//! loop {
//!     let input = rl_sys::readline("$ ") {
//!         Some(s) => s,
//!         Some("clear".to_owned()) => {
//!             rl_sys::clear_history();
//!             continue;
//!         }
//!         None => break,  // EOF encountered
//!     };
//!     println!("{}", input);
//!
//!     // Add input to history.
//!     rl_sys::add_history(input);
//! }
//! # }
//! ```
extern crate libc;
#[macro_use] extern crate log;
#[cfg(test)] extern crate sodium_sys;

pub use error::ReadlineError;
use std::ffi::{CStr, CString};
use std::fs::{self, File, OpenOptions};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::Path;
use std::str;
pub use version::version;

mod error;
mod ext_readline {
    use libc::{c_char, c_int};

    extern {
        pub fn readline(p: *const c_char) -> *const c_char;
        pub fn add_history(line: *const c_char);
        pub fn clear_history();
        pub fn stifle_history(max: c_int);
        pub fn unstifle_history() -> c_int;
        pub fn history_is_stifled() -> c_int;
    }
}
mod version;

/// Wraps the libreadline add_history functionality.  The argument is the line
/// to add to history.
///
/// # Examples
///
/// ```
/// use rl_sys;
///
/// match rl_sys::add_history("ls -al") {
///     Ok(_)  => println!("Success!"),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn add_history(line: &str) -> Result<(), ReadlineError> {
    unsafe {
        let cline = try!(CString::new(line.as_bytes()));
        ext_readline::add_history(cline.as_ptr());
        Ok(())
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
/// use rl_sys;
///
/// loop {
///     match rl_sys::readline("$ ") {
///         Ok(o) => match o {
///             Some(s) => println!("{}", s),
///             None    => break,
///         },
///        Err(e) => {
///            println!("{}", e);
///            break
///        },
///     }
///
/// }
/// ```
pub fn readline(prompt: &str) -> Result<Option<String>, ReadlineError> {
    let cprmt = try!(CString::new(prompt.as_bytes()));

    unsafe {
        let ret = ext_readline::readline(cprmt.as_ptr());
        if ret.is_null() {  // user pressed Ctrl-D
            Ok(None)
        } else {
            let slice = CStr::from_ptr(ret);
            let res = try!(str::from_utf8(slice.to_bytes()));
        } if ret.is_null() {
            // user pressed Ctrl-D
            None
        } else {
            let slice = CStr::from_ptr(ret);
            let res = str::from_utf8(slice.to_bytes())
                          .ok()
                          .expect("Failed to parse utf-8");
            Some(res.to_string())
        }
    }
}

/// Preload the readline history with lines from the given file.  This is often
/// use in conjunction with the *add_history_persist* api to maintain a readline
/// history persistently.
///
/// # Examples
///
/// ```
/// use rl_sys;
/// use std::path::Path;
///
/// let history_file = Path::new("/home/user/.app-hist");
/// match rl_sys::preload_history(&history_file) {
///     Ok(_)  => println!("Success!"),
///     Err(e) => println!("{}", e),
/// }
/// ```
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
                Ok(o) => try!(add_history(&o[..])),
                Err(e) => {
                    error!("{:?}", e);
                    return Err(ReadlineError::new("ReadlineError", e))
                },
            }
        }
    }

    Ok(())
}

/// Add the given line to readline history and persistently to a file at the
/// given path.  This is useful in conjunction with the *preload_history*
/// function for keeping a useful history for your application.
///
/// Note that this function will only add the line to the readline history and
/// the file history if it doesn't already exist there.
///
/// # Examples
///
/// ```
/// use rl_sys;
/// use std::path::Path;
///
/// let history_file = Path::new("/home/user/.app-hist");
/// match rl_sys::add_history_persist("ls -al", &history_file) {
///     Ok(_)  => println!("Success!"),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn add_history_persist(
    line: &str,
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
        try!(OpenOptions::new().append(true).write(true).open(file))
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
                return Err(ReadlineError::new("ReadlineError", e))
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
    try!(add_history(&trimmed[..]));
    Ok(())
}

/// Clear the history list by deleting all the entries.
pub fn clear_history() {
    unsafe {
        ext_readline::clear_history();
    }
}

/// Stifle the history list, remembering only the last *max* entries.
pub fn stifle_history(max: i32) {
    unsafe {
        ext_readline::stifle_history(max as libc::c_int);
    }
}

/// Stop stifling the history.
///
/// This returns the previously-set maximum number of history entries (as set by stifle_history()).
/// The value is positive if the history was stifled, negative if it wasn't.
///
/// # Examples
///
/// ```
/// # extern crate rl_sys;
/// # fn main() {
/// assert!(rl_sys::unstifle_history().is_negative());
///
/// let max = 5;
/// rl_sys::stifle_history(max);
/// assert_eq!(max, rl_sys::unstifle_history());
/// # }
/// ```
pub fn unstifle_history() -> i32 {
    unsafe { ext_readline::unstifle_history() as i32 }
}

/// Is the history stifled?
///
/// # Examples
///
/// ```
/// # extern crate rl_sys;
/// # fn main() {
/// assert!(!rl_sys::history_is_stifled());
/// rl_sys::stifle_history(1);
/// assert!(rl_sys::history_is_stifled());
/// # }
/// ```
pub fn history_is_stifled() -> bool {
    unsafe { ext_readline::history_is_stifled() != 0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_history() {
        add_history("test".to_owned());
    }

    #[test]
    fn test_stifle() {
        // History should not begin stifled.
        assert!(!history_is_stifled());
        assert!(unstifle_history().is_negative());

        let max = 5;
        stifle_history(max);
        assert!(history_is_stifled());

        assert_eq!(max, unstifle_history());
        assert!(!history_is_stifled());
        assert!(unstifle_history().is_negative());
    }
}
