pub const GRID_SIZE: usize = 19;
pub const CURSOR_MIN: usize = 1;
pub const CURSOR_MAX: usize = 17;

pub type DispArray = [[char; GRID_SIZE]; GRID_SIZE];
#[derive(Clone)]
pub struct Block {
    pub cells: Vec<usize>,
    pub value: usize,
}
#[derive(Clone)]
pub struct DispField {
    pub disp_arr: DispArray,
    pub blocks: Vec<Block>,
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

        DispField { disp_arr, blocks: vec![], }
    }
}

impl DispField {
    pub fn get_disp_arr(&self) -> &[[char; GRID_SIZE]; GRID_SIZE] {
        &self.disp_arr
    }
    pub fn get_ch(&self, pos: (usize, usize)) -> Option<char> {
        if pos.0 >= GRID_SIZE || pos.1 >= GRID_SIZE {
            return None;
        }
        Some(self.disp_arr[pos.1][pos.0])
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
    fn get_block_from_index(&self, cell_index: usize) -> Vec<usize> {
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
    fn push_block_from_index(&mut self, cell_index: usize) -> Option<&Block> {
        let v = self.get_block_from_index(cell_index);
        if v.len() >= 2 {
            self.blocks.push(Block { cells: v, value: 0 });
            self.blocks.last()
        } else {
            None
        }
    }
    pub fn get_block_from_cursor(&mut self, cursor: (usize, usize)) -> Option<usize> {
        let cell_index = get_cell_index(cursor);
        if let Some(idx) = self.blocks.iter().position(|block| block.cells.contains(&cell_index)) {
            Some(idx)
        } else {
            match self.get_ch(cursor) {
                Some(ch) if ch == ' ' => {
                    if let Some(_) = self.push_block_from_index(cell_index) {
                        Some(self.blocks.len() - 1)
                    } else {
                        None
                    }
                }
                _ => None
            }
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

    pub fn set_block_value(&mut self, block_no: usize, value: usize) {
        if self.blocks.len() <= block_no {
            return;
        }
        self.blocks[block_no].value = value;
    }

}

pub fn get_display_coords(cursor: (usize, usize)) -> (u16, u16) {
    (cursor.0 as u16 + 1, cursor.1 as u16 + 1)
}
fn get_cell_coords(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0.saturating_sub(1) / 2 + 1, cursor.1.saturating_sub(1) / 2 + 1)
}
fn get_cell_index(cursor: (usize, usize)) -> usize {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    cell_y * 10 + cell_x
}
fn get_cursor_from_index(cell_index: usize) -> (usize, usize) {
    get_cursor_from_cell_coords(get_cell_coords_from_index(cell_index))
}
fn get_cell_coords_from_index(cell_index: usize) -> (usize, usize) {
    (cell_index % 10, cell_index / 10)
}
fn get_cursor_from_cell_coords(cell_coords: (usize, usize)) -> (usize, usize) {
    ((cell_coords.0 - 1) * 2 + 1, (cell_coords.1 - 1) * 2 + 1)
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
        assert_eq!(disp.blocks[v.unwrap()].cells, [11, 12, 21, 22]);
        let v = disp.get_block_from_index(99);
        assert_eq!(v, [99]);
        let cursor = (17, 17);
        disp.remove_wall_cursor((cursor.0, cursor.1 - 1));
        let v = disp.get_block_from_index(99);
        assert_eq!(v, [89,99]);
    }
}
