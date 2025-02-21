/// An `HashSet` specialized on `Capability`.
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
                $capability = crate::nr::$capability
            ),*
        }
        impl ::std::fmt::Display for Capability {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                let name = match *self {
                    $(
                        Self::$capability => stringify!($capability)
                    ),*
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
        pub fn all() -> CapsHashSet {
            ::std::iter::FromIterator::from_iter([$(Capability::$capability),*])
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
