use core::panic;
use std::collections::VecDeque;

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct Program<'a> {
    inputs: FxHashMap<&'a str, bool>,
    gates: Vec<Gate<'a>>,
}

impl<'a> Program<'a> {
    fn parse(input: &'a str) -> Program<'a> {
        let (inputs, gates) = input
            .split_once("\n\n")
            .expect("input has gates and inputs");

        let inputs: FxHashMap<&'a str, bool> = inputs
            .lines()
            .map(|x| x.split_once(": ").unwrap())
            .map(|(k, v)| (k, v.parse::<u8>().unwrap() == 1))
            .collect();
        let gates = gates.lines().map(|x| Gate::parse(x)).collect();

        Program { inputs, gates }
    }

    fn execute(&self, inputs: &FxHashMap<&'a str, bool>) -> Option<FxHashMap<&'a str, bool>> {
        let mut state = inputs.clone();
        let mut gates = VecDeque::from(self.gates.clone());

        let mut no_ops = 0;
        while let Some(gate) = gates.pop_front() {
            let input1 = state.get(gate.inputs.0);
            let input2 = state.get(gate.inputs.1);
            match (input1, input2) {
                (Some(&input1), Some(&input2)) => {
                    let output = match gate.operation {
                        Operation::And => input1 & input2,
                        Operation::Or => input1 | input2,
                        Operation::Xor => input1 ^ input2,
                    };
                    state.insert(gate.output, output);
                    no_ops = 0;
                }
                _ => {
                    gates.push_back(gate);
                    no_ops += 1;
                    if no_ops == gates.len() {
                        // no more operations can be performed
                        return None;
                    }
                }
            }
        }
        Some(state)
    }
    fn execute_u64(&self, inputs: (u64, u64)) -> Option<u64> {
        let mut state = self.inputs.clone();
        for i in 0..64 {
            let x = format!("x{i:02}");
            state
                .get_mut(x.as_str())
                .map(|v| *v = (inputs.0 >> i) & 1 == 1);
            let y = format!("y{i:02}");
            state
                .get_mut(y.as_str())
                .map(|v| *v = (inputs.1 >> i) & 1 == 1);
        }

        let out = self.execute(&state);
        out.map(|state| {
            let mut out = 0u64;
            for i in 0..64 {
                if let Some(&value) = state.get(format!("z{i:02}").as_str()) {
                    out |= (value as u64) << i;
                }
            }
            out
        })
    }
}

#[derive(Debug, Clone)]
pub struct Gate<'a> {
    inputs: (&'a str, &'a str),
    output: &'a str,
    operation: Operation,
}

impl<'a> Gate<'a> {
    fn parse(input: &'a str) -> Gate<'a> {
        let (inputs, output) = input.split_once(" -> ").unwrap();
        let mut words = inputs.split_ascii_whitespace();
        let input1 = words.next().unwrap();
        let operation = match words.next().unwrap() {
            "AND" => Operation::And,
            "OR" => Operation::Or,
            "XOR" => Operation::Xor,
            _ => panic!("Invalid operation"),
        };
        let input2 = words.next().unwrap();

        Gate {
            inputs: (input1, input2),
            output,
            operation,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Operation {
    And,
    Or,
    Xor,
}

pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Program<'a>;
    type Part1Output = u64;
    type Part2Output = String;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        Program::parse(input)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let state = input.execute(&input.inputs).expect("can solve");
        let mut out = 0u64;
        for i in 0..64 {
            if let Some(&value) = state.get(format!("z{i:02}").as_str()) {
                out |= (value as u64) << i;
            }
        }
        out
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let num_bits = input.inputs.keys().filter(|k| k.starts_with('x')).count();

        let mut inputs = input.inputs.clone();
        for v in inputs.values_mut() {
            *v = false;
        }
        let mut game = input.to_owned();
        let mut good_outputs = FxHashSet::default();
        let mut potential_swaps = vec![Vec::new(); num_bits];
        for i in 0..num_bits {
            *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = true;
            let output = game.execute(&inputs).expect("can solve");
            let mut wrong_gates = Vec::new();
            for (k, v) in output.iter().filter(|(k, _)| k.starts_with('z')) {
                if *v != (k == &format!("z{:02}", i).as_str()) {
                    // found potentially wrong output gate
                    wrong_gates.push(*k);
                }
            }
            // set bit i to 1 for x,y
            *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = false;
            *inputs.get_mut(format!("y{i:02}").as_str()).unwrap() = true;
            let output = game.execute(&inputs).expect("can solve");
            let mut wrong_gates = Vec::new();
            for (k, v) in output.iter().filter(|(k, _)| k.starts_with('z')) {
                if *v != (k == &format!("z{:02}", i).as_str()) {
                    // found potentially wrong output gate
                    wrong_gates.push(*k);
                }
            }
            *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = true;
            *inputs.get_mut(format!("y{i:02}").as_str()).unwrap() = true;
            let output = game.execute(&inputs).expect("can solve");
            let mut wrong_gates = Vec::new();
            for (k, v) in output.iter().filter(|(k, _)| k.starts_with('z')) {
                if *v != (k == &format!("z{:02}", i + 1).as_str()) {
                    // found potentially wrong output gate
                    wrong_gates.push(*k);
                }
            }
            if !wrong_gates.is_empty() {
                let influenced_gates =
                    find_gates_influencing(&input.gates, &wrong_gates, &good_outputs);
                // try all possible gate swaps and check if the output is correct
                for (&a, &b) in influenced_gates.iter().tuple_combinations() {
                    for gate in game.gates.iter_mut() {
                        if gate.output == a {
                            gate.output = b;
                        } else if gate.output == b {
                            gate.output = a;
                        }
                    }
                    let mut wrong_gates = Vec::new();
                    *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = false;
                    *inputs.get_mut(format!("y{i:02}").as_str()).unwrap() = true;
                    let output = game.execute(&inputs);
                    if let Some(output) = output {
                        for (k, v) in output.iter().filter(|(k, _)| k.starts_with('z')) {
                            if *v != (k == &format!("z{:02}", i).as_str()) {
                                // found potentially wrong output gate
                                wrong_gates.push(*k);
                            }
                        }
                    } else {
                        // fail, swap back
                        for gate in game.gates.iter_mut() {
                            if gate.output == a {
                                gate.output = b;
                            } else if gate.output == b {
                                gate.output = a;
                            }
                        }
                        continue;
                    }
                    *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = true;
                    *inputs.get_mut(format!("y{i:02}").as_str()).unwrap() = false;
                    let output = game.execute(&inputs);
                    if let Some(output) = output {
                        for (k, v) in output.iter().filter(|(k, _)| k.starts_with('z')) {
                            if *v != (k == &format!("z{:02}", i).as_str()) {
                                // found potentially wrong output gate
                                wrong_gates.push(*k);
                            }
                        }
                    } else {
                        // fail, swap back
                        for gate in game.gates.iter_mut() {
                            if gate.output == a {
                                gate.output = b;
                            } else if gate.output == b {
                                gate.output = a;
                            }
                        }
                        continue;
                    }
                    *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = true;
                    *inputs.get_mut(format!("y{i:02}").as_str()).unwrap() = true;
                    let output = game.execute(&inputs);
                    if let Some(output) = output {
                        for (k, v) in output.iter().filter(|(k, _)| k.starts_with('z')) {
                            if *v != (k == &format!("z{:02}", i + 1).as_str()) {
                                // found potentially wrong output gate
                                wrong_gates.push(*k);
                            }
                        }
                    } else {
                        // fail, swap back
                        for gate in game.gates.iter_mut() {
                            if gate.output == a {
                                gate.output = b;
                            } else if gate.output == b {
                                gate.output = a;
                            }
                        }
                        continue;
                    }
                    if wrong_gates.is_empty() {
                        // found the correct swap
                        potential_swaps[i].push((a, b));
                    }
                    //  swap back
                    for gate in game.gates.iter_mut() {
                        if gate.output == a {
                            gate.output = b;
                        } else if gate.output == b {
                            gate.output = a;
                        }
                    }
                }
            } else {
                let good = format!("z{:02}", i);
                let influenced_gates =
                    find_gates_influencing(&input.gates, &[good.as_str()], &good_outputs);
                good_outputs.extend(influenced_gates.into_iter());
            }

            // reset inputs
            *inputs.get_mut(format!("x{i:02}").as_str()).unwrap() = false;
            *inputs.get_mut(format!("y{i:02}").as_str()).unwrap() = false;
        }
        // filter potential swaps a bit
        let mut swap_groups = Vec::new();
        for i in 0..num_bits - 1 {
            if potential_swaps[i].is_empty() {
                continue;
            }
            if potential_swaps[i + 1].is_empty() {
                swap_groups.push(potential_swaps[i].clone());
            }
        }
        assert!(swap_groups.len() == 4);
        for swap0 in &swap_groups[0] {
            for swap1 in &swap_groups[1] {
                for swap2 in &swap_groups[2] {
                    'outer: for swap3 in &swap_groups[3] {
                        let mut game = input.to_owned();
                        for gate in game.gates.iter_mut() {
                            if gate.output == swap0.0 {
                                gate.output = swap0.1;
                            } else if gate.output == swap0.1 {
                                gate.output = swap0.0;
                            } else if gate.output == swap1.0 {
                                gate.output = swap1.1;
                            } else if gate.output == swap1.1 {
                                gate.output = swap1.0;
                            } else if gate.output == swap2.0 {
                                gate.output = swap2.1;
                            } else if gate.output == swap2.1 {
                                gate.output = swap2.0;
                            } else if gate.output == swap3.0 {
                                gate.output = swap3.1;
                            } else if gate.output == swap3.1 {
                                gate.output = swap3.0;
                            }
                        }
                        // 200 reps should be good enough to weed out false positives
                        for _ in 0..200 {
                            let x = rand::random::<u64>() % (1 << num_bits);
                            let y = rand::random::<u64>() % (1 << num_bits);
                            let z = x + y;

                            if let Some(output) = game.execute_u64((x, y)) {
                                if output != z {
                                    continue 'outer;
                                }
                            } else {
                                continue 'outer;
                            }
                        }
                        return [
                            swap0.0, swap0.1, swap1.0, swap1.1, swap2.0, swap2.1, swap3.0, swap3.1,
                        ]
                        .iter()
                        .sorted()
                        .join(",");
                    }
                }
            }
        }

        panic!("No solution found");
    }
}

fn find_gates_influencing<'a, 'b>(
    gates: &[Gate<'a>],
    outputs: &[&'b str],
    good: &FxHashSet<&'a str>,
) -> FxHashSet<&'a str> {
    let mut influenced_gates = FxHashSet::default();
    let mut wrong_gates = outputs.to_vec();
    while let Some(wrong) = wrong_gates.pop() {
        for gate in gates {
            if gate.output == wrong && !good.contains(gate.output) {
                influenced_gates.insert(gate.output);
                wrong_gates.push(gate.inputs.0);
                wrong_gates.push(gate.inputs.1);
                break;
            }
        }
    }
    influenced_gates
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;

    const TEST_INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 2024);
    }
}
