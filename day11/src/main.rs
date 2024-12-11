use std::{collections::HashMap, env::args, fs::read_to_string, path::Path, vec};

fn parse<P>(filename: P) -> Vec<u64>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .next()
        .expect("Failed to parse input")
        .split_ascii_whitespace()
        .map(|stone| stone.parse().expect("Failed to parse stone"))
        .collect()
}

fn blink_once(stone: u64) -> Vec<u64> {
    if stone == 0 {
        return vec![1];
    }

    let digits = stone.ilog10() + 1;
    if digits % 2 == 0 {
        let mask = 10_u64.pow(digits / 2);
        let left = stone / mask;
        let right = stone % mask;
        return vec![left, right];
    }

    vec![stone * 2024]
}

fn solve_part1(stones: &[u64]) -> usize {
    let mut current = stones.to_vec();
    for _ in 0..25 {
        current = current
            .iter()
            .flat_map(|stone| blink_once(*stone))
            .collect::<Vec<_>>();
    }

    current.len()
}

fn blink_rec(
    stone: u64,
    iterations_remaining: u64,
    memo: &mut HashMap<(u64, u64), usize>,
) -> usize {
    if iterations_remaining == 0 {
        return 1;
    }

    if memo.contains_key(&(stone, iterations_remaining)) {
        return memo[&(stone, iterations_remaining)];
    }

    let count = blink_once(stone)
        .iter()
        .map(|new_stone| blink_rec(*new_stone, iterations_remaining - 1, memo))
        .sum();

    memo.insert((stone, iterations_remaining), count);

    count
}

fn solve_part2(stones: &[u64]) -> usize {
    let mut memo = HashMap::new();
    stones
        .iter()
        .map(|stone| blink_rec(*stone, 75, &mut memo))
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let stones = parse(filename);
    let answer_part1 = solve_part1(&stones);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&stones);
    println!("{}", answer_part2);
}
