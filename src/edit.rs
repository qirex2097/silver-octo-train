use termion::event::Key;
use termion::{clear, cursor};

use silver_octo_train::*;

pub struct EditData {
    pub disp: DispField,
}

impl EditData {
    pub fn new() -> Self {
        EditData { disp: DispField::new(), }
    }
}
pub trait EditState {
    fn initialize(&mut self, _data: &mut EditData) {}
    fn update(&mut self, _data: &mut EditData, _key_opt: Option<Key>) -> Option<Box<dyn EditState>> { None }
    fn draw(&mut self, _data: &EditData) -> String { String::new() }
    fn finalize(&mut self)  {}
}

pub fn edit_init() -> (Box<dyn EditState>, EditData) {
    (Box::new(EditStateInit::new()), EditData::new())
}
//----------------------------------------
struct EditStateInit {
    is_displayed: bool,
}
impl EditStateInit {
    pub fn new() -> Self {
        EditStateInit { is_displayed: false, }
    }
}
impl EditState for EditStateInit {
    fn update(&mut self, _data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(_key) = key_opt {
            Some(Box::new(EditStateEdit::new()))
        } else {
            None
        }
    }
    fn draw(&mut self, _data: &EditData) -> String {
        let mut moji: String = String::new();
        if !self.is_displayed {
            self.is_displayed = true;
            moji.push_str(&format!("{}PUSH ANY KEY", cursor::Goto(1,1)));
        }
        moji
    }
}
//----------------------------------------
struct EditStateEdit {
    is_redraw: bool,
    new_disp: Option<DispField>,
    _cursor: (usize, usize),
}
impl EditStateEdit {
    fn new() -> Self {
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

//----------------------------------------
struct EditStateDebug {
    state: usize,
}
impl EditStateDebug {
    fn _new() -> Self {
        EditStateDebug { state: 0 }
    }
}
impl EditState for EditStateDebug {
    fn update(&mut self, _data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(Key::Char('\n')) = key_opt {
            return Some(Box::new(EditStateEdit::new()));
        } else if let Some(Key::Char('v')) = key_opt {
            return Some(Box::new(EditStateSetValue::new()));
        }
        None
    }
    fn draw(&mut self, data: &EditData) -> String {
        let mut moji: String = String::new();
        match self.state {
            0 => {
                moji.push_str(&get_blocks_moji(&data.disp, (1, 20)));
                self.state += 1;
            }
            _ => {}
        }
        moji
    }
}
//----------------------------------------
struct EditStateSetValue {
    block_no: usize,
    value: usize,
    is_redraw: bool,
    new_value: usize,
}
impl EditStateSetValue {
    fn new() -> Self {
        EditStateSetValue { block_no: 0, value: 0, is_redraw: true, new_value: 0, }
    }
}
impl EditState for EditStateSetValue {
    fn initialize(&mut self, data: &mut EditData) {
        let cursor = data.disp.get_cursor();
        if let Some(pos) = data.disp.get_block_from_cursor(cursor) {
            self.block_no = pos;
            let block: &Block = &data.disp.blocks[self.block_no];
            self.value = block.value;
            self.new_value = self.value;
        }
    }
    fn update(&mut self, data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if self.value != self.new_value {
            self.value = self.new_value;
        }
        if let Some(key) = key_opt {
            if key == Key::Char('q') || key == Key::Char('\n') {
                if key == Key::Char('\n') {
                    data.disp.set_block_value(self.block_no, self.value);
                }
                return Some(Box::new(EditStateEdit::new()));
            }
        }

        let mut new_value = self.value;
        if let Some(key) = key_opt {
            match key {
                Key::Backspace if new_value > 0 => {
                    new_value /= 10;
                }
                Key::Char(c) => match c {
                    '0'..='9' => {
                        if new_value == 0 {
                            new_value = c as usize - '0' as usize;
                        } else {
                            new_value *= 10;
                            new_value += c as usize - '0' as usize;
                        }
                    }
                    _ => { }
                }
                _ => {}
            }
        }
        self.new_value = new_value;

        None
    }
    fn draw(&mut self, data: &EditData) -> String {
        let mut moji: String = String::new();
        let block: &Block = &data.disp.blocks[self.block_no];
        let mut cursor: (usize, usize) = (21 + block.cells.len() * 4, self.block_no);

        if self.is_redraw {
            moji.push_str(&get_blocks_moji(&data.disp, (21, 1)));
            moji.push_str(&format!("{}{}", cursor::Goto(21, self.block_no as u16 + 1), cursor::BlinkingUnderline));
            self.is_redraw = false;
        }

        if self.new_value != self.value {
            let (x, y) = get_display_coords(cursor);
            moji.push_str(&format!("{}{}{}", cursor::Goto(x, y), self.new_value.to_string(), clear::UntilNewline));
        }

        cursor.0 += if self.new_value != 0 { self.new_value.to_string().len() } else { 0 };
        let (x, y) = get_display_coords(cursor);
        moji.push_str(&format!("{}", cursor::Goto(x, y)));

        moji
    }
}

//----------------------------------------

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

fn get_blocks_moji(disp: &DispField, base_position: (u16, u16)) -> String {
    let mut moji: String = String::new();
    for (y, block) in disp.blocks.iter().enumerate() {
        let line: String = format!("{:?} {}{}", block.cells, block.value, clear::UntilNewline);
        moji.push_str(&format!("{}{}", cursor::Goto(base_position.0, y as u16 + base_position.1), line));
    }
    moji
}