//! [2.3 Readline Variables]
//! [2.3 readline variables]: https://goo.gl/E1D6om
use libc::{c_char, c_int};
use readline::{CommandFunction, GetcFunction, HookFunction, IOFile, Keymap, PrepFunction,
               VoidFunction};
use std::ffi::CStr;

bitflags!(
/// Readline State Bitflags
    pub flags ReadlineState: i32 {
/// Readline has not yet been called, nor has it begun to initialize.
        const RL_STATE_NONE         = 0x0000000,
/// Readline is initializing its internal data structures.
        const RL_STATE_INITIALIZING = 0x0000001,
/// Readline has completed its initialization.
        const RL_STATE_INITIALIZED  = 0x0000002,
/// Readline has modified the terminal modes to do its own input and redisplay.
        const RL_STATE_TERMPREPPED  = 0x0000004,
/// Readline is reading a command from the keyboard.
        const RL_STATE_READCMD      = 0x0000008,
/// Readline is reading more input after reading the meta-prefix character.
        const RL_STATE_METANEXT     = 0x0000010,
/// Readline is dispatching to a command.
        const RL_STATE_DISPATCHING  = 0x0000020,
/// Readline is reading more input while executing an editing command.
        const RL_STATE_MOREINPUT    = 0x0000040,
/// Readline is performing an incremental history search.
        const RL_STATE_ISEARCH      = 0x0000080,
/// Readline is performing a non-incremental history search.
        const RL_STATE_NSEARCH      = 0x0000100,
/// Readline is searching backward or forward through the history for a string.
        const RL_STATE_SEARCH       = 0x0000200,
/// Readline is reading a numeric argument.
        const RL_STATE_NUMERICARG   = 0x0000400,
/// Readline is currently getting its input from a previously-defined keyboard macro.
        const RL_STATE_MACROINPUT   = 0x0000800,
/// Readline is currently reading characters defining a keyboard macro.
        const RL_STATE_MACRODEF     = 0x0001000,
/// Readline is in overwrite mode.
        const RL_STATE_OVERWRITE    = 0x0002000,
/// Readline is performing word completion.
        const RL_STATE_COMPLETING   = 0x0004000,
/// Readline is currently executing the readline signal handler.
        const RL_STATE_SIGHANDLER   = 0x0008000,
/// Readline is performing an undo.
        const RL_STATE_UNDOING      = 0x0010000,
/// Readline has input pending due to a call to `rl_execute_next()`.
        const RL_STATE_INPUTPENDING = 0x0020000,
/// Readline has saved the values of the terminal's special characters.
        const RL_STATE_TTYCSAVED    = 0x0040000,
/// Readline is currently using the alternate (callback) interface (see section
/// [2.4.12 Alternate Interface]).
/// [2.4.12 alternate interface]: https://goo.gl/PHb4Kq
        const RL_STATE_CALLBACK     = 0x0080000,
/// Readline is reading the argument to a vi-mode "motion" command.
        const RL_STATE_VIMOTION     = 0x0100000,
/// Readline is reading a multiple-keystroke command.
        const RL_STATE_MULTIKEY     = 0x0200000,
/// Readline has entered vi command (movement) mode at least one time during the current
/// call to `readline()`.
        const RL_STATE_VICMDONCE    = 0x0400000,
/// Readline has read a key sequence bound to `accept-line` and is about to return the line
/// to the caller.
        const RL_STATE_DONE         = 0x1000000,
    }
);

