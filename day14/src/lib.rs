#[derive(Debug)]
pub struct Game {
    pos: (i64, i64),
    velocity: (i64, i64),
}

fn parse_coords(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(character::complete::i64, tag(","), character::complete::i64)(input)
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, (pos, velocity)) = preceded(
        tag("p="),
        separated_pair(parse_coords, tag(" v="), parse_coords),
    )(input)?;
    Ok((input, Game { pos, velocity }))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    nom::multi::separated_list1(nom::multi::many1(newline), parse_game)(input)
}

impl Game {
    fn solve_part1(&self, grid_dim: (i64, i64), steps: i64) -> usize {
        let res = self.pos_at(grid_dim, steps);
        let mut ret = 0;
        if res.0 == grid_dim.0 / 2 || res.1 == grid_dim.1 / 2 {
            return 4;
        }
        if res.0 < grid_dim.0 / 2 {
            ret |= 1;
        }
        if res.1 < grid_dim.1 / 2 {
            ret |= 2;
        }
        ret
    }
    fn pos_at(&self, grid_dim: (i64, i64), steps: i64) -> (i64, i64) {
        (
            (self.pos.0 + self.velocity.0 * steps).rem_euclid(grid_dim.0),
            (self.pos.1 + self.velocity.1 * steps).rem_euclid(grid_dim.1),
        )
    }
}

use aoc_traits::AdventOfCodeDay;
use nom::{
    bytes::complete::tag,
    character::{self, complete::newline},
    sequence::{preceded, separated_pair},
    IResult,
};
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Game>;
    type Part1Output = i64;
    type Part2Output = i64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        parse_games(input).unwrap().1
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut quadrants = [0; 5];
        input.iter().for_each(|x| {
            quadrants[x.solve_part1((101, 103), 100)] += 1;
        });
        quadrants.iter().take(4).product()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let grid = (101, 103);
        let mut best = 0;
        let mut best_i = 0;
        for i in 0..101 * 103 {
            let pos = input.iter().map(|x| x.pos_at(grid, i)).collect::<Vec<_>>();
            let mut max_line = 0;
            for x in 0..grid.0 {
                let mut diag = 0;
                for i in 0..grid.1 {
                    if pos.contains(&(x, i)) {
                        diag += 1;
                    }
                }
                max_line = max_line.max(diag);
            }
            if max_line > best {
                best = max_line;
                best_i = i;
            }
        }
        // let i = best_i;
        // let pos = input.iter().map(|x| x.pos_at(grid, i)).collect::<Vec<_>>();
        // println!("{}", i);
        // for y in 0..grid.1 {
        //     for x in 0..grid.0 {
        //         if pos.contains(&(x, y)) {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!();
        // }
        // println!("----------------------------------------------------------------------------------------------------------------------");
        best_i
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        let mut quadrants = [0; 5];
        parsed.iter().for_each(|x| {
            quadrants[x.solve_part1((11, 7), 100)] += 1;
        });
        let res: usize = quadrants.iter().take(4).product();
        assert_eq!(res, 12);
    }
}
