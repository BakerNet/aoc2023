advent_of_code::solution!(6);

fn extract_vec(s: &str) -> Vec<u32> {
    s.split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Times should be numbers"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times: Vec<u32> = extract_vec(lines.next().expect("There should be a first line"));
    let records: Vec<u32> = extract_vec(lines.next().expect("There should be a second line"));
    times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            let mut x = 1;
            // optimize with binary search instead linear search
            let winning = loop {
                if x * (time - x) > record {
                    break x;
                }
                x += 1;
            };
            time - winning - winning + 1
        })
        .reduce(|acc, x| acc * x)
}

fn extract_num(s: &str) -> u64 {
    s.split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s)
        .parse::<u64>()
        .expect("Records should be numbers")
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time: u64 = extract_num(lines.next().expect("There should be a first line"));
    let record: u64 = extract_num(lines.next().expect("There should be a second line"));
    let mut x = 1;
    // optimize with binary search instead linear search
    let winning = loop {
        if x * (time - x) > record {
            break x;
        }
        x += 1;
    };
    Some(time - winning - winning + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
