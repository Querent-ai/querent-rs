use crate::comm::types::message::{MessageState, MessageType};
use pyo3::prelude::*;

use super::IngestedTokens;

// Define the base interface for event callbacks
pub trait ChannelInterface {
	/// Receive tokens in python from rust
	fn receive_tokens_in_python(&mut self) -> Option<IngestedTokens>;
	/// Receive a message in python from rust
	fn receive_in_python(&mut self) -> Option<MessageState>;
	/// Send a message in rust from python
	fn send_in_rust(&mut self, message_type: MessageType, message_data: MessageState);
}

// Define a basic event handler struct
#[derive(Clone, Debug)]
#[pyclass]
pub struct ChannelHandler {
	pub token_receiver: Option<crossbeam_channel::Receiver<IngestedTokens>>,
	pub py_message_sender: Option<crossbeam_channel::Sender<(MessageType, MessageState)>>,
	pub py_message_receiver: Option<crossbeam_channel::Receiver<(MessageType, MessageState)>>,
	pub message_sender: Option<crossbeam_channel::Sender<(MessageType, MessageState)>>,
}

impl ChannelHandler {
	// Constructor for EventHandler
	pub fn new(
		token_receiver: Option<crossbeam_channel::Receiver<IngestedTokens>>,
		py_message_sender: Option<crossbeam_channel::Sender<(MessageType, MessageState)>>,
		py_message_receiver: Option<crossbeam_channel::Receiver<(MessageType, MessageState)>>,
		message_sender: Option<crossbeam_channel::Sender<(MessageType, MessageState)>>,
	) -> Self {
		ChannelHandler { py_message_sender, py_message_receiver, token_receiver, message_sender }
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
	pub fn new(channel: ChannelHandler) -> Self {
		PyMessageInterface { channel_handler: channel }
	}

	// Receive tokens in python from rust
	pub fn receive_tokens_in_python(&mut self) -> Option<IngestedTokens> {
		// Delegate the event handling to the internal event handler
		self.channel_handler.receive_tokens_in_python()
	}

	// Python method to handle messages
	pub fn receive_in_python(&mut self) -> Option<MessageState> {
		// Delegate the event handling to the internal event handler
		self.channel_handler.receive_in_python()
	}
	// Python method to handle messages
	pub fn send_in_rust(&mut self, message_type: MessageType, message_data: MessageState) {
		// Delegate the event handling to the internal event handler
		self.channel_handler.send_in_rust(message_type, message_data)
	}
}

// Implement the ChannelInterface for the ChannelHandler
impl ChannelInterface for ChannelHandler {
	fn receive_tokens_in_python(&mut self) -> Option<IngestedTokens> {
		if self.token_receiver.is_some() {
			let token = self.token_receiver.as_ref();
			match token {
				Some(token) => {
					let token = token.try_recv();
					match token {
						Ok(token) => Some(token),
						Err(_) => None,
					}
				},
				None => None,
			}
		} else {
			None
		}
	}

	// Implementation of the handle_event method for EventHandler
	fn receive_in_python(&mut self) -> Option<MessageState> {
		if self.py_message_receiver.is_some() {
			let message = self.py_message_receiver.as_ref();
			match message {
				Some(message) => {
					let message = message.try_recv();
					match message {
						Ok(message) => Some(message.1),
						Err(_) => None,
					}
				},
				None => None,
			}
		} else {
			None
		}
	}

	// Implementation of the handle_event method for EventHandler
	fn send_in_rust(&mut self, message_type: MessageType, message_data: MessageState) {
		if self.message_sender.is_some() {
			let message = self.message_sender.as_ref();
			match message {
				Some(message) => {
					let message = message.send((message_type, message_data));
					match message {
						Ok(_) => (),
						Err(_) => (),
					}
				},
				None => (),
			}
		} else {
			()
		}
	}
}
