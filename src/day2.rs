use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<min>\d+)-(?P<max>\d+) (?P<chr>[a-z]): (?P<password>[a-z]+)")]
pub struct Password {
    min: usize,
    max: usize,
    chr: char,
    password: String,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day2, part1)]
pub fn part1(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let n = p.password.chars().filter(|c| *c == p.chr).count();
            p.min <= n && n <= p.max
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(passwords: &[Password]) -> usize {
    passwords
        .iter()
        .filter(|p| {
            let a = p.password.chars().nth(p.min - 1);
            let b = p.password.chars().nth(p.max - 1);
            (a == Some(p.chr)) ^ (b == Some(p.chr))
        })
        .count()
}
