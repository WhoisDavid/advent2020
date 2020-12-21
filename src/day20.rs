use aoc_runner_derive::{aoc, aoc_generator};

use hashbrown::HashMap;
use std::fmt::Display;

const TILE_SIZE: usize = 10;

#[derive(Debug, Clone, Default)]
pub struct Tile {
    id: usize,
    grid: Vec<Vec<bool>>,
    edges: Vec<usize>,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.grid.iter() {
            let s: String = v.iter().map(|b| if *b { "#" } else { "." }).collect();
            writeln!(f, "{}", s)?
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Edge {
    Top(bool),
    Bottom(bool),
    Left(bool),
    Right(bool),
}

use Edge::*;

impl Tile {
    fn new(id: usize, grid: Vec<Vec<bool>>) -> Self {
        Self {
            id,
            edges: Self::edges(&grid),
            grid,
        }
    }

    // Get vec of edges represented as an integer
    fn edges(grid: &[Vec<bool>]) -> Vec<usize> {
        let fold_int = |acc, &b| (acc + b as usize) << 1;
        let row = |idx: usize| grid[idx].iter().fold(0, fold_int);
        let col = |idx: usize| grid.iter().map(|cols| &cols[idx]).fold(0, fold_int);
        let row_rev = |idx: usize| grid[idx].iter().rev().fold(0, fold_int);
        let col_rev = |idx: usize| grid.iter().rev().map(|cols| &cols[idx]).fold(0, fold_int);

        vec![
            row(0),
            row(grid.len() - 1),
            col(0),
            col(grid.len() - 1),
            row_rev(0),
            row_rev(grid.len() - 1),
            col_rev(0),
            col_rev(grid.len() - 1),
        ]
    }

    fn edge(idx: usize) -> Edge {
        match idx {
            0 => Top(false),
            1 => Bottom(false),
            2 => Left(false),
            3 => Right(false),
            4 => Top(true),
            5 => Bottom(true),
            6 => Left(true),
            7 => Right(true),
            _ => unreachable!(),
        }
    }

    fn transpose(&mut self) {
        for i in 0..self.grid.len() {
            for j in i + 1..self.grid.len() {
                let tmp = self.grid[i][j];
                self.grid[i][j] = self.grid[j][i];
                self.grid[j][i] = tmp;
            }
        }
    }

    fn flip_cols(&mut self) {
        self.grid.iter_mut().for_each(|row| row.reverse());
        self.edges = Self::edges(&self.grid);
    }

    fn flip_rows(&mut self) {
        self.grid.reverse();
        self.edges = Self::edges(&self.grid);
    }

    fn rotate(&mut self, n: u8) {
        // rotate = transpose + flip
        match n % 4 {
            0 => {} // no-op
            1 => {
                // 90 clockwise
                self.transpose();
                self.flip_cols();
            }
            2 => {
                // 180
                self.flip_rows();
                self.flip_cols();
            }
            3 => {
                // counterclockwise
                self.flip_cols();
                self.transpose();
            }
            _ => unreachable!("modulo!"),
        }
        self.edges = Self::edges(&self.grid);
    }

    /// Match with a tile t and returns the matching edges
    fn match_tile(&self, t: &Tile) -> Option<(Edge, Edge)> {
        if let Some((idx, t_idx)) = self.edges[..4].iter().enumerate().find_map(|(idx, e)| {
            t.edges
                .iter()
                .enumerate()
                .find(|(_, t_e)| t_e == &e)
                .map(|(t_idx, _)| (idx, t_idx))
        }) {
            Some((Self::edge(idx), Self::edge(t_idx)))
        } else {
            None
        }
    }

    /// Match a give tile to `self` and rotate/flip it to match orientation
    /// Return the matched edge of `self`
    fn match_and_transform(&self, t: &mut Tile) -> Option<Edge> {
        if let Some((e1, e2)) = self.match_tile(t) {
            match (e1, e2) {
                (Top(_), Top(rev)) | (Bottom(_), Bottom(rev)) => {
                    if !rev {
                        t.flip_rows();
                    } else {
                        t.rotate(2);
                    }
                }
                (Left(_), Left(rev)) | (Right(_), Right(rev)) => {
                    if !rev {
                        t.flip_cols();
                    } else {
                        t.rotate(2);
                    }
                }
                (Top(_), Bottom(rev)) | (Bottom(_), Top(rev)) => {
                    if rev {
                        t.flip_cols();
                    }
                }
                (Left(_), Right(rev)) | (Right(_), Left(rev)) => {
                    if rev {
                        t.flip_rows();
                    }
                }
                (Top(_), Left(rev)) | (Bottom(_), Right(rev)) => {
                    t.rotate(3);
                    if rev {
                        t.flip_cols();
                    }
                }
                (Top(_), Right(rev)) | (Bottom(_), Left(rev)) => {
                    t.rotate(1);
                    if !rev {
                        t.flip_cols();
                    }
                }

                (Left(_), Top(rev)) | (Right(_), Bottom(rev)) => {
                    t.rotate(1);
                    if rev {
                        t.flip_rows();
                    }
                }
                (Left(_), Bottom(rev)) | (Right(_), Top(rev)) => {
                    t.rotate(3);
                    if !rev {
                        t.flip_rows()
                    }
                }
            }
            assert!(matches!(
                self.match_tile(t).unwrap(),
                (Top(false), Bottom(false))
                    | (Bottom(false), Top(false))
                    | (Left(false), Right(false))
                    | (Right(false), Left(false))
            ));
            Some(e1)
        } else {
            None
        }
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
            Some(Tile::new(id, grid))
        })
        .collect()
}

#[aoc(day20, part1)]
pub fn part1(tiles: &[Tile]) -> usize {
    let mut match_count = vec![0; tiles.len()];
    for i in 0..tiles.len() {
        for j in (0..tiles.len()).filter(|j| *j != i) {
            match_count[i] += tiles[i].match_tile(&tiles[j]).is_some() as usize;
        }
    }
    match_count
        .iter()
        .enumerate()
        .filter(|(_idx, cnt)| **cnt == 2)
        .map(|(idx, _)| tiles[idx].id)
        .product()
}

#[derive(Debug, Default)]
pub struct Image {
    map: HashMap<(isize, isize), Tile>,
    pub image: Tile,
}

impl Image {
    fn from_tiles(mut tiles: Vec<Tile>) -> Self {
        let mut image = Image::default();
        let mut queue = vec![(0, 0, 0)];
        let mut seen = Vec::new();
        while let Some((cur_idx, x, y)) = queue.pop() {
            if seen.contains(&cur_idx) {
                continue;
            }
            seen.push(cur_idx);

            let tile = tiles[cur_idx].clone();
            for (idx, next_tile) in tiles
                .iter_mut()
                .enumerate()
                .filter(|(idx, _)| !seen.contains(idx))
            {
                let matched_edge = tile.match_and_transform(next_tile);
                if let Some(edge) = matched_edge {
                    let (x1, y1) = match edge {
                        Top(_) => (x - 1, y),
                        Bottom(_) => (x + 1, y),
                        Left(_) => (x, y - 1),
                        Right(_) => (x, y + 1),
                    };
                    queue.push((idx, x1, y1));
                }
            }
            image.map.insert((x, y), tile);
        }
        image.recenter();
        image.to_tile();
        image
    }

