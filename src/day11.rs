use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::convert::TryFrom;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Space {
    Floor,
    Seat(bool),
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;

    fn try_from(s: char) -> Result<Self, Self::Error> {
        match s {
            '.' => Ok(Self::Floor),
            'L' => Ok(Self::Seat(false)),
            '#' => Ok(Self::Seat(true)),
            c => Err(anyhow!("Invalid char: {}", c)),
        }
    }
}

impl ToString for Space {
    fn to_string(&self) -> String {
        match self {
            Space::Floor => ".".to_string(),
            Space::Seat(false) => "L".to_string(),
            Space::Seat(true) => "#".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Layout {
    grid: Vec<Space>,
    swaps: Vec<usize>,
    part2: bool,
    rows: isize,
    cols: isize,
}

impl std::fmt::Debug for Layout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Vec<String> = self.grid.iter().map(|s| s.to_string()).collect();
        let s: Vec<String> = s
            .chunks_exact(self.cols as usize)
            .map(|s| s.join(""))
            .collect();
        write!(f, "\n{}\n", s.join("\n"))
    }
}

impl Layout {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    fn bound_check(&self, x: isize, y: isize) -> bool {
        (0..self.rows).contains(&x) && (0..self.cols).contains(&y)
    }

    fn get(&self, x: isize, y: isize) -> Option<&Space> {
        if !self.bound_check(x, y) {
            return None;
        }
        self.grid.get((x * self.cols + y) as usize)
    }

    fn map_index(&self, idx: usize) -> (isize, isize) {
        let idx = idx as isize;
        (idx / self.cols, idx % self.cols)
    }

    fn count_adjacent_occupied(&self, idx: usize) -> usize {
        let (x, y) = self.map_index(idx);
        Self::DIRS
            .iter()
            .filter_map(move |(dx, dy)| self.get(x + dx, y + dy))
            .filter(|s| matches!(s, Space::Seat(true)))
            .count()
    }

    fn count_visible_occupied(&self, idx: usize) -> usize {
        let (x, y) = self.map_index(idx);
        Self::DIRS
            .iter()
            .filter_map(move |(dx, dy)| {
                let m = (1..).find(|m| {
                    let space = self.get(x + m * dx, y + m * dy);
                    space.is_none() || matches!(space, Some(Space::Seat(_)))
                })?;
                self.get(x + m * dx, y + m * dy)
            })
            .filter(|s| matches!(s, Space::Seat(true)))
            .count()
    }

    fn count_occupied_neighbors(&self, idx: usize) -> usize {
        if self.part2 {
            self.count_visible_occupied(idx)
        } else {
            self.count_adjacent_occupied(idx)
        }
    }

    fn count_occupied_seats(&self) -> usize {
        self.grid
            .iter()
            .filter(|s| **s == Space::Seat(true))
            .count()
    }
}

impl Iterator for Layout {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // Part 1 rule is 4 occupied seats, part2 is 5
        let threshold = 4 + (self.part2 as usize);
        for idx in 0..self.grid.len() {
            if let Space::Seat(occupied) = self.grid[idx] {
                let cnt = self.count_occupied_neighbors(idx);
                match (occupied, cnt) {
                    (true, cnt) if cnt >= threshold => self.swaps.push(idx),
                    (false, 0) => self.swaps.push(idx),
                    _ => (),
                }
            }
        }

        if self.swaps.is_empty() {
            None
        } else {
            for idx in self.swaps.drain(..) {
                match self.grid.get_mut(idx) {
                    Some(Space::Seat(occupied)) => *occupied = !*occupied,
                    _ => (),
                }
            }
            Some(debug!(self).count_occupied_seats())
        }
    }
}

#[aoc_generator(day11)]
pub fn input_parser(input: &str) -> Layout {
    let rows = input.lines().count() as isize;
    let cols = input.lines().next().expect("Non-empty input").len() as isize;
    let grid = input
        .lines()
        .flat_map(|s| {
            s.chars()
                .map(|c| Space::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect();
    Layout {
        grid,
        swaps: Vec::new(),
        part2: false,
        rows,
        cols,
    }
}

#[aoc(day11, part1)]
pub fn part1(layout: &Layout) -> Option<usize> {
    let layout = layout.clone();
    layout.into_iter().last()
}

#[aoc(day11, part2)]
pub fn part2(layout: &Layout) -> Option<usize> {
    let mut layout = layout.clone();
    layout.part2 = true;
    layout.into_iter().last()
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    const TESTCASE: &str = "\
L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), Some(37))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), Some(26))
    }
}
