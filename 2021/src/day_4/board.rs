#[derive(Debug, PartialEq)]
pub struct Board<const N: usize>(pub [[(u8, bool); N]; N]);

impl<const N: usize> Board<N> {
    pub fn new(values: &[u8]) -> Self {
        let mut value_iter = values.iter();

        let mut fields = [[(0, false); N]; N];

        for row in fields.iter_mut() {
            for col in row.iter_mut() {
                col.0 = *value_iter.next().unwrap_or(&0);
            }
        }

        Self(fields)
    }

    pub fn get_number(&self, row: usize, col: usize) -> u8 {
        self.0[row][col].0
    }

    pub fn is_called(&self, row: usize, col: usize) -> bool {
        self.0[row][col].1
    }

    pub fn call(&mut self, number: u8) {
        for row in &mut self.0 {
            for field in row {
                if field.0 == number {
                    field.1 = true;
                }
            }
        }
    }

    pub fn score(&self) -> u32 {
        let mut sum = 0u32;

        for row in 0..N {
            for col in 0..N {
                if !self.is_called(row, col) {
                    sum += self.get_number(row, col) as u32;
                }
            }
        }

        sum
    }

    pub fn has_bingo(&self) -> bool {
        self.any_row_has_bingo() || self.any_col_has_bingo()
    }

    fn any_row_has_bingo(&self) -> bool {
        (0..N).any(|row| (0..N).all(|col| self.is_called(row, col)))
    }

    fn any_col_has_bingo(&self) -> bool {
        (0..N).any(|col| (0..N).all(|row| self.is_called(row, col)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUES: [u8; 25] = [
        22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19,
    ];

    fn get_board() -> Board<5> {
        Board::<5>::new(&VALUES)
    }

    #[test]
    fn create_board() {
        let board = get_board();

        assert_eq!(board.get_number(0, 4), 0);
        assert_eq!(board.get_number(4, 0), 1);

        for row in 0..5 {
            for col in 0..5 {
                assert_eq!(board.get_number(row, col), VALUES[row * 5 + col]);
                assert_eq!(board.is_called(row, col), false);
            }
        }

        assert!(!board.has_bingo());
    }

    #[test]
    fn test_row_bingo() {
        let mut board = get_board();

        for draw in &[8, 2, 24, 4, 14] {
            board.call(*draw);
        }

        assert!(!board.has_bingo());

        board.call(23);
        assert!(board.has_bingo());

        board.call(21);
        assert!(board.has_bingo());
    }

    #[test]
    fn test_col_bingo() {
        let mut board = get_board();

        for draw in &[13, 2, 9, 10, 20] {
            board.call(*draw);
        }

        assert!(!board.has_bingo());

        board.call(12);
        assert!(board.has_bingo());

        board.call(7);
        assert!(board.has_bingo());
    }
}
