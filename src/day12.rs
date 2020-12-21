use eyre::Result;
use std::{num::ParseIntError, ops::AddAssign, str::FromStr};
use thiserror::Error;
use tracing::info;

#[cfg(test)]
use tracing::debug;

pub fn run() -> Result<()> {
    let input = include_str!("../data/day12.txt");
    let program = Program::parse(input)?;
    let mut ship = Part1ShipComputer::new();

    ship.run(&program.instructions);

    let distance = ship.get_distance();

    info!(distance);

    let mut ship_2 = Part2ShipComputer::new();

    ship_2.run(&program.instructions);

    let distance_2 = ship_2.get_distance();

    info!(distance_2);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    North,
    South,
    West,
    East,
}

impl Facing {
    fn turn_right(self) -> Self {
        match self {
            Facing::North => Facing::East,
            Facing::East => Facing::South,
            Facing::South => Facing::West,
            Facing::West => Facing::North,
        }
    }

    fn turn_left(self) -> Self {
        match self {
            Facing::North => Facing::West,
            Facing::East => Facing::North,
            Facing::South => Facing::East,
            Facing::West => Facing::South,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Part1ShipComputer {
    facing: Facing,
    x: isize,
    y: isize,
}

impl Part1ShipComputer {
    fn new() -> Self {
        Self {
            facing: Facing::East,
            x: 0,
            y: 0,
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter().cloned() {
            match instruction {
                Instruction::North(n) => self.y += n as isize,
                Instruction::East(n) => self.x += n as isize,
                Instruction::South(n) => self.y -= n as isize,
                Instruction::West(n) => self.x -= n as isize,
                Instruction::Left(n) => {
                    for _ in 0..n {
                        self.facing = self.facing.turn_left()
                    }
                }
                Instruction::Right(n) => {
                    for _ in 0..n {
                        self.facing = self.facing.turn_right()
                    }
                }
                Instruction::Forward(n) => match self.facing {
                    Facing::North => self.y += n as isize,
                    Facing::East => self.x += n as isize,
                    Facing::South => self.y -= n as isize,
                    Facing::West => self.x -= n as isize,
                },
            }
        }
    }

    fn get_distance(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn rotate_left(self) -> Self {
        Self {
            x: -self.y,
            y: self.x,
        }
    }

    fn rotate_right(self) -> Self {
        Self {
            x: self.y,
            y: -self.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Part2ShipComputer {
    facing: Facing,
    pos: Point,
    waypoint: Point,
}

impl Part2ShipComputer {
    fn new() -> Self {
        Self {
            facing: Facing::East,
            pos: Point { x: 0, y: 0 },
            waypoint: Point { x: 10, y: 1 },
        }
    }

    fn run(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter().cloned() {
            match instruction {
                Instruction::North(n) => self.waypoint.y += n as isize,
                Instruction::East(n) => self.waypoint.x += n as isize,
                Instruction::South(n) => self.waypoint.y -= n as isize,
                Instruction::West(n) => self.waypoint.x -= n as isize,
                Instruction::Left(n) => {
                    for _ in 0..n {
                        self.waypoint = self.waypoint.rotate_left()
                    }
                }
                Instruction::Right(n) => {
                    for _ in 0..n {
                        self.waypoint = self.waypoint.rotate_right()
                    }
                }
                Instruction::Forward(n) => {
                    for _ in 0..n {
                        self.pos += self.waypoint;
                    }
                }
            }
        }
    }

    fn get_distance(&self) -> isize {
        self.pos.x.abs() + self.pos.y.abs()
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Program {
    instructions: Vec<Instruction>,
}

impl Program {
    fn parse(input: &str) -> Result<Self, InstructionParseError> {
        let instructions = input
            .lines()
            .map(Instruction::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Self { instructions })
    }
}

#[derive(Error, Debug)]
enum InstructionParseError {
    #[error("invalid instruction: '{0}'")]
    InvalidInstruction(String),
    #[error("invalid count")]
    InvalidCount(#[from] ParseIntError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Instruction {
    North(usize),
    East(usize),
    South(usize),
    West(usize),
    Left(usize),
    Right(usize),
    Forward(usize),
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (instruction, count) = s.split_at(1);
        let count = usize::from_str(count)?;
        match instruction {
            "N" => Ok(Instruction::North(count)),
            "E" => Ok(Instruction::East(count)),
            "S" => Ok(Instruction::South(count)),
            "W" => Ok(Instruction::West(count)),
            "L" => Ok(Instruction::Left(count / 90)),
            "R" => Ok(Instruction::Right(count / 90)),
            "F" => Ok(Instruction::Forward(count)),
            _ => Err(InstructionParseError::InvalidInstruction(
                instruction.to_string(),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse_program() -> Result<()> {
        let input = include_str!("../data/day12_test.txt");
        let parsed = Program::parse(input)?;

        debug!(?parsed);

        Ok(())
    }

    #[test]
    fn test_ship() -> Result<()> {
        let input = include_str!("../data/day12_test.txt");
        let program = Program::parse(input)?;
        let mut ship = Part1ShipComputer::new();

        debug!(?ship);

        ship.run(&program.instructions);

        debug!(?ship);

        let distance = ship.get_distance();

        debug!(distance);

        Ok(())
    }

    #[test]
    fn test_ship_2() -> Result<()> {
        let input = include_str!("../data/day12_test.txt");
        let program = Program::parse(input)?;
        let mut ship = Part2ShipComputer::new();

        debug!(?ship);

        ship.run(&program.instructions);

        debug!(?ship);

        let distance = ship.get_distance();

        debug!(distance);

        Ok(())
    }
}
