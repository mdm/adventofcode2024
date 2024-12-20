use std::{
    collections::{btree_map::IntoValues, HashMap, HashSet, VecDeque},
    env::args,
    fs::read_to_string,
    path::Path,
};

type Position = (i64, i64);

fn parse<P>(filename: P) -> (Vec<Vec<char>>, Position, Position)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut start = None;
    let mut end = None;
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
                    'E' => {
                        end = Some((x as i64, y as i64));
                        '.'
                    }
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();

    (map, start.unwrap(), end.unwrap())
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

fn count_cheats(
    map: &[Vec<Option<usize>>],
    start: &Position,
    previous: &Position,
    end: &Position,
    cheated: bool,
    memo: &mut HashMap<(Position, bool), HashMap<usize, usize>>,
) {
    if memo.contains_key(&(*start, cheated)) {
        return;
    }

    if start == end {
        let mut cheats = HashMap::new();
        cheats.insert(0, 1);
        memo.insert((*start, cheated), cheats);
        return;
    }

    // dbg!(start, cheated);

    let neighbors = if cheated {
        vec![
            ((start.0 - 1, start.1), false),
            ((start.0 + 1, start.1), false),
            ((start.0, start.1 - 1), false),
            ((start.0, start.1 + 1), false),
        ]
    } else {
        vec![
            ((start.0 - 1, start.1), false),
            ((start.0 + 1, start.1), false),
            ((start.0, start.1 - 1), false),
            ((start.0, start.1 + 1), false),
            ((start.0 - 2, start.1), true),
            ((start.0 + 2, start.1), true),
            ((start.0, start.1 - 2), true),
            ((start.0, start.1 + 2), true),
        ]
    };

    let mut new_cheats: HashMap<usize, usize> = HashMap::new();
    for (neighbor, cheat) in neighbors.into_iter() {
        if neighbor == *previous {
            continue;
        }

        if neighbor.0 < 0
            || neighbor.1 < 0
            || neighbor.0 >= map[0].len() as i64
            || neighbor.1 >= map.len() as i64
        {
            continue;
        }

        let wrong_direction = match map[neighbor.1 as usize][neighbor.0 as usize] {
            Some(picoseconds) => picoseconds < map[start.1 as usize][start.0 as usize].unwrap(),
            None => true,
        };

        let wasteful = cheat
            && map[((start.1 + neighbor.1) / 2) as usize][((start.0 + neighbor.0) / 2) as usize]
                .is_some();

        if wrong_direction || wasteful {
            continue;
        }

        if !memo.contains_key(&(neighbor, cheat)) {
            count_cheats(map, &neighbor, start, end, cheat, memo);
        }

        let saved = if cheat {
            map[neighbor.1 as usize][neighbor.0 as usize].unwrap()
                - map[start.1 as usize][start.0 as usize].unwrap()
        } else {
            0
        };

        let old_cheats = &memo[&(neighbor, cheat)];
        if *start == (3, 2) {
            dbg!(neighbor, cheat, old_cheats);
        }
        for (old_saved, old_count) in old_cheats {
            // let mut old_count = *old_count;
            let new_saved = old_saved + saved;
            // new_cheats
            //     .entry(new_cost)
            //     .and_modify(|new_count| *new_count = *new_count.max(&mut old_count))
            //     .or_insert(old_count);
            *new_cheats.entry(new_saved).or_insert(0) += old_count;
        }
    }

    if *start == (1, 2) {
        dbg!(&new_cheats);
    }
    memo.insert((*start, cheated), new_cheats);
}

fn solve_part1(map: &[Vec<char>], start: &Position, end: &Position) -> usize {
    dbg!(start, end);
    let map = transform_map(map, start);
    let mut memo = HashMap::new();
    count_cheats(&map, start, start, end, false, &mut memo);
    let cheats = &memo[&(*start, false)];
    dbg!(cheats);
    cheats
        .iter()
        .filter(|(saved, _)| **saved >= 100)
        .map(|(_, count)| count)
        .sum()
}

fn solve_part2(map: &[Vec<char>], start: &Position, end: &Position) -> usize {
    0
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (map, start, end) = parse(filename);
    let answer_part1 = solve_part1(&map, &start, &end);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&map, &start, &end);
    println!("{}", answer_part2);
}
