/*!
A pure-Rust library to work with Linux capabilities.

It provides support for manipulating capabilities available on modern Linux
kernels. It supports traditional POSIX sets (Effective, Inheritable, Permitted)
as well as Linux-specific Ambient and Bounding capabilities sets.

```rust
type ExResult<T> = Result<T, Box<dyn std::error::Error + 'static>>;

fn manipulate_caps() -> ExResult<()> {
    use caps::{Capability, CapSet};

    if caps::has_cap(None, CapSet::Permitted, Capability::CAP_SYS_NICE)? {
        caps::drop(None, CapSet::Effective, Capability::CAP_SYS_NICE)?;
        let effective = caps::read(None, CapSet::Effective)?;
        assert_eq!(effective.contains(&Capability::CAP_SYS_NICE), false);

        caps::clear(None, CapSet::Effective)?;
        let cleared = caps::read(None, CapSet::Effective)?;
        assert_eq!(cleared.is_empty(), true);
    };

    Ok(())
}
```
!*/

pub mod errors;
pub mod runtime;
pub mod securebits;

// Implementation of Bounding set.
mod ambient;
// Implementation of POSIX sets.
mod base;
// Implementation of Bounding set.
mod bounding;
// Enum and structs defining capabilities and sets
mod caps;
// All kernel-related constants.
mod nr;

use crate::errors::CapsError;

pub use crate::caps::{Capability, CapsBitFlags, CapsHashSet, CapsList};

/// Linux capabilities sets.
///
/// All capabilities sets supported by Linux, including standard
/// POSIX and custom ones. See `capabilities(7)`.
#[derive(Debug, Clone, Copy)]
pub enum CapSet {
    /// Ambient capabilities set (from Linux 4.3).
    Ambient,
    /// Bounding capabilities set (from Linux 2.6.25)
    Bounding,
    /// Effective capabilities set (from POSIX)
    Effective,
    /// Inheritable capabilities set (from POSIX)
    Inheritable,
    /// Permitted capabilities set (from POSIX)
    Permitted,
}

/// Check if a thread contains a capability in a set.
///
/// Check if set `cset` for thread `tid` contains capability `cap`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot check Ambient or Bounding capabilities of other processes.
pub fn has_cap(tid: Option<i32>, cset: CapSet, cap: Capability) -> Result<bool, CapsError> {
    let t = tid.unwrap_or(0);
    match cset {
        CapSet::Ambient if t == 0 => ambient::has_cap(cap),
        CapSet::Bounding if t == 0 => bounding::has_cap(cap),
        CapSet::Effective | CapSet::Inheritable | CapSet::Permitted => base::has_cap(t, cset, cap),
        _ => Err("operation not supported".into()),
    }
}

/// Return all capabilities in a set for a thread.
///
/// Return current content of set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot read Ambient or Bounding capabilities of other processes.
pub fn read_caps<T: CapsList>(tid: Option<i32>, cset: CapSet) -> Result<T, CapsError> {
    let t = tid.unwrap_or(0);
    match cset {
        CapSet::Ambient if t == 0 => ambient::read(),
        CapSet::Bounding if t == 0 => bounding::read(),
        CapSet::Effective | CapSet::Inheritable | CapSet::Permitted => base::read(t, cset),
        _ => Err("operation not supported".into()),
    }
}

/// Return all capabilities in a set for a thread.
///
/// Return current content of set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot read Ambient or Bounding capabilities of other processes.
#[deprecated(note = "please use `read_caps` instead")]
pub fn read(tid: Option<i32>, cset: CapSet) -> Result<CapsHashSet, CapsError> {
    read_caps(tid, cset)
}

/// Set a capability set for a thread to a new value.
///
/// All and only capabilities in `value` will be set for set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient set of other processes.
/// Capabilities cannot be set in Bounding set.
pub fn set_caps<T: CapsList>(tid: Option<i32>, cset: CapSet, value: &T) -> Result<(), CapsError> {
    let t = tid.unwrap_or(0);
    match cset {
        CapSet::Ambient if t == 0 => ambient::set(value),
        CapSet::Effective | CapSet::Inheritable | CapSet::Permitted => base::set(t, cset, value),
        _ => Err("operation not supported".into()),
    }
}

