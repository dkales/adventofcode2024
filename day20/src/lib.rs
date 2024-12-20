use std::{collections::VecDeque, str::FromStr, vec};

use aoc_traits::AdventOfCodeDay;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    grid: Vec<u8>,
    start: (usize, usize),
    end: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct ScoredGrid {
    score: Vec<usize>,
    grid: Grid,
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s
            .lines()
            .next()
            .ok_or_else(|| eyre::eyre!("empty input"))?
            .len();
        let mut gy = 0;

        let mut grid = Vec::with_capacity(x * x);
        let mut start = None;
        let mut end = None;
        for (y, line) in s.lines().enumerate() {
            gy += 1;
            for (x, c) in line.chars().enumerate() {
                if c == 'S' {
                    start = Some((x, y));
                }
                if c == 'E' {
                    end = Some((x, y));
                }
                grid.push(c as u8);
            }
        }

        Ok(Grid {
            x,
            y: gy,
            grid,
            start: start.unwrap(),
            end: end.unwrap(),
        })
    }
}

impl Grid {
    fn idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.x + x
    }
    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.grid[self.idx((x, y))]
    }

    fn find_best_path(&self) -> Vec<usize> {
        let start = self.end;
        let mut score = vec![std::usize::MAX; self.grid.len()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back((start, 0));
        score[self.idx(start)] = 0;

        while let Some((idx, incoming_score)) = to_visit.pop_front() {
            for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next_idx = (
                    idx.0.checked_add_signed(offset.0),
                    idx.1.checked_add_signed(offset.1),
                );
                let next_idx = match next_idx {
                    (Some(x), Some(y)) => {
                        if x >= self.x || y >= self.y {
                            continue;
                        }
                        (x, y)
                    }
                    _ => continue,
                };
                if self.get(next_idx) == b'#' {
                    continue;
                }
                let next_score = incoming_score + 1;
                if score[self.idx(next_idx)] <= next_score {
                    continue;
                }
                score[self.idx(next_idx)] = next_score;
                to_visit.push_back((next_idx, next_score));
            }
        }
        score
    }
}

impl ScoredGrid {
    fn find_possible_cheats(
        &self,
        limit: usize,
        time_limit: usize,
        start: (usize, usize),
        current_score: usize,
        cheating_paths: &mut FxHashSet<((usize, usize), (usize, usize))>,
    ) {
        // find all possible targets in the region allowed by limit
        for y_offset in -(time_limit as isize)..=(time_limit as isize) {
            for x_offset in -(time_limit as isize)..=(time_limit as isize) {
                let path_length = (y_offset.abs() + x_offset.abs()) as usize;
                if path_length > time_limit || path_length == 0 {
                    continue;
                }
                let target = (
                    start.0.checked_add_signed(x_offset),
                    start.1.checked_add_signed(y_offset),
                );
                let target = match target {
                    (Some(x), Some(y)) => {
                        if x >= self.grid.x || y >= self.grid.y {
                            continue;
                        }
                        (x, y)
                    }
                    _ => continue,
                };
                if current_score < limit {
                    continue;
                }
                let target_score = self.score[self.grid.idx(target)];
                if current_score - limit >= target_score.saturating_add(path_length) {
                    cheating_paths.insert((start, target));
                }
            }
        }
    }

    fn get_cheating_paths(&self, limit: usize, time_limit: usize) -> usize {
        let mut current = self.grid.start;
        let mut current_score = self.score[self.grid.idx(self.grid.start)];
        let mut path = Vec::with_capacity(current_score + 1);
        let mut cheating_paths = FxHashSet::default();
        while current != self.grid.end {
            path.push(current);
            let mut next_current = current;
            self.find_possible_cheats(
                limit,
                time_limit,
                current,
                current_score,
                &mut cheating_paths,
            );
            for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next_idx = (
                    current.0.checked_add_signed(offset.0),
                    current.1.checked_add_signed(offset.1),
                );
                let next_idx = match next_idx {
                    (Some(x), Some(y)) => {
                        if x >= self.grid.x || y >= self.grid.y {
                            continue;
                        }
                        (x, y)
                    }
                    _ => continue,
                };
                if self.grid.get(next_idx) == b'#' {
                    // check if cheating here would be better than limit
                    continue;
                }
                if self.score[self.grid.idx(next_idx)] == current_score - 1 {
                    next_current = next_idx;
                }
            }
            current = next_current;
            current_score -= 1;
        }
        cheating_paths.len()
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = ScoredGrid;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        let grid: Grid = input.parse().expect("Failed to parse input");
        ScoredGrid {
            score: grid.find_best_path(),
            grid,
        }
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.get_cheating_paths(100, 2)
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.get_cheating_paths(100, 20)
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(parsed.get_cheating_paths(64, 2), 1);
        assert_eq!(parsed.get_cheating_paths(40, 2), 2);
        assert_eq!(parsed.get_cheating_paths(38, 2), 3);
        assert_eq!(parsed.get_cheating_paths(36, 2), 4);
        assert_eq!(parsed.get_cheating_paths(20, 2), 5);
        assert_eq!(parsed.get_cheating_paths(12, 2), 8);
        assert_eq!(parsed.get_cheating_paths(10, 2), 10);
        assert_eq!(parsed.get_cheating_paths(8, 2), 14);
        assert_eq!(parsed.get_cheating_paths(6, 2), 16);
        assert_eq!(parsed.get_cheating_paths(4, 2), 30);
        assert_eq!(parsed.get_cheating_paths(2, 2), 44);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(parsed.get_cheating_paths(76, 20), 3);
        assert_eq!(parsed.get_cheating_paths(74, 20), 7);
        assert_eq!(parsed.get_cheating_paths(72, 20), 29);
        assert_eq!(parsed.get_cheating_paths(70, 20), 41);
    }
}
