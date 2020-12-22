use eyre::{bail, eyre, Result};
use std::{collections::HashMap, str::FromStr};
use tracing::{debug, info, instrument, trace};

pub fn run() -> Result<()> {
    let input = include_str!("../data/day14.txt");
    let program = Program::parse(input)?;
    let mut computer_1 = ComputerPart1::default();
    computer_1.run(&program)?;

    let mut computer_2 = ComputerPart2::default();
    computer_2.run(&program)?;

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    SetMask {
        ones_mask: u64,
        zeroes_mask: u64,
        x_mask: u64,
    },
    SetMemory {
        addr: u64,
        value: u64,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    #[instrument(skip(input))]
    fn parse(input: &str) -> Result<Self> {
        let instructions = input
            .lines()
            .map(|line| {
                let mut split = line.split(" = ");

                let first = split.next().ok_or_else(|| {
                    eyre!("invalid input, first split should always succeed: {}", line)
                })?;
                let second = split.next().ok_or_else(|| {
                    eyre!(
                        "invalid input, should be possible to split on ' = ': {}",
                        line
                    )
                })?;

                if first == "mask" {
                    debug!("mask:   {}", second);
                    let (ones_mask, zeroes_mask, x_mask) = second.chars().try_fold(
                        (0u64, 0u64, 0u64),
                        |(mut ones_mask, mut zeroes_mask, mut x_mask), c| {
                            ones_mask <<= 1;
                            zeroes_mask <<= 1;
                            x_mask <<= 1;
                            if c == '1' {
                                ones_mask += 1;
                            } else if c == '0' {
                                zeroes_mask += 1;
                            } else if c == 'X' {
                                x_mask += 1;
                            } else {
                                bail!("invalid input, bad mask: {}", line);
                            }
                            Ok((ones_mask, zeroes_mask, x_mask))
                        },
                    )?;
                    debug!("zeroes: {:036b}", zeroes_mask);
                    debug!("ones:   {:036b}", ones_mask);
                    debug!("x:      {:036b}", x_mask);
                    return Ok(Instruction::SetMask {
                        ones_mask,
                        zeroes_mask,
                        x_mask,
                    });
                } else if let Some(first) =
                    first.strip_prefix("mem[").and_then(|f| f.strip_suffix("]"))
                {
                    let addr = u64::from_str(first)?;
                    let value = u64::from_str(second)?;
                    return Ok(Instruction::SetMemory { addr, value });
                }

                bail!("invalid input!: {}", line)
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { instructions })
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ComputerPart1 {
    memory: HashMap<u64, u64>,
    and_mask: u64,
    or_mask: u64,
}

impl ComputerPart1 {
    #[instrument(skip(self, program))]
    fn run(&mut self, program: &Program) -> Result<u64> {
        for instrucion in program.instructions.iter().cloned() {
            match instrucion {
                Instruction::SetMask {
                    ones_mask,
                    zeroes_mask,
                    ..
                } => {
                    let zeroes_mask_flipped = u64::MAX ^ zeroes_mask;
                    self.and_mask = zeroes_mask_flipped;
                    self.or_mask = ones_mask;
                }
                Instruction::SetMemory { addr, mut value } => {
                    debug!(value);
                    value |= self.or_mask;
                    value &= self.and_mask;
                    debug!(value);
                    self.memory.insert(addr, value);
                    // let value =
                }
            }
        }

        debug!(?self.memory);

        let sum: u64 = self.memory.values().sum();

        info!(sum);

        Ok(sum)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct ComputerPart2 {
    memory: HashMap<u64, u64>,
    ones_mask: u64,
    flipped_x_mask: u64,
    floating_masks: Vec<u64>,
}

impl ComputerPart2 {
    #[instrument(skip(self, program))]
    fn run(&mut self, program: &Program) -> Result<u64> {
        for instruction in program.instructions.iter().cloned() {
            debug!(?instruction);
            match instruction {
                Instruction::SetMask {
                    ones_mask, x_mask, ..
                } => {
                    // create a mask that alows us to "force" 1s into the positions marked by ones
                    self.ones_mask = ones_mask;

                    // creata mask that allows us to "force" 0s into the positions marked by Xs
                    self.flipped_x_mask = u64::MAX ^ x_mask;

                    // make sure the masks from the last iteration are removed
                    self.floating_masks.clear();

                    debug!("x_mask: {:036b}", x_mask);

                    // for easier looping we save the number of Xs we have in our mask
                    let x_count = x_mask.count_ones();
                    debug!("x_count: {}", x_count);

                    // for easier looping we save the number of masks that are needed
                    let mask_count = 2u64.pow(x_count);
                    debug!("mask_count: {}", mask_count);

                    // the positions of the Xs are saved so we can use them to generate masks
                    let mut shifts = Vec::new();
                    let mut x_mask_tmp = x_mask;
                    for _ in 0..x_count {
                        debug!("x_mask_tmp: {:036b}", x_mask_tmp);
                        let i = x_mask_tmp.trailing_zeros();
                        debug!(i);
                        debug!("i: {:036b}", i);

                        shifts.push(i as u64);
                        x_mask_tmp >>= i;
                        x_mask_tmp &= u64::MAX - 1; // zero out last bit
                    }
                    debug!(?shifts);
                    assert_eq!(x_mask_tmp.count_ones(), 0);

                    // each i between 0 and mask_count is a uniqe bit pattern that we use to create
                    // a mask by shifting the bits in the pattern to the positions we have saved.
                    for i in (0..mask_count).rev() {
                        debug!("");
                        debug!("i:      {:036b}", i);
                        debug!("x_mask: {:036b}", x_mask);
                        let mut mask = 0u64;
                        let mut i = i as u64;
                        for shift in shifts.iter().cloned().rev() {
                            mask += i & 1;
                            mask <<= shift;
                            i >>= 1;
                        }
                        debug!("mask:   {:036b}", mask);
                        self.floating_masks.push(mask);
                    }

                    self.floating_masks.sort_unstable();
                }
                Instruction::SetMemory { mut addr, value } => {
                    debug!("");
                    debug!("1 addr: {:036b}", addr);
                    addr |= self.ones_mask;
                    debug!("2 addr: {:036b}", addr);
                    addr &= self.flipped_x_mask;
                    debug!("3 addr: {:036b}", addr);

                    for mask in self.floating_masks.iter().cloned() {
                        debug!("");
                        debug!("mask:   {:036b}", mask);
                        let addr = addr | mask;
                        debug!("4 addr: {:036b}", addr);
                        self.memory.insert(addr, value);
                    }
                }
            }
        }

        trace!(?self.memory);

        let sum: u64 = self.memory.values().sum();

        info!(sum);

        Ok(sum)
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day14_test.txt");
        let program = Program::parse(input)?;

        debug!(?program);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        let input = include_str!("../data/day14_test.txt");
        let program = Program::parse(input)?;
        let mut computer = ComputerPart1::default();

        let sum = computer.run(&program)?;

        assert_eq!(sum, 165);

        Ok(())
    }

    #[test]
    fn test_run_2() -> Result<()> {
        let input = include_str!("../data/day14_test_2.txt");
        let program = Program::parse(input)?;
        let mut computer = ComputerPart2::default();

        let sum = computer.run(&program)?;

        assert_eq!(sum, 208);

        Ok(())
    }

    #[test]
    fn test_run_3() -> Result<()> {
        let input = include_str!("../data/day14_test_3.txt");
        let program = Program::parse(input)?;
        let mut computer = ComputerPart2::default();

        let sum = computer.run(&program)?;

        assert_eq!(sum, 20848636352);

        Ok(())
    }
}
