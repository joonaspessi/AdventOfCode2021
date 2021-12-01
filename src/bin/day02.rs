const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

fn parse(input: String) -> Vec<usize> {
    input
        .trim()
        .split('\n')
        .map(|i| i.parse::<usize>().expect("parsing"))
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
    let res2 = part_2(String::from(INPUT_FILE));
    println!("part 1: {}", res1);
    println!("part 2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1() {
        assert_eq!(part_1(String::from("1")), 1);
    }

    #[test]
    fn test_solves_part2() {
        assert_eq!(part_2(String::from("1")), 1);
    }
}
