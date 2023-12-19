use std::{
    collections::{HashMap, VecDeque},
    ops::Range,
};

use regex::Regex;

advent_of_code::solution!(19);

#[derive(Debug, Clone, Copy)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from(s: &str) -> Self {
        match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => panic!("Unexpected string for Category {}", s),
        }
    }
}

#[derive(Debug, Clone)]
enum Result {
    Accept,
    Reject,
    Next(String),
}

impl Result {
    fn from(s: &str) -> Self {
        match s {
            "A" => Self::Accept,
            "R" => Self::Reject,
            default => Self::Next(default.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
enum Op {
    Greater(Category, usize, Result),
    Less(Category, usize, Result),
    Nil(Result),
}

impl Op {
    fn from(s: &str) -> Self {
        if !s.contains(':') {
            return Self::Nil(Result::from(s));
        }
        let re = Regex::new(r"^([xmas])([><])(\d+):([a-zA-Z]+)$").unwrap();
        let items = re
            .captures(s)
            .unwrap_or_else(|| panic!("Should find Op format {}", s));
        match &items[2] {
            ">" => Self::Greater(
                Category::from(&items[1]),
                items[3]
                    .parse::<usize>()
                    .expect("Should be able to parse Op num"),
                Result::from(&items[4]),
            ),
            "<" => Self::Less(
                Category::from(&items[1]),
                items[3]
                    .parse::<usize>()
                    .expect("Should be able to parse Op num"),
                Result::from(&items[4]),
            ),
            _ => panic!("Should be comparison at second spot {}", s),
        }
    }

    fn check(&self, part: &Part) -> Option<Result> {
        match self {
            Self::Nil(s) => Some(s.clone()),
            Self::Greater(cat, num, s) => match cat {
                Category::X if part.x > *num => Some(s.clone()),
                Category::M if part.m > *num => Some(s.clone()),
                Category::A if part.a > *num => Some(s.clone()),
                Category::S if part.s > *num => Some(s.clone()),
                _ => None,
            },
            Self::Less(cat, num, s) => match cat {
                Category::X if part.x < *num => Some(s.clone()),
                Category::M if part.m < *num => Some(s.clone()),
                Category::A if part.a < *num => Some(s.clone()),
                Category::S if part.s < *num => Some(s.clone()),
                _ => None,
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn from(s: &str) -> Self {
        let re = Regex::new(r"^\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}$").unwrap();
        let splits = re
            .captures(s)
            .unwrap_or_else(|| panic!("Expected to parse Part {}", s));
        Part {
            x: splits[1].parse().unwrap(),
            m: splits[2].parse().unwrap(),
            a: splits[3].parse().unwrap(),
            s: splits[4].parse().unwrap(),
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct PartRanges {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl PartRanges {
    fn new() -> Self {
        Self {
            x: (1..4001),
            m: (1..4001),
            a: (1..4001),
            s: (1..4001),
        }
    }

    fn is_valid(&self) -> bool {
        if self.x.is_empty() {
            false
        } else if self.m.is_empty() {
            false
        } else if self.a.is_empty() {
            false
        } else if self.s.is_empty() {
            false
        } else {
            true
        }
    }

    fn total_values(&self) -> usize {
        self.x.clone().count()
            * self.m.clone().count()
            * self.a.clone().count()
            * self.s.clone().count()
    }

    fn shifted_greater(&self, cat: &Category, num: usize) -> Self {
        let mut new = self.clone();
        match cat {
            Category::X => new.x.start = num + 1,
            Category::M => new.m.start = num + 1,
            Category::A => new.a.start = num + 1,
            Category::S => new.s.start = num + 1,
        }
        new
    }

    fn shifted_less(&self, cat: &Category, num: usize) -> Self {
        let mut new = self.clone();
        match cat {
            Category::X => new.x.end = num,
            Category::M => new.m.end = num,
            Category::A => new.a.end = num,
            Category::S => new.s.end = num,
        }
        new
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let re = Regex::new(r"^([a-zA-Z]+)\{(.+)\}$").unwrap();
    let (workflows, parts, _): (HashMap<String, Vec<Op>>, Vec<Part>, bool) =
        input
            .lines()
            .fold((HashMap::new(), Vec::new(), false), |mut acc, line| {
                if line == "" {
                    acc.2 = true;
                    return acc;
                }
                if !acc.2 {
                    let splits = re
                        .captures(line)
                        .unwrap_or_else(|| panic!("Should be able to split workflow {}", line));
                    let key = &splits[1];
                    let ops = splits[2].split(',').map(Op::from).collect();
                    acc.0.insert(key.to_owned(), ops);
                } else {
                    acc.1.push(Part::from(line));
                }
                acc
            });
    let mut accepted = Vec::new();
    for part in parts.iter() {
        let mut curr = String::from("in");
        let result = 'outer: loop {
            let checks = workflows.get(&curr).expect("Should find in map");
            for check in checks.iter() {
                let item = check.check(part);
                if let Some(res) = &item {
                    match res {
                        Result::Accept => break 'outer true,
                        Result::Reject => break 'outer false,
                        Result::Next(s) => {
                            s.clone_into(&mut curr);
                            break;
                        }
                    }
                }
            }
        };
        if result {
            accepted.push(part);
        }
    }
    Some(accepted.iter().map(|p| p.sum()).sum::<usize>() as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let re = Regex::new(r"^([a-zA-Z]+)\{(.+)\}$").unwrap();
    let workflows: HashMap<String, Vec<Op>> =
        input
            .lines()
            .take_while(|&line| line != "")
            .fold(HashMap::new(), |mut acc, line| {
                let splits = re
                    .captures(line)
                    .unwrap_or_else(|| panic!("Should be able to split workflow {}", line));
                let key = &splits[1];
                let ops = splits[2].split(',').map(Op::from).collect();
                acc.insert(key.to_owned(), ops);
                acc
            });
    let mut total = 0;
    let mut queue: VecDeque<(PartRanges, &Result)> = VecDeque::new();
    let start = Result::Next(String::from("in"));
    queue.push_back((PartRanges::new(), &start));
    while !queue.is_empty() {
        let (ranges, res) = queue.pop_front().unwrap();
        if !ranges.is_valid() {
            continue;
        }
        match res {
            Result::Accept => total += ranges.total_values(),
            Result::Reject => continue,
            Result::Next(s) => {
                // for each Op in workflows[s] > create new ranges & add to queue
                let checks = workflows.get(s).expect("Should find workflow");
                let mut leftover_ranges = ranges.clone();
                checks.iter().for_each(|check| match check {
                    Op::Greater(cat, num, new_res) => {
                        let new_ranges = leftover_ranges.shifted_greater(cat, *num);
                        queue.push_back((new_ranges, new_res));
                        leftover_ranges = leftover_ranges.shifted_less(cat, *num + 1);
                    }
                    Op::Less(cat, num, new_res) => {
                        let new_ranges = leftover_ranges.shifted_less(cat, *num);
                        queue.push_back((new_ranges, new_res));
                        leftover_ranges = leftover_ranges.shifted_greater(cat, *num - 1);
                    }
                    // should be last iteration
                    Op::Nil(new_res) => queue.push_back((leftover_ranges.clone(), new_res)),
                })
            }
        }
    }
    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
