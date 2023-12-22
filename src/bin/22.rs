use std::{cmp::Ordering, collections::HashMap};

advent_of_code::solution!(22);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn from(v: Vec<usize>) -> Self {
        Self {
            x: v[0],
            y: v[1],
            z: v[2],
        }
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Block {
    key: usize,
    start: Point,
    end: Point,
}

impl Block {
    fn from(s: &str, key: usize) -> Self {
        let mut splits = s.split('~');
        let start: Vec<_> = splits
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        let end: Vec<_> = splits
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse::<usize>().unwrap())
            .collect();
        Self {
            key,
            start: Point::from(start),
            end: Point::from(end),
        }
    }

    fn lowest_z(&self) -> usize {
        if self.start.z < self.end.z {
            self.start.z
        } else {
            self.end.z
        }
    }

    fn highest_z(&self) -> usize {
        if self.start.z > self.end.z {
            self.start.z
        } else {
            self.end.z
        }
    }

    fn xy_intersect(&self, other: &Self) -> bool {
        let x_overlap = self.start.x >= other.start.x && self.start.x <= other.end.x
            || self.start.x <= other.start.x && self.start.x >= other.end.x
            || self.end.x >= other.start.x && self.end.x <= other.end.x
            || self.end.x <= other.start.x && self.end.x >= other.end.x
            || other.start.x >= self.start.x && other.start.x <= self.end.x
            || other.start.x <= self.start.x && other.start.x >= self.end.x
            || other.end.x >= self.start.x && other.end.x <= self.end.x
            || other.end.x <= self.start.x && other.end.x >= self.end.x;
        let y_overlap = self.start.y >= other.start.y && self.start.y <= other.end.y
            || self.start.y <= other.start.y && self.start.y >= other.end.y
            || self.end.y >= other.start.y && self.end.y <= other.end.y
            || self.end.y <= other.start.y && self.end.y >= other.end.y
            || other.start.y >= self.start.y && other.start.y <= self.end.y
            || other.start.y <= self.start.y && other.start.y >= self.end.y
            || other.end.y >= self.start.y && other.end.y <= self.end.y
            || other.end.y <= self.start.y && other.end.y >= self.end.y;
        x_overlap && y_overlap
    }

    fn supports_at(&self, other: &Self) -> Option<usize> {
        if self.highest_z() >= other.lowest_z() {
            return None;
        }
        if self.xy_intersect(other) {
            Some(self.highest_z() + 1)
        } else {
            None
        }
    }

    fn lower_to(&mut self, new_z: usize) {
        let diff = self.lowest_z() - new_z;
        self.start.z = self.start.z - diff;
        self.end.z = self.end.z - diff;
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.lowest_z().cmp(&other.lowest_z()) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => self.highest_z().cmp(&other.highest_z()),
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.lowest_z() == other.lowest_z()
    }
}

fn drop_floating_blocks(blocks: Vec<Block>) -> (Vec<Block>, Vec<Vec<Block>>) {
    let mut blocks = blocks;
    let highest_possible = blocks[blocks.len() - 1].highest_z();
    let mut tops: Vec<Vec<Block>> = vec![Vec::new(); highest_possible];
    let mut max = highest_possible;
    blocks.iter_mut().for_each(|b| {
        if b.lowest_z() == 1 {
            tops[b.highest_z()].push(b.clone());
        } else {
            let mut indices = (1..b.lowest_z()).rev();
            let new_z = loop {
                if let Some(i) = indices.next() {
                    if let Some(z) = tops[i].iter().find_map(|b2| b2.supports_at(b)) {
                        break z;
                    }
                } else {
                    break 1;
                }
            };
            b.lower_to(new_z);
            tops[b.highest_z()].push(b.clone());
            max = b.highest_z();
        }
    });
    (blocks, tops[0..=max].to_vec())
}

#[derive(Debug)]
struct BlockNode {
    block: Block,
    children: Vec<usize>,
    parents: Vec<usize>,
}

fn build_tree(supports: Vec<Vec<Block>>) -> (HashMap<usize, BlockNode>) {
    let mut map: HashMap<usize, BlockNode> = HashMap::new();
    supports.iter().flatten().for_each(|block| {
        let bottom = block.lowest_z();
        let mut new_block = BlockNode {
            block: *block,
            children: Vec::new(),
            parents: Vec::new(),
        };
        if bottom != 1 {
            let v = &supports[bottom - 1];
            v.iter().for_each(|supp_block| {
                if supp_block.supports_at(&new_block.block).is_some() {
                    let supp_block = map
                        .get_mut(&supp_block.key)
                        .expect("Shouldn't hit any blocks not yet in map");
                    new_block.parents.push(supp_block.block.key);
                    supp_block.children.push(new_block.block.key);
                }
            })
        }
        map.insert(new_block.block.key, new_block);
    });
    map
}

pub fn part_one(input: &str) -> Option<u64> {
    // step 1:  Bring blocks down from floating
    // step 2:  Build dependency tree for blocks
    // step 3:  From bottom up, disintegrate any block which is not necessary to support other
    //   blocks
    let mut blocks: Vec<_> = input
        .lines()
        .enumerate()
        .map(|(i, s)| Block::from(s, i))
        .collect();
    blocks.sort();
    let (_, supports) = drop_floating_blocks(blocks);
    let tree = build_tree(supports);
    let count = tree
        .values()
        .filter(|node| {
            if node.children.len() == 0 {
                true
            } else {
                if node.children.iter().all(|child| {
                    let child_node = tree.get(child).unwrap();
                    child_node.parents.len() > 1
                }) {
                    true
                } else {
                    false
                }
            }
        })
        .count();
    Some(count as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
