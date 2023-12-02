use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode};
use termion::event::Key;

mod event;
mod edit;
use crate::event::*;
use crate::edit::*;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}{}", clear::All, cursor::BlinkingBlock)?;
    stdout.flush()?;

    let mut data: EditData = EditData::new();

    let rs = event_init(stdin());
    loop {
        let key_opt = event_wait(&rs);
        if let Some(key) = key_opt {
            if key == Key::Ctrl('c') {
                break;
            }
        }
        let next_state_opt = data.update(key_opt);
        let mut moji = data.state.draw(&data.disp);
        if let Some(next_state) = next_state_opt {
            moji.push_str(&data.handle_state_change(next_state));
        }
        write!(stdout, "{}", moji)?;
        stdout.flush()?;
    }

    Ok(())
}
