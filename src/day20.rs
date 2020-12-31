use eyre::{bail, eyre, Result};
use std::convert::TryFrom;
use std::fmt::Debug;
use tracing::{debug, info, instrument};

#[instrument]
pub fn run() -> Result<()> {
    let tileset = TileSet::parse(include_str!("../data/day20.txt"))?;
    info!(t = ?tileset.tiles.len());
    let corner_product = tileset.find_corner_product();
    info!(corner_product);

    let part_2 = tileset.find_part_2()?;
    info!(part_2);

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn into_index(self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

#[derive(Clone, PartialEq)]
struct Tile {
    data: [bool; 100],
    id: u64,
    edge_ids: [u16; 4],         // up, right, down, left
    flipped_edge_ids: [u16; 4], // up, right, down, left
}
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nid={}", self.id)?;

        for (i, v) in self.data.iter().enumerate() {
            if i % 10 == 0 {
                writeln!(f)?;
            }
            if *v {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
struct RotatedTile<'a> {
    tile: &'a Tile,
    flipped: bool, // we assume that we first rotate to the correct possition and then flip
    rotation: usize,
}

#[derive(Debug, Clone, Copy)]
struct Indexer {
    start: i8,
    row_step: i8,
    col_step: i8,
}

impl Indexer {
    fn new(start: i8, row_step: i8, col_step: i8) -> Self {
        Self {
            start,
            row_step,
            col_step,
        }
    }

    fn index(&self, row: usize, col: usize) -> usize {
        (self.start as isize
            + (self.row_step as isize * row as isize)
            + (self.col_step as isize * col as isize)) as usize
    }
}

impl<'a> RotatedTile<'a> {
    fn new_with_id_in_direction(tile: &'a Tile, id: u16, direction: Direction) -> Self {
        let (flipped, index) = if let Some(index) = tile.edge_ids.iter().position(|&x| x == id) {
            (false, index)
        } else if let Some(index) = tile.flipped_edge_ids.iter().position(|&x| x == id) {
            (true, index)
        } else {
            todo!()
        };

        let direction_index = direction.into_index();
        let is_even_direction = direction_index & 1 == 0;

        // debug!(flipped, index, direction_index, rotation);

        let flipped = flipped != (direction_index > 1);

        let mut rotation = (4 + index - direction_index).rem_euclid(4);
        // if we are flipped, the order of the sides is changed
        if flipped && !is_even_direction {
            rotation = (rotation + 2).rem_euclid(4);
        }

        RotatedTile {
            tile,
            flipped,
            rotation,
        }
    }

    fn get_id_in_direction(&self, direction: Direction) -> u16 {
        let direction_index = direction.into_index();
        let is_even_direction = direction_index & 1 == 0;

        let mut index = (direction_index + self.rotation).rem_euclid(4);

        debug!(
            ?direction_index,
            self.rotation, index, is_even_direction, self.flipped
        );

        // if we are flipped, the order of the sides is changed
        if self.flipped && !is_even_direction {
            index = (index + 2).rem_euclid(4);
        }

        // the down and left directions are normally flipped
        if self.flipped != (direction_index > 1) {
            self.tile.flipped_edge_ids[index]
        } else {
            self.tile.edge_ids[index]
        }
    }

    fn up(&self) -> u16 {
        self.get_id_in_direction(Direction::Up)
    }

    #[cfg(test)]
    fn right(&self) -> u16 {
        self.get_id_in_direction(Direction::Right)
    }

    fn down(&self) -> u16 {
        self.get_id_in_direction(Direction::Down)
    }

    #[cfg(test)]
    fn left(&self) -> u16 {
        self.get_id_in_direction(Direction::Left)
    }

    fn get_indexer(&self) -> Indexer {
        let (start, row_step, col_step) = match (self.rotation, self.flipped) {
            (0, false) => (0, 10, 1),
            (1, false) => (9, -1, 10),
            (2, false) => (99, -10, -1),
            (3, false) => (90, 1, -10),
            (0, true) => (9, 10, -1),
            (1, true) => (99, -1, -10),
            (2, true) => (90, -10, 1),
            (3, true) => (0, 1, 10),
            c => panic!("bad rotation/flipped: {:?}", c),
        };
        Indexer::new(start, row_step, col_step)
    }

    fn get_index_at_pos(&self, row: usize, col: usize) -> usize {
        assert!(row < 10);
        assert!(col < 10);

        self.get_indexer().index(row, col)
    }

    // rotation:
    // 0 -> row: x, col 1..=8
    // 1 -> row 1..=8, col: x
    // 2 -> row: 8 - x, col 8..=1
    // 3 -> row: 8..=1, col: 8 - x
    fn row_iter(&self, row: usize) -> impl Iterator<Item = &bool> {
        let indexer = self.get_indexer();
        (1..=8).map(move |col| &self.tile.data[indexer.index(row, col)])
    }
}

#[instrument(level = "debug", skip(data))]
fn calc_edge_id(data: &[bool], start: isize, step: isize) -> Result<u16> {
    let mut id = 0;

    for n in 0..10 {
        let i = start + (n * step);
        id <<= 1;
        if *data
            .get(usize::try_from(i)?)
            .ok_or_else(|| eyre!("invalid index: {}", i))?
        {
            id += 1;
        }
    }

    Ok(id)
}

impl Tile {
    fn parse(input: &str) -> Result<Tile> {
        let mut iter = input.lines();

        let id = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))?
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse::<u64>()?;

        debug!(id);

        let mut data = [false; 100];

        for (i, row) in iter.enumerate() {
            for (j, c) in row.chars().enumerate() {
                match c {
                    '#' => data[j + (i * 10)] = true,
                    '.' => (),
                    c => bail!("invalid char: {}", c),
                }
            }
        }

        assert_eq!(data.len(), 100);

        let edge_ids = [
            // clock wise
            calc_edge_id(&data, 0, 1)?,    //  up
            calc_edge_id(&data, 9, 10)?,   //  right
            calc_edge_id(&data, 99, -1)?,  //  down
            calc_edge_id(&data, 90, -10)?, //  left
        ];

        let flipped_edge_ids = [
            // counter clock wise
            calc_edge_id(&data, 9, -1)?,   //  up
            calc_edge_id(&data, 99, -10)?, //  right
            calc_edge_id(&data, 90, 1)?,   //  down
            calc_edge_id(&data, 0, 10)?,   //  left
        ];

        Ok(Self {
            id,
            data,
            edge_ids,
            flipped_edge_ids,
        })
    }
}

#[derive(Debug)]
struct TileSet {
    tiles: Vec<Tile>,
}

impl TileSet {
    fn get_grid_size(&self) -> usize {
        for n in 1..=12 {
            if n * n == self.tiles.len() {
                return n;
            }
        }
        panic!("bad grid?!");
    }

    fn parse(input: &str) -> Result<Self> {
        let tiles = input
            .split("\n\n")
            .map(Tile::parse)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { tiles })
    }

    fn find_match(&self, tile: &Tile, edge_id: u16) -> impl Iterator<Item = &Tile> {
        let source_id = tile.id;
        self.tiles.iter().filter(move |&t| {
            t.id != source_id
                && (t.edge_ids.contains(&edge_id) || t.flipped_edge_ids.contains(&edge_id))
        })
    }

    fn is_corner(&self, tile: &Tile) -> bool {
        let mut n = 0;

        if self.find_match(tile, tile.edge_ids[0]).next().is_some() {
            n += 1;
        }

        if self.find_match(tile, tile.edge_ids[1]).next().is_some() {
            n += 1;
        }

        if self.find_match(tile, tile.edge_ids[2]).next().is_some() {
            n += 1;
        }

        if self.find_match(tile, tile.edge_ids[3]).next().is_some() {
            n += 1;
        }

        n == 2
    }

    fn find_corner_product(&self) -> u64 {
        self.tiles
            .iter()
            .filter_map(|t| if self.is_corner(t) { Some(t.id) } else { None })
            .take(4)
            .product()
    }

    fn find_tiles_with_edge_id(&self, skip_id: u64, edge_id: u16) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().filter(move |&t| {
            t.id != skip_id
                && (t.edge_ids.contains(&edge_id) || t.flipped_edge_ids.contains(&edge_id))
        })
    }

    fn find_part_2(&self) -> Result<u64> {
        let start = self
            .tiles
            .iter()
            .find(|t| self.is_corner(t))
            .ok_or_else(|| eyre!("no corners?!"))?;

        debug!(?start);

        let north_matches = self.find_match(start, start.edge_ids[0]).count();
        assert!(north_matches < 2);

        let east_matches = self.find_match(start, start.edge_ids[1]).count();
        assert!(east_matches < 2);

        let south_matches = self.find_match(start, start.edge_ids[2]).count();
        assert!(south_matches < 2);

        let west_matches = self.find_match(start, start.edge_ids[3]).count();
        assert!(west_matches < 2);

        debug!(north_matches, east_matches, south_matches, west_matches);

        let mut row_start_edge_id = match [north_matches, east_matches, south_matches, west_matches]
        {
            [1, 1, 0, 0] => start.edge_ids[3],
            [0, 1, 1, 0] => start.edge_ids[0],
            [0, 0, 1, 1] => start.edge_ids[1],
            [1, 0, 0, 1] => start.edge_ids[2],
            m => bail!("bad corner?! {:?}", m),
        };

        let mut rotated_tiles = vec![];
        let mut row_start_skip_id = 0;

        let n = self.get_grid_size();
        for row in 0..n {
            assert_eq!(
                self.find_tiles_with_edge_id(row_start_skip_id, row_start_edge_id)
                    .count(),
                1
            );
            let tile = self
                .find_tiles_with_edge_id(row_start_skip_id, row_start_edge_id)
                .next()
                .ok_or_else(|| eyre!("bad start ids?!"))?;

            debug!(row, col = 0, id = tile.id);

            let mut current =
                RotatedTile::new_with_id_in_direction(tile, row_start_edge_id, Direction::Up);

            row_start_skip_id = current.tile.id;
            row_start_edge_id = current.get_id_in_direction(Direction::Down);

            rotated_tiles.push(current);

            for col in 1..n {
                let skip_id = current.tile.id;
                let edge_id = current.get_id_in_direction(Direction::Right);

                assert_eq!(
                    self.find_tiles_with_edge_id(skip_id, edge_id).count(),
                    1,
                    "row:{}, col:{}, current: {:?}",
                    row,
                    col,
                    current,
                );
                let tile = self
                    .find_tiles_with_edge_id(skip_id, edge_id)
                    .next()
                    .ok_or_else(|| eyre!("bad ids?!"))?;

                debug!(row, col, id = tile.id);

                current = RotatedTile::new_with_id_in_direction(tile, edge_id, Direction::Left);

                if row > 0 {
                    let above = rotated_tiles.get_mut(((row - 1) * n) + col).unwrap();
                    let above_down = above.down();
                    let current_up = current.up();
                    assert_eq!(above_down, current_up);
                }

                rotated_tiles.push(current);
            }
        }

        debug!(?rotated_tiles);

        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::debug;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day20_test.txt");

        let result = TileSet::parse(input)?;

        debug!(?result);

        Ok(())
    }

    #[test]
    fn test_find_corner_product() -> Result<()> {
        let input = include_str!("../data/day20_test.txt");

        let tiles = TileSet::parse(input)?;

        let corner_product = tiles.find_corner_product();

        assert_eq!(corner_product, 20899048083289);

        Ok(())
    }

    #[test]
    fn test_find_part_2() -> Result<()> {
        let input = include_str!("../data/day20_test.txt");

        let tiles = TileSet::parse(input)?;

        let part_2 = tiles.find_part_2()?;

        assert_eq!(part_2, 20899048083289);

        Ok(())
    }

    #[test]
    fn test_new_with_id_in_direction() {
        let tile = &Tile {
            data: [false; 100],
            id: 1,
            edge_ids: [1, 2, 3, 4],
            flipped_edge_ids: [5, 6, 7, 8],
        };

        let directions = [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ];

        for id in 1..=8 {
            for &d in directions.iter() {
                let got_tile = RotatedTile::new_with_id_in_direction(tile, id, d);
                let got_id = got_tile.get_id_in_direction(d);
                // debug!(?got_tile);
                assert_eq!(
                    id, got_id,
                    "should work for id: {}, direction: {:?}, tile: {:?}",
                    id, d, got_tile
                );
            }
        }
    }

    #[test]
    fn test_get_id_in_direction() {
        let tile = &Tile {
            data: [false; 100],
            id: 1,
            edge_ids: [1, 2, 3, 4],
            flipped_edge_ids: [5, 6, 7, 8],
        };

        let tests = vec![
            (false, 0, 1, 2, 3, 4),
            (false, 1, 2, 3, 4, 1),
            (true, 0, 3, 6, 1, 8),
            (true, 0, 5, 4, 7, 2),
            (true, 0, 7, 8, 5, 6),
            (true, 1, 8, 5, 6, 7),
        ];

        for (flipped, rotation, up, right, down, left) in tests {
            let want = RotatedTile {
                tile,
                flipped,
                rotation,
            };

            assert_eq!(want.get_id_in_direction(Direction::Up), up);
            assert_eq!(want.get_id_in_direction(Direction::Right), right);
            assert_eq!(want.get_id_in_direction(Direction::Down), down);
            assert_eq!(want.get_id_in_direction(Direction::Left), left);
        }
    }

    #[test]
    fn test_simple() -> Result<()> {
        let input = include_str!("../data/day20_test_simple.txt");

        let tile = Tile::parse(input)?;

        debug!(?tile);
        debug!(?tile.edge_ids);
        debug!(?tile.flipped_edge_ids);

        // start
        // up: 2 (256)
        // right: 4 (128)
        // down: (8) 64
        // left: (16) 32
        // ........#.
        // ..........
        // ..........
        // ..........
        // #.........
        // ..........
        // ..........
        // .........#
        // ..........
        // ...#......

        // flip
        // up: flipped, 2 -> 256
        // right: rotated 4 -> 32
        // down: flipped, 64 -> 8
        // left: rotated+flipped -> 32 -> 4
        // .#........
        // ..........
        // ..........
        // ..........
        // .........#
        // ..........
        // ..........
        // #.........
        // ..........
        // ......#...

        // order:  0 -> 1 -> 2 -> 3
        // flip y: 0 -> 3 -> 2 -> 1
        // flip x: 2 -> 1 -> 3 -> 0

        let tests = vec![
            (false, 0, [2, 4, 64, 32]),
            (false, 1, [4, 8, 32, 256]),
            (false, 2, [8, 16, 256, 128]),
            (false, 3, [16, 2, 128, 64]),
            (true, 0, [256, 32, 8, 4]),
            (true, 1, [128, 256, 16, 8]),
            (true, 2, [64, 128, 2, 16]),
            (true, 3, [32, 64, 4, 2]),
        ];
        for (want_flipped, want_rotation, want_ids) in tests {
            let got = RotatedTile::new_with_id_in_direction(&tile, want_ids[0], Direction::Up);
            info!(want_flipped, want_rotation, ?want_ids, ?got);

            assert_eq!(got.flipped, want_flipped);
            assert_eq!(got.rotation, want_rotation);

            assert_eq!(
                got,
                RotatedTile::new_with_id_in_direction(&tile, want_ids[1], Direction::Right)
            );

            assert_eq!(
                got,
                RotatedTile::new_with_id_in_direction(&tile, want_ids[2], Direction::Down)
            );

            assert_eq!(
                got,
                RotatedTile::new_with_id_in_direction(&tile, want_ids[3], Direction::Left)
            );

            assert_eq!(got.up(), want_ids[0], "up");
            assert_eq!(got.right(), want_ids[1], "right");
            assert_eq!(got.down(), want_ids[2], "down");
            assert_eq!(got.left(), want_ids[3], "left");
        }

        Ok(())
    }

    #[test]
    fn test_simple_row() -> Result<()> {
        let input = include_str!("../data/day20_test_simple.txt");

        let tile = Tile::parse(input)?;

        debug!(?tile);
        debug!(?tile.edge_ids);
        debug!(?tile.flipped_edge_ids);

        // start
        // up: 2 (256)
        // right: 4 (128)
        // down: (8) 64
        // left: (16) 32
        // ........#.
        // ..........
        // ..........
        // ..........
        // #.........
        // ..........
        // ..........
        // .........#
        // ..........
        // ...#......

        // flip
        // up: flipped, 2 -> 256
        // right: rotated 4 -> 32
        // down: flipped, 64 -> 8
        // left: rotated+flipped -> 32 -> 4
        // .#........
        // ..........
        // ..........
        // ..........
        // .........#
        // ..........
        // ..........
        // #.........
        // ..........
        // ......#...

        // order:  0 -> 1 -> 2 -> 3
        // flip y: 0 -> 3 -> 2 -> 1
        // flip x: 2 -> 1 -> 3 -> 0

        let tests = vec![
            (false, 0, [2, 4, 64, 32], [0, 1, 2, 9], [10, 11, 12, 19]),
            (false, 1, [4, 8, 32, 256], [9, 19, 29, 99], [8, 18, 28, 98]),
            (
                false,
                2,
                [8, 16, 256, 128],
                [99, 98, 97, 90],
                [89, 88, 87, 80],
            ),
            (false, 3, [16, 2, 128, 64], [90, 80, 70, 0], [91, 81, 71, 1]),
            (true, 0, [256, 32, 8, 4], [9, 8, 7, 0], [19, 18, 17, 10]),
            (true, 1, [128, 256, 16, 8], [99, 89, 79, 9], [98, 88, 78, 8]),
            (
                true,
                2,
                [64, 128, 2, 16],
                [90, 91, 92, 99],
                [80, 81, 82, 89],
            ),
            (true, 3, [32, 64, 4, 2], [0, 10, 20, 90], [1, 11, 21, 91]),
        ];
        for (want_flipped, want_rotation, want_ids, want_indexes_row_0, want_indexes_row_1) in tests
        {
            let got = RotatedTile::new_with_id_in_direction(&tile, want_ids[0], Direction::Up);
            info!(want_flipped, want_rotation, ?want_ids, ?got);

            assert_eq!(got.flipped, want_flipped);
            assert_eq!(got.rotation, want_rotation);
            assert_eq!(got.up(), want_ids[0], "up");
            assert_eq!(got.right(), want_ids[1], "right");
            assert_eq!(got.down(), want_ids[2], "down");
            assert_eq!(got.left(), want_ids[3], "left");

            assert_eq!(got.get_index_at_pos(0, 0), want_indexes_row_0[0]);
            assert_eq!(got.get_index_at_pos(0, 1), want_indexes_row_0[1]);
            assert_eq!(got.get_index_at_pos(0, 2), want_indexes_row_0[2]);
            assert_eq!(got.get_index_at_pos(0, 9), want_indexes_row_0[3]);

            assert_eq!(got.get_index_at_pos(1, 0), want_indexes_row_1[0]);
            assert_eq!(got.get_index_at_pos(1, 1), want_indexes_row_1[1]);
            assert_eq!(got.get_index_at_pos(1, 2), want_indexes_row_1[2]);
            assert_eq!(got.get_index_at_pos(1, 9), want_indexes_row_1[3]);

            let row_0 = got.row_iter(0).collect::<Vec<_>>();
            debug!(?row_0);
        }

        Ok(())
    }
}
