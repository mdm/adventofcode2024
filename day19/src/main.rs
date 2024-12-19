use std::{collections::HashMap, env::args, fs::read_to_string, path::Path};

fn parse<P>(filename: P) -> (Vec<String>, Vec<String>)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let (patterns, designs) = raw_input.split_once("\n\n").unwrap();
    let patterns = patterns.split(", ").map(str::to_string).collect();
    let designs = designs.lines().map(str::to_string).collect();

    (patterns, designs)
}

fn dfs(patterns: &[String], design: &str, memo: &mut HashMap<String, usize>) -> Option<usize> {
    if design.is_empty() {
        return Some(1);
    }

    if memo.contains_key(design) {
        return Some(memo[design]);
    }

    let mut total_count = 0;
    for pattern in patterns {
        if design.starts_with(pattern) {
            let remaining = &design[pattern.len()..];
            if let Some(count) = dfs(patterns, remaining, memo) {
                total_count += count;
            }
        }
    }

    if total_count > 0 {
        memo.insert(design.to_string(), total_count);
        Some(total_count)
    } else {
        None
    }
}

fn solve_part1(patterns: &[String], designs: &[String]) -> usize {
    let mut memo = HashMap::new();

    designs
        .iter()
        .filter_map(|design| dfs(patterns, design, &mut memo))
        .count()
}

fn solve_part2(patterns: &[String], designs: &[String]) -> usize {
    let mut memo = HashMap::new();

    designs
        .iter()
        .filter_map(|design| dfs(patterns, design, &mut memo))
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (patterns, designs) = parse(filename);
    let answer_part1 = solve_part1(&patterns, &designs);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&patterns, &designs);
    println!("{}", answer_part2);
}
