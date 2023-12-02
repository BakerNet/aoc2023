advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut digits = line.char_indices().filter_map(|c| c.1.to_digit(10));
                let first = digits.next().expect("Expected a digit");
                let last = if let Some(last) = digits.last() {
                    last
                } else {
                    first
                };
                first * 10 + last
            })
            .sum(),
    )
}

static DIGITS: &[(&str, u32); 9] = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn starting_digit(line: &str) -> Option<u32> {
    DIGITS.iter().find_map(|ntup| {
        if line.starts_with(ntup.0) {
            Some(ntup.1)
        } else {
            None
        }
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut digits = line.char_indices().filter_map(|c| {
                    if let Some(digit) = c.1.to_digit(10) {
                        Some(digit)
                    } else {
                        starting_digit(&line[c.0..])
                    }
                });
                let first = digits.next().expect("Expected a digit");
                let last = if let Some(last) = digits.last() {
                    last
                } else {
                    first
                };
                first * 10 + last
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142 + 209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142 + 198));
    }
}
