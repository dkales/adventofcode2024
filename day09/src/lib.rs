use std::{collections::VecDeque, str::FromStr};

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct Grid {
    harddrive: Vec<(usize, usize)>,
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let harddrive = s
            .chars()
            .enumerate()
            .map(|(idx, c)| (idx / 2, (c as u8 - b'0') as usize))
            .collect::<Vec<_>>();
        Ok(Grid { harddrive })
    }
}
impl Grid {
    fn compress(&self) -> usize {
        let mut result: VecDeque<_> = self.harddrive.iter().copied().collect();

        let mut res = 0;
        let mut current_hd_index = 0;
        let mut free = false;
        loop {
            if !free {
                if result.is_empty() {
                    break;
                }
                let (block_id, block_size) = result.pop_front().unwrap();
                for _ in 0..block_size {
                    res += current_hd_index * block_id;
                    current_hd_index += 1;
                }
                free = true;
            } else {
                if result.is_empty() {
                    break;
                }
                let (_free_id, free_size) = result.pop_front().unwrap();
                if result.is_empty() {
                    break;
                }
                let (block_id, block_size) = result.pop_back().unwrap();
                if block_size < free_size {
                    for _ in 0..block_size {
                        res += current_hd_index * block_id;
                        current_hd_index += 1;
                    }
                    result.pop_back();
                    result.push_front((block_id, free_size - block_size));
                } else if block_size > free_size {
                    for _ in 0..free_size {
                        res += current_hd_index * block_id;
                        current_hd_index += 1;
                    }
                    result.push_back((block_id, block_size - free_size));
                    free = false;
                } else {
                    for _ in 0..free_size {
                        res += current_hd_index * block_id;
                        current_hd_index += 1;
                    }
                    result.pop_back();
                    free = false;
                }
            }
        }
        res
    }
    fn compress2(&self) -> usize {
        let mut result: VecDeque<_> = self.harddrive.iter().copied().collect();

        let mut res = 0;
        let mut current_hd_index = 0;
        let mut free = false;
        loop {
            if !free {
                if result.is_empty() {
                    break;
                }
                let (block_id, block_size) = result.pop_front().unwrap();
                for _ in 0..block_size {
                    res += current_hd_index * block_id;
                    current_hd_index += 1;
                }
                free = true;
            } else {
                if result.is_empty() {
                    break;
                }
                let (_free_id, free_size) = result.pop_front().unwrap();
                if result.is_empty() {
                    break;
                }
                // find the last block fitting space
                if let Some((idx, &(block_id, block_size))) = {
                    result
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(idx, (id, size))| *size <= free_size && *size > 0)
                } {
                    for _ in 0..block_size {
                        res += current_hd_index * block_id;
                        current_hd_index += 1;
                    }
                    if free_size > block_size {
                        result.push_front((_free_id, free_size - block_size));
                    }
                } else {
                    // nothing fits space, skip
                    result.pop_front();
                    free = false;
                }
            }
        }
        res
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
        input.compress()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        0
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "2333133121414131402";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 1928);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 34);
    }
}
