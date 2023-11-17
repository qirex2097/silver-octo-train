use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode, raw::RawTerminal};
use termion::event::{Event, Key};
use termion::input::TermRead;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    let disp_arr: [[char; 19]; 19] = [
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
        [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
    ];

    let mut cursor_x = 1;
    let mut cursor_y = 1;

    write!(stdout, "{}", clear::All)?;
    draw(&mut stdout, &disp_arr)?;
    write!(stdout, "{}", cursor::Goto(cursor_x + 1, cursor_y + 1))?;
    stdout.flush()?;

    let stdin = stdin();
    for event in stdin.events() {
        write!(stdout, "{}", clear::All)?;
        let event = event?;
        match event {
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
                break;
            }
            Event::Key(Key::Char('h')) => {
                cursor_x = std::cmp::max(cursor_x - 1, 1);
            }
            Event::Key(Key::Char('l')) => {
                cursor_x = std::cmp::min(cursor_x + 1, 18 - 1);
            }
            Event::Key(Key::Char('k')) => {
                cursor_y = std::cmp::max(cursor_y - 1, 1);
            }
            Event::Key(Key::Char('j')) => {
                cursor_y = std::cmp::min(cursor_y + 1, 18 - 1);
            }
            Event::Key(Key::Char('H')) => {
                cursor_x = std::cmp::max(((cursor_x / 2) * 2 + 1).saturating_sub(2), 1);
            }
            Event::Key(Key::Char('L')) => {
                cursor_x = std::cmp::min((cursor_x / 2) * 2 + 1 + 2, 18 - 1);
            }
            Event::Key(Key::Char('K')) => {
                cursor_y = std::cmp::max(((cursor_y / 2) * 2 + 1).saturating_sub(2), 1);
            }
            Event::Key(Key::Char('J')) => {
                cursor_y = std::cmp::min((cursor_y / 2) * 2 + 1 + 2, 18 - 1);
            }
            _ => {}
        }
        draw(&mut stdout, &disp_arr)?;
        write!(stdout, "{}", cursor::Goto(cursor_x + 1, cursor_y + 1))?;
        stdout.flush()?;
    }

    Ok(())
}

fn draw<T: std::io::Write>(stdout: &mut RawTerminal<T>, disp_arr: &[[char; 19]; 19]) -> Result<(), std::io::Error> {
    for (y, line) in disp_arr.iter().enumerate() {
        let line: String = line.iter().collect();
        write!(stdout, "{}{}", cursor::Goto(1, y as u16 + 1), line)?;
    }
    Ok(())
}