#[derive(Debug)]
pub struct Program {
    registers: [u64; 3],
    inst: Vec<u8>,
}

fn parse_game(input: &str) -> IResult<&str, Program> {
    let (input, _) = tag("Register A: ")(input)?;
    let (input, a) = nom::combinator::map_res(nom::character::complete::digit1, str::parse)(input)?;
    let (input, _) = tag("\nRegister B: ")(input)?;
    let (input, b) = nom::combinator::map_res(nom::character::complete::digit1, str::parse)(input)?;
    let (input, _) = tag("\nRegister C: ")(input)?;
    let (input, c) = nom::combinator::map_res(nom::character::complete::digit1, str::parse)(input)?;
    let (input, _) = tag("\n\nProgram: ")(input)?;
    let (input, inst) = nom::multi::separated_list1(
        tag(","),
        nom::combinator::map_res(nom::character::complete::digit1, str::parse),
    )(input)?;
    Ok((
        input,
        Program {
            registers: [a, b, c],
            inst,
        },
    ))
}

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

impl Program {
    fn execute(&self) -> String {
        let mut registers = self.registers;

        let mut ip = 0;

        #[inline]
        fn combo(registers: [u64; 3], val: u8) -> u64 {
            match val {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => registers[0],
                5 => registers[1],
                7 => registers[2],
                _ => unreachable!(),
            }
        }
        let mut outputs: Vec<u8> = Vec::new();

        while let Some(op) = self.inst.get(ip as usize).copied() {
            match op {
                // adv
                0 => {
                    let val = self.inst[ip + 1];
                    registers[A] = registers[A] >> combo(registers, val);
                }
                // bxl
                1 => {
                    let val = self.inst[ip + 1];
                    registers[B] ^= val as u64;
                }
                // bst
                2 => {
                    let val = self.inst[ip + 1];
                    registers[B] = combo(registers, val) % 8;
                }
                //jnz
                3 => {
                    if registers[0] != 0 {
                        let val = self.inst[ip + 1];
                        ip = val as usize;
                        continue;
                    }
                }
                // bxc
                4 => {
                    registers[B] ^= registers[C];
                }
                // out
                5 => {
                    let val = self.inst[ip + 1];
                    let out = combo(registers, val) % 8;
                    outputs.push(out as u8);
                }
                // bdv
                6 => {
                    let val = self.inst[ip + 1];
                    registers[B] = registers[A] >> combo(registers, val);
                }
                // cdv
                7 => {
                    let val = self.inst[ip + 1];
                    registers[C] = registers[A] >> combo(registers, val);
                }
                _ => unreachable!(),
            }
            ip += 2;
        }
        outputs.into_iter().join(",")
    }
    fn brute_force_part2(&self) -> u64 {
        #[inline]
        fn combo(registers: [u64; 3], val: u8) -> u64 {
            match val {
                0 => 0,
                1 => 1,
                2 => 2,
                3 => 3,
                4 => registers[0],
                5 => registers[1],
                7 => registers[2],
                _ => unreachable!(),
            }
        }
        let want_outputs = self.inst.clone();
        (0..=u64::MAX)
            .into_par_iter()
            .find_map_first(|i| {
                let mut registers = self.registers;
                registers[A] = i;

                let mut ip = 0;
                let mut output_idx = 0;

                while let Some(op) = self.inst.get(ip as usize).copied() {
                    match op {
                        // adv
                        0 => {
                            let val = self.inst[ip + 1];
                            registers[A] = registers[A] >> combo(registers, val);
                        }
                        // bxl
                        1 => {
                            let val = self.inst[ip + 1];
                            registers[B] ^= val as u64;
                        }
                        // bst
                        2 => {
                            let val = self.inst[ip + 1];
                            registers[B] = combo(registers, val) % 8;
                        }
                        //jnz
                        3 => {
                            if registers[0] != 0 {
                                let val = self.inst[ip + 1];
                                ip = val as usize;
                                continue;
                            }
                        }
                        // bxc
                        4 => {
                            registers[B] ^= registers[C];
                        }
                        // out
                        5 => {
                            let val = self.inst[ip + 1];
                            let out = combo(registers, val) % 8;
                            if output_idx >= want_outputs.len()
                                || out as u8 != want_outputs[output_idx]
                            {
                                return None;
                            }
                            output_idx += 1;
                        }
                        // bdv
                        6 => {
                            let val = self.inst[ip + 1];
                            registers[B] = registers[A] >> combo(registers, val);
                        }
                        // cdv
                        7 => {
                            let val = self.inst[ip + 1];
                            registers[C] = registers[A] >> combo(registers, val);
                        }
                        _ => unreachable!(),
                    }
                    ip += 2;
                }
                if output_idx == want_outputs.len() {
                    return Some(i);
                }
                None
            })
            .unwrap()
    }
}

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use nom::{bytes::complete::tag, IResult};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Program;
    type Part1Output = String;
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        parse_game(input).unwrap().1
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.execute()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.brute_force_part2()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const TEST_INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), "4,6,3,5,6,3,5,2,1,0");
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT2);
        assert_eq!(Solver::solve_part2(&parsed), 117440);
    }
}
