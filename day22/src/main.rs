use std::{collections::HashSet, env::args, fs::read_to_string, path::Path};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn parse<P>(filename: P) -> Vec<i64>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

#[inline(always)]
fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

#[inline(always)]
fn prune(a: i64) -> i64 {
    a % 16777216
}

fn next_secret(secret: i64) -> i64 {
    let mut secret = secret;
    secret = mix(secret * 64, secret);
    secret = prune(secret);
    secret = mix(secret / 32, secret);
    secret = prune(secret);
    secret = mix(secret * 2048, secret);
    secret = prune(secret);

    secret
}

fn solve_part1(secrets: &[i64]) -> usize {
    secrets
        .iter()
        .map(|secret| {
            let mut secret = *secret;
            for _ in 0..2_000 {
                secret = next_secret(secret);
            }

            secret
        })
        .sum::<i64>() as usize
}

fn solve_part2(secrets: &[i64]) -> usize {
    let market = secrets
        .iter()
        .map(|secret| {
            let mut secret = *secret;
            let mut price_changes = Vec::new();
            let mut prices_after = Vec::new();
            for _ in 0..2_000 {
                let new_secret = next_secret(secret);
                price_changes.push(new_secret % 10 - secret % 10);
                prices_after.push(new_secret % 10);
                secret = new_secret;
            }

            (price_changes, prices_after)
        })
        .collect::<Vec<_>>();

    let unique_sequences = market
        .iter()
        .flat_map(|reference| reference.0.windows(4).map(|sequence| sequence.to_vec()))
        .collect::<HashSet<_>>();

    unique_sequences
        .par_iter()
        .map(|sequence| {
            market
                .iter()
                .map(|buyer| {
                    buyer
                        .0
                        .windows(4)
                        .enumerate()
                        .find_map(|(j, window)| {
                            if window == sequence {
                                Some(buyer.1[j + 3])
                            } else {
                                None
                            }
                        })
                        .unwrap_or(0)
                })
                .sum::<i64>()
        })
        .max()
        .unwrap() as usize
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let secrets = parse(filename);
    let answer_part1 = solve_part1(&secrets);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&secrets);
    println!("{}", answer_part2);
}
