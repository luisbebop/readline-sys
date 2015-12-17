//! [2.4.10 Utility Functions][1]
//!
//! [1]: https://cnswww.cns.cwru.edu/php/chet/readline/readline.html#SEC39
use libc::c_void;

mod ext_util {
    use libc::c_void;

    extern "C" {
        pub fn rl_free(mem: *mut c_void) -> ();
        pub fn rl_clear_history() -> ();
    }
}

/// Deallocate the memory pointed to by `ptr`. `ptr` must have been allocated by malloc.
pub fn free(ptr: *mut c_void) {
    unsafe { ext_util::rl_free(ptr); }
}

/// Clear the history list by deleting all of the entries, in the same manner as the History
/// library's `clear_history()`` function. This differs from `clear_history` because it frees
/// private data Readline saves in the history list.
pub fn clear_history() {
    unsafe { ext_util::rl_clear_history(); }
}
