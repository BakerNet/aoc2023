use std::cmp;

advent_of_code::solution!(13);

fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let matching = a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count();
    matching == a.len() && matching == b.len()
}

fn do_vecs_differ_by_one<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
    let not_matching = a.iter().zip(b.iter()).filter(|&(a, b)| a != b).count();
    not_matching == 1
}

fn find_vertical_reflection(map: &Vec<Vec<char>>, type2: bool) -> Option<usize> {
    // invert map because horizontal check is simpler
    let mut new = Vec::new();
    for col in 0..map[0].len() {
        new.push(Vec::new());
        for row in 0..map.len() {
            new[col].push(map[row][col]);
        }
    }
    find_horizontal_reflection(&new, type2)
}

fn find_horizontal_reflection(map: &Vec<Vec<char>>, type2: bool) -> Option<usize> {
    for row in 0..map.len() - 1 {
        let mut reflection = !type2;
        for diff in 0..=cmp::min(row, map.len() - row - 2) {
            if !do_vecs_match(&map[row - diff], &map[row + diff + 1]) {
                if type2
                    && !reflection
                    && do_vecs_differ_by_one(&map[row - diff], &map[row + diff + 1])
                {
                    reflection = true;
                } else {
                    reflection = false;
                    break;
                }
            }
        }
        if reflection {
            return Some(row);
        }
    }
    None
}

fn find_reflection_value(map: &Vec<&str>, type2: bool) -> usize {
    let map = map.iter().map(|s| s.chars().collect()).collect();
    if let Some(x) = find_vertical_reflection(&map, type2) {
        return x + 1;
    }
    if let Some(x) = find_horizontal_reflection(&map, type2) {
        return (x + 1) * 100;
    }
    panic!("No reflection found: type 2 is {} - {:?}", type2, map);
}

pub fn part_one(input: &str) -> Option<u64> {
    let groups: Vec<Vec<&str>> = vec![Vec::new()];
    let groups: Vec<Vec<&str>> = input
        .lines()
        .fold((0, groups), |mut acc, s| {
            if s == "" {
                acc.1.push(Vec::new());
                acc.0 += 1;
            } else {
                acc.1[acc.0].push(s);
            }
            acc
        })
        .1;
    Some(
        groups
            .iter()
            .map(|map| find_reflection_value(map, false))
            .sum::<usize>() as u64,
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let groups: Vec<Vec<&str>> = vec![Vec::new()];
    let groups: Vec<Vec<&str>> = input
        .lines()
        .fold((0, groups), |mut acc, s| {
            if s == "" {
                acc.1.push(Vec::new());
                acc.0 += 1;
            } else {
                acc.1[acc.0].push(s);
            }
            acc
        })
        .1;
    Some(
        groups
            .iter()
            .map(|map| find_reflection_value(map, true))
            .sum::<usize>() as u64,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
