use libc;

use super::Capability;
use errors::*;
use nr;

pub fn clear() -> Result<()> {
    let ret = unsafe { libc::prctl(nr::PR_CAP_AMBIENT, nr::PR_CAP_AMBIENT_CLEAR_ALL, 0, 0, 0) };
    match ret {
        0 => Ok(()),
        _ => bail!("PR_CAP_AMBIENT_CLEAR_ALL error {:?}", ret),
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
        _ => bail!("PR_CAP_AMBIENT_LOWER error {:?}", ret),
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
        _ => bail!("PR_CAP_AMBIENT_IS_SET error {:?}", ret),
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
        _ => bail!("PR_CAP_AMBIENT_RAISE error {:?}", ret),
    }
}

pub fn read() -> Result<super::CapsHashSet> {
    let mut res = super::CapsHashSet::new();
    for c in Capability::iter_variants() {
        if try!(has_cap(c)) {
            res.insert(c);
        }
    }
    Ok(res)
}

pub fn set(value: &super::CapsHashSet) -> Result<()> {
    for c in Capability::iter_variants() {
        if value.contains(&c) {
            try!(raise(c));
        } else {
            try!(drop(c));
        };
    }
    Ok(())
}
