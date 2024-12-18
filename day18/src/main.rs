use std::{
    collections::{HashSet, VecDeque},
    env::args,
    fs::read_to_string,
    path::Path,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

fn parse<P>(filename: P) -> Vec<Position>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Position {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

fn bfs(memory: &[Vec<char>]) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    let start = Position { x: 0, y: 0 };
    let goal = Position {
        x: memory[0].len() as i64 - 1,
        y: memory.len() as i64 - 1,
    };
    queue.push_back((start, 0));
    visited.insert(start);

    while let Some((position, steps)) = queue.pop_front() {
        if position == goal {
            return Some(steps);
        }

        let neighbors = [
            Position {
                x: position.x + 1,
                y: position.y,
            },
            Position {
                x: position.x - 1,
                y: position.y,
            },
            Position {
                x: position.x,
                y: position.y + 1,
            },
            Position {
                x: position.x,
                y: position.y - 1,
            },
        ];

        for neighbor in neighbors.iter() {
            if visited.contains(neighbor) {
                continue;
            }

            if neighbor.x < 0
                || neighbor.y < 0
                || neighbor.x >= memory[0].len() as i64
                || neighbor.y >= memory.len() as i64
            {
                continue;
            }

            if memory[neighbor.y as usize][neighbor.x as usize] == '#' {
                continue;
            }

            queue.push_back((*neighbor, steps + 1));
            visited.insert(*neighbor);
        }
    }

    None
}

fn solve_part1(bytes: &[Position], space: usize, time: usize) -> usize {
    let mut memory = vec![vec!['.'; space + 1]; space + 1];

    for byte in bytes.iter().take(time) {
        memory[byte.y as usize][byte.x as usize] = '#';
    }

    bfs(&memory).unwrap()
}

fn solve_part2(bytes: &[Position], space: usize, _time: usize) -> String {
    let mut memory = vec![vec!['.'; space + 1]; space + 1];

    for byte in bytes.iter() {
        memory[byte.y as usize][byte.x as usize] = '#';
        if bfs(&memory).is_none() {
            return format!("{},{}", byte.x, byte.y);
        }
    }

    unreachable!()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let space = args()
        .nth(2)
        .expect("No space argument provided")
        .parse()
        .unwrap();
    let time = args()
        .nth(3)
        .expect("No time argument provided")
        .parse()
        .unwrap();
    let bytes = parse(filename);
    let answer_part1 = solve_part1(&bytes, space, time);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&bytes, space, time);
    println!("{}", answer_part2);
}
