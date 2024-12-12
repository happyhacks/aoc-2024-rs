use std::collections::HashSet;

advent_of_code::solution!(12);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Dir {
    North,
    East,
    West,
    South,
}

impl Dir {
    fn perpendicular(&self) -> Vec<Dir> {
        match self {
            Dir::North => vec![Dir::East, Dir::West],
            Dir::East => vec![Dir::North, Dir::South],
            Dir::South => vec![Dir::East, Dir::West],
            Dir::West => vec![Dir::North, Dir::South],
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
    fn get(&self, p: Pos, d: Dir) -> Option<Pos> {
        match d {
            Dir::North => Some(p.north()),
            Dir::East => Some(p.east()),
            Dir::South => Some(p.south()),
            Dir::West => Some(p.west()),
        }
        .filter(|&n| self.is_valid(n))
    }
    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let mut result = Vec::new();
        for d in [Dir::North, Dir::East, Dir::South, Dir::West] {
            if let Some(n) = self.get(p, d) {
                if self.at(n) == self.at(p) {
                    result.push(n);
                }
            }
        }
        result
    }

    fn outer_count(&self, p: Pos) -> u32 {
        let mut count = 0;
        for d in [Dir::North, Dir::East, Dir::South, Dir::West] {
            if let Some(n) = self.get(p, d) {
                if self.at(n) != self.at(p) {
                    count += 1;
                }
            } else {
                count += 1;
            }
        }
        count
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let g = Grid(grid);
    let mut sum = 0;
    let mut visited = HashSet::new();
    for y in 0..g.yln() {
        for x in 0..g.xln() {
            let p = Pos(x as i32, y as i32);
            if visited.contains(&p) {
                continue;
            }
            let mut perimeter = 0;
            let mut queue = Vec::new();
            queue.push(p);
            let mut visitedl = HashSet::new();
            while let Some(p) = queue.pop() {
                if visited.contains(&p) {
                    continue;
                }
                visited.insert(p);
                visitedl.insert(p);
                perimeter += g.outer_count(p);
                for n in g.neighbors(p) {
                    queue.push(n);
                }
            }
            sum += perimeter * visitedl.len() as u32;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let g = Grid(grid);
    let mut sum = 0;
    let mut visited = HashSet::new();
    for y in 0..g.yln() {
        for x in 0..g.xln() {
            let p = Pos(x as i32, y as i32);
            if visited.contains(&p) {
                continue;
            }
            let mut queue = Vec::new();
            queue.push(p);
            let mut scc = HashSet::new();
            while let Some(p) = queue.pop() {
                if visited.contains(&p) {
                    continue;
                }
                visited.insert(p);
                scc.insert(p);
                for n in g.neighbors(p) {
                    queue.push(n);
                }
            }

            let mut sides = 0;
            let mut visitedl = HashSet::new();
            // for each node in scc
            for &p in scc.iter() {
                if g.outer_count(p) == 0 {
                    continue;
                }
                for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
                    // if the neighbor is in not scc in the direction
                    if let Some(n) = g.get(p, dir) {
                        if scc.contains(&n) {
                            continue;
                        }
                    }
                    if visitedl.contains(&(p, dir)) {
                        continue;
                    }
                    sides += 1;
                    let mut queue = Vec::new();
                    queue.push((p, dir));
                    while let Some((p, d)) = queue.pop() {
                        if visitedl.contains(&(p, d)) {
                            continue;
                        }
                        visitedl.insert((p, d));
                        // move perpendicular
                        for pd in d.perpendicular() {
                            if let Some(n) = g.get(p, pd) {
                                if visitedl.contains(&(n, d)) {
                                    continue;
                                }
                                // only nodes that are not in scc
                                if !scc.contains(&n) {
                                    continue;
                                }
                                // only nodes that are on the border
                                if let Some(nd) = g.get(n, d) {
                                    if scc.contains(&nd) {
                                        continue;
                                    }
                                }
                                queue.push((n, d));
                            }
                        }
                    }
                }
            }
            // println!("{} * {} = {}", scc.len(), sides, sides * scc.len() as u32);
            sum += sides * scc.len() as u32;
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
