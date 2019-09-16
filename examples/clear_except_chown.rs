extern crate caps;

use caps::{CapSet, Capability, CapsHashSet};

fn main() {
    let mut set = CapsHashSet::new();
    set.insert(Capability::CAP_CHOWN);
    caps::set(None, CapSet::Effective, &set).unwrap();
    caps::set(None, CapSet::Permitted, &set).unwrap();
    let perm = caps::read(None, CapSet::Permitted).unwrap();
    println!("Permitted {:?}", perm);
    let eff = caps::read(None, CapSet::Effective).unwrap();
    println!("Effective {:?}", eff);
}
