//! Implementation of Ambient set.

use crate::errors::CapsError;
use crate::nr;
use crate::runtime;
use crate::{Capability, CapsBitFlags, CapsList};
use std::{io::Error, iter::FromIterator};

pub fn clear() -> Result<(), CapsError> {
    let ret = unsafe { libc::prctl(nr::PR_CAP_AMBIENT, nr::PR_CAP_AMBIENT_CLEAR_ALL, 0, 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(format!(
            "PR_CAP_AMBIENT_CLEAR_ALL failure: {}",
            Error::last_os_error()
        )
        .into()),
    }
}

pub fn drop(cap: Capability) -> Result<(), CapsError> {
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
        _ => Err(format!("PR_CAP_AMBIENT_LOWER failure: {}", Error::last_os_error()).into()),
    }
}

pub fn has_cap(cap: Capability) -> Result<bool, CapsError> {
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
        _ => Err(format!("PR_CAP_AMBIENT_IS_SET failure: {}", Error::last_os_error()).into()),
    }
}

pub fn raise(cap: Capability) -> Result<(), CapsError> {
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
        _ => Err(format!("PR_CAP_AMBIENT_RAISE failure: {}", Error::last_os_error()).into()),
    }
}

pub fn read<T: FromIterator<Capability>>() -> Result<T, CapsError> {
    runtime::thread_all_supported_caps::<CapsBitFlags>()
        .iter_caps()
        .filter_map(|c| match has_cap(c) {
            Ok(false) => None,
            Err(e) => Some(Err(e)),
            Ok(true) => Some(Ok(c)),
        })
        .collect()
}

pub fn set<T: super::CapsList>(value: &T) -> Result<(), CapsError> {
    for c in runtime::thread_all_supported_caps::<CapsBitFlags>().iter_caps() {
        if value.contains_cap(&c) {
            raise(c)?;
        } else {
            drop(c)?;
        };
    }
    Ok(())
}
