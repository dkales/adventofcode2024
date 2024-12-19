#[derive(Debug)]
pub struct Game<'a> {
    patterns: Vec<&'a str>,
    wanted: Vec<&'a str>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, patterns) = nom::sequence::terminated(
        nom::multi::separated_list1(tag(", "), nom::character::complete::alpha1),
        nom::multi::many1(newline),
    )(input)?;
    let (input, wanted) =
        nom::multi::separated_list1(newline, nom::character::complete::alpha1)(input)?;
    Ok((input, Game { patterns, wanted }))
}

fn patten_valid<'a>(
    patterns: &[&str],
    wanted: &'a str,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if cache.contains_key(wanted) {
        return *cache.get(wanted).unwrap();
    }
    patterns
        .iter()
        .filter(|&pattern| wanted.starts_with(pattern))
        .map(|&pattern| {
            let rest = &wanted[pattern.len()..];
            if rest.is_empty() {
                return 1;
            }
            let res = patten_valid(patterns, rest, cache);
            cache.insert(rest, res);
            res
        })
        .sum()
}

use std::collections::HashMap;

use aoc_traits::AdventOfCodeDay;
use nom::{bytes::complete::tag, character::complete::newline, IResult};
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Game<'a>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        parse_game(input).unwrap().1
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let regex = format!("^({})*$", input.patterns.join("|"));
        let regex = regex::Regex::new(&regex).unwrap();
        input
            .wanted
            .iter()
            .filter(|&&wanted| regex.is_match(wanted))
            .count()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut cache = HashMap::new();
        input
            .wanted
            .iter()
            .map(|&wanted| patten_valid(&input.patterns, wanted, &mut cache))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 6);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 6);
    }
}
