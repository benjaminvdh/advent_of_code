use crate::day_4;
use crate::day_4::board::Board;
use crate::day_4::parsing::BingoInput;

pub fn compute_score(input: &[BingoInput]) -> u32 {
    let (draws, mut boards) = day_4::build_input(input);

    play_bingo(&draws, &mut boards)
}

fn play_bingo(draws: &[u8], boards: &mut [Board<5>]) -> u32 {
    let mut bingoes = 0;
    let num_boards = boards.len();

    for draw in draws {
        for board in boards.iter_mut() {
            if board.has_bingo() {
                continue;
            }

            board.call(*draw);

            if board.has_bingo() {
                bingoes += 1;
            }

            if bingoes == num_boards {
                return board.score() * *draw as u32;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bingo() {
        let (draws, mut boards) = day_4::tests::create_scenario();

        let result = play_bingo(&draws, &mut boards);

        assert_eq!(result, 1924);
    }
}
