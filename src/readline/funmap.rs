//! [2.4.4 Associating Function Names and Bindings]
//! [2.4.4 associating function names and bindings]: https://goo.gl/CrXrWQ
//!
//! These functions allow you to find out what keys invoke named functions and the functions invoked
//! by a particular key sequence. You may also associate a new function name with an arbitrary
//! function.

mod ext_funmap {
    use libc::{
        //c_char,
        c_int
    };
    // use readline::{CommandFunction, Keymap};

    extern "C" {
        // pub fn rl_named_function(name: *const c_char) -> *mut Option<CommandFunction>;
        // pub fn rl_function_of_keyseq(keyseq: *const c_char, map: *mut Option<Keymap>, bind_type: *mut c_int) -> *mut Option<CommandFunction>;
        // pub fn rl_invoking_keyseqs(f: *mut Option<CommandFunction>) -> *mut *mut c_char;
        // pub fn rl_invoking_keyseqs_in_map(f: *mut Option<CommandFunction>, map: Keymap) -> *mut *mut c_char;
        pub fn rl_function_dumper(readable: c_int) -> ();
        // pub fn rl_list_funmap_names() -> *mut *mut c_char;
        // pub fn rl_add_funmap_entry(name: *const c_char, f: *mut Option<CommandFunction>) -> c_int;
    }
}

///
pub fn function_dumper(readable: bool) -> () {
    let i = if readable { 1 } else { 0 };
    unsafe { ext_funmap::rl_function_dumper(i) }
}

#[cfg(test)]
mod test {
    use readline::util;
    use super::*;

    #[test]
    fn test_function_dumper() {
        util::init();
        function_dumper(false);
        function_dumper(true);
    }
}
