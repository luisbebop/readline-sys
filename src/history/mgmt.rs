//! [2.3.1 Initializing History and State Management](https://cnswww.cns.cwru.edu/php/chet/readline/history.html#SEC10)
//!
//! This section describes functions used to initialize and manage the state of the History library
//! when you want to use the history functions in your program.
use history::HistoryState;
use std::sync::{ONCE_INIT, Once};

mod ext_mgmt {
    use history::HistoryState;
    extern {
        pub fn using_history() -> ();
        pub fn history_get_history_state() -> *mut HistoryState;
        pub fn history_set_history_state(state: *mut HistoryState) -> ();
    }
}

static START: Once = ONCE_INIT;

/// Begin a session in which the history functions might be used. This initializes the interactive
/// variables.  Note that this will only call `using_history` once after first use.
pub fn init() {
    START.call_once(|| {
        debug!("readline_sys initialized");
        unsafe { ext_mgmt::using_history() };
    });
}

/// Return a structure describing the current state of the input history.
///
/// # Examples
/// ```
/// use rl_sys::history::mgmt;
///
/// let state = mgmt::history_get_history_state();
/// println!("{:?}", state);
/// ```
pub fn history_get_history_state<'r>() -> &'r mut HistoryState {
    init();
    unsafe { &mut *ext_mgmt::history_get_history_state() }
}

/// Set the state of the history list according to state.
///
/// # Examples
/// ```
/// use rl_sys::history::mgmt;
///
/// let mut state = mgmt::history_get_history_state();
/// println!("{:?}", state);
/// mgmt::history_set_history_state(&mut state);
/// ```
pub fn history_set_history_state<'a>(state: &'a mut HistoryState) -> () {
    init();
    unsafe { ext_mgmt::history_set_history_state(state) }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_state() {
        use std::mem;
        use super::history_get_history_state;

        let state = history_get_history_state();
        assert!(mem::size_of_val(state) > 0);
    }
}
