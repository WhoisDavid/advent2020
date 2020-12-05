use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct BoardingPass {
    row: usize,
    col: usize,
    seat: usize,
}

impl BoardingPass {
    fn new(row: usize, col: usize) -> Self {
        Self {
            row,
            col,
            seat: 8 * row + col,
        }
    }
}

impl FromStr for BoardingPass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(anyhow!("Not a valid pass!"));
        }
        let row_str = &s[0..7];
        let col_str = &s[7..10];

        let mut row = 0;
        for c in row_str.chars() {
            row <<= 1;
            row += match c {
                'F' => 0,
                'B' => 1,
                _ => return Err(anyhow!("Invalid char")),
            }
        }

        let mut col = 0;
        for c in col_str.chars() {
            col <<= 1;
            col += match c {
                'L' => 0,
                'R' => 1,
                _ => return Err(anyhow!("Invalid char")),
            }
        }

        Ok(BoardingPass::new(row, col))
    }
}

#[aoc_generator(day5)]
pub fn input_parser(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|s| s.parse::<BoardingPass>().expect("Valid Pass!"))
        .map(|bp| bp.seat)
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(seats: &[usize]) -> Option<usize> {
    seats.iter().max().copied()
}

#[aoc(day5, part2, xor)]
pub fn part2_xor(seats: &[usize]) -> Option<usize> {
    let seats_xor = seats.iter().fold(0, |acc, x| acc ^ *x);
    let full_range_xor = (*seats.iter().min()?..=*seats.iter().max()?).fold(0, |acc, x| acc ^ x);
    Some(seats_xor ^ full_range_xor)
}

#[aoc(day5, part2, sort)]
pub fn part2_sort(seats: &[usize]) -> Option<usize> {
    let mut seats = seats.to_vec();
    seats.sort();
    let mut cur = seats[0];
    for &s in seats.iter().skip(1) {
        if s != cur + 1 {
            return Some(cur + 1);
        }
        cur = s;
    }
    None
}

#[cfg(test)]
mod test_day5 {
    use super::*;
    #[test]
    fn parsing() {
        let tests = vec![
            ("BFFFBBFRRR", (70, 7, 567)),
            ("FFFBBBFRRR", (14, 7, 119)),
            ("BBFFBBFRLL", (102, 4, 820)),
        ];

        for (s, (row, col, seat)) in tests.into_iter() {
            assert_eq!(
                s.parse::<BoardingPass>().unwrap(),
                BoardingPass { row, col, seat }
            );
        }
    }
}