/// Set a capability set for a thread to a new value.
///
/// All and only capabilities in `value` will be set for set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient set of other processes.
/// Capabilities cannot be set in Bounding set.
#[deprecated(note = "please use `set_caps` instead")]
pub fn set(tid: Option<i32>, cset: CapSet, value: &CapsHashSet) -> Result<(), CapsError> {
    set_caps(tid, cset, value)
}

/// Clear all capabilities in a set for a thread.
///
/// All capabilities will be cleared from set `cset` for thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient or Bounding set of other processes.
pub fn clear(tid: Option<i32>, cset: CapSet) -> Result<(), CapsError> {
    let t = tid.unwrap_or(0);
    match cset {
        CapSet::Ambient if t == 0 => ambient::clear(),
        CapSet::Bounding if t == 0 => bounding::clear(),
        CapSet::Effective | CapSet::Permitted | CapSet::Inheritable => base::clear(t, cset),
        _ => Err("operation not supported".into()),
    }
}

/// Raise a single capability in a set for a thread.
///
/// Capabilities `cap` will be raised from set `cset` of thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient set of other processes.
/// Capabilities cannot be raised in Bounding set.
pub fn raise(tid: Option<i32>, cset: CapSet, cap: Capability) -> Result<(), CapsError> {
    let t = tid.unwrap_or(0);
    match cset {
        CapSet::Ambient if t == 0 => ambient::raise(cap),
        CapSet::Effective | CapSet::Permitted | CapSet::Inheritable => base::raise(t, cset, cap),
        _ => Err("operation not supported".into()),
    }
}

/// Drop a single capability from a set for a thread.
///
/// Capabilities `cap` will be dropped from set `cset` of thread `tid`.
/// If `tid` is `None`, this operates on current thread (tid=0).
/// It cannot manipulate Ambient and Bounding sets of other processes.
pub fn drop(tid: Option<i32>, cset: CapSet, cap: Capability) -> Result<(), CapsError> {
    let t = tid.unwrap_or(0);
    match cset {
        CapSet::Ambient if t == 0 => ambient::drop(cap),
        CapSet::Bounding if t == 0 => bounding::drop(cap),
        CapSet::Effective | CapSet::Permitted | CapSet::Inheritable => base::drop(t, cset, cap),
        _ => Err("operation not supported".into()),
    }
}

/// Return the set of all capabilities supported by this library.
pub fn all_caps<T: CapsList>() -> T {
    T::from_iter(caps::all_iter())
}

/// Return the set of all capabilities supported by this library.
#[deprecated(note = "please use `all_caps` instead")]
pub fn all() -> CapsHashSet {
    all_caps()
}

/// Convert an informal capability name into a canonical form.
///
/// This converts the input string to uppercase and ensures that it starts with
/// `CAP_`, prepending it if necessary. It performs no validity checks so the
/// output may not represent an actual capability. To check if it is, pass it
/// to [`from_str`].
///
/// [`from_str`]: enum.Capability.html#method.from_str
pub fn to_canonical(name: &str) -> String {
    let uppername = name.to_uppercase();
    if uppername.starts_with("CAP_") {
        uppername
    } else {
        ["CAP_", &uppername].concat()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_all_roundtrip() {
        let all: CapsHashSet = all_caps();
        assert!(all.len() > 0);
        for c in all {
            let name = c.to_string();
            let parsed: Capability = name.parse().unwrap();
            assert_eq!(c, parsed);
        }
    }

    #[test]
    fn test_parse_invalid() {
        let p1 = Capability::from_str("CAP_FOO");
        let p1_err = p1.unwrap_err();
        assert!(p1_err.to_string().contains("invalid"));
        assert!(format!("{}", p1_err).contains("CAP_FOO"));
        let p2: Result<Capability, CapsError> = "CAP_BAR".parse();
        assert!(p2.is_err());
    }

    #[test]
    fn test_to_canonical() {
        let p1 = "foo";
        assert!(Capability::from_str(&to_canonical(p1)).is_err());
        let p2 = "sys_admin";
        assert!(Capability::from_str(&to_canonical(p2)).is_ok());
        let p3 = "CAP_SYS_CHROOT";
        assert!(Capability::from_str(&to_canonical(p3)).is_ok());
    }

    #[test]
    #[cfg(feature = "serde_support")]
    fn test_serde() {
        let p1 = Capability::from_str("CAP_CHOWN").unwrap();
        let ser = serde_json::to_value(&p1).unwrap();
        let deser: Capability = serde_json::from_value(ser).unwrap();
        assert_eq!(deser, p1);
    }
}
