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
        pub fn rl_copy_keymap(map: Keymap) -> Keymap;
        pub fn rl_make_keymap() -> Keymap;
        pub fn rl_discard_keymap(map: Keymap) -> ();
        pub fn rl_free_keymap(map: Keymap) -> ();
        pub fn rl_get_keymap() -> Keymap;
        pub fn rl_set_keymap(map: Keymap) -> ();
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
/// let keymap = keymap::create_empty().unwrap();
/// assert!(!keymap.is_null());
/// ```
pub fn create_empty() -> Result<Keymap, ::ReadlineError> {
    unsafe {
        let keymap_ptr = ext_keymap::rl_make_bare_keymap();

        if keymap_ptr.is_null() {
            Err(::ReadlineError::new("Null Pointer", "rl_make_bare_keymap returned null pointer!"))
        } else {
            Ok(keymap_ptr)
        }
    }
}

/// Return a new keymap which is a copy of map.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::create_empty().unwrap();
/// assert!(!keymap.is_null());
/// let km_copy = keymap::copy(keymap).unwrap();
/// assert!(!km_copy.is_null());
/// ```
pub fn copy(map: Keymap) -> Result<Keymap, ::ReadlineError> {
    unsafe {
        let keymap_ptr = ext_keymap::rl_copy_keymap(map);

        if keymap_ptr.is_null() {
            Err(::ReadlineError::new("Null Pointer", "rl_copy_keymap returned null pointer!"))
        } else {
            Ok(keymap_ptr)
        }
    }
}

/// Return a new keymap with the printing characters bound to `rl_insert`, the lowercase Meta
/// characters bound to run their equivalents, and the Meta digits bound to produce numeric
/// arguments.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::make().unwrap();
/// assert!(!keymap.is_null());
/// ```
pub fn make() -> Result<Keymap, ::ReadlineError> {
    unsafe {
        let keymap_ptr = ext_keymap::rl_make_keymap();

        if keymap_ptr.is_null() {
            Err(::ReadlineError::new("Null Pointer", "rl_make_keymap returned null pointer!"))
        } else {
            Ok(keymap_ptr)
        }
    }
}

/// Free the storage associated with the data in keymap. The caller should free keymap.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::make().unwrap();
/// assert!(!keymap.is_null());
/// keymap::discard(keymap);
/// ```
pub fn discard(map: Keymap) -> () {
    unsafe { ext_keymap::rl_discard_keymap(map) }
}

/// Free all storage associated with keymap. This calls `rl_discard_keymap` to free subordindate
/// keymaps and macros.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::make().unwrap();
/// assert!(!keymap.is_null());
/// keymap::free(keymap);
/// ```
pub fn free(map: Keymap) -> () {
    unsafe { ext_keymap::rl_free_keymap(map) }
}

/// Returns the currently active keymap.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::get().unwrap();
/// assert!(!keymap.is_null());
/// ```
pub fn get() -> Result<Keymap, ::ReadlineError> {
    unsafe {
        let keymap_ptr = ext_keymap::rl_get_keymap();

        if keymap_ptr.is_null() {
            Err(::ReadlineError::new("Null Pointer", "rl_get_keymap returned null pointer!"))
        } else {
            Ok(keymap_ptr)
        }
    }
}

/// Makes keymap the currently active keymap.
///
/// # Examples
///
/// ```
/// use rl_sys::readline::keymap;
///
/// let keymap = keymap::get().unwrap();
/// assert!(!keymap.is_null());
/// keymap::set(keymap);
/// ```
pub fn set(map: Keymap) -> () {
    unsafe { ext_keymap::rl_set_keymap(map) }
}
