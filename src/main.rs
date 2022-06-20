use crossterm::{
    event::{
        read, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers,
    },
    execute,
};

fn print_events() -> crossterm::Result<()> {
    loop {
        match read()? {
            Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
            }) => break,
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
    Ok(())
}

fn main() {
    let mut stdout = std::io::stdout();
    execute!(stdout, EnableMouseCapture);

    print_events().unwrap();

    execute!(stdout, DisableMouseCapture);
}
