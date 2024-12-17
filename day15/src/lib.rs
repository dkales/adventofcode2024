use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    grid: Vec<u8>,
    player: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct WideGrid {
    x: usize,
    _y: usize,
    grid: Vec<u8>,
    player: (usize, usize),
}

#[derive(Debug, Clone)]
pub struct Game {
    grid: Grid,
    instructions: Vec<Direction>,
}

impl FromStr for Game {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut player = None;

        let x = s
            .lines()
            .next()
            .ok_or_else(|| eyre::eyre!("empty input"))?
            .len();
        let mut gy = 0;

        let mut grid = Vec::with_capacity(x * x);
        let mut is_program = false;
        let mut program = Vec::with_capacity(x * x);
        for (y, line) in s.lines().enumerate() {
            if line.is_empty() {
                is_program = true;
                continue;
            }
            if !is_program {
                gy += 1;
                for (x, c) in line.chars().enumerate() {
                    if c == '@' {
                        player = Some((x, y));
                    }
                    grid.push(c as u8);
                }
            } else {
                for c in line.chars() {
                    program.push(match c {
                        '^' => Direction::Up,
                        'v' => Direction::Down,
                        '<' => Direction::Left,
                        '>' => Direction::Right,
                        _ => unreachable!(),
                    });
                }
            }
        }

        Ok(Game {
            instructions: program,
            grid: Grid {
                x,
                y: gy,
                player: player.expect("have a player"),
                grid,
            },
        })
    }
}

impl Grid {
    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.grid[y * self.x + x]
    }
    fn move_if_possible(&mut self, (x, y): (usize, usize), dir: Direction) -> bool {
        let target = dir.step((x, y));
        let target_item = self.get(target);
        match target_item {
            b'#' => false,
            b'.' => {
                self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                true
            }
            b'O' => {
                if self.move_if_possible(target, dir) {
                    self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                    true
                } else {
                    false
                }
            }
            _ => unreachable!(),
        }
    }
    fn simulate_step(mut self, dir: Direction) -> Self {
        if self.move_if_possible(self.player, dir) {
            Self {
                player: dir.step(self.player),
                ..self
            }
        } else {
            self
        }
    }
    fn run_program(self, program: &[Direction]) -> Self {
        program
            .iter()
            .fold(self, |grid, dir| grid.simulate_step(*dir))
    }
    fn stone_values(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'O')
            .map(|(idx, _)| (idx % self.x) + (idx / self.x) * 100)
            .sum()
    }
    fn make_wider(&self) -> WideGrid {
        let mut new_grid = Vec::with_capacity(self.grid.len() * 2);
        self.grid.iter().copied().for_each(|x| match x {
            b'#' => {
                new_grid.push(b'#');
                new_grid.push(b'#');
            }
            b'.' => {
                new_grid.push(b'.');
                new_grid.push(b'.');
            }
            b'O' => {
                new_grid.push(b'[');
                new_grid.push(b']');
            }
            b'@' => {
                new_grid.push(b'@');
                new_grid.push(b'.');
            }
            _ => unreachable!(),
        });
        WideGrid {
            x: self.x * 2,
            _y: self.y,
            player: (self.player.0 * 2, self.player.1),
            grid: new_grid,
        }
    }
}

impl WideGrid {
    fn get(&self, (x, y): (usize, usize)) -> u8 {
        self.grid[y * self.x + x]
    }
    fn check_if_move_possible(&self, (x, y): (usize, usize), dir: Direction) -> bool {
        let target = dir.step((x, y));
        let target_item = self.get(target);
        match target_item {
            b'#' => false,
            b'.' => true,
            b'[' => {
                self.check_if_move_possible(target, dir)
                    && self.check_if_move_possible((target.0 + 1, target.1), dir)
            }
            b']' => {
                self.check_if_move_possible(target, dir)
                    && self.check_if_move_possible((target.0 - 1, target.1), dir)
            }
            _ => unreachable!(),
        }
    }
    fn move_if_possible(&mut self, (x, y): (usize, usize), dir: Direction) -> bool {
        let target = dir.step((x, y));
        let target_item = self.get(target);
        match dir {
            Direction::Left | Direction::Right => match target_item {
                b'#' => false,
                b'.' => {
                    self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                    true
                }
                b'[' | b']' => {
                    if self.move_if_possible(target, dir) {
                        self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                        true
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            },
            Direction::Up | Direction::Down => match target_item {
                b'#' => false,
                b'.' => {
                    self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                    true
                }
                b'[' => {
                    if self.check_if_move_possible((x, y), dir) {
                        assert!(self.move_if_possible(target, dir));
                        assert!(self.move_if_possible((target.0 + 1, target.1), dir));
                        self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                        true
                    } else {
                        false
                    }
                }
                b']' => {
                    if self.check_if_move_possible((x, y), dir) {
                        assert!(self.move_if_possible(target, dir));
                        assert!(self.move_if_possible((target.0 - 1, target.1), dir));
                        self.grid.swap(target.1 * self.x + target.0, y * self.x + x);
                        true
                    } else {
                        false
                    }
                }
                _ => unreachable!(),
            },
        }
    }
    fn simulate_step(mut self, dir: Direction) -> Self {
        if self.move_if_possible(self.player, dir) {
            Self {
                player: dir.step(self.player),
                ..self
            }
        } else {
            self
        }
    }
    fn run_program(self, program: &[Direction]) -> Self {
        program
            .iter()
            .fold(self, |grid, dir| grid.simulate_step(*dir))
    }
    fn stone_values(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .filter(|(_, &x)| x == b'[')
            .map(|(idx, _)| (idx % self.x) + (idx / self.x) * 100)
            .sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
enum Direction {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
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
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Game;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.parse().expect("Failed to parse input")
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let grid = input.grid.clone().run_program(&input.instructions);
        grid.stone_values()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let grid = input.grid.make_wider();
        let grid = grid.run_program(&input.instructions);
        grid.stone_values()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT_SMALL: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const TEST_INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT_SMALL);
        assert_eq!(Solver::solve_part1(&parsed), 2028);
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 10092);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 9021);
    }
}
