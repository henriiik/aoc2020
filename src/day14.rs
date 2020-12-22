use eyre::{bail, eyre, Result};
use std::{collections::HashMap, str::FromStr};
use tracing::{debug, info};

pub fn run() -> Result<()> {
    let input = include_str!("../data/day14.txt");
    let mut parsed = Program::parse(input)?;

    parsed.run()?;

    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Instruction {
    SetMask { and_mask: u64, or_mask: u64 },
    SetMemory { addr: u64, value: u64 },
}

#[derive(Debug, Clone, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
    memory: HashMap<u64, u64>,
    and_mask: u64,
    or_mask: u64,
}

impl Program {
    fn run(&mut self) -> Result<()> {
        for instrucion in self.instructions.iter().cloned() {
            match instrucion {
                Instruction::SetMask { and_mask, or_mask } => {
                    self.and_mask = and_mask;
                    self.or_mask = or_mask;
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

        Ok(())
    }
}

impl Program {
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
                    debug!("mask:                              {}", second);
                    let (and_mask, or_mask) = second.chars().try_fold(
                        (0u64, 0u64),
                        |(mut and_mask, mut or_mask), c| {
                            and_mask <<= 1;
                            or_mask <<= 1;
                            if c == '1' {
                                or_mask += 1;
                            } else if c == '0' {
                                and_mask += 1;
                            } else if c != 'X' {
                                bail!("invalid input, bad mask: {}", line);
                            }
                            Ok((and_mask, or_mask))
                        },
                    )?;
                    debug!("or:                              {:#038b}", or_mask);
                    debug!("and:                             {:#038b}", and_mask);
                    let and_mask_flipped = u64::MAX ^ and_mask;
                    debug!("or:  {:#038b}", and_mask_flipped);
                    return Ok(Instruction::SetMask {
                        and_mask: and_mask_flipped,
                        or_mask,
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
        Ok(Self {
            instructions,
            memory: HashMap::new(),
            and_mask: 0,
            or_mask: 0,
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day14_test.txt");
        let parsed = Program::parse(input)?;

        debug!(?parsed);

        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        let input = include_str!("../data/day14_test.txt");
        let mut parsed = Program::parse(input)?;

        parsed.run()?;

        Ok(())
    }
}
