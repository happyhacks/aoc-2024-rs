advent_of_code::solution!(13);

#[derive(Debug, Clone, Copy)]
struct Pos {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

fn get_pos(input: &str, sep: &str) -> Option<Pos> {
    input.split_once(": ")
        .and_then(|(_, input)| input.split_once(", "))
        .and_then(|(x, y)| {
            let x = x.split(sep).nth(1)?.parse().ok()?;
            let y = y.split(sep).nth(1)?.parse().ok()?;
            Some(Pos { x, y })
        })
}

fn parse_input(input: &str) -> Option<Vec<Machine>> {
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let a = get_pos(lines.next()?, "+")?;
            let b = get_pos(lines.next()?, "+")?;
            let prize = get_pos(lines.next()?, "=")?;
            Some(Machine { a, b, prize })
        })
        .collect()
}

fn solve_buttons(m: &Machine) -> Option<(i64, i64)> {
    // Solve the system of equations:
    // a.x * x + b.x * y = prize.x
    // a.y * x + b.y * y = prize.y
    let a = m.a;
    let b = m.b;
    let prize = m.prize;
    let det = a.x * b.y - a.y * b.x;
    if det == 0 {
        return None; // No unique solution
    }

    let x_num = prize.x * b.y - prize.y * b.x;
    let y_num = a.x * prize.y - a.y * prize.x;

    if x_num % det != 0 || y_num % det != 0 {
        return None; // No integer solution
    }

    let x = x_num / det;
    let y = y_num / det;
    Some((x, y))
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_input(input).unwrap();
    let mut sum = 0;
    for machine in machines {
        if let Some((x, y)) = solve_buttons(&machine) {
            sum += x * 3 + y;
            assert!(x + y <= 200);
            // lol
            // You estimate that each button would need to be pressed *no more 
            //than `100` times* to win a prize. How else would someone be 
            // expected to play?
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse_input(input).unwrap();
    let mut sum = 0;
    for mut machine in machines {
        machine.prize.x += 10000000000000;
        machine.prize.y += 10000000000000;
        if let Some((x, y)) = solve_buttons(&machine) {
            sum += x * 3 + y;
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
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(875318608908));
    }
}
