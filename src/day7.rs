use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../data/day7.txt");
    let a = check_input(input).len();
    // let b = check_input2(input);

    println!("day 7: {}, {}", a, a);
}

fn parse_input(input: &str) -> Vec<(&str, HashMap<&str, usize>)> {
    input
        .trim()
        .split('\n')
        .map(|rule| {
            let mut rule = rule.split(" bags contain ");
            let container_color = rule.next().unwrap();
            // dbg!(container_color);

            let can_contain = rule.next().unwrap();
            // dbg!(contains);

            let can_contain = if can_contain.starts_with("no") {
                HashMap::new()
            } else {
                can_contain
                    .split(", ")
                    .map(|containee| {
                        let mut containee = containee.splitn(2, ' ');
                        let count: usize = containee.next().unwrap().parse().unwrap();
                        let color = containee.next().unwrap().split(" bag").next().unwrap();
                        (color, count)
                    })
                    .collect::<HashMap<_, _>>()
            };

            // dbg!(&can_contain);

            (container_color, can_contain)
        })
        .collect::<Vec<_>>()
}

fn check_input(input: &str) -> HashSet<&str> {
    let can_contain: HashMap<&str, HashSet<&str>> = parse_input(input).into_iter().fold(
        HashMap::new(),
        |mut map, (container_color, can_contain)| {
            for (contained_color, _) in can_contain {
                map.entry(contained_color)
                    .or_default()
                    .insert(container_color);
            }
            map
        },
    );

    // dbg!(&can_contain);

    let mut unchecked_candidates = can_contain.get("shiny gold").unwrap().to_owned();
    let mut checked_candidates = HashSet::new();

    for _ in 0..1000 {
        let diff = unchecked_candidates
            .difference(&checked_candidates)
            .cloned()
            .collect::<HashSet<_>>();
        let new_candidates = diff
            .iter()
            .fold(HashSet::<&str>::new(), |mut set, candidate| {
                // dbg!(candidate);
                if let Some(thing) = can_contain.get(*candidate) {
                    set.extend(thing);
                }
                set
            });

        // dbg!(&new_candidates);
        unchecked_candidates.extend(new_candidates);
        checked_candidates.extend(diff);
        // dbg!(&unchecked_candidates);

        if unchecked_candidates.difference(&checked_candidates).count() == 0 {
            return unchecked_candidates;
        }
    }

    panic!("infinite loop?!")
}

fn check_input2(input: &str) -> usize {
    parse_input(input);
    todo!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_check_input() {
        let input = include_str!("../data/day7_test.txt");
        let container_bags = check_input(input);
        assert_eq!(container_bags.len(), 4);
    }

    #[test]
    fn test_check_input2() {
        let input = include_str!("../data/day6_test.txt");
        let answered_questions_count = check_input2(input);
        assert_eq!(answered_questions_count, 6);
    }
}
