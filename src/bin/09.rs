advent_of_code::solution!(9);

fn parse_nums(line: &str) -> Vec<i64> {
    line.split_whitespace()
        .map(|s| s.parse::<i64>().expect("Expected all items to be integers"))
        .collect()
}

fn find_next_number(series: Vec<i64>) -> i64 {
    if series.iter().all(|x| *x == 0) {
        return 0;
    }
    let diffs: Vec<i64> = series.windows(2).map(|w| w[1] - w[0]).collect();
    series[series.len() - 1] + find_next_number(diffs)
}

fn find_prev_number(series: Vec<i64>) -> i64 {
    if series.iter().all(|x| *x == 0) {
        return 0;
    }
    let diffs: Vec<i64> = series.windows(2).map(|w| w[1] - w[0]).collect();
    series[0] - find_prev_number(diffs)
}

pub fn part_one(input: &str) -> Option<i64> {
    let lines: Vec<i64> = input
        .lines()
        .map(parse_nums)
        .map(find_next_number)
        .collect();
    Some(lines.iter().sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let lines: Vec<i64> = input
        .lines()
        .map(parse_nums)
        .map(find_prev_number)
        .collect();
    Some(lines.iter().sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
