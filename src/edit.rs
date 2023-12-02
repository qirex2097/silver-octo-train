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
    fn initialize(&mut self, _disp: &mut DispField) -> String { String::from("") }
    fn update(&mut self, _disp: &mut DispField, _key_opt: Option<Key>) -> Option<Box<dyn EditState>> { None }
    fn draw(&mut self, _disp: &DispField) -> String { String::from("") }
    fn finalize(&mut self) -> String { String::from("") }
}

pub fn edit_init() -> Box<dyn EditState> {
    Box::new(EditStateInit::new())
}
//----------------------------------------
struct EditStateInit {
    is_displayed: bool,
}
impl EditState for EditStateInit {
    fn update(&mut self, _disp: &mut DispField, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(_key) = key_opt {
            Some(Box::new(EditStateEdit::new()))
        } else {
            None
        }
    }
    fn draw(&mut self, _disp: &DispField) -> String {
        let mut moji: String = String::new();
        if !self.is_displayed {
            self.is_displayed = true;
            moji.push_str(&format!("{}PUSH ANY KEY", cursor::Goto(1,1)));
        }
        moji
    }
}
impl EditStateInit {
    pub fn new() -> Self {
        EditStateInit { is_displayed: false, }
    }
}

//----------------------------------------
struct EditStateEdit {
    is_redraw: bool,
    moji: String,
}
impl EditState for EditStateEdit {
    fn initialize(&mut self, disp: &mut DispField) -> String {
        let cursor = get_display_coords(disp.get_cursor());
        String::from(&format!("{}", cursor::Goto(cursor.0, cursor.1)))
    }
    fn update(&mut self, disp: &mut DispField, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        self.moji = String::new();
        if let Some(key) = key_opt {
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
            disp.set_cursor(cursor);

            if key == Key::Char('H') || key == Key::Char('L') || key == Key::Char('K') ||  key == Key::Char('J') {
                if let Some(m) = handle_remove_wall(disp, prev_cursor, key) {
                    self.moji.push_str(&m);
                }
            } else if key == Key::Char(' ') {
                disp.toggle_wall_onoff_cursor(cursor);
                if let Some(ch) = disp.get_ch(cursor) {
                    let disp_cursor = get_display_coords(cursor);
                    self.moji.push_str(&format!("{}{}", cursor::Goto(disp_cursor.0, disp_cursor.1), ch));
                }
            }

            if key == Key::Char('v') {
                if let Some(_pos) = disp.get_block_from_cursor(cursor) {
                    return Some(Box::new(EditStateSetValue::new()))
                }
            }
        }
        None
    }
    fn draw(&mut self, disp: &DispField) -> String {
        if self.is_redraw {
            self.moji.push_str(&get_board_moji(disp.get_disp_arr(), (1, 1)));
            self.is_redraw = false;
        }
        let (x, y) = get_display_coords(disp.get_cursor());
        self.moji.push_str(&format!("{}", cursor::Goto(x, y)));
        self.moji.clone()
    }
}
impl EditStateEdit {
    fn new() -> Self {
        EditStateEdit { is_redraw: true, moji: String::new(), }
    }
}
//----------------------------------------
struct EditStateSetValue {
    block_no: usize,
    value: usize,
}
impl EditState for EditStateSetValue {
    fn initialize(&mut self, disp: &mut DispField) -> String {
        let cursor = disp.get_cursor();
        if let Some(pos) = disp.get_block_from_cursor(cursor) {
            let mut moji = String::new();
            self.block_no = pos;
            let block: &Block = &disp.blocks[self.block_no];
            self.value = block.value;
            let mut cursor: (usize, usize) = (21 + block.cells.len() * 4, self.block_no);
            cursor.0 += if self.value != 0 { self.value.to_string().len() } else { 0 };
            moji.push_str(&get_blocks_moji(&disp, (21, 1)));
            moji.push_str(&format!("{}{}", cursor::Goto(21, self.block_no as u16 + 1), cursor::BlinkingUnderline));
            moji
        } else {
            String::new()
        }
    }
    fn update(&mut self, disp: &mut DispField, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(key) = key_opt {
            if key == Key::Char('q') || key == Key::Char('\n') {
                if key == Key::Char('\n') {
                    disp.set_block_value(self.block_no, self.value);
                }
                return Some(Box::new(EditStateEdit::new()));
            }
        }

        if let Some(key) = key_opt {
            match key {
                Key::Backspace if self.value > 0 => {
                    self.value /= 10;
                }
                Key::Char(c) => match c {
                    '0'..='9' => {
                        if self.value == 0 {
                            self.value = c as usize - '0' as usize;
                        } else {
                            self.value *= 10;
                            self.value += c as usize - '0' as usize;
                        }
                    }
                    _ => { }
                }
                _ => {}
            }
        }

        None
    }
    fn draw(&mut self, disp: &DispField) -> String {
        let mut moji: String = String::new();
        let block: &Block = &disp.blocks[self.block_no];
        let mut cursor = (21 + block.cells.len() * 4, self.block_no);

        let (x, y) = get_display_coords(cursor);
        moji.push_str(&format!("{}{}{}", cursor::Goto(x, y), self.value.to_string(), clear::UntilNewline));
        cursor.0 += if self.value != 0 { self.value.to_string().len() } else { 0 };
        let (x, y) = get_display_coords(cursor);
        moji.push_str(&format!("{}", cursor::Goto(x, y)));

        moji
    }
    fn finalize(&mut self) -> String {
        String::from(&format!("{}", cursor::BlinkingBlock))
    }
}

impl EditStateSetValue {
    fn new() -> Self {
        EditStateSetValue { block_no: 0, value: 0, }
    }
}
//----------------------------------------

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