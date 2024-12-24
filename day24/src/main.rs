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

fn run_gates(inputs: &HashMap<String, bool>, gates: &[Gate]) -> Option<HashMap<String, bool>> {
    let mut inputs = inputs.clone();
    let gates = gates.to_vec();

    let mut ready = 0;
    let mut modified = true;
    while modified && ready < gates.len() {
        ready = 0;
        modified = false;

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
                modified = true;
            }
        }
    }

    if modified {
        Some(inputs)
    } else {
        None
    }
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
    let outputs = run_gates(inputs, gates).unwrap();

    get_value(&outputs, "z")
}

fn visualize(gates: &[Gate], basename: &str) {
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

    fs::write(format!("{}.dot", basename), dot).expect("Failed to write dot file");
}

fn find_closure(gates: &[Gate], outputs: &[String], allow_or: bool) -> HashSet<Gate> {
    let mut closure = HashSet::new();
    let mut allow_or = allow_or;

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

fn find_carry_inputs(gates: &[Gate]) -> String {
    let inputs = gates
        .iter()
        .flat_map(|gate| gate.inputs.iter().cloned())
        .collect::<HashSet<_>>();

    let outputs = gates
        .iter()
        .map(|gate| gate.output.clone())
        .collect::<HashSet<_>>();

    let mut carries = inputs
        .difference(&outputs)
        .filter(|input| !(input.starts_with("x") || input.starts_with("y")))
        .cloned()
        .collect::<Vec<_>>();

    assert_eq!(carries.len(), 1);
    carries.pop().unwrap()
}

fn verify_adder(gates: &[Gate], inputs: &[String], result: &str, carry: &str) -> bool {
    for value in 0..8 {
        let values = inputs
            .iter()
            .enumerate()
            .map(|(shift, input)| (input.clone(), (value >> shift) & 1 == 1))
            .collect::<HashMap<_, _>>();

        let outputs = run_gates(&values, gates).unwrap();

        let expected = values
            .values()
            .map(|value| if *value { 1 } else { 0 })
            .sum::<usize>();

        if outputs[result] != (expected & 1 == 1) {
            return false;
        }

        if outputs[carry] != (expected & 2 == 2) {
            return false;
        }
    }

    true
}

fn swap_outputs(gates: &mut [Gate], output_a: &str, output_b: &str) {
    let mut gate_a = gates.len();
    let mut gate_b = gates.len();
    for (i, gate) in gates.iter().enumerate() {
        if gate.output == output_a {
            gate_a = i
        }
        if gate.output == output_b {
            gate_b = i;
        }
    }
    let tmp = gates[gate_a].output.clone();
    gates[gate_a].output = gates[gate_b].output.clone();
    gates[gate_b].output = tmp;
}

fn solve_part2(_inputs: &HashMap<String, bool>, gates: &[Gate]) -> String {
    visualize(gates, "day24");

    let mut gates = gates.to_vec();
    swap_outputs(&mut gates, "z21", "nhn");
    swap_outputs(&mut gates, "z12", "vdc");
    swap_outputs(&mut gates, "khg", "tvb");
    swap_outputs(&mut gates, "z33", "gst");
    let mut swapped = ["z21", "nhn", "z12", "vdc", "khg", "tvb", "z33", "gst"];
    swapped.sort();

    let mut carry_output = "z45".to_string();
    for i in (2..45).rev() {
        let x_input = format!("x{:02}", i);
        let y_input = format!("y{:02}", i);
        let result_output = format!("z{:02}", i);

        let result = find_closure(&gates, &[result_output.clone()], false);
        let carry = find_closure(&gates, &[carry_output.clone()], true);
        let mut adder = HashSet::new();
        adder.extend(result);
        adder.extend(carry);
        let adder = adder.iter().cloned().collect::<Vec<_>>();
        let carry_input = find_carry_inputs(&adder);

        if [12, 22, 25, 33, 44].contains(&i) {
            visualize(&adder, &format!("adder{:02}", i));
        }

        let verified = verify_adder(
            &adder,
            &[x_input, y_input, carry_input.clone()],
            &result_output,
            &carry_output,
        );

        if !verified {
            println!("Failed to verify adder {:02}", i);
        }

        carry_output = carry_input;
    }

    swapped.join(",")
}

fn main() {
    let filename = args().nth(1).expect("No input filename provided");
    let (inputs, gates) = parse(filename);
    let answer_part1 = solve_part1(&inputs, &gates);
    println!("{}", answer_part1);
    let answer_part2 = solve_part2(&inputs, &gates);
    println!("{}", answer_part2);
}
