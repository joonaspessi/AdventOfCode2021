const INPUT_FILE : &str = include_str!("../../inputs/day10.txt");

fn parse(input : String) -> Vec<Vec<char>> {
    input.trim().lines().map(|line| line.trim().chars().collect()).collect()
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
            },
            ']' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '[' {
                        return 57;
                    }
                }                
            },
            '}' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '{' {
                        return 1197;
                    }
                }                
            },
            '>' => {
                if let Some(matching) = opening_stack.pop() {
                    if matching != '<' {
                        return 25137;
                    }
                }                
            },
            _ => panic!("Invalid character {:?}", parenthesis)

        };
    }
    0

}

fn part_1(input : String) -> usize {
    let navigation_code = parse(input);
    let mut result = 0;
    for line in navigation_code {
        result += validate_navigation_code_line(line);
    }
    result
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("{:?}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {

        assert_eq!(part_1(
        "[({(<(())[]>[[{[]{<()<>>\n\
         [(()[<>])]({[<{<<[]>>(\n\
         {([(<{}[<>[]}>{[]{[(<()>\n\
         (((({<>}<{<{<>}{[]{[]{}\n\
         [[<[([]))<([[{}[[()]]]\n\
         [{[{({}]{}}([{[{{{}}([]\n\
         {<[[]]>}<{[{[{[]{()[[[]\n\
         [<(<(<(<{}))><([]([]()\n\
         <{([([[(<>()){}]>(<<{{\n\
         <{([{{}}[<[[[<>{}]]]>[]]".to_string()), 26397)
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 362271);
    }
}
