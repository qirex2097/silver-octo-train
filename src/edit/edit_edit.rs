use termion::event::Key;
use termion::cursor;
use crate::edit::*;

const CH_COLOR: &str = "\x1b[46m";
const CH_DEFAULT: &str = "\x1b[49m";

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

            let wall_cursor: Option<(usize, usize)> = match key {
                Key::Char('H') if cursor.0 != prev_cursor.0 => Some((cursor.0 + 1, cursor.1)),
                Key::Char('L') if cursor.0 != prev_cursor.0 => Some((cursor.0 - 1, cursor.1)),
                Key::Char('K') if cursor.1 != prev_cursor.1 => Some((cursor.0, cursor.1 + 1)),
                Key::Char('J') if cursor.1 != prev_cursor.1 => Some((cursor.0, cursor.1 - 1)),
                Key::Char(' ') => Some(cursor),
                _ => None,
            };
            if let Some(wall_cursor) = wall_cursor {
                if let Some(ch) = data.disp.get_ch(wall_cursor) {
                    if cursor == wall_cursor || ch != ' ' {
                        let mut disp = data.disp.clone();
                        disp.toggle_wall_onoff_at_cursor(wall_cursor);
                        self.new_disp = Some(disp);
                    }
                }
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
            let (x, y) = get_cursor_from_index(cell_index);
            moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0, y as u16 + base_position.1), CH_COLOR, CH_DEFAULT));
            if block.cells.contains(&(cell_index + 1)) {
                moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0 + 1, y as u16 + base_position.1), CH_COLOR, CH_DEFAULT));
            }
            if block.cells.contains(&(cell_index + 10)) {
                moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0, y as u16 + base_position.1 + 1), CH_COLOR, CH_DEFAULT));
            }
            if block.cells.contains(&(cell_index + 1)) && block.cells.contains(&(cell_index + 10)) && block.cells.contains(&(cell_index + 11)) {
                moji.push_str(&format!("{}{} {}", cursor::Goto(x as u16 + base_position.0 + 1, y as u16 + base_position.1 + 1), CH_COLOR, CH_DEFAULT));
            }
        }
    }
    moji
}

pub struct EditStateTerminal;
impl EditState for EditStateTerminal {
    fn is_terminal(&self) -> bool { true }
}


fn cursor_left(cursor: (usize, usize)) -> (usize, usize) {
    (std::cmp::max(cursor.0 - 1, CURSOR_MIN), cursor.1)
}
fn cursor_right(cursor: (usize, usize)) -> (usize, usize) {
    (std::cmp::min(cursor.0 + 1, CURSOR_MAX), cursor.1)
}
fn cursor_up(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0, std::cmp::max(cursor.1 - 1, CURSOR_MIN))
}
fn cursor_down(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0, std::cmp::min(cursor.1 + 1, CURSOR_MAX))
}
fn cursor_left_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);// cursor = 4, 4, cell_x, y = 2, 2
    let cell_x = if cursor.0 % 2 == 1 { cell_x } else { cell_x + 1 }; // cell_x = 3
    let (cursor_x, _) = get_cursor_from_cell_coords((std::cmp::max(cell_x - 1, 1), cell_y));
    (cursor_x, cursor.1)
}
fn cursor_right_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let (cursor_x, _) = get_cursor_from_cell_coords((std::cmp::min(cell_x + 1, 9), cell_y));
    (cursor_x, cursor.1)
}
fn cursor_up_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let cell_y = if cursor.1 % 2 == 1 { cell_y } else { cell_y + 1};
    let (_, cursor_y) = get_cursor_from_cell_coords((cell_x, std::cmp::max(cell_y - 1, 1)));
    (cursor.0, cursor_y)
}
fn cursor_down_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let (_, cursor_y) = get_cursor_from_cell_coords((cell_x, std::cmp::min(cell_y + 1, 9)));
    (cursor.0, cursor_y)
}

fn get_cell_coords_from_index(cell_index: usize) -> (usize, usize) {
    (cell_index % 10, cell_index / 10)
}
fn get_cell_coords(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0.saturating_sub(1) / 2 + 1, cursor.1.saturating_sub(1) / 2 + 1)
}
fn get_cursor_from_cell_coords(cell_coords: (usize, usize)) -> (usize, usize) {
    ((cell_coords.0 - 1) * 2 + 1, (cell_coords.1 - 1) * 2 + 1)
}
fn get_cursor_from_index(cell_index: usize) -> (usize, usize) {
    get_cursor_from_cell_coords(get_cell_coords_from_index(cell_index))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cursor_move() {
        assert_eq!(cursor_left_cell((3, 3)), (1, 3));
        assert_eq!(cursor_left_cell((2, 3)), (1, 3));
        assert_eq!(cursor_left_cell((1, 3)), (1, 3));
        assert_eq!(cursor_left_cell((4, 4)), (3, 4));
        assert_eq!(cursor_right_cell((1, 1)), (3, 1));
        assert_eq!(cursor_right_cell((2, 1)), (3, 1));
        assert_eq!(cursor_right_cell((15, 2)), (17, 2));
        assert_eq!(cursor_right_cell((16, 1)), (17, 1));
        assert_eq!(cursor_right_cell((17, 1)), (17, 1));
        assert_eq!(cursor_up_cell((1, 1)), (1, 1));
        assert_eq!(cursor_up_cell((1, 2)), (1, 1));
        assert_eq!(cursor_up_cell((1, 3)), (1, 1));
        assert_eq!(cursor_up_cell((4, 4)), (4, 3));
        assert_eq!(cursor_down_cell((1, 17)), (1, 17));
        assert_eq!(cursor_down_cell((1, 16)), (1, 17));
        assert_eq!(cursor_down_cell((1, 15)), (1, 17));
        assert_eq!(cursor_down_cell((2, 14)), (2, 15));
    }
    #[test]
    fn get_index_test() {
        assert_eq!(get_cell_coords_from_index(11), (1, 1));
        assert_eq!(get_cell_coords_from_index(99), (9, 9));
        assert_eq!(get_cell_coords_from_index(72), (2, 7));
        assert_eq!(get_cursor_from_index(11), (1, 1));
        assert_eq!(get_cursor_from_index(12), (3, 1));
        assert_eq!(get_cursor_from_index(37), (13, 5));
    }
    #[test]
    fn test_get_cell_coords() {
        assert_eq!(get_cell_coords((1, 1)), (1, 1));
        assert_eq!(get_cursor_from_cell_coords((1, 1)), (1, 1));
        assert_eq!(get_cell_coords((2, 1)), (1, 1));
        assert_eq!(get_cell_coords((3, 1)), (2, 1));
        assert_eq!(get_cursor_from_cell_coords((2, 1)), (3, 1));
        assert_eq!(get_cell_coords((3, 3)), (2, 2));
        assert_eq!(get_cursor_from_cell_coords((2, 2)), (3, 3));
        assert_eq!(get_cell_coords((7, 7)), (4, 4));
    }
}