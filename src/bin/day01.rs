const INPUT_FILE: &str = include_str!("../../inputs/day01.txt");

fn part_1(file: String) -> usize {
  input_to_numbers(file)
    .windows(2)
    .filter(|values| values[0] < values[1])
    .count()
}

fn part_2(file: String) -> usize {
  calculate_depth_measurement(calculate_sliding_windows(input_to_numbers(file)))
}

fn input_to_numbers(input: String) -> Vec<usize> {
  input
    .split('\n')
    .map(|i| i.parse::<usize>().expect("parsing"))
    .collect()
}

fn calculate_depth_measurement(numbers: Vec<usize>) -> usize {
  let mut result = 0;
  for x in 1..numbers.len() {
    if numbers[x - 1] < numbers[x] {
      result += 1;
    }
  }
  result
}

fn calculate_sliding_windows(numbers: Vec<usize>) -> Vec<usize> {
  let mut result = Vec::new();
  for x in 2..numbers.len() {
    result.push(numbers[x - 2] + numbers[x - 1] + numbers[x]);
  }
  result
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
    assert_eq!(
      part_1(String::from(
        "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
      )),
      7
    );
  }

  #[test]
  fn test_solves_part2() {
    assert_eq!(
      part_2(String::from(
        "199\n200\n208\n210\n200\n207\n240\n269\n260\n263"
      )),
      5
    );
  }
}
