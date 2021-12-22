use std::collections::HashMap;

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

fn calculate_most_and_least_common_polymers(polymers: Vec<char>) -> (i64, i64) {
    let mut counts = HashMap::new();
    for polymer in polymers.iter() {
        *counts.entry(polymer).or_insert(0) += 1;
    }

    let max = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
    let min = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();

    (*min.1, *max.1)
}

fn solver(input: String, part_2: bool) -> i64 {
    let (insertion_rules, polymer_template) = parse(input);

    let mut result = polymer_template;

    let mut iterations = 10;

    if part_2 {
        iterations = 40;
    }

    for i in 0..iterations {
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

        println!("{:?}", i);

        result = temp;
    }

    let (min, max) = calculate_most_and_least_common_polymers(result);
    max - min
}

fn main() {
    let res1 = solver(INPUT_FILE.to_string(), false);
    println!("part1: {}", res1);
    let res2 = solver(INPUT_FILE.to_string(), true);
    println!("part2: {}", res2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solves_part_1_example() {
        assert_eq!(
            solver(
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
                    .to_string(),
                false
            ),
            1588
        );
    }

    #[test]
    fn solves_part_1_input() {
        assert_eq!(solver(INPUT_FILE.to_string(), false), 2967);
    }

    #[test]
    fn solves_part_2_example() {
        assert_eq!(
            solver(
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
                    .to_string(),
                true
            ),
            1588
        );
    }
}
