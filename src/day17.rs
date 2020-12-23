use eyre::Result;
use std::{
    collections::{BTreeSet, HashSet},
    fmt::Debug,
    time::Instant,
};
use tracing::{debug, info, instrument};

#[instrument]
pub fn run() -> Result<()> {
    let input = include_str!("../data/day17.txt");

    let dimension = PocketDimension::parse(input)?;
    debug!(?dimension);

    let active_cubes = dimension.boot();
    info!(active_cubes);

    let hyper_dimension = HyperDimension::parse(input)?;
    debug!(?hyper_dimension);

    let now = Instant::now();
    let active_hyper_cubes = hyper_dimension.boot();
    let elapsed_micros = now.elapsed().as_micros();
    info!(active_hyper_cubes, ?elapsed_micros);

    assert_eq!(active_hyper_cubes, 2696);

    let hyper_dimension_i8 = HyperDimensionI8::parse(input)?;
    debug!(?hyper_dimension_i8);

    let now = Instant::now();
    let active_hyper_cubes_i8 = hyper_dimension_i8.boot();
    let elapsed_micros = now.elapsed().as_micros();
    info!(active_hyper_cubes_i8, ?elapsed_micros);

    assert_eq!(active_hyper_cubes_i8, 2696);

    let hyper_dimension_i8_array = HyperDimensionI8Array::parse(input)?;
    debug!(?hyper_dimension_i8_array);

    let now = Instant::now();
    let active_hyper_cubes_i8_array = hyper_dimension_i8_array.boot();
    let elapsed_micros = now.elapsed().as_micros();
    info!(active_hyper_cubes_i8_array, ?elapsed_micros);

    assert_eq!(active_hyper_cubes_i8_array, 2696);

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
        for z in self.min_z - 1..=self.max_z + 1 {
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

#[derive(Default, Clone, PartialEq)]
struct HyperDimension {
    active_cubes: BTreeSet<(isize, isize, isize, isize)>,
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    min_z: isize,
    max_z: isize,
    min_w: isize,
    max_w: isize,
}

impl Debug for HyperDimension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                writeln!(f, "\nz={}, w={}", z, w)?;
                for y in self.min_y..=self.max_y {
                    for x in self.min_x..=self.max_x {
                        // write!(f, "\nx:{},y:{},z:{},wat:", x, y, z)?;
                        if self.active_cubes.contains(&(x, y, z, w)) {
                            write!(f, "#")?
                        } else {
                            write!(f, ".")?
                        }
                    }
                    writeln!(f)?;
                }
            }
        }

        Ok(())
    }
}

impl HyperDimension {
    fn parse(input: &str) -> Result<Self> {
        let z = 0;
        let w = 0;
        let v = input
            .trim()
            .lines()
            .enumerate()
            .fold(Self::default(), |pd, (y, line)| {
                line.chars().enumerate().fold(pd, move |mut pd, (x, c)| {
                    if c == '#' {
                        pd.insert(x as isize, y as isize, z as isize, w as isize)
                    }
                    pd
                })
            });

        Ok(v)
    }

