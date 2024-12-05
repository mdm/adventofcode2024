use std::{env::args, fs::read_to_string, path::Path};

fn parse<P>(filename: P) -> (Vec<(usize, usize)>, Vec<Vec<usize>>)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");
    let (raw_orderings, raw_updates) = raw_input.split_once("\n\n").expect("Invalid input format");

    let orderings = raw_orderings
        .lines()
        .map(|line| line.split_once("|"))
        .map(|pair| {
            let (before, after) = pair.unwrap();
            (before.parse().unwrap(), after.parse().unwrap())
        })
        .collect();
    let updates = raw_updates
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect();

    (orderings, updates)
}

fn is_valid_ordering(update: &[usize], before: usize, after: usize) -> bool {
    let before_idx = update.iter().position(|&n| n == before);
    let after_idx = update.iter().position(|&n| n == after);
    if let (Some(before_idx), Some(after_idx)) = (before_idx, after_idx) {
        before_idx < after_idx
    } else {
        true
    }
}

fn is_correctly_ordered(oderings: &[(usize, usize)], update: &[usize]) -> bool {
    oderings
        .iter()
        .all(|(before, after)| is_valid_ordering(update, *before, *after))
}

fn first_mismatch(oderings: &[(usize, usize)], update: &[usize]) -> Option<(usize, usize)> {
    oderings
        .iter()
        .find(|(before, after)| !is_valid_ordering(update, *before, *after))
        .copied()
}

fn middle_page_number(update: &[usize]) -> usize {
    update[update.len() / 2]
}

fn solve_part1(oderings: &[(usize, usize)], updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter(|&update| is_correctly_ordered(oderings, update))
        .map(|update| middle_page_number(update))
        .sum()
}

fn solve_part2(oderings: &[(usize, usize)], updates: Vec<Vec<usize>>) -> usize {
    updates
        .into_iter()
        .filter(|update| !is_correctly_ordered(oderings, update))
        .map(|mut update| {
            while let Some((before, after)) = first_mismatch(oderings, &update) {
                let before_idx = update.iter().position(|&n| n == before).unwrap();
                let after_idx = update.iter().position(|&n| n == after).unwrap();
                update.swap(before_idx, after_idx);
            }

            update
        })
        .map(|update| middle_page_number(&update))
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (orderings, updates) = parse(filename);
    let answer_part1 = solve_part1(&orderings, &updates);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&orderings, updates);
    println!("{}", answer_part2);
}
