use std::{env::args, fs::read_to_string, path::Path};

struct Equation {
    test: usize,
    operands: Vec<usize>,
}

fn parse<P>(filename: P) -> Vec<Equation>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .map(|line| {
            let (test, operands) = line.split_once(": ").unwrap();
            let test = test.parse().unwrap();
            let operands = operands
                .split_ascii_whitespace()
                .map(|operand| operand.parse().unwrap())
                .collect();

            Equation { test, operands }
        })
        .collect()
}

fn solve_equation(equation: &Equation, result: usize, skip: usize, with_concat: bool) -> bool {
    if result > equation.test || skip == equation.operands.len() {
        return result == equation.test;
    }

    let magnitude = 10_usize.pow(equation.operands[skip].ilog10() + 1);

    solve_equation(
        equation,
        result + equation.operands[skip],
        skip + 1,
        with_concat,
    ) || solve_equation(
        equation,
        result * equation.operands[skip],
        skip + 1,
        with_concat,
    ) || (with_concat
        && solve_equation(
            equation,
            result * magnitude + equation.operands[skip],
            skip + 1,
            true,
        ))
}

fn solve_part1(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|equation| solve_equation(equation, 0, 0, false))
        .map(|equation| equation.test)
        .sum()
}

fn solve_part2(equations: &[Equation]) -> usize {
    equations
        .iter()
        .filter(|equation| solve_equation(equation, 0, 0, true))
        .map(|equation| equation.test)
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let equations = parse(filename);
    let answer_part1 = solve_part1(&equations);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&equations);
    println!("{}", answer_part2);
}
