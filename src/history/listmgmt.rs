//! [2.3.2 History List Management](https://goo.gl/P6UC5s)
//!
//! These functions manage individual entries on the history list, or set parameters managing the
//! list itself.
use libc::{c_int, c_void};
use history::{HistoryEntry, vars};
use history::mgmt::init;
use std::ffi::CString;
use std::ptr;
use time::Timespec;

mod ext_listmgmt {
    use libc::{c_char, c_int, c_void};
    use history::HistoryEntry;

    extern "C" {
        pub fn add_history(line: *const c_char) -> ();
        pub fn add_history_time(time: *const c_char) -> ();
        pub fn remove_history(which: c_int) -> *mut HistoryEntry;
        pub fn free_history_entry(entry: *mut HistoryEntry) -> *mut c_void;
        pub fn replace_history_entry(which: c_int,
                                     line: *const c_char,
                                     data: *mut c_void)
                                     -> *mut HistoryEntry;
        pub fn clear_history() -> ();
        pub fn stifle_history(arg1: c_int) -> ();
        pub fn unstifle_history() -> c_int;
        pub fn history_is_stifled() -> c_int;
    }
}

/// Place string at the end of the history list. The associated data field (if any) is set to NULL.
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// match listmgmt::add("ls -al") {
///     Ok(_)  => assert!(true),
///     Err(e) => assert!(false),
/// }
/// ```
pub fn add(line: &str) -> Result<(), ::HistoryError> {
    init();
    let csline = try!(CString::new(line));

    unsafe {
        ext_listmgmt::add_history(csline.as_ptr());
    }

    Ok(())
}

/// Change the time stamp associated with the most recent history entry to the given time.  Note
/// that if the `history_comment_char` variable has not been set this will have no effect.  This is
/// stored as seconds since the epoch, so you may lose some precision.
///
/// # Examples
///
/// ```
/// # extern crate rl_sys;
/// # extern crate time;
/// # fn main() {
/// use rl_sys::history::{listmgmt, vars};
///
/// vars::set_comment_char(':');
/// match listmgmt::add_time(time::now().to_timespec()) {
///     Ok(_)  => assert!(true),
///     Err(e) => assert!(false),
/// }
/// # }
/// ```
pub fn add_time(time: Timespec) -> Result<(), ::HistoryError> {
    init();
    let cc = vars::get_comment_char();

    if cc == '\u{0}' {
        Ok(())
    } else {
        let now_str = format!("{}{}", cc, time.sec);
        let cstime = try!(CString::new(now_str));
        unsafe {
            ext_listmgmt::add_history_time(cstime.as_ptr());
        }
        Ok(())
    }
}

/// Remove history entry at the given offset from the history. The removed element is returned so
/// you can free the line, data, and containing structure.
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let _ = listmgmt::remove(0);
/// ```
pub fn remove<'a>(offset: i32) -> &'a mut HistoryEntry {
    init();
    unsafe { &mut *ext_listmgmt::remove_history(offset) }
}

/// Free the history entry and any history library private data associated with it. If there is
/// application-specific data, an Err is returned with a pointer to the data so the caller can
/// dispose of it
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let entry = listmgmt::remove(0);
/// assert!(listmgmt::free_entry(entry).is_ok());
/// ```
pub fn free_entry(entry: &mut HistoryEntry) -> Result<(), *mut c_void> {
    init();
    unsafe {
        let data_ptr = ext_listmgmt::free_history_entry(entry);

        if data_ptr.is_null() {
            Ok(())
        } else {
            Err(data_ptr)
        }
    }
}

/// Replace the history entry at offset with the given line and data. This returns the old entry so
/// the caller can dispose of any application-specific data. In the case of an invalid offset, an
/// Err is returned.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, vars};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// assert_eq!(vars::history_length, 1);
/// assert!(listmgmt::replace_entry(0, "test", None).is_ok());
/// assert_eq!(vars::history_length, 1);
/// ```
pub fn replace_entry(offset: i32,
                     line: &str,
                     appdata: Option<*mut c_void>)
                     -> Result<&mut HistoryEntry, ::HistoryError> {
    init();
    let csline = try!(CString::new(line));
    let ptr = match appdata {
        Some(d) => d,
        None => ptr::null_mut(),
    };

    unsafe {
        let old_entry = ext_listmgmt::replace_history_entry(offset, csline.as_ptr(), ptr);

        if old_entry.is_null() {
            Err(::HistoryError::new("Null Pointer", "Invalid replace requested!"))
        } else {
            Ok(&mut *old_entry)
        }
    }
}

/// Clear the history list by deleting all the entries.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, vars};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// assert_eq!(vars::history_length, 1);
/// listmgmt::clear();
/// assert_eq!(vars::history_length, 0);
/// ```
pub fn clear() {
    init();
    unsafe { ext_listmgmt::clear_history() }
}

/// Stifle the history list, remembering only the last *max* entries.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, vars};
///
/// listmgmt::stifle(5);
/// assert_eq!(vars::history_max_entries, 5);
/// ```
pub fn stifle(max: i32) {
    init();
    unsafe { ext_listmgmt::stifle_history(max as c_int) }
}

/// Stop stifling the history. This returns the previously-set maximum number of history entries
/// (as set by `stifle_history()`).
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// let max = 5;
/// listmgmt::stifle(max);
/// assert_eq!(max, listmgmt::unstifle());
/// ```
pub fn unstifle() -> i32 {
    init();
    unsafe { ext_listmgmt::unstifle_history() }
}

/// Returns true if the history is stifled, false if it is not.
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// assert!(!listmgmt::is_stifled());
/// listmgmt::stifle(1);
/// assert!(listmgmt::is_stifled());
/// ```
pub fn is_stifled() -> bool {
    init();
    unsafe { ext_listmgmt::history_is_stifled() != 0 }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stifle() {
        // History should not begin stifled.
        assert!(!is_stifled());

        let max = 5;
        stifle(max);
        assert!(is_stifled());

        assert_eq!(max, unstifle());
        assert!(!is_stifled());
    }
}
