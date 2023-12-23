use crate::edit::*;
use termion::cursor;

pub struct EditStateCalc {
    is_redraw: bool,
}
impl EditStateCalc {
    pub fn new() -> Self {
        EditStateCalc { is_redraw: true }
    }
}

impl EditState for EditStateCalc {
    fn update(&mut self, _data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(Key::Char('q')) = key_opt {
            return Some(Box::new(EditStateEdit::new()));
        }
        None
    }
    fn draw(&mut self, data: &mut EditData) -> String {
        let mut moji = String::from("");
        if self.is_redraw {
            let board_moji = data.disp.get_board_moji((1, 1));
            moji.push_str(&board_moji);
            self.is_redraw = false;
            moji.push_str(&format!("{}CALC", cursor::Goto(1, 20)));
        }

        moji
    }
}
