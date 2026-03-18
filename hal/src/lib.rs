//! AOS HAL — Hardware Abstraction Layer
//!
//! All unsafe code is confined to this crate.
//! Implements the port traits defined in `aos-api`.

#![no_std]

pub mod arch;
pub mod mm;
pub mod sync;
pub mod console;
