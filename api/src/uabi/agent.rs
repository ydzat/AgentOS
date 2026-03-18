//! Agent-related ABI types.

/// Unique identifier for an Agent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct AgentId(pub u64);
