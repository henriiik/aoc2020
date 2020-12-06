use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../data/day5.txt");
    let a = check_input(input).unwrap();
    let b = check_input2(input).unwrap();

    println!("day 5: {}, {}", a, b);
}

fn parse_input(input: &str) -> Vec<(usize, usize, usize)> {
    input
        .trim()
        .split_terminator('\n')
        .map(parse_pass)
        .collect::<Vec<_>>()
}

fn check_input(input: &str) -> Option<usize> {
    parse_input(input).iter().map(|(_, _, id)| *id).max()
}

fn check_input2(input: &str) -> Option<usize> {
    parse_input(input)
        .iter()
        .fold(
            HashMap::<usize, HashSet<usize>>::new(),
            |mut m, (row, seat, _)| {
                m.entry(*row).or_default().insert(*seat);
                m
            },
        )
        .into_iter()
        .find_map(|(row, seats)| {
            if seats.len() == 7 {
                for i in 0..=7 {
                    if !seats.contains(&i) {
                        return Some(seat_id(row, i));
                    }
                }
                panic!("should find seat in row with only 7 taken seats");
            } else {
                None
            }
        })
}

fn parse_bsp(chars: &str, upper: char) -> usize {
    chars.chars().fold(0, |mut n, c| {
        n <<= 1;
        if c == upper {
            n += 1;
        }
        n
    })
}

fn seat_id(row: usize, seat: usize) -> usize {
    (row << 3) + seat
}

fn parse_pass(pass: &str) -> (usize, usize, usize) {
    let (rows, seats) = pass.split_at(7);
    let row = parse_bsp(rows, 'B');
    let seat = parse_bsp(seats, 'R');
    (row, seat, seat_id(row, seat))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_pass() {
        let tt = vec![
            ("FBFBBFFRLR", 44, 5, 357),
            ("BFFFBBFRRR", 70, 7, 567),
            ("FFFBBBFRRR", 14, 7, 119),
            ("BBFFBBFRLL", 102, 4, 820),
        ];

        for (pass, row, seat, id) in tt {
            let (got_row, got_col, got_id) = parse_pass(pass);
            assert_eq!(row, got_row);
            assert_eq!(seat, got_col);
            assert_eq!(id, got_id);
        }
    }

    #[test]
    fn test_day5() {
        let got = check_input2(include_str!("../data/day5.txt")).unwrap();

        assert_eq!(got, 696);
    }
}
