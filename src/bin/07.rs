use std::error::Error;
advent_of_code::solution!(7);

// 3267: 81 40 27
struct Equation {
    result: u128,
    inputs: Vec<u128>,
}

fn concat(a: u128, b: u128) -> u128 {
    a * 10u128.pow(b.ilog10() + 1) + b
}

fn evaluate_left_to_right(numbers: &Vec<u128>, operators: &Vec<char>) -> u128 {
    let mut result = numbers[0];
    for (i, &op) in operators.iter().enumerate() {
        match op {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            '|' => result = concat(result, numbers[i + 1]),
            _ => panic!("Unexpected operator"),
        }
    }
    result
}

fn parse_input(input: &str) -> Result<Vec<Equation>, Box<dyn Error>> {
    let mut equations = Vec::new();
    for line in input.lines() {
        let line = line.trim();
        if let Some((result, inputs)) = line.split_once(": ") {
            let inputs: Vec<u128> = inputs
                .split(" ")
                .map(|x| x.parse().unwrap())
                .collect();
            equations.push(Equation { result: result.parse()?, inputs });
            continue;
        }
    }

    Ok(equations)
}


pub fn part_one(input: &str) -> Option<u128> {
    let equations = parse_input(input).unwrap();
    let mut result = 0;

    for equation in equations {
        let num_ops = equation.inputs.len() - 1;
        let combinations = (0..2u128.pow(num_ops as u32))
            .map(|i| {
                (0..num_ops)
                    .map(|j| if (i & (1 << j)) != 0 { '+' } else { '*' })
                    .collect::<Vec<char>>()
            });
        for combination in combinations {
            if evaluate_left_to_right(&equation.inputs, &combination) == equation.result {
                result += equation.result;
                break;
            }
        }
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u128> {
    let equations = parse_input(input).unwrap();
    let mut result = 0;

    for equation in equations {
        let num_ops = equation.inputs.len() - 1;
        let combinations = (0..3u128.pow(num_ops as u32))
            .map(|i| {
                (0..num_ops)
                    .map(|j| {
                        match (i / 3u128.pow(j as u32)) % 3 {
                            0 => '+',
                            1 => '*',
                            _ => '|',
                        }
                    })
                    .collect::<Vec<char>>()
            });
        for combination in combinations {
            if evaluate_left_to_right(&equation.inputs, &combination) == equation.result {
                result += equation.result;
                break;
            }
        }
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
