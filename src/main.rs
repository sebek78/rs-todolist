mod events;
mod renderer;
mod state;
use crate::events::process_event;
use crate::renderer::render;
use crate::state::AppState;

use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn print_events() -> crossterm::Result<()> {
    let mut state = AppState::new();
    render(&state);

    loop {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            Event::Key(event) => {
                process_event(&mut state, event);
            }
            _ => break,
        }
        render(&state);
    }
    Ok(())
}

fn main() {
    enable_raw_mode().unwrap();

    print_events().unwrap();

    disable_raw_mode().unwrap();
}
