//! Agent scheduler (Strategy Pattern).
//!
//! The scheduler trait allows pluggable scheduling algorithms.

pub mod round_robin;
pub mod priority;

/// Scheduler trait — implement this for custom scheduling strategies.
pub trait Scheduler {
    /// Select the next Agent to run.
    fn next(&mut self) -> Option<aos_api::uabi::agent::AgentId>;
    /// Notify the scheduler that an Agent has been created.
    fn add(&mut self, id: aos_api::uabi::agent::AgentId);
    /// Notify the scheduler that an Agent has been destroyed.
    fn remove(&mut self, id: aos_api::uabi::agent::AgentId);
}
