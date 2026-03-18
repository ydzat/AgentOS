//! Interrupt controller port trait.

/// Interrupt controller interface.
pub trait InterruptController {
    /// Enable an interrupt line.
    fn enable(&mut self, irq: u32);
    /// Disable an interrupt line.
    fn disable(&mut self, irq: u32);
    /// Acknowledge an interrupt (end of interrupt).
    fn acknowledge(&mut self, irq: u32);
}
