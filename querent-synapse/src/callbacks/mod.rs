// ! Types
//
// This module contains definitions for types used in the callbacks system.
// It includes structures and enumerations crucial for handling events and
// interacting with the callback system.
pub mod types;
pub use types::*;

// ! Callbacks
//
// This module defines the callback interface and a Python-compatible
// implementation for handling events in the querent execution. Users
// can leverage these components to customize and extend the querent's
// behavior by responding to specific events during execution.
pub mod interface;
pub use interface::{EventCallbackInterface, PyEventCallbackInterface};
