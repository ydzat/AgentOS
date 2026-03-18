//! AOS Kernel — Core logic
//!
//! This crate contains all kernel-side logic and is written in pure safe Rust.
//! It depends only on `aos-api` for port traits and uabi types, never on `aos-hal`.

#![no_std]
#![forbid(unsafe_code)]

pub mod agent;
pub mod ipc;
pub mod capability;
pub mod budget;
pub mod memory;
pub mod human;
pub mod fs;
pub mod event;
pub mod syscall;
