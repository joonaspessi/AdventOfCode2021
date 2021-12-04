const INPUT_FILE: &str = include_str!("../../inputs/day04.txt");
#[derive(Debug)]
enum BingoCell {
    Checked(u32),
    UnChecked(u32),
}
#[derive(Debug)]
struct BingoBoard {
    board: Vec<Vec<BingoCell>>,
    board_size: usize,
    win_number: Option<u32>,
}

impl BingoBoard {
    fn play_number(&mut self, number: u32) {
        for i in 0..self.board.len() {
            for j in 0..self.board[i].len() {
                if let BingoCell::UnChecked(value) = self.board[i][j] {
                    if value == number {
                        self.board[i][j] = BingoCell::Checked(value)
                    }
                }
            }
        }
    }

    fn is_completed(&self) -> bool {
        if self.win_number.is_some() {
            return true;
        }
        false
    }

    fn complete_board(&mut self, win_number: u32) {
        self.win_number = Some(win_number);
    }

    fn check_win(&mut self, win_number: u32) -> Option<u32> {
        for i in 0..self.board.len() {
            let mut numbers = Vec::new();
            for j in 0..self.board[i].len() {
                if let BingoCell::Checked(value) = self.board[i][j] {
                    numbers.push(value);
                }
                if numbers.len() == self.board_size {
                    let win_number = self.calculate_win_number(win_number);
                    self.complete_board(win_number);
                    return Some(win_number);
                }
            }
        }

        for i in 0..self.board[0].len() {
            let mut numbers = Vec::new();
            for j in 0..self.board.len() {
                if let BingoCell::Checked(value) = self.board[j][i] {
                    numbers.push(value);
                }
                if numbers.len() == self.board_size {
                    let win_number = self.calculate_win_number(win_number);
                    self.complete_board(win_number);
                    return Some(win_number);
                }
            }
        }
        None
    }

    fn calculate_win_number(&self, win_number: u32) -> u32 {
        let empty_cell_values: u32 = self
            .board
            .iter()
            .flatten()
            .map(|i| {
                if let BingoCell::UnChecked(value) = i {
                    *value
                } else {
                    0
                }
            })
            .sum();
        empty_cell_values * win_number
    }
}

fn parse_bingo_table(input: String) -> BingoBoard {
    let board = input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .map(|line| {
            line.into_iter()
                .map(|i| i.parse::<u32>().unwrap())
                .map(BingoCell::UnChecked)
                .collect::<Vec<BingoCell>>()
        })
        .collect::<Vec<Vec<BingoCell>>>();
    let board_size: usize = board.len();
    BingoBoard {
        board,
        board_size,
        win_number: None,
    }
}

fn parse_bingo_numbers(input: String) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}

fn parse(input: String) -> (Vec<u32>, Vec<BingoBoard>) {
    let chunks = input.split("\n\n");

    let mut bingo_numbers = Vec::new();
    let mut bingo_tables = Vec::new();
    for (i, line) in chunks.into_iter().enumerate() {
        if i == 0 {
            bingo_numbers = parse_bingo_numbers(String::from(line));
        } else {
            bingo_tables.push(parse_bingo_table(String::from(line)));
        }
    }
    (bingo_numbers, bingo_tables)
}

fn part_1(file: String) -> u32 {
    let (bingo_numbers, mut bingo_tables) = parse(file);

    for number in &bingo_numbers {
        for table in &mut bingo_tables {
            table.play_number(*number);
            if let Some(result) = table.check_win(*number) {
                return result;
            }
        }
    }
    0
}

fn part_2(file: String) -> u32 {
    let (bingo_numbers, mut bingo_tables) = parse(file);

    let mut tables: Vec<u32> = Vec::new();
    for number in &bingo_numbers {
        for table in &mut bingo_tables {
            if table.is_completed() {
                continue;
            }

            table.play_number(*number);
            if let Some(result) = table.check_win(*number) {
                tables.push(result);
            }
        }
    }
    *tables.last().unwrap()
}

fn main() {
    let result_1 = part_1(String::from(INPUT_FILE));
    println!("part1: {}", result_1);

    let result_2 = part_2(String::from(INPUT_FILE));
    println!("part2: {}", result_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(
            part_1(String::from(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
2  0 12  3  7"
            )),
            4512
        );
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 63424);
    }

    #[test]
    fn test_solves_part_2_example() {
        assert_eq!(
            part_2(String::from(
                "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

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
2  0 12  3  7"
            )),
            1924
        );
    }

    #[test]
    fn test_solves_part_2_input() {
        assert_eq!(part_2(String::from(INPUT_FILE)), 23541);
    }
}
