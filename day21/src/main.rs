use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fs::read_to_string,
    path::Path,
    vec,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Keypad {
    position_to_key: HashMap<Position, char>,
    key_to_position: HashMap<char, Position>,
    initial_position: Position,
    missing_key_position: Position,
}

fn parse<P>(filename: P) -> Vec<String>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input.lines().map(|line| line.to_string()).collect()
}

fn extract_paths(
    keypad: &Keypad,
    start: Position,
    goal: Position,
    predecessors: &HashMap<Position, Vec<(Position, char)>>,
) -> Vec<String> {
    if start == goal {
        return vec!["".to_string()];
    }

    predecessors[&goal]
        .iter()
        .flat_map(|(predecessor, key)| {
            extract_paths(keypad, start, *predecessor, predecessors)
                .into_iter()
                .filter_map(|path| {
                    if *predecessor == keypad.missing_key_position {
                        None
                    } else {
                        let mut path = path;
                        path.push(*key);
                        Some(path)
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn all_shortest_paths(keypad: &Keypad, start: Position, goal: Position) -> Vec<String> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut predecessors = HashMap::new();

    queue.push_back((start, 0));

    while let Some((current, cost)) = queue.pop_front() {
        if current == goal {
            let mut paths = extract_paths(keypad, start, goal, &predecessors);
            paths.sort();
            paths.dedup();
            return paths;
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

            if !keypad.position_to_key.contains_key(neighbor) {
                continue;
            }

            queue.push_back((*neighbor, cost + 1));
            predecessors
                .entry(*neighbor)
                .or_insert(Vec::new())
                .push((current, *key));
        }
    }

    unreachable!("No path found");
}

fn shortest_sequence_dfs_inner(
    keypads: &[Keypad],
    start: Position,
    goal: Position,
    depth: usize,
    memo: &mut HashMap<(Position, Position, usize), usize>,
) -> usize {
    if let Some(&cost) = memo.get(&(start, goal, depth)) {
        return cost;
    }

    let cost = all_shortest_paths(&keypads[depth], start, goal)
        .into_iter()
        .map(|subsequence| {
            let mut subsequence = subsequence;
            subsequence.push('A');
            shortest_sequence_dfs(keypads, &subsequence, depth + 1, memo)
        })
        .min()
        .unwrap();

    memo.insert((start, goal, depth), cost);

    cost
}

fn shortest_sequence_dfs(
    keypads: &[Keypad],
    sequence: &str,
    depth: usize,
    memo: &mut HashMap<(Position, Position, usize), usize>,
) -> usize {
    if depth == keypads.len() {
        return sequence.len();
    }

    let mut start = keypads[depth].initial_position;
    sequence
        .chars()
        .map(|c| {
            let goal = keypads[depth].key_to_position[&c];
            let cost = shortest_sequence_dfs_inner(keypads, start, goal, depth, memo);
            start = goal;

            cost
        })
        .sum()
}

fn open_doors(codes: &[String], proximity: usize) -> usize {
    let code_pad_keys = [
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
    ];
    let code_keypad = Keypad {
        position_to_key: code_pad_keys.iter().copied().collect(),
        key_to_position: code_pad_keys.iter().map(|&(k, v)| (v, k)).collect(),
        initial_position: Position { x: 2, y: 3 },
        missing_key_position: Position { x: 0, y: 3 },
    };

    let directional_keypad_keys = [
        (Position { x: 1, y: 0 }, '^'),
        (Position { x: 2, y: 0 }, 'A'),
        (Position { x: 0, y: 1 }, '<'),
        (Position { x: 1, y: 1 }, 'v'),
        (Position { x: 2, y: 1 }, '>'),
    ];

    let directional_keypad = Keypad {
        position_to_key: directional_keypad_keys.iter().copied().collect(),
        key_to_position: directional_keypad_keys
            .iter()
            .map(|&(k, v)| (v, k))
            .collect(),
        initial_position: Position { x: 2, y: 0 },
        missing_key_position: Position { x: 0, y: 0 },
    };

    let mut robot_controlled_keypads = vec![code_keypad];
    for _ in 0..proximity {
        robot_controlled_keypads.push(directional_keypad.clone());
    }

    let mut memo = HashMap::new();
    codes
        .iter()
        // .skip(0)
        // .take(1)
        .map(|code| {
            let numeric_part = code
                .chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let cost = shortest_sequence_dfs(&robot_controlled_keypads.clone(), code, 0, &mut memo);
            cost * numeric_part
        })
        .sum()
}

fn solve_part1(codes: &[String]) -> usize {
    open_doors(codes, 2)
}

fn solve_part2(codes: &[String]) -> usize {
    open_doors(codes, 25)
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let codes = parse(filename);
    let answer_part1 = solve_part1(&codes);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&codes);
    println!("{}", answer_part2);
}
