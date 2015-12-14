//! Readline History API
use libc::{c_char, c_int, c_void};
use std::clone::Clone;
use std::default::Default;
use std::ffi::CStr;
use std::fmt;
// use std::fs::{self, File, OpenOptions};
// use std::io::{BufRead, BufReader, LineWriter, Write};
// use std::path::Path;

pub mod listinfo;
pub mod listmgmt;
pub mod mgmt;
pub mod vars;

mod ext_history {
    use super::HistoryEntry;

    extern "C" {
        // pub fn history_set_pos(arg1: c_int) -> c_int;
        pub fn previous_history() -> *mut HistoryEntry;
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
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for HistoryEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl fmt::Debug for HistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let line = unsafe { CStr::from_ptr(self.line).to_string_lossy().into_owned() };
        let time = unsafe { CStr::from_ptr(self.timestamp).to_string_lossy().into_owned() };
        write!(f,
               "HistoryEntry {{ line: {:?}, timestamp: {:?}, data: {:?} }}",
               line,
               time,
               self.data)
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
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for HistoryState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl fmt::Debug for HistoryState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "HistoryState {{ entries: {:?}, offset: {:?}, length: {:?}, size: {:?}, flags: {:?}",
               self.entries,
               self.offset,
               self.length,
               self.size,
               self.flags)
    }
}

pub fn previous_history<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    mgmt::init();
    unsafe {
        let ptr = ext_history::previous_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the previous history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}

pub fn next_history<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    mgmt::init();
    unsafe {
        let ptr = ext_history::next_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the next history!"))
        } else {
            Ok(&mut *ptr)
        }
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
// pub fn preload_history(file: &Path) -> Result<(), HistoryError> {
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
//                     return Err(HistoryError::new("HistoryError", e))
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
// ) -> Result<(), HistoryError> {
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
//                 return Err(HistoryError::new("HistoryError", e))
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
    // use std::ops::Add;
    // use super::*;
    // use time::{self, Duration};

    #[test]
    fn test_add_history() {
        // vars::set_comment_char(':');
        // assert!(listmgmt::add_history("test").is_ok());
        // println!("1: {:?}", listinfo::current_history());
        // println!("LENGTH: {:?}", vars::history_length);
        // let fut = time::now().add(Duration::seconds(60));
        // assert!(listmgmt::add_history_time(fut.to_timespec()).is_ok());
        //
        // println!("STATE 1: {:?}", mgmt::history_get_history_state());
        // println!("2: {:?}", listinfo::current_history());
        // assert!(listmgmt::add_history("ls -al").is_ok());
        // println!("STATE 2: {:?}", mgmt::history_get_history_state());
        // println!("LIST: {:?}", listinfo::history_list().unwrap());
        // let mut current = listinfo::current_history().unwrap();
        // println!("3: {:?}", current);
        // println!("TIME: {:?}", listinfo::history_get_time(&mut current));
        // println!("NEXT: {:?}", next_history());
        // println!("BASE: {:?}", vars::history_base);
        // let mut remove = listmgmt::remove_history(1);
        // println!("REMOVE: {:?}", remove);
        // println!("FREE 1: {:?}", listmgmt::free_history_entry(&mut remove));
        // println!("STATE 3: {:?}", mgmt::history_get_history_state());
        // println!("REPLACE: {:?}",
        //          listmgmt::replace_history_entry(0, String::from("blah")));
        // println!("PREVIOUS: {:?}", previous_history());
        // println!("4: {:?}", listinfo::current_history());
        // println!("FREE: {:?}",
        //          listmgmt::free_history_entry(&mut listinfo::current_history().unwrap()));
    }
}
