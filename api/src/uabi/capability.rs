//! Capability types for the security model.

/// System capabilities that can be granted to Agents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Capability {
    // Agent management
    AgentCreate = 0,
    AgentDestroy = 1,
    AgentSuspend = 2,

    // IPC
    IpcSend = 8,
    IpcRecv = 9,
    IpcBroadcast = 10,

    // Memory
    MemAlloc = 16,
    MemMap = 17,

    // Device
    DeviceAccess = 24,
    DeviceControl = 25,

    // Inference
    InferenceLocal = 32,
    InferenceRemote = 33,

    // Memory filesystem
    MemoryStore = 40,
    MemoryRecall = 41,
    MemoryForget = 42,

    // Privilege
    CapGrant = 56,
    CapRevoke = 57,
    SafetyOverride = 63,

    // 64-127: reserved for AgentPack custom capabilities
}
