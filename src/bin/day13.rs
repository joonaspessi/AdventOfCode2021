use std::collections::{HashMap, HashSet};

const INPUT_FILE: &str = include_str!("../../inputs/day13.txt");

fn parse(input: String) -> HashSet<(i64, i64)> {
    let mut parts: Vec<&str> = input.trim().split("\n\n").collect();

    let folds = parts.pop().unwrap();
    let dots_raw = parts.pop().unwrap();

    let dots_parsed: Vec<Vec<i64>> = dots_raw
        .trim()
        .lines()
        .map(|line| line.split(',').map(|c| c.parse().unwrap()).collect())
        .collect();

    let mut dots: HashSet<(i64, i64)> = HashSet::new();

    for dot in dots_parsed {
        dots.insert((dot[0], dot[1]));
    }

    println!("{:?}", dots);

    dots
}

fn part_1(input: String) -> usize {
    let dots = parse(input);
    1
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {:?}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(
            part_1(
                "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
                    .to_string()
            ),
            0
        );
    }
}
