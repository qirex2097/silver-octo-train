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
    write!(stdout, "{}PUSH ANY KEY", cursor::Goto(1,1))?;
    stdout.flush()?;

    let stdin = stdin();
    for event in stdin.events() {
        let prev_cursor = disp.get_cursor();
        let event = event?;
        match event {
            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
                break;
            }
            Event::Key(Key::Left) => {
                disp.move_cursor_left();
            }
            Event::Key(Key::Right) => {
                disp.move_cursor_right();
            }
            Event::Key(Key::Up) => {
                disp.move_cursor_up();
            }
            Event::Key(Key::Down) => {
                disp.move_cursor_down();
            }
            Event::Key(Key::Char('H')) | Event::Key(Key::Char('h'))=> {
                disp.move_cursor_left_cell();
                handle_remove_wall(&mut disp, prev_cursor, event);
            }
            Event::Key(Key::Char('L')) | Event::Key(Key::Char('l')) => {
                disp.move_cursor_right_cell();
                handle_remove_wall(&mut disp, prev_cursor, event);
            }
            Event::Key(Key::Char('K')) | Event::Key(Key::Char('k'))=> {
                disp.move_cursor_up_cell();
                handle_remove_wall(&mut disp, prev_cursor, event);
            }
            Event::Key(Key::Char('J')) | Event::Key(Key::Char('j'))=> {
                disp.move_cursor_down_cell();
                handle_remove_wall(&mut disp, prev_cursor, event);
            }
            Event::Key(Key::Char(' ')) => {
                disp.toggle_wall_onoff();
            }
            _ => {}
        }

        write!(stdout, "{}", clear::All)?;
        draw(&mut stdout, disp.get_disp_arr())?;
        let (x, y) = disp.get_display_coords();
        write!(stdout, "{}", cursor::Goto(x, y))?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_remove_wall(disp: &mut DispField, prev_cursor: (usize, usize), event: Event) {
    let curr_cursor = disp.get_cursor();
    match event {
        Event::Key(Key::Char('H')) if prev_cursor.0 != curr_cursor.0 => disp.remove_right_wall(),
        Event::Key(Key::Char('L')) if prev_cursor.0 != curr_cursor.0 => disp.remove_left_wall(),
        Event::Key(Key::Char('K')) if prev_cursor.1 != curr_cursor.1 => disp.remove_down_wall(),
        Event::Key(Key::Char('J')) if prev_cursor.1 != curr_cursor.1 => disp.remove_up_wall(),
        _ => {}
    }
}

fn draw<T: std::io::Write>(stdout: &mut RawTerminal<T>, disp_arr: &DispArray) -> Result<(), std::io::Error> {
    for (y, line) in disp_arr.iter().enumerate() {
        let line: String = line.iter().collect();
        write!(stdout, "{}{}", cursor::Goto(1, y as u16 + 1), line)?;
    }
    Ok(())
}