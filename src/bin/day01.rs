const INPUT_FILE: &str = include_str!("../../inputs/day01.txt");

fn input_to_numbers(input: String) -> Vec<usize> {
    input
        .trim()
        .split('\n')
        .map(|i| i.parse::<usize>().expect("parsing"))
        .collect()
}

fn part_1(file: String) -> usize {
    input_to_numbers(file)
        .windows(2)
        .filter(|values| values[0] < values[1])
        .count()
}

fn part_2(file: String) -> usize {
    input_to_numbers(file)
        .windows(3)
        .map(|values| values.iter().sum())
        .collect::<Vec<usize>>()
        .windows(2)
        .filter(|values| values[0] < values[1])
        .count()
}

fn main() {
    let res1 = part_1(String::from(INPUT_FILE));
    let res2 = part_2(String::from(INPUT_FILE));
    println!("part 1: {}", res1);
    println!("part 2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1_example() {
        assert_eq!(
            part_1(String::from(
                "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            )),
            7
        );
    }

    #[test]
    fn test_solves_part1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 1266);
    }

    #[test]
    fn test_solves_part2_example() {
        assert_eq!(
            part_2(String::from(
                "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
            )),
            5
        );
    }

    #[test]
    fn test_solves_part2_input() {
        assert_eq!(part_2(String::from(INPUT_FILE)), 1217);
    }
}
