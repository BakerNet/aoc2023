use std::collections::{HashMap, HashSet};

advent_of_code::solution!(3);

enum Item {
    Empty,
    Symbol(char),
    Number(u32),
}

impl Item {
    fn from_char(c: char) -> Self {
        if let Some(d) = c.to_digit(10) {
            Item::Number(d)
        } else if c == '.' {
            Item::Empty
        } else {
            Item::Symbol(c)
        }
    }

    fn is_star(self: &Self) -> bool {
        if let Item::Symbol('*') = self {
            true
        } else {
            false
        }
    }
}

fn has_adjacent_symbol(row: usize, col: usize, graph: &Vec<Vec<Item>>) -> bool {
    let rows = graph.len();
    let cols = graph[0].len();
    if col > 0 {
        if let Item::Symbol(_) = graph[row][col - 1] {
            return true;
        };
        if row > 0 {
            if let Item::Symbol(_) = graph[row - 1][col - 1] {
                return true;
            };
        }
        if row < rows - 1 {
            if let Item::Symbol(_) = graph[row + 1][col - 1] {
                return true;
            };
        }
    }
    if col < cols - 1 {
        if let Item::Symbol(_) = graph[row][col + 1] {
            return true;
        };
        if row > 0 {
            if let Item::Symbol(_) = graph[row - 1][col + 1] {
                return true;
            };
        }
        if row < rows - 1 {
            if let Item::Symbol(_) = graph[row + 1][col + 1] {
                return true;
            };
        }
    }
    if row > 0 {
        if let Item::Symbol(_) = graph[row - 1][col] {
            return true;
        };
    }
    if row < rows - 1 {
        if let Item::Symbol(_) = graph[row + 1][col] {
            return true;
        };
    }
    false
}

fn get_part_numbers(graph: &Vec<Vec<Item>>, row: usize) -> Vec<u32> {
    let line = &graph[row];
    let mut part_numbers = Vec::new();
    let mut curr_num = 0;
    let mut is_part_num = false;
    for (col, item) in line.iter().enumerate() {
        match item {
            Item::Number(x) => {
                curr_num = curr_num * 10 + x;
                if !is_part_num {
                    is_part_num = has_adjacent_symbol(row, col, graph);
                }
            }
            _ => {
                if curr_num > 0 && is_part_num {
                    part_numbers.push(curr_num);
                }
                curr_num = 0;
                is_part_num = false;
            }
        }
    }
    if curr_num > 0 && is_part_num {
        part_numbers.push(curr_num);
    }
    part_numbers
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph: Vec<Vec<Item>> = input
        .lines()
        .map(|line| line.chars().map(|c| Item::from_char(c)).collect())
        .collect();
    let mut sum: u32 = 0;
    for row in 0..graph.len() {
        sum += get_part_numbers(&graph, row).iter().sum::<u32>();
    }
    Some(sum)
}

fn get_adjacent_stars(row: usize, col: usize, graph: &Vec<Vec<Item>>) -> HashSet<(usize, usize)> {
    let mut adjacent_stars = HashSet::new();
    let rows = graph.len();
    let cols = graph[0].len();
    if col > 0 {
        if graph[row][col - 1].is_star() {
            adjacent_stars.insert((row, col - 1));
        };
        if row > 0 {
            if graph[row - 1][col - 1].is_star() {
                adjacent_stars.insert((row - 1, col - 1));
            };
        }
        if row < rows - 1 {
            if graph[row + 1][col - 1].is_star() {
                adjacent_stars.insert((row + 1, col - 1));
            };
        }
    }
    if col < cols - 1 {
        if graph[row][col + 1].is_star() {
            adjacent_stars.insert((row, col + 1));
        };
        if row > 0 {
            if graph[row - 1][col + 1].is_star() {
                adjacent_stars.insert((row - 1, col + 1));
            };
        }
        if row < rows - 1 {
            if graph[row + 1][col + 1].is_star() {
                adjacent_stars.insert((row + 1, col + 1));
            };
        }
    }
    if row > 0 {
        if graph[row - 1][col].is_star() {
            adjacent_stars.insert((row - 1, col));
        };
    }
    if row < rows - 1 {
        if graph[row + 1][col].is_star() {
            adjacent_stars.insert((row + 1, col));
        };
    }
    adjacent_stars
}

fn add_numbers_to_gear_map(
    graph: &Vec<Vec<Item>>,
    gear_map: &mut HashMap<(usize, usize), Vec<u32>>,
    row: usize,
) -> () {
    let line = &graph[row];
    let mut curr_num = 0;
    let mut adjacent_stars = HashSet::new();
    for (col, item) in line.iter().enumerate() {
        match item {
            Item::Number(x) => {
                curr_num = curr_num * 10 + x;
                adjacent_stars.extend(get_adjacent_stars(row, col, graph));
            }
            _ => {
                if curr_num != 0 {
                    adjacent_stars.iter().for_each(|point| {
                        gear_map
                            .get_mut(&point)
                            .expect("Star points should exist in gear_map")
                            .push(curr_num);
                    });
                }
                curr_num = 0;
                adjacent_stars = HashSet::new();
            }
        }
    }
    if curr_num != 0 {
        adjacent_stars.iter().for_each(|point| {
            gear_map
                .get_mut(&point)
                .expect("Star points should exist in gear_map")
                .push(curr_num);
        });
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let graph: Vec<Vec<Item>> = input
        .lines()
        .map(|line| line.chars().map(|c| Item::from_char(c)).collect())
        .collect();
    let mut gear_map: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    graph.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, item)| {
            if item.is_star() {
                gear_map.insert((row, col), Vec::new());
            }
        })
    });
    for row in 0..graph.len() {
        add_numbers_to_gear_map(&graph, &mut gear_map, row);
    }
    let sum = gear_map
        .iter()
        .map(|item| item.1)
        .filter(|nums| nums.len() == 2)
        .map(|nums| nums[0] * nums[1])
        .sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
