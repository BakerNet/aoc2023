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
                let reds = red_re
                    .captures_iter(line)
                    .filter(|c| c[1].parse::<u32>().expect("Red capture should be int") > 12)
                    .count();
                let greens = green_re
                    .captures_iter(line)
                    .filter(|c| c[1].parse::<u32>().expect("Green capture should be int") > 13)
                    .count();
                let blues = blue_re
                    .captures_iter(line)
                    .filter(|c| c[1].parse::<u32>().expect("Blue capture should be int") > 14)
                    .count();
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
            .filter_map(|line| {
                let red_re = Regex::new(r" (\d+) red").unwrap();
                let green_re = Regex::new(r" (\d+) green").unwrap();
                let blue_re = Regex::new(r" (\d+) blue").unwrap();
                let reds = red_re
                    .captures_iter(line)
                    .map(|c| c[1].parse::<u32>().expect("Red capture should be int"))
                    .reduce(|acc, e| if e > acc { e } else { acc })
                    .unwrap();
                let greens = green_re
                    .captures_iter(line)
                    .map(|c| c[1].parse::<u32>().expect("green capture should be int"))
                    .reduce(|acc, e| if e > acc { e } else { acc })
                    .unwrap();
                let blues = blue_re
                    .captures_iter(line)
                    .map(|c| c[1].parse::<u32>().expect("blue capture should be int"))
                    .reduce(|acc, e| if e > acc { e } else { acc })
                    .unwrap();
                Some(reds * greens * blues)
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
