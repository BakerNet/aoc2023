advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let times: Vec<u32> = lines
        .next()
        .expect("There should be a first line")
        .split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Times should be numbers"))
        .collect();
    let records: Vec<u32> = lines
        .next()
        .expect("There should be a second line")
        .split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .map(|s| s.parse::<u32>().expect("Records should be numbers"))
        .collect();
    times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            let mut x = 1;
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

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .expect("There should be a first line")
        .split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s)
        .parse::<u64>()
        .expect("Records should be numbers");
    let record: u64 = lines
        .next()
        .expect("There should be a second line")
        .split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .fold(String::new(), |acc, s| acc + s)
        .parse::<u64>()
        .expect("Records should be numbers");
    let mut x = 1;
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
