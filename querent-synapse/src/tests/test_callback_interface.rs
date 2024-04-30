use crate::callbacks::{
	types::{EventState, EventType},
	EventCallbackInterface,
};

struct MockCallback;

impl EventCallbackInterface for MockCallback {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState) {
		// Perform your test assertions here
		// Check if the event_type and event_data match expected values
		assert_eq!(event_type, EventType::Graph);
		assert_eq!(event_data.payload, "TestPayload");
	}
}

#[test]
fn test_callback_interface() {
	let mut mock_callback = MockCallback;
	mock_callback.handle_event(
		EventType::Graph,
		EventState {
			event_type: EventType::Graph,
			timestamp: 123.45,
			payload: "TestPayload".to_string(),
			file: "TestFile".to_string(),
			doc_source: "file://folder".to_string(),
			image_id: Some("123456".to_string()),
		},
	);
}