extern "C" {
    /// This is the line gathered so far. You are welcome to modify the contents of the line, but
    /// see [2.4.5 Allowing Undoing]. The function `rl_extend_line_buffer`
    /// is available to increase the memory allocated to `rl_line_buffer`.
    /// [2.4.5 allowing undoing]: https://goo.gl/oYj9bq
    pub static mut rl_line_buffer: *mut c_char;
    /// The offset of the current cursor position in `rl_line_buffer` (the *point*).
    pub static rl_point: c_int;
    /// The number of characters present in `rl_line_buffer`. When `rl_point` is at the end of the
    /// line, `rl_point` and `rl_end` are equal.
    pub static rl_end: c_int;
    /// The mark (saved position) in the current line. If set, the mark and point define a region.
    pub static rl_mark: c_int;
    /// Setting this to a non-zero value causes Readline to return the current line immediately.
    pub static rl_done: c_int;
    /// Setting this to a positive value before calling `readline()` causes Readline to return
    /// after accepting that many characters, rather than reading up to a character bound to
    /// `accept-line`.
    pub static rl_num_chars_to_read: c_int;
    /// Setting this to a value makes it the next keystroke read. This is a way to stuff a single
    /// character into the input stream.
    pub static rl_pending_input: c_int;
    /// Set to a non-zero value if a function is being called from a key binding; zero otherwise.
    /// Application functions can test this to discover whether they were called directly or by
    /// Readline's dispatching mechanism.
    pub static rl_dispatching: c_int;
    /// Setting this to a non-zero value causes Readline to completely erase the current line,
    /// including any prompt, any time a newline is typed as the only character on an
    /// otherwise-empty line. The cursor is moved to the beginning of the newly-blank line.
    pub static rl_erase_empty_line: c_int;
    /// The prompt Readline uses. This is set from the argument to `readline()`, and should not be
    /// assigned to directly. The `rl_set_prompt()` function (see section [2.4.6 Redisplay]) may be
    /// used to modify the prompt string after calling `readline()`.
    /// [2.4.6 redisplay]: https://goo.gl/aTbhPU
    pub static rl_prompt: *const c_char;
    /// The string displayed as the prompt. This is usually identical to `rl_prompt`, but may be
    /// changed temporarily by functions that use the prompt string as a message area, such as
    /// incremental search.
    pub static rl_display_prompt: *const c_char;
    /// If an application wishes to display the prompt itself, rather than have Readline do it the
    /// first time `readline()` is called, it should set this variable to a non-zero value after
    /// displaying the prompt. The prompt must also be passed as the argument to `readline()` so
    /// the redisplay functions can update the display properly. The calling application is
    /// responsible for managing the value; Readline never sets it.
    pub static mut rl_already_prompted: c_int;
    static rl_library_version: *const c_char;
    /// An integer encoding the current version of the library. The encoding is of the form 0xMMmm,
    /// where MM is the two-digit major version number, and mm is the two-digit minor version
    /// number. For example, for Readline-4.2, `rl_readline_version` would have the value 0x0402.
    pub static rl_readline_version: c_int;
    /// Always set to 1, denoting that this is GNU readline rather than some emulation.
    pub static rl_gnu_readline_p: c_int;
    /// The terminal type, used for initialization. If not set by the application, Readline sets
    /// this to the value of the TERM environment variable the first time it is called.
    pub static rl_terminal_name: *const c_char;
    /// This variable is set to a unique name by each application using Readline. The value allows
    /// conditional parsing of the inputrc file (see section
    /// [1.3.2 Conditional Init Constructs]).
    /// [1.3.2 conditional init constructs]: https://goo.gl/ENcVC5
    pub static rl_readline_name: *const c_char;
    /// The stdio stream from which Readline reads input. If NULL, Readline defaults to stdin.
    pub static rl_instream: *mut IOFile;
    /// The stdio stream to which Readline performs output. If NULL, Readline defaults to stdout.
    pub static rl_outstream: *mut IOFile;
    /// If non-zero, Readline gives values found in the LINES and COLUMNS environment variables
    /// greater precedence than values fetched from the kernel when computing the screen dimensions.
    pub static rl_prefer_env_winsize: c_int;
    /// The address of the last command function Readline executed. May be used to test whether or
    /// not a function is being executed twice in succession, for example.
    pub static rl_last_command_func: *mut CommandFunction;
    /// If non-zero, this is the address of a function to call just before `readline` prints the
    /// first prompt.
    pub static rl_startup_hook: *mut HookFunction;
    /// If non-zero, this is the address of a function to call after the first prompt has been
    /// printed and just before `readline` starts reading input characters.
    pub static rl_pre_input_hook: *mut HookFunction;
    /// If non-zero, this is the address of a function to call periodically when Readline is waiting
    /// for terminal input. By default, this will be called at most ten times a second if there is
    /// no keyboard input.
    pub static rl_event_hook: *mut HookFunction;
    /// If non-zero, Readline will call indirectly through this pointer to get a character from the
    /// input stream. By default, it is set to `rl_getc`, the default Readline character input
    /// function (see section [2.4.8 Character Input]). In general, an application that sets
    /// `rl_getc_function` should consider setting `rl_input_available_hook` as well.
    /// [2.4.8 character input]: https://goo.gl/olQQLc
    pub static rl_getc_function: *mut GetcFunction;
    /// If non-zero, this is the address of a function to call if a read system call is interrupted
    /// when Readline is reading terminal input.
    pub static rl_signal_event_hook: *mut HookFunction;
    /// If non-zero, Readline will use this function's return value when it needs to determine
    /// whether or not there is available input on the current input source. The default hook checks
    /// `rl_instream`; if an application is using a different input source, it should set the hook
    /// appropriately. Readline queries for available input when implementing intra-key-sequence
    /// timeouts during input and incremental searches. This may use an application-specific timeout
    /// before returning a value; Readline uses the value passed to
    /// `rl_set_keyboard_input_timeout()` or the value of the user-settable `keyseq-timeout`
    /// variable. This is designed for use by applications using Readline's callback interface (see
    /// section [2.4.12 Alternate Interface]), which may not use the traditional `read(2)` and file
    /// descriptor interface, or other applications using a different input mechanism. If an
    /// application uses an input mechanism or hook that can potentially exceed the value of
    /// `keyseq-timeout`, it should increase the timeout or set this hook appropriately even when
    /// not using the callback interface. In general, an application that sets `rl_getc_function`
    /// should consider setting `rl_input_available_hook` as well.
    /// [2.4.12 alternate interface]: https://goo.gl/PHb4Kq
    pub static rl_input_available_hook: *mut HookFunction;
    /// If non-zero, Readline will call indirectly through this pointer to update the display with
    /// the current contents of the editing buffer. By default, it is set to `rl_redisplay`, the
    /// default Readline redisplay function (see section [2.4.6 Redisplay]).
    /// [2.4.6 redisplay]: https://goo.gl/aTbhPU
    pub static rl_redisplay_function: *mut VoidFunction;
    /// If non-zero, Readline will call indirectly through this pointer to initialize the terminal.
    /// The function takes a single argument, an int flag that says whether or not to use eight-bit
    /// characters. By default, this is set to `rl_prep_terminal` (see section
    /// [2.4.9 Terminal Management]).
    /// [2.4.9 terminal management]: https://goo.gl/1xVE8y
    pub static rl_prep_term_function: *mut PrepFunction;
    /// If non-zero, Readline will call indirectly through this pointer to reset the terminal. This
    /// function should undo the effects of `rl_prep_term_function`. By default, this is set to
    /// `rl_deprep_terminal` (see section [2.4.9 Terminal Management]).
    /// [2.4.9 terminal management]: https://goo.gl/1xVE8y
    pub static rl_deprep_term_function: *mut VoidFunction;
    /// This variable is set to the keymap (see section [2.4.2 Selecting a Keymap]) in which the
    /// currently executing readline function was found.
    /// [2.4.2 selecting a keymap]: https://goo.gl/WMVvss
    pub static rl_executing_keymap: *mut Keymap;
    /// This variable is set to the keymap (see section [2.4.2 Selecting a Keymap]) in which the
    /// last key binding occurred.
    /// [2.4.2 selecting a keymap]: https://goo.gl/WMVvss
    pub static rl_binding_keymap: *mut Keymap;
    /// This variable is set to the text of any currently-executing macro.
    pub static rl_executing_macro: *mut c_char;
    /// The key that caused the dispatch to the currently-executing Readline function.
    pub static rl_executing_key: c_int;
    /// The full key sequence that caused the dispatch to the currently-executing Readline function.
    pub static rl_executing_keyseq: *mut c_char;
    /// The number of characters in `rl_executing_keyseq`.
    pub static rl_key_sequence_length: c_int;
    /// A variable with bit values that encapsulate the current Readline state.
    static rl_readline_state: c_int;
    /// Set to a non-zero value if an explicit numeric argument was specified by the user. Only
    /// valid in a bindable command function.
    pub static rl_explicit_arg: c_int;
    /// Set to the value of any numeric argument explicitly specified by the user before executing
    /// the current Readline function. Only valid in a bindable command function.
    pub static rl_numeric_arg: c_int;
    /// Set to a value denoting Readline's current editing mode. A value of 1 means Readline is
    /// currently in emacs mode; 0 means that vi mode is active.
    pub static rl_editing_mode: c_int;
}

/// Get the Readline state flags.
pub fn get_state() -> Option<ReadlineState> {
    ReadlineState::from_bits(rl_readline_state)
}

/// The version number of this revision of the library.
pub fn get_library_version() -> String {
    unsafe { CStr::from_ptr(rl_library_version).to_string_lossy().into_owned() }
}

#[cfg(test)]
mod test {
    use super::*;
    use readline::util;

    #[test]
    fn test_rl_readline_version() {
        util::init();
        assert!(get_library_version() == "6.3");
        assert!(rl_readline_version == 0x0603);
    }

    #[test]
    fn test_get_state() {
        util::init();
        if let Some(s) = get_state() {
            assert!(s | RL_STATE_INITIALIZED == RL_STATE_INITIALIZED);
        } else {
            assert!(false);
        }

    }
}
