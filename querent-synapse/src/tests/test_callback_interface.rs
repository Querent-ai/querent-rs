use crate::{
	base_python_interpreter,
	callbacks::{
		types::{EventState, EventType},
		EventCallbackInterface,
	},
	python_interpreter, INTERPRETER,
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

#[test]
fn test_python_setup() {
	let res = base_python_interpreter();
	assert!(res.is_ok());
}

// python_interpreter
#[test]
fn test_python_interpreter() {
	let res = python_interpreter();
	assert!(res.is_ok());

	// Check if the Python interpreter is initialized
	unsafe {
		assert!(INTERPRETER.is_some());
	}
}

#[test]
fn test_simple_python_interpreter() {
	let res = python_interpreter();
	assert!(res.is_ok());
	let interpreter =
		unsafe { INTERPRETER.as_ref().expect("Python interpreter NOT initialized!!!") };
	// `py` is a `pyo3::Python` instance.
	interpreter.with_gil(|py| {
		py.run("print('hello, Querent')", None, None).unwrap();
	});
}
