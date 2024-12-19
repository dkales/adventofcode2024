#[derive(Debug, Clone)]
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
                6 => registers[2],
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

    fn analyze_program(&self) -> u64 {
        let mut instructions: Vec<_> = self.inst[..].chunks_exact(2).collect();
        // some assertions for our program
        assert!(
            instructions.iter().filter(|inst| inst[0] == 3).count() == 1,
            "we only have 1 jump"
        );
        assert!(
            instructions.last().unwrap()[0] == 3 && instructions.last().unwrap()[1] == 0,
            "jump is in the end and jumps to 0"
        );
        assert!(
            instructions.iter().filter(|inst| inst[0] == 5).count() == 1,
            "we only have 1 output"
        );
        assert!(
            instructions[instructions.len() - 2][0] == 5,
            "output is second to last instruction"
        );
        let output_register = match instructions[instructions.len() - 2][1] {
            4 => A,
            5 => B,
            6 => C,
            _ => panic!("output does not actually output a register"),
        };
        // pop print and jump instructions
        instructions.pop();
        instructions.pop();

        let mut current_best_solution = u64::MAX;
        let config = z3::Config::new();
        loop {
            let ctx = z3::Context::new(&config);
            let solver = z3::Solver::new(&ctx);

            // initial register values
            const REGISTER_LEN: u32 = 64;
            let reg_a = z3::ast::BV::new_const(&ctx, "reg_a", REGISTER_LEN);
            let reg_b = z3::ast::BV::from_u64(
                &ctx,
                u64::try_from(self.registers[B]).expect("starting values are not larger than u64"),
                REGISTER_LEN,
            );
            let reg_c = z3::ast::BV::from_u64(
                &ctx,
                u64::try_from(self.registers[C]).expect("starting values are not larger than u64"),
                REGISTER_LEN,
            );
            let mut registers = [reg_a.clone(), reg_b, reg_c];
            let bound = z3::ast::BV::from_u64(&ctx, current_best_solution, REGISTER_LEN);
            solver.assert(&reg_a.bvult(&bound));

            fn z3_combo<'a>(
                ctx: &'a z3::Context,
                registers: &[z3::ast::BV<'a>; 3],
                val: u8,
            ) -> z3::ast::BV<'a> {
                match val {
                    0 => z3::ast::BV::from_u64(&ctx, 0, REGISTER_LEN),
                    1 => z3::ast::BV::from_u64(&ctx, 1, REGISTER_LEN),
                    2 => z3::ast::BV::from_u64(&ctx, 2, REGISTER_LEN),
                    3 => z3::ast::BV::from_u64(&ctx, 3, REGISTER_LEN),
                    4 => registers[0].clone(),
                    5 => registers[1].clone(),
                    6 => registers[2].clone(),
                    _ => unreachable!(),
                }
            }
            let and_mask = z3::ast::BV::from_u64(&ctx, 7, REGISTER_LEN);

            for wanted in self.inst.iter() {
                for instruction in instructions.iter() {
                    match instruction[0] {
                        // adv
                        0 => {
                            let val = instruction[1];
                            let shift_amount = z3_combo(&ctx, &registers, val);
                            let new_val = registers[A].bvlshr(&shift_amount);
                            registers[A] = new_val;
                        }
                        // bxl
                        1 => {
                            let val = instruction[1];
                            let new_val = registers[B].bvxor(&z3::ast::BV::from_u64(
                                &ctx,
                                val as u64,
                                REGISTER_LEN,
                            ));
                            registers[B] = new_val;
                        }
                        // bst
                        2 => {
                            let val = instruction[1];
                            let val = z3_combo(&ctx, &registers, val);
                            let new_val = val.bvand(&and_mask);
                            registers[B] = new_val;
                        }
                        // jmp
                        3 => {
                            panic!("jump instruction should not be here");
                        }
                        // bxc
                        4 => {
                            let new_val = registers[B].bvxor(&registers[C]);
                            registers[B] = new_val;
                        }
                        // out
                        5 => {
                            panic!("output instruction should not be here");
                        }
                        // bdv
                        6 => {
                            let val = instruction[1];
                            let shift_amount = z3_combo(&ctx, &registers, val);
                            let new_val = registers[A].bvlshr(&shift_amount);
                            registers[B] = new_val;
                        }
                        // cdv
                        7 => {
                            let val = instruction[1];
                            let shift_amount = z3_combo(&ctx, &registers, val);
                            let new_val = registers[A].bvlshr(&shift_amount);
                            registers[C] = new_val;
                        }
                        _ => unreachable!(),
                    }
                }

                // after instructions, we get an output
                let wanted = z3::ast::BV::from_u64(&ctx, *wanted as u64, REGISTER_LEN);
                let actual = registers[output_register].bvand(&and_mask);
                solver.assert(&wanted._eq(&actual));
            }
            // we also want the a register to be 0, since this is the end of the program
            solver.assert(&registers[A]._eq(&z3::ast::BV::from_u64(&ctx, 0, REGISTER_LEN)));

            if solver.check() != z3::SatResult::Sat {
                break;
            }
            let model = solver
                .get_model()
                .expect("have a model, since we checked for sat");
            let res = model
                .get_const_interp(&reg_a)
                .expect("register A should be in the model");
            let potential_res = res
                .as_u64()
                .expect("register A should be a u64 in the model");
            current_best_solution = potential_res;
        }
        current_best_solution
    }
}

use core::panic;

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use nom::{bytes::complete::tag, IResult};
use z3::ast::Ast;
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
        input.analyze_program()
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
