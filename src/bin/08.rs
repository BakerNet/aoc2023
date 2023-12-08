use std::collections::HashMap;

use regex::Regex;

advent_of_code::solution!(8);

fn build_map(lines: Vec<&str>) -> HashMap<String, (String, String)> {
    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();
    lines
        .iter()
        .map(|line| {
            let line_re = re.captures(line).expect("Expect line to match regex");
            (
                line_re[1].to_owned(),
                (line_re[2].to_owned(), line_re[3].to_owned()),
            )
        })
        .collect()
}

fn find_starting_locations(lines: Vec<&str>) -> Vec<String> {
    lines
        .iter()
        .filter_map(|&line| {
            let item = line.split_whitespace().next().unwrap();
            if item.ends_with('A') {
                Some(item.to_owned())
            } else {
                None
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next(); // burn empty line

    let map = build_map(lines.collect());

    let mut curr = String::from("AAA");
    let mut count_path: Option<u64> = None;
    for (count, instruction) in instructions.iter().cycle().enumerate() {
        let curr_tuple = map
            .get(&curr)
            .unwrap_or_else(|| panic!("Should find {} in map", curr));
        curr = if *instruction == 'L' {
            curr_tuple.0.clone()
        } else {
            curr_tuple.1.clone()
        };
        if curr == "ZZZ" {
            count_path = Some(count as u64 + 1);
            break;
        }
    }
    count_path
}

pub fn lcm(nums: &[u64]) -> u64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let instructions: Vec<char> = lines.next().unwrap().chars().collect();

    lines.next(); // burn empty line
    let lines: Vec<&str> = lines.collect();

    let map = build_map(lines.clone());

    let mut currs = find_starting_locations(lines);
    let mut counts_per_path: Vec<Option<u64>> = vec![None; currs.len()];
    for (count, instruction) in instructions.iter().cycle().enumerate() {
        let curr_tuples: Vec<&(String, String)> = currs
            .iter()
            .map(|loc| map.get(loc).expect(&format!("Should find {} in map", loc)))
            .collect();
        currs = curr_tuples
            .iter()
            .enumerate()
            .map(|(index, loc_tup)| {
                let next = if *instruction == 'L' {
                    loc_tup.0.to_owned()
                } else {
                    loc_tup.1.to_owned()
                };
                if next.ends_with('Z') && counts_per_path[index].is_none() {
                    counts_per_path[index] = Some(count as u64 + 1);
                }
                next
            })
            .collect();
        if !counts_per_path.iter().any(|i| i.is_none()) {
            break;
        }
    }
    let iters_found: Vec<u64> = counts_per_path.iter().map(|x| x.unwrap()).collect();
    let lcm = lcm(&iters_found);
    Some(lcm)
}

#[cfg(test)]
mod tests {
    use std::{env, fs};

    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    fn read_file(folder: &str, file: &str) -> String {
        let cwd = env::current_dir().unwrap();
        let filepath = cwd.join("data").join(folder).join(file);
        let f = fs::read_to_string(filepath);
        f.expect("could not open input file")
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&read_file("examples", "08_2.txt"));
        assert_eq!(result, Some(6));
    }
}
