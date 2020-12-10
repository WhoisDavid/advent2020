use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

#[aoc_generator(day9)]
pub fn input_parser(input: &str) -> Vec<usize> {
    input.lines().map(|s| s.parse().expect("Integer")).collect()
}

fn two_sum_lookup(array: &[usize], target: usize) -> bool {
    let mut h = HashSet::new();
    for n in array {
        if h.contains(n) {
            return true;
        }
        h.insert(target - n);
    }
    false
}

#[aoc(day9, part1)]
pub fn part1(data: &[usize]) -> Option<usize> {
    for window in data.windows(26) {
        let (&x, preamble) = window.split_last().expect("Len == 26");
        if !two_sum_lookup(preamble, x) {
            return Some(x);
        }
    }
    None
}

fn two_sum_nested_loop(v: &[usize], target: usize) -> bool {
    for (i, vi) in v.iter().enumerate() {
        for vj in v.iter().skip(i + 1) {
            if vi + vj == target {
                return true;
            }
        }
    }
    false
}

/// Because of the small size of the input a nested loop in O(n^2) is actually 10x faster...
#[aoc(day9, part1, nested_loop)]
pub fn part1_nested_loop(input: &[usize]) -> Option<usize> {
    let window_size = 25;
    for i in window_size..input.len() {
        if !two_sum_nested_loop(&input[i - window_size..i], input[i]) {
            return Some(input[i]);
        }
    }
    None
}

/// Find a subarray whose sum matches target
/// Returns (if found) the indices start and end of the subarray
/// s.t. array[start..=end] sums to target
fn subarray_sum_lookup(array: &[usize], target: usize) -> Option<(usize, usize)> {
    let prefix: Vec<usize> = array
        .iter()
        .scan(0, |acc, x| {
            *acc += x;
            Some(*acc)
        })
        .collect();

    let mut h = HashMap::new();
    for (idx, n) in prefix.iter().enumerate() {
        if let Some(start) = h.get(n) {
            return Some((*start + 1, idx));
        }
        h.insert(target + n, idx);
    }
    None
}

#[aoc(day9, part2)]
pub fn part2(data: &[usize]) -> Option<usize> {
    let p1 = part1(data)?;
    let (start, end) = subarray_sum_lookup(data, p1)?;
    let range = &data[start..=end];
    assert_eq!(range.iter().sum::<usize>(), p1);
    Some(range.iter().min()? + range.iter().max()?)
}
