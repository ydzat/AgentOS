//! Human Interface — kernel-level component ensuring human authority.
//!
//! This is NOT an Agent. It is part of the kernel, independent of the Agent subsystem.
//! Even if all Agents crash, the Human Interface remains operational.

pub mod interface;
pub mod delegation;
pub mod audit;
pub mod killswitch;
