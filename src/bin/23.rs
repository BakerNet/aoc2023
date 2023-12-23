advent_of_code::solution!(23);

use std::cmp::{self, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};

type Point = (usize, usize);

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
    visited: HashSet<Point>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| other.position.cmp(&self.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn neighbors(map: &[Vec<char>], position: Point, part2: bool) -> Vec<Point> {
    let mut ret = Vec::new();
    if !part2 {
        match map[position.0][position.1] {
            '^' if position.0 > 0 => ret.push((position.0 - 1, position.1)),
            'v' if position.0 < map.len() - 1 => ret.push((position.0 + 1, position.1)),
            '<' if position.1 > 0 => ret.push((position.0, position.1 - 1)),
            '>' if position.1 < map[0].len() - 1 => ret.push((position.0, position.1 + 1)),
            _ => {}
        }
        if !ret.is_empty() {
            return ret;
        }
    }
    if position.0 > 0 && map[position.0 - 1][position.1] != '#' {
        ret.push((position.0 - 1, position.1));
    }
    if position.0 < map.len() - 1 && map[position.0 + 1][position.1] != '#' {
        ret.push((position.0 + 1, position.1));
    }
    if position.1 > 0 && map[position.0][position.1 - 1] != '#' {
        ret.push((position.0, position.1 - 1));
    }
    if position.1 < map[0].len() - 1 && map[position.0][position.1 + 1] != '#' {
        ret.push((position.0, position.1 + 1));
    }
    ret
}

fn valid_neighbors(
    map: &[Vec<char>],
    position: Point,
    visited: &HashSet<Point>,
    part2: bool,
) -> Vec<Point> {
    let neighbors = neighbors(map, position, part2);

    let remove_visited = |v: Vec<Point>| {
        let mut v = v;
        v.drain(0..)
            .filter(|pos| !visited.contains(pos))
            .collect::<Vec<_>>()
    };

    remove_visited(neighbors)
}

fn greedy_dfs(map: &[Vec<char>], start: Point, goal: Point, part2: bool) -> usize {
    let mut dist: HashMap<Point, usize> = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back(State {
        cost: 0,
        position: start,
        visited: vec![start].iter().copied().collect(),
    });

    let mut best = 0;

    while let Some(State {
        cost,
        position,
        visited,
    }) = queue.pop_back()
    {
        if position == goal {
            best = cmp::max(best, cost);
            continue;
        }

        if let Some(found) = dist.get(&position) {
            if *found > cost {
                continue;
            }
        }

        for new_pos in valid_neighbors(map, position, &visited, part2) {
            let mut new_visited = visited.clone();
            new_visited.insert(new_pos);
            let new_cost = cost + 1;
            let next = State {
                cost: new_cost,
                position: new_pos,
                visited: new_visited,
            };

            if let Some(found) = dist.get(&new_pos) {
                if next.cost > *found {
                    queue.push_back(next);
                    dist.insert(new_pos, new_cost);
                }
            } else {
                queue.push_back(next);
                dist.insert(new_pos, new_cost);
            }
        }
    }
    best
}

fn points_of_interest(map: &[Vec<char>], start: Point, goal: Point) -> Vec<Point> {
    let mut points = vec![start, goal];
    map.iter().enumerate().for_each(|(row, v)| {
        v.iter().enumerate().for_each(|(col, c)| {
            if *c != '#' && neighbors(map, (row, col), true).len() > 2 {
                points.push((row, col));
            }
        })
    });
    points
}

fn poi_map(map: &[Vec<char>], poi: Vec<Point>) -> HashMap<Point, Vec<(Point, usize)>> {
    let mut ret: HashMap<Point, Vec<(Point, usize)>> = HashMap::new();
    let poi_set: HashSet<_> = poi.iter().copied().collect();
    poi.iter().for_each(|p| {
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back((*p, 0));
        seen.insert(*p);
        while !queue.is_empty() {
            let (curr, dist) = queue.pop_front().unwrap();
            let dist = dist + 1;
            for neighbor in neighbors(map, curr, true) {
                if seen.contains(&neighbor) {
                    continue;
                }
                if poi_set.contains(&neighbor) {
                    seen.insert(neighbor);
                    ret.entry(*p)
                        .and_modify(|v| v.push((neighbor, dist)))
                        .or_insert(vec![(neighbor, dist)]);
                } else {
                    seen.insert(neighbor);
                    queue.push_back((neighbor, dist));
                }
            }
        }
    });

    ret
}

fn poi_dfs(
    poi_map: &HashMap<Point, Vec<(Point, usize)>>,
    position: Point,
    goal: Point,
    visited: &mut HashSet<Point>,
    dist: usize,
    best: usize,
) -> usize {
    if position == goal {
        return cmp::max(dist, best);
    }
    let mut best = best;

    poi_map
        .get(&position)
        .expect("Should never hit point not in map")
        .iter()
        .for_each(|(p, pdist)| {
            if !visited.contains(p) {
                visited.insert(*p);
                let maybe_best = poi_dfs(poi_map, *p, goal, visited, dist + pdist, best);
                best = cmp::max(best, maybe_best);
                visited.remove(p);
            }
        });

    best
}

pub fn part_one(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map[0].iter().position(|c| *c != '#').unwrap();
    let start = (0, start);
    let goal = map[map.len() - 1].iter().position(|c| *c != '#').unwrap();
    let goal = (map.len() - 1, goal);

    let best = greedy_dfs(&map, start, goal, false);
    Some(best as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let start = map[0].iter().position(|c| *c != '#').unwrap();
    let start = (0, start);
    let goal = map[map.len() - 1].iter().position(|c| *c != '#').unwrap();
    let goal = (map.len() - 1, goal);

    let poi = points_of_interest(&map, start, goal);
    let poi_map = poi_map(&map, poi);
    let best = poi_dfs(&poi_map, start, goal, &mut HashSet::new(), 0, 0);
    Some(best as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
