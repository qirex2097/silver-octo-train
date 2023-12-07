pub const GRID_SIZE: usize = 19;
pub const CURSOR_MIN: usize = 1;
pub const CURSOR_MAX: usize = 17;

pub type DispArray = [[char; GRID_SIZE]; GRID_SIZE];
#[derive(Clone)]
pub struct Block {
    pub cells: Vec<usize>,
    pub value: usize,
}

impl Block {
    pub fn push_cells(&mut self, cells: &Vec<usize>) {
        self.cells.extend(cells);
        self.cells.sort();
    }
    pub fn push_cell(&mut self, cell_index: usize) {
        self.push_cells(&vec![cell_index]);
    }
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
    pub fn toggle_wall_onoff_at_cursor(&mut self, cursor: (usize, usize)) {
        let (x, y) = cursor;
        let ch = self.disp_arr[y][x];
        self.disp_arr[y][x] = match (x % 2, y % 2) {
            (1, 0) => { if ch == ' '  { '-' } else { ' ' } }
            (0, 1) => { if ch == ' '  { '|' } else { ' ' } }
            _ => { ch }
        };
        let (cell_0, cell_1) = match (x % 2, y % 2) {
            (1, 0) => (get_cell_index((x, y - 1)), get_cell_index((x, y + 1))),
            (0, 1) => (get_cell_index((x - 1, y)), get_cell_index((x + 1, y))),
            _ => { return; }
        };
        if self.disp_arr[y][x] == ' ' {
            match (self.get_block_from_index(cell_0), self.get_block_from_index(cell_1)) {
                (None, None) => { self.generate_block(cell_0); },
                (Some(block_0_index), Some(block_1_index)) if block_0_index != block_1_index => {
                    self.merge_block(block_0_index, block_1_index);
                },
                (Some(block_no), None) => { self.blocks[block_no].push_cell(cell_1); },
                (None, Some(block_no)) => { self.blocks[block_no].push_cell(cell_0); },
                _ => {}
            }
        } else if let Some(block_index) = self.get_block_from_index(cell_0) {
            self.split_block(block_index, cell_0, cell_1);
        } else if let Some(block_index) = self.get_block_from_index(cell_1) {
            self.split_block(block_index, cell_0, cell_1);
        }
    }
    fn generate_block(&mut self, cell: usize) {
        self.push_block_from_index(cell);
    }
    fn merge_block(&mut self, block_0_index: usize, block_1_index: usize) {
        let cells:  Vec<usize> = self.blocks[block_1_index].cells.clone();
        self.blocks[block_0_index].push_cells(&cells);
        self.blocks.remove(block_1_index);

    }
    fn split_block(&mut self, block_index: usize, cell_0: usize, cell_1: usize) {
        self.blocks.remove(block_index);
        self.push_block_from_index(cell_0);
        self.push_block_from_index(cell_1);
    }
}

impl DispField {
    fn get_cells_from_index(&self, cell_index: usize) -> Vec<usize> {
        let mut block: Vec<usize> = vec![cell_index];
        let mut stack: Vec<usize> = vec![cell_index];
        while let Some(curr) = stack.pop() {
            if let Some(left_cell) = self.get_connected_left_cell(curr) {
                if !block.contains(&left_cell) {
                    block.push(left_cell);
                    stack.push(left_cell);
                }
            }
            if let Some(right_cell) = self.get_connected_right_cell(curr) {
                if !block.contains(&right_cell) {
                    block.push(right_cell);
                    stack.push(right_cell);
                }
            }
            if let Some(up_cell) = self.get_connected_up_cell(curr) {
                if !block.contains(&up_cell) {
                    block.push(up_cell);
                    stack.push(up_cell);
                }
            }
            if let Some(down_cell) = self.get_connected_down_cell(curr) {
                if !block.contains(&down_cell) {
                    block.push(down_cell);
                    stack.push(down_cell);
                }
            }
        }
        block.sort();
        block
    }
    fn push_block_from_index(&mut self, cell_index: usize) -> Option<&Block> {
        let v = self.get_cells_from_index(cell_index);
        if v.len() >= 2 && self.get_block_from_index(cell_index).is_none() {
            self.blocks.push(Block { cells: v, value: 0 });
            self.blocks.last()
        } else {
            None
        }
    }
    pub fn get_block_from_index(&self, cell_index: usize) -> Option<usize> {
        if let Some(block_no) = self.blocks.iter().position(|block| block.cells.contains(&cell_index)) {
            Some(block_no)
        } else {
            None
        }
    }

    fn get_connected_left_cell(&self, cell_index: usize) -> Option<usize> {
        let cursor = get_cursor_from_index(cell_index);
        match self.get_ch((cursor.0 - 1, cursor.1)) {
            Some(' ') => { Some(cell_index - 1) }
            _ => { None }
        }
    }
    fn get_connected_right_cell(&self, cell_index: usize) -> Option<usize> {
        let cursor = get_cursor_from_index(cell_index);
        if let Some(' ') = self.get_ch((cursor.0 + 1, cursor.1)) {
            Some(cell_index + 1)
        } else {
            None
        }
    }
    fn get_connected_up_cell(&self, cell_index: usize) -> Option<usize> {
        let cursor = get_cursor_from_index(cell_index);
        if let Some(' ') = self.get_ch((cursor.0, cursor.1 - 1)) {
            Some(cell_index - 10)
        } else {
            None
        }
    }
    fn get_connected_down_cell(&self, cell_index: usize) -> Option<usize> {
        let cursor = get_cursor_from_index(cell_index);
        if let Some(' ') = self.get_ch((cursor.0, cursor.1 + 1)) {
            Some(cell_index + 10)
        } else {
            None
        }
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
pub fn get_cell_index(cursor: (usize, usize)) -> usize {
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


#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn test_get_cells_from_index() {
        let mut disp: DispField = DispField::new();
        let v = disp.get_cells_from_index(11);
        assert_eq!(v, [11]);
        disp.toggle_wall_onoff_at_cursor((2, 1));
        disp.toggle_wall_onoff_at_cursor((1, 2));
        disp.toggle_wall_onoff_at_cursor((3, 2));
        let v = disp.get_cells_from_index(11);
        assert_eq!(v, [11, 12, 21, 22]);
        let cell_index = get_cell_index((1, 1));
        assert_eq!(cell_index, 11);
        let v = disp.get_block_from_index(cell_index);
        assert_eq!(disp.blocks[v.unwrap()].cells, [11, 12, 21, 22]);
        let v = disp.get_cells_from_index(99);
        assert_eq!(v, [99]);
        let cursor = (17, 17);
        disp.disp_arr[cursor.1 - 1][cursor.0] = ' ';
        let v = disp.get_cells_from_index(99);
        assert_eq!(v, [89,99]);
    }
    #[test]
    fn test_get_connected_cell() {
        let mut disp: DispField = DispField::new();
        assert_eq!(disp.get_connected_left_cell(11), None);
        assert_eq!(disp.get_connected_left_cell(12), None);
        disp.disp_arr[1][2] = ' ';
        assert_eq!(disp.get_connected_left_cell(12), Some(11));
    }
}
