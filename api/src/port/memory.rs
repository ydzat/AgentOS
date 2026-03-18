//! Memory management port traits.

/// Physical frame allocator interface.
pub trait FrameAllocator {
    /// Allocate a single physical frame. Returns the physical address.
    fn alloc_frame(&mut self) -> Option<u64>;
    /// Deallocate a physical frame.
    fn dealloc_frame(&mut self, addr: u64);
}

/// Page table mapper interface.
pub trait PageMapper {
    /// Map a virtual address to a physical address.
    ///
    /// # Errors
    ///
    /// Returns an error if the mapping fails.
    fn map(&mut self, virt: u64, phys: u64, flags: u64) -> Result<(), MapError>;
    /// Unmap a virtual address.
    ///
    /// # Errors
    ///
    /// Returns an error if the address is not mapped.
    fn unmap(&mut self, virt: u64) -> Result<(), MapError>;
}

/// Error type for page mapping operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    /// The frame allocator is out of memory.
    OutOfMemory,
    /// The virtual address is already mapped.
    AlreadyMapped,
    /// The virtual address is not mapped.
    NotMapped,
}
