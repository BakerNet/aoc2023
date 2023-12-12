use std::collections::HashMap;

advent_of_code::solution!(12);

fn parse_record(line: &str) -> (String, Vec<usize>) {
    let items: Vec<&str> = line.split_whitespace().collect();
    (
        items[0].chars().collect(),
        items[1]
            .split(',')
            .map(str::parse::<usize>)
            .map(|r| r.expect("List should parse to ints"))
            .collect(),
    )
}

// fn do_vecs_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool {
//     if a.len() != b.len() {
//         return false;
//     }
//     a.iter().zip(b.iter()).filter(|&(a, b)| a == b).count() == a.len()
// }
//
// // naive solution
// fn possible_arrangements(items: String, damaged: Vec<usize>) -> usize {
//     let operational_counts: Vec<usize> = items
//         .split_whitespace()
//         .map(|substr| substr.len())
//         .collect();
//     if do_vecs_match(&damaged, &operational_counts) {
//         return 1;
//     }
//     let filtered_counts: Vec<usize> = items
//         .split_whitespace()
//         .take_while(|s| !s.contains('?'))
//         .map(|substr| substr.len())
//         .collect();
//     if filtered_counts
//         .iter()
//         .zip(&damaged)
//         .find(|(a, b)| a != b)
//         .is_some()
//     {
//         return 0;
//     }
//     for (pos, char) in items.chars().enumerate() {
//         if char == '?' {
//             let mut replace_damaged = items.clone();
//             replace_damaged.replace_range(pos..pos + 1, "#");
//             let mut replace_operational = items;
//             replace_operational.replace_range(pos..pos + 1, " ");
//             return possible_arrangements(replace_operational, damaged.clone())
//                 + possible_arrangements(replace_damaged, damaged);
//         }
//     }
//     0
// }

fn possible_arrangements_dp(
    items: &String,
    damaged: &Vec<usize>,
    item_pos: usize,
    damaged_pos: usize,
    curr_block_size: usize,
    seen: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    let key = (item_pos, damaged_pos, curr_block_size);
    if let Some(x) = seen.get(&key) {
        return *x;
    }
    if item_pos == items.len() {
        if damaged_pos == damaged.len() && curr_block_size == 0 {
            return 1;
        } else if damaged_pos == damaged.len() - 1 && curr_block_size == damaged[damaged_pos] {
            return 1;
        } else {
            return 0;
        }
    }
    let dot_ans = |seen: &mut HashMap<(usize, usize, usize), usize>| {
        if curr_block_size == 0 {
            possible_arrangements_dp(items, damaged, item_pos + 1, damaged_pos, 0, seen)
        } else if damaged_pos < damaged.len() && curr_block_size == damaged[damaged_pos] {
            possible_arrangements_dp(items, damaged, item_pos + 1, damaged_pos + 1, 0, seen)
        } else {
            0
        }
    };
    let hash_ans = |seen: &mut HashMap<(usize, usize, usize), usize>| {
        possible_arrangements_dp(
            items,
            damaged,
            item_pos + 1,
            damaged_pos,
            curr_block_size + 1,
            seen,
        )
    };
    let answer = match items.chars().collect::<Vec<char>>()[item_pos] {
        '.' => dot_ans(seen),
        '#' => hash_ans(seen),
        '?' => dot_ans(seen) + hash_ans(seen),
        _ => panic!("Unexpected char"),
    };
    seen.insert(key, answer);
    answer
}

pub fn part_one(input: &str) -> Option<u64> {
    let num_arrangements: Vec<usize> = input
        .lines()
        .map(parse_record)
        .map(|(items, damaged)| {
            possible_arrangements_dp(&items, &damaged, 0, 0, 0, &mut HashMap::new())
        })
        .collect();
    Some(num_arrangements.iter().sum::<usize>() as u64)
}

fn parse_part2(items: String, damaged: Vec<usize>) -> (String, Vec<usize>) {
    let mut items_x5: String = (items + "?").repeat(5);
    items_x5.pop();
    let damaged_x5 = damaged.repeat(5);
    (items_x5, damaged_x5)
}

pub fn part_two(input: &str) -> Option<u64> {
    let num_arrangements: Vec<usize> = input
        .lines()
        .map(parse_record)
        .map(|(items, damaged)| parse_part2(items, damaged))
        .map(|(items, damaged)| {
            possible_arrangements_dp(&items, &damaged, 0, 0, 0, &mut HashMap::new())
        })
        .collect();
    Some(num_arrangements.iter().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
