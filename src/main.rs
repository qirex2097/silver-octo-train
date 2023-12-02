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
    let mut state: Box<dyn EditState> = edit_init();

    let rs = event_init(stdin());
    loop {
        let key_opt = event_wait(&rs);
        if let Some(Key::Ctrl('c')) = key_opt {
            break;
        }
        let next_state_opt = state.update(&mut data.disp, key_opt);
        let mut moji = state.draw(&data.disp);
        if let Some(next_state) = next_state_opt {
            moji.push_str(&state.finalize());
            state = next_state;
            moji.push_str(&state.initialize(&mut data.disp));
        }
        write!(stdout, "{}", moji)?;
        stdout.flush()?;
    }

    Ok(())
}
