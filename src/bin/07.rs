use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(7);

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u64,
    type2: bool,
}

impl Hand {
    fn from(line: &str) -> Self {
        let line: Vec<&str> = line.split_whitespace().collect();
        Hand {
            cards: line[0].to_owned(),
            bid: line[1]
                .parse::<u64>()
                .expect("Expected second item bid number"),
            type2: false,
        }
    }

    fn from_type2(line: &str) -> Self {
        let line: Vec<&str> = line.split_whitespace().collect();
        Hand {
            cards: line[0].to_owned(),
            bid: line[1]
                .parse::<u64>()
                .expect("Expected second item bid number"),
            type2: true,
        }
    }

    fn strength(&self) -> u64 {
        let mut groups = self.cards.chars().fold(HashMap::new(), |mut acc, c| {
            acc.entry(c).and_modify(|x| *x += 1).or_insert(1);
            acc
        });
        if self.type2 && groups.len() != 1 && groups.contains_key(&'J') {
            let j = groups.remove(&'J').unwrap();
            let max_item = groups
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .expect("There should be a max in groups for J logic")
                .0;
            groups.entry(*max_item).and_modify(|x| *x += j);
        }
        let mut groups = groups.values().collect::<Vec<_>>();
        groups.sort();
        groups.reverse();
        match groups.len() {
            1 => 7,
            2 => {
                if *groups[0] == 4 {
                    6
                } else {
                    5
                }
            }
            3 => {
                if *groups[0] == 3 {
                    4
                } else {
                    3
                }
            }
            4 => 2,
            5 => 1,
            _ => panic!("Unexpected card group length"),
        }
    }
}

fn card_map(c: char, type2: bool) -> u64 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => {
            if type2 {
                0
            } else {
                11
            }
        }
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        _ => panic!("Unexpected char checked"),
    }
}

fn card_cmp(first: char, second: char, type2: bool) -> Ordering {
    card_map(first, type2).cmp(&card_map(second, type2))
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_strength = self.strength();
        let other_strength = other.strength();
        if self_strength == other_strength {
            let diff = self
                .cards
                .chars()
                .zip(other.cards.chars())
                .find(|&(a, b)| a != b)
                .expect("Two hands shouldn't be identical");
            card_cmp(diff.0, diff.1, self.type2)
        } else {
            self_strength.cmp(&other_strength)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

pub fn part_one(input: &str) -> Option<u64> {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from).collect();
    hands.sort();
    let total_winnings = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (hand.bid * ((index as u64) + 1))
    });
    Some(total_winnings)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut hands: Vec<Hand> = input.lines().map(Hand::from_type2).collect();
    hands.sort();
    let total_winnings = hands.iter().enumerate().fold(0, |acc, (index, hand)| {
        acc + (hand.bid * ((index as u64) + 1))
    });
    Some(total_winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
