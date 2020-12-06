use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc(day6, part1, hashset)]
pub fn part1(input: &str) -> usize {
    let unique = &mut HashSet::new();
    input
        .split("\n\n")
        .map(|s| {
            unique.clear();
            for answer in s.chars().filter(|c| *c != '\n') {
                unique.insert(answer);
            }
            unique.len()
        })
        .sum()
}

/// Code golf but 2x slower because (guess) it reallocates the HashSet for every person
#[aoc(day6, part1, itertools_unique)]
pub fn part1_unique(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|s| s.chars().filter(|c| *c != '\n').unique().count())
        .sum()
}

#[aoc(day6, part2)]
pub fn part2(input: &str) -> usize {
    let counter = &mut HashMap::new();
    input
        .split("\n\n")
        .map(|s| {
            let group_size = s.lines().count();
            for answer in s.chars().filter(|c| *c != '\n') {
                *counter.entry(answer).or_insert(0) += 1;
            }
            counter
                .drain()
                .filter(|(_, cnt)| *cnt == group_size)
                .count()
        })
        .sum()
}

#[cfg(test)]
mod test_day6 {
    use super::*;

    const TESTCASE: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTCASE), 11)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTCASE), 6)
    }
}
