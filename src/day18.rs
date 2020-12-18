use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
pub enum Token {
    Digit(u64),
    Add,
    Mul,
    ParenL,
    ParenR,
}

#[aoc_generator(day18)]
pub fn input_parser(input: &str) -> Vec<Vec<Token>> {
    input
        .lines()
        .map(|expr| {
            expr.chars()
                .filter_map(|c| match c {
                    c if c.is_ascii_digit() => Some(Token::Digit(c.to_digit(10)? as u64)),
                    '+' => Some(Token::Add),
                    '*' => Some(Token::Mul),
                    '(' => Some(Token::ParenL),
                    ')' => Some(Token::ParenR),
                    _ => None,
                })
                .collect()
        })
        .collect()
}

fn match_left_paren(expr: &[Token], right_paren_idx: usize) -> usize {
    let mut acc = 1;
    let mut idx = right_paren_idx;
    while acc > 0 {
        idx -= 1;
        match expr[idx] {
            Token::ParenL => acc -= 1,
            Token::ParenR => acc += 1,
            _ => (),
        }
    }
    idx
}

fn match_right_paren(expr: &[Token], left_paren_idx: usize) -> usize {
    let mut acc = 1;
    let mut idx = left_paren_idx;
    while acc > 0 {
        idx += 1;
        match expr[idx] {
            Token::ParenL => acc += 1,
            Token::ParenR => acc -= 1,
            _ => (),
        }
    }
    idx
}

fn eval(expr: &[Token]) -> u64 {
    let (mut idx, mut out) = match &expr[0] {
        Token::Digit(d) => (1, *d),
        Token::ParenL => {
            let right_idx = match_right_paren(&expr, 0);
            (right_idx + 1, eval(&expr[1..right_idx]))
        }
        _ => panic!(),
    };

    while idx < expr.len() - 1 {
        match (expr[idx], expr[idx + 1]) {
            // A + B
            (Token::Add, Token::Digit(d)) => {
                out += d;
                idx += 2
            }
            // A * B
            (Token::Mul, Token::Digit(d)) => {
                out *= d;
                idx += 2
            }
            // A + (..)
            (Token::Add, Token::ParenL) => {
                let right_paren_idx = match_right_paren(&expr, idx + 1);
                out += eval(&expr[idx + 2..right_paren_idx]);
                idx = right_paren_idx + 1;
            }
            // A * (..)
            (Token::Mul, Token::ParenL) => {
                let right_paren_idx = match_right_paren(&expr, idx + 1);
                out *= eval(&expr[idx + 2..right_paren_idx]);
                idx = right_paren_idx + 1;
            }
            // Unreachable!
            other => unreachable!("{:?}", other),
        }
    }
    out
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec<Token>]) -> u64 {
    input.iter().map(|t| eval(t)).sum()
}

fn insert_precedence_paren(mut expr: Vec<Token>) -> Vec<Token> {
    let mut idx = 0;
    while idx < expr.len() {
        if matches!(&expr[idx], Token::Add) {
            let start = match &expr[idx - 1] {
                Token::Digit(_) => idx - 1,
                Token::ParenR => match_left_paren(&expr, idx - 1),
                other => unreachable!("{:?}", other),
            };
            let end = match &expr[idx + 1] {
                Token::Digit(_) => idx + 2,
                Token::ParenL => 1 + match_right_paren(&expr, idx + 1),
                other => unreachable!("{:?}", other),
            };
            expr.insert(end, Token::ParenR);
            expr.insert(start, Token::ParenL);
            idx += 1;
        }
        idx += 1;
    }
    expr
}

#[aoc(day18, part2)]
pub fn part2(input: &[Vec<Token>]) -> u64 {
    input
        .iter()
        .cloned()
        .map(|t| eval(&insert_precedence_paren(t)))
        .sum()
}

#[cfg(test)]
mod test_day18 {
    use super::*;

    const TESTCASE: &[(&str, u64, u64)] = &[
        ("2 * 3 + (4 * 5)", 26, 46),
        ("5 + (8 * ((3 + 9) + 3) * 4 * 3)", 437, 1445),
        ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240, 669060),
    ];

    #[test]
    fn test_part1() {
        for (s, exp, _) in TESTCASE {
            assert_eq!(part1(&input_parser(*s)), *exp)
        }
    }

    #[test]
    fn test_part2() {
        for (s, _, exp) in TESTCASE {
            assert_eq!(part2(&input_parser(*s)), *exp)
        }
    }
}
