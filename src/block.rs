use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

impl Block {
    pub fn search_combination(&self) -> Vec<Vec<u8>> {
        let result1 = self.search_combination_for_sum();
        let result2 = self.search_combination_for_product();
        let result3 = self.search_combination_for_difference();
        let result4 = self.search_combination_for_division();
        let unique_elements: std::collections::HashSet<_> = [result1, result2, result3, result4]
            .concat()
            .drain(..)
            .collect();

        unique_elements.into_iter().collect()
    }
    fn search_combination_for_sum(&self) -> Vec<Vec<u8>> {
        let combinations = {
            let mut stack: Vec<(Vec<u8>, usize)> = Vec::new();
            let mut result: Vec<Vec<u8>> = Vec::new();

            stack.push((Vec::new(), 0));
            while let Some((combination, sum)) = stack.pop() {
                if sum == self.value && combination.len() == self.cells.len() {
                    result.push(combination);
                } else if combination.len() < self.cells.len() {
                    let max_digit = if let Some(&max) = combination.iter().min() {
                        max
                    } else {
                        9
                    };
                    for digit in 1..=max_digit as usize {
                        let mut combination = combination.clone();
                        combination.push(digit as u8);
                        stack.push((combination, sum + digit));
                    }
                }
            }

            result
        };

        self.find_candidate_combinations(&combinations)
    }
    fn search_combination_for_product(&self) -> Vec<Vec<u8>> {
        let combinations = {
            let mut stack: Vec<(Vec<u8>, usize)> = Vec::new();
            let mut result: Vec<Vec<u8>> = Vec::new();

            for digit in 1..=std::cmp::min(self.value, 9) {
                stack.push((vec![digit as u8], digit));
            }
            while let Some((mut combination, product)) = stack.pop() {
                if product == self.value {
                    while combination.len() < self.cells.len() {
                        combination.push(1);
                    }
                    result.push(combination);
                } else if product < self.value && combination.len() < self.cells.len() {
                    let max_digit = if let Some(&max) = combination.iter().min() {
                        max
                    } else {
                        9
                    };
                    for digit in 1..=max_digit {
                        if product * digit as usize <= self.value {
                            let mut combination = combination.clone();
                            combination.push(digit as u8);
                            stack.push((combination, product * digit as usize));
                        }
                    }
                }
            }
            result
        };

        self.find_candidate_combinations(&combinations)
    }
    fn search_combination_for_difference(&self) -> Vec<Vec<u8>> {
        if self.cells.len() != 2 {
            return vec![];
        }

        let combinations = {
            let mut result: Vec<Vec<u8>> = Vec::new();
            for i in 1..=8 {
                for j in i + 1..=9 {
                    if j - i == self.value {
                        result.push(vec![i as u8, j as u8]);
                    }
                }
            }
            result
        };

        self.find_candidate_combinations(&combinations)
    }
    fn search_combination_for_division(&self) -> Vec<Vec<u8>> {
        if self.cells.len() != 2 {
            return vec![];
        }

        let combinations = {
            let mut result: Vec<Vec<u8>> = Vec::new();
            for i in 1..=9 {
                for j in i..=9 {
                    if j / i == self.value && i * self.value == j {
                        result.push(vec![i as u8, j as u8]);
                    }
                }
            }
            result
        };

        self.find_candidate_combinations(&combinations)
    }
    fn find_candidate_combinations(&self, combinations: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut result: Vec<Vec<u8>> = Vec::new();
        for combination in combinations {
            'outer: for perm in combination.iter().permutations(combination.len()) {
                let perm: Vec<u8> = perm.into_iter().map(|x| *x).collect();
                let mut tate: Vec<HashSet<u8>> = vec![HashSet::new(); 10];
                let mut yoko: Vec<HashSet<u8>> = vec![HashSet::new(); 10];
                for (i, &cell) in self.cells.iter().enumerate() {
                    let (x, y) = (cell % 10, cell / 10);
                    let value = perm[i];
                    if tate[x].contains(&value) || yoko[y].contains(&value) {
                        continue 'outer;
                    }
                    tate[x].insert(value);
                    yoko[y].insert(value);
                }
                if !result.contains(&perm) {
                    result.push(perm);
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod test_block {
    use super::*;
    #[test]
    fn test_search_combination() {
        let block = Block {
            cells: vec![11, 12],
            value: 8,
        };
        let combi = block.search_combination();
        assert_eq!(combi.len(), 12);
    }
    #[test]
    fn test_search_combination_for_division() {
        let block = Block {
            cells: vec![11, 12],
            value: 3,
        };
        let combi = block.search_combination_for_division();
        assert_eq!(combi.len(), 6);
    }
    #[test]
    fn test_search_combination_for_difference() {
        let block = Block {
            cells: vec![11, 12],
            value: 1,
        };
        let combi = block.search_combination_for_difference();
        assert_eq!(combi.len(), 16);
    }
    #[test]
    fn test_search_combination_for_product() {
        let block = Block {
            cells: vec![11, 12],
            value: 1,
        };
        let combi = block.search_combination_for_product();
        assert_eq!(combi.len(), 0);

        let block = Block {
            cells: vec![11, 12],
            value: 2,
        };
        let combi = block.search_combination_for_product();
        assert_eq!(combi.len(), 2);

        let block = Block {
            cells: vec![11, 21, 31],
            value: 12,
        };
        let combi = block.search_combination_for_product();
        assert_eq!(combi.len(), 12);

        let block = Block {
            cells: vec![11, 21, 22, 32],
            value: 24,
        };
        let combi = block.search_combination_for_product();
        assert_eq!(combi.len(), 42);
    }
    #[test]
    fn test_search_combination_for_sum() {
        let block = Block {
            cells: vec![11, 12, 22],
            value: 20,
        };
        let combi = block.search_combination_for_sum();
        assert_eq!(combi.len(), 28);

        let block = Block {
            cells: vec![11, 12],
            value: 1,
        };
        let combi = block.search_combination_for_sum();
        assert_eq!(combi.len(), 0);

        let block = Block {
            cells: vec![67, 68, 69, 78],
            value: 11,
        };
        let combi = block.search_combination_for_sum();
        assert_eq!(combi.len(), 52);

        let block = Block {
            cells: vec![11, 12, 22],
            value: 21,
        };
        let combi = block.search_combination_for_sum();
        assert_eq!(combi.len(), 21);
    }
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
