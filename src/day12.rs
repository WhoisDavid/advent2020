use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Action {
    N,
    S,
    E,
    W,
    L,
    R,
    F,
}
pub struct Instruction {
    action: Action,
    value: isize,
}

impl FromStr for Action {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" => Self::N,
            "S" => Self::S,
            "E" => Self::E,
            "W" => Self::W,
            "L" => Self::L,
            "R" => Self::R,
            "F" => Self::F,
            c => return Err(anyhow!("Invalid char: {}", c)),
        })
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let action = s[0..1].parse()?;
        let value = s[1..].parse::<isize>()?;
        Ok(Instruction { action, value })
    }
}

#[aoc_generator(day12)]
pub fn input_parser(input: &str) -> Vec<Instruction> {
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
    const DIRS: [Action; 4] = [Action::E, Action::S, Action::W, Action::N];

    fn execute(&mut self, instruction: &Instruction) {
        let value = instruction.value;
        match instruction.action {
            Action::N => {
                self.ship.x -= value;
            }
            Action::S => {
                self.ship.x += value;
            }
            Action::E => {
                self.ship.y += value;
            }
            Action::W => {
                self.ship.y -= value;
            }
            Action::L => self.dir -= value,
            Action::R => self.dir += value,
            Action::F => self.execute(&Instruction {
                action: Nav::DIRS[self.dir.rem_euclid(360) as usize / 90],
                value,
            }),
        }
    }

    fn manhattan_distance(&self) -> isize {
        self.ship.x.abs() + self.ship.y.abs()
    }

    fn execute2(&mut self, instruction: &Instruction) {
        let value = instruction.value;
        match instruction.action {
            Action::N => {
                self.waypoint.x -= value;
            }
            Action::S => {
                self.waypoint.x += value;
            }
            Action::E => {
                self.waypoint.y += value;
            }
            Action::W => {
                self.waypoint.y -= value;
            }
            Action::L => {
                let (x, y) = (self.waypoint.x, self.waypoint.y);
                match value.rem_euclid(360) {
                    0 => {}
                    90 => {
                        // 90º rotation
                        self.waypoint.x = -y;
                        self.waypoint.y = x;
                    }
                    190 => {
                        // 180º rotation
                        self.waypoint.x = -x;
                        self.waypoint.y = -y;
                    }
                    270 => {
                        // 270º rotation
                        self.waypoint.x = y;
                        self.waypoint.y = -x
                    }
                    _ => unreachable!(),
                };
            }
            Action::R => self.execute2(&Instruction {
                action: Action::L,
                value: 360 - value.rem_euclid(360),
            }),
            Action::F => {
                self.ship.x += value * self.waypoint.x;
                self.ship.y += value * self.waypoint.y;
            }
        }
    }
}

#[aoc(day12, part1)]
pub fn part1(input: &[Instruction]) -> isize {
    let mut nav = Nav::default();
    input.iter().for_each(|a| nav.execute(a));
    nav.manhattan_distance()
}

#[aoc(day12, part2)]
pub fn part2(input: &[Instruction]) -> isize {
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
