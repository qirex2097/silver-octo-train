use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
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
    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
    pub fn get_width(&self) -> usize {
        let mut leftmost_cell = 9;
        let mut rightmost_cell = 1;
        for cell in &self.cells {
            leftmost_cell = std::cmp::min(cell % 10, leftmost_cell);
            rightmost_cell = std::cmp::max(cell % 10, rightmost_cell);
        }
        rightmost_cell - leftmost_cell + 1
    }
    pub fn get_height(&self) -> usize {
        let mut topmost_cell = 9;
        let mut bottommost_cell = 1;
        for cell in &self.cells {
            topmost_cell = std::cmp::min(cell / 10, topmost_cell);
            bottommost_cell = std::cmp::max(cell / 10, bottommost_cell);
        }
        bottommost_cell - topmost_cell + 1
    }
}

#[cfg(test)]
mod test_block {
    use super::*;
    #[test]
    fn test_get_width_and_get_height() {
        let block = Block {
            cells: vec![11, 12, 22],
            value: 20,
        };
        assert_eq!(block.get_width(), 2);
        assert_eq!(block.get_height(), 2);
        let block = Block {
            cells: vec![88, 96, 97, 98],
            value: 9,
        };
        assert_eq!(block.get_width(), 3);
        assert_eq!(block.get_height(), 2);
    }
}
