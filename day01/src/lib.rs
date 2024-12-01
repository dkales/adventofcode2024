use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
use itertools::izip;

#[derive(Default)]
pub struct Day1Solver;
impl<'a> AdventOfCodeDay<'a> for Day1Solver {
    type ParsedInput = (Vec<u32>, Vec<u32>);
    type Part1Output = u32;
    type Part2Output = u32;

    fn parse_input(input: &str) -> Self::ParsedInput {
        let mut a = Vec::new();
        let mut b = Vec::new();
        for line in input.lines() {
            let mut it = line.split_whitespace();
            a.push(
                it.next()
                    .unwrap()
                    .parse()
                    .expect("Could not parse first number"),
            );
            b.push(
                it.next()
                    .unwrap()
                    .parse()
                    .expect("Could not parse second number"),
            );
        }
        (a, b)
    }

    fn solve_part1(input: &Self::ParsedInput) -> Self::Part1Output {
        let (mut a, mut b) = input.clone();
        a.sort();
        b.sort();

        izip!(a, b).map(|(x, y)| x.abs_diff(y)).sum()
    }
    fn solve_part2((x, y): &Self::ParsedInput) -> Self::Part2Output {
        let mut counts = HashMap::new();
        for x in x.iter() {
            *counts.entry(x).or_insert(0) += 1;
        }

        y.iter().map(|y| counts.get(y).unwrap_or(&0) * y).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Day1Solver;
    const TEST_INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_stage1() {
        let parsed = Day1Solver::parse_input(TEST_INPUT);
        assert_eq!(Day1Solver::solve_part1(&parsed), 11);
    }
    #[test]
    fn test_stage2() {
        let parsed = Day1Solver::parse_input(TEST_INPUT);
        assert_eq!(Day1Solver::solve_part2(&parsed), 31);
    }
}
