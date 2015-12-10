//! This module contains functions used to initialize the state of the History
//! library when you use the history functions in your program.

mod ext_readline {
    extern {
        pub fn using_history();
    }
}

/// Begins a session in which the history functions might be used.
///
/// This initializes the interactive variables.
pub fn using_history() {
    unsafe { ext_readline::using_history() }
}
