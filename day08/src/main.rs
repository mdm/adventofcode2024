use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    path::Path,
};

#[derive(Debug)]
struct Antenna {
    frequency: char,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Antinode {
    frequency: char,
    x: usize,
    y: usize,
}

fn parse<P>(filename: P) -> ((usize, usize), HashMap<char, Vec<Antenna>>)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let width = raw_input.lines().next().unwrap().len();
    let height = raw_input.lines().count();
    let mut antennas: HashMap<char, Vec<Antenna>> = HashMap::new();
    raw_input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '.' => (),
            _ => {
                antennas
                    .entry(c)
                    .or_default()
                    .push(Antenna { frequency: c, x, y });
            }
        });
    });

    ((width, height), antennas)
}

fn solve_part1(width: usize, height: usize, antennas: &HashMap<char, Vec<Antenna>>) -> usize {
    let unique_antinode_positions = antennas
        .iter()
        .flat_map(|(frequency, antennas)| {
            let mut antinodes = Vec::new();

            for (i, antenna_a) in antennas.iter().enumerate() {
                for (j, antenna_b) in antennas.iter().enumerate() {
                    if i >= j {
                        continue;
                    }

                    let dx = antenna_b.x as isize - antenna_a.x as isize;
                    let dy = antenna_b.y as isize - antenna_a.y as isize;

                    let x = antenna_a.x as isize - dx;
                    let y = antenna_a.y as isize - dy;
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        antinodes.push(Antinode {
                            frequency: *frequency,
                            x: x as usize,
                            y: y as usize,
                        });
                    }

                    let x = antenna_b.x as isize + dx;
                    let y = antenna_b.y as isize + dy;
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        antinodes.push(Antinode {
                            frequency: *frequency,
                            x: x as usize,
                            y: y as usize,
                        });
                    }
                }
            }

            antinodes
        })
        .map(|antinode| (antinode.x, antinode.y))
        .collect::<HashSet<_>>();

    unique_antinode_positions.len()
}

fn solve_part2(width: usize, height: usize, antennas: &HashMap<char, Vec<Antenna>>) -> usize {
    let unique_antinode_positions = antennas
        .iter()
        .flat_map(|(frequency, antennas)| {
            let mut antinodes = Vec::new();

            for (i, antenna_a) in antennas.iter().enumerate() {
                for (j, antenna_b) in antennas.iter().enumerate() {
                    if i >= j {
                        continue;
                    }

                    let dx = antenna_b.x as isize - antenna_a.x as isize;
                    let dy = antenna_b.y as isize - antenna_a.y as isize;

                    let mut x = antenna_b.x as isize - dx;
                    let mut y = antenna_b.y as isize - dy;
                    while x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        antinodes.push(Antinode {
                            frequency: *frequency,
                            x: x as usize,
                            y: y as usize,
                        });

                        x -= dx;
                        y -= dy;
                    }

                    let mut x = antenna_a.x as isize + dx;
                    let mut y = antenna_a.y as isize + dy;
                    while x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        antinodes.push(Antinode {
                            frequency: *frequency,
                            x: x as usize,
                            y: y as usize,
                        });

                        x += dx;
                        y += dy;
                    }
                }
            }

            antinodes
        })
        .map(|antinode| (antinode.x, antinode.y))
        .collect::<HashSet<_>>();

    unique_antinode_positions.len()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let ((width, height), antennas) = parse(filename);
    let answer_part1 = solve_part1(width, height, &antennas);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(width, height, &antennas);
    println!("{}", answer_part2);
}
