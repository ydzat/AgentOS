//! AOS API — Shared interface layer
//!
//! This crate defines:
//! - `uabi`: User ABI types shared between kernel and userspace (syscall contract)
//! - `port`: Port traits that the kernel depends on (DIP interfaces for hardware abstraction)

#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::perf, clippy::style, clippy::complexity)]
#![warn(missing_docs)]

pub mod port;
pub mod uabi;
