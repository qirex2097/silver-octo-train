use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode, raw::RawTerminal};
use termion::event::{Event, Key};
use termion::input::TermRead;

use silver_octo_train::*;

fn main() -> Result<(), std::io::Error> {
    let mut disp: DispField = DispField::new();

    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}{}", clear::All, cursor::BlinkingBlock)?;
    draw(&mut stdout, &mut disp.disp_arr)?;
    write!(stdout, "{}", cursor::Goto(disp.cursor.0 as u16 + 1, disp.cursor.1 as u16 + 1))?;
    stdout.flush()?;

    let stdin = stdin();
    for event in stdin.events() {
        let (prev_cursor_x, prev_cursor_y) = disp.cursor;
        let event = event?;
        match event {
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
                break;
            }
            Event::Key(Key::Char('h')) => {
                disp.cursor = cursor_left(disp.cursor);
            }
            Event::Key(Key::Char('l')) => {
                disp.cursor = cursor_right(disp.cursor);
            }
            Event::Key(Key::Char('k')) => {
                disp.cursor = cursor_up(disp.cursor);
            }
            Event::Key(Key::Char('j')) => {
                disp.cursor = cursor_down(disp.cursor);
            }
            Event::Key(Key::Char('H')) => {
                disp.cursor = cursor_left_cell(disp.cursor);
                let (cursor_x, cursor_y) = disp.cursor;
                let disp_arr = &mut disp.disp_arr;
                disp_arr[cursor_y][cursor_x + 1] = if cursor_y % 2 == 1 && prev_cursor_x != cursor_x { ' ' } else { disp_arr[cursor_y][cursor_x + 1] };
            }
            Event::Key(Key::Char('L')) => {
                disp.cursor = cursor_right_cell(disp.cursor);
                let (cursor_x, cursor_y) = disp.cursor;
                let disp_arr = &mut disp.disp_arr;
                disp_arr[cursor_y][cursor_x - 1] = if cursor_y % 2 == 1 && prev_cursor_x != cursor_x { ' ' } else { disp_arr[cursor_y][cursor_x - 1] };
            }
            Event::Key(Key::Char('K')) => {
                disp.cursor = cursor_up_cell(disp.cursor);
                let (cursor_x, cursor_y) = disp.cursor;
                let disp_arr = &mut disp.disp_arr;
                disp_arr[cursor_y + 1][cursor_x] = if cursor_x % 2 == 1 && prev_cursor_y != cursor_y { ' ' } else { disp_arr[cursor_y + 1][cursor_x] };
            }
            Event::Key(Key::Char('J')) => {
                disp.cursor = cursor_down_cell(disp.cursor);
                let (cursor_x, cursor_y) = disp.cursor;
                let disp_arr = &mut disp.disp_arr;
                disp_arr[cursor_y - 1][cursor_x] = if cursor_x % 2 == 1 && prev_cursor_y != cursor_y { ' ' } else { disp_arr[cursor_y - 1][cursor_x] };
            }
            Event::Key(Key::Char(' ')) => {
                let (cursor_x, cursor_y) = disp.cursor;
                let disp_arr = &mut disp.disp_arr;
                match (cursor_x % 2, cursor_y % 2) {
                    (1, 0) => { disp_arr[cursor_y][cursor_x] = if disp_arr[cursor_y][cursor_x] == ' '  { '-' } else { ' ' } }
                    (0, 1) => { disp_arr[cursor_y][cursor_x] = if disp_arr[cursor_y][cursor_x] == ' '  { '|' } else { ' ' } }
                    _ => {}
                }
            }
            _ => {}
        }

        write!(stdout, "{}", clear::All)?;
        draw(&mut stdout, &mut disp.disp_arr)?;
        write!(stdout, "{}", cursor::Goto(disp.cursor.0 as u16 + 1, disp.cursor.1 as u16 + 1))?;
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