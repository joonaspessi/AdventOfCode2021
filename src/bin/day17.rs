const INPUT_FILE: &str = include_str!("../../inputs/day17.txt");

fn parse(input: String) -> (i64, i64, i64, i64) {
    let (_start, xy) = input.trim().split_once(": ").unwrap();
    let (x_value, y_value) = xy.trim().split_once(", ").unwrap();
    let (x_min, x_max) = x_value[2..].split_once("..").unwrap();
    let (y_min, y_max) = y_value[2..].split_once("..").unwrap();

    (
        x_min.parse().unwrap(),
        x_max.parse().unwrap(),
        y_min.parse().unwrap(),
        y_max.parse().unwrap(),
    )
}

fn part_1(input: String) -> i64 {
    let (min_x, max_x, min_y, max_y) = parse(input);
    println!("{} {} {} {}", min_x, max_x, min_y, max_y);
    1
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part_1("target area: x=20..30, y=-10..-5 ".to_string()), 0);
    }
}
