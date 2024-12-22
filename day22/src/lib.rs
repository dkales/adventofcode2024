use aoc_traits::AdventOfCodeDay;

#[inline]
fn mix_and_prune(secret: u64, input: u64) -> u64 {
    (secret ^ input) % 16777216
}

#[inline]
fn evolve(secret: u64) -> u64 {
    let secret = mix_and_prune(secret, secret * 64);
    let secret = mix_and_prune(secret, secret / 32);
    let secret = mix_and_prune(secret, secret * 2048);
    secret
}
fn evolve_n(secret: u64, n: usize) -> u64 {
    let mut secret = secret;
    for _ in 0..n {
        secret = evolve(secret);
    }
    secret
}

fn idx(d1: i64, d2: i64, d3: i64, d4: i64) -> usize {
    ((d1 + 10) * 20 * 20 * 20 + (d2 + 10) * 20 * 20 + (d3 + 10) * 20 + (d4 + 10)) as usize
}
// fn rev_idx(idx: usize) -> (i64, i64, i64, i64) {
//     let d4 = (idx % 20) as i64 - 10;
//     let idx = idx / 20;
//     let d3 = (idx % 20) as i64 - 10;
//     let idx = idx / 20;
//     let d2 = (idx % 20) as i64 - 10;
//     let idx = idx / 20;
//     let d1 = (idx % 20) as i64 - 10;
//     (d1, d2, d3, d4)
// }

fn part2(secret: u64, overall: &mut [u64], n: usize) {
    let mut values = vec![u64::MAX; 20 * 20 * 20 * 20];
    let mut diffs = vec![(0i64, 0u64); n];

    // fill up initial diffs
    let mut last = secret;
    let mut current = evolve(last);

    for i in 0..n {
        diffs[i] = ((current % 10) as i64 - (last % 10) as i64, current % 10);
        last = current;
        current = evolve(last);
    }
    for i in 3..n {
        let index = idx(diffs[i - 3].0, diffs[i - 2].0, diffs[i - 1].0, diffs[i].0);
        if values[index] == u64::MAX {
            values[index] = diffs[i].1;
        }
    }

    overall
        .iter_mut()
        .zip(values.into_iter())
        .for_each(|(a, b)| {
            *a += if b == u64::MAX { 0 } else { b };
        });
}

pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Vec<u64>;
    type Part1Output = u64;
    type Part2Output = u64;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input.lines().map(|x| x.parse().unwrap()).collect()
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.iter().map(|&x| evolve_n(x, 2000)).sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut overall = vec![0u64; 20 * 20 * 20 * 20];
        input.iter().for_each(|&x| part2(x, &mut overall, 2000));

        overall.into_iter().max().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;

    const TEST_INPUT: &str = "1
10
100
2024";
    const TEST_INPUT2: &str = "1
2
3
2024";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 37327623);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT2);
        assert_eq!(Solver::solve_part2(&parsed), 23);
    }
}
