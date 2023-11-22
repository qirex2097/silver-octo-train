use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode, raw::RawTerminal};
use termion::event::{Event, Key};
use termion::input::TermRead;

use silver_octo_train::*;

enum GameState {
    GameInit,
    GameEdit,
    GameSetValue,
}

fn main() -> Result<(), std::io::Error> {
    let mut disp: DispField = DispField::new();
    let mut state: GameState = GameState::GameInit;

    let stdout = stdout().into_alternate_screen()?;
    let mut stdout = stdout.into_raw_mode()?;

    write!(stdout, "{}{}", clear::All, cursor::BlinkingBlock)?;
    write!(stdout, "{}PUSH ANY KEY", cursor::Goto(1,1))?;
    stdout.flush()?;

    let stdin = stdin();
    for event in stdin.events() {
        let prev_cursor = disp.get_cursor();
        let event = event?;
        if event == Event::Key(Key::Ctrl('c')) {
            break;
        }
        match state {
            GameState::GameInit => {
                state = GameState::GameEdit;

                write!(stdout, "{}", clear::All)?;
                let mut moji = String::from(format!("{}", clear::All));
                moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
                let (x, y) = get_display_coords(disp.get_cursor());
                moji.push_str(&format!("{}", cursor::Goto(x, y)));
                write!(stdout, "{}", moji)?;
            }
            GameState::GameEdit => {
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
                    Event::Key(Key::Char('v')) => {
                        state = GameState::GameSetValue;
                    }
                    _ => {}
                }

                write!(stdout, "{}", clear::All)?;
                draw(&mut stdout, disp.get_disp_arr(), (1, 1))?;
                let (x, y) = get_display_coords(disp.get_cursor());
                write!(stdout, "{}", cursor::Goto(x, y))?;
            }
            GameState::GameSetValue => {
                if event == Event::Key(Key::Char('q')) {
                    state = GameState::GameEdit;
                }
            }
        }
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

fn get_board_moji(disp_arr: &DispArray, base_position: (u16, u16)) -> String {
    let mut moji: String = String::new();
    for (y, line) in disp_arr.iter().enumerate() {
        let line: String = line.iter().collect();
        moji.push_str(&format!("{}{}", cursor::Goto(base_position.0, y as u16 + base_position.1), line));
    }
    moji
}

fn draw<T: std::io::Write>(stdout: &mut RawTerminal<T>, disp_arr: &DispArray, base_position: (u16, u16)) -> Result<(), std::io::Error> {
    let moji = get_board_moji(&disp_arr, base_position);
    write!(stdout, "{}", moji)?;
    /*
    for (y, line) in disp_arr.iter().enumerate() {
        let line: String = line.iter().collect();
        write!(stdout, "{}{}", cursor::Goto(base_position.0, y as u16 + base_position.1), line)?;
    }
     */
    Ok(())
}
