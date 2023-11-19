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
        let event = event?;
        match event {
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
                break;
            }
            Event::Key(Key::Char('h')) => {
                disp.move_cursor_left();
            }
            Event::Key(Key::Char('l')) => {
                disp.move_cursor_right();
            }
            Event::Key(Key::Char('k')) => {
                disp.move_cursor_up();
            }
            Event::Key(Key::Char('j')) => {
                disp.move_cursor_down();
            }
            Event::Key(Key::Char('H')) => {
                let prev_cursor = disp.cursor;
                disp.move_cursor_left_cell();
                if prev_cursor.0 != disp.cursor.0 {
                    disp.clear_right_wall();
                }
            }
            Event::Key(Key::Char('L')) => {
                let prev_cursor = disp.cursor;
                disp.move_cursor_right_cell();
                if prev_cursor.0 != disp.cursor.0 {
                    disp.clear_left_wall();
                }
            }
            Event::Key(Key::Char('K')) => {
                let prev_cursor = disp.cursor;
                disp.move_cursor_up_cell();
                if prev_cursor.1 != disp.cursor.1 {
                    disp.clear_down_wall();
                }
            }
            Event::Key(Key::Char('J')) => {
                let prev_cursor = disp.cursor;
                disp.move_cursor_down_cell();
                if prev_cursor.1 != disp.cursor.1 {
                    disp.clear_up_wall();
                }
            }
            Event::Key(Key::Char(' ')) => {
                disp.toggle_wall_onoff();
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