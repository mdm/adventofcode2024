use std::{
    collections::{HashMap, HashSet},
    env::args,
    fs::{self, read_to_string},
    path::Path,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum GateKind {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Gate {
    kind: GateKind,
    inputs: Vec<String>,
    output: String,
}

fn parse<P>(filename: P) -> (HashMap<String, bool>, Vec<Gate>)
where
    P: AsRef<Path>,
{
    let raw_input = read_to_string(filename).expect("Failed to read input file");

    let (inputs, gates) = raw_input.split_once("\n\n").unwrap();

    let inputs = inputs
        .lines()
        .map(|line| {
            let (input, value) = line.split_once(": ").unwrap();
            let value = match value {
                "0" => false,
                "1" => true,
                _ => panic!("Invalid input value"),
            };

            (input.to_string(), value)
        })
        .collect();

    let gates = gates
        .lines()
        .map(|line| {
            let parts = line.split_ascii_whitespace().collect::<Vec<_>>();
            let kind = match parts[1] {
                "AND" => GateKind::And,
                "OR" => GateKind::Or,
                "XOR" => GateKind::Xor,
                _ => panic!("Invalid gate kind"),
            };
            let inputs = vec![parts[0].to_string(), parts[2].to_string()];
            let output = parts[4].to_string();

            Gate {
                kind,
                inputs,
                output,
            }
        })
        .collect();

    (inputs, gates)
}

fn run_gates(inputs: &HashMap<String, bool>, gates: &[Gate]) -> HashMap<String, bool> {
    let mut inputs = inputs.clone();
    let gates = gates.to_vec();

    let mut ready = 0;
    while ready < gates.len() {
        ready = 0;

        for gate in &gates {
            if gate.inputs.iter().all(|input| inputs.contains_key(input)) {
                let values = gate
                    .inputs
                    .iter()
                    .map(|input| inputs[input])
                    .collect::<Vec<_>>();
                let output = match gate.kind {
                    GateKind::And => values.into_iter().reduce(|a, b| a && b).unwrap(),
                    GateKind::Or => values.into_iter().reduce(|a, b| a || b).unwrap(),
                    GateKind::Xor => values
                        .into_iter()
                        .reduce(|a, b| (a || b) && !(a && b))
                        .unwrap(),
                };
                inputs.insert(gate.output.clone(), output);
                ready += 1;
                // } else {
                //     let missing_inputs = gate
                //         .inputs
                //         .iter()
                //         .filter(|input| !inputs.contains_key(*input))
                //         .collect::<Vec<_>>();
                //     dbg!(missing_inputs);
            }
        }
    }

    inputs
}

fn get_value(outputs: &HashMap<String, bool>, prefix: &str) -> usize {
    let mut keys = outputs
        .keys()
        .filter(|output| output.starts_with(prefix))
        .collect::<Vec<_>>();

    keys.sort_by(|a, b| b.cmp(a));

    keys.iter().fold(0, |acc, key| {
        let value = if outputs[*key] { 1 } else { 0 };

        (acc << 1) + value
    })
}

fn solve_part1(inputs: &HashMap<String, bool>, gates: &[Gate]) -> usize {
    let outputs = run_gates(inputs, gates);

    get_value(&outputs, "z")
}

fn find_closure(gates: &[Gate], outputs: &[String], allow_or: bool) -> HashSet<Gate> {
    let mut closure = HashSet::new();
    let mut allow_or = allow_or;
    // dbg!(outputs, allow_or);

    for gate in gates {
        if !outputs.contains(&gate.output) {
            continue;
        } else {
            if gate.kind == GateKind::Or {
                if allow_or {
                    allow_or = false;
                } else {
                    continue;
                }
            }

            closure.insert(gate.clone());

            let subclosure = find_closure(gates, &gate.inputs, allow_or);
            closure.extend(subclosure);
        }
    }

    closure
}

fn find_carry_inputs(gates: &[Gate]) -> Vec<String> {
    let inputs = gates
        .iter()
        .flat_map(|gate| gate.inputs.iter().cloned())
        .collect::<HashSet<_>>();

    let outputs = gates
        .iter()
        .map(|gate| gate.output.clone())
        .collect::<HashSet<_>>();

    inputs
        .difference(&outputs)
        .filter(|input| !(input.starts_with("x") || input.starts_with("y")))
        .cloned()
        .collect()
}

fn visualize(gates: &[Gate]) {
    let mut values = HashSet::new();
    values.extend(gates.iter().flat_map(|gate| gate.inputs.iter().cloned()));
    values.extend(gates.iter().map(|gate| gate.output.clone()));

    let mut dot = "digraph day24 {\n".to_string();
    for value in values {
        dot.push_str(&format!("  {} [shape=circle];\n", value));
    }
    for (i, gate) in gates.iter().enumerate() {
        match gate.kind {
            GateKind::And => dot.push_str(&format!("  gate{} [shape=box, label=\"AND\"];\n", i)),
            GateKind::Or => dot.push_str(&format!("  gate{} [shape=box, label=\"OR\"];\n", i)),
            GateKind::Xor => dot.push_str(&format!("  gate{} [shape=box, label=\"XOR\"];\n", i)),
        }
        for input in &gate.inputs {
            dot.push_str(&format!("  {} -> gate{};\n", input, i));
        }
        dot.push_str(&format!("  gate{} -> {};\n", i, gate.output));
    }
    dot.push_str("}\n");

    fs::write("day24.dot", dot).expect("Failed to write dot file");
}

fn solve_part2(_inputs: &HashMap<String, bool>, gates: &[Gate]) -> String {
    visualize(gates);
    let result = find_closure(gates, &["z44".into()], false);
    let carry = find_closure(gates, &["z45".into()], true);
    let mut adder = HashSet::new();
    adder.extend(result);
    adder.extend(carry);
    dbg!(&adder);
    dbg!(find_carry_inputs(
        &adder.iter().cloned().collect::<Vec<_>>()
    ));

    "TODO".to_string()
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (inputs, gates) = parse(filename);
    let answer_part1 = solve_part1(&inputs, &gates);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&inputs, &gates);
    println!("{}", answer_part2);
}
