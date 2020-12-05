use anyhow::anyhow;
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

type Seat = u32;

#[derive(Debug, PartialEq)]
pub struct BoardingPass {
    seat: Seat,
}

impl FromStr for BoardingPass {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(anyhow!("Not a valid pass!"));
        }

        let mut seat = 0;
        for c in s.chars() {
            seat <<= 1;
            seat += match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => return Err(anyhow!("Invalid char")),
            }
        }

        Ok(BoardingPass { seat })
    }
}

#[aoc_generator(day5)]
pub fn input_parser(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|s| s.parse::<BoardingPass>().expect("Valid Pass!"))
        .map(|bp| bp.seat)
        .collect()
}

#[aoc(day5, part1)]
pub fn part1(seats: &[Seat]) -> Option<Seat> {
    seats.iter().max().copied()
}

#[aoc(day5, part2, xor)]
pub fn part2_xor(seats: &[Seat]) -> Option<Seat> {
    let seats_xor = seats.iter().fold(0, |acc, b| acc ^ b);
    let full_range_xor = (*seats.iter().min()?..=*seats.iter().max()?).fold(0, |acc, b| acc ^ b);
    Some(seats_xor ^ full_range_xor)
}

#[aoc(day5, part2, sum)]
pub fn part2_sum(seats: &[Seat]) -> Option<Seat> {
    let (min, max) = (*seats.iter().min()?, *seats.iter().max()?);
    Some((min..=max).sum::<Seat>() - seats.iter().sum::<Seat>())
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

        for (s, (_row, _col, seat)) in tests.into_iter() {
            assert_eq!(s.parse::<BoardingPass>().unwrap(), BoardingPass { seat });
        }
    }
}
