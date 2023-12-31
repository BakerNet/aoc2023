use std::collections::HashSet;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let cards: Vec<u32> = input
        .lines()
        .map(|line| {
            let split: Vec<HashSet<u32>> = line
                .split(':')
                .last()
                .expect("Should be a string afer :")
                .split('|')
                .map(|s| {
                    s.split_whitespace()
                        .map(|ss| ss.parse::<u32>().expect("Should all be numbers"))
                        .collect()
                })
                .collect();
            split[0].intersection(&split[1]).count().try_into().unwrap()
        })
        .collect();
    let sum: u32 = cards
        .iter()
        .filter_map(|&count| {
            if count < 1 {
                None
            } else {
                Some(u32::pow(2, count - 1))
            }
        })
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards: Vec<u32> = input
        .lines()
        .map(|line| {
            let split: Vec<HashSet<u32>> = line
                .split(':')
                .last()
                .expect("Should be a string afer :")
                .split('|')
                .map(|s| {
                    s.split_whitespace()
                        .map(|ss| ss.parse::<u32>().expect("Should all be numbers"))
                        .collect()
                })
                .collect();
            split[0].intersection(&split[1]).count().try_into().unwrap()
        })
        .collect();
    let mut cards_count = vec![1_u32; cards.len()];
    cards.iter().enumerate().for_each(|(index, &num)| {
        if num < 1 {
            return;
        }
        for _ in 0..cards_count[index] {
            for i in 1..=num {
                cards_count[index + (i as usize)] += 1;
            }
        }
    });
    Some(cards_count.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
