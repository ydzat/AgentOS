//! AOS Init — Boot entry point
//!
//! This binary:
//! 1. Receives control from the bootloader
//! 2. Initializes hal (hardware abstraction)
//! 3. Injects hal implementations into kernel via generics (DIP assembly)
//! 4. Starts the first Agent (init-agent)

#![no_std]
#![no_main]

// TODO: UEFI boot → hal init → kernel init → first Agent
