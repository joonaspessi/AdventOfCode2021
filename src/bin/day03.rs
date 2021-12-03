use std::convert::TryInto;
use std::iter::FromIterator;

const INPUT_FILE: &str = include_str!("../../inputs/day03.txt");

fn parse(input: String) -> Vec<Vec<u32>> {
    input
        .trim()
        .lines()
        //.map(|i| String::from(i))
        .map(|i| i.chars().map(|i| i.to_digit(2).unwrap()).collect())
        .collect()
}

fn pivot(data: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![vec![0; data.len()]; data[0].len()];
    for i in 0..data[0].len() {
        for j in 0..data.len() {
            result[i][j] = data[j][i];
        }
    }
    result
}

fn part_1(file: String) -> usize {
    let data = pivot(parse(file));
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();
    for line in data {
        let zeros = line.iter().filter(|&n| *n == 0).count();
        let ones = line.iter().filter(|&n| *n == 1).count();

        if ones > zeros {
            gamma.push("1");
            epsilon.push("0");
        } else {
            gamma.push("0");
            epsilon.push("1");
        }
    }
    let gamma_int = isize::from_str_radix(&gamma.join(""), 2).unwrap();
    let epsilon_int = isize::from_str_radix(&epsilon.join(""), 2).unwrap();
    ((gamma_int * epsilon_int) as usize).try_into().unwrap()
}

fn part_2(file: String) -> usize {
    let data = parse(file);

    0
}

fn main() {
    let res1 = part_1(String::from(INPUT_FILE));
    println!("part 1: {}", res1);
    //let res2 = part_2(String::from(INPUT_FILE));
    //println!("part 2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1_example() {
        assert_eq!(part_1(String::from("00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010")), 198);
    }

    // #[test]
    // fn test_solves_part1_input() {
    //     assert_eq!(part_1(String::from(INPUT_FILE)), 0);
    // }

    // #[test]
    // fn test_solves_part2_example() {
    //     assert_eq!(
    //         part_2(String::from(
    //             "1\n2\n3\n4\n5"
    //         )),
    //         0
    //     );
    // }

    // #[test]
    // fn test_solves_part2_input() {
    //     assert_eq!(part_2(String::from(INPUT_FILE)), 0);
    // }
}
