//! AOS Init — Boot entry point
//!
//! This binary:
//! 1. Receives control from the bootloader
//! 2. Initializes hal (hardware abstraction)
//! 3. Injects hal implementations into kernel via generics (DIP assembly)
//! 4. Starts the first Agent (init-agent)

#![no_std]
#![no_main]
#![deny(warnings)]
#![deny(clippy::correctness, clippy::suspicious)]
#![warn(clippy::perf, clippy::style, clippy::complexity, clippy::pedantic)]
#![warn(missing_docs)]

// TODO: UEFI boot → hal init → kernel init → first Agent
