use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    env::args,
    fs::read_to_string,
    path::Path,
};

use priority_queue::PriorityQueue;

fn parse<P>(filename: P) -> (Vec<Vec<char>>, (usize, usize), (usize, usize))
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

fn solve_part1(map: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> isize {
    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();

    visited.insert((start, Direction::East));
    queue.push((start, Direction::East), 0);

    while let Some(((position, heading), cost)) = queue.pop() {
        if position == end {
            return -cost;
        }

        // dbg!(position, heading, cost);
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
                if *old_cost < cost - 1 {
                    queue.change_priority(&(new_position, heading), cost - 1);
                }
            } else {
                queue.push((new_position, heading), cost - 1);
            }
        }

        if heading != Direction::East
            && heading != Direction::West
            && !visited.contains(&(position, Direction::East))
        {
            if let Some((_, old_cost)) = queue.get(&(new_position, Direction::East)) {
                if *old_cost < cost - 1000 {
                    queue.change_priority(&(position, Direction::East), cost - 1000);
                }
            } else {
                queue.push((position, Direction::East), cost - 1000);
            }
        }
        if heading != Direction::South
            && heading != Direction::North
            && !visited.contains(&(position, Direction::South))
        {
            if let Some((_, old_cost)) = queue.get(&(new_position, Direction::South)) {
                if *old_cost < cost - 1000 {
                    queue.change_priority(&(position, Direction::South), cost - 1000);
                }
            } else {
                queue.push((position, Direction::South), cost - 1000);
            }
        }
        if heading != Direction::West
            && heading != Direction::East
            && !visited.contains(&(position, Direction::West))
        {
            if let Some((_, old_cost)) = queue.get(&(new_position, Direction::West)) {
                if *old_cost < cost - 1000 {
                    queue.change_priority(&(position, Direction::West), cost - 1000);
                }
            } else {
                queue.push((position, Direction::West), cost - 1000);
            }
        }
        if heading != Direction::North
            && heading != Direction::South
            && !visited.contains(&(position, Direction::North))
        {
            if let Some((_, old_cost)) = queue.get(&(new_position, Direction::North)) {
                if *old_cost < cost - 1000 {
                    queue.change_priority(&(position, Direction::North), cost - 1000);
                }
            } else {
                queue.push((position, Direction::North), cost - 1000);
            }
        }
    }

    0
}

fn solve_part2(map: &[Vec<char>], start: (usize, usize), end: (usize, usize)) -> usize {
    0
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (map, start, end) = parse(filename);
    let answer_part1 = solve_part1(&map, start, end);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&map, start, end);
    println!("{}", answer_part2);
}
