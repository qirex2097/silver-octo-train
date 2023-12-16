use crate::edit::*;
use termion::cursor;

pub struct EditStateCalc;

impl EditState for EditStateCalc {
    fn update(&mut self, _data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(Key::Char('q')) = key_opt {
            return Some(Box::new(EditStateEdit::new()));
        }
        None
    }
    fn draw(&mut self, _data: &mut EditData) -> String {
        return String::from(&format!("{}CALC", cursor::Goto(1, 20)));
    }
}
