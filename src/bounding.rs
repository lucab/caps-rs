use libc;

use super::Capability;
use errors::*;
use nr;

pub fn clear() -> Result<()> {
    for c in Capability::iter_variants() {
        if try!(has_cap(c)) {
            try!(drop(c));
        }
    }
    return Ok(());
}

pub fn drop(cap: Capability) -> Result<()> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_DROP, cap.index() as libc::c_uint, 0, 0) };
    return match ret {
        0 => Ok(()),
        _ => bail!("PR_CAPBSET_DROP error {:?}", ret),
    };
}

pub fn has_cap(cap: Capability) -> Result<bool> {
    let ret = unsafe { libc::prctl(nr::PR_CAPBSET_READ, cap.index() as libc::c_uint, 0, 0) };
    return match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => bail!("PR_CAPBSET_READ error {:?}", ret),
    };
}

pub fn read() -> Result<super::CapsHashSet> {
    let mut res = super::CapsHashSet::new();
    for c in Capability::iter_variants() {
        if try!(has_cap(c)) {
            res.insert(c);
        }
    }
    return Ok(res);
}
