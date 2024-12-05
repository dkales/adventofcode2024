use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct PageGame {
    invalid_after: Vec<Vec<u8>>,
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

        let mut invalid_after: Vec<Vec<u8>> = vec![vec![]; 256];
        for (a, b) in &rules {
            invalid_after[*a as usize].push(*b);
        }

        Ok(PageGame {
            invalid_after,
            pages,
        })
    }
}

fn intersection_len(a: &[u8], b: &[u8]) -> usize {
    let mut count = 0;
    for i in a {
        if b.contains(i) {
            count += 1;
        }
    }
    count
}

impl PageGame {
    fn is_valid(&self, page: &[u8]) -> bool {
        for i in 0..page.len() {
            let check_against = &self.invalid_after[page[i] as usize];
            if !check_against.is_empty() {
                for check in 0..i {
                    if check_against.contains(&page[check]) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn sort(&self, page: &[u8]) -> Vec<u8> {
        let res = page.to_vec();
        let mut sorted: Vec<_> = res
            .into_iter()
            .map(|x| {
                let check_against = &self.invalid_after[x as usize];
                (x, intersection_len(&check_against, page))
            })
            .collect();
        sorted.sort_by_key(|(_, count)| *count);
        sorted.into_iter().map(|(x, _)| x).collect()
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
