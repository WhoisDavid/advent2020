use aoc_runner_derive::{aoc, aoc_generator};

pub struct TreeMap {
    map: Vec<Vec<bool>>,
    width: usize,
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> TreeMap {
    let map: Vec<Vec<bool>> = input
        .lines()
        .map(|i| i.chars().map(|c| c == '#').collect())
        .collect();
    let width = map
        .first()
        .expect("Input should have at least one row!")
        .len();
    TreeMap { map, width }
}

#[aoc(day3, part1)]
pub fn part1(input: &TreeMap) -> usize {
    slopy_walk(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn part2(input: &TreeMap) -> usize {
    /* Right 1, down 1.
       Right 3, down 1. (This is the slope you already checked.)
       Right 5, down 1.
       Right 7, down 1.
       Right 1, down 2.
    */
    let slopes: &[(usize, usize)] = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes
        .iter()
        .map(|(right, down)| slopy_walk(input, *right, *down))
        .product()
}

fn slopy_walk(input: &TreeMap, right: usize, down: usize) -> usize {
    let mut trees = 0;
    let mut col = 0;
    for row in input.map.iter().step_by(down) {
        if row[col] {
            trees += 1
        }
        col = (col + right) % input.width;
    }
    trees
}

#[cfg(test)]
mod test_day3 {
    use super::*;

    const TESTCASE: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(TESTCASE)), 7)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(TESTCASE)), 336)
    }
}
