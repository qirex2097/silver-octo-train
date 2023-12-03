use termion::event::Key;
use termion::cursor;
use crate::edit::*;

pub struct EditStateEdit {
    is_redraw: bool,
    new_disp: Option<DispField>,
    _cursor: (usize, usize),
}
impl EditStateEdit {
    pub fn new() -> Self {
        EditStateEdit { is_redraw: true, new_disp: None, _cursor: (0, 0), }
    }
}
impl EditState for EditStateEdit {
    fn initialize(&mut self, _data: &mut EditData) {
        self.is_redraw = true;
    }
    fn update(&mut self, data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        self.new_disp.as_mut().map(|disp| data.disp = disp.clone());
        self.new_disp = None;
        if let Some(key) = key_opt {
            let prev_cursor = data.disp.get_cursor();
            let mut cursor = data.disp.get_cursor();
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
            data.disp.set_cursor(cursor);

            if key == Key::Char('H') || key == Key::Char('L') || key == Key::Char('K') ||  key == Key::Char('J') {
                self.new_disp = handle_remove_wall(&data.disp, prev_cursor, key);
            } else if key == Key::Char(' ') {
                self.new_disp = toggle_wall_onoff_cursor(&data.disp, cursor);
            }
            if key == Key::Char('v') {
                if let Some(_pos) = data.disp.get_block_from_cursor(cursor) {
                    return Some(Box::new(EditStateSetValue::new()))
                }
            }
        }
        None
    }
    fn draw(&mut self, data: &EditData) -> String {
        let mut moji: String = String::new();
        if self.new_disp.is_some() {
            let disp = self.new_disp.as_mut().unwrap();
            moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
        } else if self.is_redraw {
            moji.push_str(&get_board_moji(data.disp.get_disp_arr(), (1, 1)));
            moji.push_str(&format!("{}", cursor::BlinkingBlock));
            self.is_redraw = false;
        }
        let (x, y) = get_display_coords(data.disp.get_cursor());
        moji.push_str(&format!("{}", cursor::Goto(x, y)));
        moji
    }
}

fn handle_remove_wall(prev_disp: &DispField, prev_cursor: (usize, usize), key: termion::event::Key) -> Option<DispField> {
    let mut disp = prev_disp.clone();

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
    Some(disp)
}

fn toggle_wall_onoff_cursor(prev_disp: &DispField, cursor: (usize, usize)) -> Option<DispField> {
    let mut disp = prev_disp.clone();
    disp.toggle_wall_onoff_cursor(cursor);
    Some(disp)
}

fn get_board_moji(disp_arr: &DispArray, base_position: (u16, u16)) -> String {
    let mut moji: String = String::new();
    for (y, line) in disp_arr.iter().enumerate() {
        let line: String = line.iter().collect();
        moji.push_str(&format!("{}{}", cursor::Goto(base_position.0, y as u16 + base_position.1), line));
    }
    moji
}
