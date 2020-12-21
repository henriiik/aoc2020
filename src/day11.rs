use eyre::{bail, eyre, Result};
use std::{convert::TryFrom, time::Instant};
use tracing::info;

#[cfg(test)]
use tracing::debug;

pub fn run() -> Result<()> {
    let input = include_str!("../data/day11.txt");
    let waiting_area = WaitingArea::parse(input)?;

    Part1::run(waiting_area.clone());
    Part2::run(waiting_area);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct WaitingArea {
    seat_layout: Vec<Vec<SeatState>>,
    width: usize,
    height: usize,
    max_side: isize,
}

impl WaitingArea {
    fn count_occupied(&self) -> usize {
        self.seat_layout
            .iter()
            .cloned()
            .map(|row| {
                row.iter()
                    .cloned()
                    .filter(|&seat| seat == SeatState::Occupied)
                    .count()
            })
            .sum()
    }

    fn parse(input: &str) -> Result<Self> {
        let seat_layout = input
            .trim()
            .lines()
            .map(SeatState::parse_line)
            .collect::<Result<Vec<_>>>()?;

        let height = seat_layout.len();
        let width = seat_layout
            .first()
            .ok_or_else(|| eyre!("no rows in layout!"))?
            .len();

        Ok(Self {
            seat_layout,
            width,
            height,
            max_side: width.max(height) as isize,
        })
    }

    fn get_seat_state(&self, x: usize, y: usize) -> SeatState {
        // debug!(x, y);
        if x >= self.width || y >= self.height {
            SeatState::Unavailable
        } else if let Some(&seat) = self
            .seat_layout
            .get(y as usize)
            .and_then(|row| row.get(x as usize))
        {
            seat
        } else {
            panic!(
                "invalid seat layout! no seat for x: {}, y: {} with width: {}, height: {}",
                x, y, self.width, self.height
            )
        }
    }

    #[cfg(test)]
    fn debug(&self) {
        debug!(?self.width, ?self.height);
        for line in &self.seat_layout {
            debug!(?line);
        }
    }
}

struct Part1;

impl SeatingSystem for Part1 {
    fn num_occupied_needed_for_empty() -> usize {
        4
    }

    fn look_in_direction(
        waiting_area: &WaitingArea,
        pos_x: usize,
        pos_y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> SeatState {
        let x = usize::try_from(pos_x as isize + delta_x);
        let y = usize::try_from(pos_y as isize + delta_y);

        match (x, y) {
            (Ok(x), Ok(y)) => waiting_area.get_seat_state(x, y),
            _ => SeatState::Unavailable, // outside map
        }
    }
}

struct Part2;

impl SeatingSystem for Part2 {
    fn num_occupied_needed_for_empty() -> usize {
        5
    }

    fn look_in_direction(
        waiting_area: &WaitingArea,
        pos_x: usize,
        pos_y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> SeatState {
        for i in 1..waiting_area.max_side {
            let x = usize::try_from(pos_x as isize + (i * delta_x));
            let y = usize::try_from(pos_y as isize + (i * delta_y));

            let (x, y) = match (x, y) {
                (Ok(x), Ok(y)) => (x, y),
                _ => return SeatState::Unavailable, // outside map
            };

            let current = waiting_area.get_seat_state(x, y);
            if current != SeatState::Unavailable {
                return current;
            }
        }

        SeatState::Unavailable
    }
}

trait SeatingSystem {
    fn num_occupied_needed_for_empty() -> usize;

    fn look_in_direction(
        waiting_area: &WaitingArea,
        pos_x: usize,
        pos_y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> SeatState;

    fn occupied_in_direction(
        waiting_area: &WaitingArea,
        pos_x: usize,
        pos_y: usize,
        delta_x: isize,
        delta_y: isize,
    ) -> usize {
        if Self::look_in_direction(waiting_area, pos_x, pos_y, delta_x, delta_y)
            == SeatState::Occupied
        {
            1
        } else {
            0
        }
    }

    fn get_next_seat_state(waiting_area: &WaitingArea, x: usize, y: usize) -> SeatState {
        let current = waiting_area.get_seat_state(x, y);
        if let SeatState::Unavailable = current {
            return SeatState::Unavailable;
        }

        let mut occupied: usize = 0;
        occupied += Self::occupied_in_direction(waiting_area, x, y, 1, 1);
        occupied += Self::occupied_in_direction(waiting_area, x, y, 0, 1);
        occupied += Self::occupied_in_direction(waiting_area, x, y, 1, 0);

        if x > 0 {
            occupied += Self::occupied_in_direction(waiting_area, x, y, -1, 0);
            occupied += Self::occupied_in_direction(waiting_area, x, y, -1, 1);
        }

        if y > 0 {
            occupied += Self::occupied_in_direction(waiting_area, x, y, 0, -1);
            occupied += Self::occupied_in_direction(waiting_area, x, y, 1, -1);
        }

        if x > 0 && y > 0 {
            occupied += Self::occupied_in_direction(waiting_area, x, y, -1, -1);
        }

        if current == SeatState::Empty && occupied == 0 {
            SeatState::Occupied
        } else if current == SeatState::Occupied
            && occupied >= Self::num_occupied_needed_for_empty()
        {
            SeatState::Empty
        } else {
            current
        }
    }

    fn step(waiting_area: &WaitingArea) -> WaitingArea {
        let mut seat_layout = Vec::with_capacity(waiting_area.height);
        for y in 0..waiting_area.height {
            let mut row = Vec::with_capacity(waiting_area.width);
            for x in 0..waiting_area.height {
                row.push(Self::get_next_seat_state(waiting_area, x, y));
            }
            seat_layout.push(row);
        }

        WaitingArea {
            seat_layout,
            width: waiting_area.width,
            height: waiting_area.height,
            max_side: waiting_area.max_side,
        }
    }

    fn run(mut waiting_area: WaitingArea) {
        let now = Instant::now();
        for _ in 0..1000 {
            let next = Self::step(&waiting_area);

            if next == waiting_area {
                break;
            } else {
                waiting_area = next
            }
        }
        let elapsed_ms = now.elapsed().as_millis();

        let occupied = waiting_area.count_occupied();
        info!(occupied, ?elapsed_ms);
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SeatState {
    Empty,
    Occupied,
    Unavailable,
}

impl std::fmt::Debug for SeatState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeatState::Empty => write!(f, "L"),
            SeatState::Occupied => write!(f, "#"),
            SeatState::Unavailable => write!(f, "."),
        }
    }
}

impl SeatState {
    fn parse_line(line: &str) -> Result<Vec<SeatState>> {
        line.chars().map(SeatState::try_from).collect()
    }
}

impl TryFrom<char> for SeatState {
    type Error = eyre::Report;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(SeatState::Empty),
            '#' => Ok(SeatState::Occupied),
            '.' => Ok(SeatState::Unavailable),
            _ => bail!("invalid input: '{}'", value),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use eyre::Result;
    use tracing::trace;

    #[test]
    fn test_parse_input() -> Result<()> {
        let input = include_str!("../data/day11_test_1_step_1.txt");
        let now = Instant::now();
        let parsed = WaitingArea::parse(input)?;
        let elapsed_ms = now.elapsed().as_millis();

        parsed.debug();
        debug!(?elapsed_ms);

        // assert_eq!(parsed, vec![1, 4, 5, 6, 7, 11, 11, 12, 15, 16, 19]);

        Ok(())
    }

    #[test]
    fn test_get_seat_layout() -> Result<()> {
        let input = include_str!("../data/day11_test_1_step_1.txt");
        let now = Instant::now();
        let wa = WaitingArea::parse(input)?;
        let elapsed_ms = now.elapsed().as_millis();

        wa.debug();
        debug!(?elapsed_ms);

        assert_eq!(wa.get_seat_state(0, 0), SeatState::Empty, "0,0");
        assert_eq!(wa.get_seat_state(1, 0), SeatState::Unavailable, "1,0");
        assert_eq!(wa.get_seat_state(0, 1), SeatState::Empty, "0,1");
        assert_eq!(wa.get_seat_state(1, 1), SeatState::Empty, "1,1");

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let now = Instant::now();
        let step_1 = WaitingArea::parse(include_str!("../data/day11_test_1_step_1.txt"))?;
        let elapsed_ms = now.elapsed().as_millis();

        step_1.debug();
        debug!(?elapsed_ms);

        let now = Instant::now();
        let got_step_2 = Part1::step(&step_1);
        let elapsed_ms = now.elapsed().as_millis();

        got_step_2.debug();
        debug!(?elapsed_ms);

        let now = Instant::now();
        let want_step_2 = WaitingArea::parse(include_str!("../data/day11_test_1_step_2.txt"))?;
        let elapsed_ms = now.elapsed().as_millis();

        want_step_2.debug();
        debug!(?elapsed_ms);

        assert_eq!(got_step_2, want_step_2);

        let inputs = vec![
            include_str!("../data/day11_test_1_step_2.txt"),
            include_str!("../data/day11_test_1_step_3.txt"),
            include_str!("../data/day11_test_1_step_4.txt"),
            include_str!("../data/day11_test_1_step_5.txt"),
            include_str!("../data/day11_test_1_step_6.txt"),
        ];

        let mut got = step_1;

        for input in inputs {
            let now = Instant::now();
            got = Part1::step(&got);
            let elapsed_ms = now.elapsed().as_millis();

            got.debug();
            trace!(?elapsed_ms);

            let now = Instant::now();
            let want = WaitingArea::parse(input)?;
            let elapsed_ms = now.elapsed().as_millis();

            want.debug();
            trace!(?elapsed_ms);

            assert_eq!(got, want);
        }

        assert_eq!(Part1::step(&got), got, "should be stable after last step");

        assert_eq!(got.count_occupied(), 37);

        Ok(())
    }

    #[test]
    fn test_look_in_direction() -> Result<()> {
        let input = include_str!("../data/day11_test_2_1.txt");
        let now = Instant::now();
        let wa = WaitingArea::parse(input)?;
        let elapsed_ms = now.elapsed().as_millis();

        wa.debug();
        debug!(?elapsed_ms);

        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, -1, -1),
            SeatState::Occupied,
            "(-1, -1)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, -1, 1),
            SeatState::Occupied,
            "(-1, 1)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, -1, 0),
            SeatState::Occupied,
            "(-1, 0)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, 0, -1),
            SeatState::Occupied,
            "(0, -1)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, 0, 1),
            SeatState::Occupied,
            "(0, 1)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, 1, -1),
            SeatState::Occupied,
            "(1, -1)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, 1, 0),
            SeatState::Occupied,
            "(1, 0)"
        );
        assert_eq!(
            Part2::look_in_direction(&wa, 3, 4, 1, 1),
            SeatState::Occupied,
            "(1, 1)"
        );

        assert_eq!(Part2::get_next_seat_state(&wa, 3, 4), SeatState::Empty);
        assert_eq!(wa.get_seat_state(3, 4), SeatState::Empty);

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let now = Instant::now();
        let step_1 = WaitingArea::parse(include_str!("../data/day11_test_2_step_1.txt"))?;
        let elapsed_ms = now.elapsed().as_millis();

        step_1.debug();
        debug!(?elapsed_ms);

        let now = Instant::now();
        let got_step_2 = Part2::step(&step_1);
        let elapsed_ms = now.elapsed().as_millis();

        // got_step_2.debug();
        debug!(?elapsed_ms);

        let now = Instant::now();
        let want_step_2 = WaitingArea::parse(include_str!("../data/day11_test_2_step_2.txt"))?;
        let elapsed_ms = now.elapsed().as_millis();

        // want_step_2.debug();
        debug!(?elapsed_ms);

        assert_eq!(got_step_2, want_step_2);

        let inputs = vec![
            include_str!("../data/day11_test_2_step_2.txt"),
            include_str!("../data/day11_test_2_step_3.txt"),
            include_str!("../data/day11_test_2_step_4.txt"),
            include_str!("../data/day11_test_2_step_5.txt"),
            include_str!("../data/day11_test_2_step_6.txt"),
            include_str!("../data/day11_test_2_step_7.txt"),
        ];

        let mut got = step_1;

        for input in inputs {
            let now = Instant::now();
            got = Part2::step(&got);
            let elapsed_ms = now.elapsed().as_millis();

            got.debug();
            trace!(?elapsed_ms);

            let now = Instant::now();
            let want = WaitingArea::parse(input)?;
            let elapsed_ms = now.elapsed().as_millis();

            want.debug();
            trace!(?elapsed_ms);

            assert_eq!(got, want);
        }

        assert_eq!(Part2::step(&got), got, "should be stable after last step");

        assert_eq!(got.count_occupied(), 26);

        Ok(())
    }
}
