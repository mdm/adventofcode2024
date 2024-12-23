use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::read_to_string,
    path::Path,
};

fn parse<P>(filename: P) -> HashMap<String, Vec<String>>
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let mut network = HashMap::new();
    raw_input.lines().for_each(|line| {
        let (from, to) = line.split_once("-").unwrap();

        network
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());

        network
            .entry(to.to_string())
            .or_insert_with(Vec::new)
            .push(from.to_string());
    });

    network
}

fn find_connected_sets(network: &HashMap<String, Vec<String>>) -> Vec<HashSet<String>> {
    let mut connected_sets = network
        .keys()
        .cloned()
        .map(|computer| [computer].into_iter().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let mut modified = true;
    while modified {
        modified = false;

        for computer in network.keys() {
            for connected_set in connected_sets.iter_mut() {
                if connected_set.contains(computer) {
                    continue;
                }

                if connected_set
                    .iter()
                    .all(|other| network[computer].contains(other))
                {
                    connected_set.insert(computer.clone());
                    modified = true;
                }
            }
        }
    }

    connected_sets
}

fn solve_part1(network: &HashMap<String, Vec<String>>) -> usize {
    let mut computers = network.keys().cloned().collect::<Vec<_>>();
    computers.sort_unstable();

    let mut count = 0;
    for (i, computer_a) in computers.iter().enumerate() {
        for (j, computer_b) in computers.iter().enumerate() {
            if j >= i {
                continue;
            }
            for (k, computer_c) in computers.iter().enumerate() {
                if k >= j {
                    continue;
                }

                let connected_triple = network[computer_a].contains(computer_b)
                    && network[computer_b].contains(computer_c)
                    && network[computer_c].contains(computer_a);

                if !connected_triple {
                    continue;
                }

                let candidate = computer_a.starts_with("t")
                    || computer_b.starts_with("t")
                    || computer_c.starts_with("t");

                if candidate {
                    count += 1;
                }
            }
        }
    }

    count
}

fn solve_part2(network: &HashMap<String, Vec<String>>) -> String {
    let connected_sets = find_connected_sets(network);

    let mut lan_party = connected_sets
        .iter()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    lan_party.sort_unstable();

    lan_party.join(",")
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let network = parse(filename);
    let answer_part1 = solve_part1(&network);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&network);
    println!("{}", answer_part2);
}
