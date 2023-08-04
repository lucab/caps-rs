use crate::errors::CapsError;
use crate::capability::Capability;
use crate::runtime;
use crate::Capabilities;
use std::io::Error;

pub fn clear() -> Result<(), CapsError> {
    for c in Capabilities::all() {
        if has_cap(&c)? {
            drop(&c)?;
        }
    }
    Ok(())
}

pub fn drop(cap: &Capability) -> Result<(), CapsError> {
    let ret = unsafe { libc::prctl(crate::nr::PR_CAPBSET_DROP, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(()),
        _ => Err(CapsError::from(format!(
            "PR_CAPBSET_DROP failure: {}",
            Error::last_os_error()
        ))),
    }
}

pub fn has_cap(cap: &Capability) -> Result<bool, CapsError> {
    let ret = unsafe { libc::prctl(crate::nr::PR_CAPBSET_READ, libc::c_uint::from(cap.index()), 0, 0) };
    match ret {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(CapsError::from(format!(
            "PR_CAPBSET_READ failure: {}",
            Error::last_os_error()
        ))),
    }
}

pub fn read() -> Result<super::Capabilities, CapsError> {
    let mut res = super::Capabilities::new();
    for c in runtime::thread_all_supported() {
        if has_cap(&c)? {
            res.insert(&c);
        }
    }
    Ok(res)
}
