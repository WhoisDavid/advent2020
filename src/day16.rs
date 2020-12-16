use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;

type Int = usize;
type Ticket = Vec<Int>;

pub struct Input {
    rules: Vec<Rule>,
    ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

#[derive(Debug, Deserialize, Recap, Clone)]
#[recap(regex = r"^(?P<name>.*?): (?P<r1>\d+)-(?P<r2>\d+) or (?P<r3>\d+)-(?P<r4>\d+)$")]
pub struct Rule {
    name: String,
    r1: Int,
    r2: Int,
    r3: Int,
    r4: Int,
}

impl Rule {
    fn is_valid(&self, val: Int) -> bool {
        (self.r1..=self.r2).contains(&val) || (self.r3..=self.r4).contains(&val)
    }
}

#[aoc_generator(day16)]
pub fn input_parser(input: &str) -> Input {
    let mut split = input.split("\nyour ticket:\n");
    let rules = split.next().unwrap().to_string();
    let rules = rules
        .lines()
        .map(|s| s.parse::<Rule>().expect("name: 0-1 or 2-3"))
        .collect::<Vec<_>>();

    let tickets = split.next().unwrap();

    let mut split = tickets.split("\nnearby tickets:\n");
    let ticket: Ticket = split
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|s| s.parse::<Int>().expect("Integer"))
        .collect();
    let nearby_tickets: Vec<Ticket> = split
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|s| {
            s.split(",")
                .map(|s| s.parse::<Int>().expect("Integer"))
                .collect()
        })
        .collect();

    Input {
        rules,
        ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> usize {
    input
        .ticket
        .iter()
        .chain(input.nearby_tickets.iter().flat_map(|t| t))
        .filter(|val| !input.rules.iter().any(|r| r.is_valid(**val)))
        .sum()
}

fn map_fields(input: &Input) -> HashMap<String, usize> {
    let valid_tickets: Vec<&Ticket> = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|val| input.rules.iter().any(|r| r.is_valid(*val)))
        })
        .collect();

    let mut rules = input.rules.clone();
    // Map all valid fields for each rule
    let mut rules_valid_fields_indices = input
        .rules
        .iter()
        .map(|r| {
            (0..input.ticket.len())
                .filter(|&field_idx| {
                    valid_tickets
                        .iter()
                        .all(|ticket| r.is_valid(ticket[field_idx]))
                })
                .collect::<HashSet<_>>()
        })
        .collect::<Vec<_>>();

    // Map a field name to its index
    let mut mapping = HashMap::new();

    // Greedy allocate fields
    // If there is only one option take it and continue
    while let Some(idx) = rules_valid_fields_indices
        .iter()
        .enumerate()
        .find_map(|(idx, vf)| if vf.len() == 1 { Some(idx) } else { None })
    {
        // Remove matched rule
        let Rule { name, .. } = rules.swap_remove(idx);
        // Get field
        let mut valid_field = rules_valid_fields_indices.swap_remove(idx);
        let field_idx = valid_field.drain().next().unwrap();

        // Insert field mapping
        mapping.insert(name, field_idx);

        // Remove allocated field from other rules
        rules_valid_fields_indices.iter_mut().for_each(|v| {
            v.remove(&field_idx);
        })
    }

    mapping
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> usize {
    let mapping = map_fields(input);
    // Filter "departures", map the field value from ticket and multiply!
    mapping
        .iter()
        .filter(|(name, _)| name.starts_with("departure"))
        .map(|(_, field_idx)| input.ticket[*field_idx])
        .product()
}

#[cfg(test)]
mod test_day16 {
    use super::*;

    const TESTCASE_PART_1: &str = "\
class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE_PART_1)), 71)
    }

    const TESTCASE_PART_2: &str = "\
class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";

    #[test]
    fn test_part2() {
        let exp: HashMap<String, usize> = vec![("row", 0), ("class", 1), ("seat", 2)]
            .into_iter()
            .map(|(s, u)| (s.to_string(), u))
            .collect();

        assert_eq!(map_fields(&input_parser(TESTCASE_PART_2)), exp)
    }
}
