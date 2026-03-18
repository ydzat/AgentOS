//! Identity and delegation chain types.

/// Unique identifier for a human user (owner).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct UserId(pub u64);
