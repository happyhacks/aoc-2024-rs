use std::{
    collections::{HashMap, HashSet},
    ops::Add,
    cmp::Ordering::{Greater, Less},
};

advent_of_code::solution!(14);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos {
    x: isize,
    y: isize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    pos: Pos,
    speed: Pos,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Pos {
    fn wrapping_add(self, bounds: Pos) -> Pos {
        Pos {
            x: ((self.x % bounds.x) + bounds.x) % bounds.x,
            y: ((self.y % bounds.y) + bounds.y) % bounds.y,
        }
    }
}

fn parse_pv(pv: &str) -> Option<Pos> {
    pv.split_once("=").and_then(|(_, v)| {
        v.split_once(",").map(|(x, y)| {
            Pos {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
    })
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(" ").and_then(|(p, v)| {
                if let Some(p) = parse_pv(p) {
                    if let Some(v) = parse_pv(v) {
                        return Some(Robot { pos: p, speed: v });
                    }
                }
                None
            })
        })
        .collect()
}

fn quadrant<const HEIGHT: isize, const WIDTH: isize>(pos: Pos) -> Option<Pos> {
    match (pos.x.cmp(&(WIDTH / 2)), pos.y.cmp(&(HEIGHT / 2))) {
        (Less, Less) => Some(Pos { x: 0, y: 0 }),
        (Less, Greater) => Some(Pos { x: 0, y: 1 }),
        (Greater, Less) => Some(Pos { x: 1, y: 0 }),
        (Greater, Greater) => Some(Pos { x: 1, y: 1 }),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let robots = parse_input(input);
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    let mut quads = HashMap::new();
    for mut robot in robots {
        for _ in 0..100 {
            robot.pos = (robot.pos + robot.speed).wrapping_add(Pos {
                x: WIDTH,
                y: HEIGHT,
            });
        }
        if let Some(pos) = quadrant::<HEIGHT, WIDTH>(robot.pos) {
            quads.entry(pos).or_insert(Vec::new()).push(robot);
        }
    }
    Some(quads.values().map(|v| v.len() as u32).product::<u32>())
}

fn dump_robots(uniq: &HashSet<Pos>) {
    if !cfg!(debug_assertions) {
        return;
    }
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if uniq.contains(&Pos { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn max_scc_size<const HEIGHT: isize, const WIDTH: isize>(uniq: &HashSet<Pos>) -> u32 {
    let mut max = 0;
    let mut visited = HashSet::new();
    for p in uniq {
        if !visited.contains(p) {
            let mut count = 0;
            let mut queue = Vec::new();
            queue.push(*p);
            while let Some(p) = queue.pop() {
                if visited.contains(&p) {
                    continue;
                }
                visited.insert(p);
                count += 1;
                for d in [
                    Pos { x: 0, y: 1 },
                    Pos { x: 0, y: -1 },
                    Pos { x: 1, y: 0 },
                    Pos { x: -1, y: 0 },
                ] {
                    let n = (p + d).wrapping_add(Pos {
                        x: WIDTH,
                        y: HEIGHT,
                    });
                    if !uniq.contains(&n) {
                        continue;
                    }
                    queue.push(n);
                }
            }
            max = max.max(count);
        }
    }
    max
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots = parse_input(input);
    const WIDTH: isize = 101;
    const HEIGHT: isize = 103;
    for i in 1.. {
        robots.iter_mut().for_each(|robot| {
            robot.pos = (robot.pos + robot.speed).wrapping_add(Pos {
                x: WIDTH,
                y: HEIGHT,
            });
        });
        let uniq = robots.iter().map(|r| r.pos).collect::<HashSet<_>>();
        // dump_robots(&uniq);
        // println!("{}: {}", i, max_scc_size::<HEIGHT, WIDTH>(&uniq));
        if uniq.len() == robots.len() {
            dump_robots(&uniq);  
            return Some(i as u32);
        }
        // if max_scc_size::<HEIGHT, WIDTH>(&uniq) > 100 {
        //     dump_robots(&uniq);
        //     return Some(i as u32);
        // }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
