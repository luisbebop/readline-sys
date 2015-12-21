//! [2.4.2 Selecting A Keymap]
//! [2.4.2 selecting a keymap]: https://cnswww.cns.cwru.edu/php/chet/readline/readline.html#SEC31
//!
//! Key bindings take place on a `keymap`. The keymap is the association between the keys that the
//! user types and the functions that get run. You can make your own keymaps, copy existing keymaps,
//! and tell Readline which keymap to use.
use readline::Keymap;

mod ext_keymap {
    use readline::Keymap;

    extern "C" {
        pub fn rl_make_bare_keymap() -> Keymap;
    }
}

/// Returns a new, empty keymap. The space for the keymap is allocated with `malloc()`; the caller
/// should free it by calling `rl_free_keymap()` when done.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::create_empty_keymap().unwrap();
/// assert!(!keymap.is_null());
/// ```
pub fn create_empty_keymap() -> Result<Keymap, ::ReadlineError> {
    unsafe {
        let keymap_ptr = ext_keymap::rl_make_bare_keymap();

        if keymap_ptr.is_null() {
            Err(::ReadlineError::new("Null Pointer", "Make bare keymap returned null pointer!"))
        } else {
            println!("Keymap PTR: {:?}", keymap_ptr);
            Ok(keymap_ptr)
        }
    }
}
