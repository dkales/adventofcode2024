use aoc_traits::AdventOfCodeDay;
use rustc_hash::FxHashMap;

struct KeyPad1 {
    state: u8,
}

impl KeyPad1 {
    fn new() -> Self {
        Self { state: b'A' }
    }

    fn sequence_for(&mut self, c: u8) -> &'static [u8] {
        let ret: &'static [u8] = match self.state {
            b'A' => match c {
                b'A' => b"A",
                b'0' => b"<A",
                b'1' => b"^<<A",
                b'2' => b"<^A",
                b'3' => b"^A",
                b'4' => b"^^<<A",
                b'5' => b"<^^A",
                b'6' => b"^^A",
                b'7' => b"^^^<<A",
                b'8' => b"<^^^A",
                b'9' => b"^^^A",
                _ => panic!("Invalid state"),
            },
            b'0' => match c {
                b'A' => b">A",
                b'0' => b"A",
                b'1' => b"^<A",
                b'2' => b"^A",
                b'3' => b"^>A",
                b'4' => b"^^<A",
                b'5' => b"^^A",
                b'6' => b"^^>A",
                b'7' => b"^^^<A",
                b'8' => b"^^^A",
                b'9' => b"^^^>A",
                _ => panic!("Invalid state"),
            },
            b'1' => match c {
                b'A' => b">>vA",
                b'0' => b">vA",
                b'1' => b"A",
                b'2' => b">A",
                b'3' => b">>A",
                b'4' => b"^A",
                b'5' => b"^>A",
                b'6' => b"^>>A",
                b'7' => b"^^A",
                b'8' => b"^^>A",
                b'9' => b"^^>>A",
                _ => panic!("Invalid state"),
            },
            b'2' => match c {
                b'A' => b"v>A",
                b'0' => b"vA",
                b'1' => b"<A",
                b'2' => b"A",
                b'3' => b">A",
                b'4' => b"<^A",
                b'5' => b"^A",
                b'6' => b"^>A",
                b'7' => b"<^^A",
                b'8' => b"^^A",
                b'9' => b"^^>A",
                _ => panic!("Invalid state"),
            },
            b'3' => match c {
                b'A' => b"vA",
                b'0' => b"<vA",
                b'1' => b"<<A",
                b'2' => b"<A",
                b'3' => b"A",
                b'4' => b"<<^A",
                b'5' => b"<^A",
                b'6' => b"^A",
                b'7' => b"<<^^A",
                b'8' => b"<^^A",
                b'9' => b"^^A",
                _ => panic!("Invalid state"),
            },
            b'4' => match c {
                b'A' => b">>vvA",
                b'0' => b">vvA",
                b'1' => b"vA",
                b'2' => b"v>A",
                b'3' => b"v>>A",
                b'4' => b"A",
                b'5' => b">A",
                b'6' => b">>A",
                b'7' => b"^A",
                b'8' => b"^>A",
                b'9' => b"^>>A",
                _ => panic!("Invalid state"),
            },
            b'5' => match c {
                b'A' => b"vv>A",
                b'0' => b"vvA",
                b'1' => b"<vA",
                b'2' => b"vA",
                b'3' => b"v>A",
                b'4' => b"<A",
                b'5' => b"A",
                b'6' => b">A",
                b'7' => b"<^A",
                b'8' => b"^A",
                b'9' => b"^>A",
                _ => panic!("Invalid state"),
            },
            b'6' => match c {
                b'A' => b"vvA",
                b'0' => b"<vvA",
                b'1' => b"<<vA",
                b'2' => b"<vA",
                b'3' => b"vA",
                b'4' => b"<<A",
                b'5' => b"<A",
                b'6' => b"A",
                b'7' => b"<<^A",
                b'8' => b"<^A",
                b'9' => b"^A",
                _ => panic!("Invalid state"),
            },
            b'7' => match c {
                b'A' => b">>vvvA",
                b'0' => b">vvvA",
                b'1' => b"vvA",
                b'2' => b"vv>A",
                b'3' => b"vv>>A",
                b'4' => b"vA",
                b'5' => b"v>A",
                b'6' => b"v>>A",
                b'7' => b"A",
                b'8' => b">A",
                b'9' => b">>A",
                _ => panic!("Invalid state"),
            },
            b'8' => match c {
                b'A' => b"vvv>A",
                b'0' => b"vvvA",
                b'1' => b"<vvA",
                b'2' => b"vvA",
                b'3' => b"vv>A",
                b'4' => b"<vA",
                b'5' => b"vA",
                b'6' => b"v>A",
                b'7' => b"<A",
                b'8' => b"A",
                b'9' => b">A",
                _ => panic!("Invalid state"),
            },
            b'9' => match c {
                b'A' => b"vvvA",
                b'0' => b"<vvvA",
                b'1' => b"<<vvA",
                b'2' => b"<vvA",
                b'3' => b"vvA",
                b'4' => b"<<vA",
                b'5' => b"<vA",
                b'6' => b"vA",
                b'7' => b"<<A",
                b'8' => b"<A",
                b'9' => b"A",
                _ => panic!("Invalid state"),
            },
            _ => panic!("Invalid state"),
        };
        self.state = c;
        ret
    }
}

