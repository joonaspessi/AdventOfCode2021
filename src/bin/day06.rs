const INPUT_FILE: &str = include_str!("../../inputs/day06.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LanternFish {
    pub age: u32,
    is_new: bool,
}

impl LanternFish {
    fn progress_day(&mut self) -> Option<LanternFish> {
        if self.age > 0 {
            self.age -= 1;
            None
        } else {
            self.age = 6;
            self.is_new = false;
            Some(LanternFish {
                age: 8,
                is_new: true,
            })
        }
    }
}

fn print_fish(day: u32, fishes: &Vec<LanternFish>) {
    print!("day {}: ", day);
    for fish in fishes {
        print!("{:?},", fish.age);
    }
    println!();
}

fn parse(input: String) -> Vec<LanternFish> {
    input
        .trim()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .map(|age| LanternFish { age, is_new: false })
        .collect()
}

fn part_1(input: String) -> u32 {
    let mut fishes = parse(input);

    for _day in 1..=80 {
        let mut new_fish_generation = Vec::new();
        for fish in &mut fishes {
            if let Some(new_fish) = fish.progress_day() {
                new_fish_generation.push(new_fish);
            };
        }
        fishes.append(&mut new_fish_generation);
        //print_fish(day, &fishes);
    }

    fishes.len() as u32
}

fn main() {
    let res_1 = part_1(INPUT_FILE.to_string());
    println!("part1: {}", res_1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(part_1(String::from("3,4,3,1,2")), 5934);
    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(String::from(INPUT_FILE)), 352151);
    }
}