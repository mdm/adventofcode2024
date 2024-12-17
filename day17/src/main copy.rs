use std::{collections::VecDeque, env::args, fs::read_to_string, path::Path, str::FromStr};

#[derive(Debug, Clone)]
struct Computer {
    ip: usize,
    registers: [u128; 3],
}

impl Computer {
    fn new() -> Self {
        Self {
            ip: 0,
            registers: [0; 3],
        }
    }

    fn execute(&mut self, program: &[u128]) -> Vec<u128> {
        let mut output = Vec::new();

        while self.ip < program.len() {
            let opcode = program[self.ip];
            let operand = program[self.ip + 1];

            // dbg!(&self, opcode, operand);
            match opcode {
                0 => {
                    // adv
                    let numerator = self.registers[0];
                    let denominator = 2_u128.pow(self.read_combo(operand) as u32);
                    self.registers[0] = numerator / denominator;

                    self.ip += 2;
                }
                1 => {
                    // bxl
                    self.registers[1] ^= operand;

                    self.ip += 2;
                }
                2 => {
                    // bst
                    self.registers[1] = self.read_combo(operand) % 8;

                    self.ip += 2;
                }
                3 => {
                    // jnz
                    if self.registers[0] != 0 {
                        self.ip = operand as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                4 => {
                    // bxc
                    self.registers[1] ^= self.registers[2];

                    self.ip += 2;
                }
                5 => {
                    // out
                    let value = self.read_combo(operand) % 8;

                    output.push(value);

                    self.ip += 2;
                }
                6 => {
                    let numerator = self.registers[0];
                    let denominator = 2_u128.pow(self.read_combo(operand) as u32);
                    self.registers[1] = numerator / denominator;

                    self.ip += 2;
                }
                7 => {
                    let numerator = self.registers[0];
                    let denominator = 2_u128.pow(self.read_combo(operand) as u32);
                    self.registers[2] = numerator / denominator;

                    self.ip += 2;
                }
                _ => panic!("Invalid opcode"),
            }
        }

        output
    }

    fn read_combo(&self, operand: u128) -> u128 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            _ => panic!("Invalid operand"),
        }
    }
}

impl FromStr for Computer {
    type Err = ();

    fn from_str(config: &str) -> Result<Self, Self::Err> {
        let registers = config
            .lines()
            .map(|line| {
                let init_value = line.split_ascii_whitespace().last().unwrap();
                init_value.parse::<u128>().unwrap()
            })
            .collect::<Vec<u128>>()
            .try_into()
            .unwrap();
        Ok(Self { ip: 0, registers })
    }
}

fn parse<P>(filename: P) -> (Computer, Vec<u128>)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let (config, program) = raw_input.split_once("\n\n").unwrap();

    let computer = config.parse::<Computer>().unwrap();
    let program = program
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|item| item.parse::<u128>().unwrap())
        .collect();

    (computer, program)
}

fn print_program(program: &[u128]) {
    for (i, &opcode) in program.iter().enumerate() {
        if i % 2 != 0 {
            continue;
        }

        let operand = program[i + 1];
        let combo = match operand {
            0..=3 => program[i + 1].to_string(),
            4 => "A".to_string(),
            5 => "B".to_string(),
            6 => "C".to_string(),
            _ => panic!("Invalid operand"),
        };

        match opcode {
            0 => {
                println!("{}: adv A, 2^{}", i, combo);
            }
            1 => {
                println!("{}: bxl {}", i, operand);
            }
            2 => {
                println!("{}: bst {} % 8", i, combo);
            }
            3 => {
                println!("{}: jnz {}", i, operand);
            }
            4 => {
                println!("{}: bxc", i);
            }
            5 => {
                println!("{}: out {} % 8", i, combo);
            }
            6 => {
                println!("{}: bdv A, 2^{}", i, combo);
            }
            7 => {
                println!("{}: cdv A, 2^{}", i, combo);
            }
            _ => panic!("Invalid opcode"),
        }
    }
}

fn solve_part1(computer: &Computer, program: &[u128]) -> String {
    let mut computer = computer.clone();
    let output = computer.execute(program);
    // dbg!(&output);
    output
        .iter()
        .map(|&value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

fn find_quine(program: &[u128], register_a: u128, increment_base: usize) -> Option<u128> {
    let mut register_a = register_a;

    let mut results = Vec::new();
    for _ in 0..250 {
        if register_a > 8_u128.pow(program.len() as u32) {
            break;
        }

        let mut computer = Computer::new();
        computer.registers[0] = register_a;
        let output = computer.execute(program);

        let matching = program
            .iter()
            .zip(output.iter())
            .take_while(|(a, b)| a == b)
            .count();

        if matching == program.len() && output.len() == program.len() {
            results.push(register_a);
        }

        if matching > increment_base {
            // dbg!(increment_base, program, &output);
            if let Some(result) = find_quine(program, register_a, matching) {
                results.push(result);
            }
        }

        register_a += 8_u128.pow(increment_base as u32);
    }

    results.iter().min().copied()
}

fn solve_part2(original_computer: &Computer, program: &[u128]) -> usize {
    print_program(program);
    // return 0;

    println!("Finding quine");
    dbg!(find_quine(program, 0, 0));
    println!("Quine found");
    return 0;

    // let mut register_a = 8_u128.pow(program.len() as u32 - 1);
    let mut register_a = 0;
    let mut increment_base = 0;

    loop {
        // dbg!(register_a, 1_u128 << 63);
        let mut computer = original_computer.clone();
        // computer.registers[0] = 117440;
        computer.registers[0] = register_a;
        let output = computer.execute(program);

        // if computer.ip >= program.len() {
        //     println!("done");
        // }

        let quine = program
            .iter()
            .zip(output.iter().take(increment_base + 1))
            .all(|(a, b)| a == b);
        // dbg!(quine, register_a, increment_base, output.len());
        // return register_a as usize;

        if quine && output.len() > increment_base {
            increment_base = output.len();
            dbg!(increment_base, program, &output);
        }

        if quine && program.len() == output.len() {
            dbg!(&program, &output, register_a);
            return register_a as usize;
        }

        register_a += 8_u128.pow(increment_base as u32);
    }
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (computer, program) = parse(filename);
    // dbg!(&computer, &program);
    let answer_part1 = solve_part1(&computer, &program);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&computer, &program);
    println!("{}", answer_part2);
}
