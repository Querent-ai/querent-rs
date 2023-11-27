// Import necessary items from the callbacks module
use crate::comm::types::message::{MessageState, MessageType};
// Import necessary items from the pyo3 crate
use pyo3::prelude::*;

// Define the base interface for event callbacks
pub trait ChannelInterface {
	fn receive_message(&mut self, message_type: MessageType, message_data: MessageState);
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

	// Python method to handle events
	fn handle_event(&mut self, message_type: MessageType, message_data: MessageState) {
		// Delegate the receive message to the internal channel handler
		self.channel_handler.receive_message(message_type, message_data);
	}
}

// Implement the ChannelInterface for the ChannelHandler
impl ChannelInterface for ChannelHandler {
	// Implementation of the receive_message method for ChannelHandler
	fn receive_message(&mut self, message_type: MessageType, message_data: MessageState) {
		// Print basic information about the message (TODO: Handle different message types)
		println!("Message: {:?}, {:?}", message_type, message_data);
		println!("TODO: handle different message types: Message coming from python");
	}
}
