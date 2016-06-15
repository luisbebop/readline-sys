//! [2.4.9 Terminal Management]
//! [2.4.9 terminal management]: https://goo.gl/NKP48u
use readline::Keymap;
use std::ffi::CString;
use std::ptr;

mod ext_termmgmt {
    use libc::{c_char, c_int};
    use readline::Keymap;

    extern "C" {
        pub fn rl_prep_terminal(meta_flag: c_int) -> ();
        pub fn rl_deprep_terminal() -> ();
        pub fn rl_tty_set_default_bindings(kmap: Keymap) -> ();
        pub fn rl_tty_unset_default_bindings(kmap: Keymap) -> ();
        pub fn rl_reset_terminal(name: *const c_char) -> c_int;
    }
}

/// Modify the terminal settings for Readline's use, so `readline()` can read a single character at
/// a time from the keyboard. The `meta_flag` argument should be non-zero if Readline should read
/// eight-bit input.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::termmgmt;
///
/// termmgmt::prep_terminal(1);
/// termmgmt::deprep_terminal();
/// ```
pub fn prep_terminal(meta_flag: i32) -> () {
    unsafe { ext_termmgmt::rl_prep_terminal(meta_flag) }
}

/// Undo the effects of `rl_prep_terminal()`, leaving the terminal in the state in which it was
/// before the most recent call to `rl_prep_terminal()`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::termmgmt;
///
/// termmgmt::prep_terminal(1);
/// termmgmt::deprep_terminal();
/// ```
pub fn deprep_terminal() -> () {
    unsafe { ext_termmgmt::rl_deprep_terminal() }
}

/// Read the operating system's terminal editing characters (as would be displayed by stty) to their
/// Readline equivalents. The bindings are performed in `kmap`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{keymap, termmgmt, util};
/// use std::ptr;
///
/// util::init();
///
/// let km = keymap::create_empty().unwrap_or(ptr::null_mut());
/// termmgmt::tty_set_default_bindings(km);
/// termmgmt::tty_unset_default_bindings(km);
/// ```
pub fn tty_set_default_bindings(kmap: Keymap) -> () {
    unsafe { ext_termmgmt::rl_tty_set_default_bindings(kmap) }
}

/// Reset the bindings manipulated by `rl_tty_set_default_bindings` so that the terminal editing
/// characters are bound to `rl_insert`. The bindings are performed in `kmap`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{keymap, termmgmt, util};
/// use std::ptr;
///
/// util::init();
///
/// let km = keymap::create_empty().unwrap_or(ptr::null_mut());
/// termmgmt::tty_set_default_bindings(km);
/// termmgmt::tty_unset_default_bindings(km);
/// ```
pub fn tty_unset_default_bindings(kmap: Keymap) -> () {
    unsafe { ext_termmgmt::rl_tty_unset_default_bindings(kmap) }
}

/// Reinitialize Readline's idea of the terminal settings using `name` as the terminal type (e.g.,
/// vt100). If `name` is None, the value of the TERM environment variable is used.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::{termmgmt, util};
///
/// util::init();
///
/// assert!(termmgmt::reset_terminal(Some("vt100")).is_ok());
/// assert!(termmgmt::reset_terminal(None).is_ok());
/// ```
pub fn reset_terminal(name: Option<&str>) -> Result<i32, ::ReadlineError> {
    let res = match name {
        Some(s) => {
            let cs = try!(CString::new(s));
            unsafe { ext_termmgmt::rl_reset_terminal(cs.as_ptr()) }
        },
        None => {
            unsafe { ext_termmgmt::rl_reset_terminal(ptr::null()) }
        }
    };
    if res == 0 {
        Ok(res)
    } else {
        Err(::ReadlineError::new("Termmgmt Error", "Unable to reset terminal!"))
    }
}
