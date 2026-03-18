//! Accelerator (GPU/NPU) resource management port trait.
//!
//! The kernel uses this trait to manage accelerator resources (memory, scheduling,
//! isolation) without knowing anything about the actual computation (CUDA, Vulkan,
//! tilelang, etc.). Computation happens in userspace inference services.

/// Handle to an accelerator memory allocation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccelMemHandle(pub u64);

/// Accelerator work queue identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueueId(pub u32);

/// Accelerator device status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccelStatus {
    /// Device is ready and idle.
    Ready,
    /// Device is currently processing work.
    Busy,
    /// Device has encountered an error.
    Error,
    /// Device is not present.
    NotPresent,
}

/// Error type for accelerator operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccelError {
    /// Out of device memory.
    OutOfMemory,
    /// Work queue is full.
    QueueFull,
    /// Device error.
    DeviceError,
    /// Invalid handle.
    InvalidHandle,
}

/// Accelerator resource management interface.
///
/// This trait handles resource allocation and scheduling only.
/// The kernel does not interpret work payloads — actual computation
/// (CUDA kernels, shader dispatch, etc.) is defined by userspace.
pub trait Accelerator {
    /// Allocate memory on the accelerator.
    ///
    /// # Errors
    ///
    /// Returns [`AccelError::OutOfMemory`] if allocation fails.
    fn alloc_mem(&mut self, size: usize) -> Result<AccelMemHandle, AccelError>;

    /// Free previously allocated accelerator memory.
    ///
    /// # Errors
    ///
    /// Returns [`AccelError::InvalidHandle`] if the handle is not valid.
    fn free_mem(&mut self, handle: AccelMemHandle) -> Result<(), AccelError>;

    /// Submit an opaque work payload to the accelerator queue.
    ///
    /// The kernel does not parse the payload — it is passed directly
    /// to the accelerator hardware. Format is defined by the userspace
    /// inference service and the specific accelerator driver.
    ///
    /// # Errors
    ///
    /// Returns [`AccelError::QueueFull`] if the queue cannot accept more work.
    fn submit(&mut self, queue: QueueId, work: &[u8]) -> Result<(), AccelError>;

    /// Query the accelerator device status.
    fn status(&self) -> AccelStatus;

    /// Get total device memory in bytes.
    fn total_mem(&self) -> usize;

    /// Get currently available device memory in bytes.
    fn available_mem(&self) -> usize;
}
