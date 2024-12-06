use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    stones: HashSet<(usize, usize)>,
    player: (usize, usize),
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g_x = 0;
        let mut g_y = 0;
        let mut player = None;

        let mut stones = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            g_y += 1;
            g_x = line.len();
            for (x, c) in line.chars().enumerate() {
                if c == '^' {
                    player = Some((x, y));
                } else if c == '#' {
                    stones.insert((x, y));
                }
            }
        }
        Ok(Grid {
            x: g_x,
            y: g_y,
            player: player.expect("have a player"),
            stones,
        })
    }
}

impl Grid {
    fn in_bounds(&self, x: isize, y: isize) -> bool {
        x >= 0 && y >= 0 && x < self.x as isize && y < self.y as isize
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
        if !grid.in_bounds(next_player.0, next_player.1) {
            break;
        }
        if grid
            .stones
            .contains(&(next_player.0 as usize, next_player.1 as usize))
        {
            dir = dir.turn();
        } else {
            player = next_player;
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
        if !grid.in_bounds(next_player.0, next_player.1) {
            break;
        }
        if grid
            .stones
            .contains(&(next_player.0 as usize, next_player.1 as usize))
        {
            dir = dir.turn();
        } else {
            player = next_player;
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
        if !grid.in_bounds(next_player.0, next_player.1) {
            break;
        }
        if grid
            .stones
            .contains(&(next_player.0 as usize, next_player.1 as usize))
        {
            dir = dir.turn();
        } else {
            player = next_player;
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
