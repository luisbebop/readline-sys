//! [2.4.10 Utility Functions](https://goo.gl/wg27lL)
use libc::c_void;
use std::sync::{ONCE_INIT, Once};

mod ext_util {
    use libc::c_void;

    extern "C" {
        pub fn rl_free(mem: *mut c_void) -> ();
        pub fn rl_clear_history() -> ();
        pub fn rl_initialize() -> ();
    }
}

static START: Once = ONCE_INIT;

/// Initialize or re-initialize Readline's internal state. It's not strictly necessary to call this;
/// `readline()` calls it before reading any input. Note that this will only call `initialize` once
/// after first use.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::util;
///
/// util::init();
/// ```
pub fn init() {
    START.call_once(|| {
        debug!("Readline API initialized");
        unsafe { ext_util::rl_initialize() };
    });
}

/// Deallocate the memory pointed to by `ptr`. `ptr` must have been allocated by malloc.
pub fn free(ptr: *mut c_void) {
    unsafe {
        ext_util::rl_free(ptr);
    }
}

/// Clear the history list by deleting all of the entries, in the same manner as the History
/// library's `clear_history()`` function. This differs from `clear_history` because it frees
/// private data Readline saves in the history list.
pub fn clear_history() {
    unsafe {
        ext_util::rl_clear_history();
    }
}
