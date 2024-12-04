use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;
use eyre::ensure;

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<u8>,
    x: usize,
    y: usize,
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.lines().next().unwrap().len();
        let y = s.lines().count();
        let grid: Vec<u8> = s
            .lines()
            .map(|line| line.chars().map(|c| c as u8).collect::<Vec<_>>())
            .flatten()
            .collect();
        ensure!(grid.len() == x * y, "Invalid grid");
        Ok(Grid { grid, x, y })
    }
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> Option<u8> {
        self.grid.get(y * self.x + x).copied()
    }

    fn find_xmas(&self) -> usize {
        let mut count = 0;
        const DIRS: &[(isize, isize)] = &[
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
        ];
        for x in 0..self.x {
            for y in 0..self.y {
                if self.get(x, y) != Some(b'X') {
                    continue;
                }
                for &(dx, dy) in DIRS {
                    let x = x as isize + dx;
                    let y = y as isize + dy;
                    if self.get(x as usize, y as usize) != Some(b'M') {
                        continue;
                    }
                    let x = x as isize + dx;
                    let y = y as isize + dy;
                    if self.get(x as usize, y as usize) != Some(b'A') {
                        continue;
                    }
                    let x = x as isize + dx;
                    let y = y as isize + dy;
                    if self.get(x as usize, y as usize) != Some(b'S') {
                        continue;
                    }
                    count += 1;
                }
            }
        }
        count
    }
    fn find_cross_mas(&self) -> usize {
        let mut count = 0;
        for x in 1..self.x - 1 {
            for y in 1..self.y - 1 {
                if self.get(x, y) != Some(b'A') {
                    continue;
                }

                let corners = [
                    self.get(x - 1, y - 1),
                    self.get(x + 1, y - 1),
                    self.get(x - 1, y + 1),
                    self.get(x + 1, y + 1),
                ];
                match corners {
                    [Some(b'M'), Some(b'M'), Some(b'S'), Some(b'S')] => count += 1,
                    [Some(b'S'), Some(b'S'), Some(b'M'), Some(b'M')] => count += 1,
                    [Some(b'M'), Some(b'S'), Some(b'M'), Some(b'S')] => count += 1,
                    [Some(b'S'), Some(b'M'), Some(b'S'), Some(b'M')] => count += 1,
                    _ => {}
                }
            }
        }
        count
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
        input.find_xmas()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.find_cross_mas()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 18);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 9);
    }
}
