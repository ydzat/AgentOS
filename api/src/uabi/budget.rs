//! Resource budget types.

/// Resource budget for an Agent, enforced by the kernel.
#[derive(Debug, Clone)]
#[repr(C)]
pub struct ResourceBudget {
    /// Remaining inference token budget.
    pub token_remaining: u64,
    /// Memory usage limit (bytes).
    pub memory_limit: usize,
    /// Current memory usage (bytes).
    pub memory_used: usize,
    /// CPU time budget (nanoseconds).
    pub cpu_time_limit: u64,
    /// CPU time used (nanoseconds).
    pub cpu_time_used: u64,
    /// Accelerator (GPU/NPU) memory limit (bytes).
    pub accel_mem_limit: usize,
    /// Accelerator memory currently used (bytes).
    pub accel_mem_used: usize,
    /// Accelerator time budget (nanoseconds).
    pub accel_time_limit: u64,
    /// Accelerator time used (nanoseconds).
    pub accel_time_used: u64,
}
