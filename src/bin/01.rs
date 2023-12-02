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

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let mut digits = line.char_indices().filter_map(|c| {
                    if let Some(digit) = c.1.to_digit(10) {
                        Some(digit)
                    } else {
                        if line[c.0..].starts_with("one") {
                            Some(1)
                        } else if line[c.0..].starts_with("two") {
                            Some(2)
                        } else if line[c.0..].starts_with("three") {
                            Some(3)
                        } else if line[c.0..].starts_with("four") {
                            Some(4)
                        } else if line[c.0..].starts_with("five") {
                            Some(5)
                        } else if line[c.0..].starts_with("six") {
                            Some(6)
                        } else if line[c.0..].starts_with("seven") {
                            Some(7)
                        } else if line[c.0..].starts_with("eight") {
                            Some(8)
                        } else if line[c.0..].starts_with("nine") {
                            Some(9)
                        } else {
                            None
                        }
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
