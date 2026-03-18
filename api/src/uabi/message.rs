//! IPC message types.

use crate::uabi::agent::AgentId;

/// IPC message type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MessageType {
    /// Request another Agent to perform a task.
    Request = 0,
    /// Response to a previous request.
    Response = 1,
    /// One-way notification.
    Notify = 2,
    /// System control instruction.
    Control = 3,
}

/// Fixed-size IPC message header (kernel parses this).
#[derive(Debug, Clone)]
#[repr(C)]
pub struct MessageHeader {
    /// Sender AgentId (filled by kernel, unforgeable).
    pub sender: AgentId,
    /// Receiver AgentId.
    pub receiver: AgentId,
    /// Message type.
    pub msg_type: MessageType,
    /// Request ID for matching request-response pairs.
    pub request_id: u64,
    /// Payload length in bytes.
    pub payload_len: u32,
}
