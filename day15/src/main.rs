use std::{env::args, fs::read_to_string, path::Path};

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse<P>(filename: P) -> (Vec<Vec<char>>, Vec<Direction>, usize, usize)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let (map, moves) = raw_input.split_once("\n\n").unwrap();

    let mut map: Vec<Vec<char>> = map.lines().map(|line| line.chars().collect()).collect();
    let moves = moves
        .lines()
        .flat_map(|line| {
            line.chars().map(|c| match c {
                '<' => Direction::Left,
                '>' => Direction::Right,
                '^' => Direction::Up,
                'v' => Direction::Down,
                _ => panic!("Invalid direction"),
            })
        })
        .collect();

    let (start_x, start_y) = map
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &c)| if c == '@' { Some((x, y)) } else { None })
        })
        .fold(None, Option::or)
        .expect("Start position not found");

    map[start_y][start_x] = '.';

    (map, moves, start_x, start_y)
}

#[allow(dead_code)]
fn print(map: &[Vec<char>], robot_x: usize, robot_y: usize) {
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if x == robot_x && y == robot_y {
                print!("@");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
}

fn push_narrow(map: &mut [Vec<char>], x: usize, y: usize, direction: &Direction) -> (usize, usize) {
    let (offset_x, offset_y) = match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
    };

    let start_x = x.checked_add_signed(offset_x).unwrap();
    let start_y = y.checked_add_signed(offset_y).unwrap();
    let mut stopper_x = start_x;
    let mut stopper_y = start_y;
    while map[stopper_y][stopper_x] != '#' && map[stopper_y][stopper_x] != '.' {
        stopper_x = stopper_x.checked_add_signed(offset_x).unwrap();
        stopper_y = stopper_y.checked_add_signed(offset_y).unwrap();
    }

    if map[stopper_y][stopper_x] == '#' {
        return (x, y);
    }

    let tmp = map[start_y][start_x];
    map[start_y][start_x] = map[stopper_y][stopper_x];
    map[stopper_y][stopper_x] = tmp;

    (start_x, start_y)
}

fn solve_part1(map: &[Vec<char>], moves: &[Direction], start_x: usize, start_y: usize) -> usize {
    let mut map = map.to_vec();
    let mut x = start_x;
    let mut y = start_y;

    for direction in moves {
        (x, y) = push_narrow(&mut map, x, y, direction);
    }

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &c)| if c == 'O' { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn transform_map(
    map: &[Vec<char>],
    start_x: usize,
    start_y: usize,
) -> (Vec<Vec<char>>, usize, usize) {
    let map = map
        .iter()
        .map(|row| {
            row.iter()
                .flat_map(|&c| match c {
                    '#' => vec!['#', '#'],
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();

    (map, 2 * start_x, start_y)
}

fn push_wide(
    map: &mut [Vec<char>],
    x: usize,
    y: usize,
    direction: &Direction,
    modify: bool,
) -> Option<(usize, usize)> {
    let (offset_x, offset_y) = match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
    };

    let start_x = x.checked_add_signed(offset_x).unwrap();
    let start_y = y.checked_add_signed(offset_y).unwrap();

    match map[start_y][start_x] {
        '#' => None,
        '.' => Some((start_x, start_y)),
        stopper @ ('[' | ']') => match direction {
            Direction::Left | Direction::Right => {
                match push_wide(map, start_x, start_y, direction, modify) {
                    Some((new_x, new_y)) => {
                        if modify {
                            map[new_y][new_x] = stopper;
                            map[start_y][start_x] = '.';
                        }
                        Some((start_x, start_y))
                    }
                    None => None,
                }
            }
            Direction::Up | Direction::Down => {
                let (left_x, left_y, right_x, right_y) = match stopper {
                    '[' => (start_x, start_y, start_x + 1, start_y),
                    ']' => (start_x - 1, start_y, start_x, start_y),
                    _ => unreachable!(),
                };

                if let (Some((_, left)), Some((_, right))) = (
                    push_wide(map, left_x, left_y, direction, modify),
                    push_wide(map, right_x, right_y, direction, modify),
                ) {
                    if modify {
                        map[left][left_x] = '[';
                        map[right][right_x] = ']';
                        map[left_y][left_x] = '.';
                        map[right_y][right_x] = '.';
                    }
                    Some((start_x, start_y))
                } else {
                    None
                }
            }
        },
        _ => unreachable!(),
    }
}

fn solve_part2(map: &[Vec<char>], moves: &[Direction], start_x: usize, start_y: usize) -> usize {
    let (mut map, mut x, mut y) = transform_map(map, start_x, start_y);

    for direction in moves {
        if let Some(new_position) = push_wide(&mut map, x, y, direction, false) {
            push_wide(&mut map, x, y, direction, true);
            (x, y) = new_position;
        }
    }

    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &c)| if c == '[' { 100 * y + x } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (map, moves, start_x, start_y) = parse(filename);
    let answer_part1 = solve_part1(&map, &moves, start_x, start_y);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&map, &moves, start_x, start_y);
    println!("{}", answer_part2);
}
