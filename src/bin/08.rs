use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(i32, i32);
impl Pos {
    fn extend(&self, p: &Pos) -> Pos {
        self.add(&self.sub(p))
    }
    fn add(&self, p: &Pos) -> Pos {
        let &Pos(x1, y1) = self;
        let &Pos(x2, y2) = p;
        Pos(x1 + x2, y1 + y2)
    }
    fn sub(&self, p: &Pos) -> Pos {
        let &Pos(x1, y1) = self;
        let &Pos(x2, y2) = p;
        Pos(x1 - x2, y1 - y2)
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
    fn is_occupied(&self, p: Pos) -> bool {
        self.0[p.1 as usize][p.0 as usize] != b'.'
    }
}

struct City {
    grid: Grid,                     // grid
    antenna: HashMap<u8, Vec<Pos>>, // frequency -> list of positions
}
fn parse_input(input: &str) -> Option<City> {
    let grid: Vec<Vec<u8>> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    let g = Grid(grid);
    let mut antenna = HashMap::new();
    for y in 0..g.yln() {
        for x in 0..g.xln() {
            let p = Pos(x as i32, y as i32);
            if g.is_occupied(p) {
                antenna.entry(g.at(p)).or_insert(Vec::new()).push(p);
            }
        }
    }
    Some(City { grid: g, antenna })
}

pub fn part_one(input: &str) -> Option<u32> {
    let city = parse_input(input).unwrap();
    let mut antinodes = HashSet::new();
    for (_, positions) in city.antenna.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                if city.grid.is_valid(p1.extend(&p2)) {
                    antinodes.insert(p1.extend(&p2));
                }
                if city.grid.is_valid(p2.extend(&p1)) {
                    antinodes.insert(p2.extend(&p1));
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let city = parse_input(input).unwrap();
    let mut antinodes = HashSet::new();
    for (_, positions) in city.antenna.iter() {
        for i in 0..positions.len() {
            for j in i + 1..positions.len() {
                let p1 = positions[i];
                let p2 = positions[j];
                antinodes.insert(p2);
                antinodes.insert(p1);
                let mut e1 = p1.extend(&p2);
                while city.grid.is_valid(e1) {
                    antinodes.insert(e1);
                    e1 = e1.add(&p1.sub(&p2));
                }
                let mut e2 = p2.extend(&p1);
                while city.grid.is_valid(e2) {
                    antinodes.insert(e2);
                    e2 = e2.add(&p2.sub(&p1));
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
