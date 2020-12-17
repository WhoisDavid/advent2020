use aoc_runner_derive::{aoc, aoc_generator};
use itertools::iproduct;
use std::collections::HashSet;

type Int = isize;
type Coord3D = (Int, Int, Int);
type Grid3D = HashSet<Coord3D>;

const DIRS: [Int; 3] = [-1, 0, 1];

#[aoc_generator(day17, part1)]
pub fn input_parser_part1(input: &str) -> Grid3D {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y, _)| (x as Int, y as Int, 0))
        })
        .collect()
}

fn get_neighbors_3d(cube: Coord3D) -> impl Iterator<Item = Coord3D> {
    iproduct!(&DIRS, &DIRS, &DIRS)
        .filter(|(x, y, z)| !(**x == 0 && **y == 0 && **z == 0))
        .map(move |(x, y, z)| (cube.0 + x, cube.1 + y, cube.2 + z))
}

#[aoc(day17, part1)]
pub fn part1(grid: &Grid3D) -> usize {
    let mut grid = grid.clone();
    grid = (0..6).fold(grid, |grid, _| {
        grid.iter()
            .copied()
            .flat_map(get_neighbors_3d) // get all neighbors to visit
            .filter(|c| {
                let active_neighbors = get_neighbors_3d(*c)
                    .filter(|neighbor| grid.contains(neighbor))
                    .count();
                match active_neighbors {
                    2 if grid.contains(c) => true, // if active and 2 neighbors are active, active
                    3 => true,  // if active/inactive and 3 neighbors are active, active
                    _ => false, // otherwise inactive
                }
            })
            .collect()
    });

    grid.len()
}

// Part 2

type Coord4D = (Int, Int, Int, Int);
type Grid4D = HashSet<Coord4D>;

#[aoc_generator(day17, part2)]
pub fn input_parser_part2(input: &str) -> Grid4D {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y, _)| (x as Int, y as Int, 0, 0))
        })
        .collect()
}

fn get_neighbors_4d(cube: Coord4D) -> impl Iterator<Item = Coord4D> {
    iproduct!(&DIRS, &DIRS, &DIRS, &DIRS)
        .filter(|(x, y, z, w)| !(**x == 0 && **y == 0 && **z == 0 && **w == 0))
        .map(move |(x, y, z, w)| (cube.0 + x, cube.1 + y, cube.2 + z, cube.3 + w))
}

#[aoc(day17, part2)]
pub fn part2(grid: &Grid4D) -> usize {
    let mut grid = grid.clone();
    grid = (0..6).fold(grid, |grid, _| {
        grid.iter()
            .copied()
            .flat_map(get_neighbors_4d) // get all neighbors to visit
            .filter(|c| {
                let active_neighbors = get_neighbors_4d(*c)
                    .filter(|neighbor| grid.contains(neighbor))
                    .count();
                match active_neighbors {
                    2 if grid.contains(c) => true, // if active and 2 neighbors are active, active
                    3 => true,  // if active/inactive and 3 neighbors are active, active
                    _ => false, // otherwise inactive
                }
            })
            .collect()
    });

    grid.len()
}

#[cfg(test)]
mod test_day17 {
    use super::*;

    const TESTCASE: &str = "\
.#.
..#
###";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser_part1(TESTCASE)), 112)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser_part2(TESTCASE)), 848)
    }
}
