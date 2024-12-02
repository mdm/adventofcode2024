use std::{env::args, fs::read_to_string, path::Path};

fn parse<P>(filename: P) -> Vec<Vec<i32>>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|level| level.parse().expect("Failed to parse level"))
                .collect()
        })
        .collect()
}

fn safe_report(report: &[i32]) -> bool {
    let all_increasing = report.windows(2).all(|w| w[0] <= w[1]);
    let all_decreasing = report.windows(2).all(|w| w[0] >= w[1]);
    let distance_valid = report.windows(2).all(|w| {
        let distance = (w[1] - w[0]).abs();
        (1..=3).contains(&distance)
    });

    (all_increasing || all_decreasing) && distance_valid
}

fn solve_part1(parsed_input: &[Vec<i32>]) -> usize {
    parsed_input
        .iter()
        .filter(|report| safe_report(report))
        .count()
}

fn solve_part2(parsed_input: &[Vec<i32>]) -> usize {
    parsed_input
        .iter()
        .filter(|report| {
            for i in 0..report.len() {
                let mut modified_report = (*report).clone();
                modified_report.remove(i);

                if safe_report(&modified_report) {
                    return true;
                }
            }

            false
        })
        .count()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let parsed_input = parse(filename);
    let answer_part1 = solve_part1(&parsed_input);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&parsed_input);
    println!("{}", answer_part2);
}
