use std::{cmp::Ordering, collections::HashMap};

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

    fn from_tile(tile: char) -> (Self, Self) {
        match tile {
            '|' => (Self::North, Self::South),
            '-' => (Self::East, Self::West),
            'L' => (Self::North, Self::East),
            'J' => (Self::North, Self::West),
            '7' => (Self::South, Self::West),
            'F' => (Self::South, Self::East),
            _ => panic!("Unknown tiles"),
        }
    }

    fn to_tile(self, dir: Self) -> char {
        match (self, dir) {
            (Self::North, Self::South) | (Self::South, Self::North) => '|',
            (Self::East, Self::West) | (Self::West, Self::East) => '-',
            (Self::North, Self::East) | (Self::East, Self::North) => 'L',
            (Self::North, Self::West) | (Self::West, Self::North) => 'J',
            (Self::South, Self::West) | (Self::West, Self::South) => '7',
            (Self::South, Self::East) | (Self::East, Self::South) => 'F',
            _ => panic!("Impossible combo of directions"),
        }
    }

    fn is_connected(&self, tile: char) -> bool {
        match tile {
            '|' => matches!(self, Self::North | Self::South),
            '-' => matches!(self, Self::East | Self::West),
            'L' => matches!(self, Self::South | Self::West),
            'J' => matches!(self, Self::South | Self::East),
            '7' => matches!(self, Self::North | Self::East),
            'F' => matches!(self, Self::North | Self::West),
            'S' => true,
            _ => false,
        }
    }

    fn opposite(self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
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
    if !direction.is_connected(tile) {
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

fn determine_tile(
    origin: (usize, usize),
    neighbor1: (usize, usize),
    neighbor2: (usize, usize),
) -> char {
    let dir1 = Direction::from_indices(origin, neighbor1);
    let dir2 = Direction::from_indices(origin, neighbor2);
    dir1.to_tile(dir2)
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&graph);
    let graph_loop = find_loop(&graph, start);
    let mut graph = graph;
    graph[start.0][start.1] =
        determine_tile(start, graph_loop[1], graph_loop[graph_loop.len() - 1]);
    let seen: HashMap<(usize, usize), (Direction, Direction)> = graph_loop
        .iter()
        .cloned()
        .map(|index| (index, Direction::from_tile(graph[index.0][index.1])))
        .collect();
    let mut count = 0;
    for (x, row) in graph.iter().enumerate() {
        let mut inside = false;
        for (y, _) in row.iter().enumerate() {
            if let Some(dirs) = seen.get(&(x, y)) {
                if matches!(dirs.0, Direction::North) {
                    inside = !inside;
                }
            } else {
                if inside {
                    count += 1;
                }
            }
        }
    }
    Some(count as u64)
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
