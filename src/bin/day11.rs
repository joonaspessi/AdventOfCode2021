use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../../inputs/day11.txt");
const WIDTH: i64 = 10;
const HEIGHT: i64 = 10;

fn parse(input: String) -> HashMap<(i64, i64), u32> {
    let grid: Vec<Vec<u32>> = input
        .trim()
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut energy_levels = HashMap::new();

    for (y, column) in grid.into_iter().enumerate() {
        for (x, energy_level) in column.into_iter().enumerate() {
            energy_levels.insert((x as i64, y as i64), energy_level);
        }
    }

    energy_levels
}

fn print_energy_levels(energy_levels: &HashMap<(i64, i64), u32>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{:?}", energy_levels.get(&(x, y)).unwrap());
        }
        println!();
    }
    println!();
}

fn incerment_energy_levels(energy_levels: &mut HashMap<(i64, i64), u32>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            *energy_levels.get_mut(&(x, y)).unwrap() += 1;
        }
    }
}

fn flash_energy_levels(energy_levels: &mut HashMap<(i64, i64), u32>) -> usize {
    let mut flashed = HashMap::new();
    let mut is_dirty = true;

    while is_dirty {
        is_dirty = false;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if *energy_levels.get(&(x, y)).unwrap() > 9 && !flashed.contains_key(&(x, y)) {
                    flashed.insert((x, y), true);
                    is_dirty = true;
                    for a in -1..=1 {
                        for b in -1..=1 {
                            if let Some(val) = energy_levels.get_mut(&(x + a, y + b)) {
                                *val += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    for (x, y) in flashed.keys() {
        *energy_levels.get_mut(&(*x, *y)).unwrap() = 0;
    }
    flashed.len()
}

fn part_1(input: String) -> usize {
    let mut result = 0;
    let mut energy_levels = parse(input);
    println!("Before Any steps:");
    print_energy_levels(&energy_levels);
    for i in 1..=100 {
        incerment_energy_levels(&mut energy_levels);
        result += flash_energy_levels(&mut energy_levels);
        println!("After step {}", i);
        print_energy_levels(&energy_levels);
    }
    result
}

fn part_2(input: String) -> usize {
    let mut result = 0;
    let mut energy_levels = parse(input);
    println!("Before Any steps:");
    print_energy_levels(&energy_levels);
    for i in 1..=1000 {
        incerment_energy_levels(&mut energy_levels);
        let flash_count = flash_energy_levels(&mut energy_levels);
        if flash_count == 100 {
            result = i;
            break;
        }
        println!("After step {}", i);
        print_energy_levels(&energy_levels);
        if flash_count == 100 {
            result = i;
            break;
        }
    }
    result
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
    fn test_solves_part_1_example() {
        assert_eq!(
            part_1(
                "5483143223\n\
                 2745854711\n\
                 5264556173\n\
                 6141336146\n\
                 6357385478\n\
                 4167524645\n\
                 2176841721\n\
                 6882881134\n\
                 4846848554\n\
                 5283751526"
                    .to_string()
            ),
            1656
        );
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 1642);
    }

    #[test]
    fn test_solves_part_2_example() {
        assert_eq!(
            part_2(
                "5483143223\n\
                 2745854711\n\
                 5264556173\n\
                 6141336146\n\
                 6357385478\n\
                 4167524645\n\
                 2176841721\n\
                 6882881134\n\
                 4846848554\n\
                 5283751526"
                    .to_string()
            ),
            195
        );
    }

    #[test]
    fn test_solves_part_2_input() {
        assert_eq!(part_2(INPUT_FILE.to_string()), 320);
    }
}
