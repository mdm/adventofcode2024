use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    path::Path,
};

use priority_queue::PriorityQueue;

type Map = Vec<Vec<char>>;
type Position = (usize, usize);

fn parse<P>(filename: P) -> (Map, Position, Position)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut map: Vec<Vec<char>> = raw_input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'S' { Some((x, y)) } else { None })
        })
        .fold(None, Option::or)
        .expect("Start position not found");

    map[start_y][start_x] = '.';

    let (end_x, end_y) = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == 'E' { Some((x, y)) } else { None })
        })
        .fold(None, Option::or)
        .expect("End position not found");

    map[end_x][end_y] = '.';

    (map, (start_x, start_y), (end_x, end_y))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    East,
    South,
    West,
    North,
}

#[allow(dead_code)]
fn print(map: &[Vec<char>], on_path: &HashSet<(usize, usize)>) {
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if on_path.contains(&(x, y)) {
                print!("O");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn measure_paths(
    predecessors: &HashMap<(Position, Direction), Vec<(Position, Direction)>>,
    start: (usize, usize),
    end: (usize, usize),
    heading: Direction,
) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();

    result.insert(end);
    if end == start {
        return result;
    }

    for predecessor in predecessors.get(&(end, heading)).unwrap() {
        result.extend(measure_paths(
            predecessors,
            start,
            predecessor.0,
            predecessor.1,
        ));
    }

    result
}

fn shortest_path(map: &[Vec<char>], start: Position, end: Position) -> (isize, usize) {
    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();

    visited.insert((start, Direction::East));
    queue.push((start, Direction::East), 0);

    let mut predecessors = HashMap::new();

    while let Some(((position, heading), cost)) = queue.pop() {
        if position == end {
            let on_path = measure_paths(&predecessors, start, end, heading);
            return (-cost, on_path.len());
        }

        visited.insert((position, heading));

        let new_position = match heading {
            Direction::East => (position.0 + 1, position.1),
            Direction::South => (position.0, position.1 + 1),
            Direction::West => (position.0 - 1, position.1),
            Direction::North => (position.0, position.1 - 1),
        };

        if map[new_position.1][new_position.0] != '#' && !visited.contains(&(new_position, heading))
        {
            if let Some((_, old_cost)) = queue.get(&(new_position, heading)) {
                // same as *old_cost <= cost - 1
                if *old_cost < cost {
                    queue.change_priority(&(new_position, heading), cost - 1);
                    predecessors
                        .entry((new_position, heading))
                        .or_insert(Vec::new())
                        .push((position, heading))
                }
            } else {
                queue.push((new_position, heading), cost - 1);
                predecessors
                    .entry((new_position, heading))
                    .or_insert(Vec::new())
                    .push((position, heading))
            }
        }

        if heading != Direction::East
            && heading != Direction::West
            && !visited.contains(&(position, Direction::East))
        {
            if let Some((_, old_cost)) = queue.get(&(position, Direction::East)) {
                if *old_cost <= cost - 1000 {
                    queue.change_priority(&(position, Direction::East), cost - 1000);
                    predecessors
                        .entry((position, Direction::East))
                        .or_insert(Vec::new())
                        .push((position, heading));
                }
            } else {
                queue.push((position, Direction::East), cost - 1000);
                predecessors
                    .entry((position, Direction::East))
                    .or_insert(Vec::new())
                    .push((position, heading));
            }
        }
        if heading != Direction::South
            && heading != Direction::North
            && !visited.contains(&(position, Direction::South))
        {
            if let Some((_, old_cost)) = queue.get(&(position, Direction::South)) {
                if *old_cost <= cost - 1000 {
                    queue.change_priority(&(position, Direction::South), cost - 1000);
                    predecessors
                        .entry((position, Direction::South))
                        .or_insert(Vec::new())
                        .push((position, heading));
                }
            } else {
                queue.push((position, Direction::South), cost - 1000);
                predecessors
                    .entry((position, Direction::South))
                    .or_insert(Vec::new())
                    .push((position, heading));
            }
        }
        if heading != Direction::West
            && heading != Direction::East
            && !visited.contains(&(position, Direction::West))
        {
            if let Some((_, old_cost)) = queue.get(&(position, Direction::West)) {
                if *old_cost <= cost - 1000 {
                    queue.change_priority(&(position, Direction::West), cost - 1000);
                    predecessors
                        .entry((position, Direction::West))
                        .or_insert(Vec::new())
                        .push((position, heading));
                }
            } else {
                queue.push((position, Direction::West), cost - 1000);
                predecessors
                    .entry((position, Direction::West))
                    .or_insert(Vec::new())
                    .push((position, heading));
            }
        }
        if heading != Direction::North
            && heading != Direction::South
            && !visited.contains(&(position, Direction::North))
        {
            if let Some((_, old_cost)) = queue.get(&(position, Direction::North)) {
                if *old_cost <= cost - 1000 {
                    queue.change_priority(&(position, Direction::North), cost - 1000);
                    predecessors
                        .entry((position, Direction::North))
                        .or_insert(Vec::new())
                        .push((position, heading));
                }
            } else {
                queue.push((position, Direction::North), cost - 1000);
                predecessors
                    .entry((position, Direction::North))
                    .or_insert(Vec::new())
                    .push((position, heading));
            }
        }
    }

    (-1, 0)
}

fn solve_part1(map: &[Vec<char>], start: Position, end: Position) -> isize {
    shortest_path(map, start, end).0
}

fn solve_part2(map: &[Vec<char>], start: Position, end: Position) -> usize {
    shortest_path(map, start, end).1
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (map, start, end) = parse(filename);
    let answer_part1 = solve_part1(&map, start, end);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&map, start, end);
    println!("{}", answer_part2);
}
