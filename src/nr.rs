/* from <sys/prctl.h> */

pub const PR_GET_KEEPCAPS: i32 = 7;
pub const PR_SET_KEEPCAPS: i32 = 8;
pub const PR_CAPBSET_READ: i32 = 23;
pub const PR_CAPBSET_DROP: i32 = 24;
pub const PR_CAP_AMBIENT: i32 = 47;
pub const PR_CAP_AMBIENT_IS_SET: i32 = 1;
pub const PR_CAP_AMBIENT_RAISE: i32 = 2;
pub const PR_CAP_AMBIENT_LOWER: i32 = 3;
pub const PR_CAP_AMBIENT_CLEAR_ALL: i32 = 4;

/* from <unistd.h> */

#[cfg(target_arch = "x86")]
pub const CAPGET: i32 = 184;
#[cfg(target_arch = "x86")]
pub const CAPSET: i32 = 185;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub const CAPGET: i64 = 125;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
pub const CAPSET: i64 = 126;

#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub const CAPGET: i32 = 0x40000000 + 125;
#[cfg(all(target_arch = "x86_64", target_pointer_width = "32"))]
pub const CAPSET: i32 = 0x40000000 + 126;

#[cfg(target_arch = "aarch64")]
pub const CAPGET: i64 = 90;
#[cfg(target_arch = "aarch64")]
pub const CAPSET: i64 = 91;

#[cfg(target_arch = "powerpc")]
pub const CAPGET: i32 = 183;
#[cfg(target_arch = "powerpc")]
pub const CAPSET: i32 = 184;

#[cfg(target_arch = "powerpc64")]
pub const CAPGET: i64 = 183;
#[cfg(target_arch = "powerpc64")]
pub const CAPSET: i64 = 184;

#[cfg(target_arch = "mips")]
pub const CAPGET: i32 = 4204;
#[cfg(target_arch = "mips")]
pub const CAPSET: i32 = 4205;

#[cfg(target_arch = "mips64")]
pub const CAPGET: i64 = 5123;
#[cfg(target_arch = "mips64")]
pub const CAPSET: i64 = 5124;

#[cfg(target_arch = "arm")]
pub const CAPGET: i32 = 184;
#[cfg(target_arch = "arm")]
pub const CAPSET: i32 = 185;

#[cfg(target_arch = "s390x")]
pub const CAPGET: i64 = 184;
#[cfg(target_arch = "s390x")]
pub const CAPSET: i64 = 185;

#[cfg(target_arch = "sparc")]
pub const CAPGET: i64 = 21;
#[cfg(target_arch = "sparc")]
pub const CAPSET: i64 = 22;

#[cfg(target_arch = "sparc64")]
pub const CAPGET: i64 = 21;
#[cfg(target_arch = "sparc64")]
pub const CAPSET: i64 = 22;

#[cfg(target_arch = "riscv64")]
pub const CAPGET: i64 = 90;
#[cfg(target_arch = "riscv64")]
pub const CAPSET: i64 = 91;

#[cfg(target_arch = "loongarch64")]
pub const CAPGET: i64 = 90;
#[cfg(target_arch = "loongarch64")]
pub const CAPSET: i64 = 91;
