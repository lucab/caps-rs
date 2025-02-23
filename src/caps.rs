use std::{collections::HashSet, convert::TryInto, iter::FromIterator, marker::PhantomData};

/// List of [`Capability`] stored as [`HashSet`].
pub type CapsHashSet = std::collections::HashSet<Capability>;

macro_rules! capability_list {
    ($($capability:ident $(: $doc:literal)? ),*) => {
        /// Linux capabilities.
        ///
        /// All capabilities supported by Linux, including standard
        /// POSIX and custom ones. See `capabilities(7)`.
        #[allow(clippy::manual_non_exhaustive)]
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
        #[repr(u8)]
        #[cfg_attr(
            feature = "serde_support",
            derive(serde::Serialize, serde::Deserialize)
        )]
        pub enum Capability {
            $(
                $(#[doc = $doc])?
                $capability = crate::nr::$capability,
            )*
            #[doc(hidden)]
            __Nonexhaustive,
        }
        impl ::std::fmt::Display for Capability {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let name = match *self {
                    $(
                        Self::$capability => stringify!($capability),
                    )*
                    Self::__Nonexhaustive => unreachable!("invalid capability")
                };
                write!(f, "{}", name)
            }
        }
        impl ::std::str::FromStr for Capability {
            type Err = crate::CapsError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    $(
                        stringify!($capability) => Ok(Self::$capability),
                    )*
                    _ => Err(format!("invalid capability: {}", s).into())
                }
            }
        }

        /// Return the set of all capabilities supported by this library.
        pub fn all_iter() -> impl ::std::iter::Iterator<Item = Capability> {
            ::std::iter::IntoIterator::into_iter([$(Capability::$capability),*])
        }

        ::bitflags::bitflags! {
            /// List of [`Capability`] stored as `BitFlag`.
            #[derive(PartialEq, Eq, Debug, Clone, Copy)]
            pub struct CapsBitFlags: u64 {
                $(
                    $(#[doc = $doc])?
                    const $capability = 1u64 << crate::nr::$capability;
                )*
            }
        }

        impl ::std::convert::TryFrom<CapsBitFlags> for Capability {
            type Error = crate::CapsError;

            /// Converts a [`CapsBitFlags`] into [`Capability`], returning an error if the bitflag
            /// contains more than one [`Capability`].
            fn try_from(value: CapsBitFlags) -> ::std::result::Result<Self, Self::Error> {
                $(
                    const $capability: u64 = 1u64 << crate::nr::$capability;
                )*
                match value.bits() {
                    $(
                        $capability => Ok(Self::$capability),
                    )*
                    _ => Err(format!("cannot convert following bits to single capability: {}", value.bits()).into())
                }
            }
        }
    };
}

capability_list!(
    CAP_CHOWN: "`CAP_CHOWN` (from POSIX)",
    CAP_DAC_OVERRIDE: "`CAP_DAC_OVERRIDE` (from POSIX)",
    CAP_DAC_READ_SEARCH: "`CAP_DAC_READ_SEARCH` (from POSIX)",
    CAP_FOWNER: "`CAP_FOWNER` (from POSIX)",
    CAP_FSETID: "`CAP_FSETID` (from POSIX)",
    CAP_KILL: "`CAP_KILL` (from POSIX)",
    CAP_SETGID: "`CAP_SETGID` (from POSIX)",
    CAP_SETUID: "`CAP_SETUID` (from POSIX)",
    CAP_SETPCAP: "`CAP_SETPCAP` (from Linux)",
    CAP_LINUX_IMMUTABLE,
    CAP_NET_BIND_SERVICE,
    CAP_NET_BROADCAST,
    CAP_NET_ADMIN,
    CAP_NET_RAW,
    CAP_IPC_LOCK,
    CAP_IPC_OWNER,
    CAP_SYS_MODULE: "`CAP_SYS_MODULE` (from Linux)",
    CAP_SYS_RAWIO: "`CAP_SYS_RAWIO` (from Linux)",
    CAP_SYS_CHROOT: "`CAP_SYS_CHROOT` (from Linux)",
    CAP_SYS_PTRACE: "`CAP_SYS_PTRACE` (from Linux)",
    CAP_SYS_PACCT: "`CAP_SYS_PACCT` (from Linux)",
    CAP_SYS_ADMIN: "`CAP_SYS_ADMIN` (from Linux)",
    CAP_SYS_BOOT: "`CAP_SYS_BOOT` (from Linux)",
    CAP_SYS_NICE: "`CAP_SYS_NICE` (from Linux)",
    CAP_SYS_RESOURCE: "`CAP_SYS_RESOURCE` (from Linux)",
    CAP_SYS_TIME: "`CAP_SYS_TIME` (from Linux)",
    CAP_SYS_TTY_CONFIG: "`CAP_SYS_TTY_CONFIG` (from Linux)",
    CAP_MKNOD: "`CAP_SYS_MKNOD` (from Linux, >= 2.4)",
    CAP_LEASE: "`CAP_LEASE` (from Linux, >= 2.4)",
    CAP_AUDIT_WRITE,
    CAP_AUDIT_CONTROL: "`CAP_AUDIT_CONTROL` (from Linux, >= 2.6.11)",
    CAP_SETFCAP,
    CAP_MAC_OVERRIDE,
    CAP_MAC_ADMIN,
    CAP_SYSLOG: "`CAP_SYSLOG` (from Linux, >= 2.6.37)",
    CAP_WAKE_ALARM: "`CAP_WAKE_ALARM` (from Linux, >= 3.0)",
    CAP_BLOCK_SUSPEND,
    CAP_AUDIT_READ: "`CAP_AUDIT_READ` (from Linux, >= 3.16).",
    CAP_PERFMON: "`CAP_PERFMON` (from Linux, >= 5.8).",
    CAP_BPF: "`CAP_BPF` (from Linux, >= 5.8).",
    CAP_CHECKPOINT_RESTORE: "`CAP_CHECKPOINT_RESTORE` (from Linux, >= 5.9)."
);

