//! AOS Kernel — Core logic
//!
//! This crate contains all kernel-side logic and is written in pure safe Rust.
//! It depends only on `aos-api` for port traits and uabi types, never on `aos-hal`.

#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::perf, clippy::style, clippy::complexity)]
#![warn(missing_docs)]

pub mod agent;
pub mod budget;
pub mod capability;
pub mod event;
pub mod fs;
pub mod human;
pub mod ipc;
pub mod memory;
pub mod syscall;
