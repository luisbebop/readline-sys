//! [2.3.3 Information About the History List](https://cnswww.cns.cwru.edu/php/chet/readline/history.html#SEC12)
use history::HistoryEntry;
use time::Timespec;

mod ext_listinfo {
    use libc::c_long;
    use history::HistoryEntry;

    extern {
        pub fn history_list() -> *mut *mut HistoryEntry;
        // pub fn where_history() -> c_int;
        pub fn current_history() -> *mut HistoryEntry;
        // pub fn history_get(arg1: c_int) -> *mut HistoryEntry;
        pub fn history_get_time(arg1: *mut HistoryEntry) -> c_long;
        // pub fn history_total_bytes() -> c_int;
    }
}

/// Return a NULL terminated array of HIST_ENTRY * which is the current input history. Element 0 of
/// this list is the beginning of time. If there is no history, return NULL.
///
/// # Examples
///
/// ```
///
/// ```
pub fn history_list() -> Result<(), ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptrptr = &mut *ext_listinfo::history_list();

        // TODO: Loop through the list and build a vector.
        if ptrptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to access history list"))
        } else {
            Ok(())
        }
    }
}

/// Return the history entry at the current position, as determined by `where_history()``. If there
/// is no entry there, return a HistoryError.
///
/// # Examples
///
/// ```
///
/// ```
pub fn current_history<'a>() -> Result<&'a mut HistoryEntry, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptr = ext_listinfo::current_history();

        if ptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to read the current history!"))
        } else {
            Ok(&mut *ptr)
        }
    }
}

/// Return the time stamp associated with the history entry entry.
///
/// # Examples
///
/// ```
///
/// ```
pub fn history_get_time<'a>(entry: &'a mut HistoryEntry) -> Timespec {
    ::history::mgmt::init();
    Timespec::new(unsafe { ext_listinfo::history_get_time(entry) } as i64, 0)
}
