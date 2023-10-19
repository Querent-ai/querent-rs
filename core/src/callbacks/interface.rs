// Base interface for callbacks
use crate::callbacks::types::{EventState, EventType};
pub trait CallbackInterface {
	fn handle_event(&self, event_type: EventType, event_data: EventState);
}
