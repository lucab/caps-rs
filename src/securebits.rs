//! Manipulate securebits flags
//!
//! This module exposes methods to get and set per-thread securebits
//! flags, which can be used to disable special handling of capabilities
//! for UID 0 (root).

use std::io::{Error, Result};

use crate::nr;

/// Return whether the current thread's "keep capabilities" flag is set.
pub fn has_keepcaps() -> Result<bool> {
    let ret = unsafe { libc::prctl(nr::PR_GET_KEEPCAPS, 0, 0, 0) };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::last_os_error()),
    }
}

/// Set the value of the current thread's "keep capabilities" flag.
pub fn set_keepcaps(keep_caps: bool) -> Result<()> {
    let flag = if keep_caps { 1 } else { 0 };
    let ret = unsafe { libc::prctl(nr::PR_SET_KEEPCAPS, flag, 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}
