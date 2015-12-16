//! [2.3.5 Searching the History List](https://goo.gl/Xltw2k)
//!
//! These functions allow searching of the history list for entries containing a specific string.
//! Searching may be performed both forward and backward from the current history position. The
//! search may be *anchored*, meaning that the string must match at the beginning of the history
//! entry.
use libc::c_int;
use std::ffi::CString;
use self::Direction::*;

/// The direction to search through the history entries.
pub enum Direction {
    Forward,
    Backward,
}

mod ext_search {
    use libc::{c_char, c_int};
    extern "C" {
        pub fn history_search(arg1: *const c_char, arg2: c_int) -> c_int;
        pub fn history_search_prefix(arg1: *const c_char, arg2: c_int) -> c_int;
        pub fn history_search_pos(arg1: *const c_char, arg2: c_int, arg3: c_int) -> c_int;
    }
}

/// Search the history for string, starting at the current history offset. If direction is
/// `Backward`, then the search is through previous entries, otherwise through subsequent entries.
/// If string is found, then the current history index is set to that history entry, and the value
/// returned is the offset in the line of the entry where string was found. Otherwise, nothing is
/// changed, and a -1 is returned.
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
/// use rl_sys::history::search::{self, Direction};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let res = search::search("-al", Direction::Forward).unwrap();
/// assert!(res == 3);
/// let res1 = search::search("blah", Direction::Forward).unwrap();
/// assert!(res1 < 0);
/// ```
pub fn search(s: &str, dir: Direction) -> Result<isize, ::HistoryError> {
    let d = match dir {
        Forward => 0,
        Backward => -1,
    };
    let cline = try!(CString::new(s));
    ::history::mgmt::init();
    unsafe { Ok(ext_search::history_search(cline.as_ptr(), d as c_int) as isize) }
}

/// Search the history for string, starting at the current history offset. The search is anchored:
/// matching lines must begin with string. If direction is `Backward`, then the search is through
/// previous entries, otherwise through subsequent entries. If string is found, then the current
/// history index is set to that entry, and the return value is 0. Otherwise, nothing is changed,
/// and a -1 is returned.
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
/// use rl_sys::history::search::{self, Direction};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let res = search::search_prefix("ls", Direction::Forward).unwrap();
/// assert!(res == 0);
/// let res1 = search::search_prefix("blah", Direction::Forward).unwrap();
/// assert!(res1 < 0);
/// ```
pub fn search_prefix(s: &str, dir: Direction) -> Result<isize, ::HistoryError> {
    let d = match dir {
        Forward => 0,
        Backward => -1,
    };
    let cline = try!(CString::new(s));
    ::history::mgmt::init();
    unsafe { Ok(ext_search::history_search_prefix(cline.as_ptr(), d as c_int) as isize) }
}

/// Search for string in the history list, starting at pos, an absolute index into the list. If
/// direction `Backward`, the search proceeds backward from pos, otherwise forward. Returns the
/// absolute index of the history element where string was found, or -1 otherwise.
///
/// # Examples
///
/// ```
/// use rl_sys::history::listmgmt;
/// use rl_sys::history::search::{self, Direction};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// let res = search::search_pos("ls", Direction::Backward, 1).unwrap();
/// assert!(res == 0);
/// ```
pub fn search_pos(s: &str, dir: Direction, pos: i32) -> Result<isize, ::HistoryError> {
    let d: i32 = match dir {
        Forward => 0,
        Backward => -1,
    };
    let cline = try!(CString::new(s));
    ::history::mgmt::init();
    unsafe { Ok(ext_search::history_search_pos(cline.as_ptr(), d, pos) as isize) }
}
