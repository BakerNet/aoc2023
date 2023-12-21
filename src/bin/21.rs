use std::collections::HashSet;

advent_of_code::solution!(21);

pub fn valid_neighbors(point: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::with_capacity(4);
    if point.0 > 0 && map[point.0 - 1][point.1] != '#' {
        ret.push((point.0 - 1, point.1));
    }
    if point.0 < map.len() - 1 && map[point.0 + 1][point.1] != '#' {
        ret.push((point.0 + 1, point.1));
    }
    if point.1 > 0 && map[point.0][point.1 - 1] != '#' {
        ret.push((point.0, point.1 - 1));
    }
    if point.1 < map[0].len() - 1 && map[point.0][point.1 + 1] != '#' {
        ret.push((point.0, point.1 + 1));
    }
    ret
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let start = map
        .iter()
        .enumerate()
        .find_map(|(row, v)| {
            v.iter()
                .enumerate()
                .find_map(|(col, c)| if *c == 'S' { Some(col) } else { None })
                .map(|col| (row, col))
        })
        .expect("There should be a starting tile.");
    let final_plots = (0..64).fold(
        vec![start].iter().copied().collect::<HashSet<_>>(),
        |acc, _| {
            let mut new_acc = HashSet::new();
            acc.iter().for_each(|point| {
                let mut neighbors = valid_neighbors(*point, &map);
                neighbors.drain(0..).for_each(|p| {
                    new_acc.insert(p);
                });
            });
            new_acc
        },
    );
    Some(final_plots.len() as u64)
}

pub fn part_two(input: &str) -> Option<usize> {
    // I actually have no idea how to do part 2
    input.find(|_| false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
