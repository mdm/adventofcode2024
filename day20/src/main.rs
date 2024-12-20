use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fs::read_to_string,
    path::Path,
};

type Position = (i64, i64);

fn parse<P>(filename: P) -> (Vec<Vec<char>>, Position)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut start = None;
    let map = raw_input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => '.',
                    '#' => '#',
                    'S' => {
                        start = Some((x as i64, y as i64));
                        '.'
                    }
                    'E' => '.',
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();

    (map, start.unwrap())
}

fn transform_map(map: &[Vec<char>], start: &Position) -> Vec<Vec<Option<usize>>> {
    let mut new_map = vec![vec![None; map[0].len()]; map.len()];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((*start, 0));
    visited.insert(*start);

    while let Some((position, picoseconds)) = queue.pop_front() {
        new_map[position.1 as usize][position.0 as usize] = Some(picoseconds);

        let neighbors = [
            (position.0 - 1, position.1),
            (position.0 + 1, position.1),
            (position.0, position.1 - 1),
            (position.0, position.1 + 1),
        ];

        for neighbor in neighbors.iter() {
            if visited.contains(neighbor) {
                continue;
            }

            if neighbor.0 < 0
                || neighbor.1 < 0
                || neighbor.0 >= map[0].len() as i64
                || neighbor.1 >= map.len() as i64
            {
                continue;
            }

            if map[neighbor.1 as usize][neighbor.0 as usize] == '#' {
                continue;
            }

            queue.push_back((*neighbor, picoseconds + 1));
            visited.insert(*neighbor);
        }
    }

    new_map
}

fn count_cheats(map: &[Vec<Option<usize>>], picoseconds: i64) -> HashMap<usize, usize> {
    let mut cheats = HashMap::new();

    for start_y in 0..map.len() as i64 {
        for start_x in 0..map[0].len() as i64 {
            for end_y in 0..map.len() as i64 {
                for end_x in 0..map[0].len() as i64 {
                    let (Some(start_steps), Some(end_steps)) = (
                        map[start_y as usize][start_x as usize],
                        map[end_y as usize][end_x as usize],
                    ) else {
                        continue;
                    };

                    if end_steps <= start_steps {
                        continue;
                    }

                    let manhattan_distance = (start_x - end_x).abs() + (start_y - end_y).abs();

                    if manhattan_distance <= picoseconds {
                        *cheats
                            .entry(end_steps - start_steps - manhattan_distance as usize)
                            .or_insert(0) += 1;
                    }
                }
            }
        }
    }

    cheats
}

fn solve_part1(map: &[Vec<Option<usize>>]) -> usize {
    let cheats = count_cheats(map, 2);
    cheats
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, count)| count)
        .sum()
}

fn solve_part2(map: &[Vec<Option<usize>>]) -> usize {
    let cheats = count_cheats(map, 20);
    cheats
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, count)| count)
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (map, start) = parse(filename);
    let map = transform_map(&map, &start);
    let answer_part1 = solve_part1(&map);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&map);
    println!("{}", answer_part2);
}
