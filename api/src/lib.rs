//! AOS API — Shared interface layer
//!
//! This crate defines:
//! - `uabi`: User ABI types shared between kernel and userspace (syscall contract)
//! - `port`: Port traits that the kernel depends on (DIP interfaces for hardware abstraction)

#![no_std]
#![forbid(unsafe_code)]

pub mod uabi;
pub mod port;
