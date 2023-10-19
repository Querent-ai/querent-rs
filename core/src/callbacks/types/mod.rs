///! Callback types
/// This module contains the event types used by the callback system.
/// The callback system is used to notify the user of events that occur
/// during the execution of the querent.
///
///
pub mod event_state;
pub use event_state::*;
pub mod event_type;
pub use event_type::*;
