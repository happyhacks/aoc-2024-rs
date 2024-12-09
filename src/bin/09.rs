use std::{collections::BTreeSet, iter};

advent_of_code::solution!(9);

#[derive(Debug, Clone, Copy)]
struct File {
    id: u64,
}

#[derive(Debug, Clone, Copy)]
enum Node {
    File(File),
    Space,
}

fn parse_input_for_part_one(input: &str) -> Vec<Node> {
    // input should be a single line - of digits
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .flat_map(|(i, c)| {
            let count = c.to_digit(10).unwrap() as usize;
            iter::repeat(match i % 2 {
                0 => Node::File(File { id: i as u64 / 2 }),
                1 => Node::Space,
                _ => unreachable!(),
            })
            .take(count)
        })
        .collect::<Vec<_>>()
}

fn checksum_part_one(nodes: &[Node]) -> u64 {
    nodes
        .iter()
        .enumerate()
        .filter_map(|(i, n)| match n {
            Node::File(f) => Some(f.id * (i as u64)),
            Node::Space => None,
        })
        .sum()
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nodes = parse_input_for_part_one(input);

    let mut l = 0;
    let mut r = nodes.len() - 1;

    while l < r {
        while l < nodes.len() && matches!(nodes[l], Node::File(_)) {
            l += 1;
        }

        while r > 0 && matches!(nodes[r], Node::Space) {
            r -= 1;
        }

        if l < r {
            nodes.swap(l, r);
            l += 1;
            r -= 1;
        }
    }

    Some(checksum_part_one(&nodes))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct FileBlock {
    id: u64,
    idx: u64,
    size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SpaceBlock {
    // id: u64,
    idx: u64,
    size: u64,
}

impl Ord for SpaceBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl PartialOrd for SpaceBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
enum Chunk {
    File(FileBlock),
    Space(SpaceBlock),
}

fn parse_input_for_part_two(input: &str) -> Vec<Chunk> {
    let mut idx = 0;
    // input should be a single line - of digits
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .enumerate()
        .map(|(i, c)| {
            let size = c.to_digit(10).unwrap() as u64;
            idx += size;
            match i % 2 {
                0 => Chunk::File(FileBlock {
                    id: i as u64 / 2,
                    size,
                    idx: idx - size,
                }),
                1 => Chunk::Space(SpaceBlock {
                    size,
                    // id: i as u64 / 2,
                    idx: idx - size,
                }),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn part_two(input: &str) -> Option<u64> {
    let chunks = parse_input_for_part_two(input);
    // dbg!(&chunks);
    let mut spaces = chunks
        .iter()
        .filter_map(|c| match c {
            Chunk::File(_) => None,
            Chunk::Space(s) => Some(*s),
        })
        .collect::<BTreeSet<SpaceBlock>>();
    let files = chunks
        .iter()
        .filter_map(|c| match c {
            Chunk::File(f) => Some(*f),
            Chunk::Space(_) => None,
        })
        .rev()
        .collect::<Vec<FileBlock>>();
    let mut compressed = Vec::new();

    for file in files.iter() {
        let space = spaces
            .iter()
            .take_while(|&space| space.idx < file.idx)
            .find(|&space| space.size >= file.size);
        if let Some(&space) = space {
            compressed.push(FileBlock {
                id: file.id,
                idx: space.idx,
                size: file.size,
            });
            spaces.remove(&space);
            if space.size > file.size {
                spaces.insert(
                    SpaceBlock {
                        // id: space.id,
                        idx: space.idx + file.size,
                        size: space.size - file.size,
                    },
                );
            }
        } else {
            compressed.push(*file);
        }
    }

    Some(
        compressed
            .iter()
            .map(|f| (f.idx..(f.idx + f.size)).map(|i| i * f.id).sum::<u64>())
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
