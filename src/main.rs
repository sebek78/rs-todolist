use crossterm::{
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{disable_raw_mode, enable_raw_mode},
};

fn process_event(mut state: u32, event: KeyEvent) -> u32 {
    println!("{:?}", event);
    state += 1;
    state
}

fn render(state: u32) {
    println!("{}", state);
}

fn print_events() -> crossterm::Result<()> {
    let mut state: u32 = 0;
    render(state);

    loop {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            Event::Key(event) => {
                state = process_event(state, event);
            }
            _ => break,
        }

        render(state);
    }
    Ok(())
}

fn main() {
    enable_raw_mode().unwrap();

    print_events().unwrap();

    disable_raw_mode().unwrap();
}
