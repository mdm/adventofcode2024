use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    path::Path,
    str::FromStr,
};

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone)]
struct Robot {
    position: Vec2,
    velocity: Vec2,
}

impl Robot {
    fn run(&mut self, steps: i64, width: i64, height: i64) {
        self.position.x = (self.position.x + steps * self.velocity.x).rem_euclid(width);
        self.position.y = (self.position.y + steps * self.velocity.y).rem_euclid(height);
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();
        let captures = re.captures(s).ok_or(())?;
        let x = captures[1].parse().unwrap();
        let y = captures[2].parse().unwrap();
        let position = Vec2 { x, y };
        let x = captures[3].parse().unwrap();
        let y = captures[4].parse().unwrap();
        let velocity = Vec2 { x, y };

        Ok(Self { position, velocity })
    }
}

fn parse<P>(filename: P) -> Vec<Robot>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let robots = raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    robots
}

fn print(robots: &[Robot], width: i64, height: i64) {
    let mut counts = HashMap::new();
    for robot in robots {
        *counts.entry(&robot.position).or_insert(0) += 1;
    }

    for y in 0..height {
        for x in 0..width {
            match counts.get(&Vec2 { x, y }) {
                Some(count) => {
                    print!("{}", count % 10);
                }
                None => {
                    print!(".");
                }
            }
        }
        println!();
    }
}

fn solve_part1(robots: &[Robot], width: i64, height: i64) -> usize {
    let robots = robots.to_vec();
    let mut top_left = 0;
    let mut top_right = 0;
    let mut bottom_left = 0;
    let mut bottom_right = 0;
    robots.into_iter().for_each(|mut robot| {
        robot.run(100, width, height);
        let x = robot.position.x;
        let y = robot.position.y;

        if x < width / 2 && y < height / 2 {
            top_left += 1;
        }

        if x > width / 2 && y < height / 2 {
            top_right += 1;
        }

        if x < width / 2 && y > height / 2 {
            bottom_left += 1;
        }

        if x > width / 2 && y > height / 2 {
            bottom_right += 1;
        }
    });

    [top_left, top_right, bottom_left, bottom_right]
        .iter()
        .product()
}

fn detect_tree(robots: &[Robot], width: i64, height: i64) -> bool {
    let mut positions = HashSet::new();
    for robot in robots {
        positions.insert((robot.position.x, robot.position.y));
    }

    let mut longest_run = 0;
    for y in 0..height {
        for x in 0..width {
            let mut run = 0;
            while positions.contains(&(x + run, y)) {
                run += 1;
            }

            if run > longest_run {
                longest_run = run;
            }
        }
    }

    longest_run > 10
}

fn solve_part2(robots: &[Robot], width: i64, height: i64) -> usize {
    let mut robots = robots.to_vec();
    let mut steps = 1;
    loop {
        robots = robots
            .into_iter()
            .map(|mut robot| {
                robot.run(1, width, height);
                robot
            })
            .collect();

        if detect_tree(&robots, width, height) {
            break;
        }

        steps += 1;
    }

    print(&robots, width, height);

    steps
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let robots = parse(filename);
    let width = args()
        .nth(2)
        .expect("Missing width argument")
        .parse()
        .expect("Failed to parse width");
    let height = args()
        .nth(3)
        .expect("Missing height argument")
        .parse()
        .expect("Failed to parse height");
    let answer_part1 = solve_part1(&robots, width, height);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&robots, width, height);
    println!("{}", answer_part2);
}
