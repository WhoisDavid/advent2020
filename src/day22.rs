use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;
use std::collections::VecDeque;

type Int = usize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Decks {
    p1: VecDeque<Int>,
    p2: VecDeque<Int>,
}

pub enum Winner {
    Player1,
    Player2,
}

impl Decks {
    fn winning_score(&self) -> usize {
        let winner = if self.p1.len() == 0 {
            &self.p2
        } else {
            &self.p1
        };
        winner
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, card)| (idx + 1) * card)
            .sum()
    }
}

#[aoc_generator(day22)]
pub fn input_parser(input: &str) -> Decks {
    let mut split = input.split("\n\n");
    let p1 = split.next().unwrap();
    let p2 = split.next().unwrap();
    let deck = |s: &str| {
        s.lines()
            .skip(1)
            .map(|d| d.parse::<Int>().unwrap())
            .collect()
    };
    Decks {
        p1: deck(p1),
        p2: deck(p2),
    }
}

fn play_part1(decks: &mut Decks) -> Winner {
    while let (Some(_), Some(_)) = (decks.p1.front(), decks.p2.front()) {
        let p1 = decks.p1.pop_front().unwrap();
        let p2 = decks.p2.pop_front().unwrap();
        if p1 > p2 {
            decks.p1.extend(&[p1, p2]);
        } else {
            decks.p2.extend(&[p2, p1]);
        }
    }

    if decks.p2.is_empty() {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

#[aoc(day22, part1)]
pub fn part1(decks: &Decks) -> usize {
    let mut decks = decks.clone();
    play_part1(&mut decks);
    decks.winning_score()
}

fn play_part2(decks: &mut Decks) -> Winner {
    let mut history = HashSet::new();

    while let (Some(_), Some(_)) = (decks.p1.front(), decks.p2.front()) {
        // Infinite game protection
        if !history.insert(decks.clone()) {
            return Winner::Player1;
        };

        let p1 = decks.p1.pop_front().unwrap();
        let p2 = decks.p2.pop_front().unwrap();

        let round_winner = if p1 <= decks.p1.len() && p2 <= decks.p2.len() {
            // Recursive game
            let subdecks = &mut Decks {
                p1: decks.p1.iter().copied().take(p1).collect(),
                p2: decks.p2.iter().copied().take(p2).collect(),
            };
            play_part2(subdecks)
        } else if p1 > p2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match round_winner {
            Winner::Player1 => decks.p1.extend(&[p1, p2]),
            Winner::Player2 => decks.p2.extend(&[p2, p1]),
        }
    }

    if decks.p2.is_empty() {
        Winner::Player1
    } else {
        Winner::Player2
    }
}

#[aoc(day22, part2)]
pub fn part2(decks: &Decks) -> usize {
    let mut decks = decks.clone();
    play_part2(&mut decks);
    decks.winning_score()
}

#[cfg(test)]
mod test_day22 {
    use super::*;

    const TESTCASE: &str = "\
Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 306)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 291);
    }
}
