//! [2.4.11 Miscellaneous Functions]
//! [2.4.11 miscellaneous functions]: https://goo.gl/2BieXb

mod ext_misc {
    extern "C" {
        pub fn rl_clear_history() -> ();
    }
}

/// Clear the history list by deleting all of the entries, in the same manner as the History
/// library's `clear_history()` function. This differs from `clear_history` because it frees
/// private data Readline saves in the history list.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::misc;
///
/// misc::clear_history();
/// ```
pub fn clear_history() -> () {
    unsafe { ext_misc::rl_clear_history() }
}
