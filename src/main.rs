use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode};
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
        let mut cursor = disp.get_cursor();
        let key = match event? {
            Event::Key(key) => key,
            _ => Key::Ctrl('c')
        };
        let mut moji: String = String::new();
        match state {
            GameState::GameInit => {
                state = GameState::GameEdit;

                moji.push_str(&format!("{}", clear::All));
                moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
            }
            GameState::GameEdit => {
                match key {
                    Key::Ctrl('c') | Key::Char('q') => {
                        break;
                    }
                    Key::Left => {
                        disp.move_cursor_left();
                        cursor = disp.get_cursor();
                    }
                    Key::Right => {
                        disp.move_cursor_right();
                        cursor = disp.get_cursor();
                    }
                    Key::Up => {
                        disp.move_cursor_up();
                        cursor = disp.get_cursor();
                    }
                    Key::Down => {
                        disp.move_cursor_down();
                        cursor = disp.get_cursor();
                    }
                    Key::Char('H') | Key::Char('h')=> {
                        disp.move_cursor_left_cell();
                        handle_remove_wall(&mut disp, prev_cursor, key);
                        moji.push_str(&format!("{}", clear::All));
                        moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
                        cursor = disp.get_cursor();
                    }
                    Key::Char('L') | Key::Char('l') => {
                        disp.move_cursor_right_cell();
                        handle_remove_wall(&mut disp, prev_cursor, key);
                        moji.push_str(&format!("{}", clear::All));
                        moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
                        cursor = disp.get_cursor();
                    }
                    Key::Char('K') | Key::Char('k')=> {
                        disp.move_cursor_up_cell();
                        handle_remove_wall(&mut disp, prev_cursor, key);
                        moji.push_str(&format!("{}", clear::All));
                        moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
                        cursor = disp.get_cursor();
                    }
                    Key::Char('J') | Key::Char('j')=> {
                        disp.move_cursor_down_cell();
                        handle_remove_wall(&mut disp, prev_cursor, key);
                        moji.push_str(&format!("{}", clear::All));
                        moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
                        cursor = disp.get_cursor();
                    }
                    Key::Char(' ') => {
                        disp.toggle_wall_onoff();
                    }
                    Key::Char('v') => {
                        state = GameState::GameSetValue;
                        moji.push_str(&format!("{}({},{})", cursor::Goto(1, 20), prev_cursor.0, prev_cursor.1));
                        cursor = (0, 19);
                    }
                    _ => {}
                }
            }
            GameState::GameSetValue => {
                if key == Key::Char('q') {
                    moji.push_str(&format!("{}           ", cursor::Goto(1, 20)));
                    state = GameState::GameEdit;
                } else {
                    moji.push_str(&format!("{}({},{})", cursor::Goto(1, 20), prev_cursor.0, prev_cursor.1));
                }
            }
        }
        write!(stdout, "{}", moji)?;
        let (x, y) = get_display_coords(cursor);
        write!(stdout, "{}", &format!("{}", cursor::Goto(x, y)))?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_remove_wall(disp: &mut DispField, prev_cursor: (usize, usize), key: termion::event::Key) {
    let curr_cursor = disp.get_cursor();
    match key {
        Key::Char('H') if prev_cursor.0 != curr_cursor.0 => disp.remove_right_wall(),
        Key::Char('L') if prev_cursor.0 != curr_cursor.0 => disp.remove_left_wall(),
        Key::Char('K') if prev_cursor.1 != curr_cursor.1 => disp.remove_down_wall(),
        Key::Char('J') if prev_cursor.1 != curr_cursor.1 => disp.remove_up_wall(),
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
