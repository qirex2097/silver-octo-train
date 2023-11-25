const GRID_SIZE: usize = 19;
const CURSOR_MIN: usize = 1;
const CURSOR_MAX: usize = 17;

pub type DispArray = [[char; GRID_SIZE]; GRID_SIZE];
pub struct DispField {
    pub disp_arr: DispArray,
    pub cursor: (usize, usize),
}

impl DispField {
    pub fn new() -> DispField {
        let disp_arr: [[char; GRID_SIZE]; GRID_SIZE] = [
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
            [ '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ' ', '|', ],
            [ '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', '-', '+', ],
        ];

        DispField { disp_arr, cursor: (CURSOR_MIN, CURSOR_MIN), }
    }
}

impl DispField {
    pub fn get_disp_arr(&self) -> &[[char; GRID_SIZE]; GRID_SIZE] {
        &self.disp_arr
    }
    pub fn get_cursor(&self) -> (usize, usize) {
        self.cursor
    }
    pub fn get_ch(&self, pos: (usize, usize)) -> Option<char> {
        if pos.0 >= GRID_SIZE || pos.1 >= GRID_SIZE {
            return None;
        }
        Some(self.disp_arr[pos.1][pos.0])
    }
}

impl DispField {
    pub fn move_cursor_left(&mut self) {
        self.cursor.0 = std::cmp::max(self.cursor.0 - 1, CURSOR_MIN);
    }
    pub fn move_cursor_right(&mut self) {
        self.cursor.0 = std::cmp::min(self.cursor.0 + 1, CURSOR_MAX);
    }
    pub fn move_cursor_up(&mut self) {
        self.cursor.1 = std::cmp::max(self.cursor.1 - 1, CURSOR_MIN);
    }
    pub fn move_cursor_down(&mut self) {
        self.cursor.1 = std::cmp::min(self.cursor.1 + 1, CURSOR_MAX);
    }
    pub fn move_cursor_left_cell(&mut self) {
        self.cursor = cursor_left_cell(self.cursor);
    }
    pub fn move_cursor_right_cell(&mut self) {
        self.cursor = cursor_right_cell(self.cursor);
    }
    pub fn move_cursor_up_cell(&mut self) {
        self.cursor = cursor_up_cell(self.cursor);
    }
    pub fn move_cursor_down_cell(&mut self) {
        self.cursor = cursor_down_cell(self.cursor);
    }
}

impl DispField {
    pub fn remove_wall_cursor(&mut self, cursor: (usize, usize)) {
        if cursor.0 < CURSOR_MIN || CURSOR_MAX < cursor.0 { return; }
        if cursor.1 < CURSOR_MIN || CURSOR_MAX < cursor.1 { return; }
        if cursor.0 % 2 == 0 && cursor.1 % 2 == 0 { return; }
        self.disp_arr[cursor.1][cursor.0] = ' ';
    }
    pub fn toggle_wall_onoff_cursor(&mut self, cursor: (usize, usize)) {
        let (x, y) = cursor;
        let ch = self.disp_arr[y][x];
        self.disp_arr[y][x] = match (x % 2, y % 2) {
            (1, 0) => { if ch == ' '  { '-' } else { ' ' } }
            (0, 1) => { if ch == ' '  { '|' } else { ' ' } }
            _ => { ch }
        };
    }
}

impl DispField {
    pub fn get_block_from_index(&self, cell_index: usize) -> Vec<usize> {
        let mut block: Vec<usize> = vec![cell_index];
        let mut stack: Vec<usize> = vec![cell_index];
        while let Some(curr) = stack.pop() {
            if let Some(left_cell) = get_left_cell(curr) {
                if !block.contains(&left_cell) {
                    match self.get_left_wall(curr) {
                        Some(ch) if ch == ' ' => {
                            block.push(left_cell);
                            stack.push(left_cell);

                        }
                        _ => {}
                    }
                }
            }
            if let Some(right_cell) = get_right_cell(curr) {
                if !block.contains(&right_cell) {
                    match self.get_right_wall(curr) {
                        Some(ch) if ch == ' ' => {
                            block.push(right_cell);
                            stack.push(right_cell);
                        }
                        _ => {}
                    }
                }
            }
            if let Some(up_cell) = get_up_cell(curr) {
                if !block.contains(&up_cell) {
                    match self.get_up_wall(curr) {
                        Some(ch) if ch == ' ' => {
                            block.push(up_cell);
                            stack.push(up_cell);
                        }
                        _ => {}
                    }
                }
            }
            if let Some(down_cell) = get_down_cell(curr) {
                if !block.contains(&down_cell) {
                    match self.get_down_wall(curr) {
                        Some(ch) if ch == ' ' => {
                            block.push(down_cell);
                            stack.push(down_cell);
                        }
                        _ => {}
                    }
                }
            }
        }
        block.sort();
        block
    }
    pub fn get_block_from_cursor(&self, cursor: (usize, usize)) -> Vec<usize> {
        match self.get_ch(cursor) {
            Some(ch) if ch == ' ' => self.get_block_from_index(get_cell_index(cursor)),
            _ => vec![]
        }
    }

