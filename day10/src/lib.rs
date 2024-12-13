use std::{iter, str::FromStr};

use aoc_traits::AdventOfCodeDay;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    classes: Vec<Vec<(isize, isize)>>,
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g_y = 0;
        let mut g_x = 0;
        let mut classes: Vec<Vec<(isize, isize)>> = vec![Vec::with_capacity(200); 10];
        for (y, line) in s.lines().enumerate() {
            g_y = y;
            g_x = line.len();
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let class = c as usize - '0' as usize;
                    classes[class].push((x as isize, y as isize));
                }
            }
        }
        Ok(Grid {
            x: g_x,
            y: g_y + 1,
            classes,
        })
    }
}

impl Grid {
    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.x as isize && y < self.y as isize
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Grid;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.parse().expect("Failed to parse input")
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut current_level: FxHashMap<_, _> = input.classes[9]
            .iter()
            .map(|&(x, y)| ((x, y), FxHashSet::from_iter(iter::once((x, y)))))
            .collect();
        for level in (0..9).rev() {
            let next_level = input.classes[level]
                .iter()
                .flat_map(|&(x, y)| {
                    let mut reachable_peaks = FxHashSet::default();
                    for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        if input.in_bounds(x, y) {
                            if let Some(peaks) = current_level.get(&(x, y)) {
                                reachable_peaks.extend(peaks.iter());
                            }
                        }
                    }
                    if reachable_peaks.is_empty() {
                        None
                    } else {
                        Some(((x, y), reachable_peaks))
                    }
                })
                .collect();
            current_level = next_level;
        }
        current_level.values().map(|x| x.len()).sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut current_level: FxHashMap<_, _> =
            input.classes[9].iter().map(|&(x, y)| ((x, y), 1)).collect();
        for level in (0..9).rev() {
            let next_level = input.classes[level]
                .iter()
                .flat_map(|&(x, y)| {
                    let mut count = 0;
                    for (x, y) in [(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)] {
                        if input.in_bounds(x, y) {
                            if let Some(peaks) = current_level.get(&(x, y)) {
                                count += peaks;
                            }
                        }
                    }
                    if count == 0 {
                        None
                    } else {
                        Some(((x, y), count))
                    }
                })
                .collect();
            current_level = next_level;
        }
        current_level.values().map(|x| x).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT_SMALL: &str = "0123
1234
8765
9876";
    const TEST_INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT_SMALL);
        assert_eq!(Solver::solve_part1(&parsed), 1);
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 36);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 81);
    }
}
