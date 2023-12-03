use termion::event::Key;
use termion::cursor;
use crate::edit::*;

pub struct EditStateEdit {
    is_redraw: bool,
    new_disp: Option<DispField>,
    cursor: (usize, usize),
}
impl EditStateEdit {
    pub fn new() -> Self {
        EditStateEdit { is_redraw: true, new_disp: None, cursor: (0, 0), }
    }
}
impl EditState for EditStateEdit {
    fn initialize(&mut self, data: &mut EditData) {
        self.is_redraw = true;
        self.cursor = data.cursor;
    }
    fn update(&mut self, data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(key) = key_opt {
            let prev_cursor = self.cursor;
            let mut cursor = self.cursor;
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
            self.cursor = cursor;

            if key == Key::Char('H') || key == Key::Char('L') || key == Key::Char('K') ||  key == Key::Char('J') {
                self.new_disp = handle_remove_wall(&data.disp, cursor, prev_cursor, key);
            } else if key == Key::Char(' ') {
                self.new_disp = toggle_wall_onoff_cursor(&data.disp, cursor);
            }

            if key == Key::Char('v') || key == Key::Char('\n') {
                if let Some(_pos) = data.disp.get_block_from_cursor(cursor) {
                    self.is_redraw = true;
                    return Some(Box::new(EditStateSetValue::new()))
                }
            }

            if key == Key::Char('r') {
                self.is_redraw = true;
            }
            if key == Key::Char('q') {
                return Some(Box::new(EditStateTerminal));
            }
        }
        None
    }
    fn draw(&mut self, data: &mut EditData) -> String {
        let mut moji: String = String::new();
        if let Some(disp) = self.new_disp.take() {
            data.disp = disp;
            self.is_redraw = true;
        }
        if self.is_redraw {
            moji.push_str(&get_board_moji(data.disp.get_disp_arr(), (1, 1)));
            moji.push_str(&get_cell_color(&data.disp, (1, 1)));
            moji.push_str(&format!("{}", cursor::BlinkingBlock));
            self.is_redraw = false;
        }
        let (x, y) = get_display_coords(self.cursor);
        moji.push_str(&format!("{}", cursor::Goto(x, y)));
        moji
    }
    fn finalize(&mut self, data: &mut EditData) {
        data.cursor = self.cursor;
    }
}

fn handle_remove_wall(prev_disp: &DispField, curr_cursor: (usize, usize), prev_cursor: (usize, usize), key: termion::event::Key) -> Option<DispField> {
    let mut disp = prev_disp.clone();

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

fn get_cell_color(disp: &DispField, base_position: (u16, u16)) -> String {
    let mut moji = String::new();
    for block in disp.blocks.iter() {
        for &cell_index in block.cells.iter() {
            let ch_color = "\x1b[46m";
            let ch_default = "\x1b[49m";
            let (x, y) = get_cursor_from_index(cell_index);
            moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0, y as u16 + base_position.1), ch_color, ch_default));
            if block.cells.contains(&(cell_index + 1)) {
                moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0 + 1, y as u16 + base_position.1), ch_color, ch_default));
            }
            if block.cells.contains(&(cell_index + 10)) {
                moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0, y as u16 + base_position.1 + 1), ch_color, ch_default));
            }
            if block.cells.contains(&(cell_index + 1)) && block.cells.contains(&(cell_index + 10)) && block.cells.contains(&(cell_index + 11)) {
                moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0 + 1, y as u16 + base_position.1 + 1), ch_color, ch_default));
            }
        }
    }
    moji
}

pub struct EditStateTerminal;
impl EditState for EditStateTerminal {
    fn is_terminal(&self) -> bool { true }
}