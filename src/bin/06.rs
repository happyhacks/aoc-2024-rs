advent_of_code::solution!(6);

use std::collections::HashSet;
#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    West,
    South,
}
impl Dir {
    fn right(&self) -> Dir {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);

impl Pos {
    fn north(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y - 1)
    }
    fn south(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x, y + 1)
    }
    fn east(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x + 1, y)
    }
    fn west(&self) -> Pos {
        let &Pos(x, y) = self;
        Pos(x - 1, y)
    }
}

struct Grid(Vec<Vec<u8>>);
impl Grid {
    fn yln(&self) -> usize {
        self.0.len()
    }
    fn xln(&self) -> usize {
        if self.yln() > 0 {
            return self.0[0].len();
        }
        0
    }
    fn is_valid(&self, p: Pos) -> bool {
        p.0 >= 0 && p.1 >= 0 && p.0 < self.xln() as i32 && p.1 < self.yln() as i32
    }
    fn at(&self, p: Pos) -> u8 {
        self.0[p.1 as usize][p.0 as usize]
    }
    fn block(&mut self, p: Pos) {
        self.0[p.1 as usize][p.0 as usize] = b'#';
    }
    fn unblock(&mut self, p: Pos) {
        self.0[p.1 as usize][p.0 as usize] = b'.';
    }
    fn occupied(&self, p: Pos) -> bool {
        self.at(p) == b'#'
    }

    fn start(&self) -> Option<Pos> {
        for y in 0..self.yln() {
            for x in 0..self.xln() {
                if self.at(Pos(x as i32, y as i32)) == b'^' {
                    return Some(Pos(x as i32, y as i32));
                }
            }
        }
        None
    }

    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
        let next = match d {
            Dir::East => p.east(),
            Dir::West => p.west(),
            Dir::North => p.north(),
            Dir::South => p.south(),
        };
        if self.is_valid(next) {
            return Some(next);
        }
        None
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let g = Grid(grid);
    let mut visited = HashSet::new();
    let mut direction = Dir::North;
    if let Some(p) = g.start() {
        let mut pos = p;
        while g.is_valid(pos) {
            if let Some(next) = g.get(pos, direction) {
                if g.occupied(next) {
                    direction = direction.right();
                } else {
                    pos = next;
                    visited.insert(pos);
                }
            } else {
                break;
            }
        }
        return  Some(visited.len() as u32);
    }
    None
}

fn count_visited(g: &Grid, p: Pos, d: Dir) -> i32 {
    let mut visited = 0;
    let mut p = p;
    let mut direction = d;
    while g.is_valid(p) {
        if visited > 10000 {
            break;
        }
        if let Some(next) = g.get(p, direction) {
            if g.occupied(next) {
                direction = direction.right();
            } else {
                p = next;
                visited += 1;
            }
        } else {
            break;
        }
    }
    visited
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().to_vec())
        .collect();
    let mut g = Grid(grid);
    let mut visited = HashSet::new();
    let mut direction = Dir::North;
    if let Some(p) = g.start() {
        {
            let mut pos = p;
            while g.is_valid(pos) {
                if let Some(next) = g.get(pos, direction) {
                    if g.occupied(next) {
                        direction = direction.right();
                    } else {
                        pos = next;
                        visited.insert(pos);
                    }
                } else {
                    break;
                }
            }
        }
        {
            let mut sum = 0;
            for &pos in visited.iter() {
                g.block(pos);
                if count_visited(&g, p, Dir::North) > 10000 {
                    sum += 1;
                }
                g.unblock(pos);
            }
            return Some(sum);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
