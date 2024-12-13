use std::str::FromStr;

use aoc_traits::AdventOfCodeDay;

#[derive(Debug, Clone)]
pub struct Grid {
    areas: Vec<(u8, Vec<(isize, isize)>)>,
}

impl FromStr for Grid {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut classes: Vec<Vec<(isize, isize)>> = vec![Vec::with_capacity(200); 256];
        for (y, line) in s.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' {
                    let class = c as usize;
                    classes[class].push((x as isize, y as isize));
                }
            }
        }
        let mut areas = Vec::new();
        let mut to_check = Vec::with_capacity(100);
        for (idx, mut class) in classes
            .into_iter()
            .enumerate()
            .filter(|(_, x)| !x.is_empty())
        {
            to_check.clear();
            while let Some(plot) = class.pop() {
                let mut new_area = vec![plot];
                to_check.extend(&[
                    (plot.0 + 1, plot.1),
                    (plot.0 - 1, plot.1),
                    (plot.0, plot.1 + 1),
                    (plot.0, plot.1 - 1),
                ]);
                while let Some((x, y)) = to_check.pop() {
                    if new_area.contains(&(x, y)) {
                        continue;
                    }
                    if let Some(idx) = class.iter().position(|c| c == &(x, y)) {
                        new_area.push(class.swap_remove(idx));
                        to_check.push((x + 1, y));
                        to_check.push((x - 1, y));
                        to_check.push((x, y + 1));
                        to_check.push((x, y - 1));
                    }
                }
                areas.push((idx as u8, new_area));
            }
        }

        Ok(Grid { areas })
    }
}

fn fence_cost_one(area: &[(isize, isize)]) -> usize {
    let size = area.len();
    let boundary: usize = area
        .iter()
        .map(|&(x, y)| {
            let mut sum = 0;
            if !area.contains(&(x + 1, y)) {
                sum += 1
            }
            if !area.contains(&(x - 1, y)) {
                sum += 1
            }
            if !area.contains(&(x, y + 1)) {
                sum += 1
            }
            if !area.contains(&(x, y - 1)) {
                sum += 1
            }
            sum
        })
        .sum();
    boundary * size
}

fn fence_cost_two(area: &[(isize, isize)]) -> usize {
    let size = area.len();
    let boundary: usize = area
        .iter()
        .map(|&(x, y)| {
            let mut sum = 0;
            let left = area.contains(&(x - 1, y));
            let right = area.contains(&(x + 1, y));
            let up = area.contains(&(x, y - 1));
            let down = area.contains(&(x, y + 1));
            let ul = area.contains(&(x - 1, y - 1));
            let ur = area.contains(&(x + 1, y - 1));
            let dl = area.contains(&(x - 1, y + 1));
            let dr = area.contains(&(x + 1, y + 1));
            if !left && !up {
                sum += 1
            }
            if !up && !right {
                sum += 1
            }
            if !right && !down {
                sum += 1
            }
            if !down && !left {
                sum += 1
            }
            if left && up && !ul {
                sum += 1
            }
            if up && right && !ur {
                sum += 1
            }
            if right && down && !dr {
                sum += 1
            }
            if down && left && !dl {
                sum += 1
            }
            sum
        })
        .sum();
    boundary * size
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
        input.areas.iter().map(|x| fence_cost_one(&x.1)).sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.areas.iter().map(|x| fence_cost_two(&x.1)).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT_SMALL: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";
    const TEST_INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
    const TEST_INPUT_SMALL2: &str = "AAAA
BBCD
BBCC
EEEC";

    const TEST_INPUT_SMALL3: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT_SMALL);
        assert_eq!(Solver::solve_part1(&parsed), 772);
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 1930);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT_SMALL2);
        assert_eq!(Solver::solve_part2(&parsed), 80);
        let parsed = Solver::parse_input(TEST_INPUT_SMALL3);
        assert_eq!(Solver::solve_part2(&parsed), 236);
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 1206);
    }
}
