advent_of_code::solution!(6);

fn get_lower_race_quadratic(time: u64, record: u64) -> u64 {
    let delta = time.pow(2) - 4 * record;
    let delta_f64: f64 = delta as f64;
    let delta_sq = delta_f64.sqrt();
    let delta_sq_u64 = delta_sq as u64;
    let root = (time - delta_sq_u64) / 2;
    // usually root + 1, but sometimes off
    if root * (time - root) > record {
        root
    } else {
        root + 1
    }
}

fn extract_vec(s: &str) -> Vec<u64> {
    s.split(':')
        .last()
        .expect("There should be a :")
        .split_whitespace()
        .map(|s| s.parse::<u64>().expect("Times should be numbers"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times: Vec<u64> = extract_vec(lines.next().expect("There should be a first line"));
    let records: Vec<u64> = extract_vec(lines.next().expect("There should be a second line"));
    times
        .iter()
        .zip(records.iter())
        .map(|(&time, &record)| {
            // naive approach - is actually slightly faster
            // let mut x = 1;
            // let winning = loop {
            //     if x * (time - x) > record {
            //         break x;
            //     }
            //     x += 1;
            // };
            let winning = get_lower_race_quadratic(time, record);
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
    // naive approach - is much slower
    // let mut x = 1;
    // let winning = loop {
    //     if x * (time - x) > record {
    //         break x;
    //     }
    //     x += 1;
    // };
    let winning = get_lower_race_quadratic(time, record);
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
