use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode};
use termion::event::Key;

mod event;
mod edit;
use crate::event::*;
use crate::edit::*;
use silver_octo_train::*;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}{}", clear::All, cursor::BlinkingBlock)?;
    stdout.flush()?;

    let mut data: EditData = EditData { disp: DispField::new(), state: Box::new(EditStateInit::new()) };

    let rs = event_init(stdin());
    loop {
        let key_opt = event_wait(&rs);
        if let Some(key) = key_opt {
            if key == Key::Ctrl('c') {
                break;
            }
        }
        let next_state_opt = data.state.update(&mut data.disp, key_opt);
        let moji = data.state.draw(&data.disp);
        if let Some(next_state) = next_state_opt {
            data.state.finalize();
            data.state = next_state;
            data.state.initialize(&mut data.disp);
        }
        write!(stdout, "{}", moji)?;
        stdout.flush()?;
    }

    Ok(())
}
