use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode, raw::RawTerminal};
use termion::event::{Event, Key};
use termion::input::TermRead;

use silver_octo_train::DispField;

fn main() -> Result<(), std::io::Error> {
    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    let mut disp: DispField = DispField::new();
    let disp_arr: &mut [[char; 19]; 19] = &mut disp.disp_arr;

    let mut cursor_x: usize = 1;
    let mut cursor_y: usize = 1;

    write!(stdout, "{}{}", clear::All, cursor::BlinkingBlock)?;
    draw(&mut stdout, &disp_arr)?;
    write!(stdout, "{}", cursor::Goto(cursor_x as u16 + 1, cursor_y as u16 + 1))?;
    stdout.flush()?;

    let stdin = stdin();
    for event in stdin.events() {
        let mut cell_x = cursor_x.saturating_sub(1) / 2;
        let mut cell_y = cursor_y.saturating_sub(1) / 2;
        let prev_cursor_x = cursor_x;
        let prev_cursor_y = cursor_y;
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
                cell_x = if cursor_x % 2 == 1 { cell_x } else { cell_x + 1 };
                cursor_x = std::cmp::max(cell_x.saturating_sub(1) * 2 + 1, 1);
                disp_arr[cursor_y][cursor_x + 1] = if cursor_y % 2 == 1 && prev_cursor_x != cursor_x { ' ' } else { disp_arr[cursor_y][cursor_x + 1] };
            }
            Event::Key(Key::Char('L')) => {
                cursor_x = std::cmp::min(cell_x.saturating_add(1) * 2 + 1, 18 - 1);
                disp_arr[cursor_y][cursor_x - 1] = if cursor_y % 2 == 1 && prev_cursor_x != cursor_x { ' ' } else { disp_arr[cursor_y][cursor_x - 1] };
            }
            Event::Key(Key::Char('K')) => {
                cell_y = if cursor_y % 2 == 1 { cell_y } else { cell_y + 1 };
                cursor_y = std::cmp::max(cell_y.saturating_sub(1) * 2 + 1, 1);
                disp_arr[cursor_y + 1][cursor_x] = if cursor_x % 2 == 1 && prev_cursor_y != cursor_y { ' ' } else { disp_arr[cursor_y + 1][cursor_x] };
            }
            Event::Key(Key::Char('J')) => {
                cursor_y = std::cmp::min(cell_y.saturating_add(1) * 2 + 1, 18 - 1);
                disp_arr[cursor_y - 1][cursor_x] = if cursor_x % 2 == 1 && prev_cursor_y != cursor_y { ' ' } else { disp_arr[cursor_y - 1][cursor_x] };
            }
            Event::Key(Key::Char(' ')) => {
                match (cursor_x % 2, cursor_y %2) {
                    (1, 0) => { disp_arr[cursor_y][cursor_x] = if disp_arr[cursor_y][cursor_x] == ' '  { '-' } else { ' ' } }
                    (0, 1) => { disp_arr[cursor_y][cursor_x] = if disp_arr[cursor_y][cursor_x] == ' '  { '|' } else { ' ' } }
                    _ => {}
                }
            }
            _ => {}
        }
        draw(&mut stdout, &disp_arr)?;
        write!(stdout, "{}{},{}", cursor::Goto(1, 20), cursor_x, cursor_y)?;
        write!(stdout, "{}", cursor::Goto(cursor_x as u16 + 1, cursor_y as u16 + 1))?;
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