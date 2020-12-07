use std::collections::HashSet;

pub fn run() {
    let input = include_str!("../data/day6.txt");
    let a = check_input(input);
    let b = check_input2(input);

    println!("day 6: {}, {}", a, b);
}

fn parse_input(input: &str) -> Vec<Vec<HashSet<char>>> {
    input
        .trim()
        .split_terminator("\n\n")
        .map(parse_goup)
        .collect::<Vec<_>>()
}

fn check_input(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|group_answered| {
            group_answered.iter().fold(
                HashSet::<char>::new(),
                |mut all_answered, person_answered| {
                    all_answered.extend(person_answered);
                    all_answered
                },
            )
        })
        .map(|set| set.len())
        .sum()
}

fn check_input2(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|group_answered| {
            let mut iter = group_answered.iter();
            let first = iter.next().unwrap();
            iter.fold(first.to_owned(), |common, person| {
                common.intersection(person).cloned().collect()
            })
        })
        .map(|common| common.len())
        .sum()
}

fn parse_goup(group: &str) -> Vec<HashSet<char>> {
    group
        .split('\n')
        .fold(Vec::new(), |mut all_answered, person_answered| {
            all_answered.push(person_answered.chars().collect());
            all_answered
        })
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_input() {
        let input = include_str!("../data/day6_test.txt");
        let answered_questions_count = check_input(input);
        assert_eq!(answered_questions_count, 11);
    }

    #[test]
    fn test_check_input2() {
        let input = include_str!("../data/day6_test.txt");
        let answered_questions_count = check_input2(input);
        assert_eq!(answered_questions_count, 6);
    }
}
