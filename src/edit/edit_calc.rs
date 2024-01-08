use crate::edit::*;
use termion::{clear, cursor};

use crate::edit::block::Block;

pub struct EditStateCalc {
    is_redraw: bool,
    blocks_and_values: Vec<Vec<(usize, u8)>>,
    element_no_of_blocks: Vec<usize>,
    combinations: Vec<Vec<Vec<u8>>>,
    block_no: usize,
}
impl EditStateCalc {
    pub fn new() -> Self {
        EditStateCalc {
            is_redraw: true,
            blocks_and_values: vec![],
            element_no_of_blocks: vec![],
            combinations: vec![],
            block_no: 0,
        }
    }
}
impl EditState for EditStateCalc {
    fn update(&mut self, data: &mut EditData, key_opt: Option<Key>) -> Option<Box<dyn EditState>> {
        if let Some(Key::Char('q')) = key_opt {
            return Some(Box::new(EditStateEdit::new()));
        }
        if self.block_no >= data.disp.blocks.len() {
            // 完成した
            return None;
            // return Some(Box::new(EditStateTerminal));
        }
        if self.combinations[self.block_no].len() == 0 {
            // combinatinos をまだ生成していない。
            self.combinations[self.block_no] = data.disp.blocks[self.block_no].search_combination();
        }

        if self.element_no_of_blocks[self.block_no] >= self.combinations[self.block_no].len() {
            // 当てはまる values がなかった
            self.blocks_and_values.pop();
            self.block_no -= 1;
            self.element_no_of_blocks[self.block_no] += 1;
        } else if is_block_placement_valid(
            &self.blocks_and_values,
            &data.disp.blocks[self.block_no],
            &self.combinations[self.block_no][self.element_no_of_blocks[self.block_no]],
        ) {
            self.blocks_and_values.push(set_block_value(
                &data.disp.blocks[self.block_no],
                &self.combinations[self.block_no][self.element_no_of_blocks[self.block_no]],
            ));
            self.block_no += 1;
            self.element_no_of_blocks[self.block_no] = 0;
        } else {
            self.element_no_of_blocks[self.block_no] += 1;
        }
        None
    }
    fn draw(&mut self, data: &mut EditData) -> String {
        let mut moji = String::from("");
        if self.is_redraw {
            self.is_redraw = false;
            let board_moji = data.disp.get_board_moji((1, 1));
            moji.push_str(&board_moji);
        }
        moji.push_str(&format!("{}{}>", cursor::Goto(1, 20), self.block_no));
        for i in 0..=self.block_no {
            moji.push_str(&format!(
                " {}({})",
                self.element_no_of_blocks[i],
                self.combinations[i].len(),
            ));
        }
        moji.push_str(&format!("{}", clear::UntilNewline));
        moji
    }
    fn initialize(&mut self, data: &mut EditData) {
        let block_kazu = data.disp.blocks.len();
        self.element_no_of_blocks = vec![0; block_kazu];
        self.combinations = vec![vec![]; block_kazu];
        self.block_no = 0;
        self.combinations[self.block_no] = data.disp.blocks[self.block_no].search_combination();

        self.element_no_of_blocks[self.block_no] = 19;
    }
}

fn is_block_placement_valid(
    blocks_and_values: &Vec<Vec<(usize, u8)>>,
    block: &Block,
    values: &Vec<u8>,
) -> bool {
    let mut cells: Vec<usize> = Vec::new();
    let mut tate_arr: [Vec<u8>; 10] = Default::default();
    let mut yoko_arr: [Vec<u8>; 10] = Default::default();
    for block in blocks_and_values {
        for (cell, value) in block {
            let tate = *cell % 10;
            let yoko = (*cell / 10) % 10;
            cells.push(*cell);
            tate_arr[tate].push(*value);
            yoko_arr[yoko].push(*value);
        }
    }
    for i in 0..block.cells.len() {
        let cell = block.cells[i];
        let value = values[i];
        if cells.contains(&cell)
            || tate_arr[cell % 10].contains(&value)
            || yoko_arr[cell / 10].contains(&value)
        {
            return false;
        }
    }
    true
}

fn set_block_value(block: &Block, value: &Vec<u8>) -> Vec<(usize, u8)> {
    let mut result: Vec<(usize, u8)> = Vec::new();
    for (&b, &v) in block.cells.iter().zip(value.iter()) {
        result.push((b, v));
    }
    result
}

#[cfg(test)]
mod test {
    use super::is_block_placement_valid;
    use super::Block;

    #[test]
    fn test_is_block_placement_valid() {
        let mut blocks_and_values: Vec<Vec<(usize, u8)>> = vec![];
        blocks_and_values.push(vec![(11, 1), (12, 2), (21, 3)]);

        let block: Block = Block {
            cells: vec![13, 14],
            value: 12,
        };
        let values: Vec<u8> = vec![3, 4];
        assert!(is_block_placement_valid(
            &blocks_and_values,
            &block,
            &values
        ));
        let values: Vec<u8> = vec![1, 2];
        assert!(!is_block_placement_valid(
            &blocks_and_values,
            &block,
            &values
        ));
    }
}

mod test2 {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test_test() {
        let (_, mut data) = super::edit_init();
        match super::read_file(&mut data.disp) {
            Err(_) => panic!("couldn't read_file"),
            Ok(_) => {}
        }
        println!("blocks.len() = {}", data.disp.blocks.len());

        let mut state: EditStateCalc = EditStateCalc::new();
        state.initialize(&mut data);
        for _ in 0..=100 {
            state.update(&mut data, None);
            print!("{}: ", state.block_no);
            for i in 0..=state.block_no {
                print!("{} ", state.element_no_of_blocks[i]);
            }
            println!();
        }
    }
}
