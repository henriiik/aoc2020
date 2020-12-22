use eyre::{bail, eyre, Result};
use std::{collections::HashMap, str::FromStr};
use tracing::{debug, info, instrument, trace};

pub fn run() -> Result<()> {
    let input = include_str!("../data/day15.txt");
    let nums = parse(input)?;
    let answer = memory_game(nums.clone(), 2020);
    info!(answer);
    let answer = memory_game_2(nums, 30000000);
    info!(answer);

    Ok(())
}

fn parse(input: &str) -> Result<Vec<usize>> {
    input
        .trim()
        .split(',')
        .map(usize::from_str)
        .collect::<Result<_, _>>()
        .map_err(eyre::Report::new)
}

fn memory_game(mut nums: Vec<usize>, end: usize) -> usize {
    let start = nums.len();

    for _ in start..end {
        // debug!(?nums);
        let mut iter = nums.iter().cloned().enumerate().rev();
        let (last_index, last) = iter.next().unwrap();
        let mut iter = iter.filter(|(_, n)| *n == last).map(|(i, _)| i);
        let prev = iter.next();
        match prev {
            Some(prev_index) => nums.push(last_index - prev_index),
            None => nums.push(0),
        }
    }

    *nums.last().unwrap()
}

fn memory_game_2(mut nums: Vec<usize>, end: usize) -> usize {
    debug!(?nums);
    let mut current = nums.pop().unwrap();
    let start = nums.len();

    let mut indexes = nums
        .into_iter()
        .enumerate()
        .map(|(i, n)| (n, i))
        .collect::<HashMap<_, _>>();

    for index in start..(end - 1) {
        // debug!(current);
        let next = match indexes.get(&current) {
            Some(&prev_index) => {
                // debug!(prev_index);
                index - prev_index
            }
            None => 0,
        };
        indexes.insert(current, index);
        current = next;
        // debug!(next);
    }

    current
}

#[cfg(test)]
mod tests {

    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day15_test.txt");
        let nums = parse(input)?;

        debug!(?nums);

        Ok(())
    }

    #[test]
    fn test_memory_game() -> Result<()> {
        let input = include_str!("../data/day15_test.txt");
        let nums = parse(input)?;

        let tests = vec![
            (nums, 436),
            (vec![1, 3, 2], 1),
            (vec![2, 1, 3], 10),
            (vec![1, 2, 3], 27),
            (vec![2, 3, 1], 78),
            (vec![3, 2, 1], 438),
            (vec![3, 1, 2], 1836),
        ];
        for (nums, want) in tests {
            assert_eq!(memory_game(nums, 2020), want);
            assert_eq!(memory_game_2(nums, 2020), want);
        }

        Ok(())
    }

    #[test]
    fn test_memory_game_big() -> Result<()> {
        let tests = vec![
            (vec![0, 3, 6], 175594),
            (vec![1, 3, 2], 2578),
            (vec![2, 1, 3], 3544142),
            (vec![1, 2, 3], 261214),
            (vec![2, 3, 1], 6895259),
            (vec![3, 2, 1], 18),
            (vec![3, 1, 2], 362),
        ];
        for (nums, want) in tests {
            assert_eq!(memory_game_2(nums, 30000000), want);
        }

        Ok(())
    }

    #[test]
    fn test_run_() -> Result<()> {
        let input = include_str!("../data/day15_test.txt");
        let nums = parse(input)?;
        assert_eq!(memory_game(nums, 2020), 436);

        Ok(())
    }
}
