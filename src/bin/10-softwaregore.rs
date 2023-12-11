// NOTE:  This was my actual working implementation when Day 10 dropped
// ... I was on 4 hours of sleep and it was past 1AM when I finished

use std::{
    cmp::Ordering,
    collections::{HashSet, VecDeque},
};

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from_indices(from: (usize, usize), to: (usize, usize)) -> Self {
        match from.0.cmp(&to.0) {
            Ordering::Equal => {
                if from.1 < to.1 {
                    Self::East
                } else {
                    Self::West
                }
            }
            Ordering::Greater => Self::North,
            Ordering::Less => Self::South,
        }
    }
}

fn neighbors(graph: &[Vec<char>], of: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let row = of.0;
    let col = of.1;
    let rows = graph.len();
    let cols = graph[0].len();
    if col > 0 {
        neighbors.push((row, col - 1));
        if row > 0 {
            neighbors.push((row - 1, col - 1));
        }
        if row < rows - 1 {
            neighbors.push((row + 1, col - 1));
        }
    }
    if col < cols - 1 {
        neighbors.push((row, col + 1));
        if row > 0 {
            neighbors.push((row - 1, col + 1));
        }
        if row < rows - 1 {
            neighbors.push((row + 1, col + 1));
        }
    }
    if row > 0 {
        neighbors.push((row - 1, col));
    }
    if row < rows - 1 {
        neighbors.push((row + 1, col));
    }
    neighbors
}

fn find_start(graph: &[Vec<char>]) -> (usize, usize) {
    let mut start: (usize, usize) = (0, 0);
    'outer: for (x, v) in graph.iter().enumerate() {
        for (y, c) in v.iter().enumerate() {
            if *c == 'S' {
                start = (x, y);
                break 'outer;
            }
        }
    }
    start
}

fn next_in_loop(direction: Direction, tile: char, index: (usize, usize)) -> Option<(usize, usize)> {
    if !is_connected(direction, tile) {
        return None;
    }
    match tile {
        '|' => {
            if matches!(direction, Direction::North) {
                Some((index.0 - 1, index.1))
            } else {
                Some((index.0 + 1, index.1))
            }
        }
        '-' => {
            if matches!(direction, Direction::West) {
                Some((index.0, index.1 - 1))
            } else {
                Some((index.0, index.1 + 1))
            }
        }
        'L' => {
            if matches!(direction, Direction::South) {
                Some((index.0, index.1 + 1))
            } else {
                Some((index.0 - 1, index.1))
            }
        }
        'J' => {
            if matches!(direction, Direction::South) {
                Some((index.0, index.1 - 1))
            } else {
                Some((index.0 - 1, index.1))
            }
        }
        '7' => {
            if matches!(direction, Direction::North) {
                Some((index.0, index.1 - 1))
            } else {
                Some((index.0 + 1, index.1))
            }
        }
        'F' => {
            if matches!(direction, Direction::North) {
                Some((index.0, index.1 + 1))
            } else {
                Some((index.0 + 1, index.1))
            }
        }
        _ => None,
    }
}

fn is_connected(direction: Direction, tile: char) -> bool {
    match tile {
        '|' => matches!(direction, Direction::North | Direction::South),
        '-' => matches!(direction, Direction::East | Direction::West),
        'L' => matches!(direction, Direction::South | Direction::West),
        'J' => matches!(direction, Direction::South | Direction::East),
        '7' => matches!(direction, Direction::North | Direction::East),
        'F' => matches!(direction, Direction::North | Direction::West),
        'S' => true,
        _ => false,
    }
}

fn find_loop(graph: &[Vec<char>], start: (usize, usize)) -> Vec<(usize, usize)> {
    for neighbor in neighbors(graph, start) {
        let mut path = vec![start, neighbor];
        let mut curr = start;
        let mut next = neighbor;
        loop {
            let tile = graph[next.0][next.1];
            if let Some(index) = next_in_loop(Direction::from_indices(curr, next), tile, next) {
                if index == start {
                    return path;
                }
                curr = next;
                next = index;
                path.push(index);
            } else {
                break;
            }
        }
    }
    Vec::new()
}

fn find_loop_len(graph: &[Vec<char>], start: (usize, usize)) -> usize {
    find_loop(graph, start).len()
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let length = find_loop_len(&graph, find_start(&graph)) as u64;
    Some(length / 2 + length % 2)
}

#[derive(Debug, PartialEq, Eq)]
enum Handedness {
    Right,
    Left,
}

