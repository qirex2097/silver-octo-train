use termion::event::Key;

pub mod edit_calc;
pub mod edit_edit;
pub mod edit_file;
pub mod edit_init;
pub mod edit_value;
use crate::edit::edit_calc::EditStateCalc;
use crate::edit::edit_edit::EditStateEdit;
use crate::edit::edit_file::{read_file, write_file};
use crate::edit::edit_init::EditStateInit;
use crate::edit::edit_value::EditStateSetValue;
use silver_octo_train::*;

pub struct EditData {
    pub disp: DispField,
    pub cursor: (usize, usize),
}

impl EditData {
    pub fn new() -> Self {
        EditData {
            disp: DispField::new(),
            cursor: (CURSOR_MIN, CURSOR_MIN),
        }
    }
}
pub trait EditState {
    fn initialize(&mut self, _data: &mut EditData) {}
    fn update(
        &mut self,
        _data: &mut EditData,
        _key_opt: Option<Key>,
    ) -> Option<Box<dyn EditState>> {
        None
    }
    fn draw(&mut self, _data: &mut EditData) -> String {
        String::new()
    }
    fn finalize(&mut self, _data: &mut EditData) {}
    fn is_terminal(&self) -> bool {
        false
    }
}

pub fn edit_init() -> (Box<dyn EditState>, EditData) {
    (Box::new(EditStateInit::new()), EditData::new())
}
