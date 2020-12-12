use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    N(isize),
    S(isize),
    E(isize),
    W(isize),
    L(isize),
    R(isize),
    F(isize),
}

pub enum Direction {
    N,
    S,
    E,
    W,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = &s[0..1];
        let value = s[1..].parse::<isize>()?;
        match action {
            "N" => Ok(Self::N(value)),
            "S" => Ok(Self::S(value)),
            "E" => Ok(Self::E(value)),
            "W" => Ok(Self::W(value)),
            "L" => Ok(Self::L(value / 90)), // normalize by 90º rotations
            "R" => Ok(Self::R(value / 90)),
            "F" => Ok(Self::F(value)),
            c => Err(anyhow!("Invalid char: {}", c)),
        }
    }
}

#[aoc_generator(day12)]
pub fn input_parser(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|s| s.parse().expect("Instruction!"))
        .collect()
}
#[derive(Default)]
struct Coord {
    x: isize,
    y: isize,
}
struct Nav {
    ship: Coord,
    waypoint: Coord,
    dir: isize,
}

impl Default for Nav {
    fn default() -> Self {
        Nav {
            ship: Coord::default(),
            waypoint: Coord { x: -1, y: 10 },
            dir: 0,
        }
    }
}

impl Nav {
    const DIRS: [Direction; 4] = [Direction::E, Direction::S, Direction::W, Direction::N];

    fn execute(&mut self, a: &Action) {
        match a {
            Action::N(v) => {
                self.ship.x -= v;
            }
            Action::S(v) => {
                self.ship.x += v;
            }
            Action::E(v) => {
                self.ship.y += v;
            }
            Action::W(v) => {
                self.ship.y -= v;
            }
            Action::L(v) => self.dir -= v,
            Action::R(v) => self.dir += v,
            Action::F(v) => {
                let a = match Nav::DIRS[self.dir.rem_euclid(4) as usize] {
                    Direction::N => Action::N(*v),
                    Direction::S => Action::S(*v),
                    Direction::E => Action::E(*v),
                    Direction::W => Action::W(*v),
                };
                self.execute(&a)
            }
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.ship.x.abs() + self.ship.y.abs()
    }

    fn execute2(&mut self, a: &Action) {
        match a {
            Action::N(v) => {
                self.waypoint.x -= v;
            }
            Action::S(v) => {
                self.waypoint.x += v;
            }
            Action::E(v) => {
                self.waypoint.y += v;
            }
            Action::W(v) => {
                self.waypoint.y -= v;
            }
            Action::L(v) => {
                let (x, y) = (self.waypoint.x, self.waypoint.y);
                match v.rem_euclid(4) {
                    0 => {}
                    1 => { // 90º rotation
                        self.waypoint.x = -y;
                        self.waypoint.y = x;
                    }
                    2 => { // 180º rotation
                        self.waypoint.x = -x;
                        self.waypoint.y = -y;
                    }
                    3 => { // 270º rotation
                        self.waypoint.x = y;
                        self.waypoint.y = -x
                    }
                    _ => unreachable!(),
                };
            }
            Action::R(v) => self.execute2(&Action::L(4 - v.rem_euclid(4))),
            Action::F(v) => {
                self.ship.x += v * self.waypoint.x;
                self.ship.y += v * self.waypoint.y;
            }
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &[Action]) -> isize {
    let mut nav = Nav::default();
    input.iter().for_each(|a| nav.execute(a));
    nav.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Action]) -> isize {
    let mut nav = Nav::default();
    input.iter().for_each(|a| nav.execute2(a));
    nav.manhattan_distance()
}

#[cfg(test)]
mod test_day12 {
    use super::*;

    const TESTCASE: &str = "\
F10
N3
F7
R90
F11";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 25)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 286)
    }
}
