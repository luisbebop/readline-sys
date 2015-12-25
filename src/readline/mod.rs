//! GNU Readline API
//!
//! This chapter describes the interface between the GNU Readline Library and other programs. If you
//! are a programmer, and you wish to include the features found in GNU Readline such as completion,
//! line editing, and interactive history manipulation in your own programs, this section is for
//! you.
//!
//! [2.1 Basic Behavior]
//! [2.1 basic behavior]: https://goo.gl/lGL9K0
//! Many programs provide a command line interface, such as mail, ftp, and sh. For such programs,
//! the default behaviour of Readline is sufficient. This section describes how to use Readline in
//! the simplest way possible, perhaps to replace calls in your code to gets() or fgets().
//!
//! The function `readline()` prints a prompt `prompt` and then reads and returns a single line of
//! text from the user. If prompt is NULL or the empty string, no prompt is displayed. The line
//! readline returns is allocated with malloc(); the caller should free() the line when it has
//! finished with it. The declaration for readline in ANSI C is:
//!
//! ```c
//!     char *readline (const char *prompt);
//! ```
//!
//! So, one might say
//!
//! ```c
//!     char *line = readline ("Enter a line: ");
//! ```
//!
//! in order to read a line of text from the user. The line returned has the final newline removed,
//! so only the text remains.
//!
//! If readline encounters an EOF while reading the line, and the line is empty at that point, then
//! ```(char *)NULL``` is returned. Otherwise, the line is ended just as if a newline had been
//! typed.
//!
//! If you want the user to be able to get at the line later, (with `C-p` for example), you must
//! call `add_history()` to save the line away in a history list of such lines.
//!
//! ```c
//!     add_history (line);
//! ```
//!
//! For full details on the GNU History Library, see the associated manual.
use libc::{c_char, c_int, c_long, c_uint, c_ushort, c_void, free, size_t};
use std::ffi::{CStr, CString};
use std::fmt;
use std::mem;

pub mod binding;
pub mod charin;
mod ext_readline {
    use libc::c_char;
    use super::HandlerFunction;

    extern "C" {
        pub fn readline(p: *const c_char) -> *const c_char;
        pub fn rl_callback_handler_install(p: *const c_char,
                                           lhandler: *mut Option<HandlerFunction>)
                                           -> ();
        pub fn rl_callback_read_char() -> ();
        pub fn rl_callback_handler_remove() -> ();
    }
}
pub mod funmap;
pub mod keymap;
pub mod misc;
pub mod modtext;
pub mod naming;
pub mod redisplay;
pub mod termmgmt;
pub mod util;
pub mod undo;
pub mod vars;

/// Readline Command Function Type
pub type CommandFunction = extern "C" fn(count: c_int, key: c_int) -> c_int;
/// Readline Hook Function Type
pub type HookFunction = extern "C" fn() -> c_int;
/// Readline getc Function Type
pub type GetcFunction = unsafe extern "C" fn(io: *mut IOFile) -> c_int;
/// Readline void Function Type
pub type VoidFunction = extern "C" fn() -> ();
/// Readline prep Function Type
pub type PrepFunction = extern "C" fn(flag: c_int) -> ();
/// Readline Callback Handler
pub type HandlerFunction = unsafe extern "C" fn(line: *mut c_char) -> ();
/// Keymap Entry Array
pub type KeymapEntryArray = [KeymapEntry; 257usize];
/// Keymap
pub type Keymap = *mut KeymapEntryArray;


#[repr(C)]
#[derive(Copy, Debug)]
/// I/O Marker
pub struct IOMarker {
    /// Next I/O marker.
    pub next: *mut IOMarker,
    /// File buffer.
    pub sbuf: *mut IOFile,
    /// Position.
    pub pos: c_int,
}

