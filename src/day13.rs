use eyre::{eyre, Result};
use std::{collections::HashMap, str::FromStr};
use tracing::{debug, info};

pub fn run() -> Result<()> {
    let input = include_str!("../data/day13.txt");
    let shedule = Schedule::parse(input)?;
    shedule.find_bus();

    win_contest(input)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Schedule {
    ttl: usize, // time to leave
    bus_ids: Vec<usize>,
}

impl Schedule {
    fn parse(input: &str) -> Result<Self> {
        let mut lines = input.lines();
        let ttl: usize = lines
            .next()
            .ok_or_else(|| eyre!("invalid input: no lines"))?
            .parse()?;

        let bus_ids = lines
            .next()
            .ok_or_else(|| eyre!("invalid input: not enough lines"))?
            .split(',')
            .filter(|&s| s != "x")
            .inspect(|s| debug!(?s))
            .map(usize::from_str)
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Self { ttl, bus_ids })
    }

    fn find_bus(&self) {
        let ttl = self.ttl;
        let omg = self
            .bus_ids
            .iter()
            .cloned()
            .map(|id| {
                let round_trips = ttl / id;
                let wait_min = ((round_trips + 1) * id) - ttl;
                debug!(ttl, id, round_trips, wait_min);
                (wait_min, id)
            })
            .collect::<HashMap<_, _>>();

        let min = omg.keys().min().unwrap();

        let id = omg.get(min).unwrap();

        let answer = min * id;

        info!(?answer);
    }
}

fn win_contest(input: &str) -> Result<()> {
    let mut lines = input.lines();
    let _ = lines.next(); // skip first line

    let mut bus_ids = lines
        .next()
        .ok_or_else(|| eyre!("invalid input: not enough lines"))?
        .split(',')
        .enumerate()
        .filter(|(_, s)| *s != "x")
        .inspect(|(offset, s)| debug!(offset, ?s))
        .map(|(offset, s)| isize::from_str(s).map(|s| (s, offset as isize)))
        .collect::<Result<Vec<(isize, isize)>, _>>()?;

    bus_ids.sort_unstable();

    let mut seen_ids = Vec::new();

    let mut start = 0;
    let mut step = 1;
    for (id, offset) in bus_ids.into_iter() {
        for i in 1..1000 {
            let candidate = start + (i * step);
            debug!(start, i, step, id, candidate);
            if (candidate + offset) % id == 0 {
                start = candidate;
                debug!(state = "found start", start);
                break;
            }
        }

        for i in 1..1000 {
            let candidate = start + (i * step);
            debug!(start, i, step, id, candidate);
            if (candidate + offset) % id == 0 {
                step *= i;
                debug!(state = "found step", step);
                break;
            }
        }

        seen_ids.push((id, offset));

        for (id, offset) in seen_ids.iter().cloned() {
            let candidate = start;
            assert_eq!((candidate + offset) % id, 0);
            let candidate_2 = start + (step);
            assert_eq!((candidate_2 + offset) % id, 0);
        }
    }

    let candidate = start;

    info!(candidate);

    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse_schedule() -> Result<()> {
        let input = include_str!("../data/day13_test.txt");
        debug!(?input);
        let parsed = Schedule::parse(input)?;

        debug!(?parsed);

        Ok(())
    }

    #[test]
    fn test_find_bus() -> Result<()> {
        let input = include_str!("../data/day13_test.txt");
        debug!(?input);
        let parsed = Schedule::parse(input)?;

        debug!(?parsed);

        parsed.find_bus();

        Ok(())
    }

    #[test]
    fn test_henke() -> Result<()> {
        let input = include_str!("../data/day13_test.txt");
        win_contest(input)
    }
}
