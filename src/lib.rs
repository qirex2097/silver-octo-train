pub struct DispField {
    pub disp_arr: [[char; 19]; 19],
    pub cursor: (usize, usize),
}

impl DispField {
    pub fn new() -> DispField {
        let disp_arr: [[char; 19]; 19] = [
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

        DispField { disp_arr, cursor: (1, 1), }
    }
}

impl DispField {
    pub fn move_cursor_left(&mut self) {
        self.cursor = cursor_left(self.cursor);
    }
    pub fn move_cursor_right(&mut self) {
        self.cursor = cursor_right(self.cursor);
    }
    pub fn move_cursor_up(&mut self) {
        self.cursor = cursor_up(self.cursor);
    }
    pub fn move_cursor_down(&mut self) {
        self.cursor = cursor_down(self.cursor);
    }
}

pub fn get_cell_coords(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0.saturating_sub(1) / 2, cursor.1.saturating_sub(1) / 2)
}
fn get_cursor_from_cell_coords(cell_coords: (usize, usize)) -> (usize, usize) {
    (cell_coords.0 * 2 + 1, cell_coords.1 * 2 + 1)
}
pub fn cursor_left(cursor: (usize, usize)) -> (usize, usize) {
    (std::cmp::max(cursor.0 - 1, 1), cursor.1)
}
pub fn cursor_right(cursor: (usize, usize)) -> (usize, usize) {
    (std::cmp::min(cursor.0 + 1, 18 - 1), cursor.1)
}
pub fn cursor_up(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0, std::cmp::max(cursor.1 - 1, 1))
}
pub fn cursor_down(cursor: (usize, usize)) -> (usize, usize) {
    (cursor.0, std::cmp::min(cursor.1 + 1, 18 - 1))

}

pub fn cursor_left_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let cell_x = if cursor.0 % 2 == 1 { cell_x } else { cell_x + 1 };
    let (cursor_x, _) = get_cursor_from_cell_coords((cell_x.saturating_sub(1), cell_y));
    (cursor_x, cursor.1)
}
pub fn cursor_right_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let (cursor_x, _) = get_cursor_from_cell_coords((std::cmp::min(cell_x + 1, 8), cell_y));
    (cursor_x, cursor.1)
}
pub fn cursor_up_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let cell_y = if cursor.1 % 2 == 1 { cell_y } else { cell_y + 1 };
    let (_, cursor_y) = get_cursor_from_cell_coords((cell_x, cell_y.saturating_sub(1)));
    (cursor.0, cursor_y)
}
pub fn cursor_down_cell(cursor: (usize, usize)) -> (usize, usize) {
    let (cell_x, cell_y) = get_cell_coords(cursor);
    let (_, cursor_y) = get_cursor_from_cell_coords((cell_x, std::cmp::min(cell_y + 1, 8)));
    (cursor.0, cursor_y)
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
    fn it_works() {
        let mut disp: DispField = DispField::new();
        assert_eq!(get_cell_coords(disp.cursor), (0, 0));
        assert_eq!(disp.cursor, (1, 1));
        disp.cursor = cursor_left(disp.cursor);
        assert_eq!(disp.cursor, (1, 1));
        disp.cursor = cursor_right(disp.cursor);
        assert_eq!(disp.cursor, (2, 1));
        disp.cursor = cursor_up(disp.cursor);
        assert_eq!(disp.cursor, (2, 1));
        disp.cursor = cursor_down(disp.cursor);
        assert_eq!(disp.cursor, (2, 2));
        disp.cursor = cursor_left(disp.cursor);
        assert_eq!(disp.cursor, (1, 2));
        disp.cursor = cursor_up(disp.cursor);
        assert_eq!(disp.cursor, (1, 1));
    }
}