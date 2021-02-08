use std::io::{Error, Result};

use crate::nr;
use crate::runtime;
use crate::Capability;

pub fn clear() -> Result<()> {
    for c in super::all() {
        if has_cap(c)? {
            drop(c)?;
        }
    }
    Ok(())
}

pub fn drop(cap: Capability) -> Result<()> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_DROP, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}

pub fn has_cap(cap: Capability) -> Result<bool> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_READ, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::last_os_error()),
    }
}

pub fn read() -> Result<super::CapsHashSet> {
    let mut res = super::CapsHashSet::new();
    for c in runtime::thread_all_supported() {
        if has_cap(c)? {
            res.insert(c);
        }
    }
    Ok(res)
}
