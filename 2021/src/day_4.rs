pub mod part_1;
pub mod part_2;

pub mod parsing;

pub use parsing::Parser;

mod board;

use board::Board;
use parsing::BingoInput;

pub fn print_success_message(score: u32) {
    println!("The score of the winning board is {}.", score);
}

pub fn print_error_message(error: crate::InputError) {
    eprintln!("Failed to solve the problem: {}.", error);
}

fn build_input(input: &[BingoInput]) -> (Vec<u8>, Vec<Board<5>>) {
    let mut draws = vec![];
    let mut rows = vec![];

    for component in input {
        match component {
            BingoInput::Draws(d) => draws.extend_from_slice(d),
            BingoInput::Row(r) => rows.extend_from_slice(r),
            BingoInput::Empty => (),
        }
    }

    let mut boards = vec![];

    for fields in rows.chunks(25) {
        boards.push(Board::<5>::new(&fields));
    }

    (draws, boards)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_DRAWS: [u8; 27] = [
        7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8, 19, 3,
        26, 1,
    ];

    pub fn create_scenario() -> (Vec<u8>, Vec<Board<5>>) {
        let input = [
            BingoInput::Draws(INPUT_DRAWS.iter().copied().collect()),
            BingoInput::Row(vec![22, 13, 17, 11, 0]),
            BingoInput::Row(vec![8, 2, 23, 4, 24]),
            BingoInput::Row(vec![21, 9, 14, 16, 7]),
            BingoInput::Row(vec![6, 10, 3, 18, 5]),
            BingoInput::Row(vec![1, 12, 20, 15, 19]),
            BingoInput::Empty,
            BingoInput::Row(vec![3, 15, 0, 2, 22]),
            BingoInput::Row(vec![9, 18, 13, 17, 5]),
            BingoInput::Row(vec![19, 8, 7, 25, 23]),
            BingoInput::Row(vec![20, 11, 10, 24, 4]),
            BingoInput::Row(vec![14, 21, 16, 12, 6]),
            BingoInput::Empty,
            BingoInput::Row(vec![14, 21, 17, 24, 4]),
            BingoInput::Row(vec![10, 16, 15, 9, 19]),
            BingoInput::Row(vec![18, 8, 23, 26, 20]),
            BingoInput::Row(vec![22, 11, 13, 6, 5]),
            BingoInput::Row(vec![2, 0, 12, 3, 7]),
        ];

        build_input(&input)
    }

    #[test]
    fn test_scenario() {
        let (draws, boards) = create_scenario();

        assert_eq!(&draws, &INPUT_DRAWS);

        assert_eq!(
            &boards,
            &[
                Board::<5>::new(&[
                    22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12,
                    20, 15, 19
                ]),
                Board::<5>::new(&[
                    3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21,
                    16, 12, 6
                ]),
                Board::<5>::new(&[
                    14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2,
                    0, 12, 3, 7
                ])
            ]
        );
    }
}
