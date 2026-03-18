//! Architecture-specific implementations.

#[cfg(feature = "arch-x86_64")]
pub mod x86_64;

#[cfg(feature = "arch-aarch64")]
pub mod aarch64;
