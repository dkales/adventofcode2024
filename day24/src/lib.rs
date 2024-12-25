use std::collections::VecDeque;

use aoc_traits::AdventOfCodeDay;
use rustc_hash::FxHashMap;

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

    fn execute(&self) -> FxHashMap<&'a str, bool> {
        let mut state = self.inputs.clone();
        let mut gates = VecDeque::from(self.gates.clone());

        while let Some(gate) = gates.pop_front() {
            let input1 = state.get(gate.inputs.0);
            let input2 = state.get(gate.inputs.1);
            match (input1, input2) {
                (Some(&input1), Some(&input2)) => {
                    dbg!(&gate);
                    let output = match gate.operation {
                        Operation::And => input1 & input2,
                        Operation::Or => input1 | input2,
                        Operation::Xor => input1 ^ input2,
                    };
                    state.insert(gate.output, output);
                }
                _ => {
                    gates.push_back(gate);
                }
            }
        }
        state
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
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        Program::parse(input)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let state = input.execute();
        dbg!(&state);
        let mut out = 0u64;
        for i in 0..64 {
            if let Some(&value) = state.get(format!("z{i:02}").as_str()) {
                dbg!(i, value);
                out |= (value as u64) << i;
            }
        }
        out
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        0
    }
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
