use itertools::Itertools;

const INPUT_FILE: &str = include_str!("../../inputs/day10.txt");

fn parse(input: String) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn validate_navigation_code_line(line: Vec<char>) -> usize {
    let mut opening_stack = Vec::new();
    for parenthesis in line {
        match parenthesis {
            '(' | '[' | '{' | '<' => opening_stack.push(parenthesis),
            ')' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '(' {
                        return 3;
                    }
                }
            }
            ']' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '[' {
                        return 57;
                    }
                }
            }
            '}' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '{' {
                        return 1197;
                    }
                }
            }
            '>' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '<' {
                        return 25137;
                    }
                }
            }
            _ => panic!("Invalid character {:?}", parenthesis),
        };
    }
    0
}

fn complete_line(line: Vec<char>) -> Vec<char> {
    let mut opening_stack = Vec::new();
    for parenthesis in line {
        match parenthesis {
            '(' | '[' | '{' | '<' => {
                opening_stack.push(parenthesis);
            }
            _ => {
                opening_stack.pop();
            }
        };
    }

    opening_stack
        .into_iter()
        .rev()
        .map(|parenthesis| match parenthesis {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("Unsupported character {:?}", parenthesis),
        })
        .collect()
}

fn calculate_score(completions: Vec<char>) -> usize {
    completions
        .iter()
        .map(|parenthesis| match parenthesis {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("Unsupported character {:?}", parenthesis),
        })
        .fold(0, |acc, point| acc * 5 + point)
}

fn part_1(input: String) -> usize {
    let navigation_code = parse(input);
    let mut result = 0;
    for line in navigation_code {
        result += validate_navigation_code_line(line);
    }
    result
}

fn part_2(input: String) -> usize {
    let navigation_code = parse(input);
    let mut scores = Vec::new();
    for line in navigation_code {
        if validate_navigation_code_line(line.clone()) == 0 {
            let completions = complete_line(line.clone());
            scores.push(calculate_score(completions.clone()));
        }
    }

    let index = scores.len() / 2;
    scores.into_iter().sorted().nth(index).unwrap()
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {:?}", res1);

    let res2 = part_2(INPUT_FILE.to_string());
    println!("part2: {:?}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(
            part_1(
                "[({(<(())[]>[[{[]{<()<>>\n\
         [(()[<>])]({[<{<<[]>>(\n\
         {([(<{}[<>[]}>{[]{[(<()>\n\
         (((({<>}<{<{<>}{[]{[]{}\n\
         [[<[([]))<([[{}[[()]]]\n\
         [{[{({}]{}}([{[{{{}}([]\n\
         {<[[]]>}<{[{[{[]{()[[[]\n\
         [<(<(<(<{}))><([]([]()\n\
         <{([([[(<>()){}]>(<<{{\n\
         <{([{{}}[<[[[<>{}]]]>[]]"
                    .to_string()
            ),
            26397
        );
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 362271);
    }

    #[test]
    fn test_solves_part_2_example() {
        assert_eq!(
            part_2(
                "[({(<(())[]>[[{[]{<()<>>\n\
         [(()[<>])]({[<{<<[]>>(\n\
         {([(<{}[<>[]}>{[]{[(<()>\n\
         (((({<>}<{<{<>}{[]{[]{}\n\
         [[<[([]))<([[{}[[()]]]\n\
         [{[{({}]{}}([{[{{{}}([]\n\
         {<[[]]>}<{[{[{[]{()[[[]\n\
         [<(<(<(<{}))><([]([]()\n\
         <{([([[(<>()){}]>(<<{{\n\
         <{([{{}}[<[[[<>{}]]]>[]]"
                    .to_string()
            ),
            288957
        );
    }

    #[test]
    fn test_solves_part_2_input() {
        assert_eq!(part_2(INPUT_FILE.to_string()), 1698395182);
    }
}
