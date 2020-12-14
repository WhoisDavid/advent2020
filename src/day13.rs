use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone)]
pub struct Bus {
    id: usize,
    offset: usize,
}

#[derive(Debug)]
pub struct Input {
    ts: usize,
    buses: Vec<Bus>,
}

#[aoc_generator(day13)]
pub fn input_parser(input: &str) -> Option<Input> {
    let mut lines = input.lines();
    let ts = lines.next()?.parse::<usize>().ok()?;
    let buses = lines
        .next()?
        .split(",")
        .enumerate()
        .filter_map(|(pos, s)| {
            Some(Bus {
                id: s.parse::<usize>().ok()?,
                offset: pos,
            })
        })
        .collect();
    Some(Input { ts, buses })
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> Option<usize> {
    let a = input
        .buses
        .iter()
        .map(|b| (b.id - input.ts % b.id, b.id))
        .min()?;
    Some(a.0 * a.1)
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> Option<usize> {
    let mut acc = input.buses[0].clone();
    // Solve by finding the (offset + cycle) for each element
    for b in input.buses.iter().skip(1) {
        acc = cycle_euclide(&acc, b)
    }
    // At this stage the offset satisfies all
    Some(acc.offset)
}

// Find the cycle and return as Bus {id: lcm, pos: offset } (such that id*k + offset is )
fn cycle_euclide(a: &Bus, b: &Bus) -> Bus {
    // solve a.id * x + a.pos = b.id * y - b.pos
    // <=> b.id * y - a.id * x  = - (a.pos + b.pos)
    let (x, _y) = diophantine(
        a.id as i128,
        -(b.id as i128),
        -(a.offset as i128 + b.offset as i128),
    );

    let lcm = a.id * b.id; // primes so lcm == product
    let offset = (a.id as i128 * x + a.offset as i128).rem_euclid(lcm as i128) as usize;
    Bus { id: lcm, offset }
}

// Solve linear diophantine equatioon: ax + by = c
fn diophantine(a: i128, b: i128, c: i128) -> (i128, i128) {
    // b*q + r = a
    let q = a.div_euclid(b);
    let r = a.rem_euclid(b);
    // if b divides a then a/b x + y = c/b
    if r == 0 {
        (0, c.div_euclid(b))
    } else {
        // otherwise solve: (bq +r)x + by = c <=>  (qx+y)b + rx = c
        // i = qx +y, j = x => (x, y) = (j, i-qx)
        let (i, j) = diophantine(b, r, c);
        (j, i - q * j)
    }
}

// Unfortunately too slow...
fn _cycle_finder(a: &Bus, b: &Bus) -> Bus {
    let offset = (0..)
        .step_by(a.id)
        .map(|i| i + a.offset)
        .find(|i| b.id - i % b.id == b.offset)
        .unwrap();

    let lcm = a.id * b.id;

    Bus { id: lcm, offset }
}

#[cfg(test)]
mod test_day13 {
    use super::*;

    const TESTCASE: &str = "\
939
7,13,x,x,59,x,31,19";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE).unwrap()), Some(295))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE).unwrap()), Some(1068781))
    }
}
