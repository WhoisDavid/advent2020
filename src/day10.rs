use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
pub fn input_parser(input: &str) -> Vec<usize> {
    let mut adapters: Vec<usize> = input.lines().map(|s| s.parse().expect("Integer")).collect();
    adapters.push(0);
    adapters.sort();
    let device = *adapters.last().unwrap() + 3;
    adapters.push(device);
    adapters
}

#[derive(Default)]
struct Joltage {
    cur: usize,
    diff1: usize,
    diff3: usize,
}

#[aoc(day10, part1)]
pub fn part1(adapters: &[usize]) -> usize {
    let j = adapters
        .iter()
        .fold(Joltage::default(), |mut j, adapter_joltage| {
            match adapter_joltage - j.cur {
                1 => j.diff1 += 1,
                3 => j.diff3 += 1,
                _ => (),
            };
            j.cur = *adapter_joltage;
            j
        });

    j.diff1 * j.diff3
}

/// Stupid DFS with memoization
#[aoc(day10, part2, DFS)]
pub fn part2_dfs(adapters: &[usize]) -> usize {
    let hm = &mut HashMap::new();
    dfs(0, &adapters, hm)
}

fn dfs(idx: usize, adapters: &[usize], hm: &mut HashMap<usize, usize>) -> usize {
    if idx == adapters.len() - 1 {
        return 1;
    }

    if let Some(c) = hm.get(&idx) {
        return *c;
    }

    let cur_joltage = adapters[idx];
    let r = (idx + 1..=idx + 3)
        .filter(|j| *j < adapters.len())
        .filter(|j| adapters[*j] - cur_joltage <= 3)
        .map(|j| dfs(j, adapters, hm))
        .sum::<usize>();
    hm.insert(idx, r);
    r
}

/// DFS to Dynamic Progamming...
/// path_counts[idx] == number of path from adapters[idx]
#[aoc(day10, part2, DP)]
pub fn part2_dp(adapters: &[usize]) -> usize {
    let mut path_counts = vec![0; adapters.len()];
    path_counts[adapters.len() - 1] = 1;
    for (idx, joltage) in adapters.iter().enumerate().rev().skip(1) {
        path_counts[idx] = (idx + 1..=idx + 3)
            .filter(|j| *j < adapters.len())
            .filter(|j| adapters[*j] - joltage <= 3)
            .map(|j| path_counts[j])
            .sum::<usize>();
    }
    path_counts[0]
}

/// Dynamic Progamming solution optimized
/// Since max_joltage is fairly small, just allocate a vec of size max_joltage..
/// path_counts[joltage] == number of path from given adapter joltage
#[aoc(day10, part2, DP_optimized)]
pub fn part2_dp_optimized(adapters: &[usize]) -> usize {
    let max_joltage = *adapters.last().unwrap();
    let mut path_counts = vec![0; max_joltage + 1];
    path_counts[max_joltage] = 1;
    for &joltage in adapters.iter().rev().skip(1) {
        path_counts[joltage] = path_counts[joltage + 1..=(joltage + 3).min(max_joltage)]
            .iter()
            .sum();
    }
    path_counts[0]
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    const TESTCASE1: &str = "\
16
10
15
5
1
11
7
19
6
12
4";

    const TESTCASE2: &str = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE1)), 35);
        assert_eq!(part1(&input_parser(TESTCASE2)), 220);
    }

    fn test_part2_helper(f: impl Fn(&[usize]) -> usize) {
        assert_eq!(f(&input_parser(TESTCASE1)), 8);
        assert_eq!(f(&input_parser(TESTCASE2)), 19208);
    }

    #[test]
    fn test_part2() {
        test_part2_helper(part2_dfs);
        test_part2_helper(part2_dp);
        test_part2_helper(part2_dp_optimized);
    }
}
