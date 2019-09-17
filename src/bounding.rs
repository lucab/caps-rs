use failure::ResultExt;

use crate::errors::*;
use crate::nr;
use crate::{all, Capability, CapsHashSet};

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
        _ => Err(Error::Sys(errno::errno())).context("PR_CAPBSET_DROP error")?,
    }
}

pub fn has_cap(cap: Capability) -> Result<bool> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_READ, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(Error::Sys(errno::errno())).context("PR_CAPBSET_READ error")?,
    }
}

pub fn read() -> Result<CapsHashSet> {
    let mut res = CapsHashSet::new();
    for c in all() {
        if has_cap(c)? {
            res.insert(c);
        }
    }
    Ok(res)
}
