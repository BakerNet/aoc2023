advent_of_code::solution!(11);

fn dist_with_expansion(
    g1: (usize, usize),
    g2: (usize, usize),
    empty_rows: &[usize],
    empty_cols: &[usize],
    expand_by: usize,
) -> u64 {
    let diff_x = if g1.0 > g2.0 {
        let rows_to_expand = empty_rows
            .iter()
            .filter(|&row| *row < g1.0 && *row > g2.0)
            .count();
        g1.0 - g2.0 - rows_to_expand + (rows_to_expand * expand_by)
    } else {
        let rows_to_expand = empty_rows
            .iter()
            .filter(|&row| *row < g2.0 && *row > g1.0)
            .count();
        g2.0 - g1.0 - rows_to_expand + (rows_to_expand * expand_by)
    };
    let diff_y = if g1.1 > g2.1 {
        let cols_to_expand = empty_cols
            .iter()
            .filter(|&col| *col < g1.1 && *col > g2.1)
            .count();
        g1.1 - g2.1 - cols_to_expand + (cols_to_expand * expand_by)
    } else {
        let cols_to_expand = empty_cols
            .iter()
            .filter(|&col| *col < g2.1 && *col > g1.1)
            .count();
        g2.1 - g1.1 - cols_to_expand + (cols_to_expand * expand_by)
    };
    (diff_x + diff_y) as u64
}

fn find_empty_cols(graph: &[Vec<char>]) -> Vec<usize> {
    let mut empty_cols = Vec::new();
    for col in 0..graph[0].len() {
        let mut empty = true;
        for row in graph.iter() {
            if row[col] == '#' {
                empty = false;
            }
        }
        if empty {
            empty_cols.push(col)
        }
    }
    empty_cols
}

pub fn part_one(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let empty_cols = find_empty_cols(&graph);
    let empty_rows: Vec<usize> = graph
        .iter()
        .enumerate()
        .filter_map(|(row, row_vec)| {
            if !row_vec.contains(&'#') {
                Some(row)
            } else {
                None
            }
        })
        .collect();
    let galaxies = graph
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .filter(|&(_, c)| *c == '#')
                .for_each(|(col, _)| acc.push((row, col)));
            acc
        });
    let mut galaxy_pairs = Vec::new();
    for (x, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(x + 1) {
            galaxy_pairs.push((g1, g2));
        }
    }
    Some(
        galaxy_pairs
            .iter()
            .map(|&(g1, g2)| dist_with_expansion(*g1, *g2, &empty_rows, &empty_cols, 2))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let graph: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let empty_cols = find_empty_cols(&graph);
    let empty_rows: Vec<usize> = graph
        .iter()
        .enumerate()
        .filter_map(|(row, row_vec)| {
            if !row_vec.contains(&'#') {
                Some(row)
            } else {
                None
            }
        })
        .collect();
    let galaxies = graph
        .iter()
        .enumerate()
        .fold(Vec::new(), |mut acc, (row, row_vec)| {
            row_vec
                .iter()
                .enumerate()
                .filter(|&(_, c)| *c == '#')
                .for_each(|(col, _)| acc.push((row, col)));
            acc
        });
    let mut galaxy_pairs = Vec::new();
    for (x, g1) in galaxies.iter().enumerate() {
        for g2 in galaxies.iter().skip(x + 1) {
            galaxy_pairs.push((g1, g2));
        }
    }
    Some(
        galaxy_pairs
            .iter()
            .map(|&(g1, g2)| dist_with_expansion(*g1, *g2, &empty_rows, &empty_cols, 1_000_000))
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(82_000_210));
    }
}
