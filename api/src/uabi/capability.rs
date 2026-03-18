//! Capability types for the security model.

/// System capabilities that can be granted to Agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Capability {
    /// Create a child Agent.
    AgentCreate = 0,
    /// Destroy an Agent.
    AgentDestroy = 1,
    /// Suspend an Agent.
    AgentSuspend = 2,

    /// Send IPC messages.
    IpcSend = 8,
    /// Receive IPC messages.
    IpcRecv = 9,
    /// Broadcast messages to multiple Agents.
    IpcBroadcast = 10,

    /// Allocate memory.
    MemAlloc = 16,
    /// Map shared memory regions.
    MemMap = 17,

    /// Access devices.
    DeviceAccess = 24,
    /// Device control (ioctl).
    DeviceControl = 25,

    /// Use local inference (GPU/NPU).
    InferenceLocal = 32,
    /// Use remote inference (cloud API).
    InferenceRemote = 33,

    /// Store Agent memory.
    MemoryStore = 40,
    /// Recall Agent memory.
    MemoryRecall = 41,
    /// Delete Agent memory.
    MemoryForget = 42,

    /// Grant capabilities to other Agents.
    CapGrant = 56,
    /// Revoke capabilities from other Agents.
    CapRevoke = 57,
    /// Safety Agent override (highest privilege).
    SafetyOverride = 63,
    // 64-127: reserved for AgentPack custom capabilities
}
