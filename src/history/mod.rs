//! Readline History API
use libc::c_int;
use std::ffi::CString;
// use std::fs::{self, File, OpenOptions};
// use std::io::{BufRead, BufReader, LineWriter, Write};
// use std::path::Path;
use std::sync::{ONCE_INIT, Once};

static START: Once = ONCE_INIT;

fn init() {
    START.call_once(|| {
        debug!("readline_sys initialized");
        using_history();
    });
}

mod ext_history {
    use libc::{c_char, c_int};

    extern {
        pub fn add_history(line: *const c_char);
        pub fn clear_history();
        pub fn history_is_stifled() -> c_int;
        pub fn stifle_history(max: c_int);
        pub fn unstifle_history() -> c_int;
        pub fn using_history();
    }
}

/// Wraps the libreadline add_history functionality.  The argument is the line
/// to add to history.
///
/// # Examples
///
/// ```
/// use rl_sys::history;
///
/// match history::add_history("ls -al") {
///     Ok(_)  => println!("Success!"),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn add_history(line: &str) -> Result<(), ::ReadlineError> {
    unsafe {
        let cline = try!(CString::new(line.as_bytes()));
        init();
        ext_history::add_history(cline.as_ptr());
        Ok(())
    }
}

/// Clear the history list by deleting all the entries.
pub fn clear_history() {
    unsafe {
        init();
        ext_history::clear_history();
    }
}

/// Is the history stifled?
///
/// # Examples
///
/// ```
/// use rl_sys::history;
///
/// assert!(!history::history_is_stifled());
/// history::stifle_history(1);
/// assert!(history::history_is_stifled());
/// ```
pub fn history_is_stifled() -> bool {
    unsafe {
        init();
        ext_history::history_is_stifled() != 0
    }
}

/// Stifle the history list, remembering only the last *max* entries.
pub fn stifle_history(max: i32) {
    unsafe {
        init();
        ext_history::stifle_history(max as c_int);
    }
}

/// Stop stifling the history.
///
/// This returns the previously-set maximum number of history entries (as set by stifle_history()).
///
/// # Examples
///
/// ```
/// use rl_sys::history;
///
/// let max = 5;
/// history::stifle_history(max);
/// assert_eq!(max, history::unstifle_history());
///
/// ```
pub fn unstifle_history() -> i32 {
    unsafe {
        init();
        ext_history::unstifle_history()
    }
}

/// Begins a session in which the history functions might be used.
///
/// This initializes the interactive variables.
fn using_history() {
    unsafe { ext_history::using_history() }
}

// /// Preload the readline history with lines from the given file.  This is often
// /// use in conjunction with the *add_history_persist* api to maintain a readline
// /// history persistently.
// ///
// /// # Examples
// ///
// /// ```
// /// use rl_sys;
// /// use std::path::Path;
// ///
// /// let history_file = Path::new("/home/user/.app-hist");
// /// match rl_sys::preload_history(&history_file) {
// ///     Ok(_)  => println!("Success!"),
// ///     Err(e) => println!("{}", e),
// /// }
// /// ```
// pub fn preload_history(file: &Path) -> Result<(), ReadlineError> {
//     let exists = match fs::metadata(file) {
//         Ok(meta) => meta.is_file(),
//         Err(e)   => {
//             error!("{:?}", e);
//             false
//         },
//     };
//
//     if exists {
//         let file = BufReader::new(File::open(file).unwrap());
//         for opt in file.lines() {
//             match opt {
//                 Ok(o) => try!(add_history(&o[..])),
//                 Err(e) => {
//                     error!("{:?}", e);
//                     return Err(ReadlineError::new("ReadlineError", e))
//                 },
//             }
//         }
//     }
//
//     Ok(())
// }
//
// /// Add the given line to readline history and persistently to a file at the
// /// given path.  This is useful in conjunction with the *preload_history*
// /// function for keeping a useful history for your application.
// ///
// /// Note that this function will only add the line to the readline history and
// /// the file history if it doesn't already exist there.
// ///
// /// # Examples
// ///
// /// ```
// /// use rl_sys;
// /// use std::path::Path;
// ///
// /// let history_file = Path::new("/home/user/.app-hist");
// /// match rl_sys::add_history_persist("ls -al", &history_file) {
// ///     Ok(_)  => println!("Success!"),
// ///     Err(e) => println!("{}", e),
// /// }
// /// ```
// pub fn add_history_persist(
//     line: &str,
//     file: &Path
// ) -> Result<(), ReadlineError> {
//     let exists = match fs::metadata(file) {
//         Ok(meta) => meta.is_file(),
//         Err(e)   => {
//             error!("{:?}", e);
//             false
//         },
//     };
//
//     let mut write = LineWriter::new(if exists {
//         try!(OpenOptions::new().append(true).write(true).open(file))
//     } else {
//         try!(File::create(file))
//     });
//
//     // Only add the line to the history file if it doesn't already
//     // contain the line to add.
//     let read = BufReader::new(try!(File::open(file)));
//     // The lines method returns strings without the trailing '\n'
//     let mut cmds: Vec<String> = Vec::new();
//
//     for line in read.lines() {
//         match line {
//             Ok(l)  => cmds.push(l),
//             Err(e) => {
//                 error!("{:?}", e);
//                 return Err(ReadlineError::new("ReadlineError", e))
//             },
//         }
//     }
//
//     let trimmed = line.trim_right().to_string();
//
//     // Only add the line to history if it doesn't exist already and isn't empty.
//     if !cmds.contains(&trimmed) && !trimmed.is_empty() {
//         // Write the line with the trailing '\n' to the file.
//         try!(write.write(line.as_bytes()));
//     }
//
//     // Add the line witout the trailing '\n' to the readline history.
//     try!(add_history(&trimmed[..]));
//     Ok(())
// }

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_history() {
        assert!(add_history("test").is_ok());
    }

    #[test]
    fn test_stifle() {
        // History should not begin stifled.
        assert!(!history_is_stifled());

        let max = 5;
        stifle_history(max);
        assert!(history_is_stifled());

        assert_eq!(max, unstifle_history());
        assert!(!history_is_stifled());
    }
}
