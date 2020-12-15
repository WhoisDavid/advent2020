use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

type Int = u64;

#[aoc_generator(day15)]
pub fn input_parser(input: &str) -> Vec<Int> {
    input.split(",").map(|s| s.parse().expect("int")).collect()
}

fn memory_game(starting_numbers: &[Int], nth: usize) -> Int {
    let mut hm = HashMap::new();
    // Insert starting numbers
    for (turn, &i) in starting_numbers.iter().enumerate() {
        hm.insert(i, Vec::with_capacity(2));
        hm.entry(i).and_modify(|v| v.push(turn));
    }

    // Go turn by turn and apply rule...
    let mut last_spoken = *starting_numbers.last().unwrap();
    for turn in starting_numbers.len()..nth {
        if (turn + 1) % 100_000 == 0 {
            // Improvised progress bar :-)
            print!(
                "\r[{:<20}] - Turn {}/{}\r",
                "=".repeat(20 * (turn + 1) / nth),
                turn,
                nth
            );
        }
        last_spoken = match hm.get_mut(&last_spoken) {
            // If the last spoken was seen twice, the new one is the difference of turns (and we remove the oldest turn)
            Some(prev) if prev.len() == 2 => {
                let p = prev.swap_remove(0);
                (prev[0] - p) as Int
            }
            // Otherwise, the last spoken is 0
            _ => 0,
        };
        // Always insert the current turn for the last spoken number
        hm.entry(last_spoken)
            .or_insert(Vec::with_capacity(2))
            .push(turn);
    }

    // Clear progress bar
    print!("\r{:<100}\r", "");

    last_spoken
}

#[aoc(day15, part1)]
pub fn part1(starting_numbers: &[Int]) -> Int {
    memory_game(starting_numbers, 2020)
}

#[aoc(day15, part2)]
pub fn part2(starting_numbers: &[Int]) -> Int {
    memory_game(starting_numbers, 30_000_000)
}

#[cfg(test)]
mod test_day15 {
    use super::*;

    const TESTCASES: &[(&str, Int, Int)] = &[
        ("0,3,6", 436, 175594),
        ("1,3,2", 1, 2578),
        ("2,1,3", 10, 3544142),
        ("1,2,3", 27, 261214),
        ("2,3,1", 78, 6895259),
        ("3,2,1", 438, 18),
        ("3,1,2", 1836, 362),
    ];

    #[test]
    fn test_part1() {
        for (t, exp, _) in TESTCASES {
            assert_eq!(part1(&input_parser(t)), *exp)
        }
    }

    #[test]
    fn test_part2() {
        for (t, _, exp) in TESTCASES.iter().take(1) {
            assert_eq!(part2(&input_parser(t)), *exp)
        }
    }
}
