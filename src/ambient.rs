//! Implementation of Ambient set.

use std::io::{Error, Result};

use crate::nr;
use crate::runtime;
use crate::{Capability, CapsHashSet};

pub fn clear() -> Result<()> {
    let ret = unsafe { libc::prctl(nr::PR_CAP_AMBIENT, nr::PR_CAP_AMBIENT_CLEAR_ALL, 0, 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}

pub fn drop(cap: Capability) -> Result<()> {
    let ret = unsafe {
        libc::prctl(
            nr::PR_CAP_AMBIENT,
            nr::PR_CAP_AMBIENT_LOWER,
            libc::c_uint::from(cap.index()),
            0,
            0,
        )
    };
    match ret {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}

pub fn has_cap(cap: Capability) -> Result<bool> {
    let ret = unsafe {
        libc::prctl(
            nr::PR_CAP_AMBIENT,
            nr::PR_CAP_AMBIENT_IS_SET,
            libc::c_uint::from(cap.index()),
            0,
            0,
        )
    };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::last_os_error()),
    }
}

pub fn raise(cap: Capability) -> Result<()> {
    let ret = unsafe {
        libc::prctl(
            nr::PR_CAP_AMBIENT,
            nr::PR_CAP_AMBIENT_RAISE,
            libc::c_uint::from(cap.index()),
            0,
            0,
        )
    };
    match ret {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}

pub fn read() -> Result<CapsHashSet> {
    let mut res = super::CapsHashSet::new();
    for c in runtime::thread_all_supported() {
        if has_cap(c)? {
            res.insert(c);
        }
    }
    Ok(res)
}

pub fn set(value: &super::CapsHashSet) -> Result<()> {
    for c in runtime::thread_all_supported() {
        if value.contains(&c) {
            raise(c)?;
        } else {
            drop(c)?;
        };
    }
    Ok(())
}
