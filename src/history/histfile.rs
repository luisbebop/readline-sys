//! [2.3.6 Managing the History File](https://goo.gl/ovVU4a)
//!
//! The History library can read the history from and write it to a file. This section documents the
//! functions for managing a history file.
use errno::errno;
use history::mgmt::init;
use std::ffi::CString;
use std::path::Path;
use std::ptr;

mod ext_histfile {
    use libc::{c_char, c_int};

    extern "C" {
        pub fn read_history(file: *const c_char) -> c_int;
        pub fn read_history_range(file: *const c_char, from: c_int, to: c_int) -> c_int;
        pub fn write_history(file: *const c_char) -> c_int;
        pub fn append_history(n: c_int, file: *const c_char) -> c_int;
        pub fn history_truncate_file(file: *const c_char, n: c_int) -> c_int;
    }
}

fn with_path_ptr<F>(path: Option<&Path>, f: F) -> Result<i32, ::HistoryError>
    where F: Fn(*const i8) -> Result<i32, ::HistoryError>
{
    if let Some(p) = path {
        match p.to_str() {
            Some(p) => {
                if let Ok(cs) = CString::new(p) {
                    return f(cs.as_ptr())
                }
            }
            None => return Err(::HistoryError::new("History Error", "Unable to determine path!")),
        }
    }
    f(ptr::null())
}

fn gen_result(res: i32) -> Result<i32, ::HistoryError> {
    if res == 0 {
        Ok(res)
    } else {
        let e = errno();
        let code = e.0 as i32;
        let out = format!("Error {}: {}", code, e);
        Err(::HistoryError::new("History Error", &out[..]))
    }
}

/// Add the contents of filename at path to the history list, a line at a time. If the path is
/// `None`, then read from `~/.history`. Returns 0 if successful, or `HistoryError` with errno
/// information if not.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, histfile};
/// use std::env;
/// use std::fs;
///
/// let home = env::home_dir().unwrap();
/// let path = home.join(".test-history");
/// assert!(listmgmt::add("ls -al").is_ok());
/// let mut res = histfile::write(Some(path.as_path())).unwrap();
/// assert!(res == 0);
/// res = histfile::read(Some(path.as_path())).unwrap();
/// assert!(res == 0);
/// assert!(fs::remove_file(path).is_ok());
pub fn read(path: Option<&Path>) -> Result<i32, ::HistoryError> {
    init();
    with_path_ptr(path, |ptr| {
        unsafe { gen_result(ext_histfile::read_history(ptr)) }
    })
}

/// Read a range of lines from filename, adding them to the history list. Start reading at line
/// `from` and end at `to`. If `from` is zero, start at the beginning. If `to` is less than `from`,
/// then read until the end of the file. If filename is None, then read from `~/.history`. Returns 0
/// if successful, or `HistoryError` with errno information if not.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, histfile};
/// use std::env;
/// use std::fs;
///
/// let home = env::home_dir().unwrap();
/// let path = home.join(".test-history");
/// assert!(listmgmt::add("ls -al").is_ok());
/// let mut res = histfile::write(Some(path.as_path())).unwrap();
/// assert!(res == 0);
/// res = histfile::read_range(Some(path.as_path()), 0, -1).unwrap();
/// assert!(res == 0);
/// assert!(fs::remove_file(path).is_ok());
pub fn read_range(path: Option<&Path>, from: i32, to: i32) -> Result<i32, ::HistoryError> {
    init();
    with_path_ptr(path, |ptr| {
        unsafe { gen_result(ext_histfile::read_history_range(ptr, from, to)) }
    })
}

/// Write the current history to `filename`, overwriting `filename` if necessary. If `filename` is
/// None, then write the history list to `~/.history`. Returns 0 on success, or `HistoryError` with
/// `errno` information on a read or write error.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, histfile};
/// use std::env;
/// use std::fs;
///
/// let home = env::home_dir().unwrap();
/// let path = home.join(".test-history");
/// assert!(listmgmt::add("ls -al").is_ok());
/// let res = histfile::write(Some(path.as_path())).unwrap();
/// assert!(res == 0);
/// assert!(fs::remove_file(path).is_ok());
/// ```
pub fn write(path: Option<&Path>) -> Result<i32, ::HistoryError> {
    init();
    with_path_ptr(path, |ptr| {
        unsafe { gen_result(ext_histfile::write_history(ptr)) }
    })
}

/// Append the last `n` elements of the history list to `filename`. If `filename` is None, then
/// append to `~/.history`. Returns 0 on success, or `HistoryError` with `errno` information on a
/// write error.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, histfile};
/// use std::env;
/// use std::fs::{self, File};
///
/// let home = env::home_dir().unwrap();
/// let path = home.join(".test-history");
/// let _ = File::create(path.as_path());
/// assert!(listmgmt::add("ls -al").is_ok());
/// let res = histfile::append(Some(path.as_path()), 1).unwrap();
/// assert!(res == 0);
/// assert!(fs::remove_file(path).is_ok());
/// ```
pub fn append(path: Option<&Path>, n: i32) -> Result<i32, ::HistoryError> {
    init();
    with_path_ptr(path, |ptr| {
        unsafe { gen_result(ext_histfile::append_history(n, ptr)) }
    })
}

/// Truncate the history file `filename`, leaving only the last `n` lines. If `filename` is None,
/// then `~/.history` is truncated. Returns 0 on success, or `HistoryError` with `errno` information
/// on a write error.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, histfile};
/// use std::env;
/// use std::fs::{self, File};
///
/// let home = env::home_dir().unwrap();
/// let path = home.join(".test-history");
/// let _ = File::create(path.as_path());
/// assert!(listmgmt::add("ls -al").is_ok());
/// assert!(listmgmt::add("test").is_ok());
/// let mut res = histfile::write(Some(path.as_path())).unwrap();
/// assert!(res == 0);
/// res = histfile::truncate(Some(path.as_path()), 1).unwrap();
/// assert!(res == 0);
/// assert!(fs::remove_file(path).is_ok());
/// ```
pub fn truncate(path: Option<&Path>, n: i32) -> Result<i32, ::HistoryError> {
    init();
    with_path_ptr(path, |ptr| {
        unsafe { gen_result(ext_histfile::history_truncate_file(ptr, n)) }
    })
}