impl Handedness {
    fn from_neighbor_and_direction(
        index: (usize, usize),
        tile: char,
        neighbor: (usize, usize),
        direction: Direction,
    ) -> Self {
        match direction {
            Direction::North => match tile {
                '|' => {
                    if neighbor.1 > index.1 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                '7' => {
                    if neighbor.0 < index.0 || neighbor.1 > index.1 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                'F' => {
                    if neighbor.0 < index.0 || neighbor.1 < index.1 {
                        Self::Left
                    } else {
                        Self::Right
                    }
                }
                _ => panic!("Unexpected tile"),
            },
            Direction::South => match tile {
                '|' => {
                    if neighbor.1 < index.1 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                'L' => {
                    if neighbor.0 > index.0 || neighbor.1 < index.1 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                'J' => {
                    if neighbor.0 > index.0 || neighbor.1 > index.1 {
                        Self::Left
                    } else {
                        Self::Right
                    }
                }
                _ => panic!("Unexpected tile"),
            },
            Direction::East => match tile {
                '-' => {
                    if neighbor.0 > index.0 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                '7' => {
                    if neighbor.0 < index.0 || neighbor.1 > index.1 {
                        Self::Left
                    } else {
                        Self::Right
                    }
                }
                'J' => {
                    if neighbor.0 > index.0 || neighbor.1 > index.1 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                _ => panic!("Unexpected tile"),
            },
            Direction::West => match tile {
                '-' => {
                    if neighbor.0 > index.0 {
                        Self::Left
                    } else {
                        Self::Right
                    }
                }
                'L' => {
                    if neighbor.0 > index.0 || neighbor.1 < index.1 {
                        Self::Left
                    } else {
                        Self::Right
                    }
                }
                'F' => {
                    if neighbor.0 < index.0 || neighbor.1 < index.1 {
                        Self::Right
                    } else {
                        Self::Left
                    }
                }
                _ => panic!("Unexpected tile"),
            },
        }
    }

    fn from_origin_and_direction(direction: Direction) -> Self {
        match direction {
            Direction::North => Self::Left,
            Direction::West => Self::Right,
            _ => panic!("Impossible direction when reaching origin"),
        }
    }
}

fn find_handedness(graph: &[Vec<char>], graph_loop: &[(usize, usize)]) -> Handedness {
    if graph_loop.contains(&(0, 0)) {
        return Handedness::from_origin_and_direction(
            graph_loop
                .windows(2)
                .find_map(|v| {
                    if v[1] == (0, 0) {
                        Some(Direction::from_indices(v[0], v[1]))
                    } else {
                        None
                    }
                })
                .unwrap(),
        );
    }
    let mut from = (0, 0);
    let mut curr = (0, 0);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    while !graph_loop.contains(&curr) {
        let curr_neighbors = neighbors(graph, curr);
        curr_neighbors.iter().for_each(|index| {
            if seen.contains(index) {
            } else {
                queue.push_back(*index);
            }
        });
        seen.insert(curr);
        from = curr;
        curr = queue
            .pop_back()
            .expect("Shouldn't ever empty queue find_handedness BFS");
    }
    Handedness::from_neighbor_and_direction(
        curr,
        graph[curr.0][curr.1],
        from,
        graph_loop
            .windows(2)
            .find_map(|v| {
                if v[1] == (curr.0, curr.1) {
                    Some(Direction::from_indices(v[0], v[1]))
                } else {
                    None
                }
            })
            .unwrap(),
    )
}

fn bfs_count(
    graph: &[Vec<char>],
    index: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
) -> usize {
    let mut count = 1;
    let mut queue = VecDeque::new();
    queue.push_back(index);
    while !queue.is_empty() {
        let curr = queue.pop_front().unwrap();
        let curr_neighbors = neighbors(graph, curr);
        curr_neighbors.iter().for_each(|index| {
            if seen.contains(index) {
            } else {
                count += 1;
                seen.insert(*index);
                queue.push_back(*index);
            }
        });
    }
    count
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let graph_loop = find_loop(&graph, find_start(&graph));
    let handedness = find_handedness(&graph, &graph_loop);
    let mut seen: HashSet<(usize, usize)> = graph_loop.iter().cloned().collect();
    let indices: usize = graph_loop
        .windows(2)
        .map(|v| {
            let index_neighbors = neighbors(&graph, v[1]);
            let filtered_neighbors: Vec<&(usize, usize)> = index_neighbors
                .iter()
                .filter(|&idx| {
                    if seen.contains(idx) {
                        return false;
                    }
                    seen.insert(*idx);
                    Handedness::from_neighbor_and_direction(
                        v[1],
                        graph[v[1].0][v[1].1],
                        *idx,
                        Direction::from_indices(v[0], v[1]),
                    ) != handedness
                })
                .collect();
            filtered_neighbors
                .iter()
                .map(|&idx| bfs_count(&graph, *idx, &mut seen))
                .sum::<usize>()
        })
        .sum();
    Some(indices as u64)
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    fn read_file(folder: &str, file: &str) -> String {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("data").join(folder).join(file);
        let f = fs::read_to_string(filepath);
        f.expect("could not open input file")
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", "10_2.txt"));
        assert_eq!(result, Some(10));
    }
}
