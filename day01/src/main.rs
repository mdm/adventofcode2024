use std::{collections::HashMap, env::args, fs::read_to_string, path::Path};

fn parse<P>(filename: P) -> (Vec<i32>, Vec<i32>)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut left = Vec::new();
    let mut right = Vec::new();
    raw_input.lines().for_each(|line| {
        let mut parts = line.split(" ");

        let l = parts.next().expect("No left side");
        let l = l.parse::<i32>().expect("Failed to parse left side");
        left.push(l);

        let r = parts.last().expect("No right side");
        let r = r.parse::<i32>().expect("Failed to parse right side");
        right.push(r);
    });

    (left, right)
}

fn solve_part1(left: &[i32], right: &[i32]) -> i32 {
    let mut left = left.to_owned();
    left.sort();
    let mut right = right.to_owned();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum()
}

fn solve_part2(left: &[i32], right: &[i32]) -> i32 {
    let mut histogram = HashMap::new();
    for r in right {
        *histogram.entry(r).or_insert(0) += 1;
    }

    left.iter()
        .map(|l| match histogram.get(l) {
            Some(&count) => l * count,
            None => 0,
        })
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (left, right) = parse(filename);
    let answer_part1 = solve_part1(&left, &right);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&left, &right);
    println!("{}", answer_part2);
}
