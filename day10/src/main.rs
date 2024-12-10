use std::{
    collections::{HashSet, VecDeque},
    env::args,
    fs::read_to_string,
    path::Path,
};

fn parse<P>(filename: P) -> Vec<Vec<i32>>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    raw_input
        .lines()
        .map(|line| {
            line.chars()
                .map(|tile| tile.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

fn on_map(map: &[Vec<i32>], x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && (x as usize) < map[0].len() && (y as usize) < map.len()
}

fn bfs(map: &[Vec<i32>], start: (i32, i32), measure: &Measure) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut trail_count = 0;
    queue.push_back(start);
    visited.insert(start);

    while let Some((x, y)) = queue.pop_front() {
        let current_height = map[y as usize][x as usize];
        if current_height == 9 {
            trail_count += 1;
        }

        if let Measure::Rating = measure {
            visited.insert((x, y));
        }

        let neighbors = vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)];
        for (nx, ny) in neighbors {
            if on_map(map, nx, ny) && map[ny as usize][nx as usize] - current_height == 1 {
                let next = (nx, ny);
                if !visited.contains(&next) {
                    queue.push_back(next);
                    if let Measure::Score = measure {
                        visited.insert(next);
                    }
                }
            }
        }
    }

    trail_count
}

enum Measure {
    Score,
    Rating,
}

fn measure(map: &[Vec<i32>], measure: &Measure) -> usize {
    map.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(x, col)| {
                    if *col == 0 {
                        Some(bfs(map, (x as i32, y as i32), measure))
                    } else {
                        None
                    }
                })
                .sum::<usize>()
        })
        .sum()
}

fn solve_part1(map: &[Vec<i32>]) -> usize {
    measure(map, &Measure::Score)
}

fn solve_part2(map: &[Vec<i32>]) -> usize {
    measure(map, &Measure::Rating)
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let map = parse(filename);
    let answer_part1 = solve_part1(&map);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&map);
    println!("{}", answer_part2);
}