impl Clone for IOMarker {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for IOMarker {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
/// I/O File Type
pub struct IOFile {
    /// Flags
    pub flags: c_int,
    /// Flags
    pub read_ptr: *mut c_char,
    /// Flags
    pub read_end: *mut c_char,
    /// Flags
    pub read_base: *mut c_char,
    /// Flags
    pub write_base: *mut c_char,
    /// Flags
    pub write_ptr: *mut c_char,
    /// Flags
    pub write_end: *mut c_char,
    /// Flags
    pub buf_base: *mut c_char,
    /// Flags
    pub buf_end: *mut c_char,
    /// Flags
    pub save_base: *mut c_char,
    /// Flags
    pub backup_base: *mut c_char,
    /// Flags
    pub save_end: *mut c_char,
    /// Flags
    pub markers: *mut IOMarker,
    /// Flags
    pub chain: *mut IOFile,
    /// Flags
    pub fileno: c_int,
    /// Flags
    pub flags2: c_int,
    /// Flags
    pub old_offset: c_long,
    /// Flags
    pub cur_column: c_ushort,
    /// Flags
    pub vtable_offset: c_char,
    /// Flags
    pub shortbuf: [c_char; 1usize],
    /// Flags
    pub lock: *mut c_void,
    /// Flags
    pub offset: c_long,
    /// Flags
    pub pad1: *mut c_void,
    /// Flags
    pub pad2: *mut c_void,
    /// Flags
    pub pad3: *mut c_void,
    /// Flags
    pub pad4: *mut c_void,
    /// Flags
    pub pad5: size_t,
    /// Flags
    pub mode: c_int,
    /// Flags
    pub unused2: [c_char; 20usize],
}

impl Clone for IOFile {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for IOFile {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
/// Keymap Entry
pub struct KeymapEntry {
    /// Keymap Type
    pub type_: c_char,
    /// Keymap Function
    pub kfunc: *mut Option<extern "C" fn() -> c_int>,
}

impl Clone for KeymapEntry {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for KeymapEntry {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
/// Readline State
pub struct ReadlineState {
    /// Current point in the line.
    pub point: c_int,
    /// The end of the line.
    pub end: c_int,
    /// The mark point in the line.
    pub mark: c_int,
    /// The current line buffer.
    pub buffer: *mut c_char,
    /// The line buffer length.
    pub buflen: c_int,
    /// The undo list.
    pub ul: *mut UndoList,
    /// The current prompt.
    pub prompt: *mut c_char,
    /// The state bitflag.
    pub rlstate: c_int,
    /// The done flag.
    pub done: c_int,
    /// The current keymap.
    pub kmap: Keymap,
    /// The last function executed.
    pub lastfunc: *mut Option<extern "C" fn() -> c_int>,
    /// The insert mode.
    pub insmode: c_int,
    /// The edit mode.
    pub edmode: c_int,
    /// The last key sequence length.
    pub kseqlen: c_int,
    /// Infile.
    pub inf: *mut IOFile,
    /// Outfile.
    pub outf: *mut IOFile,
    /// The pending input.
    pub pendingin: c_int,
    /// A macro.
    pub makro: *mut c_char,
    /// Catch signals flag.
    pub catchsigs: c_int,
    /// Catch winch signal flag.
    pub catchsigwinch: c_int,
    /// Reserved for later expansion so the struct size doesn't change.
    pub reserved: [c_char; 64usize],
}

impl fmt::Debug for ReadlineState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("ReadlineState")
           .field("point", &self.point)
           .field("end", &self.end)
           .field("mark", &self.mark)
           .field("buffer", &self.buffer)
           .field("buflen", &self.buflen)
           .field("ul", &self.ul)
           .field("prompt", &self.prompt)
           .field("rlstate", &self.rlstate)
           .field("done", &self.done)
           .field("kmap", &self.kmap)
           .field("lastfunc", &self.lastfunc)
           .field("insmode", &self.insmode)
           .field("edmode", &self.edmode)
           .field("kseqlen", &self.kseqlen)
           .field("inf", &self.inf)
           .field("outf", &self.outf)
           .field("pendingin", &self.pendingin)
           .field("macro", &self.makro)
           .field("catchsigs", &self.catchsigs)
           .field("catchsigwinch", &self.catchsigwinch)
           .finish()
    }
}

impl Clone for ReadlineState {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for ReadlineState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy, Debug)]
/// Undo List
pub struct UndoList {
    /// Next Undo item.
    pub next: *mut UndoList,
    /// Start of the text for this undo.
    pub start: c_int,
    /// End of the text for this undo.
    pub end: c_int,
    /// The undo text.
    pub text: *mut c_char,
    /// The type of undo (UndoDelete, UndoInsert, UndoBegin, UndoEnd)
    pub what: c_uint,
}

impl Clone for UndoList {
    fn clone(&self) -> Self {
        *self
    }
}

impl Default for UndoList {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

/// Wraps the libreadline readline function.  The argument is the prompt to use.
///
/// If readline encounters an `EOF` while reading the line, and the line is empty at that point,
/// then `Ok(None)` is returned. Otherwise, the line is ended just as if a newline has been typed.
///
/// # Examples
///
/// ```
/// use rl_sys::readline;
///
/// loop {
///     match readline::readline("$ ") {
///         Ok(Some(s)) => println!("{}", s),
///         Ok(None) => break,
///         Err(e) => {
///             println!("{}", e);
///             break;
///        },
///     }
/// }
/// ```
pub fn readline(prompt: &str) -> Result<Option<String>, ::ReadlineError> {
    let prompt_ptr = try!(CString::new(prompt)).as_ptr();

    unsafe {
        let ret = ext_readline::readline(prompt_ptr);
        if ret.is_null() {
            // user pressed Ctrl-D
            Ok(None)
        } else {
            let line = CStr::from_ptr(ret).to_string_lossy().into_owned();
            free(ret as *mut c_void);
            Ok(Some(line))
        }
    }
}

/// [2.4.12 Alternate Interface]
/// [2.4.12 alternate interface]: https://goo.gl/gg9u4P
///
/// An alternate interface is available to plain `readline()``. Some applications need to interleave
/// keyboard I/O with file, device, or window system I/O, typically by using a main loop to select()
/// on various file descriptors. To accommodate this need, readline can also be invoked as a
/// 'callback' function from an event loop. There are functions available to make this easy.
///
/// Set up the terminal for readline I/O and display the initial expanded value of prompt `p`. Save
/// the value of `lhandler` to use as a handler function to call when a complete line of input has
/// been entered. The handler function receives the text of the line as an argument.
pub fn callback_handler_install(p: &str,
                                lhandler: *mut Option<HandlerFunction>)
                                -> Result<(), ::ReadlineError> {
    let ptr = try!(CString::new(p)).as_ptr();

    unsafe { Ok(ext_readline::rl_callback_handler_install(ptr, lhandler)) }
}

/// Whenever an application determines that keyboard input is available, it should call
/// `rl_callback_read_char()`, which will read the next character from the current input source. If
/// that character completes the line, `rl_callback_read_char` will invoke the `lhandler` function
/// installed by `rl_callback_handler_install` to process the line. Before calling the `lhandler`
/// function, the terminal settings are reset to the values they had before calling
/// `rl_callback_handler_install`. If the `lhandler` function returns, and the line handler remains
/// installed, the terminal settings are modified for Readline's use again. EOF is indicated by
/// calling `lhandler` with a NULL line.
pub fn callback_read_char() -> () {
    unsafe { ext_readline::rl_callback_read_char() }
}

/// Restore the terminal to its initial state and remove the line handler. This may be called from
/// within a callback as well as independently. If the `lhandler` installed by
/// `rl_callback_handler_install` does not exit the program, either this function or the function
/// referred to by the value of `rl_deprep_term_function` should be called before the program exits
/// to reset the terminal settings.
pub fn callback_handler_remove() -> () {
    unsafe { ext_readline::rl_callback_handler_remove() }
}
