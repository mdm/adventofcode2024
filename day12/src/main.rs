use std::{collections::HashSet, env::args, fs::read_to_string, path::Path};

#[derive(Debug, Clone)]
struct Region {
    type_: char,
    tiles: HashSet<(i32, i32)>,
}

fn parse<P>(filename: P) -> Vec<Region>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut regions: Vec<Region> = Vec::new();
    raw_input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            let (merge, keep) = regions.iter().cloned().partition::<Vec<_>, _>(|r| {
                r.type_ == c
                    && (r.tiles.contains(&(x as i32 - 1, y as i32))
                        || r.tiles.contains(&(x as i32 + 1, y as i32))
                        || r.tiles.contains(&(x as i32, y as i32 - 1))
                        || r.tiles.contains(&(x as i32, y as i32 + 1)))
            });
            regions = keep;

            let mut new_region = Region {
                type_: c,
                tiles: HashSet::new(),
            };
            new_region.tiles.insert((x as i32, y as i32));

            if !merge.is_empty() {
                merge.iter().for_each(|r| {
                    new_region.tiles.extend(r.tiles.iter());
                });
            }

            regions.push(new_region);
        });
    });

    regions
}

fn solve_part1(garden: &[Region]) -> usize {
    garden
        .iter()
        .map(|region| {
            let perimeter: usize = region
                .tiles
                .iter()
                .map(|(x, y)| {
                    let mut perimeter = 0;
                    if !region.tiles.contains(&(*x - 1, *y)) {
                        perimeter += 1;
                    }
                    if !region.tiles.contains(&(*x + 1, *y)) {
                        perimeter += 1;
                    }
                    if !region.tiles.contains(&(*x, *y - 1)) {
                        perimeter += 1;
                    }
                    if !region.tiles.contains(&(*x, *y + 1)) {
                        perimeter += 1;
                    }

                    perimeter
                })
                .sum();
            let area = region.tiles.len();
            perimeter * area
        })
        .sum()
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Side {
    direction: Direction,
    offset: i32,
    start: i32,
    end: i32,
}

fn solve_part2(garden: &[Region]) -> usize {
    garden
        .iter()
        .map(|region| {
            let sides: HashSet<Side> = region
                .tiles
                .iter()
                .flat_map(|(x, y)| {
                    let mut sides = Vec::new();
                    if !region.tiles.contains(&(*x - 1, *y)) {
                        let mut start = *y;
                        while region.tiles.contains(&(*x, start))
                            && !region.tiles.contains(&(*x - 1, start))
                        {
                            start -= 1;
                        }
                        let mut end = *y;
                        while region.tiles.contains(&(*x, end))
                            && !region.tiles.contains(&(*x - 1, end))
                        {
                            end += 1;
                        }
                        sides.push(Side {
                            direction: Direction::Left,
                            offset: *x,
                            start: start + 1,
                            end: end - 1,
                        });
                    }
                    if !region.tiles.contains(&(*x + 1, *y)) {
                        let mut start = *y;
                        while region.tiles.contains(&(*x, start))
                            && !region.tiles.contains(&(*x + 1, start))
                        {
                            start -= 1;
                        }
                        let mut end = *y;
                        while region.tiles.contains(&(*x, end))
                            && !region.tiles.contains(&(*x + 1, end))
                        {
                            end += 1;
                        }
                        sides.push(Side {
                            direction: Direction::Right,
                            offset: *x,
                            start: start + 1,
                            end: end - 1,
                        });
                    }
                    if !region.tiles.contains(&(*x, *y - 1)) {
                        let mut start = *x;
                        while region.tiles.contains(&(start, *y))
                            && !region.tiles.contains(&(start, *y - 1))
                        {
                            start -= 1;
                        }
                        let mut end = *x;
                        while region.tiles.contains(&(end, *y))
                            && !region.tiles.contains(&(end, *y - 1))
                        {
                            end += 1;
                        }
                        sides.push(Side {
                            direction: Direction::Up,
                            offset: *y,
                            start: start + 1,
                            end: end - 1,
                        });
                    }
                    if !region.tiles.contains(&(*x, *y + 1)) {
                        let mut start = *x;
                        while region.tiles.contains(&(start, *y))
                            && !region.tiles.contains(&(start, *y + 1))
                        {
                            start -= 1;
                        }
                        let mut end = *x;
                        while region.tiles.contains(&(end, *y))
                            && !region.tiles.contains(&(end, *y + 1))
                        {
                            end += 1;
                        }
                        sides.push(Side {
                            direction: Direction::Down,
                            offset: *y,
                            start: start + 1,
                            end: end - 1,
                        });
                    }

                    sides
                })
                .collect();
            let sides = sides.len();
            let area = region.tiles.len();
            sides * area
        })
        .sum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let garden = parse(filename);
    let answer_part1 = solve_part1(&garden);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&garden);
    println!("{}", answer_part2);
}
