use crate::state::AppState;
use crossterm::event::{KeyCode, KeyEvent};

pub fn process_event(state: &mut AppState, event: KeyEvent) {
    match event {
        KeyEvent {
            code: KeyCode::Left,
            ..
        } => state.menu_next(),
        KeyEvent {
            code: KeyCode::Right,
            ..
        } => state.menu_prev(),
        // _ => (),
        _ => {
            println!("{:?}", event);
        }
    }
}
