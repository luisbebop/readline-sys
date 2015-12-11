//! Readline History API
use libc::{c_char, c_int, c_void};
use std::clone::Clone;
use std::default::Default;
use std::ffi::{CStr, CString};
use std::fmt;
// use std::fs::{self, File, OpenOptions};
// use std::io::{BufRead, BufReader, LineWriter, Write};
// use std::path::Path;
use std::sync::{ONCE_INIT, Once};
use time;

mod ext_history {
    use super::{HistoryEntry, HistoryState};
    use libc::{
        c_char,
        c_int,
        // c_long,
        // c_void
    };

    extern {
        pub fn using_history() -> ();
        pub fn history_get_history_state() -> *mut HistoryState;
        // pub fn history_set_history_state(arg1: *mut HistoryState) -> ();
        pub fn add_history(arg1: *const c_char) -> ();
        pub fn add_history_time(arg1: *const c_char) -> ();
        // pub fn remove_history(arg1: c_int) -> *mut HistoryEntry;
        // pub fn free_history_entry(arg1: *mut HistoryEntry) -> *mut c_void;
        // pub fn replace_history_entry(arg1: c_int,
        //                              arg2: *const c_char,
        //                              arg3: *mut c_void) -> *mut HistoryEntry;
        pub fn clear_history() -> ();
        pub fn stifle_history(arg1: c_int) -> ();
        pub fn unstifle_history() -> c_int;
        pub fn history_is_stifled() -> c_int;
        // pub fn history_list() -> *mut *mut HistoryEntry;
        // pub fn where_history() -> c_int;
        pub fn current_history() -> *mut HistoryEntry;
        // pub fn history_get(arg1: c_int) -> *mut HistoryEntry;
        // pub fn history_get_time(arg1: *mut HistoryEntry) -> c_long;
        // pub fn history_total_bytes() -> c_int;
        // pub fn history_set_pos(arg1: c_int) -> c_int;
        // pub fn previous_history() -> *mut HistoryEntry;
        pub fn next_history() -> *mut HistoryEntry;
        // pub fn history_search(arg1: *const c_char, arg2: c_int) -> c_int;
        // pub fn history_search_prefix(arg1: *const c_char,
        //                              arg2: c_int) -> c_int;
        // pub fn history_search_pos(arg1: *const c_char,
        //                           arg2: c_int, arg3: c_int) -> c_int;
        // pub fn read_history(arg1: *const c_char) -> c_int;
        // pub fn read_history_range(arg1: *const c_char,
        //                           arg2: c_int, arg3: c_int) -> c_int;
        // pub fn write_history(arg1: *const c_char) -> c_int;
        // pub fn append_history(arg1: c_int, arg2: *const c_char) -> c_int;
        // pub fn history_truncate_file(arg1: *const c_char,
        //                              arg2: c_int) -> c_int;
        // pub fn history_expand(arg1: *mut c_char,
        //                       arg2: *mut *mut c_char) -> c_int;
        // pub fn history_arg_extract(arg1: c_int, arg2: c_int,
        //                            arg3: *const c_char) -> *mut c_char;
        // pub fn get_history_event(arg1: *const c_char,
        //                          arg2: *mut c_int, arg3: c_int) -> *mut c_char;
        // pub fn history_tokenize(arg1: *const c_char) -> *mut *mut c_char;
    }
}

pub type HistoryData = *mut c_void;
#[repr(C)]
#[derive(Copy)]
pub struct HistoryEntry {
    pub line: *mut c_char,
    pub timestamp: *mut c_char,
    pub data: HistoryData,
}

impl Clone for HistoryEntry {
    fn clone(&self) -> Self { *self }
}

impl Default for HistoryEntry {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

impl fmt::Debug for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = unsafe { CStr::from_ptr(self.line).to_string_lossy().into_owned() };
        let time = unsafe { CStr::from_ptr(self.timestamp).to_string_lossy().into_owned() };
        write!(
            f,
            "HistoryEntry {{ line: {:?}, timestamp: {:?}, data: {:?} }}",
            line,
            time,
            self.data
        )
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct HistoryState {
    pub entries: *mut *mut HistoryEntry,
    pub offset: c_int,
    pub length: c_int,
    pub size: c_int,
    pub flags: c_int,
}

impl Clone for HistoryState {
    fn clone(&self) -> Self { *self }
}

impl Default for HistoryState {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

impl fmt::Debug for HistoryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "HistoryState {{ entries: {:?}, offset: {:?}, length: {:?}, size: {:?}, flags: {:?}",
            self.entries,
            self.offset,
            self.length,
            self.size,
            self.flags
        )
    }
}

static START: Once = ONCE_INIT;

fn init() {
    START.call_once(|| {
        debug!("readline_sys initialized");
        using_history();
    });
}

/// Begins a session in which the history functions might be used.
///
/// This initializes the interactive variables.
fn using_history() {
    unsafe { ext_history::using_history() }
}

/// # Examples
/// ```
/// use rl_sys::history;
///
/// let state = history::history_get_history_state();
/// ```
pub fn history_get_history_state<'r>() -> &'r mut HistoryState {
    unsafe {
        init();
        &mut *ext_history::history_get_history_state()
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

pub fn add_history_time() -> Result<(), ::ReadlineError> {
    unsafe {
        let now = time::now();
        let now_str = format!("{}", now.asctime());
        let cline = try!(CString::new(now_str.as_bytes()));
        init();
        ext_history::add_history_time(cline.as_ptr());
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

pub fn current_history<'a>() -> &'a HistoryEntry {
    unsafe {
        init();
        &*ext_history::current_history()
    }
}

pub fn next_history<'a>() -> &'a HistoryEntry {
    unsafe {
        init();
        &*ext_history::next_history()
    }
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
    fn test_history_state() {
        let state = history_get_history_state();
        println!("{:?}", state);
    }

    #[test]
    fn test_add_history() {
        assert!(add_history("test").is_ok());
        assert!(add_history_time().is_ok());
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

    #[test]
    fn test_current_history() {
        println!("{:?}", current_history());
        assert!(add_history("ls -al").is_ok());
        println!("{:?}", current_history());
        println!("{:?}", next_history());
    }
}
