use std::cmp;
use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day05.txt");

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point(u32, u32);

fn parse(file: String) -> Vec<(Point, Point)> {
    file.lines()
        .map(|line| line.trim().split_whitespace().collect::<Vec<&str>>())
        .map(|i| {
            let start_point: Vec<u32> = i
                .first()
                .unwrap()
                .trim()
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect();

            let end_point: Vec<u32> = i
                .last()
                .unwrap()
                .trim()
                .split(",")
                .map(|c| c.parse::<u32>().unwrap())
                .collect();

            (
                Point(start_point[0], start_point[1]),
                Point(end_point[0], end_point[1]),
            )
        })
        .collect()
}

fn calculate_straight_line_coordinate_points(p1: Point, p2: Point) -> Option<Vec<Point>> {
    if p1.0 != p2.0 && p1.1 != p2.1 {
        return None;
    }
    // X points
    let x_min = cmp::min(p1.0, p2.0);
    let x_max = cmp::max(p1.0, p2.0);
    // Y
    let y_min = cmp::min(p1.1, p2.1);
    let y_max = cmp::max(p1.1, p2.1);

    let mut points: Vec<Point> = Vec::new();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            points.push(Point(x, y));
        }
    }

    Some(points)
}

fn part_1(file: String) -> usize {
    let coord_pairs = parse(file);
    let mut lines = HashMap::new();
    for c_pair in coord_pairs {
        if let Some(points) = calculate_straight_line_coordinate_points(c_pair.0, c_pair.1) {
            for point in points {
                *lines.entry(point).or_insert(0) += 1;
            }
        }
    }
    let result: usize = lines.iter().filter(|(_point, &count)| count > 1).count();
    result
}

fn main() {
    let part1_result = part_1(String::from(INPUT_FILE));
    println!("part_1: {}", part1_result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(
            part_1(String::from(
                "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            )),
            5
        )
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 5147);
    }
}
