use std::{cmp, collections::VecDeque, default};

use regex::Regex;

advent_of_code::solution!(18);

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from(s: &str) -> Direction {
        match s.chars().next().unwrap() {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Unexpected direction {}", s),
        }
    }
}

// shoelace formula
fn poly_area(points: &[(usize, usize)]) -> usize {
    let last = points[points.len() - 1];
    let (x, y): (Vec<_>, Vec<_>) = points.iter().copied().unzip();
    let x_rolled = [vec![last.0], x[0..x.len() - 1].to_vec()].concat();
    let y_rolled = [vec![last.1], y[0..y.len() - 1].to_vec()].concat();
    let x_dot: usize = x.iter().zip(y_rolled.iter()).map(|(x, y)| x * y).sum();
    let y_dot: usize = y.iter().zip(x_rolled.iter()).map(|(y, x)| y * x).sum();
    let diff = if x_dot > y_dot {
        x_dot - y_dot
    } else {
        y_dot - x_dot
    };
    diff / 2
}

fn find_start(instructions: &Vec<(Direction, usize)>) -> (usize, usize) {
    let horizontal_bounds = instructions
        .iter()
        .fold((0_isize, 0_isize), |mut acc, item| {
            match item.0 {
                Direction::Left => {
                    acc.0 -= item.1 as isize;
                    acc.1 = cmp::min(acc.0, acc.1);
                }
                _ => {}
            }
            acc
        });
    let vertical_bounds = instructions
        .iter()
        .fold((0_isize, 0_isize), |mut acc, item| {
            match item.0 {
                Direction::Up => {
                    acc.0 -= item.1 as isize;
                    acc.1 = cmp::min(acc.0, acc.1);
                }
                _ => {}
            }
            acc
        });
    let offset_x = horizontal_bounds.1.abs() as usize;
    let offset_y = vertical_bounds.1.abs() as usize;
    (offset_y, offset_x)
}

// // evidence of original naive solution - build map border -> fill in with trenches -> count trenches
// fn fill(map: &mut [Vec<char>], start_fill: (usize, usize)) -> () {
//     // bfs
//     let mut queue = VecDeque::new();
//     queue.push_back(start_fill);
//     while !queue.is_empty() {
//         let curr = queue.pop_front().unwrap();
//         let curr_neighbors = neighbors(curr);
//         let valid_neighbors: Vec<&(usize, usize)> = curr_neighbors
//             .iter()
//             .filter(|loc| map[loc.0][loc.1] != '#')
//             .collect();
//         for &item in valid_neighbors.iter() {
//             queue.push_back(*item);
//             map[item.0][item.1] = '#';
//         }
//     }
// }

pub fn part_one(input: &str) -> Option<u64> {
    let instructions: Vec<(Direction, usize)> = input
        .lines()
        .map(|line| {
            let mut items = line.split_whitespace();
            let dir = Direction::from(items.next().unwrap());
            let num = items.next().unwrap().parse::<usize>().unwrap();
            (dir, num)
        })
        .collect();
    let start = find_start(&instructions);
    let (_, circumferance, points) = instructions.iter().fold(
        (start, 0, Vec::with_capacity(instructions.len())),
        |mut acc, item| {
            match item.0 {
                Direction::Up => {
                    let next = (acc.0 .0 - item.1, acc.0 .1);
                    acc.2.push(next);
                    acc.0 = next;
                }
                Direction::Down => {
                    let next = (acc.0 .0 + item.1, acc.0 .1);
                    acc.2.push(next);
                    acc.0 = next;
                }
                Direction::Left => {
                    let next = (acc.0 .0, acc.0 .1 - item.1);
                    acc.2.push(next);
                    acc.0 = next;
                }
                Direction::Right => {
                    let next = (acc.0 .0, acc.0 .1 + item.1);
                    acc.2.push(next);
                    acc.0 = next;
                }
            };
            acc.1 += item.1;
            acc
        },
    );
    let inner_area = poly_area(&points);
    Some((inner_area + 1 + circumferance / 2) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"\(#([0-9a-fA-F]{6})\)").unwrap();
    let instructions: Vec<(Direction, usize)> = input
        .lines()
        .map(|line| {
            let items = line.split_whitespace();
            let hex = re.captures(items.last().unwrap()).unwrap()[1].to_owned();
            let dir = match hex.chars().last().unwrap() {
                '0' => Direction::Right,
                '1' => Direction::Down,
                '2' => Direction::Left,
                '3' => Direction::Up,
                default => panic!("Unexpected last char in hex: {}", default),
            };
            let num = usize::from_str_radix(&hex[0..hex.len() - 1], 16)
                .expect("Expected to be able to parse hex");
            (dir, num)
        })
        .collect();
    let start = find_start(&instructions);
    let (_, circumferance, points) = instructions.iter().fold(
        (start, 0, Vec::with_capacity(instructions.len())),
        |mut acc, item| {
            match item.0 {
                Direction::Up => {
                    let next = (acc.0 .0 - item.1, acc.0 .1);
                    acc.2.push(next);
                    acc.0 = next;
                }
                Direction::Down => {
                    let next = (acc.0 .0 + item.1, acc.0 .1);
                    acc.2.push(next);
                    acc.0 = next;
                }
                Direction::Left => {
                    let next = (acc.0 .0, acc.0 .1 - item.1);
                    acc.2.push(next);
                    acc.0 = next;
                }
                Direction::Right => {
                    let next = (acc.0 .0, acc.0 .1 + item.1);
                    acc.2.push(next);
                    acc.0 = next;
                }
            };
            acc.1 += item.1;
            acc
        },
    );
    let inner_area = poly_area(&points);
    Some((inner_area + 1 + circumferance / 2) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(952408144115));
    }
}
