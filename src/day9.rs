use std::str::FromStr;
use tracing::{debug, info};

pub fn run() {
    let input = include_str!("../data/day9.txt");
    let parsed = parse_input(input);
    let first_invalid = find_first_invalid(&parsed, 25);
    let (min, max) = find_range(&parsed, first_invalid);
    let sum = min + max;

    info!(first_invalid, sum);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<usize>, _>>()
        .expect("invalid input")
}

fn find_first_invalid(input: &[usize], preamble_length: usize) -> usize {
    for i in preamble_length..input.len() {
        let current = *input.get(i).unwrap();
        debug!(i, current);
        let mut found = false;

        'search: for j in i - preamble_length..i - 1 {
            let a = *input.get(j).unwrap();
            for k in (i - preamble_length + 1)..i {
                let b = *input.get(k).unwrap();
                let candidate = a + b;
                debug!(j, k, candidate);
                if candidate == current {
                    found = true;
                    break 'search;
                }
            }
        }

        debug!(found);

        if !found {
            return current;
        }
    }

    panic!("invalid input!");
}

fn find_range(input: &[usize], target: usize) -> (usize, usize) {
    'search: for i in 0..input.len() {
        let mut candidate = *input.get(i).unwrap();
        let mut min = candidate;
        let mut max = candidate;
        for j in i + 1..input.len() {
            let current = *input.get(j).unwrap();

            candidate += current;
            if candidate > target {
                continue 'search;
            }

            min = min.min(current);
            max = max.max(current);

            if candidate == target {
                return (min, max);
            }
        }
    }

    panic!("invalid input!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use eyre::Result;
    use tracing::Level;

    #[test]
    fn test_parse_input() -> Result<()> {
        crate::init_tracing(Level::DEBUG)?;

        let input = include_str!("../data/day9_test.txt");
        let parsed = parse_input(input);
        dbg!(parsed);

        Ok(())
    }

    #[test]
    fn test_part_1() -> Result<()> {
        crate::init_tracing(Level::DEBUG)?;

        let input = include_str!("../data/day9_test.txt");
        let parsed = parse_input(input);
        let answer = find_first_invalid(&parsed, 5);

        debug!(answer);

        assert_eq!(answer, 127);
        Ok(())
    }

    #[test]
    fn test_part_2() -> Result<()> {
        crate::init_tracing(Level::DEBUG)?;

        let input = include_str!("../data/day9_test.txt");
        let parsed = parse_input(input);
        let first_invalid = find_first_invalid(&parsed, 5);
        let (min, max) = find_range(&parsed, first_invalid);

        assert_eq!((min, max), (15, 47));
        Ok(())
    }
}
