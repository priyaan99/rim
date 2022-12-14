use core::time::Duration;
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::{event, terminal};

struct CleanUp;

impl Drop for CleanUp {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("could not enable raw mode");
    }
}

fn main() -> crossterm::Result<()> {
    let _clean_up = CleanUp;

    terminal::enable_raw_mode().expect("could not enable raw mode");

    loop {
        if event::poll(Duration::from_millis(500))? {
            if let Event::Key(event) = event::read()? {
                match event {
                    KeyEvent {
                        code: KeyCode::Char('q'),
                        modifiers: event::KeyModifiers::NONE,
                        kind: _,
                        state: _,
                    } => break,
                    _ => {
                        //todo
                    }
                }
                println!("{:?}\r", event);
            };
        } else {
            println!("No input yet\r");
        }
    }

    Ok(())
}
