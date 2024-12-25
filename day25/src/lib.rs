use aoc_traits::AdventOfCodeDay;

#[derive(Debug)]
pub struct Collection {
    keys: Vec<[u8; 5]>,
    locks: Vec<[u8; 5]>,
}

impl Collection {
    fn parse(input: &str) -> Collection {
        let mut locks = Vec::new();
        let mut keys = Vec::new();
        for lock_or_key in input.split("\n\n") {
            let mut lines = lock_or_key.lines();
            let first_line = lines.next().unwrap();
            if first_line.contains("#") {
                // this is a lock
                let mut val = [0u8; 5];
                for line in lines {
                    for (i, c) in line.chars().enumerate() {
                        if c == '#' {
                            val[i] += 1;
                        }
                    }
                }
                locks.push(val);
            } else {
                // this is a key
                let mut val = [5u8; 5];
                for line in lines {
                    for (i, c) in line.chars().enumerate() {
                        if c == '.' {
                            val[i] -= 1;
                        }
                    }
                }
                keys.push(val);
            }
        }

        Collection { locks, keys }
    }
}

pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Collection;
    type Part1Output = usize;
    type Part2Output = &'static str;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        Collection::parse(input)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut count = 0;
        for lock in &input.locks {
            for key in &input.keys {
                count += if lock.iter().zip(key.iter()).all(|(l, k)| k + l < 6) {
                    1
                } else {
                    0
                };
            }
        }
        count
    }
    fn solve_part2(_input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        ""
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;

    const TEST_INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 3);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), 23);
    }
}
