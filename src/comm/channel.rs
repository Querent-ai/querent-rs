// Import necessary items from the callbacks module
use crate::comm::types::message::{MessageState, MessageType};
// Import necessary items from the pyo3 crate
use pyo3::prelude::*;

// Define the base interface for event callbacks
pub trait ChannelInterface {
	/// Receive a message in python from rust
	fn receive_in_python(&mut self) -> Option<MessageState>;

	/// Receive a message in rust from python
	fn receive_in_rust(&mut self) -> Option<MessageState>;

	/// Send a message in python from rust
	fn send_in_python(&mut self, message_type: MessageType, message_data: MessageState);

	/// Send a message in rust from python
	fn send_in_rust(&mut self, message_type: MessageType, message_data: MessageState);
}

// Define a basic event handler struct
#[derive(Clone, Debug)]
pub struct ChannelHandler {}

impl ChannelHandler {
	// Constructor for EventHandler
	pub fn new() -> Self {
		ChannelHandler {}
	}
}

// Define a Python-compatible message
#[derive(Clone, Debug)]
#[pyclass]
pub struct PyMessageInterface {
	// Internal channel handler instance
	channel_handler: ChannelHandler,
}

// Implement Python methods for PyEventCallbackInterface
#[pymethods]
impl PyMessageInterface {
	// Python constructor for PyMessageInterface
	#[new]
	pub fn new() -> Self {
		PyMessageInterface { channel_handler: ChannelHandler::new() }
	}

	// Python method to handle messages
	pub fn receive_in_python(&mut self) -> Option<MessageState> {
		// Delegate the event handling to the internal event handler
		self.channel_handler.receive_in_python()
	}

	// Python method to handle messages
	pub fn receive_in_rust(&mut self) -> Option<MessageState> {
		// Delegate the event handling to the internal event handler
		self.channel_handler.receive_in_rust()
	}

	// Python method to handle messages
	pub fn send_in_python(&mut self, message_type: MessageType, message_data: MessageState) {
		// Delegate the event handling to the internal event handler
		self.channel_handler.send_in_python(message_type, message_data)
	}

	// Python method to handle messages
	pub fn send_in_rust(&mut self, message_type: MessageType, message_data: MessageState) {
		// Delegate the event handling to the internal event handler
		self.channel_handler.send_in_rust(message_type, message_data)
	}
}

// Implement the ChannelInterface for the ChannelHandler
impl ChannelInterface for ChannelHandler {
	// Implementation of the handle_event method for EventHandler
	fn receive_in_python(&mut self) -> Option<MessageState> {
		// Print basic information about the event (TODO: Handle different event types)
		println!("TODO: handle different event types: Event coming from rust");
		None
	}

	// Implementation of the handle_event method for EventHandler
	fn receive_in_rust(&mut self) -> Option<MessageState> {
		// Print basic information about the event (TODO: Handle different event types)
		println!("TODO: handle different event types: Event coming from rust");
		None
	}

	// Implementation of the handle_event method for EventHandler
	fn send_in_python(&mut self, message_type: MessageType, message_data: MessageState) {
		// Print basic information about the event (TODO: Handle different event types)
		println!("TODO: handle different event types: Event coming from rust");
		println!("Message type: {:?}", message_type);
		println!("Message data: {:?}", message_data);
	}

	// Implementation of the handle_event method for EventHandler
	fn send_in_rust(&mut self, message_type: MessageType, message_data: MessageState) {
		// Print basic information about the event (TODO: Handle different event types)
		println!("TODO: handle different event types: Event coming from rust");
		println!("Message type: {:?}", message_type);
		println!("Message data: {:?}", message_data);
	}
}
