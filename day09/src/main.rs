use std::{collections::HashSet, env::args, fs::read_to_string, path::Path};

#[derive(Debug, Clone)]
enum DiskMapBlock {
    Free { len: usize },
    File { id: usize, len: usize },
}

#[derive(Debug, Clone)]
struct DiskMap {
    blocks: Vec<DiskMapBlock>,
}

impl DiskMap {
    fn first_free_pos(&self, min_size: usize) -> Option<usize> {
        self.blocks
            .iter()
            .enumerate()
            .find_map(|(i, block)| match block {
                DiskMapBlock::Free { len } => {
                    if *len < min_size {
                        None
                    } else {
                        Some(i)
                    }
                }
                _ => None,
            })
    }

    fn last_file_pos(&self, exclude: &HashSet<usize>) -> Option<usize> {
        self.blocks
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, block)| match block {
                DiskMapBlock::File { id, .. } => {
                    if exclude.contains(id) {
                        None
                    } else {
                        Some(i)
                    }
                }
                _ => None,
            })
    }

    fn move_fragments(&mut self) {
        let first_free_pos = self.first_free_pos(0).unwrap();
        let exclude = HashSet::new();
        let last_file_pos = self.last_file_pos(&exclude).unwrap();

        let first_free_len = match self.blocks[first_free_pos] {
            DiskMapBlock::Free { len } => len,
            _ => unreachable!(),
        };
        let (last_file_id, last_file_len) = match self.blocks[last_file_pos] {
            DiskMapBlock::File { id, len } => (id, len),
            _ => unreachable!(),
        };

        self.blocks = self
            .blocks
            .iter()
            .enumerate()
            .flat_map(|(i, block)| {
                if i == first_free_pos {
                    if last_file_len < first_free_len {
                        vec![
                            DiskMapBlock::File {
                                id: last_file_id,
                                len: last_file_len,
                            },
                            DiskMapBlock::Free {
                                len: first_free_len - last_file_len,
                            },
                        ]
                    } else {
                        vec![DiskMapBlock::File {
                            id: last_file_id,
                            len: first_free_len,
                        }]
                    }
                } else if i == last_file_pos {
                    if last_file_len > first_free_len {
                        vec![
                            DiskMapBlock::Free {
                                len: first_free_len,
                            },
                            DiskMapBlock::File {
                                id: last_file_id,
                                len: last_file_len - first_free_len,
                            },
                        ]
                    } else {
                        vec![DiskMapBlock::Free { len: last_file_len }]
                    }
                } else {
                    vec![block.clone()]
                }
            })
            .collect();
    }

    fn move_whole(&mut self, move_file_pos: usize) -> bool {
        let (move_file_id, move_file_len) = match self.blocks[move_file_pos] {
            DiskMapBlock::File { id, len } => (id, len),
            _ => unreachable!(),
        };

        let Some(first_free_pos) = self.first_free_pos(move_file_len) else {
            return false;
        };

        if move_file_pos < first_free_pos {
            return false;
        }

        let first_free_len = match self.blocks[first_free_pos] {
            DiskMapBlock::Free { len } => len,
            _ => unreachable!(),
        };

        self.blocks = self
            .blocks
            .iter()
            .enumerate()
            .flat_map(|(i, block)| {
                if i == first_free_pos {
                    if first_free_len == move_file_len {
                        vec![DiskMapBlock::File {
                            id: move_file_id,
                            len: move_file_len,
                        }]
                    } else {
                        vec![
                            DiskMapBlock::File {
                                id: move_file_id,
                                len: move_file_len,
                            },
                            DiskMapBlock::Free {
                                len: first_free_len - move_file_len,
                            },
                        ]
                    }
                } else if i == move_file_pos {
                    vec![DiskMapBlock::Free { len: move_file_len }]
                } else {
                    vec![block.clone()]
                }
            })
            .collect();

        true
    }

    fn checksum(&self) -> usize {
        let mut index = 0;
        self.blocks
            .iter()
            .map(|block| match block {
                DiskMapBlock::Free { len } => {
                    index += len;
                    0
                }
                DiskMapBlock::File { id, len } => {
                    let mut result = 0;
                    for i in index..index + len {
                        result += i * id
                    }
                    index += len;
                    result
                }
            })
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for block in &self.blocks {
            match block {
                DiskMapBlock::Free { len } => {
                    for _i in 0..*len {
                        print!(".");
                    }
                }
                DiskMapBlock::File { id, len } => {
                    for _i in 0..*len {
                        print!("{}", id);
                    }
                }
            }
        }
        println!();
    }
}

fn parse<P>(filename: P) -> DiskMap
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let disk_map = raw_input.lines().next().unwrap();
    let blocks = disk_map
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let len = c.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                let id = i / 2;
                DiskMapBlock::File { id, len }
            } else {
                DiskMapBlock::Free { len }
            }
        })
        .collect();

    DiskMap { blocks }
}

fn solve_part1(disk_map: &DiskMap) -> usize {
    let mut disk_map = disk_map.clone();
    let exclude = HashSet::new();
    while disk_map.first_free_pos(0).unwrap() < disk_map.last_file_pos(&exclude).unwrap() {
        disk_map.move_fragments();
    }

    disk_map.checksum()
}

fn solve_part2(disk_map: &DiskMap) -> usize {
    let mut disk_map = disk_map.clone();
    let mut exclude = HashSet::new();
    while let Some(file_pos) = disk_map.last_file_pos(&exclude) {
        let file_id = match disk_map.blocks[file_pos] {
            DiskMapBlock::File { id, .. } => id,
            _ => unreachable!(),
        };

        disk_map.move_whole(file_pos);

        exclude.insert(file_id);
    }

    disk_map.checksum()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let disk_map = parse(filename);
    let answer_part1 = solve_part1(&disk_map);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&disk_map);
    println!("{}", answer_part2);
}
