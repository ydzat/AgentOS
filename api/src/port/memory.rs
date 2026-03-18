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
    fn map(&mut self, virt: u64, phys: u64, flags: u64) -> Result<(), ()>;
    /// Unmap a virtual address.
    fn unmap(&mut self, virt: u64) -> Result<(), ()>;
}
