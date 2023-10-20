use crate::callbacks::{
	types::{EventState, EventType},
	CallbackInterface,
};

struct MockCallback;

impl CallbackInterface for MockCallback {
	fn handle_event(&self, event_type: EventType, event_data: EventState) {
		// Perform your test assertions here
		// Check if the event_type and event_data match expected values
		assert_eq!(event_type, EventType::StateTransition);
		assert_eq!(event_data.payload, "TestPayload");
	}
}

#[test]
fn test_callback_interface() {
	let mock_callback = MockCallback;
	mock_callback.handle_event(
		EventType::StateTransition,
		EventState {
			event_type: EventType::StateTransition,
			timestamp: 123.45,
			payload: "TestPayload".to_string(),
		},
	);
}