#[derive(Debug, Clone)]
struct KeyPad2 {
    state: u8,
}
impl KeyPad2 {
    fn new() -> Self {
        Self { state: b'A' }
    }

    fn sequence_for(&mut self, c: u8) -> &'static [u8] {
        let ret: &'static [u8] = match self.state {
            b'A' => match c {
                b'A' => b"A",
                b'>' => b"vA",
                b'^' => b"<A",
                b'v' => b"<vA",
                b'<' => b"v<<A",
                _ => panic!("Invalid state"),
            },
            b'^' => match c {
                b'A' => b">A",
                b'>' => b"v>A",
                b'^' => b"A",
                b'v' => b"vA",
                b'<' => b"v<A",
                _ => panic!("Invalid state"),
            },
            b'>' => match c {
                b'A' => b"^A",
                b'>' => b"A",
                b'^' => b"<^A",
                b'v' => b"<A",
                b'<' => b"<<A",
                _ => panic!("Invalid state"),
            },
            b'v' => match c {
                b'A' => b"^>A",
                b'>' => b">A",
                b'^' => b"^A",
                b'v' => b"A",
                b'<' => b"<A",
                _ => panic!("Invalid state"),
            },
            b'<' => match c {
                b'A' => b">>^A",
                b'>' => b">>A",
                b'^' => b">^A",
                b'v' => b">A",
                b'<' => b"A",
                _ => panic!("Invalid state"),
            },
            _ => panic!("Invalid state"),
        };
        self.state = c;
        ret
    }
}

struct NestedKeyPads {
    cache: FxHashMap<Vec<u8>, usize>,
    current_buffer: Vec<u8>,
    keypad: KeyPad2,
    nested: Option<Box<NestedKeyPads>>,
}

impl NestedKeyPads {
    fn new(num_type_2_keypads: usize) -> Self {
        Self {
            keypad: KeyPad2::new(),
            cache: FxHashMap::default(),
            current_buffer: Vec::new(),
            nested: if num_type_2_keypads > 1 {
                Some(Box::new(NestedKeyPads::new(num_type_2_keypads - 1)))
            } else {
                None
            },
        }
    }
    fn sequence_for(&mut self, c: u8) -> usize {
        self.current_buffer.push(c);
        if c != b'A' {
            return 0;
        }
        assert!(self.keypad.state == b'A');
        if self.cache.contains_key(&self.current_buffer) {
            let res = *self.cache.get(&self.current_buffer).unwrap();
            self.current_buffer.clear();
            return res;
        }

        let res = self
            .current_buffer
            .iter()
            .map(|&c| {
                let sequence = self.keypad.sequence_for(c);

                if let Some(nested) = &mut self.nested {
                    sequence.iter().map(|&c| nested.sequence_for(c)).sum()
                } else {
                    sequence.len()
                }
            })
            .sum();
        self.cache.insert(self.current_buffer.clone(), res);
        self.current_buffer.clear();
        res
    }

    fn sanity_check(&self) {
        if let Some(nested) = &self.nested {
            nested.sanity_check();
        }
        assert!(self.current_buffer.is_empty());
    }
}

fn calculate_score(input: &str, num_type_2_keypads: usize) -> usize {
    let number: usize = input
        .trim_start_matches("0")
        .trim_end_matches("A")
        .parse()
        .expect("can parse number");
    let mut key_pad = KeyPad1::new();
    let x = input
        .as_bytes()
        .iter()
        .flat_map(|&c| key_pad.sequence_for(c))
        .copied()
        .collect::<Vec<_>>();

    let mut nested = NestedKeyPads::new(num_type_2_keypads);
    let len = x
        .iter()
        .map(|&c| {
            let res = nested.sequence_for(c);
            res
        })
        .sum::<usize>();
    nested.sanity_check();

    // println!("{}*{}={}", len, number, len * number);
    len * number
}

pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = &'a str;
    type Part1Output = usize;
    type Part2Output = usize;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        input
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        input.lines().map(|x| calculate_score(x, 2)).sum()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        input.lines().map(|x| calculate_score(x, 25)).sum()
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::{KeyPad1, KeyPad2, Solver};
    const TEST_INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn kats() {
        let input = b"029A";
        let mut key_pad = KeyPad1::new();
        let x = input
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        assert_eq!(x.len(), b"<A^A>^^AvvvA".len());
        let mut key_pad = KeyPad2::new();
        let x = x
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        assert_eq!(x.len(), b"v<<A>>^A<A>AvA<^AA>A<vAAA>^A".len());
        let mut key_pad = KeyPad2::new();
        let x = x
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        assert_eq!(
            x.len(),
            b"<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );

        let mut key_pad = KeyPad2::new();
        let x = b"<<^^A";
        let x = x
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        let mut key_pad = KeyPad2::new();
        let x = x
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        let mut key_pad = KeyPad2::new();
        let x = b"^^<<A";
        let x = x
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        let mut key_pad = KeyPad2::new();
        let x = x
            .iter()
            .flat_map(|&c| key_pad.sequence_for(c))
            .copied()
            .collect::<Vec<_>>();
        println!("{:?}", String::from_utf8(x.clone()));
        panic!("stop")
    }
    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 126384);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 6);
    }
}
