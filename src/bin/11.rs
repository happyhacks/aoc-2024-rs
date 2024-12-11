use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(11);

fn blink(n: u64) -> Vec<u64> {
    if n == 0 {
        return vec![1];
    }
    let s = n.to_string();
    if s.len() % 2 == 0 {
        vec![
            s[0..s.len() / 2].parse::<u64>().unwrap(),
            s[s.len() / 2..].parse::<u64>().unwrap(),
        ]
    } else {
        vec![n * 2024]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut nums = input
        .trim()
        .split(' ')
        .map(|f| f.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    for _ in 0..25 {
        let new_nums = nums.iter().flat_map(|n| blink(*n)).collect::<Vec<u64>>();
        nums = new_nums;
    }
    if cfg!(debug_assertions) {
        println!("{}", nums.iter().unique().count());
    }
    Some(nums.len() as u64)
}

fn blink_stones(count: &HashMap<u64, usize>) -> HashMap<u64, usize> {
    let mut new_count = HashMap::new();
    for (n, c) in count {
        for b in blink(*n) {
            *new_count.entry(b).or_insert(0) += c;
        }
    }
    new_count
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut counter = input
        .trim()
        .split(' ')
        .map(|f| f.parse::<u64>().unwrap())
        .counts();

    for _ in 0..75 {
        counter = blink_stones(&counter);
    }
    Some(counter.values().sum::<usize>() as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
