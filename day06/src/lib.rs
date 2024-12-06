use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_traits::AdventOfCodeDay;
use eyre::ensure;

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<u8>,
    x: usize,
    y: usize,
    player: (usize, usize),
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = s.lines().next().unwrap().len();
        let y = s.lines().count();
        let mut player = None;
        let grid: Vec<u8> = s
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == '^' {
                            player = Some((x, y));
                            b'.'
                        } else {
                            c as u8
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();
        ensure!(grid.len() == x * y, "Invalid grid");
        Ok(Grid {
            grid,
            x,
            y,
            player: player.expect("have a player"),
        })
    }
}

impl Grid {
    fn get(&self, x: isize, y: isize) -> Option<u8> {
        if x >= 0 && y >= 0 && x < self.x as isize && y < self.y as isize {
            Some(self.grid[y as usize * self.x + x as usize])
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
    fn step(&self, player: (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (player.0, player.1 - 1),
            Self::Left => (player.0 - 1, player.1),
            Self::Down => (player.0, player.1 + 1),
            Self::Right => (player.0 + 1, player.1),
        }
    }
}

fn part1(grid: &Grid) -> usize {
    let mut visited = HashSet::new();
    let mut dir = Direction::Up;
    let mut player = (grid.player.0 as isize, grid.player.1 as isize);
    loop {
        visited.insert(player);

        let next_player = dir.step(player);
        match grid.get(next_player.0, next_player.1) {
            None => break,
            Some(b'#') => {
                dir = dir.turn();
                continue;
            }
            Some(b'.') => {
                player = next_player;
            }
            _ => unreachable!(),
        }
    }
    visited.len()
}

fn check_if_loops(
    grid: &Grid,
    mut visited: HashMap<(isize, isize), Vec<Direction>>,
    mut player: (isize, isize),
    mut dir: Direction,
    new_rock: (isize, isize),
) -> bool {
    // if we put a rock in this position we could not have gotten here at all
    if visited.contains_key(&new_rock) {
        return false;
    }
    loop {
        if let Some(inter_dir) = visited.get(&player) {
            if inter_dir.contains(&dir) {
                return true;
            }
        }
        visited.entry(player).or_default().push(dir);

        let next_player = dir.step(player);
        if next_player == new_rock {
            dir = dir.turn();
            continue;
        }
        match grid.get(next_player.0, next_player.1) {
            None => break,
            Some(b'#') => {
                dir = dir.turn();
                continue;
            }
            Some(b'.') => {
                player = next_player;
            }
            _ => unreachable!(),
        }
    }
    false
}

fn part2(grid: &Grid) -> usize {
    let mut potential_loops = HashSet::new();
    let mut visited: HashMap<(isize, isize), Vec<Direction>> = HashMap::new();
    let mut dir = Direction::Up;
    let mut player = (grid.player.0 as isize, grid.player.1 as isize);
    loop {
        if check_if_loops(grid, visited.clone(), player, dir.turn(), dir.step(player)) {
            potential_loops.insert(dir.step(player));
        }
        visited.entry(player).or_default().push(dir);

        let next_player = dir.step(player);
        match grid.get(next_player.0, next_player.1) {
            None => break,
            Some(b'#') => {
                dir = dir.turn();
                continue;
            }
            Some(b'.') => {
                player = next_player;
            }
            _ => unreachable!(),
        }
    }
    potential_loops.len()
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
        part1(input)
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        part2(input)
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 41);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 6);
    }
}
