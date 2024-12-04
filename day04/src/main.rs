use std::{env::args, fs::read_to_string, path::Path};

fn parse<P>(filename: P) -> Vec<Vec<char>>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn check_location(
    parsed_input: &[Vec<char>],
    needle: &str,
    x: i32,
    y: i32,
    step_x: i32,
    step_y: i32,
    offset: i32,
) -> bool {
    if needle.chars().enumerate().all(|(i, c)| {
        let x = x + (i as i32 + offset) * step_x;
        let y = y + (i as i32 + offset) * step_y;
        if x < 0 || y < 0 || x >= parsed_input[0].len() as i32 || y >= parsed_input.len() as i32 {
            return false;
        }
        parsed_input[y as usize][x as usize] == c
    }) {
        return true;
    }

    false
}

fn solve_part1(parsed_input: &[Vec<char>]) -> usize {
    let mut count = 0;

    for y in 0..parsed_input.len() {
        for x in 0..parsed_input[0].len() {
            for step_y in -1..=1 {
                for step_x in -1..=1 {
                    if check_location(parsed_input, "XMAS", x as i32, y as i32, step_x, step_y, 0) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn solve_part2(parsed_input: &[Vec<char>]) -> usize {
    let mut count = 0;

    for y in 0..parsed_input.len() {
        for x in 0..parsed_input[0].len() {
            let mut matches = 0;

            for step_y in -1..=1 {
                for step_x in -1..=1 {
                    if step_x == 0 || step_y == 0 {
                        continue;
                    }

                    if check_location(parsed_input, "MAS", x as i32, y as i32, step_x, step_y, -1) {
                        matches += 1;
                    }
                }
            }

            if matches == 2 {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let parsed_input = parse(filename);
    let answer_part1 = solve_part1(&parsed_input);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&parsed_input);
    println!("{}", answer_part2);
}
