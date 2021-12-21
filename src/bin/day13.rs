use std::collections::HashSet;

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
        .map(|s| s.split("fold along ").nth(1).unwrap().split('=').collect())
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

fn print_fold(dots: HashSet<(i64, i64)>, x_size: i64, y_size: i64) {
    for yy in 0..y_size {
        for xx in 0..x_size {
            if dots.contains(&(xx, yy)) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!()
    }
}
fn solver(input: String, part_1: bool) -> i64 {
    let (mut result, folds) = parse(input);
    let mut x_size = result.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let mut y_size = result.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    for fold in folds {
        match fold {
            Fold::Y(value) => {
                result = mirror_y(result, value);
                y_size = value;
            }
            Fold::X(value) => {
                result = mirror_x(result, value);
                x_size = value;
            }
        }

        if part_1 {
            break;
        }
    }

    print_fold(result.clone(), x_size, y_size);
    result.len() as i64
}

fn main() {
    let res1 = solver(INPUT_FILE.to_string(), true);
    println!("part1: {:?}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(
            solver(
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
                    .to_string(),
                true
            ),
            17
        );
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(solver(INPUT_FILE.to_string(), true), 675);
    }
    #[test]
    fn test_solves_part_2_example() {
        assert_eq!(
            solver(
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
                    .to_string(),
                false
            ),
            16
        );
    }

    #[test]
    fn test_solves_part_2_input() {
        assert_eq!(solver(INPUT_FILE.to_string(), false), 98);
    }
}
