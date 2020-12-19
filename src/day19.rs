use aoc_runner_derive::aoc;
use regex::Regex;

pub struct Input<'a> {
    rules: Vec<&'a str>,
    messages: Vec<&'a str>,
}

pub fn input_parser(input: &str) -> Input {
    let mut split = input.split("\n\n");
    let rules_str = split.next().unwrap();
    let messages: Vec<&str> = split.next().unwrap().lines().collect();
    // Min size 43 for tests... 
    let mut rules = vec![""; 43.max(rules_str.lines().count())];
    rules_str.lines().for_each(|s| {
        let mut split = s.split(": ");
        let id = split.next().unwrap().parse::<usize>().unwrap();
        let rule = split.next().unwrap();
        rules[id] = rule;
    });
    Input { rules, messages }
}

fn build_regex(rule: &str, rules: &[&str]) -> String {
    match rule {
        "\"a\"" => "a".to_string(),
        "\"b\"" => "b".to_string(),
        "42 | 42 8" => format!("({})+", build_regex(rules[42], rules)),
        "42 31 | 42 11 31" => {
            let r42 = build_regex(rules[42], rules).to_string();
            let r31 = build_regex(rules[31], rules);
            // Rule 11 is equivalent to matching both rule 42 and 31 the same number of times.
            // we match both groups exactly n times using regex: `(grp1){n}(grp2){n}`
            let rule11 = |n: u8| format!("(({0}){{{n}}}({1}){{{n}}})", r42, r31, n = n);
            format!(
                "({}|{}|{}|{}|{})",
                rule11(1), //    match each exactly 1 time
                rule11(2), // or match each exactly 2 times
                rule11(3), // or match each exactly 3 times
                rule11(4), // ...
                rule11(5)
            )
        }
        r if r.contains("|") => {
            let mut split = r.split(" | ");
            let r1 = build_regex(split.next().unwrap(), rules);
            let r2 = build_regex(split.next().unwrap(), rules);
            format!("({}|{})", r1, r2)
        }
        r => r
            .split_whitespace()
            .map(|id_str| id_str.parse::<usize>().unwrap())
            .map(|id| build_regex(rules[id], rules).to_string())
            .collect(),
    }
}

#[aoc(day19, part1, regex)]
pub fn part1(input: &str) -> usize {
    let Input { rules, messages } = input_parser(input);
    let pos = build_regex(rules[0], &rules);
    let re = Regex::new(&format!("^{}$", pos)).unwrap();

    messages.iter().filter(|msg| re.is_match(*msg)).count()
}

#[aoc(day19, part2)]
pub fn part2(input: &str) -> usize {
    let input = input
        .replace("8: 42", "8: 42 | 42 8")
        .replace("11: 42 31", "11: 42 31 | 42 11 31");
    let Input { rules, messages } = input_parser(&input);
    let pos = build_regex(rules[0], &rules);
    let re = Regex::new(&format!("^{}$", pos)).unwrap();

    messages.iter().filter(|msg| re.is_match(*msg)).count()
}

#[cfg(test)]
mod test_day19 {
    use super::*;

    const TESTCASE: &str = "\
0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTCASE), 2)
    }

    const TESTCASE_PART_2: &str = "\
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTCASE_PART_2), 12)
    }
}
