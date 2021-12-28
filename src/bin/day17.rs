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

fn is_target_hit(xv: i64, yv: i64, min_x: i64, max_x: i64, min_y: i64, max_y: i64) -> Option<i64> {
    let mut x = 0;
    let mut y = 0;
    let mut x_vel = xv;
    let mut y_vel = yv;

    let mut max_y_trajectory_height = 0;
    loop {
        x += x_vel;
        y += y_vel;

        max_y_trajectory_height = std::cmp::max(max_y_trajectory_height, y);

        if x >= min_x && x <= max_x && y >= min_y && y <= max_y {
            return Some(max_y_trajectory_height);
        }

        if x > max_x || y < min_y {
            return None;
        }

        if x_vel > 0 {
            x_vel -= 1;
        }
        y_vel -= 1;
    }
}

fn part_1(input: String) -> i64 {
    let (min_x, max_x, min_y, max_y) = parse(input);

    // Determine minimum x velocity
    // Use sum-first-n-natural-numbers equation
    // min_x = n * (n + 1) / 2
    // Solve n aka velocity from the equation
    // n^2 + n - 2min_x = 0
    // n = (-1 +- sqrt(1 - 4 * 1 * -2min_x) ) / 2
    // n = (-1 +- sqrt(1 + 8min_x)) / 2
    let velocity_x_min = ((-1f64 + (1f64 + 8f64 * min_x as f64).sqrt()) / 2f64).ceil() as i64;
    let velocity_x_max = max_x;

    // Just too tired right now, Probably under this range.
    let velocity_y_min = 2 * min_y;
    let velocity_y_max = -2 * (min_y - 1);

    let mut max_y_trajectory_height = 0;

    for xv in velocity_x_min..velocity_x_max {
        for yv in velocity_y_min..=velocity_y_max {
            if let Some(y_hit) = is_target_hit(xv, yv, min_x, max_x, min_y, max_y) {
                if y_hit > max_y_trajectory_height {
                    max_y_trajectory_height = y_hit
                }
            }
        }
    }

    max_y_trajectory_height
}

fn part_2(input: String) -> i64 {
    let (min_x, max_x, min_y, max_y) = parse(input);

    // Determine minimum x velocity
    // Use sum-first-n-natural-numbers equation
    // min_x = n * (n + 1) / 2
    // Solve n aka velocity from the equation
    // n^2 + n - 2min_x = 0
    // n = (-1 +- sqrt(1 - 4 * 1 * -2min_x) ) / 2
    // n = (-1 +- sqrt(1 + 8min_x)) / 2
    let velocity_x_min = ((-1f64 + (1f64 + 8f64 * min_x as f64).sqrt()) / 2f64).ceil() as i64;
    let velocity_x_max = max_x;

    // Just too tired right now, Probably under this range.
    let velocity_y_min = 2 * min_y;
    let velocity_y_max = -2 * min_y;

    let mut hit_count = 0;

    for xv in velocity_x_min..=velocity_x_max {
        for yv in velocity_y_min..=velocity_y_max {
            if let Some(_y_hit) = is_target_hit(xv, yv, min_x, max_x, min_y, max_y) {
                hit_count += 1;
            }
        }
    }
    hit_count
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res1);

    let res2 = part_2(INPUT_FILE.to_string());
    println!("part2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solves_part1_example() {
        assert_eq!(part_1("target area: x=20..30, y=-10..-5 ".to_string()), 45);
    }

    #[test]
    fn solves_part1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 4186);
    }

    #[test]
    fn solves_part2_example() {
        assert_eq!(part_2("target area: x=20..30, y=-10..-5 ".to_string()), 112);
    }

    #[test]
    fn solves_part2_input() {
        assert_eq!(part_2(INPUT_FILE.to_string()), 2709);
    }
}
