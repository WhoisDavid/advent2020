use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|i| i.parse::<i32>().expect("Integer"))
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> i32 {
    let mut h = HashSet::new();
    for n in input.iter() {
        if h.contains(n) {
            // println!("a={} b={} a*b={}", n, 2020 - n, (2020 - n) * n);
            return (2020 - n) * n;
        }
        h.insert(2020 - n);
    }
    unreachable!("Should find a solution")
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> i32 {
    for (i, a) in input.iter().enumerate() {
        let t = 2020 - a;
        let mut h = HashSet::new();
        for b in input.iter().skip(i + 1) {
            if h.contains(b) {
                // println!("a={} b={} c={} a*b*c={}", a, t - b, b, a * (t - b) * b);
                return a * (t - b) * b;
            }
            h.insert(t - b);
        }
    }
    unreachable!("Should find a solution")
}
