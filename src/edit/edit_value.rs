use termion::event::Key;
use termion::{clear, cursor};
use crate::edit::*;

pub struct EditStateSetValue {
    block_no: usize,
    value: usize,
    is_redraw: bool,
    new_value: usize,
}
impl EditStateSetValue {
    pub fn new() -> Self {
        EditStateSetValue { block_no: 0, value: 0, is_redraw: true, new_value: 0, }
    }
}
impl EditState for EditStateSetValue {
    fn initialize(&mut self, data: &mut EditData) {
        let cursor = data.cursor;
        if let Some(pos) = data.disp.get_block_from_index(get_cell_index(cursor)) {
            self.block_no = pos;
            let block: &Block = &data.disp.blocks[self.block_no];
            self.value = block.value;
            self.new_value = self.value;
        }
    }
    fn update(&mut self, data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
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
    fn draw(&mut self, data: &mut EditData) -> String {
        let mut moji: String = String::new();
        let block: &Block = &data.disp.blocks[self.block_no];
        let mut cursor: (usize, usize) = (21 + block.cells.len() * 4, self.block_no);

        if self.is_redraw {
            moji.push_str(&get_blocks_moji(&data.disp, (21, 1)));
            moji.push_str(&format!("{}{}", cursor::Goto(21, self.block_no as u16 + 1), cursor::BlinkingUnderline));
            self.is_redraw = false;
        }

        if self.new_value != self.value {
            self.value = self.new_value;
            let (x, y) = get_display_coords(cursor);
            moji.push_str(&format!("{}{}{}", cursor::Goto(x, y), self.value.to_string(), clear::UntilNewline));
        }

        cursor.0 += if self.value != 0 { self.value.to_string().len() } else { 0 };
        let (x, y) = get_display_coords(cursor);
        moji.push_str(&format!("{}", cursor::Goto(x, y)));

        moji
    }
}

fn get_blocks_moji(disp: &DispField, base_position: (u16, u16)) -> String {
    let mut moji: String = String::new();
    for (y, block) in disp.blocks.iter().enumerate() {
        let line: String = format!("{:?} {}{}", block.cells, block.value, clear::UntilNewline);
        moji.push_str(&format!("{}{}", cursor::Goto(base_position.0, y as u16 + base_position.1), line));
    }
    moji
}