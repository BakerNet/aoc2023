use std::ops::{Add, Sub};

advent_of_code::solution!(5);

// Handle unsigned-safe shifting from start to dest for any unsigned int type
fn unsigned_safe_shift<T>(dest: T, start: T, input: T) -> T
where
    T: Ord + Sub<Output = T> + Add<Output = T>,
{
    if dest > start {
        input + (dest - start)
    } else {
        input - (start - dest)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines();
    let mut seeds: Vec<u32> = lines
        .next()
        .expect("There should be at least one line")
        .split(':')
        .last()
        .expect("Expected seeds format")
        .split_whitespace()
        .map(|s| str::parse::<u32>(s).expect("Expected seeds to be numbers"))
        .collect();

    let mut section: Vec<(u32, u32, u32)> = Vec::new();
    let transform_seed = |x: u32, section: &Vec<(u32, u32, u32)>| {
        for &(dest, start, range) in section.iter() {
            if x >= start && x < start + range {
                return unsigned_safe_shift(dest, start, x);
            }
        }
        x
    };
    lines.next(); // burn an empty line
    for line in lines {
        let trimmed = line.trim();
        if trimmed.ends_with(':') {
            // skip title
            continue;
        }
        if trimmed.is_empty() {
            // end of section - use map
            seeds = seeds.iter().map(|&x| transform_seed(x, &section)).collect();
            section = Vec::new();
        } else {
            // build section maps
            let trimmed: Vec<u32> = line
                .split_whitespace()
                .map(|s| str::parse::<u32>(s).expect("Expected maps to all be numbers"))
                .collect();
            section.push((trimmed[0], trimmed[1], trimmed[2]))
        }
    }
    seeds = seeds.iter().map(|&x| transform_seed(x, &section)).collect();
    Some(*seeds.iter().min().unwrap())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let seed_nums: Vec<u64> = lines
        .next()
        .expect("There should be at least one line")
        .split(':')
        .last()
        .expect("Expected seeds format")
        .split_whitespace()
        .map(|s| str::parse::<u64>(s).expect("Expected seeds to be numbers"))
        .collect();

    let mut seeds: Vec<(u64, u64)> = seed_nums
        .chunks(2)
        .map(|chunk| {
            let seed_num = chunk[0];
            let range = chunk[1];
            (seed_num, range)
        })
        .collect();

    let mut section: Vec<(u64, u64, u64)> = Vec::new();
    let transform_seed_range = |x: (u64, u64), section: &Vec<(u64, u64, u64)>| {
        // println!("Testing ({}, {})", x.0, x.1);
        let mut ret: Vec<(u64, u64)> = Vec::new();
        let mut unfound: Vec<(u64, u64)> = vec![x];
        for &(dest, start, range) in section.iter() {
            let searching = unfound.clone();
            unfound = Vec::new();
            for &(seed_start, seed_range) in searching.iter() {
                if seed_start >= start && seed_start + seed_range < start + range {
                    // seed range is inside section range - searching done
                    ret.push((unsigned_safe_shift(dest, start, seed_start), seed_range));
                } else if start >= seed_start && start + range < seed_start + seed_range {
                    // seed range surrounds section range - searching will be split in two
                    ret.push((dest, range));
                    let first_unfound_range = start - seed_start;
                    unfound.push((seed_start, first_unfound_range));
                    unfound.push((start + range, seed_range - range - first_unfound_range));
                } else if seed_start >= start && seed_start < start + range {
                    // seed range partial overalp  with section range on left - searching range will change
                    let overlap_range = start + range - seed_start;
                    ret.push((unsigned_safe_shift(dest, start, seed_start), overlap_range));
                    unfound.push((seed_start + overlap_range, seed_range - overlap_range));
                } else if start >= seed_start && start < seed_start + seed_range {
                    // seed range partial overlap with section range on right - searching range will change
                    let overlap_range = seed_start + seed_range - start;
                    ret.push((dest, overlap_range));
                    unfound.push((seed_start, seed_range - overlap_range));
                } else {
                    // no overlap
                    unfound.push((seed_start, seed_range));
                }
            }
            if unfound.is_empty() {
                break;
            }
        }
        ret.extend(unfound);
        ret
    };
    lines.next(); // burn an empty line
    for line in lines {
        let trimmed = line.trim();
        if trimmed.ends_with(':') {
            // skip title
            continue;
        }
        if trimmed.is_empty() {
            // end of section - use map
            seeds = seeds.iter().fold(Vec::new(), |mut acc, &x| {
                acc.extend(transform_seed_range(x, &section));
                acc
            });
            section = Vec::new();
        } else {
            // build section maps
            let trimmed: Vec<u64> = line
                .split_whitespace()
                .map(|s| str::parse::<u64>(s).expect("Expected maps to all be numbers"))
                .collect();
            section.push((trimmed[0], trimmed[1], trimmed[2]))
        }
    }
    seeds = seeds.iter().fold(Vec::new(), |mut acc, &x| {
        acc.extend(transform_seed_range(x, &section));
        acc
    });
    Some(seeds.iter().map(|&x| x.0).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
