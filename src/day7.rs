use std::collections::{HashMap, HashSet};

pub fn run() {
    let input = include_str!("../data/day7.txt");
    let a = check_input(input).len();
    let b = check_input2(input);

    println!("day 7: {}, {}", a, b);
}

fn parse_input(input: &str) -> Vec<(&str, HashMap<&str, usize>)> {
    input
        .trim()
        .split('\n')
        .map(|rule| {
            let mut rule = rule.split(" bags contain ");
            let container_color = rule.next().unwrap();
            let can_contain = rule.next().unwrap();

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

            (container_color, can_contain)
        })
        .collect::<Vec<_>>()
}

fn check_input(input: &str) -> HashSet<&str> {
    let contains_by_color: HashMap<&str, HashSet<&str>> = parse_input(input).into_iter().fold(
        HashMap::new(),
        |mut map, (container_color, contains)| {
            for (contained_color, _) in contains {
                map.entry(contained_color)
                    .or_default()
                    .insert(container_color);
            }
            map
        },
    );

    let mut outermost_bag_colors = contains_by_color.get("shiny gold").unwrap().to_owned();
    let mut checked_bag_colors = HashSet::new();

    for _ in 0..1000 {
        let unchecked_bag_colors = outermost_bag_colors
            .difference(&checked_bag_colors)
            .cloned()
            .collect::<HashSet<_>>();

        for bag_color in unchecked_bag_colors.iter() {
            if let Some(contains) = contains_by_color.get(bag_color) {
                outermost_bag_colors.extend(contains);
            }
        }

        checked_bag_colors.extend(unchecked_bag_colors);

        if outermost_bag_colors.difference(&checked_bag_colors).count() == 0 {
            return outermost_bag_colors;
        }
    }

    panic!("infinite loop?!")
}

fn check_input2(input: &str) -> usize {
    let mut contains_by_color = parse_input(input).into_iter().fold(
        HashMap::new(),
        |mut map, (container_color, contains)| {
            map.insert(container_color, contains);
            map
        },
    );

    let mut contains_count_by_color = HashMap::new();

    let count = count_contains(
        1,
        "shiny gold",
        &mut contains_by_color,
        &mut contains_count_by_color,
    );

    // we dont count the shiny gold bag
    count - 1
}

fn count_contains<'a>(
    count: usize,
    color: &'a str,
    contains_by_color: &mut HashMap<&'a str, HashMap<&'a str, usize>>,
    total_contained_by_color: &mut HashMap<&'a str, usize>,
) -> usize {
    let total_contained = match total_contained_by_color.get(color) {
        Some(total_contained) => *total_contained,
        None => {
            let contains = contains_by_color.remove(color).unwrap();

            let mut total_contained = 0;
            for (contained_color, contained_count) in contains.into_iter() {
                total_contained += count_contains(
                    contained_count,
                    contained_color,
                    contains_by_color,
                    total_contained_by_color,
                );
            }

            total_contained_by_color.insert(color, total_contained);

            total_contained
        }
    };

    count + (count * total_contained)
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
        let input = include_str!("../data/day7_test.txt");
        let answered_questions_count = check_input2(input);
        assert_eq!(answered_questions_count, 32);
    }

    #[test]
    fn test_check_input2_2() {
        let input = include_str!("../data/day7_test_2.txt");
        let answered_questions_count = check_input2(input);
        assert_eq!(answered_questions_count, 126);
    }
}
