use std::{
    cmp,
    collections::{HashSet, VecDeque},
};

advent_of_code::solution!(16);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn reflect_back(self) -> Self {
        // \
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Right => Self::Down,
            Self::Left => Self::Up,
        }
    }

    fn reflect_forward(self) -> Self {
        // /
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Right => Self::Up,
            Self::Left => Self::Down,
        }
    }
}

fn next(
    from: (usize, usize),
    direction: Direction,
    bounds: (usize, usize),
) -> Option<(usize, usize)> {
    match direction {
        Direction::Up => {
            if from.0 > 0 {
                Some((from.0 - 1, from.1))
            } else {
                None
            }
        }
        Direction::Down => {
            if from.0 < bounds.0 {
                Some((from.0 + 1, from.1))
            } else {
                None
            }
        }
        Direction::Left => {
            if from.1 > 0 {
                Some((from.0, from.1 - 1))
            } else {
                None
            }
        }
        Direction::Right => {
            if from.1 < bounds.1 {
                Some((from.0, from.1 + 1))
            } else {
                None
            }
        }
    }
}

fn find_energized(map: &[Vec<char>], start: (usize, usize), direction: Direction) -> usize {
    let mut seen: HashSet<((usize, usize), Direction)> = HashSet::new();
    let mut energized = vec![vec![false; map[0].len()]; map.len()];
    let mut queue = VecDeque::from(vec![(start, direction)]);
    let bounds = (map.len() - 1, map[0].len() - 1);
    while !queue.is_empty() {
        let (curr, dir) = queue.pop_front().unwrap();
        if seen.contains(&(curr, dir)) {
            continue;
        }
        seen.insert((curr, dir));
        energized[curr.0][curr.1] = true;
        let mut add_to_queue = |curr: (usize, usize), dir: Direction| {
            if let Some(point) = next(curr, dir, bounds) {
                queue.push_back((point, dir));
            }
        };
        match map[curr.0][curr.1] {
            '\\' => {
                let new_dir = dir.reflect_back();
                add_to_queue(curr, new_dir);
            }
            '/' => {
                let new_dir = dir.reflect_forward();
                add_to_queue(curr, new_dir);
            }
            '|' if matches!(dir, Direction::Right | Direction::Left) => {
                add_to_queue(curr, Direction::Up);
                add_to_queue(curr, Direction::Down);
            }
            '-' if matches!(dir, Direction::Up | Direction::Down) => {
                add_to_queue(curr, Direction::Left);
                add_to_queue(curr, Direction::Right);
            }
            _ => {
                add_to_queue(curr, dir);
            }
        }
    }
    energized.iter().flatten().filter(|&b| *b).count()
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let energized = find_energized(&map, (0, 0), Direction::Right);
    Some(energized as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let rows = map.len();
    let cols = map[0].len();
    let horizontals = (0..rows).fold(0, |acc, index| {
        let from_left = find_energized(&map, (index, 0), Direction::Right);
        let from_right = find_energized(&map, (index, cols - 1), Direction::Left);
        cmp::max(acc, cmp::max(from_left, from_right))
    });
    let verticals = (0..cols).fold(0, |acc, index| {
        let from_top = find_energized(&map, (0, index), Direction::Down);
        let from_bottom = find_energized(&map, (rows - 1, index), Direction::Up);
        cmp::max(acc, cmp::max(from_top, from_bottom))
    });
    Some(cmp::max(horizontals, verticals) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
