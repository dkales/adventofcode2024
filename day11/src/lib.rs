use rustc_hash::FxHashMap;
#[memoize::memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn step(number: u64, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }
    match number {
        0 => step(1, depth - 1),
        number if number.ilog10() % 2 == 1 => {
            let split = 10u64.pow(number.ilog10() / 2 + 1);
            step(number / split, depth - 1) + step(number % split, depth - 1)
        }
        number => step(number * 2024, depth - 1),
    }
}

use aoc_traits::AdventOfCodeDay;
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let res = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .map(|x| step(x, 25))
            .sum();
        memoized_flush_step();
        res
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let res = input
            .split_ascii_whitespace()
            .map(|x| x.parse::<u64>().unwrap())
            .map(|x| step(x, 75))
            .sum();
        memoized_flush_step();
        res
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "125 17";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 55312);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 34);
    }
}
