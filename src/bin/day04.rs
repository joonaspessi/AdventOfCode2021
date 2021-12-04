const INPUT_FILE: &str = include_str!("../../inputs/day04.txt");

fn parse_bingo_table(input: String) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .for_each(|i| println!("{:?}", i));
    input
        .trim()
        .lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .map(|line| {
            line.into_iter()
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}

fn parse_bingo_numbers(input: String) -> Vec<u32> {
    input
        .trim()
        .split(',')
        .into_iter()
        .map(|i| i.parse::<u32>().unwrap())
        .collect()
}

fn parse(input: String) {
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
}

fn part_1(file: String) -> u32 {
    let input = parse(file);
    0
}

fn main() {
    let result_1 = part_1(String::from(INPUT_FILE));
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
}
