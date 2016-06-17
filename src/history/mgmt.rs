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

//! [2.3.1 Initializing History and State Management](https://goo.gl/An1iSg)
//!
//! This section describes functions used to initialize and manage the state of the History library
//! when you want to use the history functions in your program.
use history::HistoryState;
use std::sync::{ONCE_INIT, Once};

mod ext_mgmt {
    use history::HistoryState;
    extern "C" {
        pub fn using_history() -> ();
        pub fn history_get_history_state() -> *mut HistoryState;
        pub fn history_set_history_state(state: *mut HistoryState) -> ();
    }
}

static START: Once = ONCE_INIT;

/// Begin a session in which the history functions might be used. This initializes the interactive
/// variables.  Note that this will only call `using_history` once after first use.
///
/// # Examples
///
/// ```
/// use rl_sys::history::mgmt;
///
/// mgmt::init();
/// ```
pub fn init() {
    START.call_once(|| {
        debug!("History API initialized");
        unsafe { ext_mgmt::using_history() }
    });
}

/// Get pointer for cleanup
fn get_state_ptr() -> *mut HistoryState {
    init();
    unsafe { ext_mgmt::history_get_history_state() }
}

/// Return a structure describing the current state of the input history.
///
/// # Examples
/// ```
/// use rl_sys::history::mgmt;
///
/// mgmt::init();
///
/// let state = mgmt::get_state();
/// assert!(state.offset == 0);
/// ```
pub fn get_state<'a>() -> &'a mut HistoryState {
    init();
    unsafe { &mut *get_state_ptr() }
}

/// Set the state of the history list according to state.
///
/// # Examples
/// ```
/// use rl_sys::history::mgmt;
///
/// mgmt::init();
///
/// let mut state = mgmt::get_state();
/// assert!(state.offset == 0);
/// mgmt::set_state(&mut state);
/// ```
pub fn set_state(state: &mut HistoryState) -> () {
    init();
    unsafe { ext_mgmt::history_set_history_state(state) }
}

/// Cleanup the history state.  This should be called on program exit or after you are completely
/// finished using the History API.
///
/// # Examples
///
/// ```
/// use rl_sys::history::{listmgmt, mgmt};
///
/// assert!(listmgmt::add("ls -al").is_ok());
/// mgmt::cleanup();
/// ```
pub fn cleanup() -> () {
    use libc::c_void;
    use readline::{misc, util};

    // Clear the history via Readline API.  This frees all Histoy Entry data, but not the list
    // itself.
    misc::clear_history();

    // Get a pointer to the History State.
    let state_ptr = get_state_ptr();

    // Free the History Entries array.
    util::free(unsafe { (&*state_ptr).entries } as *mut c_void);

    // Free the History State.
    util::free(state_ptr as *mut c_void);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_state() {
        use std::mem;
        use super::get_state;

        let state = get_state();
        assert!(mem::size_of_val(state) > 0);
    }
}
