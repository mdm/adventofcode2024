use std::{
    cmp::Reverse,
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    path::Path,
};

use priority_queue::PriorityQueue;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Keypad {
    keys: HashMap<Position, char>,
    position: Position,
}

fn parse<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input.lines().map(|line| line.to_string()).collect()
}

fn shortest_sequence_inner(keypads: &mut [Keypad], goal: &Position, depth: usize) -> usize {
    let mut queue = PriorityQueue::new();
    let mut visited = HashSet::new();

    queue.push(keypads[depth].position, Reverse(0));

    while let Some((current, Reverse(cost))) = queue.pop() {
        if current == *goal {
            dbg!(depth, goal, cost);
            keypads[depth].position = current;
            return cost;
        }

        visited.insert(current);

        let neighbors = [
            (
                Position {
                    x: current.x - 1,
                    y: current.y,
                },
                '<',
            ),
            (
                Position {
                    x: current.x + 1,
                    y: current.y,
                },
                '>',
            ),
            (
                Position {
                    x: current.x,
                    y: current.y - 1,
                },
                '^',
            ),
            (
                Position {
                    x: current.x,
                    y: current.y + 1,
                },
                'v',
            ),
        ];

        for (neighbor, key) in neighbors.iter() {
            if visited.contains(neighbor) {
                continue;
            }

            if !keypads[depth].keys.contains_key(neighbor) {
                continue;
            }

            let subcost = if depth < keypads.len() - 1 {
                let subgoal = *keypads[depth + 1]
                    .keys
                    .iter()
                    .find(|(_, &v)| v == *key)
                    .unwrap()
                    .0;

                shortest_sequence_inner(keypads, &subgoal, depth + 1)
            } else {
                1
            };
            match queue.get(neighbor) {
                Some((_, Reverse(old_cost))) if *old_cost <= cost + subcost => continue,
                _ => {
                    queue.push(*neighbor, Reverse(cost + subcost));
                }
            }
        }
    }

    unreachable!();
}

fn shortest_sequence(keypads: &mut [Keypad], sequence: &str) -> usize {
    sequence
        .chars()
        .map(|c| {
            let goal = *keypads[0]
                .keys
                .iter()
                .find(|(_, &v)| dbg!(v) == dbg!(c))
                .unwrap()
                .0;
            let cost = shortest_sequence_inner(keypads, &goal, 0);

            cost
        })
        .sum()
}

fn solve_part1(codes: &[String]) -> usize {
    let code_keypad = Keypad {
        keys: [
            (Position { x: 0, y: 0 }, '7'),
            (Position { x: 1, y: 0 }, '8'),
            (Position { x: 2, y: 0 }, '9'),
            (Position { x: 0, y: 1 }, '4'),
            (Position { x: 1, y: 1 }, '5'),
            (Position { x: 2, y: 1 }, '6'),
            (Position { x: 0, y: 2 }, '1'),
            (Position { x: 1, y: 2 }, '2'),
            (Position { x: 2, y: 2 }, '3'),
            (Position { x: 1, y: 3 }, '0'),
            (Position { x: 2, y: 3 }, 'A'),
        ]
        .into_iter()
        .collect(),
        position: Position { x: 2, y: 3 },
    };

    let directional_keypad = Keypad {
        keys: [
            (Position { x: 1, y: 0 }, '^'),
            (Position { x: 2, y: 0 }, 'A'),
            (Position { x: 0, y: 1 }, '<'),
            (Position { x: 1, y: 1 }, 'v'),
            (Position { x: 2, y: 1 }, '>'),
        ]
        .into_iter()
        .collect(),
        position: Position { x: 2, y: 0 },
    };

    // let robot_controlled_keypads =
    //     vec![code_keypad, directional_keypad.clone(), directional_keypad];

    let robot_controlled_keypads = vec![code_keypad];

    codes
        .iter()
        .take(1)
        .map(|code| {
            let numeric_part = code
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let cost = shortest_sequence(&mut robot_controlled_keypads.clone(), code);
            dbg!(cost) * dbg!(numeric_part)
        })
        .sum()
}

fn solve_part2(codes: &[String]) -> usize {
    0
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let codes = parse(filename);
    let answer_part1 = solve_part1(&codes);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&codes);
    println!("{}", answer_part2);
}
