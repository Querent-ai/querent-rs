import querent_rs.querent_rs.callback_interface as querent_rs

# Set up a mock callback
class MockCallback(querent_rs.CallbackInterface):
    def handle_event(self, event_type, event_data):
        # Perform Python-side assertions here
        assert event_type == "state_transition"
        assert event_data["payload"] == "TestPayload"


# Create an instance of the mock callback
mock_callback = MockCallback()

# Trigger an event
event_data = {
    "event_type": "state_transition",
    "timestamp": 123.45,
    "payload": "TestPayload",
}
mock_callback.handle_event(event_data)
