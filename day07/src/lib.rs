use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

impl FromStr for Equation {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, numbers) = s.split_once(": ").ok_or_else(|| eyre::eyre!("no colon"))?;
        let target = target.parse()?;
        let numbers = numbers
            .split_whitespace()
            .map(|n| n.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Equation { target, numbers })
    }
}
impl Equation {
    fn solvable_part1(&self) -> bool {
        check_equation(self.target, &self.numbers)
    }
    fn solvable_part2(&self) -> bool {
        check_equation_with_concat(self.target, self.numbers[0], &self.numbers[1..])
    }
}

fn check_equation(target: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return target == 0;
    }
    let last = numbers.len() - 1;

    return check_equation(target - numbers[last], &numbers[..last])
        || (target % numbers[last] == 0
            && check_equation(target / numbers[last], &numbers[..last]));
}

fn check_equation_with_concat(target: u64, current: u64, numbers: &[u64]) -> bool {
    if numbers.is_empty() {
        return target == current;
    }
    let add = check_equation_with_concat(target, current + numbers[0], &numbers[1..]);
    let mul = check_equation_with_concat(target, current * numbers[0], &numbers[1..]);

    let concat = {
        let next = numbers[0];
        let log = next.ilog10() + 1;
        let new = current * 10u64.pow(log) + next;
        // let new = format!("{}{}", current, next).parse().unwrap();

        check_equation_with_concat(target, new, &numbers[1..])
    };

    add || mul || concat
}

#[derive(Debug, Clone)]
pub struct Game {
    equations: Vec<Equation>,
}

impl FromStr for Game {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations = s
            .lines()
            .map(|l| l.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Game { equations })
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Game;
    type Part1Output = u64;
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.parse().expect("Failed to parse input")
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input
            .equations
            .iter()
            .filter(|e| e.solvable_part1())
            .map(|e| e.target)
            .sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .equations
            .iter()
            .filter(|e| e.solvable_part2())
            .map(|e| e.target)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 3749);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 11387);
    }
}
