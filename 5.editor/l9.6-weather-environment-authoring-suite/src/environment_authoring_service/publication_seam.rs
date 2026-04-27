use crate::{EditorEvent, EventBus};

pub fn emit(event_bus: &dyn EventBus, event: EditorEvent) {
    event_bus.emit(event);
}
