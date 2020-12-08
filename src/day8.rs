use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Acc,
    Jmp,
    Nop,
}
#[derive(Debug, Deserialize, Recap, Copy, Clone)]
#[recap(regex = r"^(?P<op>[a-z]+) (?P<arg>[+-]\d+)$")]
pub struct Instruction {
    op: Operation,
    arg: isize,
}

#[aoc_generator(day8)]
pub fn input_parser(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| s.parse().expect("Instruction!"))
        .collect()
}

fn run_program(program: &[Instruction]) -> (isize, bool) {
    let mut acc = 0;
    let mut idx = 0;
    let mut seen = vec![false; program.len()];
    while idx < program.len() && !seen[idx] {
        seen[idx] = true;
        let cur = &program[idx];
        match &cur.op {
            Operation::Acc => {
                acc += cur.arg;
                idx += 1;
            }
            Operation::Jmp => {
                if cur.arg >= 0 {
                    idx += cur.arg as usize
                } else {
                    idx -= cur.arg.abs() as usize
                }
            }
            Operation::Nop => idx += 1,
        };
    }
    let terminated = idx >= program.len();
    (acc, terminated)
}

#[aoc(day8, part1)]
pub fn part1(input: &[Instruction]) -> isize {
    run_program(input).0
}

fn swap_op(inst: &mut Instruction) {
    inst.op = match inst.op {
        Operation::Jmp => Operation::Nop,
        Operation::Nop => Operation::Jmp,
        Operation::Acc => Operation::Acc,
    }
}

// Super unoptimized but actually worked really fast (127 us..) >.<
#[aoc(day8, part2, BruteForce)]
pub fn part2(program: &[Instruction]) -> Option<isize> {
    for (idx, instruction) in program.iter().enumerate() {
        if instruction.op == Operation::Jmp || instruction.op == Operation::Nop {
            let mut prog = program.to_vec();
            swap_op(&mut prog[idx]);
            let (acc, terminated) = run_program(&prog);
            if terminated {
                return Some(acc);
            }
        }
    }
    None
}

// Still unoptimized but at least removed the the 500 Vec allocation... (79 us)
#[aoc(day8, part2, BruteForceNoAlloc)]
pub fn part2_noalloc(program: &[Instruction]) -> Option<isize> {
    let mut mut_program = program.to_vec();
    let mut previous_swap = None;
    for (idx, instruction) in program.iter().enumerate() {
        if instruction.op == Operation::Jmp || instruction.op == Operation::Nop {
            // Swap back the previous swap
            if let Some(prev) = previous_swap {
                swap_op(&mut mut_program[prev])
            }
            // Swap operation
            swap_op(&mut mut_program[idx]);
            previous_swap = Some(idx);

            let (acc, terminated) = run_program(&mut_program);
            if terminated {
                return Some(acc);
            }
        }
    }
    None
}

#[cfg(test)]
mod test_day8 {
    use super::*;

    const TESTCASE: &str = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), Some(8))
    }
}
