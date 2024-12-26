use aoc_traits::AdventOfCodeDay;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};

#[derive(Debug)]
pub struct Graph<'a> {
    adj: FxHashMap<&'a str, FxHashSet<&'a str>>,
}

impl<'a> Graph<'a> {
    fn parse(input: &'a str) -> Graph<'a> {
        let mut adj: FxHashMap<&'a str, FxHashSet<&'a str>> = FxHashMap::default();
        for line in input.lines() {
            let (a, b) = line.split_once('-').unwrap();
            adj.entry(a).or_default().insert(b);
            adj.entry(b).or_default().insert(a);
        }
        Graph { adj }
    }
}

fn bron_kerbosch<'a>(
    adj: &FxHashMap<&'a str, FxHashSet<&'a str>>,
    r: FxHashSet<&'a str>,
    mut p: FxHashSet<&'a str>,
    mut x: FxHashSet<&'a str>,
    out: &mut Vec<FxHashSet<&'a str>>,
) {
    if p.is_empty() && x.is_empty() {
        out.push(r);
        return;
    }
    for v in p.clone() {
        let mut rec_r = r.clone();
        rec_r.insert(v);
        let neighbors = adj.get(v).unwrap();
        let rec_p = p.intersection(neighbors).copied().collect();
        let rec_x = x.intersection(neighbors).copied().collect();
        bron_kerbosch(adj, rec_r, rec_p, rec_x, out);
        x.insert(v);
        p.remove(v);
    }
}

pub struct Solver;
impl AdventOfCodeDay for Solver {
    type ParsedInput<'a> = Graph<'a>;
    type Part1Output = usize;
    type Part2Output = String;

    fn parse_input(input: &str) -> Self::ParsedInput<'_> {
        Graph::parse(input)
    }

    fn solve_part1(input: &Self::ParsedInput<'_>) -> Self::Part1Output {
        let mut cliques = FxHashSet::default();
        for (a, adj) in &input.adj {
            if !a.starts_with("t") {
                continue;
            }
            for (b, c) in adj.iter().tuple_combinations() {
                if input.adj[b].contains(c) {
                    let mut abc = vec![a, b, c];
                    abc.sort();
                    cliques.insert((abc[0], abc[1], abc[2]));
                }
            }
        }
        cliques.len()
    }
    fn solve_part2(input: &Self::ParsedInput<'_>) -> Self::Part2Output {
        let mut out = Vec::new();
        bron_kerbosch(
            &input.adj,
            FxHashSet::default(),
            input.adj.keys().copied().collect(),
            FxHashSet::default(),
            &mut out,
        );
        out.sort_by(|a, b| a.len().cmp(&b.len()));
        out.pop().unwrap().into_iter().sorted().join(",")
    }
}

#[cfg(test)]
mod tests {
    use aoc_traits::AdventOfCodeDay;

    use crate::Solver;

    const TEST_INPUT: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_stage1() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part1(&parsed), 7);
    }
    #[test]
    fn test_stage2() {
        let parsed = Solver::parse_input(TEST_INPUT);
        assert_eq!(Solver::solve_part2(&parsed), "co,de,ka,ta".to_owned());
    }
}
