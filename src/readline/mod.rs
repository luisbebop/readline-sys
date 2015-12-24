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
use libc::{c_char, c_int, c_long, c_ushort, c_void, free, size_t};
use std::ffi::{CStr, CString};
use std::mem;

pub mod binding;
mod ext_readline {
    use libc::c_char;

    extern "C" {
        pub fn readline(p: *const c_char) -> *const c_char;
    }
}
pub mod funmap;
pub mod keymap;
pub mod naming;
pub mod redisplay;
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
/// Keymap Entry Array
pub type KeymapEntryArray = [KeymapEntry; 257usize];
/// Keymap
pub type Keymap = *mut KeymapEntryArray;


#[repr(C)]
#[derive(Copy)]
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
#[derive(Copy)]
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
#[derive(Copy)]
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
