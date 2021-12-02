const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

fn parse(input: String) -> Vec<usize> {
    input
        .trim()
        .split('\n')
        .map(|i| i.parse::<usize>().unwrap())
        .collect()
}

fn part_1(file: String) -> usize {
    parse(file).iter().count()
}

fn part_2(file: String) -> usize {
    parse(file).iter().count()
}

fn main() {
    let res1 = part_1(String::from(INPUT_FILE));
    println!("part 1: {}", res1);
    //let res2 = part_2(String::from(INPUT_FILE));
    //println!("part 2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1_example() {
        assert_eq!(part_1(String::from("1\n2\n3\n4\n5")), 5);
    }

    // #[test]
    // fn test_solves_part1_input() {
    //     assert_eq!(part_1(String::from(INPUT_FILE)), 0);
    // }

    // #[test]
    // fn test_solves_part2_example() {
    //     assert_eq!(
    //         part_2(String::from(
    //             "1\n2\n3\n4\n5"
    //         )),
    //         0
    //     );
    // }

    // #[test]
    // fn test_solves_part2_input() {
    //     assert_eq!(part_2(String::from(INPUT_FILE)), 0);
    // }
}
