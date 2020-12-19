use std::str::FromStr;

pub fn run() {
    let input = include_str!("../data/day9.txt");
    let parsed = parse_input(input);
    let answer = check_input(&parsed, 25);
    let answer_2 = check_input2(input);

    println!("day 8: {} {}", answer, answer_2);
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .trim()
        .lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<usize>, _>>()
        .expect("invalid input")
}

fn check_input(input: &[usize], preamble_length: usize) -> usize {
    for i in preamble_length..input.len() {
        let current = *input.get(i).unwrap();
        dbg!(i, current);
        let mut found = false;

        'search: for j in i - preamble_length..i - 1 {
            let a = *input.get(j).unwrap();
            for k in (i - preamble_length + 1)..i {
                let b = *input.get(k).unwrap();
                let candidate = a + b;
                println!("{}+{} = {}", j, k, candidate);
                if candidate == current {
                    found = true;
                    break 'search;
                }
            }
        }

        dbg!(found);

        if !found {
            return current;
        }
    }

    panic!("invalid input!");
}

fn check_input2(input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_input() {
        let input = include_str!("../data/day9_test.txt");
        let parsed = parse_input(input);
        dbg!(parsed);
    }

    #[test]
    fn test_check_input() {
        let input = include_str!("../data/day9_test.txt");
        let parsed = parse_input(input);
        let answer = check_input(&parsed, 5);
        dbg!(answer);
        assert_eq!(answer, 127);
    }

    #[test]
    fn test_check_input2() {
        let input = include_str!("../data/day9_test.txt");
        let answer2 = check_input2(input);
        assert_eq!(answer2, 1);
    }
}
