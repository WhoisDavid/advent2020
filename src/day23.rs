use aoc_runner_derive::aoc;
use std::convert::TryFrom;

static CUPS: [usize; 9] = [1, 5, 8, 9, 3, 7, 4, 6, 2];

fn play(mut cups: Vec<usize>, rounds: usize) -> Vec<usize> {
    // Push the first element to complete the cycle (last links to first)
    cups.push(cups[0]);

    // Build a "linked list" simuli using an array containing at index i the next cup for cup i
    let mut next_cup = vec![0; cups.len() + 1];
    for &[cup, next] in cups.windows(2).flat_map(<&[usize; 2]>::try_from) {
        next_cup[cup] = next
    }

    // Start with the first cup
    let mut cur = cups[0];
    for _ in 0..rounds {
        // Pick up next 3 cups
        let picked_up = &[
            next_cup[cur],
            next_cup[next_cup[cur]],
            next_cup[next_cup[next_cup[cur]]],
        ];

        // Connect next item:
        // From: cur -> picked up 1 -> picked up 2 -> picked up  3 -> next
        // To: cur -> next
        next_cup[cur] = next_cup[picked_up[2]];

        // Get destination
        let mut destination = if cur == 1 { cups.len() - 1 } else { cur - 1 };
        while picked_up.contains(&&mut destination) {
            if destination == 1 {
                destination = cups.len() - 1
            } else {
                destination -= 1
            }
        }

        // Insert after destination: destination -> picked up 1 and picked up  3 -> next
        // to get: destination -> picked up 1 -> picked up 2 -> picked up  3 -> next
        next_cup[picked_up[2]] = next_cup[destination];
        next_cup[destination] = picked_up[0];

        // Move to the next cup
        cur = next_cup[cur]
    }

    next_cup
}

fn play_part1(cups: Vec<usize>, rounds: usize) -> String {
    let n = cups.len();
    let cups = play(cups, rounds);
    (0..n - 1)
        .scan(1, |next, _| {
            *next = cups[*next];
            Some(*next)
        })
        .map(|s| s.to_string())
        .collect()
}

#[aoc(day23, part1)]
pub fn part1(_: &str) -> String {
    play_part1(CUPS.to_vec(), 100)
}

fn play_part2(mut cups: Vec<usize>) -> usize {
    cups.extend(10..=1_000_000);
    let cups = play(cups, 10_000_000);
    cups[1] * cups[cups[1]]
}

#[aoc(day23, part2)]
pub fn part2(_: &str) -> usize {
    play_part2(CUPS.to_vec())
}

#[cfg(test)]
mod test_day23 {
    use super::*;

    const TESTCASE: [usize; 9] = [3, 8, 9, 1, 2, 5, 4, 6, 7];

    #[test]
    fn test_part1() {
        assert_eq!(play_part1(TESTCASE.to_vec(), 10), "92658374");
        assert_eq!(play_part1(TESTCASE.to_vec(), 100), "67384529");
    }

    #[test]
    fn test_part2() {
        assert_eq!(play_part2(TESTCASE.to_vec()), 149245887792)
    }
}
