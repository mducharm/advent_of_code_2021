use itertools::Itertools;

use crate::helper;

pub fn run(input_data: &[(&str, &str)]) -> anyhow::Result<()> {
    let data = parse_data(helper::get_file_data_by_name(input_data, "day4"))?;
    let data2 = parse_data(helper::get_file_data_by_name(input_data, "day4"))?;

    let answer_1 = get_score_of_winning_board(data);
    let answer_2 = get_last_winning_board_score(data2);

    dbg!(answer_1);
    dbg!(answer_2);

    Ok(())
}

fn parse_data(s: String) -> anyhow::Result<BingoGame> {
    let mut lines = s.lines();

    let numbers_to_draw = lines
        .next()
        .ok_or(helper::ParseError::Expected("expected nums to draw"))?
        .trim()
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap)
        .collect::<Vec<usize>>();

    let boards = lines
        .chunks(6)
        .into_iter()
        .map(|board_numbers| Board {
            board_numbers: board_numbers
                .skip(1)
                .collect_vec()
                .iter()
                .map(|n| {
                    n.trim()
                        .split_whitespace()
                        .map(|x| x.parse::<usize>())
                        .map(Result::unwrap)
                        .map(|value| BoardNumber::new(value, false))
                        .collect_vec()
                })
                .collect_vec(),
        })
        .collect_vec();


    Ok(BingoGame {
        numbers_to_draw,
        boards,
    })
}

struct BingoGame {
    numbers_to_draw: Vec<usize>,
    boards: Vec<Board>,
}

#[derive(Clone, Default, Debug)]
struct Board {
    board_numbers: Vec<Vec<BoardNumber>>,
}

impl Board {
    fn is_winning_board(&self) -> bool {
        // horizontal
        for row in &self.board_numbers {
            if row.iter().all(|b| b.marked) {
                return true;
            }
        }

        // vertical
        for i in 0..self.board_numbers.len() {
            let vertical_win_exists = self.board_numbers.iter().all(|row| {
                if let Some(x) = row.get(i) {
                    x.marked
                } else {
                    false
                }
            });

            if vertical_win_exists {
                return true;
            }
        }

        false
    }

    fn get_score(&self, winning_num: usize) -> usize {
        let mut sum = 0;

        for row in &self.board_numbers {
            for num in row {
                if !num.marked {
                    sum += num.value;
                }
            }
        }
        sum * winning_num
    }
}

#[derive(Clone, Copy, Default, Debug)]
struct BoardNumber {
    value: usize,
    marked: bool,
}
impl BoardNumber {
    pub fn new(value: usize, marked: bool) -> Self {
        Self { value, marked }
    }
}

fn mark_boards(boards: Vec<Board>, number: usize) -> Vec<Board> {
    boards
        .into_iter()
        .map(|b| -> Board {
            Board {
                board_numbers: b
                    .board_numbers
                    .into_iter()
                    .map(|row| {
                        row.into_iter()
                            .map(|num| {
                                BoardNumber::new(num.value, num.value == number || num.marked)
                            })
                            .collect_vec()
                    })
                    .collect_vec(),
            }
        })
        .collect_vec()
}

fn get_score_of_winning_board(game: BingoGame) -> usize {
    let mut boards = game.boards;

    for num in game.numbers_to_draw {
        boards = mark_boards(boards.clone(), num);

        let winning_board = boards.iter().find(|b| b.is_winning_board());

        
        if let Some(board) = winning_board {
            println!("{:#?}", winning_board);
            return board.get_score(num);
        }
    }
    0
}

fn get_last_winning_board_score(game: BingoGame) -> usize{ 
    let mut boards = game.boards;
    let mut winning_boards = vec![];

    for num in game.numbers_to_draw {
        boards = mark_boards(boards.clone(), num);

        let winning_board = boards.iter().find(|b| b.is_winning_board());

        if let Some(board) = winning_board {
            winning_boards.push((board.clone(), num));
            boards.retain(|b| !b.is_winning_board());
        }
    }

    if let Some((winning_board, winning_num)) = winning_boards.last() {
        return winning_board.get_score(*winning_num);
    }
    0 
}

#[cfg(test)]
mod tests {
    use crate::days::day4::{get_score_of_winning_board, parse_data, get_last_winning_board_score};

    const INPUT: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part_1() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT))?;

        let score = get_score_of_winning_board(data);

        assert_eq!(score, 4512);

        Ok(())
    }

    #[test]
    fn part_2() -> anyhow::Result<()> {
        let data = parse_data(String::from(INPUT))?;

        let score = get_last_winning_board_score(data);

        assert_eq!(score, 1924);
        Ok(())
    }
}
