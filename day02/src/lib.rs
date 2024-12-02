use aoc_traits::AdventOfCodeDay;

fn sequence_ok(seq: &[u32]) -> bool {
    if seq.len() <= 1 {
        return true;
    }
    let sign = seq[0] > seq[1];
    seq[..]
        .windows(2)
        .all(|w| (w[0] > w[1]) == sign && [1, 2, 3].contains(&w[0].abs_diff(w[1])))
}
fn subsequence_ok(seq: &[u32]) -> bool {
    let mut buf = vec![0; seq.len() - 1];

    for i in 0..seq.len() {
        buf[..i].copy_from_slice(&seq[..i]);
        buf[i..].copy_from_slice(&seq[i + 1..]);
        if sequence_ok(&buf) {
            return true;
        }
    }
    false
}

#[derive(Default)]
pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<Vec<u32>>;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.iter().filter(|x| sequence_ok(x)).count()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input
            .iter()
            .filter(|x| sequence_ok(x) || subsequence_ok(x))
            .count()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 2);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 4);
    }

    #[test]
    fn test() {
        let tests: [&[u32]; 2] = [&[20, 23, 25, 24, 26], &[44, 46, 49, 52, 55, 59]];

        for test in tests {
            assert_eq!(
                super::sequence_ok(&test) || super::subsequence_ok(&test),
                true
            );
        }
    }
}
