use std::collections::{HashMap, HashSet, VecDeque};

const INPUT_FILE : &str = include_str!("../../inputs/day12.txt"); 


fn parse(input: String) -> HashMap<String, HashSet<String>> {
    let mut cave_map : HashMap<String, HashSet<String>> = HashMap::new();

    let lines : Vec<Vec<&str>> = input.trim().lines().map(|line| line.trim().split('-').collect()).collect();

    for line in lines {
 
        let start = line[0];
        let end = line[1];
        
        let temp = cave_map.entry(start.to_string()).or_insert(HashSet::new());
        temp.insert(end.to_string());
        
        let temp2 = cave_map.entry(end.to_string()).or_insert(HashSet::new());
        temp2.insert(start.to_string());
    }

    cave_map
}

fn part_1(input: String) -> usize {
    let cave_map = parse(input);
    
    let mut  start_position : (String, HashSet<String>)= ("start".to_string(), HashSet::new());
    start_position.1.insert("start".to_string());


    let mut moves = VecDeque::from([start_position]);
    
    let mut result = 0; 
    while moves.len() > 0 {
        let (current, small_visited) = moves.pop_front().unwrap();

        if current == "end" {
            result += 1;
        } else {
            for next in cave_map.get(&current).unwrap() {
                if !small_visited.contains(next) {
                    let mut new_small = small_visited.clone();

                    if next.to_ascii_lowercase() == *next {
                        new_small.insert(next.to_string());
                    }
                    moves.push_back((next.to_string(), new_small));
                }

            }

        }
    }
    result
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1: {:?}", res1);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part_1_example_1() {
        assert_eq!(part_1(
            "start-A\n\
             start-b\n\
             A-c\n\
             A-b\n\
             b-d\n\
             A-end\n\
             b-end".to_string()),10 )

    }

    #[test]
    fn test_solves_part_1_example() {
        assert_eq!(part_1(
               "dc-end\n\
                HN-start\n\
                start-kj\n\
                dc-start\n\
                dc-HN\n\
                LN-dc\n\
                HN-end\n\
                kj-sa\n\
                kj-HN\n\
                kj-dc".to_string()
                ),19)

    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 4413);

    }

}
