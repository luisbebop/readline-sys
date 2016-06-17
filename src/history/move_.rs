// Copyright 2015-2016 Jason Ozias
//
// This file is part of rl-sys.
//
// rl-sys is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rl-sys is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rl-sys.  If not, see <http://www.gnu.org/licenses/>.

//! [2.3.4 Moving Around the History List](https://goo.gl/ROYfRB)
//!
//! These functions allow the current index into the history list to be set or changed.
use history::HistoryEntry;
use history::mgmt::init;

mod ext_move {
    use libc::c_int;
    use history::HistoryEntry;
    extern "C" {
        pub fn history_set_pos(which: c_int) -> c_int;
        pub fn previous_history() -> *mut HistoryEntry;
        pub fn next_history() -> *mut HistoryEntry;
    }
}

/// Set the current history offset, an absolute index into the list. Returns true on success, false
/// if the offset is less than zero or greater than the number of history entries.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, move_};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// assert!(listmgmt::add("test").is_ok());
/// assert!(move_::set_pos(2));
/// assert!(!move_::set_pos(3));
/// ```
pub fn set_pos(offset: i32) -> bool {
    init();
    unsafe { ext_move::history_set_pos(offset) == 1 }
}

/// Back up the current history offset to the previous history entry, and return a pointer to that
/// entry. If there is no previous entry, return a `HistoryError`.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, move_};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// assert!(listmgmt::add("test").is_ok());
/// assert!(move_::set_pos(2));  // Set pos after last entry, and back up through both.
/// assert!(move_::previous().is_ok());
/// assert!(move_::previous().is_ok());
/// assert!(move_::previous().is_err());
/// ```
pub fn previous<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    init();
    unsafe {
        let ptr = ext_move::previous_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the previous history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}

/// Move the current history offset forward to the next history entry, and return a pointer to that
/// entry. If there is no next entry, return a `HistoryError`.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, move_};
///
/// assert!(listmgmt::add("ls -al").is_ok());  // Pos 0
/// assert!(listmgmt::add("test").is_ok());
/// assert!(move_::set_pos(0));
/// assert!(move_::next().is_ok());  // Move to pos 1.
/// assert!(move_::next().is_err()); // There is no pos 2.
/// ```
pub fn next<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    init();
    unsafe {
        let ptr = ext_move::next_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the next history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}
