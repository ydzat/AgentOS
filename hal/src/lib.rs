//! AOS HAL — Hardware Abstraction Layer
//!
//! All unsafe code is confined to this crate.
//! Implements the port traits defined in `aos-api`.

#![no_std]
#![deny(warnings)]
#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::perf, clippy::style, clippy::complexity)]
#![warn(missing_docs)]

pub mod arch;
pub mod console;
pub mod mm;
pub mod sync;
