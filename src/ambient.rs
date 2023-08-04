//! Implementation of Ambient set.

use crate::Capabilities;
use crate::errors::CapsError;
use crate::runtime;
use crate::capability::Capability;
use std::io::Error;

pub fn clear() -> Result<(), CapsError> {
    let ret = unsafe { libc::prctl(crate::nr::PR_CAP_AMBIENT, crate::nr::PR_CAP_AMBIENT_CLEAR_ALL, 0, 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(format!(
            "PR_CAP_AMBIENT_CLEAR_ALL failure: {}",
            Error::last_os_error()
        )
        .into()),
    }
}

pub fn drop(cap: &Capability) -> Result<(), CapsError> {
    let ret = unsafe {
        libc::prctl(
            crate::nr::PR_CAP_AMBIENT,
            crate::nr::PR_CAP_AMBIENT_LOWER,
            libc::c_uint::from(cap.index()),
            0,
            0,
        )
    };
    match ret {
        0 => Ok(()),
        _ => Err(format!("PR_CAP_AMBIENT_LOWER failure: {}", Error::last_os_error()).into()),
    }
}

pub fn has_cap(cap: &Capability) -> Result<bool, CapsError> {
    let ret = unsafe {
        libc::prctl(
            crate::nr::PR_CAP_AMBIENT,
            crate::nr::PR_CAP_AMBIENT_IS_SET,
            libc::c_uint::from(cap.index()),
            0,
            0,
        )
    };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(format!("PR_CAP_AMBIENT_IS_SET failure: {}", Error::last_os_error()).into()),
    }
}

pub fn raise(cap: &Capability) -> Result<(), CapsError> {
    let ret = unsafe {
        libc::prctl(
            crate::nr::PR_CAP_AMBIENT,
            crate::nr::PR_CAP_AMBIENT_RAISE,
            libc::c_uint::from(cap.index()),
            0,
            0,
        )
    };
    match ret {
        0 => Ok(()),
        _ => Err(format!("PR_CAP_AMBIENT_RAISE failure: {}", Error::last_os_error()).into()),
    }
}

pub fn read() -> Result<Capabilities, CapsError> {
    let mut res = super::Capabilities::new();
    for c in runtime::thread_all_supported() {
        if has_cap(&c)? {
            res.insert(&c);
        }
    }
    Ok(res)
}

pub fn set(value: &super::Capabilities) -> Result<(), CapsError> {
    for c in runtime::thread_all_supported() {
        if value.contains(&c) {
            raise(&c)?;
        } else {
            drop(&c)?;
        };
    }
    Ok(())
}
