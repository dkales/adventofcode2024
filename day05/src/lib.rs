use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct PageGame {
    invalid_after: HashMap<u8, HashSet<u8>>,
    pages: Vec<Vec<u8>>,
}

impl FromStr for PageGame {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rules, pages) = s
            .split_once("\n\n")
            .ok_or_else(|| eyre::eyre!("Invalid input, no separation between pages and rules"))?;
        let rules = rules
            .lines()
            .map(|line| {
                let (a, b) = line
                    .split_once('|')
                    .ok_or_else(|| eyre::eyre!("Invalid rule"))?;
                Ok((a.parse()?, b.parse()?))
            })
            .collect::<Result<Vec<(u8, u8)>, eyre::Report>>()?;

        let pages = pages
            .lines()
            .map(|line| line.split(',').map(|n| n.parse()).collect::<Result<_, _>>())
            .collect::<Result<_, _>>()?;

        let mut invalid_after: HashMap<u8, HashSet<u8>> = HashMap::new();
        for (a, b) in &rules {
            invalid_after.entry(*a).or_default().insert(*b);
        }

        Ok(PageGame {
            invalid_after,
            pages,
        })
    }
}

impl PageGame {
    fn is_valid(&self, page: &[u8]) -> bool {
        for i in 0..page.len() {
            if let Some(check_against) = self.invalid_after.get(&page[i]) {
                for check in 0..i {
                    if check_against.contains(&page[check]) {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    fn fix_last_rec(&self, page: &mut [u8]) {
        let len = page.len();
        if len < 2 {
            return;
        }
        let not_allowed_set = page.iter().copied().collect::<HashSet<_>>();
        let empty = HashSet::new();
        for i in 0..len {
            let check_against = self.invalid_after.get(&page[i]).unwrap_or(&empty);
            if not_allowed_set.intersection(check_against).count() == 0 {
                page.swap(i, len - 1);
                break;
            }
        }
        self.fix_last_rec(&mut page[..len - 1]);
    }

    fn sort(&self, page: &[u8]) -> Vec<u8> {
        let mut res = page.to_vec();
        self.fix_last_rec(&mut res[..]);

        res
    }
    fn part1(&self) -> usize {
        self.pages
            .iter()
            .filter(|page| self.is_valid(page))
            .map(|page| page[page.len() / 2] as usize)
            .sum()
    }
    fn part2(&self) -> usize {
        self.pages
            .iter()
            .filter(|page| !self.is_valid(page))
            .map(|page| self.sort(page))
            .map(|page| page[page.len() / 2] as usize)
            .sum()
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = PageGame;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.parse().expect("Failed to parse input")
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.part1()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.part2()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 143);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 123);
    }
}
