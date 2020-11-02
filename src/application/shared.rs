use crate::domain::models::shared::key_code::KeyCode;
use kurenai::key_event::KeyEvent;

pub fn key_event_arrow_to_key_code(key_event: &KeyEvent) -> Option<KeyCode> {
    if key_event.arrow_left() {
        Some(KeyCode::ArrowLeft)
    } else if key_event.arrow_up() {
        Some(KeyCode::ArrowUp)
    } else if key_event.arrow_right() {
        Some(KeyCode::ArrowRight)
    } else if key_event.arrow_down() {
        Some(KeyCode::ArrowDown)
    } else {
        None
    }
}
