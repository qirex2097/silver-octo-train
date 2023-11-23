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
    pub fn get_ch(&self, pos: (usize, usize)) -> char {
        self.disp_arr[pos.1][pos.0]
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
    pub fn remove_right_wall(&mut self) {
        if self.cursor.1 % 2 == 0 { return; }
        self.disp_arr[self.cursor.1][self.cursor.0 + 1] = ' ';
    }
    pub fn remove_left_wall(&mut self) {
        if self.cursor.1 % 2 == 0 { return; }
        self.disp_arr[self.cursor.1][self.cursor.0 - 1] = ' ';
    }
    pub fn remove_up_wall(&mut self) {
        if self.cursor.0 % 2 == 0 { return; }
        self.disp_arr[self.cursor.1 - 1][self.cursor.0] = ' ';
    }
    pub fn remove_down_wall(&mut self) {
        if self.cursor.0 % 2 == 0 { return; }
        self.disp_arr[self.cursor.1 + 1][self.cursor.0] = ' ';
    }
    pub fn toggle_wall_onoff(&mut self) {
        let (x, y) = self.cursor;
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
            let cursor = get_cursor_from_index(curr);
            let left_cell = curr - 1;
            if self.get_ch((cursor.0 - 1, cursor.1)) == ' ' && !block.contains(&left_cell) {
                block.push(left_cell);
                stack.push(left_cell);
            }
            let right_cell = curr + 1;
            if self.get_ch((cursor.0 + 1, cursor.1)) == ' ' && !block.contains(&right_cell) {
                block.push(right_cell);
                stack.push(right_cell);
            }
            let up_cell = curr - 10;
            if self.get_ch((cursor.0, cursor.1 - 1)) == ' ' && !block.contains(&up_cell) {
                block.push(up_cell);
                stack.push(up_cell);
            }
            let down_cell = curr + 10;
            if self.get_ch((cursor.0, cursor.1 + 1)) == ' ' && !block.contains(&down_cell) {
                block.push(down_cell);
                stack.push(down_cell);
            }
        }
        block
    }
}

#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn test_get_block_from_index() {
        let mut disp: DispField = DispField::new();
        disp.disp_arr[1 + 1][1] = ' ';
        disp.disp_arr[1][1 + 1] = ' ';
        disp.disp_arr[3][1 + 1] = ' ';
        let v = disp.get_block_from_index(11);
        println!("{:?}", v);
    }
}

pub fn get_display_coords(cursor: (usize, usize)) -> (u16, u16) {
    (cursor.0 as u16 + 1, cursor.1 as u16 + 1)
}
pub fn get_cell_index(cursor: (usize, usize)) -> usize {
    let cell_position = get_cell_coords(cursor);
    (cell_position.1 + 1) * 10 + cell_position.0 + 1
}
pub fn get_cursor_from_index(cell_index: usize) -> (usize, usize) {
    let cell_position = get_cell_coords_from_index(cell_index);
    get_cursor_from_cell_coords(cell_position)
}
pub fn get_cell_coords_from_index(cell_index: usize) -> (usize, usize) {
    (cell_index % 10 - 1, cell_index / 10 - 1)
}
fn get_cell_coords(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0.saturating_sub(1) / 2, cursor.1.saturating_sub(1) / 2)
}
fn get_cursor_from_cell_coords(cell_coords: (usize, usize)) -> (usize, usize) {
    (cell_coords.0 * 2 + 1, cell_coords.1 * 2 + 1)
}

fn cursor_left_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let cell_x = if cursor.0 % 2 == 1 { cell_x } else { cell_x + 1 };
    let (cursor_x, _) = get_cursor_from_cell_coords((cell_x.saturating_sub(1), cell_y));
    (cursor_x, cursor.1)
}
fn cursor_right_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let (cursor_x, _) = get_cursor_from_cell_coords((std::cmp::min(cell_x + 1, 8), cell_y));
    (cursor_x, cursor.1)
}
fn cursor_up_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let cell_y = if cursor.1 % 2 == 1 { cell_y } else { cell_y + 1 };
    let (_, cursor_y) = get_cursor_from_cell_coords((cell_x, cell_y.saturating_sub(1)));
    (cursor.0, cursor_y)
}
fn cursor_down_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let (_, cursor_y) = get_cursor_from_cell_coords((cell_x, std::cmp::min(cell_y + 1, 8)));
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
        assert_eq!(get_cell_coords_from_index(11), (0, 0));
        assert_eq!(get_cell_coords_from_index(99), (8, 8));
        assert_eq!(get_cell_coords_from_index(55), (4, 4));
        assert_eq!(get_cursor_from_index(11), (1, 1));
    }
    #[test]
    fn cursor_move() {
        assert_eq!(cursor_left_cell((3, 3)), (1, 3));
        assert_eq!(cursor_left_cell((2, 3)), (1, 3));
        assert_eq!(cursor_left_cell((1, 3)), (1, 3));
        assert_eq!(cursor_left_cell((4, 4)), (3, 4));
        assert_eq!(cursor_right_cell((1, 1)), (3, 1));
        assert_eq!(cursor_right_cell((2, 1)), (3, 1));
        assert_eq!(cursor_right_cell((16, 1)), (17, 1));
        assert_eq!(cursor_right_cell((17, 1)), (17, 1));
        assert_eq!(cursor_up_cell((1, 1)), (1, 1));
        assert_eq!(cursor_up_cell((1, 2)), (1, 1));
        assert_eq!(cursor_up_cell((1, 3)), (1, 1));
        assert_eq!(cursor_down_cell((1, 17)), (1, 17));
        assert_eq!(cursor_down_cell((1, 16)), (1, 17));
        assert_eq!(cursor_down_cell((1, 15)), (1, 17));
        assert_eq!(cursor_down_cell((2, 14)), (2, 15));
    }
    #[test]
    fn test_get_cell_coords() {
        assert_eq!(get_cell_coords((1, 1)), (0, 0));
        assert_eq!(get_cursor_from_cell_coords((0, 0)), (1, 1));
        assert_eq!(get_cell_coords((2, 1)), (0, 0));
        assert_eq!(get_cell_coords((3, 1)), (1, 0));
        assert_eq!(get_cursor_from_cell_coords((1, 0)), (3, 1));
        assert_eq!(get_cell_coords((3, 3)), (1, 1));
        assert_eq!(get_cursor_from_cell_coords((1, 1)), (3, 3));
        assert_eq!(get_cell_coords((7, 7)), (3, 3));
    }
    #[test]
    fn test_cursor_boundaries() {
        let mut disp: DispField = DispField::new();
        assert_eq!(get_cell_coords(disp.cursor), (0, 0));
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