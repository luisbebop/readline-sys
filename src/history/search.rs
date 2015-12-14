//! [2.3.5 Searching the History List](https://goo.gl/Xltw2k)
//!
//! These functions allow searching of the history list for entries containing a specific string.
//! Searching may be performed both forward and backward from the current history position. The
//! search may be *anchored*, meaning that the string must match at the beginning of the history
//! entry.

mod ext_search {
    extern "C" {
    // pub fn history_search(arg1: *const c_char, arg2: c_int) -> c_int;
    // pub fn history_search_prefix(arg1: *const c_char,
    //                              arg2: c_int) -> c_int;
    // pub fn history_search_pos(arg1: *const c_char,
    //                           arg2: c_int, arg3: c_int) -> c_int;
    }
}
