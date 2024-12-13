#[derive(Debug)]
pub struct Game {
    a_vec: (i64, i64),
    b_vec: (i64, i64),
    target: (i64, i64),
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Button A: ")(input)?;
    let (input, a_x) = nom::combinator::map_res(
        nom::sequence::preceded(tag("X+"), nom::character::complete::digit1),
        str::parse,
    )(input)?;
    let (input, a_y) = nom::combinator::map_res(
        nom::sequence::preceded(tag(", Y+"), nom::character::complete::digit1),
        str::parse,
    )(input)?;
    let (input, _) = tag("\nButton B: ")(input)?;
    let (input, b_x) = nom::combinator::map_res(
        nom::sequence::preceded(tag("X+"), nom::character::complete::digit1),
        str::parse,
    )(input)?;
    let (input, b_y) = nom::combinator::map_res(
        nom::sequence::preceded(tag(", Y+"), nom::character::complete::digit1),
        str::parse,
    )(input)?;
    let (input, _) = tag("\nPrize: ")(input)?;
    let (input, target_x) = nom::combinator::map_res(
        nom::sequence::preceded(tag("X="), nom::character::complete::digit1),
        str::parse,
    )(input)?;
    let (input, target_y) = nom::combinator::map_res(
        nom::sequence::preceded(tag(", Y="), nom::character::complete::digit1),
        str::parse,
    )(input)?;
    Ok((
        input,
        Game {
            a_vec: (a_x, a_y),
            b_vec: (b_x, b_y),
            target: (target_x, target_y),
        },
    ))
}

fn parse_games(input: &str) -> IResult<&str, Vec<Game>> {
    nom::multi::separated_list1(nom::multi::many1(newline), parse_game)(input)
}

impl Game {
    fn solve(&self) -> Option<i64> {
        let v_num = self.a_vec.1 * self.target.0 - self.a_vec.0 * self.target.1;
        let v_den = self.a_vec.1 * self.b_vec.0 - self.a_vec.0 * self.b_vec.1;
        if v_num % v_den != 0 {
            return None;
        }
        let v = v_num / v_den;
        let u_num = self.target.0 - self.b_vec.0 * v;
        let u_den = self.a_vec.0;
        if u_num % u_den != 0 {
            return None;
        }
        let u = u_num / u_den;
        Some(u * 3 + v)
    }
}

use aoc_traits::AdventOfCodeDay;
use nom::{bytes::complete::tag, character::complete::newline, IResult};
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Game>;
    type Part1Output = i64;
    type Part2Output = i64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        parse_games(input).unwrap().1
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.iter().map(|game| game.solve().unwrap_or(0)).sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .map(|game| {
                let Game {
                    a_vec,
                    b_vec,
                    target: (target_x, target_y),
                } = *game;
                Game {
                    a_vec,
                    b_vec,
                    target: (target_x + 10000000000000, target_y + 10000000000000),
                }
            })
            .map(|game| game.solve().unwrap_or(0))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 480);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 875318608908);
    }
}
