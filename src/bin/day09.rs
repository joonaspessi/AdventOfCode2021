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


fn main() {
    let result1 = part_1(INPUT_FILE.to_string());
    println!("part1: {:?}", result1)

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

}
