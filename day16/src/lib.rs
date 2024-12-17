use std::{collections::VecDeque, str::FromStr, vec};

use aoc_traits::AdventOfCodeDay;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    grid: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct ScoredGrid {
    score: Vec<[usize; 4]>,
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
        for line in s.lines() {
            gy += 1;
            for c in line.chars() {
                grid.push(c as u8);
            }
        }

        Ok(Grid { x, y: gy, grid })
    }
}

impl Grid {
    fn idx(&self, (x, y): (usize, usize)) -> usize {
        y * self.x + x
    }
    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.grid[self.idx((x, y))]
    }

    fn find_best_path(&self) -> Vec<[usize; 4]> {
        let start = (1, self.y - 2);
        let mut score = vec![[std::usize::MAX; 4]; self.grid.len()];
        let mut to_visit = VecDeque::new();
        to_visit.push_back((start, Direction::Right, 0));

        while let Some((idx, dir, incoming_score)) = to_visit.pop_front() {
            let next_idx = dir.step(idx);
            let next_tile = self.get(dir.step(idx));
            let next_score = incoming_score + 1;

            let left_score = incoming_score + 1000;
            let left_dir = dir.turn_left();
            if incoming_score >= score[self.idx(next_idx)][dir as usize] {
                continue;
            }
            if left_score < score[self.idx(idx)][left_dir as usize] {
                score[self.idx(idx)][left_dir as usize] = left_score;
                to_visit.push_back((idx, left_dir, left_score));
            }
            let right_score = incoming_score + 1000;
            let right_dir = dir.turn_right();
            if right_score < score[self.idx(idx)][right_dir as usize] {
                score[self.idx(idx)][right_dir as usize] = right_score;
                to_visit.push_back((idx, right_dir, right_score));
            }

            if next_tile == b'#' {
                continue;
            }

            if next_score < score[self.idx(next_idx)][dir as usize] {
                score[self.idx(next_idx)][dir as usize] = next_score;
                to_visit.push_back((next_idx, dir, next_score));
            }
        }
        score
    }
}

impl ScoredGrid {
    fn get_score(&self) -> usize {
        let end = (self.grid.x - 2, 1);
        self.score[self.grid.idx(end)]
            .iter()
            .copied()
            .min()
            .unwrap()
    }
    fn find_best_path_tiles(&self) -> usize {
        let end = (self.grid.x - 2, 1);

        let mut to_visit = VecDeque::new();
        // now walk back from the end
        let min_score = self.score[self.grid.idx(end)]
            .iter()
            .copied()
            .min()
            .unwrap();
        if self.score[self.grid.idx(end)][Direction::Up as usize] == min_score {
            to_visit.push_back((end, Direction::Up, min_score));
        }
        if self.score[self.grid.idx(end)][Direction::Down as usize] == min_score {
            to_visit.push_back((end, Direction::Down, min_score));
        }
        if self.score[self.grid.idx(end)][Direction::Left as usize] == min_score {
            to_visit.push_back((end, Direction::Left, min_score));
        }
        if self.score[self.grid.idx(end)][Direction::Right as usize] == min_score {
            to_visit.push_back((end, Direction::Right, min_score));
        }
        let mut points_on_path = FxHashSet::default();
        while let Some((idx, dir, incoming_score)) = to_visit.pop_front() {
            points_on_path.insert(idx);

            if incoming_score == 0 {
                continue;
            }
            let next_idx = dir.step_back(idx);
            if self.score[self.grid.idx(next_idx)][dir as usize] == incoming_score - 1 {
                to_visit.push_back((next_idx, dir, incoming_score - 1));
            }
            if incoming_score < 1000 {
                continue;
            }
            let left_dir = dir.turn_right();
            let right_dir = dir.turn_left();
            if self.score[self.grid.idx(idx)][left_dir as usize] == incoming_score - 1000 {
                to_visit.push_back((idx, left_dir, incoming_score - 1000));
            }
            if self.score[self.grid.idx(idx)][right_dir as usize] == incoming_score - 1000 {
                to_visit.push_back((idx, right_dir, incoming_score - 1000));
            }
        }
        points_on_path.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
}

impl Direction {
    fn step(&self, player: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (player.0, player.1 - 1),
            Self::Left => (player.0 - 1, player.1),
            Self::Down => (player.0, player.1 + 1),
            Self::Right => (player.0 + 1, player.1),
        }
    }
    fn step_back(&self, player: (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (player.0, player.1 + 1),
            Self::Left => (player.0 + 1, player.1),
            Self::Down => (player.0, player.1 - 1),
            Self::Right => (player.0 - 1, player.1),
        }
    }
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
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
        input.get_score()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.find_best_path_tiles()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT_SMALL: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    const TEST_INPUT: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT_SMALL);
        assert_eq!(Solver::solve_part1(&parsed), 7036);
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 11048);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT_SMALL);
        assert_eq!(Solver::solve_part2(&parsed), 45);
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 64);
    }
}
