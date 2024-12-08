use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone)]
pub struct Grid {
    x: usize,
    y: usize,
    classes: FxHashMap<u8, Vec<(isize, isize)>>,
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut g_y = 0;
        let mut g_x = 0;
        let mut classes: FxHashMap<u8, Vec<(isize, isize)>> = FxHashMap::default();
        for (y, line) in s.lines().enumerate() {
            g_y = y;
            g_x = line.len();
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let class = c as u8;
                    classes
                        .entry(class)
                        .or_default()
                        .push((x as isize, y as isize));
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

fn get_combinations(grid: &Grid, points: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut res = Vec::new();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let point1 = (2 * points[j].0 - points[i].0, 2 * points[j].1 - points[i].1);
            let point2 = (2 * points[i].0 - points[j].0, 2 * points[i].1 - points[j].1);
            if grid.in_bounds(point1.0, point1.1) {
                res.push(point1);
            }
            if grid.in_bounds(point2.0, point2.1) {
                res.push(point2);
            }
        }
    }
    res
}

fn get_combinations2(grid: &Grid, points: &[(isize, isize)]) -> FxHashSet<(isize, isize)> {
    let mut res = FxHashSet::default();

    for i in 0..points.len() {
        for j in i + 1..points.len() {
            let vector = (points[j].0 - points[i].0, points[j].1 - points[i].1);
            let mut point1 = points[j];
            while grid.in_bounds(point1.0, point1.1) {
                res.insert(point1);
                point1 = (point1.0 + vector.0, point1.1 + vector.1);
            }
            let mut point2 = points[i];
            while grid.in_bounds(point2.0, point2.1) {
                res.insert(point2);
                point2 = (point2.0 - vector.0, point2.1 - vector.1);
            }
        }
    }
    res
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
        input
            .classes
            .values()
            .flat_map(|v| get_combinations(input, v))
            .unique()
            .count()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .classes
            .values()
            .map(|v| get_combinations2(input, v))
            .reduce(|mut a, b| {
                a.extend(b);
                a
            })
            .unwrap()
            .len()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 14);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 34);
    }
}
