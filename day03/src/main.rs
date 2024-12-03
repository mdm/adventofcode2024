use std::{env::args, fs::read_to_string, path::Path};

use regex::Regex;

fn parse<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input.lines().map(|line| line.to_string()).collect()
}

fn execute(parsed_input: &[String], track_enabled: bool) -> i32 {
    let operations = Regex::new(r"(mul\(\d+,\d+\)|don't\(\)|do\(\))").expect("Invalid regex");
    let operands = Regex::new(r"mul\((\d+),(\d+)\)").expect("Invalid regex");

    let mut enabled = true;
    parsed_input
        .iter()
        .map(|line| {
            operations
                .captures_iter(line)
                .map(|matched| {
                    matched
                        .iter()
                        .skip(1)
                        .map(|operation| match operation.unwrap().as_str() {
                            "don't()" => {
                                if track_enabled {
                                    enabled = false;
                                }
                                0
                            }
                            "do()" => {
                                enabled = true;
                                0
                            }
                            mul => {
                                if enabled {
                                    let captures = operands.captures(mul).unwrap();
                                    let a =
                                        captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
                                    let b =
                                        captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
                                    a * b
                                } else {
                                    0
                                }
                            }
                        })
                        .sum::<i32>()
                })
                .sum::<i32>()
        })
        .sum()
}

fn solve_part1(parsed_input: &[String]) -> i32 {
    execute(parsed_input, false)
}

fn solve_part2(parsed_input: &[String]) -> i32 {
    execute(parsed_input, true)
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let parsed_input = parse(filename);
    let answer_part1 = solve_part1(&parsed_input);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&parsed_input);
    println!("{}", answer_part2);
}
