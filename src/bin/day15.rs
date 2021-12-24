const INPUT_FILE: &str = include_str!("../../inputs/day15.txt");

fn parse(input: String) -> (Vec<Vec<u32>>, usize) {
    let parsed: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let size_y: usize = parsed.len();
    let size_x = parsed[0].len();

    assert_eq!(size_x, size_y);

    let vertex_count = size_y * size_x;

    let mut vertex_map = Vec::new();
    for v in 0..vertex_count {
        let mut vertex = vec![0; vertex_count];
        let (x, y) = flat_to_xy(v, size_y);
        // Top
        if y > 0 {
            vertex[xy_to_flat(x, y - 1, size_y)] = parsed[y - 1][x];
        }
        // Right
        if x + 1 < size_x {
            vertex[xy_to_flat(x + 1, y, size_y)] = parsed[y][x + 1];
        }
        // Down
        if y + 1 < size_y {
            vertex[xy_to_flat(x, y + 1, size_y)] = parsed[y + 1][x];
        }
        // Left
        if x > 0 {
            vertex[xy_to_flat(x - 1, y, size_y)] = parsed[y][x - 1];
        }

        vertex_map.push(vertex);
    }
    (vertex_map, vertex_count)
}

fn xy_to_flat(x: usize, y: usize, size: usize) -> usize {
    y * size + x
}

fn flat_to_xy(i: usize, size: usize) -> (usize, usize) {
    let x = i % size;
    let y = i / size;
    (x, y)
}

fn min_distance(dist: &[u32], spt_set: &[bool], vertex_count: usize) -> usize {
    let mut min = u32::MAX;
    let mut min_index = 0;
    for vertex in 0..vertex_count {
        if !spt_set[vertex] && dist[vertex] <= min {
            min = dist[vertex];
            min_index = vertex;
        }
    }
    min_index
}

fn part_1(input: String) -> u32 {
    let (graph, vertex_count) = parse(input);

    let mut dist = vec![u32::MAX; vertex_count];
    let mut spt_set = vec![false; vertex_count];

    dist[0] = 0;

    for _ in 0..(vertex_count - 1) {
        let u = min_distance(&dist, &spt_set, vertex_count);
        println!("{}", u);
        spt_set[u] = true;

        for v in 0..vertex_count {
            if !spt_set[v]
                && graph[u][v] > 0
                && dist[u] != u32::MAX
                && dist[u] + graph[u][v] < dist[v]
            {
                dist[v] = dist[u] + graph[u][v];
            }
        }
    }
    println!("{:?}", dist);
    dist.pop().unwrap()
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1 {:?}", res1);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solves_part1_example() {
        assert_eq!(
            part_1(
                "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
                    .to_string()
            ),
            40
        );
    }

    #[test]
    fn test_solves_part1_input() {
        assert_eq!(part_1(INPUT_FILE.to_string()), 685);
    }
}
