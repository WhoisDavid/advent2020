use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

use Direction::*;

impl FromStr for Direction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "e" => E,
            "se" => SE,
            "sw" => SW,
            "w" => W,
            "nw" => NW,
            "ne" => NE,
            _ => return Err(()),
        })
    }
}

#[aoc_generator(day24)]
pub fn input_parser(input: &str) -> Vec<Vec<Direction>> {
    input
        .lines()
        .map(|s| {
            let mut i = 0;
            let mut v = Vec::new();
            while i < s.len() {
                if let Ok(d) = s[i..i + 1].parse::<Direction>() {
                    v.push(d);
                    i += 1;
                } else {
                    if let Ok(d) = s[i..i + 2].parse::<Direction>() {
                        v.push(d);
                        i += 2;
                    }
                }
            }
            v
        })
        .collect()
}

#[derive(Debug, Default)]
pub struct Grid {
    cur: (isize, isize),
    map: HashMap<(isize, isize), bool>,
}

impl Grid {
    fn new(input: &[Vec<Direction>]) -> Self {
        let mut grid = Grid::default();
        for dirs in input {
            grid.switch_tile(dirs)
        }
        grid
    }

    fn switch_cur_tile(&mut self) {
        self.map
            .entry(self.cur)
            .and_modify(|e| *e = !*e)
            .or_insert(true);
    }

    fn step_xy((mut x, mut y): (isize, isize), dir: &Direction) -> (isize, isize) {
        match dir {
            E => x += 2,
            SE => {
                x += 1;
                y += 1
            }
            SW => {
                x -= 1;
                y += 1;
            }
            W => x -= 2,

            NW => {
                x -= 1;
                y -= 1;
            }
            NE => {
                x += 1;
                y -= 1;
            }
        }
        (x, y)
    }

    fn step(&mut self, dir: &Direction) {
        self.cur = Self::step_xy(self.cur, dir);
    }

    fn switch_tile(&mut self, dirs: &[Direction]) {
        self.cur = (0, 0);
        dirs.iter().for_each(|dir| self.step(dir));
        self.switch_cur_tile()
    }

    fn count_black(&self) -> usize {
        self.map.values().filter(|v| **v).count()
    }

    const DIRS: [Direction; 6] = [E, SE, SW, W, NW, NE];

    fn neighbors(xy: (isize, isize)) -> impl Iterator<Item = (isize, isize)> {
        Self::DIRS.iter().map(move |d| Self::step_xy(xy, d))
    }

    fn daily_flip(&mut self) {
        let mut black_neighbors_counter: HashMap<_, usize> = HashMap::new();
        for (&cur_black_tile, _) in self.map.iter().filter(|(_, black)| **black) {
            black_neighbors_counter.entry(cur_black_tile).or_default();

            Self::neighbors(cur_black_tile).for_each(|neighbor| {
                *black_neighbors_counter.entry(neighbor).or_default() += 1;
            })
        }
        for (xy, cnt) in black_neighbors_counter.drain() {
            match self.map.get(&xy) {
                Some(true) if cnt == 0 || cnt > 2 => self.map.insert(xy, false),
                Some(false) | None if cnt == 2 => self.map.insert(xy, true),
                _ => None,
            };
        }
    }

    fn days(&mut self, n: usize) -> usize {
        (0..n).for_each(|_| self.daily_flip());
        self.count_black()
    }
}

#[aoc(day24, part1)]
pub fn part1(input: &[Vec<Direction>]) -> usize {
    Grid::new(input).count_black()
}

#[aoc(day24, part2)]
pub fn part2(input: &[Vec<Direction>]) -> usize {
    Grid::new(input).days(100)
}

#[cfg(test)]
mod test_day24 {
    use super::*;

    const TESTCASE: &str = "\
sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 10)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 2208)
    }
}
