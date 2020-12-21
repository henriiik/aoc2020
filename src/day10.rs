use eyre::{bail, Result};
use std::{collections::HashMap, str::FromStr, time::Instant};
use tracing::{debug, info, instrument};

pub fn run() -> Result<()> {
    let input = include_str!("../data/day10.txt");
    let mut parsed = parse_input(input)?;
    let diffs = find_diffs(&mut parsed)?;
    let answer = diffs.0 * diffs.2;

    info!(?diffs, answer);

    let now = Instant::now();
    let num_arrangements = find_num_arrangements(parsed);
    let elapsed_ms = now.elapsed().as_millis();

    info!(num_arrangements, ?elapsed_ms);

    Ok(())
}

#[instrument]
fn parse_input(input: &str) -> Result<Vec<usize>> {
    Ok(input
        .trim()
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<usize>, _>>()?)
}

fn find_diffs(input: &mut [usize]) -> Result<(usize, usize, usize)> {
    input.sort_unstable();

    let mut last = 0; // outlet is 0

    let mut num_diff_1 = 0;
    let mut num_diff_2 = 0;
    let mut num_diff_3 = 0;

    for current in input.iter().cloned() {
        let diff = current - last;
        match diff {
            1 => num_diff_1 += 1,
            2 => num_diff_2 += 1,
            3 => num_diff_3 += 1,
            // 0 => (),
            _ => bail!(
                "invalid input, diff: {}, current: {}, last: {}",
                diff,
                current,
                last
            ),
        }
        last = current;
    }

    num_diff_3 += 1; // computer is 3 higher than highest

    Ok((num_diff_1, num_diff_2, num_diff_3))
}

#[allow(dead_code)]
fn find_num_arrangements_sloooooow(mut input: Vec<usize>) -> usize {
    input.sort_unstable();

    let computer = *input.last().unwrap() + 3;
    input.push(computer);

    debug!(?input, computer);

    r(0, -1, &input)
}

#[instrument(skip(input))] // this makes it a lot slower, 8 ms vs 800 ms for day10_test_2
fn r(c: usize, i: isize, input: &[usize]) -> usize {
    let mut num_arrangements = 0;
    for n in 1..=3 {
        let index = i + n;
        match input.get(index as usize) {
            Some(&num) if num <= c + 3 => {
                // debug!(num);
                num_arrangements += r(num, index, input);
            }
            Some(_) => return num_arrangements,
            None => return 1, // we have reached the end, thus this path represents one valid arrangement
        }
    }

    num_arrangements
}

fn get_num_arrangements(
    current: usize,
    num_arrangements_at_point: &HashMap<usize, usize>,
) -> usize {
    (num_arrangements_at_point
        .get(&(current + 1))
        .cloned()
        .unwrap_or_default())
        + (num_arrangements_at_point
            .get(&(current + 2))
            .cloned()
            .unwrap_or_default())
        + (num_arrangements_at_point
            .get(&(current + 3))
            .cloned()
            .unwrap_or_default())
}

fn find_num_arrangements(mut input: Vec<usize>) -> usize {
    input.sort_unstable();

    let computer = *input.last().unwrap() + 3;

    let mut num_arrangements_at_point = HashMap::new();
    num_arrangements_at_point.insert(computer, 1usize);

    input.iter().rev().cloned().for_each(|point| {
        num_arrangements_at_point.insert(
            point,
            get_num_arrangements(point, &num_arrangements_at_point),
        );
    });

    get_num_arrangements(0, &num_arrangements_at_point)
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse_input() -> Result<()> {
        let input = include_str!("../data/day10_test_1.txt");
        let mut parsed = parse_input(input)?;
        parsed.sort_unstable();

        debug!(?parsed);

        assert_eq!(parsed, vec![1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19]);

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        let input_1 = include_str!("../data/day10_test_1.txt");
        let input_2 = include_str!("../data/day10_test_2.txt");

        let tests = vec![(input_1, (7, 0, 5)), (input_2, (22, 0, 10))];

        for (input, want) in tests {
            let mut parsed = parse_input(input)?;
            let got = find_diffs(&mut parsed)?;
            debug!(?got);
            assert_eq!(got, want);
        }

        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        let input_1 = include_str!("../data/day10_test_1.txt");
        let input_2 = include_str!("../data/day10_test_2.txt");

        let tests = vec![(input_1, 8), (input_2, 19208)];

        for (input, want) in tests {
            let parsed = parse_input(input)?;
            let now = Instant::now();
            let got = find_num_arrangements(parsed);
            let elapsed_ms = now.elapsed().as_millis();
            debug!(?got, ?elapsed_ms);
            assert_eq!(got, want);
        }

        Ok(())
    }
}
