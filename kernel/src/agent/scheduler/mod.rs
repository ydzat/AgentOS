//! Agent scheduler (Strategy Pattern).
//!
//! The scheduler trait allows pluggable scheduling algorithms.

pub mod priority;
pub mod round_robin;

use aos_api::uabi::agent::AgentId;

/// Scheduler trait — implement this for custom scheduling strategies.
pub trait Scheduler {
    /// Select the next Agent to run.
    fn next(&mut self) -> Option<AgentId>;
    /// Notify the scheduler that an Agent has been created.
    fn add(&mut self, id: AgentId);
    /// Notify the scheduler that an Agent has been destroyed.
    fn remove(&mut self, id: AgentId);
}
