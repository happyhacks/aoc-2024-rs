use std::collections::HashSet;

advent_of_code::solution!(10);

#[derive(Clone, Copy, Debug)]
enum Dir {
    North,
    East,
    West,
    South,
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

    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        for d in [Dir::North, Dir::East, Dir::South, Dir::West] {
            if let Some(n) = self.get(p, d) {
                result.push(n);
            }
        }
        result
    }
}

// implment iterator for Grid
struct GridIter<'a> {
    grid: &'a Grid,
    x: usize,
    y: usize,
}

impl<'a> Iterator for GridIter<'a> {
    type Item = (Pos, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= self.grid.yln() {
            return None; // End of iteration
        }

        let pos = Pos(self.x as i32, self.y as i32);
        let value = self.grid.0[self.y][self.x];

        self.x += 1;
        if self.x >= self.grid.xln() {
            self.x = 0;
            self.y += 1;
        }

        Some((pos, value))
    }
}

impl Grid {
    fn iter(&self) -> GridIter {
        GridIter {
            grid: self,
            x: 0,
            y: 0,
        }
    }
}

// count visited 9
fn count_visited9(grid: &Grid, p: Pos) -> u32 {
    let mut visited = HashSet::new();
    let mut queue = Vec::new();
    queue.push(p);
    while let Some(p) = queue.pop() {
        let curr = grid.at(p);
        if grid.at(p) == 9 {
            visited.insert(p);
        } else {
            for n in grid.neighbors(p) {
                if grid.at(n) == curr + 1 {
                    queue.push(n);
                }
            }
        }
    }
    visited.len() as u32
}

// count visited paths
fn count_visited_paths(grid: &Grid, p: Pos) -> u32 {
    let mut visited = 0;
    let mut queue = Vec::new();
    queue.push(p);
    while let Some(p) = queue.pop() {
        let curr = grid.at(p);
        if grid.at(p) == 9 {
            visited += 1;
        } else {
            for n in grid.neighbors(p) {
                if grid.at(n) == curr + 1 {
                    queue.push(n);
                }
            }
        }
    }
    visited
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|x| x - b'0').collect())
        .collect();
    let g = Grid(grid);
    let trailheads = g
        .iter()
        .filter_map(|(p, v)| if v == 0 { Some(p) } else { None })
        .collect::<Vec<Pos>>();
    // score is the count of reachable 9
    let mut sum = 0;
    for p in trailheads {
        sum += count_visited9(&g, p);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|x| x - b'0').collect())
        .collect();
    let g = Grid(grid);
    let trailheads = g
        .iter()
        .filter_map(|(p, v)| if v == 0 { Some(p) } else { None })
        .collect::<Vec<Pos>>();
    // score is the count of paths to 9
    let mut sum = 0;
    for p in trailheads {
        sum += count_visited_paths(&g, p);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
