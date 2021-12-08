const INPUT_FILE: &str = include_str!("../../inputs/day08.txt");

// Digit to signal line count
// 1 -> 2
// 4 -> 4
// 7 -> 3
// 8 -> 7

fn parse(file: String) -> Vec<(Vec<String>, Vec<String>)> {


    file.lines().map(|line| {
        let line_two_parts : Vec<&str> = line.split('|').map(|i| i.trim()).collect();
        let input_signals : Vec<String> = line_two_parts.first().unwrap().split_whitespace().map(String::from).collect();
        let output_digits : Vec<String> = line_two_parts.last().unwrap().split_whitespace().map(String::from).collect();
        return (input_signals, output_digits)
    }).collect()
    
}


fn part_1(file: String) -> usize {
    let signal_patterns = parse(file);

    let mut result: usize = 0;
    for (_input_signals, output_digits) in signal_patterns {
        println!("{:?}", output_digits);
        result += output_digits.iter().filter(|i| i.len() == 2 || i.len() == 3 ||i.len() == 4 || i.len() == 7).count();
    }
    result
}

fn main() {
    let res_1 = part_1(String::from(INPUT_FILE));
    println!("part1: {}", res_1);
}

#[cfg(test)] 
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(part_1(String::from("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")),26)
    }
}
