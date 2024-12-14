use std::{env::args, fs::read_to_string, path::Path, str::FromStr};

use regex::Regex;

#[derive(Debug, Clone)]
struct Button {
    delta_x: i64,
    delta_y: i64,
}

impl FromStr for Button {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Button (?:A|B): X\+(\d+), Y\+(\d+)").unwrap();
        let captures = re.captures(s).ok_or(())?;
        let delta_x = captures[1].parse().unwrap();
        let delta_y = captures[2].parse().unwrap();

        Ok(Self { delta_x, delta_y })
    }
}

#[derive(Debug, Clone)]
struct Prize {
    x: i64,
    y: i64,
}

impl FromStr for Prize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
        let captures = re.captures(s).ok_or(())?;
        let x = captures[1].parse().unwrap();
        let y = captures[2].parse().unwrap();

        Ok(Self { x, y })
    }
}

#[derive(Debug, Clone)]
struct Machine {
    button_a: Button,
    button_b: Button,
    prize: Prize,
}

fn parse<P>(filename: P) -> Vec<Machine>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut machines = Vec::new();
    let mut lines = raw_input.lines();
    while let Some(line) = lines.next() {
        let button_a = line.parse().unwrap();
        let button_b = lines.next().unwrap().parse().unwrap();
        let prize = lines.next().unwrap().parse().unwrap();

        machines.push(Machine {
            button_a,
            button_b,
            prize,
        });

        lines.next();
    }

    machines
}

fn min_tokens_brute_force(machine: &Machine) -> Option<usize> {
    let mut min_tokens = 1_000;
    for pressed_a in 0..=100 {
        let remaining_x = machine.prize.x - machine.button_a.delta_x * pressed_a;
        let remaining_y = machine.prize.y - machine.button_a.delta_y * pressed_a;

        if remaining_x < 0 || remaining_y < 0 {
            continue;
        }

        if remaining_x % machine.button_b.delta_x == 0
            && remaining_y % machine.button_b.delta_y == 0
            && remaining_x / machine.button_b.delta_x == remaining_y / machine.button_b.delta_y
        {
            let pressed_b = remaining_x / machine.button_b.delta_x;
            if 3 * pressed_a + pressed_b < min_tokens {
                min_tokens = 3 * pressed_a + pressed_b;
            }
        }
    }

    if min_tokens == 1_000 {
        None
    } else {
        Some(min_tokens as usize)
    }
}

fn min_tokens_smart(machine: &Machine) -> Option<usize> {
    let lhs = machine.button_a.delta_x * machine.button_b.delta_y
        - machine.button_b.delta_x * machine.button_a.delta_y;
    let rhs =
        machine.prize.y * machine.button_a.delta_x - machine.prize.x * machine.button_a.delta_y;

    // if rhs.signum() != lhs.signum() {
    //     return None;
    // }

    if lhs == 0 {
        return None;
    }

    if rhs % lhs != 0 {
        return None;
    }

    let pressed_b = rhs / lhs;
    let remaining_x = machine.prize.x - machine.button_b.delta_x * pressed_b;

    if remaining_x % machine.button_a.delta_x != 0 {
        return None;
    }

    let pressed_a = remaining_x / machine.button_a.delta_x;

    Some(3 * pressed_a as usize + pressed_b as usize)
}

fn solve_part1(machines: &[Machine]) -> usize {
    machines.iter().filter_map(min_tokens_brute_force).sum()
}

fn solve_part2(machines: &[Machine]) -> usize {
    machines
        .iter()
        .cloned()
        .filter_map(|mut machine| {
            machine.prize.x += 10_000_000_000_000;
            machine.prize.y += 10_000_000_000_000;
            min_tokens_smart(&machine)
        })
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let machines = parse(filename);
    let answer_part1 = solve_part1(&machines);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&machines);
    println!("{}", answer_part2);
}
