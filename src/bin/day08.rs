use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../../inputs/day08.txt");

const DIGITS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

// 0: abcefg (6)
// 1: cf (2)
// 2: acdeg (4)
// 3: acdfg (4)
// 4: bcdf (3)
// 5: abdfg (6)
// 6: abdefg (6)
// 7: acf (3)
// 8: abcdefg (7)
// 9: abcdfg (6)

// 4: bcdf (3)
// 1: cf (2)
// 7: acf (3)
// 8: abcdefg (7)

// 2: acdeg (5)
// 3: acdfg (5)
// 5: abdfg (5)

// 0: abcefg (6)
// 6: abdefg (6)
// 9: abcdfg (6)

fn parse(file: String) -> Vec<(Vec<String>, Vec<String>)> {
    file.lines()
        .map(|line| {
            let line_two_parts: Vec<&str> = line.split('|').map(|i| i.trim()).collect();
            let input_signals: Vec<String> = line_two_parts
                .first()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect();
            let output_digits: Vec<String> = line_two_parts
                .last()
                .unwrap()
                .split_whitespace()
                .map(String::from)
                .collect();
            (input_signals, output_digits)
        })
        .collect()
}

fn parse_2(file: String) -> Vec<(Vec<HashSet<char>>, Vec<String>)> {
    file.lines()
        .map(|line| {
            let mut line_two_parts = line.split('|');
            let input_signals = line_two_parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|i| i.chars().collect())
                .collect();
            let output_digits: Vec<String> = line_two_parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|i| i.chars().sorted().collect())
                .collect();
            (input_signals, output_digits)
        })
        .collect()
}

fn find_digits(input_signals: Vec<String>) {
    let signal_map: HashMap<char, char> = HashMap::new();
    let mut cf = String::new();
    for i in &input_signals {
        println!("{}", i);
        if i.chars().count() == 2 {
            cf = i.to_string();
        }
    }
    println!("cf is {}", cf);
}

fn concat_number(vec: &[usize]) -> usize {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}

fn part_1(file: String) -> usize {
    let signal_patterns = parse(file);

    let mut result: usize = 0;
    for (_input_signals, output_digits) in signal_patterns {
        println!("{:?}", output_digits);
        result += output_digits
            .iter()
            .filter(|i| i.len() == 2 || i.len() == 3 || i.len() == 4 || i.len() == 7)
            .count();
    }
    result
}

fn part_2(file: String) -> usize {
    let digits = HashSet::from([
        "abcefg".to_string(),
        "cf".to_string(),
        "acdeg".to_string(),
        "acdfg".to_string(),
        "bcdf".to_string(),
        "abdfg".to_string(),
        "abdefg".to_string(),
        "acf".to_string(),
        "abcdefg".to_string(),
        "abcdfg".to_string(),
    ]);

    let signals_to_numbers = vec![
        "abcefg".to_string(),
        "cf".to_string(),
        "acdeg".to_string(),
        "acdfg".to_string(),
        "bcdf".to_string(),
        "abdfg".to_string(),
        "abdefg".to_string(),
        "acf".to_string(),
        "abcdefg".to_string(),
        "abcdfg".to_string(),
    ];
    let signals = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g'];

    let mut result: usize = 0;
    let signal_patterns = parse(file);
    for sp in signal_patterns {
        let mut mappings = HashMap::new();
        for permutation in signals.iter().permutations(signals.len()).unique() {
            mappings = HashMap::new();
            for (i, sig) in signals.iter().enumerate() {
                mappings.insert(*sig, *permutation[i]);
            }

            let mut count = 0;
            for input_signal in &sp.0 {
                let mapped: String = input_signal
                    .chars()
                    .map(|c| mappings.get(&c).unwrap())
                    .sorted()
                    .collect();

                if digits.contains(&mapped) {
                    count += 1;
                }
            }

            if count == 10 {
                break;
            }
        }

        let mut output_number = Vec::new();
        for output in &sp.1 {
            let value: String = output
                .chars()
                .map(|c| *mappings.get(&c).unwrap())
                .sorted()
                .collect();

            let index = signals_to_numbers.iter().position(|v| v == &value).unwrap();

            output_number.push(index);
        }

        let line_result = concat_number(&output_number);
        result += line_result;
    }
    result
}

fn main() {
    let res_1 = part_1(String::from(INPUT_FILE));
    println!("part1: {}", res_1);
    let res_2 = part_2(String::from(INPUT_FILE));
    println!("part2: {}", res_2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(part_1(String::from(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe\n\
             edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc\n\
             fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg\n\
             fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb\n\
             aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea\n\
             fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb\n\
             dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe\n\
             bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef\n\
             egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb\n\
             gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce")),26)
    }

    #[test]
    fn test_solves_part1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 310);
    }

    #[test]
    fn test_solves_part_2_example() {
        assert_eq!(part_2(String::from(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf")), 5353)
    }

    #[test]
    fn test_solves_part2_input() {
        assert_eq!(part_2(String::from(INPUT_FILE)), 310);
    }
}
