use std::collections::HashMap;

advent_of_code::solution!(14);

fn rotate_north(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    for col in 0..map[0].len() {
        let mut loc = 0;
        for row in 0..map.len() {
            match map[row][col] {
                'O' => {
                    if loc != row {
                        map[loc][col] = 'O';
                        map[row][col] = '.';
                    }
                    loc += 1;
                }
                '#' => loc = row + 1,
                _ => {}
            }
        }
    }
    map
}

fn rotate_south(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    for col in 0..map[0].len() {
        let mut loc = map.len() - 1;
        for row in 0..map.len() {
            let row = map.len() - row - 1;
            match map[row][col] {
                'O' => {
                    if loc != row {
                        map[loc][col] = 'O';
                        map[row][col] = '.';
                    }
                    loc = loc.saturating_sub(1);
                }
                '#' => {
                    if row > 0 {
                        loc = row - 1
                    }
                }
                _ => {}
            }
        }
    }
    map
}

fn rotate_east(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    for row in 0..map.len() {
        let mut loc = map[0].len() - 1;
        for col in 0..map[0].len() {
            let col = map[0].len() - col - 1;
            match map[row][col] {
                'O' => {
                    if loc != col {
                        map[row][loc] = 'O';
                        map[row][col] = '.';
                    }
                    loc = loc.saturating_sub(1);
                }
                '#' => {
                    if col > 0 {
                        loc = col - 1
                    }
                }
                _ => {}
            }
        }
    }
    map
}

fn rotate_west(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    for row in 0..map.len() {
        let mut loc = 0;
        for col in 0..map[0].len() {
            match map[row][col] {
                'O' => {
                    if loc != col {
                        map[row][loc] = 'O';
                        map[row][col] = '.';
                    }
                    loc += 1;
                }
                '#' => loc = col + 1,
                _ => {}
            }
        }
    }
    map
}

fn north_load(map: &Vec<Vec<char>>) -> u64 {
    let mut acc = 0;
    for col in 0..map[0].len() {
        for row in 0..map.len() {
            if map[row][col] == 'O' {
                acc += map.len() - row;
            }
        }
    }
    acc as u64
}

fn run_cycle(map: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut map = map;
    map = rotate_north(map);
    map = rotate_west(map);
    map = rotate_south(map);
    map = rotate_east(map);
    map
}

pub fn part_one(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    let map = rotate_north(map);
    let acc = north_load(&map);
    Some(acc as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();
    // let printmap = |map: &Vec<Vec<char>>| {
    //     println!("");
    //     map.iter().for_each(|line| println!("{:?}", line));
    // };
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut map = map;
    let mut count = 0;
    let start_cycle = loop {
        map = run_cycle(map);
        let as_string = map.iter().fold(String::new(), |acc, v| {
            acc + &v.iter().cloned().collect::<String>()
        });
        if let Some(x) = seen.get(&as_string) {
            break x;
        }
        seen.insert(as_string, count);
        count += 1;
    };
    let rotations_needed = (1_000_000_000 - start_cycle - 1) % (count - start_cycle);
    let map = (0..rotations_needed).fold(map, |mut map, _| {
        map = run_cycle(map);
        map
    });
    let acc = north_load(&map);
    Some(acc as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
