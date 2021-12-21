use std::collections::HashMap;

use itertools::Itertools;

const INPUT_FILE: &str = include_str!("../../inputs/day14.txt");

fn parse(input: String) -> (HashMap<(char, char), char>, Vec<char>) {
    let mut temp: Vec<&str> = input.trim().split("\n\n").collect();

    let insertion_rules: Vec<(String, String)> = temp
        .pop()
        .unwrap()
        .lines()
        .map(|line| line.split(" -> ").collect())
        .map(|mut pairs: Vec<&str>| {
            (
                pairs.pop().unwrap().to_string(),
                pairs.pop().unwrap().to_string(),
            )
        })
        .collect();

    let mut result = HashMap::new();
    for rule in insertion_rules {
        let c: Vec<char> = rule.1.chars().collect();
        result.insert((c[0], c[1]), rule.0.chars().next().unwrap());
    }

    let polymer_template: Vec<char> = temp.pop().unwrap().chars().collect();

    (result, polymer_template)
}

fn part_1(input: String) -> i64 {
    let (insertion_rules, polymer_template) = parse(input);

    println!("{:?}", insertion_rules);
    let mut result = polymer_template.clone();

    for _ in 0..4 {
        let mut temp = vec![];
        for (i, pair) in result.windows(2).enumerate() {
            let prev = pair[0];
            let next = pair[1];

            let insert = insertion_rules.get(&(prev, next)).unwrap();

            if i == 0 {
                temp.push(prev);
            }
            temp.push(*insert);
            temp.push(next);
        }

        result = temp;
    }

    println!("{:?}", result);
    0
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solves_part_1_example() {
        assert_eq!(
            part_1(
                "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
                    .to_string()
            ),
            1588
        );
    }
}