    fn insert(&mut self, x: isize, y: isize, z: isize, w: isize) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.min_z = self.min_z.min(z);
        self.max_z = self.max_z.max(z);
        self.min_w = self.min_w.min(w);
        self.max_w = self.max_w.max(w);
        self.active_cubes.insert((x, y, z, w));
    }

    fn step_cube(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        let source = (x, y, z, w);
        let source_active = self.active_cubes.contains(&source);
        let mut active_neighbors = 0;
        for z in z - 1..=z + 1 {
            for w in w - 1..=w + 1 {
                for y in y - 1..=y + 1 {
                    for x in x - 1..=x + 1 {
                        let current = (x, y, z, w);
                        if current == source {
                            continue;
                        } else if self.active_cubes.contains(&current) {
                            active_neighbors += 1;
                        }
                    }
                }
            }
        }

        match (source_active, active_neighbors) {
            (true, active_neighbors) if active_neighbors == 2 || active_neighbors == 3 => true,
            (true, _) => false,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    fn step(&self) -> Self {
        let mut next = Self::default();
        for z in self.min_z - 1..=self.max_z + 1 {
            for w in self.min_w - 1..=self.max_w + 1 {
                for y in self.min_y - 1..=self.max_y + 1 {
                    for x in self.min_x - 1..=self.max_x + 1 {
                        if self.step_cube(x, y, z, w) {
                            next.insert(x, y, z, w);
                        }
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

#[derive(Default, Clone, PartialEq)]
struct HyperDimensionI8 {
    active_cubes: HashSet<(i8, i8, i8, i8)>,
    min_x: i8,
    max_x: i8,
    min_y: i8,
    max_y: i8,
    min_z: i8,
    max_z: i8,
    min_w: i8,
    max_w: i8,
}

impl Debug for HyperDimensionI8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                writeln!(f, "\nz={}, w={}", z, w)?;
                for y in self.min_y..=self.max_y {
                    for x in self.min_x..=self.max_x {
                        // write!(f, "\nx:{},y:{},z:{},wat:", x, y, z)?;
                        if self.active_cubes.contains(&(x, y, z, w)) {
                            write!(f, "#")?
                        } else {
                            write!(f, ".")?
                        }
                    }
                    writeln!(f)?;
                }
            }
        }

        Ok(())
    }
}

impl HyperDimensionI8 {
    fn parse(input: &str) -> Result<Self> {
        let z = 0;
        let w = 0;
        let v = input
            .trim()
            .lines()
            .enumerate()
            .fold(Self::default(), |pd, (y, line)| {
                line.chars().enumerate().fold(pd, move |mut pd, (x, c)| {
                    if c == '#' {
                        pd.insert(x as i8, y as i8, z as i8, w as i8)
                    }
                    pd
                })
            });

        Ok(v)
    }

    fn insert(&mut self, x: i8, y: i8, z: i8, w: i8) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.min_z = self.min_z.min(z);
        self.max_z = self.max_z.max(z);
        self.min_w = self.min_w.min(w);
        self.max_w = self.max_w.max(w);
        self.active_cubes.insert((x, y, z, w));
    }

    fn step_cube(&self, x: i8, y: i8, z: i8, w: i8) -> bool {
        let source = (x, y, z, w);
        let source_active = self.active_cubes.contains(&source);
        let mut active_neighbors = 0;
        for z in z - 1..=z + 1 {
            for w in w - 1..=w + 1 {
                for y in y - 1..=y + 1 {
                    for x in x - 1..=x + 1 {
                        let current = (x, y, z, w);
                        if current == source {
                            continue;
                        } else if self.active_cubes.contains(&current) {
                            active_neighbors += 1;
                        }
                    }
                }
            }
        }

        match (source_active, active_neighbors) {
            (true, active_neighbors) if active_neighbors == 2 || active_neighbors == 3 => true,
            (true, _) => false,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    fn step(&self) -> Self {
        let mut next = Self::default();
        for z in self.min_z - 1..=self.max_z + 1 {
            for w in self.min_w - 1..=self.max_w + 1 {
                for y in self.min_y - 1..=self.max_y + 1 {
                    for x in self.min_x - 1..=self.max_x + 1 {
                        if self.step_cube(x, y, z, w) {
                            next.insert(x, y, z, w);
                        }
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

#[derive(Default, Clone, PartialEq)]
struct HyperDimensionI8Array {
    active_cubes: HashSet<[i8; 4]>,
    min_x: i8,
    max_x: i8,
    min_y: i8,
    max_y: i8,
    min_z: i8,
    max_z: i8,
    min_w: i8,
    max_w: i8,
}

impl Debug for HyperDimensionI8Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for w in self.min_w..=self.max_w {
            for z in self.min_z..=self.max_z {
                writeln!(f, "\nz={}, w={}", z, w)?;
                for y in self.min_y..=self.max_y {
                    for x in self.min_x..=self.max_x {
                        // write!(f, "\nx:{},y:{},z:{},wat:", x, y, z)?;
                        if self.active_cubes.contains(&[x, y, z, w]) {
                            write!(f, "#")?
                        } else {
                            write!(f, ".")?
                        }
                    }
                    writeln!(f)?;
                }
            }
        }

        Ok(())
    }
}

impl HyperDimensionI8Array {
    fn parse(input: &str) -> Result<Self> {
        let z = 0;
        let w = 0;
        let v = input
            .trim()
            .lines()
            .enumerate()
            .fold(Self::default(), |pd, (y, line)| {
                line.chars().enumerate().fold(pd, move |mut pd, (x, c)| {
                    if c == '#' {
                        pd.insert(x as i8, y as i8, z as i8, w as i8)
                    }
                    pd
                })
            });

        Ok(v)
    }

    fn insert(&mut self, x: i8, y: i8, z: i8, w: i8) {
        self.min_x = self.min_x.min(x);
        self.max_x = self.max_x.max(x);
        self.min_y = self.min_y.min(y);
        self.max_y = self.max_y.max(y);
        self.min_z = self.min_z.min(z);
        self.max_z = self.max_z.max(z);
        self.min_w = self.min_w.min(w);
        self.max_w = self.max_w.max(w);
        self.active_cubes.insert([x, y, z, w]);
    }

    fn step_cube(&self, x: i8, y: i8, z: i8, w: i8) -> bool {
        let source = [x, y, z, w];
        let source_active = self.active_cubes.contains(&source);
        let mut active_neighbors = 0;
        for z in z - 1..=z + 1 {
            for w in w - 1..=w + 1 {
                for y in y - 1..=y + 1 {
                    for x in x - 1..=x + 1 {
                        let current = [x, y, z, w];
                        if current == source {
                            continue;
                        } else if self.active_cubes.contains(&current) {
                            active_neighbors += 1;
                        }
                    }
                }
            }
        }

        match (source_active, active_neighbors) {
            (true, active_neighbors) if active_neighbors == 2 || active_neighbors == 3 => true,
            (true, _) => false,
            (false, 3) => true,
            (false, _) => false,
        }
    }

    fn step(&self) -> Self {
        let mut next = Self::default();
        for z in self.min_z - 1..=self.max_z + 1 {
            for w in self.min_w - 1..=self.max_w + 1 {
                for y in self.min_y - 1..=self.max_y + 1 {
                    for x in self.min_x - 1..=self.max_x + 1 {
                        if self.step_cube(x, y, z, w) {
                            next.insert(x, y, z, w);
                        }
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
    use super::*;

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

    #[test]
    fn test_parse_hyper() -> Result<()> {
        let input = include_str!("../data/day17_test.txt");

        let d = HyperDimension::parse(input)?;
        // dbg!(d.min_x, d.max_x, d.min_y, d.max_y, d.min_z, d.max_z, d.min_w, d.max_w,);
        debug!(?d);

        let d1 = d.step();
        // dbg!(d1.min_x, d1.max_x, d1.min_y, d1.max_y, d1.min_z, d1.max_z, d1.min_w, d1.max_w,);
        debug!(?d1);

        // assert_eq!(
        //     d,
        //     PocketDimension {
        //         active_cubes: [(0, 2, 0), (1, 0, 0), (1, 2, 0), (2, 1, 0), (2, 2, 0)]
        //             .iter()
        //             .cloned()
        //             .collect(),
        //         min_x: 0,
        //         max_x: 2,
        //         min_y: 0,
        //         max_y: 2,
        //         min_z: 0,
        //         max_z: 0
        //     }
        // );

        // assert_eq!(
        //     d1,
        //     PocketDimension {
        //         active_cubes: [
        //             (0, 1, -1),
        //             (0, 1, 0),
        //             (0, 1, 1),
        //             (1, 2, 0),
        //             (1, 3, -1),
        //             (1, 3, 0),
        //             (1, 3, 1),
        //             (2, 1, 0),
        //             (2, 2, -1),
        //             (2, 2, 0),
        //             (2, 2, 1)
        //         ]
        //         .iter()
        //         .cloned()
        //         .collect(),
        //         min_x: 0,
        //         max_x: 2,
        //         min_y: 0,
        //         max_y: 3,
        //         min_z: -1,
        //         max_z: 1
        //     }
        // );

        // let d2 = d1.step();
        // debug!(?d2);

        // let d3 = d2.step();
        // // debug!(?d3);

        // let d4 = d3.step();
        // // debug!(?d4);

        // let d5 = d4.step();
        // // debug!(?d5);

        // let d6 = d5.step();
        // // debug!(?d6);

        // let active_cube_count = d6.active_cubes.len();
        // debug!(active_cube_count);
        // assert_eq!(active_cube_count, 112);

        assert_eq!(d.boot(), 848);

        Ok(())
    }
}
