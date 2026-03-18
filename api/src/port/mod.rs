//! Port traits — interfaces the kernel depends on (DIP).
//!
//! These traits define what the kernel needs from hardware.
//! Concrete implementations live in the `hal` crate.

pub mod memory;
pub mod interrupt;
pub mod timer;
pub mod console;
