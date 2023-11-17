use std::io::{stdin, stdout, Write};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode};
use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}", "Hello")?;
    stdout.flush()?;

    let stdin = stdin();
    for event in stdin.events() {
        if event? == Event::Key(Key::Ctrl('c')) {
            return Ok(());
        }
    }

    Ok(())
}