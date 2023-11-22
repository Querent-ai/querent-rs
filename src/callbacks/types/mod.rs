/// Callback Types
///
/// This module defines the event types used by the callback system in the querent execution.
/// The callback system is employed to notify the user of various events that occur during the
/// execution of the querent, providing a way to react to and handle these events in a customized manner.
///
/// # Module Overview
///
/// The module includes definitions for different callback event types, allowing users to subscribe
/// and respond to specific events of interest. The event types are crucial for understanding and
/// responding to the querent's execution lifecycle.
///
/// # Usage
///
/// To utilize the callback system, users can import the module and use the provided event types
/// to define their custom callbacks. These callbacks can then be registered with the querent to
/// receive notifications about specific events.
///
/// ```rust
/// use your_crate::event::EventType;
/// use your_crate::Querent;
///
/// fn main() {
///     // Create a querent instance
///     let querent = Querent::new();
///
///     // Define a callback for the 'ChatCompleted' event
///     querent.register_callback(EventType::ChatCompleted, |event| {
///         // Custom logic to handle the 'ChatCompleted' event
///         println!("Chat completed! Event details: {:?}", event);
///     });
///
///     // Execute the querent, triggering events and invoking registered callbacks
///     querent.execute();
/// }
/// ```
///
/// # Event Types
///
/// The module exports several event types, such as `ContextualGraphUpdated`,
/// `SemanticGraphUpdated`, and `ChatCompleted`. Users can match on these event types to
/// implement specific behavior based on the type of event received.
///
/// # Examples
///
/// ```rust
/// use your_crate::event::EventType;
/// use your_crate::Querent;
///
/// fn main() {
///     // Create a querent instance
///     let querent = Querent::new();
///
///     // Define a callback for the 'SemanticGraphUpdated' event
///     querent.register_callback(EventType::SemanticGraphUpdated, |event| {
///         // Custom logic to handle the 'SemanticGraphUpdated' event
///         println!("Semantic graph updated! Event details: {:?}", event);
///     });
///
///     // Execute the querent, triggering events and invoking registered callbacks
///     querent.execute();
/// }
/// ```
///
pub mod event;
pub use event::*;
