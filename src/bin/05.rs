use std::ops::{Add, Range, Sub};

advent_of_code::solution!(5);

// Handle unsigned-safe shifting from start to dest for any unsigned int type
fn unsigned_safe_shift<T>(to_change: T, start: T, dest: T) -> T
where
    T: Ord + Sub<Output = T> + Add<Output = T>,
{
    if dest > start {
        to_change + (dest - start)
    } else {
        to_change - (start - dest)
    }
}

trait SubDiv
where
    Self: Sized,
{
    fn subdiv(&self, other: &Self) -> (Option<Self>, Option<Vec<Self>>);
}

impl<T> SubDiv for Range<T>
where
    T: PartialOrd<T> + Copy,
{
    fn subdiv(&self, other: &Self) -> (Option<Self>, Option<Vec<Self>>) {
        if self.start >= other.start && self.end < other.end {
            // self range is inside other range
            (Some(self.clone()), None)
        } else if other.start >= self.start && other.end < self.end {
            // self range surrounds other range
            let overlapping_range = Some(other.clone());
            let non_overlapping = Some(vec![
                Range {
                    start: self.start,
                    end: other.start,
                },
                Range {
                    start: other.end,
                    end: self.end,
                },
            ]);
            (overlapping_range, non_overlapping)
        } else if self.start >= other.start && self.start < other.end {
            // self range partial overalp  with other range on left
            let overlapping_range = Some(Range {
                start: self.start,
                end: other.end,
            });
            let non_overlapping = Some(vec![Range {
                start: other.end,
                end: self.end,
            }]);
            (overlapping_range, non_overlapping)
        } else if other.start >= self.start && other.start < self.end {
            // self range partial overlap with other range on right
            let overlapping_range = Some(Range {
                start: other.start,
                end: self.end,
            });
            let non_overlapping = Some(vec![Range {
                start: self.start,
                end: other.start,
            }]);
            (overlapping_range, non_overlapping)
        } else {
            // no overlap
            (None, Some(vec![self.clone()]))
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .expect("There should be at least one line")
        .split(':')
        .last()
        .expect("Expected seeds format")
        .split_whitespace()
        .map(|s| str::parse::<u64>(s).expect("Expected seeds to be numbers"))
        .collect();

    lines.next(); // burn an empty line
    let transformers = lines.fold(Vec::new(), |mut acc, line| {
        let trimmed = line.trim();
        if trimmed.ends_with(':') {
            // start of section
            acc.push(Vec::new());
        } else if trimmed.is_empty() {
            // end of section - do nothing
        } else {
            // add map to section transformer
            let trimmed: Vec<u64> = line
                .split_whitespace()
                .map(|s| str::parse::<u64>(s).expect("Expected maps to all be numbers"))
                .collect();
            let len = acc.len();
            acc[len - 1].push((
                trimmed[0],
                Range {
                    start: trimmed[1],
                    end: trimmed[1] + trimmed[2],
                },
            ))
        }
        acc
    });

    let transform_item = |x: u64, transformer: &Vec<(u64, Range<u64>)>| {
        for (dest, range) in transformer.iter() {
            if x >= range.start && x < range.end {
                return unsigned_safe_shift(x, range.start, *dest);
            }
        }
        x
    };
    let locations = transformers.iter().fold(seeds, |acc, transformer| {
        acc.iter()
            .map(|&x| transform_item(x, transformer))
            .collect()
    });
    Some(*locations.iter().min().unwrap())
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

    let seeds: Vec<Range<u64>> = seed_nums
        .chunks(2)
        .map(|chunk| {
            let seed_num = chunk[0];
            let range = chunk[1];
            Range {
                start: seed_num,
                end: seed_num + range,
            }
        })
        .collect();

    lines.next(); // burn an empty line
    let transformers = lines.fold(Vec::new(), |mut acc, line| {
        let trimmed = line.trim();
        if trimmed.ends_with(':') {
            // start of section
            acc.push(Vec::new());
        } else if trimmed.is_empty() {
            // end of section - do nothing
        } else {
            // add map to section transformer
            let trimmed: Vec<u64> = line
                .split_whitespace()
                .map(|s| str::parse::<u64>(s).expect("Expected maps to all be numbers"))
                .collect();
            let len = acc.len();
            acc[len - 1].push((
                trimmed[0],
                Range {
                    start: trimmed[1],
                    end: trimmed[1] + trimmed[2],
                },
            ))
        }
        acc
    });

    let transform_range = |x: Range<u64>, transformer: &Vec<(u64, Range<u64>)>| {
        let mut returns = Vec::new();
        let mut searching = vec![x];
        for (dest, transform_range) in transformer.iter() {
            let search_clone = searching.clone();
            for check_range in search_clone.iter() {
                let (overlapping, non_overlapping) = check_range.subdiv(transform_range);
                if let Some(overlapping) = overlapping {
                    returns.push(Range {
                        start: unsigned_safe_shift(overlapping.start, transform_range.start, *dest),
                        end: unsigned_safe_shift(overlapping.end, transform_range.start, *dest),
                    });
                }
                if let Some(non_overlapping) = non_overlapping {
                    searching = non_overlapping;
                } else {
                    searching = Vec::new();
                }
            }
            if searching.is_empty() {
                break;
            }
        }
        returns.extend(searching);
        returns
    };

    let locations = transformers.iter().fold(seeds, |acc, transformer| {
        acc.iter().fold(Vec::new(), |mut inner_acc, x| {
            inner_acc.extend(transform_range(x.clone(), transformer));
            inner_acc
        })
    });
    Some(locations.iter().map(|x| x.start).min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subdiv_range() {
        let range1 = Range { start: 5, end: 10 };

        // self surrounds other
        let range2 = Range { start: 7, end: 9 };
        let rest = range1.subdiv(&range2);
        assert_eq!(rest.0, Some(Range { start: 7, end: 9 }));
        let Some(non_overlapping) = rest.1 else {
            panic!("Should be non_overlapping");
        };
        assert_eq!(non_overlapping.len(), 2);
        assert_eq!(non_overlapping[0], Range { start: 5, end: 7 });
        assert_eq!(non_overlapping[1], Range { start: 9, end: 10 });

        // no overlapping range
        let range3 = Range { start: 1, end: 5 };
        let rest = range1.subdiv(&range3);
        assert_eq!(rest.0, None);
        let Some(non_overlapping) = rest.1 else {
            panic!("Should be non_overlapping");
        };
        assert_eq!(non_overlapping.len(), 1);
        assert_eq!(non_overlapping[0], Range { start: 5, end: 10 });

        // overlapping range left
        let range4 = Range { start: 1, end: 8 };
        let rest = range1.subdiv(&range4);
        assert_eq!(rest.0, Some(Range { start: 5, end: 8 }));
        let Some(non_overlapping) = rest.1 else {
            panic!("Should be non_overlapping");
        };
        assert_eq!(non_overlapping.len(), 1);
        assert_eq!(non_overlapping[0], Range { start: 8, end: 10 });

        // overlapping range left
        let range5 = Range { start: 7, end: 15 };
        let rest = range1.subdiv(&range5);
        assert_eq!(rest.0, Some(Range { start: 7, end: 10 }));
        let Some(non_overlapping) = rest.1 else {
            panic!("Should be non_overlapping");
        };
        assert_eq!(non_overlapping.len(), 1);
        assert_eq!(non_overlapping[0], Range { start: 5, end: 7 });

        // self surrounded by other
        let range6 = Range { start: 2, end: 15 };
        let rest = range1.subdiv(&range6);
        assert_eq!(rest.0, Some(Range { start: 5, end: 10 }));
        assert_eq!(rest.1, None);
    }

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
