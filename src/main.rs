use std::io::{stdin, stdout, Write};
use termion::{clear, cursor};
use termion::{screen::IntoAlternateScreen, raw::IntoRawMode};
use termion::event::Key;

mod event;
use crate::event::*;
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
    stdout.flush()?;

    let mut block_no: usize = 0;
    let mut value: usize = 0;

    let rs = event_init(stdin());
    loop {
        let key_opt = event_wait(&rs);
        let mut moji: String = String::new();
        match state {
            GameState::GameInit => {
                if let Some(_key) = key_opt {
                    state = GameState::GameEdit;
                } else {
                    moji.push_str(&format!("{}PUSH ANY KEY", cursor::Goto(1,1)));
                }
            }
            GameState::GameEdit => {
                if let Some(key) = key_opt {
                if key == Key::Ctrl('c') || key == Key::Char('q') {
                    break;
                }
                let prev_cursor = disp.get_cursor();
                let mut cursor = disp.get_cursor();
                match key {
                    Key::Left => {
                        cursor = cursor_left(cursor);
                    }
                    Key::Right => {
                        cursor = cursor_right(cursor);
                    }
                    Key::Up => {
                        cursor = cursor_up(cursor);
                    }
                    Key::Down => {
                        cursor = cursor_down(cursor);
                    }
                    Key::Char('H') | Key::Char('h')=> {
                        cursor = cursor_left_cell(cursor);
                    }
                    Key::Char('L') | Key::Char('l') => {
                        cursor = cursor_right_cell(cursor);
                    }
                    Key::Char('K') | Key::Char('k')=> {
                        cursor = cursor_up_cell(cursor);
                    }
                    Key::Char('J') | Key::Char('j')=> {
                        cursor = cursor_down_cell(cursor);
                    }
                    _ => {}
                }
                disp.cursor = cursor;

                if key == Key::Char('H') || key == Key::Char('L') || key == Key::Char('K') ||  key == Key::Char('J') {
                    if let Some(m) = handle_remove_wall(&mut disp, prev_cursor, key) {
                        moji.push_str(&m);
                    }
                } else if key == Key::Char(' ') {
                    disp.toggle_wall_onoff_cursor(cursor);
                    if let Some(ch) = disp.get_ch(cursor) {
                        let disp_cursor = get_display_coords(cursor);
                        moji.push_str(&format!("{}{}", cursor::Goto(disp_cursor.0, disp_cursor.1), ch));
                    }
                }

                if key == Key::Char('v') {
                    if let Some(pos) = disp.get_block_from_cursor(cursor) {
                        block_no = pos;
                        let block: &Block = &disp.blocks[block_no];
                        value = block.value;
                        cursor.0 = 21 + block.cells.len() * 4;
                        cursor.1 = block_no;
                        cursor.0 += if value != 0 { value.to_string().len() } else { 0 };
                        moji.push_str(&get_blocks_moji(&disp, (21, 1)));
                        moji.push_str(&format!("{}{}", cursor::Goto(21, block_no as u16 + 1), cursor::BlinkingUnderline));
                        state = GameState::GameSetValue;
                    }
                }
                let (x, y) = get_display_coords(cursor);
                moji.push_str(&format!("{}", cursor::Goto(x, y)));
                } else {
                    // moji.push_str(&format!("{}", clear::All));
                    moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
                    let (x, y) = get_display_coords(disp.get_cursor());
                    moji.push_str(&format!("{}", cursor::Goto(x, y)));
                }
            }
            GameState::GameSetValue => {
                let block: &Block = &disp.blocks[block_no];
                if let Some(key) = key_opt {
                if key == Key::Char('q') || key == Key::Char('\n') {
                    if key == Key::Char('\n') {
                        disp.set_block_value(block_no, value);
                    }
                    let (x, y) = get_display_coords(disp.get_cursor());
                    moji.push_str(&format!("{}{}", cursor::BlinkingBlock, cursor::Goto(x, y)));
                    state = GameState::GameEdit;
                } else {
                    let mut cursor = (21 + block.cells.len() * 4, block_no);
                    match key {
                        Key::Backspace if value > 0 => {
                            value /= 10;
                        }
                        Key::Char(c) => match c {
                            '0'..='9' => {
                                if value == 0 {
                                    value = c as usize - '0' as usize;
                                } else {
                                    value *= 10;
                                    value += c as usize - '0' as usize;
                                }
                            }
                            _ => { }
                        }
                        _ => {}
                    }
                    let (x, y) = get_display_coords(cursor);
                    moji.push_str(&format!("{}{}{}", cursor::Goto(x, y), value.to_string(), clear::UntilNewline));
                    cursor.0 += if value != 0 { value.to_string().len() } else { 0 };
                    let (x, y) = get_display_coords(cursor);
                    moji.push_str(&format!("{}", cursor::Goto(x, y)));
                }
                }
            }
        }
        write!(stdout, "{}", moji)?;
        stdout.flush()?;
    }

    Ok(())
}

fn handle_remove_wall(disp: &mut DispField, prev_cursor: (usize, usize), key: termion::event::Key) -> Option<String> {
    let curr_cursor = disp.get_cursor();
    let wall_cursor;
    match key {
        Key::Char('H') if prev_cursor.0 != curr_cursor.0 => {
            wall_cursor = (curr_cursor.0 + 1, curr_cursor.1);
        },
        Key::Char('L') if prev_cursor.0 != curr_cursor.0 => {
            wall_cursor = (curr_cursor.0 - 1, curr_cursor.1);
        },
        Key::Char('K') if prev_cursor.1 != curr_cursor.1 => {
            wall_cursor = (curr_cursor.0, curr_cursor.1 + 1);
        },
        Key::Char('J') if prev_cursor.1 != curr_cursor.1 => {
            wall_cursor = (curr_cursor.0, curr_cursor.1 - 1);
        },
        _ => { return None; }
    }

    disp.remove_wall_cursor(wall_cursor);
    if let Some(ch) = disp.get_ch(wall_cursor) {
        let disp_cursor = get_display_coords(wall_cursor);
        let moji = format!("{}{}", cursor::Goto(disp_cursor.0, disp_cursor.1), ch);
        Some(moji)
    } else {
        None
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

fn get_blocks_moji(disp: &DispField, base_position: (u16, u16)) -> String {
    let mut moji: String = String::new();
    for (y, block) in disp.blocks.iter().enumerate() {
        let line: String = format!("{:?} {}{}", block.cells, block.value, clear::UntilNewline);
        moji.push_str(&format!("{}{}", cursor::Goto(base_position.0, y as u16 + base_position.1), line));
    }
    moji
}