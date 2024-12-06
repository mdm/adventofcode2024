use std::{collections::HashSet, env::args, fs::read_to_string, path::Path};

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Obstacle,
    Visited,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Guard {
    x: i32,
    y: i32,
    direction: Direction,
}

fn parse<P>(filename: P) -> (Vec<Vec<Tile>>, Guard)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut guard = None;
    let grid = raw_input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => Tile::Empty,
                    '#' => Tile::Obstacle,
                    '^' => {
                        guard = Some(Guard {
                            x: x as i32,
                            y: y as i32,
                            direction: Direction::Up,
                        });
                        Tile::Empty
                    }
                    _ => panic!("Invalid character in input"),
                })
                .collect()
        })
        .collect();

    (grid, guard.unwrap())
}

#[allow(dead_code)]
fn print_grid(grid: &Vec<Vec<Tile>>) {
    for row in grid {
        for tile in row {
            match tile {
                Tile::Empty => print!("."),
                Tile::Obstacle => print!("#"),
                Tile::Visited => print!("X"),
            }
        }
        println!();
    }
}

fn on_grid(grid: &[Vec<Tile>], x: i32, y: i32) -> bool {
    x >= 0 && x < grid[0].len() as i32 && y >= 0 && y < grid.len() as i32
}

fn run_guard(grid: &mut [Vec<Tile>], guard: &mut Guard) {
    let mut history = HashSet::new();

    loop {
        grid[guard.y as usize][guard.x as usize] = Tile::Visited;

        if history.contains(guard) {
            break;
        }

        history.insert(guard.clone());

        let (x, y) = match guard.direction {
            Direction::Left => (guard.x - 1, guard.y),
            Direction::Right => (guard.x + 1, guard.y),
            Direction::Up => (guard.x, guard.y - 1),
            Direction::Down => (guard.x, guard.y + 1),
        };

        if !on_grid(grid, x, y) {
            guard.x = x;
            guard.y = y;

            break;
        }

        match grid[y as usize][x as usize] {
            Tile::Obstacle => match guard.direction {
                Direction::Left => guard.direction = Direction::Up,
                Direction::Right => guard.direction = Direction::Down,
                Direction::Up => guard.direction = Direction::Right,
                Direction::Down => guard.direction = Direction::Left,
            },
            _ => {
                guard.x = x;
                guard.y = y;
            }
        }
    }
}

fn solve_part1(grid: &Vec<Vec<Tile>>, guard: &Guard) -> usize {
    let mut grid = (*grid).clone();
    let mut guard = (*guard).clone();

    run_guard(&mut grid, &mut guard);

    grid.iter()
        .flatten()
        .filter(|tile| matches!(tile, Tile::Visited))
        .count()
}

fn solve_part2(original_grid: &Vec<Vec<Tile>>, original_guard: &Guard) -> usize {
    let mut count = 0;

    for y in 0..original_grid.len() {
        for x in 0..original_grid[0].len() {
            let mut grid = (*original_grid).clone();
            let mut guard = (*original_guard).clone();

            grid[y][x] = Tile::Obstacle;
            run_guard(&mut grid, &mut guard);

            if on_grid(&grid, guard.x, guard.y) {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (grid, guard) = parse(filename);
    let answer_part1 = solve_part1(&grid, &guard);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&grid, &guard);
    println!("{}", answer_part2);
}
