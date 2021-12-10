use std::collections::HashSet;

use itertools::Itertools;

const INPUT_FILE : &str = include_str!("../../inputs/day09.txt");


fn parse(input: String) -> Vec<Vec<u32>> {
    input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap()).collect()
    }).collect()
}

fn is_lava_tube(height_map : &Vec<Vec<u32>>, point: u32, x: usize, y: usize) -> bool {
    let up = y == 0 || (height_map[y-1][x] > point);
    let right = x == height_map[y].len() - 1 || (height_map[y][x+1] > point ); 
    let down = y == height_map.len() - 1 || (height_map[y+1][x] > point);
    let left = x == 0 ||(height_map[y][x-1] > point);
    
    up && right && down && left
}

fn risk_level(low_points : Vec<u32>) -> u32 {
    low_points.iter().fold(0, |acc, x| acc + x + 1)
}

fn basin_size(height_map: &[Vec<u32>], x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> u32 {
    if visited.contains(&(x,y)) || height_map[y][x] == 9 {
        0
    } else {
        let mut result = 1;
        visited.insert((x,y));

        if y != 0 {
            result += basin_size(height_map, x, y -1, visited);
        } 

        if x != height_map[y].len() - 1 {
            result += basin_size(height_map, x + 1, y,  visited);
        } 

        if y != height_map.len() - 1 {
            result += basin_size(height_map, x, y + 1,  visited);
        } 

        if x != 0 {
            result += basin_size(height_map, x - 1, y,  visited);
        } 
        result

    }
    
}

fn find_3_largest_basins(basins: Vec<u32>) -> u32 {

    if basins.len() < 3 {
        panic!("Must have at least 3 basins");
    }

    basins.iter().sorted().rev().take(3).product()
}

fn part_1(input: String) -> u32 {
    let height_map = parse(input);

    let mut low_points = Vec::new();
    for (y,row) in height_map.iter().enumerate() {
        for (x,point) in row.iter().enumerate() {
            
            if is_lava_tube(&height_map, *point,  x, y) {
                low_points.push(*point);
            } 

        } 
    }
    risk_level(low_points)
}

fn part_2(input: String) -> u32 {
    let height_map = parse(input);

    let mut basins = Vec::new();
    for (y,row) in height_map.iter().enumerate() {
        for (x,point) in row.iter().enumerate() {
            
            if is_lava_tube(&height_map, *point,  x, y) {
                let mut visited = HashSet::new();
                basins.push(basin_size(&height_map, x, y, &mut visited));
                
            } 

        } 
    }
    find_3_largest_basins(basins)
}


fn main() {
    let result1 = part_1(INPUT_FILE.to_string());
    println!("part1: {:?}", result1);
    let result2 = part_2(INPUT_FILE.to_string());
    println!("part2: {:?}", result2)

}


#[cfg(test)]
mod test {

    use super::*;
    
    #[test]
    fn test_solves_part_1_example() {
        
        assert_eq!(part_1(String::from(
           "2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678")), 15);

    }

    #[test]
    fn test_solves_part_1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()),572);
    }

    #[test]
    fn test_solves_part_2_example() {
        
        assert_eq!(part_2(String::from(
           "2199943210\n\
            3987894921\n\
            9856789892\n\
            8767896789\n\
            9899965678")), 1134);

    }

    #[test]
    fn test_solves_part_2_input() {
        assert_eq!(part_2(INPUT_FILE.to_string()),847044);
    }

}
