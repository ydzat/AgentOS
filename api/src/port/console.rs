//! Console output port trait.

/// Console output interface (serial port, framebuffer, etc.).
pub trait Console {
    /// Write a string to the console.
    fn write_str(&mut self, s: &str);
}
