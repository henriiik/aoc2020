use std::str::FromStr;

pub fn run() {
    let input = include_str!("../data/day1.txt")
        .split_ascii_whitespace()
        .map(|a| u64::from_str(a).unwrap())
        .collect::<Vec<_>>();

    println!("day 1: {}, {}", calc(&input), calc2(&input));
}

fn calc(input: &[u64]) -> u64 {
    for i in input {
        for j in input {
            if i + j == 2020 {
                return i * j;
            }
        }
    }

    0
}

fn calc2(input: &[u64]) -> u64 {
    for i in input {
        for j in input {
            for k in input {
                if i + j + k == 2020 {
                    return i * j * k;
                }
            }
        }
    }

    0
}
