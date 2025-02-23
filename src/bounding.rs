use crate::errors::CapsError;
use crate::nr;
use crate::runtime;
use crate::{caps::all_iter, Capability, CapsList};
use std::{io::Error, iter::FromIterator};

pub fn clear() -> Result<(), CapsError> {
    for c in all_iter() {
        if has_cap(c)? {
            drop(c)?;
        }
    }
    Ok(())
}

pub fn drop(cap: Capability) -> Result<(), CapsError> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_DROP, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(CapsError::from(format!(
            "PR_CAPBSET_DROP failure: {}",
            Error::last_os_error()
        ))),
    }
}

pub fn has_cap(cap: Capability) -> Result<bool, CapsError> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_READ, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(CapsError::from(format!(
            "PR_CAPBSET_READ failure: {}",
            Error::last_os_error()
        ))),
    }
}

pub fn read<T: FromIterator<Capability>>() -> Result<T, CapsError> {
    runtime::thread_all_supported_caps::<crate::CapsBitFlags>()
        .iter_caps()
        .filter_map(|c| match has_cap(c) {
            Ok(false) => None,
            Err(e) => Some(Err(e)),
            Ok(true) => Some(Ok(c)),
        })
        .collect()
}
