use aoc_runner_derive::{aoc, aoc_generator};

const TILE_SIZE: usize = 10;
pub struct Tile {
    id: usize,
    grid: Vec<Vec<bool>>,
}

impl Tile {
    fn row(&self, idx: usize) -> impl Iterator<Item = &bool> {
        self.grid[idx].iter()
    }

    fn row_rev(&self, idx: usize) -> impl Iterator<Item = &bool> {
        self.grid[idx].iter().rev()
    }

    fn col(&self, idx: usize) -> impl Iterator<Item = &bool> {
        self.grid.iter().map(move |cols| &cols[idx])
    }

    fn col_rev(&self, idx: usize) -> impl Iterator<Item = &bool> {
        self.grid.iter().rev().map(move |cols| &cols[idx])
    }

    fn last_row(&self) -> impl Iterator<Item = &bool> {
        self.row(TILE_SIZE - 1)
    }

    fn last_col(&self) -> impl Iterator<Item = &bool> {
        self.col(TILE_SIZE - 1)
    }

    fn last_row_rev(&self) -> impl Iterator<Item = &bool> {
        self.row_rev(TILE_SIZE - 1)
    }

    fn last_col_rev(&self) -> impl Iterator<Item = &bool> {
        self.col_rev(TILE_SIZE - 1)
    }

    // fn edges(&self) -> itertools::IntoChunks<impl Iterator<Item = &bool>> {
    //     // self.first_row()
    //     //     .chain(self.last_col())
    //     //     .chain(self.last_row())
    //     //     .chain(self.first_col())
    //     //     .chunks(TILE_SIZE)
    //     self.row(0)
    //         .chain(self.last_row())
    //         .chain(self.col(0))
    //         .chain(self.last_col())
    //         .chain(self.row_rev(0))
    //         .chain(self.row_rev(TILE_SIZE - 1))
    //         .chain(self.col_rev(0))
    //         .chain(self.col_rev(TILE_SIZE - 1))
    //         .chunks(TILE_SIZE)
    // }

    fn match_tile(&self, t: &Tile) -> bool {
        // self.edges().into_iter().any(|e| e.eq(t.first_col()))
        //     || self.edges().into_iter().any(|e| e.eq(t.first_row()))
        //     || self.edges().into_iter().any(|e| e.eq(t.last_col()))
        //     || self.edges().into_iter().any(|e| e.eq(t.last_row()));

        self.col(0).eq(t.row(0))
            || self.col(0).eq(t.col(0))
            || self.col(0).eq(t.last_row())
            || self.col(0).eq(t.last_col())

            || self.row(0).eq(t.row(0))
            || self.row(0).eq(t.col(0))
            || self.row(0).eq(t.last_row())
            || self.row(0).eq(t.last_col())

            || self.last_col().eq(t.row(0))
            || self.last_col().eq(t.col(0))
            || self.last_col().eq(t.last_row())
            || self.last_col().eq(t.last_col())

            || self.last_row().eq(t.row(0))
            || self.last_row().eq(t.col(0))
            || self.last_row().eq(t.last_row())
            || self.last_row().eq(t.last_col())

            || self.col_rev(0).eq(t.row(0))
            || self.col_rev(0).eq(t.col(0))
            || self.col_rev(0).eq(t.last_row())
            || self.col_rev(0).eq(t.last_col())

            || self.row_rev(0).eq(t.row(0))
            || self.row_rev(0).eq(t.col(0))
            || self.row_rev(0).eq(t.last_row())
            || self.row_rev(0).eq(t.last_col())
            
            || self.last_col_rev().eq(t.row(0))
            || self.last_col_rev().eq(t.col(0))
            || self.last_col_rev().eq(t.last_row())
            || self.last_col_rev().eq(t.last_col())
            || self.last_row_rev().eq(t.row(0))
            || self.last_row_rev().eq(t.col(0))
            || self.last_row_rev().eq(t.last_row())
            || self.last_row_rev().eq(t.last_col())
    }
}

#[aoc_generator(day20)]
pub fn input_parser(input: &str) -> Option<Vec<Tile>> {
    let tiles = input.split("\n\n");
    tiles
        .map(|t| {
            let mut l = t.lines();
            let id = l
                .next()?
                .strip_prefix("Tile ")?
                .strip_suffix(":")?
                .parse()
                .ok()?;
            let grid = l.map(|s| s.chars().map(|c| c == '#').collect()).collect();
            Some(Tile { id, grid })
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(tiles: &[Tile]) -> usize {
    let mut match_count = vec![0; tiles.len()];
    for i in 0..tiles.len() {
        for j in (0..tiles.len()).filter(|j| *j != i) {
            match_count[i] += tiles[i].match_tile(&tiles[j]) as usize;
        }
    }
    match_count
        .iter()
        .enumerate()
        .filter(|(_idx, cnt)| **cnt == 2)
        .map(|(idx, _)| tiles[idx].id)
        .product()
}

// #[aoc(day20, part2)]
// pub fn part2(_input: &[Input]) -> usize {
//     unimplemented!()
// }

#[cfg(test)]
mod test_day20 {
    use super::*;

    const TESTCASE: &str = "\
Tile 2311:
..##.#..#.
##..#.....
#...##..#.
####.#...#
##.##.###.
##...#.###
.#.#.#..##
..#....#..
###...#.#.
..###..###

Tile 1951:
#.##...##.
#.####...#
.....#..##
#...######
.##.#....#
.###.#####
###.##.##.
.###....#.
..#.#..#.#
#...##.#..

Tile 1171:
####...##.
#..##.#..#
##.#..#.#.
.###.####.
..###.####
.##....##.
.#...####.
#.##.####.
####..#...
.....##...

Tile 1427:
###.##.#..
.#..#.##..
.#.##.#..#
#.#.#.##.#
....#...##
...##..##.
...#.#####
.#.####.#.
..#..###.#
..##.#..#.

Tile 1489:
##.#.#....
..##...#..
.##..##...
..#...#...
#####...#.
#..#.#.#.#
...#.#.#..
##.#...##.
..##.##.##
###.##.#..

Tile 2473:
#....####.
#..#.##...
#.##..#...
######.#.#
.#...#.#.#
.#########
.###.#..#.
########.#
##...##.#.
..###.#.#.

Tile 2971:
..#.#....#
#...###...
#.#.###...
##.##..#..
.#####..##
.#..####.#
#..#.#..#.
..####.###
..#.#.###.
...#.#.#.#

Tile 2729:
...#.#.#.#
####.#....
..#.#.....
....#..#.#
.##..##.#.
.#.####...
####.#.#..
##.####...
##..#.##..
#.##...##.

Tile 3079:
#.#.#####.
.#..######
..#.......
######....
####.#..#.
.#...#.##.
#.#####.##
..#.###...
..#.......
..#.###...";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE).unwrap()), 20899048083289)
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(&input_parser(TESTCASE)), 0)
    // }
}
