use aoc_runner_derive::{aoc, aoc_generator};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct Bag {
    color: String,
    n: usize,
}

type OuterToInners = HashMap<String, Vec<Bag>>;

#[aoc_generator(day7)]
pub fn input_parser(input: &str) -> OuterToInners {
    input
        .lines()
        .map(|s| {
            // E.g : shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            let mut split = s.splitn(2, " bags contain ");
            // outer == "shiny gold"
            let outer = split.next().expect("outer bag");
            // inners == "1 dark olive bag, 2 vibrant plum bags."
            let inners = split.next().expect("inner bag(s)");

            // Match inners e.g n="1" and bag_color="dark olive"
            lazy_static! {
                static ref INNER_RE: Regex =
                    Regex::new(r"(?P<n>\d+) (?P<bag_color>.*?) bag[s]?(, |.)").unwrap();
            }
            let inners: Vec<Bag> = INNER_RE
                .captures_iter(inners)
                .map(|c| {
                    let color = c["bag_color"].to_string();
                    let n = c["n"].parse::<usize>().expect("Integer");
                    Bag { color, n }
                })
                .collect();
    
            (outer.to_string(), inners)
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &OuterToInners) -> usize {
    // Reverse map
    let mut inner_to_outers: HashMap<&str, Vec<&str>> = HashMap::new();
    for (outer, inners) in input.iter() {
        for inner in inners {
            inner_to_outers.entry(&inner.color).or_default().push(outer);
        }
    }

    let mut stack = vec!["shiny gold"];
    let mut seen: HashSet<&str> = HashSet::new();
    while let Some(bag) = stack.pop() {
        if seen.insert(bag) {
            if let Some(outers) = inner_to_outers.get(bag) {
                stack.extend(outers);
            }
        }
    }
    seen.len() - 1
}

#[aoc(day7, part2)]
pub fn part2(input: &OuterToInners) -> usize {
    // Count the bags recursively
    fn count_inner_bags(color: &str, map: &OuterToInners) -> usize {
        if let Some(inner_bags) = map.get(color) {
            let mut count = 0;
            for bag in inner_bags {
                count += bag.n * (1 + count_inner_bags(bag.color.as_str(), map));
            }
            count
        } else {
            1
        }
    }

    count_inner_bags("shiny gold", input)
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    const TESTCASE: &str = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 4)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 32);
        let test = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        assert_eq!(part2(&input_parser(test)), 126)
    }
}
