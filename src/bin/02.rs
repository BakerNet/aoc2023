advent_of_code::solution!(2);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .filter_map(|line| {
                let game_re = Regex::new(r"^Game (\d+):").unwrap();
                let red_re = Regex::new(r" (\d+) red").unwrap();
                let green_re = Regex::new(r" (\d+) green").unwrap();
                let blue_re = Regex::new(r" (\d+) blue").unwrap();
                let game_id = game_re.captures(line).expect("Expected game_id to exist")[1]
                    .parse::<u32>()
                    .expect("Expect game_id to be an int");
                let large_count = |re: Regex, max: u32| {
                    re.captures_iter(line)
                        .filter(|c| c[1].parse::<u32>().expect("Color capture should be int") > max)
                        .count()
                };
                let reds = large_count(red_re, 12);
                let greens = large_count(green_re, 13);
                let blues = large_count(blue_re, 14);
                if reds != 0 || greens != 0 || blues != 0 {
                    None
                } else {
                    Some(game_id)
                }
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let red_re = Regex::new(r" (\d+) red").unwrap();
                let green_re = Regex::new(r" (\d+) green").unwrap();
                let blue_re = Regex::new(r" (\d+) blue").unwrap();
                let largest_capture = |re: Regex| {
                    re.captures_iter(line)
                        .map(|c| c[1].parse::<u32>().expect("Color capture should be int"))
                        .reduce(|acc, e| if e > acc { e } else { acc })
                        .unwrap()
                };
                let reds = largest_capture(red_re);
                let greens = largest_capture(green_re);
                let blues = largest_capture(blue_re);
                reds * greens * blues
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
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
