//! AOS error codes.

/// System error codes returned by syscalls.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AosError {
    /// Operation succeeded.
    Ok = 0,
    /// `AgentId` does not exist.
    InvalidAgent = -1,
    /// Capability check failed.
    PermissionDenied = -2,
    /// Resource budget exhausted.
    BudgetExhausted = -3,
    /// Invalid argument.
    InvalidArgument = -4,
    /// Non-blocking call but no data available.
    WouldBlock = -5,
    /// Out of memory.
    NoMemory = -6,
    /// IPC queue is full.
    QueueFull = -7,
    /// Resource not found.
    NotFound = -8,
}
