use std::{collections::VecDeque, str::FromStr};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct HardDrive {
    harddrive: Vec<Segment>,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    id: usize,
    start: usize,
    size: usize,
    free_after: usize,
}

impl FromStr for HardDrive {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start = 0;
        let mut idx = 0;
        let mut res = Vec::with_capacity(s.len() / 2 + 1);
        for i in (0..s.len()).step_by(2) {
            let size = (s.as_bytes()[i] as u8 - b'0') as usize;
            let free_after =
                (s.as_bytes().get(i + 1).copied().unwrap_or(b'0') as u8 - b'0' as u8) as usize;
            res.push(Segment {
                id: idx,
                start,
                size,
                free_after,
            });
            start += size + free_after;
            idx += 1;
        }
        Ok(HardDrive { harddrive: res })
    }
}
impl HardDrive {
    fn compress(&self) -> usize {
        let mut result: VecDeque<_> = self
            .harddrive
            .iter()
            .flat_map(|x| [(x.id, x.size), (0, x.free_after)])
            .collect();
        result.pop_back();

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

        'outer: while let Some(segment) = result.pop_back() {
            // already handled this
            if segment.id == 0 {
                continue;
            }
            // try to find a fitting space
            for i in 0..result.len() {
                if result[i].free_after >= segment.size {
                    let rest = result[i].free_after - segment.size;
                    result[i].free_after = 0;
                    let new = Segment {
                        id: 0,
                        start: result[i].start + result[i].size,
                        size: segment.size,
                        free_after: rest,
                    };
                    for i in 0..segment.size {
                        res += segment.id * (new.start + i);
                    }
                    result.insert(i + 1, new);
                    continue 'outer;
                }
            }
            // no fitting space found, do as is
            for i in 0..segment.size {
                res += segment.id * (segment.start + i);
            }
        }
        res
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = HardDrive;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.parse().expect("Failed to parse input")
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.compress()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.compress2()
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
        assert_eq!(Solver::solve_part2(&parsed), 2858);
    }
}
