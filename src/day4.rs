use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("day4.txt");
    let a = check_input(input);
    let b = check_input2(input);

    println!("day 4: {}, {}", a, b);
}

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .trim()
        .split_terminator("\n\n")
        .map(|a| {
            a.split_whitespace()
                .map(|b| {
                    let mut b = b.splitn(2, ':');
                    (b.next().unwrap(), b.next().unwrap())
                })
                .collect::<HashMap<_, _>>()
        })
        .collect::<Vec<_>>()
}

fn check_input(input: &str) -> usize {
    let mut want = HashSet::new();
    want.insert("byr");
    want.insert("ecl");
    want.insert("eyr");
    want.insert("hcl");
    want.insert("hgt");
    want.insert("iyr");
    want.insert("pid");

    let input = parse_input(input);

    input.into_iter().fold(0, |mut n, a| {
        let a = a.into_iter().map(|a| a.0).collect::<HashSet<_>>();

        if want.difference(&a).count() == 0 {
            n += 1;
        }

        n
    })
}

fn check_input2(input: &str) -> usize {
    let input = parse_input(input);
    let num = "1234567890";
    let hex = "abcdef";

    let mut ecls = HashSet::new();
    ecls.insert("amb");
    ecls.insert("blu");
    ecls.insert("brn");
    ecls.insert("gry");
    ecls.insert("grn");
    ecls.insert("hzl");
    ecls.insert("oth");

    input
        .into_iter()
        .filter_map(|mut a| {
            let byr = a.remove("byr")?.parse::<usize>().ok()?;
            if byr < 1920 || byr > 2002 {
                return None;
            }

            let iyr = a.remove("iyr")?.parse::<usize>().ok()?;
            if iyr < 2010 || iyr > 2020 {
                return None;
            }

            let eyr = a.remove("eyr")?.parse::<usize>().ok()?;
            if eyr < 2020 || eyr > 2030 {
                return None;
            }

            let hgt = a.remove("hgt")?;
            if let Some(hgt) = hgt.strip_suffix("cm") {
                let hgt = hgt.parse::<usize>().ok()?;
                if hgt < 150 || hgt > 193 {
                    return None;
                }
            } else if let Some(hgt) = hgt.strip_suffix("in") {
                let hgt = hgt.parse::<usize>().ok()?;
                if hgt < 59 || hgt > 76 {
                    return None;
                }
            } else {
                return None;
            }

            let hcl = a.remove("hcl")?.strip_prefix("#")?;
            if hcl.len() != 6 || hcl.chars().any(|c| !num.contains(c) && !hex.contains(c)) {
                return None;
            }

            if !ecls.contains(a.remove("ecl")?) {
                return None;
            }

            let pid = a.remove("pid")?;
            if pid.len() != 9 || pid.chars().any(|c| !num.contains(c)) {
                return None;
            }

            Some(1)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_day4() {
        let got = check_input(include_str!("day4_test.txt"));

        assert_eq!(got, 2);
    }

    #[test]
    fn test_day4_part2() {
        let got = check_input2(include_str!("day4_test.txt"));

        assert_eq!(got, 2);
    }
}
