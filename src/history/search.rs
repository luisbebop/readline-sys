//! [2.3.5 Searching the History List](https://goo.gl/Xltw2k)
//!
//! These functions allow searching of the history list for entries containing a specific string.
//! Searching may be performed both forward and backward from the current history position. The
//! search may be *anchored*, meaning that the string must match at the beginning of the history
//! entry.
use history::mgmt::init;
use std::ffi::CString;
use self::Direction::*;

/// The direction to search through the history entries.
pub enum Direction {
    /// Search forward through history entries.
    Forward,
    /// Search backward through history entries.
    Backward,
}

impl Into<i32> for Direction {
    fn into(self) -> i32 {
        match self {
            Forward => 0,
            Backward => -1,
        }
    }
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
pub fn search<T>(s: &str, dir: T) -> Result<isize, ::HistoryError>
    where T: Into<i32>
{
    init();
    let ptr = try!(CString::new(s)).as_ptr();
    unsafe { Ok(ext_search::history_search(ptr, dir.into()) as isize) }
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
pub fn search_prefix<T>(s: &str, dir: T) -> Result<isize, ::HistoryError>
    where T: Into<i32>
{
    init();
    let ptr = try!(CString::new(s)).as_ptr();
    unsafe { Ok(ext_search::history_search_prefix(ptr, dir.into()) as isize) }
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
pub fn search_pos<T>(s: &str, dir: T, pos: i32) -> Result<isize, ::HistoryError>
    where T: Into<i32>
{
    init();
    let ptr = try!(CString::new(s)).as_ptr();
    unsafe { Ok(ext_search::history_search_pos(ptr, dir.into(), pos) as isize) }
}
