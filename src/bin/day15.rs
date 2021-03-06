use std::{cmp::Ordering, collections::BinaryHeap, usize};

const INPUT_FILE: &str = include_str!("../../inputs/day15.txt");

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Edge {
    node: usize,
    cost: usize,
}

fn dijkstra_adj_list_heap(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if position == goal {
            return Some(cost);
        }

        if cost > dist[position] {
            continue;
        }

        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }
    None
}

fn parse(input: String, part2: bool) -> Vec<Vec<Edge>> {
    let mut parsed: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    if part2 {
        parsed = extend_map_for_part2(parsed);
    }
    let size_y: usize = parsed.len();
    let size_x = parsed[0].len();

    assert_eq!(size_x, size_y);

    let vertex_count = size_y * size_x;

    let mut adj_list = Vec::new();
    for v in 0..vertex_count {
        let mut edges = vec![];
        let (x, y) = flat_to_xy(v, size_y);
        // Top
        if y > 0 {
            edges.push(Edge {
                node: xy_to_flat(x, y - 1, size_y),
                cost: parsed[y - 1][x] as usize,
            });
        }
        // Right
        if x + 1 < size_x {
            edges.push(Edge {
                node: xy_to_flat(x + 1, y, size_y),
                cost: parsed[y][x + 1] as usize,
            });
        }
        // Down
        if y + 1 < size_y {
            edges.push(Edge {
                node: xy_to_flat(x, y + 1, size_y),
                cost: parsed[y + 1][x] as usize,
            });
        }
        // Left
        if x > 0 {
            edges.push(Edge {
                node: xy_to_flat(x - 1, y, size_y),
                cost: parsed[y][x - 1] as usize,
            });
        }
        adj_list.push(edges);
    }

    adj_list
}

fn extend_map_for_part2(input_map: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut result = input_map.clone();
    let mut last = input_map;
    // append y
    for _i in 1..5 {
        last = last
            .iter()
            .map(|line| {
                line.iter()
                    .map(|risk_level| std::cmp::max((risk_level + 1) % 10, 1))
                    .collect()
            })
            .collect();
        result.append(&mut last.clone())
    }

    // append x
    result = result
        .into_iter()
        .map(|line| {
            let mut l = line.clone();
            let mut last = line;
            for _ in 1..5 {
                last = last
                    .iter()
                    .map(|risk_level| std::cmp::max((risk_level + 1) % 10, 1))
                    .collect();
                l.append(&mut last.clone())
            }
            l
        })
        .collect();

    result
}

fn xy_to_flat(x: usize, y: usize, size: usize) -> usize {
    y * size + x
}

fn flat_to_xy(i: usize, size: usize) -> (usize, usize) {
    let x = i % size;
    let y = i / size;
    (x, y)
}

fn part_1(input: String) -> usize {
    let adj_list = parse(input, false);
    dijkstra_adj_list_heap(&adj_list, 0, adj_list.len() - 1).unwrap()
}

fn part_2(input: String) -> usize {
    let adj_list = parse(input, true);
    dijkstra_adj_list_heap(&adj_list, 0, adj_list.len() - 1).unwrap()
}

fn main() {
    let res1 = part_1(INPUT_FILE.to_string());
    println!("part1 {:?}", res1);

    let res2 = part_2(INPUT_FILE.to_string());
    println!("part2 {:?}", res2);
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

    #[test]
    fn test_solves_part2_example() {
        assert_eq!(
            part_2(
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
            315
        );
    }

    #[test]
    fn test_solves_part2_input() {
        assert_eq!(part_2(INPUT_FILE.to_string()), 2995);
    }
}
