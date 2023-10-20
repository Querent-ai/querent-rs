// Base interface for callbacks
use crate::callbacks::types::{EventState, EventType};
pub trait EventCallbackInterface {
	fn handle_event(&mut self, event_type: EventType, event_data: EventState);
}
