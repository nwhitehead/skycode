use std::io;

use crossterm::event::{read, Event, KeyEvent, KeyCode};
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen)?;

    loop {
        // Blocks until an `Event` is available
        let inp = read()?;
        println!("{:?}\r", inp);
        match inp {
            Event::Key(KeyEvent { code: KeyCode::Esc, ..}) => {
                break;
            }
            _ => {}
        }
    }
    crossterm::execute!(stdout, LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}
