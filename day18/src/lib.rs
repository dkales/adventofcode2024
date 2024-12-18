use std::collections::VecDeque;

use aoc_traits::AdventOfCodeDay;
use eyre::OptionExt;

#[derive(Debug, Clone)]
pub struct Grid<const N: usize> {
    grid: [[u8; N]; N],
}
impl<const N: usize> Grid<N> {
    fn from_input(s: &str, limit: usize) -> Result<Self, eyre::Report> {
        let mut grid = [[0; N]; N];
        for line in s.lines().take(limit) {
            let (a, b) = line
                .split_once(",")
                .ok_or_eyre("invalid input, expected comma")?;
            let (x, y) = (a.parse::<usize>()?, b.parse::<usize>()?);
            grid[x][y] = 1;
        }

        Ok(Grid { grid })
    }
    fn add_coord(&mut self, x: usize, y: usize) {
        self.grid[x][y] = 1;
    }
    fn find_best_path(&self) -> usize {
        let start = (0usize, 0usize);
        let mut score = [[std::usize::MAX; N]; N];
        let mut to_visit = VecDeque::new();
        to_visit.push_back((start, 0));

        while let Some((idx, incoming_score)) = to_visit.pop_front() {
            for offset in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let next_idx = (
                    idx.0.checked_add_signed(offset.0),
                    idx.1.checked_add_signed(offset.1),
                );
                let next_idx = match next_idx {
                    (Some(x), Some(y)) => {
                        if x >= N || y >= N {
                            continue;
                        }
                        (x, y)
                    }
                    _ => continue,
                };
                if self.grid[next_idx.0][next_idx.1] == 1 {
                    continue;
                }
                let next_score = incoming_score + 1;
                if score[next_idx.0][next_idx.1] <= next_score {
                    continue;
                }
                score[next_idx.0][next_idx.1] = next_score;
                to_visit.push_back((next_idx, next_score));
            }
        }
        score[N - 1][N - 1]
    }
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = usize;
    type Part2Output = String;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let grid = Grid::<71>::from_input(input, 1024).unwrap();
        grid.find_best_path()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut grid = Grid::<71>::from_input(input, 1024).unwrap();
        for line in input.lines().skip(1024) {
            let (a, b) = line.split_once(",").expect("invalid input, expected comma");
            let (x, y) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
            grid.add_coord(x, y);
            if grid.find_best_path() == usize::MAX {
                return line.to_owned();
            }
        }
        "no solution found".into()
    }
}

#[cfg(test)]
mod tests {

    use crate::Grid;
    const TEST_INPUT_SMALL: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_stage1() {
        let grid = Grid::<7>::from_input(TEST_INPUT_SMALL, 12).unwrap();
        assert_eq!(grid.find_best_path(), 22);
    }
    #[test]
    fn test_stage2() {
        let mut grid = Grid::<7>::from_input(TEST_INPUT_SMALL, 12).unwrap();
        for line in TEST_INPUT_SMALL.lines().skip(12) {
            let (a, b) = line.split_once(",").expect("invalid input, expected comma");
            let (x, y) = (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap());
            grid.add_coord(x, y);
            if grid.find_best_path() == usize::MAX {
                assert_eq!(line, "6,1");
                return;
            }
        }
        panic!("no solution found");
    }
}
