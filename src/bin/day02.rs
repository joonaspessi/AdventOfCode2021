const INPUT_FILE: &str = include_str!("../../inputs/day02.txt");

#[derive(Debug)]
enum Command {
    FORWARD(usize),
    DOWN(usize),
    UP(usize),
}

fn parse(input: String) -> Vec<Command> {
    input
        .trim()
        .split('\n')
        .map(|i| i.trim())
        .map(|i| i.split(" ").collect::<Vec<&str>>())
        .map(|i| {
            assert_eq!(i.len(), 2);
            let command = i[0];
            let amount: usize = i[1].parse().unwrap();
            match command {
                "forward" => Command::FORWARD(amount),
                "down" => Command::DOWN(amount),
                "up" => Command::UP(amount),
                _ => panic!("Cannot parse command"),
            }
        })
        .collect::<Vec<Command>>()
}

fn part_1(file: String) -> usize {
    let mut position: usize = 0;
    let mut depth: usize = 0;
    for command in parse(file).iter() {
        match command {
            Command::FORWARD(amount) => position += amount,
            Command::DOWN(amount) => depth += amount,
            Command::UP(amount) => depth -= amount,
        }
    }
    position * depth
}

fn part_2(file: String) -> usize {
    let mut position: usize = 0;
    let mut depth: usize = 0;
    let mut aim: usize = 0;
    for command in parse(file).iter() {
        match command {
            Command::FORWARD(amount) => {
                position += amount;
                depth += aim * amount
            }
            Command::DOWN(amount) => aim += amount,
            Command::UP(amount) => aim -= amount,
        }
    }
    position * depth
}

fn main() {
    let res1 = part_1(String::from(INPUT_FILE));
    println!("part 1: {}", res1);
    let res2 = part_2(String::from(INPUT_FILE));
    println!("part 2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1_example() {
        assert_eq!(
            part_1(String::from(
                "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"
            )),
            150
        );
    }

    #[test]
    fn test_solves_part1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 1714680);
    }

    #[test]
    fn test_solves_part2_example() {
        assert_eq!(
            part_2(String::from(
                "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2"
            )),
            900
        );
    }

    #[test]
    fn test_solves_part2_input() {
        assert_eq!(part_2(String::from(INPUT_FILE)), 1963088820);
    }
}
