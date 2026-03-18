//! Timer port trait.

/// Hardware timer interface.
pub trait Timer {
    /// Get the current time in nanoseconds since boot.
    fn now_ns(&self) -> u64;
    /// Set a one-shot timer to fire after `ns` nanoseconds.
    fn set_oneshot(&mut self, ns: u64);
}
