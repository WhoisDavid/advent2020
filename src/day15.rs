use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type Int = u64;

#[aoc_generator(day15)]
pub fn input_parser(input: &str) -> Vec<Int> {
    input.split(",").map(|s| s.parse().expect("int")).collect()
}

fn memory_game(starting_numbers: &[Int], nth: usize) -> Int {
    let mut hm: HashMap<Int, usize> = HashMap::new();
    // Insert starting numbers
    for (turn, &i) in starting_numbers.iter().enumerate() {
        hm.insert(i, turn);
    }

    // Go turn by turn and apply rule...
    // hm.insert(*starting_numbers.last().unwrap(), starting_numbers.len());
    let mut last_spoken = *starting_numbers.last().unwrap();
    for turn in starting_numbers.len()..nth {
        if turn % 100_000 == 0 {
            // Improvised progress bar :-)
            print!(
                "\r[{:<20}] - Turn {}/{}\r",
                "=".repeat(20 * turn / nth),
                turn,
                nth
            );
        }

        // Always insert the current turn for the last spoken number
        let spoken = match hm.insert(last_spoken, turn - 1) {
            // If the last spoken was seen, the new one is the difference of turns
            Some(prev) => (turn - 1 - prev) as Int,
            // Otherwise, the last spoken is 0
            _ => 0,
        };

        last_spoken = spoken;
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
