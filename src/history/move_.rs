//! [2.3.4 Moving Around the History List](https://goo.gl/ROYfRB)
//!
//! These functions allow the current index into the history list to be set or changed.
use history::HistoryEntry;

mod ext_move {
    use history::HistoryEntry;
    extern "C" {
        // pub fn history_set_pos(arg1: c_int) -> c_int;
        pub fn previous_history() -> *mut HistoryEntry;
        pub fn next_history() -> *mut HistoryEntry;
    }
}

pub fn previous_history<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptr = ext_move::previous_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the previous history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}

pub fn next_history<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptr = ext_move::next_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the next history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}
