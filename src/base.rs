use std::io::{Error, ErrorKind, Result};

use crate::nr;
use crate::{CapSet, Capability, CapsHashSet};

#[allow(clippy::unreadable_literal)]
const CAPS_V3: u32 = 0x20080522;

fn capget(hdr: &mut CapUserHeader, data: &mut CapUserData) -> Result<()> {
    let r = unsafe { libc::syscall(nr::CAPGET, hdr, data) };
    match r {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}

fn capset(hdr: &mut CapUserHeader, data: &CapUserData) -> Result<()> {
    let r = unsafe { libc::syscall(nr::CAPSET, hdr, data) };
    match r {
        0 => Ok(()),
        _ => Err(Error::last_os_error()),
    }
}

pub fn has_cap(tid: i32, cset: CapSet, cap: Capability) -> Result<bool> {
    let mut hdr = CapUserHeader {
        version: CAPS_V3,
        pid: tid,
    };
    let mut data: CapUserData = Default::default();
    capget(&mut hdr, &mut data)?;
    let caps: u64 = match cset {
        CapSet::Effective => (u64::from(data.effective_s1) << 32) + u64::from(data.effective_s0),
        CapSet::Inheritable => {
            (u64::from(data.inheritable_s1) << 32) + u64::from(data.inheritable_s0)
        }
        CapSet::Permitted => (u64::from(data.permitted_s1) << 32) + u64::from(data.permitted_s0),
        CapSet::Bounding | CapSet::Ambient => {
            return Err(Error::new(ErrorKind::Other, "not a base set"))
        }
    };
    let has_cap = (caps & cap.bitmask()) != 0;
    Ok(has_cap)
}

pub fn clear(tid: i32, cset: CapSet) -> Result<()> {
    let mut hdr = CapUserHeader {
        version: CAPS_V3,
        pid: tid,
    };
    let mut data: CapUserData = Default::default();
    capget(&mut hdr, &mut data)?;
    match cset {
        CapSet::Effective => {
            data.effective_s0 = 0;
            data.effective_s1 = 0;
        }
        CapSet::Inheritable => {
            data.inheritable_s0 = 0;
            data.inheritable_s1 = 0;
        }
        CapSet::Permitted => {
            data.effective_s0 = 0;
            data.effective_s1 = 0;
            data.permitted_s0 = 0;
            data.permitted_s1 = 0;
        }
        CapSet::Bounding | CapSet::Ambient => {
            return Err(Error::new(ErrorKind::Other, "not a base set"))
        }
    }
    capset(&mut hdr, &data)
}

pub fn read(tid: i32, cset: CapSet) -> Result<CapsHashSet> {
    let mut hdr = CapUserHeader {
        version: CAPS_V3,
        pid: tid,
    };
    let mut data: CapUserData = Default::default();
    capget(&mut hdr, &mut data)?;
    let caps: u64 = match cset {
        CapSet::Effective => (u64::from(data.effective_s1) << 32) + u64::from(data.effective_s0),
        CapSet::Inheritable => {
            (u64::from(data.inheritable_s1) << 32) + u64::from(data.inheritable_s0)
        }
        CapSet::Permitted => (u64::from(data.permitted_s1) << 32) + u64::from(data.permitted_s0),
        CapSet::Bounding | CapSet::Ambient => {
            return Err(Error::new(ErrorKind::Other, "not a base set"))
        }
    };
    let mut res = CapsHashSet::new();
    for c in super::all() {
        if (caps & c.bitmask()) != 0 {
            res.insert(c);
        }
    }
    Ok(res)
}

pub fn set(tid: i32, cset: CapSet, value: &CapsHashSet) -> Result<()> {
    let mut hdr = CapUserHeader {
        version: CAPS_V3,
        pid: tid,
    };
    let mut data: CapUserData = Default::default();
    capget(&mut hdr, &mut data)?;
    {
        let (s1, s0) = match cset {
            CapSet::Effective => (&mut data.effective_s1, &mut data.effective_s0),
            CapSet::Inheritable => (&mut data.inheritable_s1, &mut data.inheritable_s0),
            CapSet::Permitted => (&mut data.permitted_s1, &mut data.permitted_s0),
            CapSet::Bounding | CapSet::Ambient => {
                return Err(Error::new(ErrorKind::Other, "not a base set"))
            }
        };
        *s1 = 0;
        *s0 = 0;
        for c in value {
            match c.index() {
                0..=31 => {
                    *s0 |= c.bitmask() as u32;
                }
                32..=63 => {
                    *s1 |= (c.bitmask() >> 32) as u32;
                }
                _ => {
                    return Err(Error::new(
                        ErrorKind::InvalidInput,
                        format!("overlarge capability index {}", c.index()),
                    ))
                }
            }
        }
    }
    capset(&mut hdr, &data)?;
    Ok(())
}

pub fn drop(tid: i32, cset: CapSet, cap: Capability) -> Result<()> {
    let mut caps = read(tid, cset)?;
    if caps.remove(&cap) {
        set(tid, cset, &caps)?;
    };
    Ok(())
}

pub fn raise(tid: i32, cset: CapSet, cap: Capability) -> Result<()> {
    let mut caps = read(tid, cset)?;
    if caps.insert(cap) {
        set(tid, cset, &caps)?;
    };
    Ok(())
}

#[derive(Debug)]
#[repr(C)]
struct CapUserHeader {
    // Linux capabilities version (runtime kernel support)
    version: u32,
    // Process ID (thread)
    pid: i32,
}

#[derive(Debug, Default, Clone)]
#[repr(C)]
struct CapUserData {
    effective_s0: u32,
    permitted_s0: u32,
    inheritable_s0: u32,
    effective_s1: u32,
    permitted_s1: u32,
    inheritable_s1: u32,
}
