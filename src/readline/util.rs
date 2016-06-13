//! [2.4.10 Utility Functions]
//! [2.4.10 utility functions]: https://goo.gl/wg27lL
use libc::c_void;
use readline::ReadlineState;
use std::ffi::CString;
use std::sync::{ONCE_INIT, Once};

mod ext_util {
    use libc::{c_char, c_int, c_void};
    use readline::ReadlineState;

    extern "C" {
        pub fn rl_save_state(state: *mut ReadlineState) -> c_int;
        pub fn rl_restore_state(state: *mut ReadlineState) -> c_int;
        pub fn rl_free(mem: *mut c_void) -> ();
        pub fn rl_replace_line(text: *const c_char, clear_undo: c_int) -> ();
        pub fn rl_extend_line_buffer(len: c_int) -> ();
        pub fn rl_initialize() -> ();
        pub fn rl_ding() -> ();
        pub fn rl_alphabetic(char: c_int) -> c_int;
        pub fn rl_display_match_list(matches: *mut *mut c_char, len: c_int, max: c_int) -> ();
    }
}

static START: Once = ONCE_INIT;

/// Save a snapshot of Readline's internal state to `state`. The contents of the `readline_state`
/// structure are documented in 'readline.h'. The caller is responsible for allocating the
/// structure.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{ReadlineState, util};
/// use std::default::Default;
///
/// util::init();
///
/// let mut state: ReadlineState = Default::default();
/// assert!(util::save_state(&mut state).is_ok());
/// ```
pub fn save_state(state: &mut ReadlineState) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_util::rl_save_state(state);

        if res == 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Util Error", "Unable to save the readline state!"))
        }
    }
}

/// Restore Readline's internal state to that stored in `state`, which must have been saved by a
/// call to `save_state`. The contents of the `readline_state` structure are documented in
/// 'readline.h'. The caller is responsible for freeing the structure.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{ReadlineState, redisplay, util};
/// use std::default::Default;
///
/// util::init();
///
/// let mut state: ReadlineState = Default::default();
/// assert!(util::save_state(&mut state).is_ok());
/// assert!(state.prompt.is_null());
/// assert!(redisplay::set_prompt("promptly: ").is_ok());
/// let mut state2: ReadlineState = Default::default();
/// assert!(util::save_state(&mut state2).is_ok());
/// assert!(!state2.prompt.is_null());
/// assert!(util::restore_state(&mut state).is_ok());
/// assert!(state.prompt.is_null());
/// ```
pub fn restore_state(state: &mut ReadlineState) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_util::rl_restore_state(state);

        if res == 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Util Error", "Unable to save the readline state!"))
        }
    }
}


/// Deallocate the memory pointed to by `ptr`. `ptr` must have been allocated by malloc.
///
/// # Examples
///
/// See the Readline History API [cleanup] source.
/// [cleanup]: ../../../src/rl_sys/history/mgmt.rs.html#82-98
pub fn free(ptr: *mut c_void) {
    unsafe {
        ext_util::rl_free(ptr);
    }
}

/// Replace the contents of `rl_line_buffer` with text. The point and mark are preserved, if
/// possible. If `clear_undo` is true, the undo list associated with the current line is cleared.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::util;
///
/// util::init();
///
/// assert!(util::replace_line("replacement", false).is_ok());
/// ```
pub fn replace_line(text: &str, clear_undo: bool) -> Result<(), ::ReadlineError> {
    let ptr = try!(CString::new(text)).as_ptr();
    let clear = if clear_undo { 1 } else { 0 };

    unsafe { Ok(ext_util::rl_replace_line(ptr, clear)) }
}

/// Ensure that `rl_line_buffer` has enough space to hold len characters, possibly reallocating it
/// if necessary.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::util;
///
/// util::init();
///
/// util::extend_line_buffer(512);
/// ```
pub fn extend_line_buffer(len: i32) -> () {
    unsafe { ext_util::rl_extend_line_buffer(len) }
}

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
        unsafe {
            ext_util::rl_initialize();
        }
    });
}

/// Ring the terminal bell, obeying the setting of `bell-style`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::util;
///
/// util::ding();
/// ```
pub fn ding() -> () {
    unsafe { ext_util::rl_ding() }
}

/// Return 1 if `c` is an alphabetic character.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::util;
///
/// assert!(util::alphabetic('a'));
/// assert!(!util::alphabetic('?'));
/// ```
pub fn alphabetic(c: char) -> bool {
    unsafe { ext_util::rl_alphabetic(c as i32) == 1 }
}

/// A convenience function for displaying a list of strings in columnar format on Readline's output
/// stream. `matches` is the list of strings, in argv format, such as a list of completion matches.
/// `len` is the number of strings in `matches`, and `max` is the length of the longest string in
/// `matches`. This function uses the setting of `print-completions-horizontally` to select how the
/// matches are displayed (see section [1.3.1 Readline Init File Syntax]). When displaying
/// completions, this function sets the number of columns used for display to the value of
/// `completion-display-width`, the value of the environment variable COLUMNS, or the screen width,
/// in that order.
/// [1.3.1 readline init file syntax]: https://goo.gl/Ivqovs
///
/// # Examples
///
/// ```
/// use rl_sys::readline::util;
///
/// util::init();
///
/// let matches = vec!["alpha", "albert", "algorithm", "blah"];
/// assert!(util::display_match_list(matches).is_ok());
/// ```
#[cfg_attr(feature = "clippy", allow(cast_possible_truncation, cast_possible_wrap))]
pub fn display_match_list(matches: Vec<&str>) -> Result<(), ::ReadlineError> {
    let len = matches.len() - 1;
    let mut max = 0;
    let mut char_ptrs = Vec::new();

    for s in &matches {
        let l = s.len();
        if l > max {
            max = l;
        }
        char_ptrs.push(try!(CString::new(*s)).into_raw());
    }

    unsafe { Ok(ext_util::rl_display_match_list(char_ptrs.as_mut_ptr(), len as i32, max as i32)) }
}

#[cfg(test)]
mod test {
    use readline::{ReadlineState, redisplay};
    use std::default::Default;
    use super::*;

    #[test]
    fn test_save_state() {
        init();

        let mut state: ReadlineState = Default::default();
        assert!(redisplay::set_prompt("promptly: ").is_ok());
        assert!(save_state(&mut state).is_ok());
        assert!(!state.prompt.is_null());
    }

    // #[test]
    // fn test_display_match_list() {
    //     init();
    //
    //     let matches = vec!["prog", "alpha", "albert", "algorithm"];
    //     assert!(display_match_list(matches).is_ok());
    // }
}