    /// Recenter the Hashmap so that the image is on (0.., 0..)
    fn recenter(&mut self) {
        let &(min_x, min_y) = self.map.keys().min_by_key(|(x, y)| x + y).unwrap();
        let mut map = HashMap::with_capacity(self.map.len());
        for ((x, y), t) in self.map.drain() {
            map.insert((x - min_x, y - min_y), t);
        }
        self.map = map;
    }

    fn to_tile(&mut self) {
        let mut dim = 0;
        while let Some(_) = self.map.get(&(dim as isize, dim as isize)) {
            dim += 1
        }

        let size_wo_edges = TILE_SIZE - 2;

        self.image = Tile {
            grid: vec![Vec::with_capacity(dim * size_wo_edges); dim * size_wo_edges],
            ..Tile::default()
        };
        for x in 0..dim {
            for y in 0..dim {
                let tile = self.map.remove(&(x as isize, y as isize)).unwrap();
                for (idx, row) in tile.grid.iter().skip(1).take(size_wo_edges).enumerate() {
                    self.image.grid[x * size_wo_edges + idx]
                        .extend(row.iter().skip(1).take(size_wo_edges))
                }
            }
        }
    }

    fn count_sea_monsters(&self, sea_monster_coords: &[(usize, usize)]) -> usize {
        let mut count = 0;
        for x in 0..self.image.grid.len() {
            for y in 0..self.image.grid.len() {
                if sea_monster_coords.iter().all(|(dx, dy)| {
                    if let Some(row) = self.image.grid.get(x + dx) {
                        if let Some(true) = row.get(y + dy) {
                            return true;
                        }
                    }
                    false
                }) {
                    count += 1
                }
            }
        }
        count
    }

    fn transform_and_count(&mut self, sea_monster_coords: &[(usize, usize)]) -> usize {
        let rotate_lookup = |s: &mut Self| {
            for _ in 0..3 {
                let cnt = s.count_sea_monsters(sea_monster_coords);
                if cnt > 0 {
                    return cnt;
                }
                s.image.rotate(1);
            }
            0
        };

        let cnt = rotate_lookup(self);
        if cnt > 0 {
            return cnt;
        }
        self.image.flip_cols();

        let cnt = rotate_lookup(self);
        if cnt > 0 {
            return cnt;
        }

        self.image.flip_rows();

        rotate_lookup(self)
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.image)
    }
}

#[aoc(day20, part2)]
pub fn part2(tiles: &[Tile]) -> usize {
    let mut image = Image::from_tiles(tiles.to_vec());
    let sea_monster = "\
-                 # 
#    ##    ##    ###
 #  #  #  #  #  #   ";
    let sea_monster_coords: Vec<(usize, usize)> = sea_monster
        .split("\n")
        .enumerate()
        .flat_map(|(x, s)| {
            s.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(y, _)| (x, y))
        })
        .collect();

    let count = image.transform_and_count(&sea_monster_coords);

    println!("Found {} sea monsters!", count);
    image.to_string().chars().filter(|c| *c == '#').count()
        - count * sea_monster.chars().filter(|c| *c == '#').count()
}

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

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE).unwrap()), 273)
    }
}
