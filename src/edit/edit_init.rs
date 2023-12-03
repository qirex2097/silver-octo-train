use termion::event::Key;
use termion::cursor;
use crate::edit::*;

pub struct EditStateInit {
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