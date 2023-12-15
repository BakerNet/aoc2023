advent_of_code::solution!(15);

fn hash_it(s: &str) -> usize {
    s.as_bytes()
        .iter()
        .map(|x| *x as usize)
        .fold(0, |acc, x| (acc + x) * 17 % 256)
}

pub fn part_one(input: &str) -> Option<u64> {
    let sum = input
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(hash_it)
        .sum::<usize>();
    Some(sum as u64)
}

enum Op {
    Rem,
    Set(usize),
}

impl Op {
    fn from(s: &str) -> Self {
        match s.chars().next().unwrap() {
            '-' => Self::Rem,
            '=' => {
                let num = &s[1..2]
                    .parse::<usize>()
                    .expect("= should be followed by number");
                Self::Set(*num)
            }
            _ => panic!("Unexpected op {}", s),
        }
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
    input.lines().next().unwrap().split(',').for_each(|s| {
        let len = if s.chars().last().unwrap() == '-' {
            s.len() - 1
        } else {
            s.len() - 2
        };
        let label = &s[0..len];
        let hash_val = hash_it(label);
        let op = Op::from(&s[len..]);
        if let Some(index) = boxes[hash_val].iter().position(|item| item.0 == label) {
            match op {
                Op::Rem => {
                    boxes[hash_val].remove(index);
                }
                Op::Set(x) => {
                    boxes[hash_val][index] = (label.to_owned(), x);
                }
            }
        } else if let Op::Set(x) = op {
            boxes[hash_val].push((label.to_owned(), x));
        }
    });
    let sum = boxes
        .iter()
        .enumerate()
        .map(|(num, v)| {
            v.iter()
                .enumerate()
                .map(|(index, item)| (1 + num) * (1 + index) * item.1)
                .sum::<usize>()
        })
        .sum::<usize>();
    Some(sum as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
