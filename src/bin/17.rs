advent_of_code::solution!(17);

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn valid_next(self, dir_count: usize, part2: bool) -> Vec<Self> {
        let mut ret = Vec::with_capacity(3);
        match self {
            Direction::Up => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::Up);
                    }
                    ret.push(Self::Right);
                    ret.push(Self::Left);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::Up)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::Right);
                        ret.push(Self::Left);
                    }
                }
            }
            Direction::Down => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::Down);
                    }
                    ret.push(Self::Right);
                    ret.push(Self::Left);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::Down)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::Right);
                        ret.push(Self::Left);
                    }
                }
            }
            Direction::Left => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::Left);
                    }
                    ret.push(Self::Up);
                    ret.push(Self::Down);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::Left)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::Up);
                        ret.push(Self::Down);
                    }
                }
            }
            Direction::Right => {
                if !part2 {
                    if dir_count < 3 {
                        ret.push(Self::Right);
                    }
                    ret.push(Self::Up);
                    ret.push(Self::Down);
                } else {
                    if dir_count < 10 {
                        ret.push(Self::Right)
                    }
                    if dir_count >= 4 {
                        ret.push(Self::Up);
                        ret.push(Self::Down);
                    }
                }
            }
        }
        ret
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: (usize, usize),
    direction: Direction,
    dir_count: usize,
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

fn valid_neighbors(
    map: &[Vec<usize>],
    position: (usize, usize),
    direction: Direction,
    dir_count: usize,
    part2: bool,
) -> Vec<((usize, usize), Direction)> {
    let ret = Vec::new();
    direction
        .valid_next(dir_count, part2)
        .iter()
        .fold(ret, |mut acc, dir| {
            match dir {
                Direction::Up if position.0 > 0 => {
                    acc.push(((position.0 - 1, position.1), Direction::Up))
                }
                Direction::Down if position.0 < map.len() - 1 => {
                    acc.push(((position.0 + 1, position.1), Direction::Down))
                }
                Direction::Left if position.1 > 0 => {
                    acc.push(((position.0, position.1 - 1), Direction::Left))
                }
                Direction::Right if position.1 < map[0].len() - 1 => {
                    acc.push(((position.0, position.1 + 1), Direction::Right))
                }
                _ => {}
            };
            acc
        })
}

fn shortest_path(
    map: &[Vec<usize>],
    start: (usize, usize),
    goal: (usize, usize),
    part2: bool,
) -> Option<u64> {
    let mut dist: HashMap<((usize, usize), Direction, usize), usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    heap.push(State {
        cost: 0,
        position: start,
        direction: Direction::Right, // dummy value
        dir_count: 0,                // don't count toward limit
    });

    while let Some(State {
        cost,
        position,
        direction,
        dir_count,
    }) = heap.pop()
    {
        if position == goal {
            if part2 && dir_count < 4 {
                continue;
            }
            return Some(cost as u64);
        }

        if let Some(found) = dist.get(&(position, direction, dir_count)) {
            if *found < cost {
                continue;
            }
        }

        for (new_pos, new_dir) in valid_neighbors(map, position, direction, dir_count, part2) {
            let next = State {
                cost: cost + map[new_pos.0][new_pos.1],
                position: new_pos,
                direction: new_dir,
                dir_count: if new_dir != direction {
                    1
                } else {
                    dir_count + 1
                },
            };

            if let Some(found) = dist.get(&(new_pos, new_dir, next.dir_count)) {
                if next.cost < *found {
                    heap.push(next);
                    dist.insert((new_pos, new_dir, next.dir_count), next.cost);
                }
            } else {
                heap.push(next);
                dist.insert((new_pos, new_dir, next.dir_count), next.cost);
            }
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|s| s.to_digit(10).expect("All items should be digits") as usize)
                .collect()
        })
        .collect();
    shortest_path(&map, (0, 0), (map.len() - 1, map[0].len() - 1), false)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|s| s.to_digit(10).expect("All items should be digits") as usize)
                .collect()
        })
        .collect();
    shortest_path(&map, (0, 0), (map.len() - 1, map[0].len() - 1), true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
