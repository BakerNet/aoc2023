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

pub fn valid_neighbors_p2(point: (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut ret = Vec::with_capacity(4);
    let width = map.len();
    if point.0 % width == 0 {
        if map[width - 1][point.1 % width] != '#' {
            ret.push((point.0 - 1, point.1));
        }
    } else if map[point.0 % width - 1][point.1 % width] != '#' {
        ret.push((point.0 - 1, point.1));
    }
    if point.0 % width == width - 1 {
        if map[0][point.1 % width] != '#' {
            ret.push((point.0 + 1, point.1));
        }
    } else if map[point.0 % width + 1][point.1 % width] != '#' {
        ret.push((point.0 + 1, point.1));
    }
    if point.1 % width == 0 {
        if map[point.0 % width][width - 1] != '#' {
            ret.push((point.0, point.1 - 1));
        }
    } else if map[point.0 % width][point.1 % width - 1] != '#' {
        ret.push((point.0, point.1 - 1));
    }
    if point.1 % width == width - 1 {
        if map[point.0 % width][0] != '#' {
            ret.push((point.0, point.1 + 1));
        }
    } else if map[point.0 % width][point.1 % width + 1] != '#' {
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

    assert_eq!(map.len(), map[0].len());
    assert_eq!(start.0, map.len() / 2);

    let width = map.len();
    let to_edge = width - start.0 - 1;
    let start = (start.0 + width * 7, start.1 + width * 7);

    // Map is divided into 8 right triangles
    // The start is the stat is the center
    // By travelling to the edge, you fill in one diamond (4 triangles)
    // After reaching edge, every width steps you add a new ring of diamonds
    //
    // To demonstrate, imagine the map is rotated 45 degrees, so you complete squares instead of
    // diamonds
    //
    // afer to_edge steps: 1 diamond
    // .....
    // .....
    // ..#..
    // .....
    // .....
    //
    // after to_edge + width steps: 9 diamonds
    // .....
    // .###.
    // .###.
    // .###.
    // .....
    //
    // after to_edge + width * 2 steps: 27 diamonds
    // #####
    // #####
    // #####
    // #####
    // #####
    //
    // etc.
    //
    // Theory:  The growth from one set of diamonds to the next stabilizes after a few sets
    //
    let (_, diamond_plots) = (1..(to_edge + width * 3) + 1).fold(
        (
            vec![start].iter().copied().collect::<HashSet<_>>(),
            Vec::new(),
        ),
        |mut acc, i| {
            let mut new_acc = HashSet::new();
            acc.0.iter().for_each(|point| {
                let mut neighbors = valid_neighbors_p2(*point, &map);
                neighbors.drain(0..).for_each(|p| {
                    new_acc.insert(p);
                });
            });
            if i >= to_edge && (i - to_edge) % width == 0 {
                acc.1.push(new_acc.len());
            }
            (new_acc, acc.1)
        },
    );

    let first_diamond = diamond_plots[0]; // to_edge
    let nine_diamonds = diamond_plots[1]; // to_edge + width
    let twentyfive_diamonds = diamond_plots[2]; // to_edge + width * 2
    let thirtysix_diamonds = diamond_plots[3]; // to_edge + width * 3
    let diff1 = nine_diamonds - first_diamond;
    let diff2 = twentyfive_diamonds - nine_diamonds;
    let diff3 = thirtysix_diamonds - twentyfive_diamonds;
    assert_ne!(diff1, diff2 - diff1);
    assert_eq!(diff2 - diff1, diff3 - diff2);

    // Above assertion confirms theory - growth per width steps becomes constant
    let transition_growth = diff2 - diff1;

    let steps = 26501365;
    let mut curr_steps = to_edge + width * 2;
    let mut curr_val = twentyfive_diamonds;
    let mut growth = diff2;
    while curr_steps < steps {
        growth += transition_growth;
        curr_val += growth;
        curr_steps += width;
    }
    Some(curr_val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(42));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
