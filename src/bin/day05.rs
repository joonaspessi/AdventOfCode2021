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

fn straight_hv_coordinate_points(p1: Point, p2: Point) -> Option<Vec<Point>> {
    if p1.0 != p2.0 && p1.1 != p2.1 {
        return None;
    }

    let x_min = cmp::min(p1.0, p2.0);
    let y_min = cmp::min(p1.1, p2.1);

    let x_max = cmp::max(p1.0, p2.0);
    let y_max = cmp::max(p1.1, p2.1);

    let mut points: Vec<Point> = Vec::new();

    for x in x_min..=x_max {
        for y in y_min..=y_max {
            points.push(Point(x, y));
        }
    }

    Some(points)
}

fn diagonal_hv_coordinate_points(p1: Point, p2: Point) -> Option<Vec<Point>> {
    if p1.0 == p2.0 || p1.1 == p2.1 {
        return None;
    }
    let (first_point, end_point) = if p1.0 < p2.0 { (p1, p2) } else { (p2, p1) };

    let mut points: Vec<Point> = Vec::new();

    let mut point = first_point.clone();
    points.push(point.clone());

    while point.0 != end_point.0 && point.1 != end_point.1 {
        if point.0 < end_point.0 {
            point = Point(point.0 + 1, point.1);
        }

        if point.1 < end_point.1 {
            point = Point(point.0, point.1 + 1);
        } else if point.1 > end_point.1 {
            point = Point(point.0, point.1 - 1);
        }

        points.push(point.clone());
    }

    Some(points)
}

fn calculate_overlapping_hv_points(points: HashMap<Point, i32>) -> usize {
    points.iter().filter(|(_point, &count)| count > 1).count()
}

fn dangerous_hv_points(input: String, allow_diagonal_hv: bool) -> usize {
    let mut line_points = HashMap::new();

    for (point_1, point_2) in parse(input) {
        if let Some(points) = straight_hv_coordinate_points(point_1, point_2) {
            for point in points {
                *line_points.entry(point).or_insert(0) += 1;
            }
        }

        if allow_diagonal_hv {
            if let Some(points) = diagonal_hv_coordinate_points(point_1, point_2) {
                for point in points {
                    *line_points.entry(point).or_insert(0) += 1;
                }
            }
        }
    }
    calculate_overlapping_hv_points(line_points)
}

fn part_1(file: String) -> usize {
    dangerous_hv_points(file, false)
}

fn part_2(file: String) -> usize {
    dangerous_hv_points(file, true)
}

fn main() {
    let part1_result = part_1(String::from(INPUT_FILE));
    println!("part_1: {}", part1_result);

    let part2_result = part_2(String::from(INPUT_FILE));
    println!("part_1: {}", part2_result)
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

    #[test]
    fn test_solves_part_2_example() {
        assert_eq!(
            part_2(String::from(
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
            12
        )
    }

    #[test]
    fn test_solves_part2_input() {
        assert_eq!(part_2(String::from(INPUT_FILE)), 16925);
    }
}
