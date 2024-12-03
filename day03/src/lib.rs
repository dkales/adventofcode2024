use aoc_traits::AdventOfCodeDay;
use regex::Regex;

#[derive(Debug)]
pub enum Instruction {
    Do(),
    Mul(u32, u32),
    Dont(),
}

trait Executor {
    fn execute(inst: &[Instruction]) -> u32;
}

struct Phase1Executor;
impl Executor for Phase1Executor {
    fn execute(inst: &[Instruction]) -> u32 {
        let mut sum = 0;
        for i in inst {
            match i {
                Instruction::Do() => {}
                Instruction::Mul(a, b) => sum += a * b,
                Instruction::Dont() => {}
            }
        }
        sum
    }
}

struct Phase2Executor;

impl Executor for Phase2Executor {
    fn execute(inst: &[Instruction]) -> u32 {
        let mut sum = 0;
        let mut active = true;
        for i in inst {
            match i {
                Instruction::Do() => active = true,
                Instruction::Mul(a, b) => {
                    if active {
                        sum += a * b
                    }
                }
                Instruction::Dont() => active = false,
            }
        }
        sum
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Instruction>;
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let regex_mul = Regex::new("mul\\((\\d{1,3}),(\\d{1,3})\\)").unwrap();
        let regex_dont = Regex::new("don't\\(\\)").unwrap();
        let regex_do = Regex::new("do\\(\\)").unwrap();
        let dos = regex_do
            .find_iter(input)
            .map(|m| (m.start(), Instruction::Do()));
        let donts = regex_dont
            .find_iter(input)
            .map(|m| (m.start(), Instruction::Dont()));
        let muls = regex_mul.captures_iter(input).map(|cap| {
            let a = cap[1].parse().unwrap();
            let b = cap[2].parse().unwrap();
            (cap.get(0).unwrap().start(), Instruction::Mul(a, b))
        });
        let mut instructions = dos.chain(donts).chain(muls).collect::<Vec<_>>();
        instructions.sort_by_key(|x| x.0);
        instructions.into_iter().map(|x| x.1).collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        Phase1Executor::execute(input)
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        Phase2Executor::execute(input)
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 161);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT2);
        assert_eq!(Solver::solve_part2(&parsed), 48);
    }
}
