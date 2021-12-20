use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy)]
enum Fold {
    Y(i64),
    X(i64),
}

const INPUT_FILE: &str = include_str!("../../inputs/day13.txt");

fn parse(input: String) -> (HashSet<(i64, i64)>, Vec<Fold>) {
    let mut parts: Vec<&str> = input.trim().split("\n\n").collect();

    let folds_raw = parts.pop().unwrap();
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

    let folds: Vec<Fold> = folds_raw
        .trim()
        .lines()
        .map(|s| s.split("fold along ").nth(1).unwrap().split("=").collect())
        .map(|c: Vec<&str>| {
            let axis: i64 = c[1].parse().unwrap();
            if c[0] == "y" {
                Fold::Y(axis)
            } else {
                Fold::X(axis)
            }
        })
        .collect();

    (dots, folds)
}

fn mirror_x(dots: HashSet<(i64, i64)>, axis: i64) -> HashSet<(i64, i64)> {
    let mut result: HashSet<(i64, i64)> = HashSet::new();
    for dot in dots {
        assert_ne!(dot.0, axis);
        if dot.0 < axis {
            result.insert(dot);
        } else {
            result.insert((axis - (dot.0 - axis), dot.1));
        }
    }
    result
}

fn mirror_y(dots: HashSet<(i64, i64)>, axis: i64) -> HashSet<(i64, i64)> {
    let mut result: HashSet<(i64, i64)> = HashSet::new();
    for dot in dots {
        assert_ne!(dot.1, axis);
        if dot.1 < axis {
            result.insert(dot);
        } else {
            result.insert((dot.0, axis - (dot.1 - axis)));
        }
    }
    result
}
fn part_1(input: String) -> i64 {
    let (dots, folds) = parse(input);
    println!("{:?}", dots);
    let mut x = dots.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let mut y = dots.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;
    let mut result = dots.clone();

    for fold in folds {
        match fold {
            Fold::Y(value) => {
                println!("mirroring y {}", value);
                result = mirror_y(result, value);
                y = value;
            }
            Fold::X(value) => {
                println!("mirroing x {}", value);
                result = mirror_x(result, value);
                x = value;
            }
        }

        println!("{:?} {:?}", x, y);
    }

    result.len() as i64
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
            17
        );
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 0);
    }
}
