use std::{
    collections::{BTreeSet, HashMap},
    fmt::Debug,
    hash::Hash,
    ops::{AddAssign, Range},
};

use eyre::{eyre, Result};
// use std::{
//     collections::{HashMap, BTreeSet},
//     ops::RangeInclusive,
//     str::FromStr,
// };
use tracing::{debug, info, instrument};

#[instrument]
pub fn run() -> Result<()> {
    let input = include_str!("../data/day17.txt");
    let dimension = PocketDimension::parse(input)?;
    debug!(?dimension);
    let active_cubes = dimension.boot();
    info!(active_cubes);
    Ok(())
}

#[derive(Default, Clone, PartialEq)]
struct PocketDimension {
    active_cubes: BTreeSet<(isize, isize, isize)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
}

impl Debug for PocketDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for z in self.min_z..=self.max_z {
            writeln!(f, "\nz={}", z)?;
            for y in self.min_y..=self.max_y {
                for x in self.min_x..=self.max_x {
                    // write!(f, "\nx:{},y:{},z:{},wat:", x, y, z)?;
                    if self.active_cubes.contains(&(x, y, z)) {
                        write!(f, "#")?
                    } else {
                        write!(f, ".")?
                    }
                }
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl PocketDimension {
    fn parse(input: &str) -> Result<Self> {
        let z = 0;
        let v = input
            .trim()
            .lines()
            .enumerate()
            .fold(Self::default(), |pd, (y, line)| {
                line.chars().enumerate().fold(pd, move |mut pd, (x, c)| {
                    if c == '#' {
                        pd.insert(x as isize, y as isize, z as isize)
                    }
                    pd
                })
            });

        Ok(v)
    }

    fn insert(&mut self, x: isize, y: isize, z: isize) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.min_z = self.min_z.min(z);
        self.max_z = self.max_z.max(z);
        self.active_cubes.insert((x, y, z));
    }

    fn step_cube(&self, x: isize, y: isize, z: isize) -> bool {
        let source = (x, y, z);
        let source_active = self.active_cubes.contains(&source);
        let mut active_neighbors = 0;
        for z in z - 1..=z + 1 {
            for y in y - 1..=y + 1 {
                for x in x - 1..=x + 1 {
                    let current = (x, y, z);
                    if current == source {
                        continue;
                    } else if self.active_cubes.contains(&current) {
                        active_neighbors += 1;
                    }
                }
            }
        }

        match (source_active, active_neighbors) {
            (true, n) if n == 2 || n == 3 => true,
            (true, _) => false,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    fn step(&self) -> Self {
        let mut next = PocketDimension::default();
        for z in self.min_z - 1..=self.max_z + 13 {
            for y in self.min_y - 1..=self.max_y + 1 {
                for x in self.min_x - 1..=self.max_x + 1 {
                    if self.step_cube(x, y, z) {
                        next.insert(x, y, z);
                    }
                }
            }
        }
        next
    }

    fn boot(self) -> usize {
        let mut next = self;
        for _ in 0..6 {
            next = next.step()
        }
        next.active_cubes.len()
    }
}

#[cfg(test)]
mod tests {

    // use super::*;
    use eyre::Result;
    use tracing::debug;

    use super::PocketDimension;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day17_test.txt");

        let d = PocketDimension::parse(input)?;
        debug!(?d);

        let d1 = d.step();
        debug!(?d1);

        assert_eq!(
            d,
            PocketDimension {
                active_cubes: [(0, 2, 0), (1, 0, 0), (1, 2, 0), (2, 1, 0), (2, 2, 0)]
                    .iter()
                    .cloned()
                    .collect(),
                min_x: 0,
                max_x: 2,
                min_y: 0,
                max_y: 2,
                min_z: 0,
                max_z: 0
            }
        );

        assert_eq!(
            d1,
            PocketDimension {
                active_cubes: [
                    (0, 1, -1),
                    (0, 1, 0),
                    (0, 1, 1),
                    (1, 2, 0),
                    (1, 3, -1),
                    (1, 3, 0),
                    (1, 3, 1),
                    (2, 1, 0),
                    (2, 2, -1),
                    (2, 2, 0),
                    (2, 2, 1)
                ]
                .iter()
                .cloned()
                .collect(),
                min_x: 0,
                max_x: 2,
                min_y: 0,
                max_y: 3,
                min_z: -1,
                max_z: 1
            }
        );

        let d2 = d1.step();
        // debug!(?d2);

        let d3 = d2.step();
        // debug!(?d3);

        let d4 = d3.step();
        // debug!(?d4);

        let d5 = d4.step();
        // debug!(?d5);

        let d6 = d5.step();
        // debug!(?d6);

        let active_cube_count = d6.active_cubes.len();
        debug!(active_cube_count);
        assert_eq!(active_cube_count, 112);

        assert_eq!(d.boot(), 112);

        Ok(())
    }
}
