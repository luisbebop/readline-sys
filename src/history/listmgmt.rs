//! [2.3.2 History List Management](https://goo.gl/P6UC5s)
use libc::{c_int, c_void};
use history::{HistoryEntry, vars};
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
/// match listmgmt::add_history("ls -al") {
///     Ok(_)  => println!("Success!"),
///     Err(e) => println!("{}", e),
/// }
/// ```
pub fn add_history(line: &str) -> Result<(), ::HistoryError> {
    let cline = try!(CString::new(line.as_bytes()));
    ::history::mgmt::init();
    unsafe {
        ext_listmgmt::add_history(cline.as_ptr());
    }
    Ok(())
}

/// Change the time stamp associated with the most recent history entry to the given time.
pub fn add_history_time(time: Timespec) -> Result<(), ::HistoryError> {
    let cc = vars::get_comment_char();
    let now_str = format!("{}{}", cc, time.sec);
    let cline = try!(CString::new(now_str.as_bytes()));
    ::history::mgmt::init();
    unsafe {
        ext_listmgmt::add_history_time(cline.as_ptr());
    }
    Ok(())
}

pub fn remove_history<'a>(idx: usize) -> &'a mut HistoryEntry {
    ::history::mgmt::init();
    unsafe { &mut *ext_listmgmt::remove_history(idx as i32) }
}

pub fn free_history_entry<'a>(entry: &'a mut HistoryEntry) -> Result<(), *mut c_void> {
    ::history::mgmt::init();
    unsafe {
        let data_ptr = ext_listmgmt::free_history_entry(entry);

        if data_ptr.is_null() {
            Ok(())
        } else {
            Err(data_ptr)
        }
    }
}

pub fn replace_history_entry<'a>(which: usize,
                                 line: String)
                                 -> Result<&'a mut HistoryEntry, ::HistoryError> {
    ::history::mgmt::init();
    let cline = try!(CString::new(line.as_bytes()));

    unsafe {
        let old_entry = ext_listmgmt::replace_history_entry(which as i32,
                                                            cline.as_ptr(),
                                                            ptr::null_mut());

        if old_entry.is_null() {
            Err(::HistoryError::new("Null Pointer", "Invalid replace requested!"))
        } else {
            Ok(&mut *old_entry)
        }
    }
}

/// Clear the history list by deleting all the entries.
pub fn clear_history() {
    ::history::mgmt::init();
    unsafe { ext_listmgmt::clear_history() }
}

/// Stifle the history list, remembering only the last *max* entries.
pub fn stifle_history(max: i32) {
    ::history::mgmt::init();
    unsafe { ext_listmgmt::stifle_history(max as c_int) }
}

/// Stop stifling the history.
///
/// This returns the previously-set maximum number of history entries (as set by stifle_history()).
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// let max = 5;
/// listmgmt::stifle_history(max);
/// assert_eq!(max, listmgmt::unstifle_history());
///
/// ```
pub fn unstifle_history() -> i32 {
    ::history::mgmt::init();
    unsafe { ext_listmgmt::unstifle_history() }
}

/// Is the history stifled?
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
///
/// assert!(!listmgmt::history_is_stifled());
/// listmgmt::stifle_history(1);
/// assert!(listmgmt::history_is_stifled());
/// ```
pub fn history_is_stifled() -> bool {
    ::history::mgmt::init();
    unsafe { ext_listmgmt::history_is_stifled() != 0 }
}

#[cfg(test)]
mod test {
    use super::*;

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
