use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day07.txt");

fn parse(file: String) -> Vec<i32> {
    file.trim()
        .split(',')
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

fn part_1(file: String) -> i32 {
    let crab_positions = parse(file);
    let min = *crab_positions.iter().min().unwrap();
    let max = *crab_positions.iter().max().unwrap();

    let mut alignment_map = HashMap::new();

    for i in min..=max {
        for pos in &crab_positions {
            *alignment_map.entry(i).or_insert(0) += (pos - i).abs()
        }
    }

    *alignment_map
        .iter()
        .min_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
        .map(|(_k, v)| v)
        .unwrap()
}

fn fuel_consumption(position: i32, target: i32) -> i32 {
    let distance = (target - position).abs();

    if distance == 0 {
        return 0;
    }

    let mut result = 0;
    for i in 0..(target - position).abs() {
        result += i + 1;
    }
    result
}

fn part_2(file: String) -> i32 {
    let crab_positions = parse(file);
    let min = *crab_positions.iter().min().unwrap();
    let max = *crab_positions.iter().max().unwrap();

    let mut alignment_map = HashMap::new();

    for i in min..=max {
        for pos in &crab_positions {
            *alignment_map.entry(i).or_insert(0) += fuel_consumption(*pos, i)
        }
    }

    *alignment_map
        .iter()
        .min_by(|(_k1, v1), (_k2, v2)| v1.cmp(v2))
        .map(|(_k, v)| v)
        .unwrap()
}
fn main() {
    let res_1 = part_1(String::from(INPUT_FILE));
    println!("part1: {}", res_1);

    let res_2 = part_2(String::from(INPUT_FILE));
    println!("part2: {}", res_2)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1_example() {
        assert_eq!(part_1(String::from("16,1,2,0,4,2,7,1,2,14")), 37);
    }

    #[test]
    fn test_solves_part1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 335271)
    }

    #[test]
    fn test_solves_part2_example() {
        assert_eq!(part_2(String::from("16,1,2,0,4,2,7,1,2,14")), 168);
    }

    #[test]
    fn test_fuel_consumption() {
        assert_eq!(fuel_consumption(1, 5), 10);
    }

    #[test]
    fn test_solves_part2_input() {
        assert_eq!(part_2(String::from(INPUT_FILE)), 95851339);
    }
}