    fn get_left_wall(&self, cell_index: usize) -> Option<char> {
        let cursor = get_cursor_from_index(cell_index);
        self.get_ch((cursor.0 - 1, cursor.1))
    }
    fn get_right_wall(&self, cell_index: usize) -> Option<char> {
        let cursor = get_cursor_from_index(cell_index);
        self.get_ch((cursor.0 + 1, cursor.1))
    }
    fn get_up_wall(&self, cell_index: usize) -> Option<char> {
        let cursor = get_cursor_from_index(cell_index);
        self.get_ch((cursor.0, cursor.1 - 1))
    }
    fn get_down_wall(&self, cell_index: usize) -> Option<char> {
        let cursor = get_cursor_from_index(cell_index);
        self.get_ch((cursor.0, cursor.1 + 1))
    }
}

fn get_left_cell(cell_index: usize) -> Option<usize> {
    if cell_index % 10 == 1 { return None; }
    Some(cell_index - 1)
}
fn get_right_cell(cell_index: usize) -> Option<usize> {
    if cell_index % 10 == 9 { return None; }
    Some(cell_index + 1)
}
fn get_up_cell(cell_index: usize) -> Option<usize> {
    if cell_index / 10 == 1 { return None; }
    Some(cell_index - 10)
}
fn get_down_cell(cell_index: usize) -> Option<usize> {
    if cell_index / 10 == 9 { return None; }
    Some(cell_index + 10)
}


#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn test_get_block_from_index() {
        let mut disp: DispField = DispField::new();
        let v = disp.get_block_from_index(11);
        assert_eq!(v, [11]);
        disp.remove_wall_cursor((1, 2));
        disp.remove_wall_cursor((2, 1));
        disp.remove_wall_cursor((2, 3));
        let v = disp.get_block_from_index(11);
        assert_eq!(v, [11, 12, 21, 22]);
        let v = disp.get_block_from_cursor((2, 1));
        assert_eq!(v, [11, 12, 21, 22]);
        let v = disp.get_block_from_cursor((4, 1));
        assert_eq!(v, []);
        let v = disp.get_block_from_index(99);
        assert_eq!(v, [99]);
        let cursor = get_cursor_from_index(99);
        disp.remove_wall_cursor((cursor.0, cursor.1 - 1));
        let v = disp.get_block_from_index(99);
        assert_eq!(v, [89,99]);
    }
}

pub fn get_display_coords(cursor: (usize, usize)) -> (u16, u16) {
    (cursor.0 as u16 + 1, cursor.1 as u16 + 1)
}
pub fn get_cell_index(cursor: (usize, usize)) -> usize {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    cell_y * 10 + cell_x
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
pub fn get_cursor_from_index(cell_index: usize) -> (usize, usize) {
    get_cursor_from_cell_coords(get_cell_coords_from_index(cell_index))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_index_test() {
        assert_eq!(get_cell_index((0, 0)), 11);
        assert_eq!(get_cell_index((1, 1)), 11);
        assert_eq!(get_cell_index((9, 9)), 55);
        assert_eq!(get_cell_index((18, 18)), 99);
        assert_eq!(get_cell_coords_from_index(11), (1, 1));
        assert_eq!(get_cell_coords_from_index(99), (9, 9));
        assert_eq!(get_cell_coords_from_index(72), (2, 7));
        assert_eq!(get_cursor_from_index(11), (1, 1));
        assert_eq!(get_cursor_from_index(12), (3, 1));
        assert_eq!(get_cursor_from_index(37), (13, 5));
    }
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
    #[test]
    fn test_cursor_boundaries() {
        let mut disp: DispField = DispField::new();
        assert_eq!(get_cell_coords(disp.cursor), (1, 1));
        assert_eq!(disp.cursor, (1, 1));
        disp.move_cursor_left();
        assert_eq!(disp.cursor, (1, 1));
        disp.move_cursor_right();
        assert_eq!(disp.cursor, (2, 1));
        disp.move_cursor_up();
        assert_eq!(disp.cursor, (2, 1));
        disp.move_cursor_down();
        assert_eq!(disp.cursor, (2, 2));
        disp.move_cursor_left();
        assert_eq!(disp.cursor, (1, 2));
        disp.move_cursor_up();
        assert_eq!(disp.cursor, (1, 1));
    }
}