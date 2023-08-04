use caps::capability::Capability;
use caps::{Capabilities, CapSet};

#[test]
fn test_ambient_has_cap() {
    caps::has_cap(None, CapSet::Ambient, &Capability::CAP_CHOWN).unwrap();
}

#[test]
fn test_ambient_read() {
    caps::read(None, CapSet::Ambient).unwrap();
}

#[test]
fn test_ambient_clear() {
    caps::clear(None, CapSet::Ambient).unwrap();
    let empty = caps::read(None, CapSet::Ambient).unwrap();
    assert_eq!(empty.len(), 0);
}

#[test]
fn test_ambient_drop() {
    caps::drop(None, CapSet::Ambient, Capability::CAP_CHOWN).unwrap();
    let no_cap = caps::has_cap(None, CapSet::Ambient, &Capability::CAP_CHOWN).unwrap();
    assert_eq!(no_cap, false);
}

#[test]
fn test_ambient_drop_other() {
    assert!(caps::drop(Some(1), CapSet::Ambient, Capability::CAP_CHOWN).is_err());
}

#[test]
fn test_ambient_raise() {
    let r = caps::raise(None, CapSet::Ambient, Capability::CAP_CHOWN);
    let perm = caps::has_cap(None, CapSet::Permitted, &Capability::CAP_CHOWN).unwrap();
    let inhe = caps::has_cap(None, CapSet::Inheritable, &Capability::CAP_CHOWN).unwrap();
    match (perm, inhe) {
        (false, _) => assert!(r.is_err()),
        (true, false) => {
            caps::raise(None, CapSet::Inheritable, Capability::CAP_CHOWN).unwrap();
            caps::raise(None, CapSet::Ambient, Capability::CAP_CHOWN).unwrap();
        }
        (true, true) => r.unwrap(),
    };
}

#[test]
fn test_ambient_set() {
    let mut v = Capabilities::new();
    caps::set(None, CapSet::Ambient, &v).unwrap();
    let empty = caps::read(None, CapSet::Ambient).unwrap();
    assert_eq!(empty.len(), 0);
    v.insert(&Capability::CAP_CHOWN);
    caps::drop(None, CapSet::Ambient, Capability::CAP_CHOWN).unwrap();
    assert!(caps::set(None, CapSet::Ambient, &v).is_err());
}
