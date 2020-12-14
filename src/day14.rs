use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default, Clone)]
pub struct BitMask {
    mask: u64, // bitmask: 1 wherever there is a 0 or a 1
    val: u64,  // val = value of the input mask as is
    floating: Vec<usize>, // indices of floating bits (== 'X')
}

// Parse "mask = XXXXX.." into a BitMask
impl FromStr for BitMask {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("mask = ").expect("Format: mask = XXX");
        
        // Mask = bitmask: 1 wherever there is a 0 or a 1
        let mask = u64::from_str_radix(&s.replace("0", "1").replace("X", "0"), 2)?;
        
        // val = value of the input mask as is
        let val = u64::from_str_radix(&s.replace("X", "0"), 2)?;
        
        // floating = indices of floating bits (== 'X')
        let floating = s
            .chars()
            .rev()
            .enumerate()
            .filter(|(_, c)| *c == 'X')
            .map(|(idx, _)| idx)
            .collect();

        Ok(BitMask {
            mask,
            val,
            floating,
        })
    }
}

impl BitMask {
    fn apply(&self, value: u64) -> u64 {
        (value & !self.mask) | self.val
    }

    fn decode_address(&self, mut addr: u64) -> Vec<u64> {
        addr |= self.val;

        if self.floating.is_empty() {
            return vec![addr];
        }
        // Loop over the 0..# floating bits to enumerate bit combinations
        let addrs: Vec<u64> = (0..1 << self.floating.len())
            .map(|bits| {
                let mut v = addr;
                for (idx, floating_idx) in self.floating.iter().enumerate() {
                    let floating_bit = (1 << idx) & bits > 0;
                    let mask = 1 << floating_idx;
                    if floating_bit {
                        v |= mask; // set bit
                    } else {
                        v &= !mask; // clear bit
                    }
                }
                v
            })
            .collect();
        addrs
    }
}

#[derive(Debug, Deserialize, Recap, Copy, Clone)]
#[recap(regex = r"^mem\[(?P<addr>\d+)\] = (?P<value>\d+)$")]
pub struct MemWrite {
    addr: u64,
    value: u64,
}

#[derive(Debug)]
pub enum Instruction {
    Mask(BitMask),
    Write(MemWrite),
}

#[aoc_generator(day14)]
pub fn input_parser(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| {
            if s.starts_with("mask") {
                Instruction::Mask(s.parse().expect("Format: mask = XXX"))
            } else {
                Instruction::Write(s.parse().expect("Format: mem[XXX] = XXX"))
            }
        })
        .collect()
}

#[derive(Default)]
struct Program {
    mask: BitMask,
    memory: HashMap<u64, u64>,
}

impl Program {
    fn write(&mut self, write: &MemWrite) {
        self.memory.insert(write.addr, self.mask.apply(write.value));
    }

    fn write_v2(&mut self, write: &MemWrite) {
        let addrs = self.mask.decode_address(write.addr);
        for addr in addrs {
            self.memory.insert(addr, write.value);
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[aoc(day14, part1)]
pub fn part1(instructions: &[Instruction]) -> u64 {
    let mut prog = Program::default();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => prog.mask = m.clone(),
            Instruction::Write(w) => prog.write(w),
        }
    }
    prog.memory_sum()
}

#[aoc(day14, part2)]
pub fn part2(instructions: &[Instruction]) -> u64 {
    let mut prog = Program::default();
    for instruction in instructions {
        match instruction {
            Instruction::Mask(m) => prog.mask = m.clone(),
            Instruction::Write(w) => prog.write_v2(w),
        }
    }
    prog.memory_sum()
}

#[cfg(test)]
mod test_day14 {
    use super::*;

    const TESTCASE_PART_1: &str = "\
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE_PART_1)), 165)
    }

    const TESTCASE_PART_2: &str = "\
mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE_PART_2)), 208)
    }
}
