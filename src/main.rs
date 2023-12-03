use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode};
use termion::event::Key;

mod event;
mod edit;
use crate::event::*;
use crate::edit::edit_init;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}{}", clear::All, cursor::BlinkingBlock)?;
    stdout.flush()?;

    let (mut state, mut data) = edit_init();

    let rs = event_init(stdin());
    loop {
        let key_opt = event_wait(&rs);
        if let Some(Key::Ctrl('c')) = key_opt {
            break;
        }
        let next_state_opt = state.update(&mut data, key_opt);
        let moji = state.draw(&mut data);
        write!(stdout, "{}", moji)?;
        stdout.flush()?;

        if let Some(next_state) = next_state_opt {
            state.finalize(&mut data);
            state = next_state;
            state.initialize(&mut data);
        }

        if state.is_terminal() {
            break;
        }
    }

    Ok(())
}
