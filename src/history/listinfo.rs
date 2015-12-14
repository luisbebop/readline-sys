//! [2.3.3 Information About the History List](https://goo.gl/8OWMTy)
use history::{HistoryEntry, vars};
use time::Timespec;

mod ext_listinfo {
    use libc::c_long;
    use history::HistoryEntry;

    extern "C" {
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
pub fn history_list() -> Result<Vec<HistoryEntry>, ::HistoryError> {
    ::history::mgmt::init();
    unsafe {
        let ptrptr = &mut *ext_listinfo::history_list();

        if ptrptr.is_null() {
            Err(::HistoryError::new("Null Pointer", "Unable to access history list"))
        } else {
            // TODO: Loop through the list and build a vector.
            let len = vars::history_length;
            for i in 0..len {
                let ptr = *ptrptr.offset(i as isize);
                println!("{:?}", ptr);
            }
            Ok(Vec::new())
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_history_list() {
        ::history::mgmt::init();
        assert!(::history::listmgmt::add_history("ls -al").is_ok());
        assert!(history_list().is_ok());
    }
}
