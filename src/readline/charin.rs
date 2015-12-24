//! [2.4.8 Character Input]
//! [2.4.8 character input]: https://goo.gl/yNtf60
use readline::IOFile;

mod ext_charin {
    use libc::c_int;
    use readline::IOFile;

    extern "C" {
        pub fn rl_read_key() -> c_int;
        pub fn rl_getc(stream: *mut IOFile) -> c_int;
        pub fn rl_stuff_char(c: c_int) -> c_int;
        pub fn rl_execute_next(c: c_int) -> c_int;
        pub fn rl_clear_pending_input() -> c_int;
        pub fn rl_set_keyboard_input_timeout(timeout: c_int) -> c_int;
    }
}

/// Return the next character available from Readline's current input stream. This handles input
/// inserted into the input stream via `rl_pending_input` (see section [2.3 Readline Variables]) and
/// `rl_stuff_char()`, macros, and characters read from the keyboard. While waiting for input, this
/// function will call any function assigned to the `rl_event_hook` variable.
/// [2.3 readline variables]: https://goo.gl/E1D6om
#[cfg_attr(feature = "clippy", allow(cast_possible_truncation, cast_sign_loss))]
pub fn read_key() -> Result<char, ::ReadlineError> {
    unsafe { Ok((ext_charin::rl_read_key() as u8) as char) }
}

/// Return the next character available from `stream`, which is assumed to be the keyboard.
#[cfg_attr(feature = "clippy", allow(cast_possible_truncation, cast_sign_loss))]
pub fn getc(stream: *mut IOFile) -> Result<char, ::ReadlineError> {
    unsafe { Ok((ext_charin::rl_getc(stream) as u8) as char) }
}

/// Insert `c` into the Readline input stream. It will be "read" before Readline attempts to read
/// characters from the terminal with `rl_read_key()`. Up to 512 characters may be pushed back.
/// `rl_stuff_char` returns 1 if the character was successfully inserted; 0 otherwise.
pub fn stuff_char(c: char) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_charin::rl_stuff_char(c as i32);

        if res == 1 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Charin Error", "Unable to stuff character!"))
        }
    }
}

/// Make `c` be the next command to be executed when `rl_read_key()`` is called. This sets
/// `rl_pending_input`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::charin;
///
/// assert!(charin::execute_next('t').is_ok());
/// ```
pub fn execute_next(c: char) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_charin::rl_execute_next(c as i32);

        if res == 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Charin Error", "Unable to execute next character!"))
        }
    }
}

/// Unset `rl_pending_input`, effectively negating the effect of any previous call to
/// `rl_execute_next()`. This works only if the pending input has not already been read with
/// `rl_read_key()`.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::charin;
///
/// assert!(charin::execute_next('t').is_ok());
/// assert!(charin::clear_pending_input().is_ok());
/// ```
pub fn clear_pending_input() -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_charin::rl_clear_pending_input();

        if res == 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Charin Error", "Unable to execute next character!"))
        }
    }
}

/// While waiting for keyboard input in `rl_read_key()`, Readline will wait for `us` microseconds
/// for input before calling any function assigned to `rl_event_hook`. `us` must be greater than or
/// equal to zero (a zero-length timeout is equivalent to a poll). The default waiting period is
/// one-tenth of a second. Returns the old timeout value.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::charin;
///
/// assert!(charin::set_keyboard_input_timeout(500000).is_ok());
/// ```
pub fn set_keyboard_input_timeout(us: i32) -> Result<i32, ::ReadlineError> {
    unsafe {
        let res = ext_charin::rl_set_keyboard_input_timeout(us);

        if res >= 0 {
            Ok(res)
        } else {
            Err(::ReadlineError::new("Charin Error", "Unable to set keyboard input timeout!"))
        }
    }
}
