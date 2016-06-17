// Copyright 2015-2016 Jason Ozias
//
// This file is part of rl-sys.
//
// rl-sys is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rl-sys is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rl-sys.  If not, see <http://www.gnu.org/licenses/>.

include!(concat!(env!("OUT_DIR"), "/version.rs"));

#[cfg(unix)]
fn verbose_ver() -> String {
    format!("\x1b[32;1mrl-sys {}\x1b[0m ({} {}) (built {})\ncommit-hash: {}\ncommit-date: \
             {}\nbuild-date: {}\nhost: {}\nrelease: {}",
            semver(),
            short_sha(),
            commit_date(),
            short_now(),
            sha(),
            commit_date(),
            short_now(),
            target(),
            semver())
}

#[cfg(windows)]
fn verbose_ver() -> String {
    format!("rl-sys {} ({} {}) (built {})\ncommit-hash: {}\ncommit-date: {}\nbuild-date: \
             {}\nhost: {}\nrelease: {}",
            semver(),
            short_sha(),
            commit_date(),
            short_now(),
            sha(),
            commit_date(),
            short_now(),
            target(),
            semver())
}

#[cfg(unix)]
fn ver() -> String {
    format!("\x1b[32;1mrl-sys {}\x1b[0m ({} {}) (built {})",
            semver(),
            short_sha(),
            commit_date(),
            short_now())
}

#[cfg(windows)]
fn ver() -> String {
    format!("rl-sys {}[0m ({} {}) (built {})",
            semver(),
            short_sha(),
            commit_date(),
            short_now())
}

/// Generate a version string.
///
/// # Examples
/// ```
/// use rl_sys;
///
/// // Normal
/// println!("{}", rl_sys::version(false));
/// // rl-sys v0.1.3-pre-11-gd90443d (d90443d 2015-12-07) (built 2015-12-07)
///
/// // Verbose
/// println!("{}", rl_sys::version(true));
/// // rl-sys v0.1.3-pre-11-gd90443d (d90443d 2015-12-07) (built 2015-12-07)
/// // commit-hash: d90443d92db3826c648817e6bd6cb757729f7209
/// // commit-date: 2015-12-07
/// // build-date: 2015-12-07
/// // host: x86_64-unknown-linux-gnu
/// // release: v0.1.3-pre-11-gd90443d
/// ```
pub fn version(verbose: bool) -> String {
    if verbose { verbose_ver() } else { ver() }
}

#[cfg(test)]
mod test {
    #[cfg(all(unix,test))]
    const TEST_VER: [u8; 13] = [27, 91, 51, 50, 59, 49, 109, 114, 108, 45, 115, 121, 115];

    #[cfg(all(windows,test))]
    const TEST_VER: [u8; 6] = [114, 108, 45, 115, 121, 115];

    #[test]
    #[cfg(unix)]
    #[cfg_attr(feature="clippy", allow(indexing_slicing))]
    fn test_version() {
        use sodium_sys::crypto::utils::secmem;
        use super::version;

        let ver = version(false);
        let vb = ver.as_bytes();
        assert!(secmem::memcmp(&vb[..13], &TEST_VER) == 0);
        let verbose_ver = version(true);
        let verbose_vb = verbose_ver.as_bytes();
        assert!(secmem::memcmp(&verbose_vb[..13], &TEST_VER) == 0);
    }

    #[test]
    #[cfg(windows)]
    #[cfg_attr(feature="clippy", allow(indexing_slicing))]
    fn test_version() {
        use sodium_sys::crypto::utils::secmem;
        use super::version;

        let ver = version(false);
        let vb = ver.as_bytes();
        assert!(secmem::memcmp(&vb[..6], &TEST_VER) == 0);
        let verbose_ver = version(true);
        let verbose_vb = verbose_ver.as_bytes();
        assert!(secmem::memcmp(&verbose_vb[..6], &TEST_VER) == 0);
    }
}