impl Capability {
    /// Returns the bitmask corresponding to this capability value.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn bitmask(&self) -> u64 {
        1u64 << (*self as u8)
    }

    /// Returns the index of this capability, i.e. its kernel-defined value.
    #[allow(clippy::trivially_copy_pass_by_ref)]
    pub fn index(&self) -> u8 {
        *self as u8
    }
}

// Conversions between Capability <-> CapsBitFlags <-> CapsHashSet
impl From<Capability> for CapsBitFlags {
    fn from(value: Capability) -> Self {
        Self::from_bits_retain(value.bitmask())
    }
}
impl FromIterator<Capability> for CapsBitFlags {
    fn from_iter<T: IntoIterator<Item = Capability>>(iter: T) -> Self {
        Self::from_bits_retain(iter.into_iter().map(|c| c.bitmask()).sum())
    }
}
impl From<&CapsBitFlags> for CapsHashSet {
    fn from(value: &CapsBitFlags) -> Self {
        value
            .iter_names()
            .map(|(_, f)| f.try_into().expect("invalid capability"))
            .collect()
    }
}
impl From<CapsBitFlags> for CapsHashSet {
    fn from(value: CapsBitFlags) -> Self {
        From::<&CapsBitFlags>::from(&value)
    }
}
impl From<&CapsHashSet> for CapsBitFlags {
    fn from(value: &CapsHashSet) -> Self {
        value.iter().copied().collect()
    }
}
impl From<CapsHashSet> for CapsBitFlags {
    fn from(value: CapsHashSet) -> Self {
        value.into_iter().collect()
    }
}

/// A collection capable of storing a set of [`Capability`].
///
/// Both [`CapsBitFlags`] and [`CapsHashSet`] implements this trait, using different inner containers.
pub trait CapsList: FromIterator<Capability> + Clone {
    type Iter<'a>: Iterator<Item = Capability>
    where
        Self: 'a;
    /// An iterator visiting all capabilities in the set.
    fn iter_caps(&self) -> Self::Iter<'_>;
    /// Returns `true` if the collection contains specified capability.
    fn contains_cap(&self, value: &Capability) -> bool;
    /// Insert a capability into the collection, returning `false` if already contained.
    fn insert_cap(&mut self, value: Capability) -> bool;
    /// Create a collection with no capabilities.
    fn empty() -> Self;
    /// Remove a capability from the collection, returning `true` if the value was present.
    fn remove_cap(&mut self, value: &Capability) -> bool;
}

impl CapsList for CapsHashSet {
    type Iter<'a> = std::iter::Copied<std::collections::hash_set::Iter<'a, Capability>>;

    fn iter_caps(&self) -> Self::Iter<'_> {
        self.iter().copied()
    }

    fn contains_cap(&self, value: &Capability) -> bool {
        self.contains(value)
    }

    fn insert_cap(&mut self, value: Capability) -> bool {
        self.insert(value)
    }

    fn empty() -> Self {
        HashSet::new()
    }

    fn remove_cap(&mut self, value: &Capability) -> bool {
        self.remove(value)
    }
}
pub struct CapsBitFlagsIterator<'a>(bitflags::iter::IterNames<CapsBitFlags>, PhantomData<&'a ()>);
impl Iterator for CapsBitFlagsIterator<'_> {
    type Item = Capability;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|(_, f)| f.try_into().expect("invalid capability"))
    }
}
impl CapsList for CapsBitFlags {
    type Iter<'a> = CapsBitFlagsIterator<'a>;

    fn iter_caps(&self) -> Self::Iter<'_> {
        CapsBitFlagsIterator(self.iter_names(), PhantomData)
    }

    fn contains_cap(&self, value: &Capability) -> bool {
        self.contains(CapsBitFlags::from(*value))
    }

    fn insert_cap(&mut self, value: Capability) -> bool {
        if self.contains_cap(&value) {
            false
        } else {
            self.insert(CapsBitFlags::from(value));
            true
        }
    }

    fn empty() -> Self {
        Self::empty()
    }

    fn remove_cap(&mut self, value: &Capability) -> bool {
        if self.contains_cap(value) {
            self.remove(CapsBitFlags::from(*value));
            true
        } else {
            false
        }
    }
}
