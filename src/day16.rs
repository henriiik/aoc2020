use eyre::{eyre, Result};
use std::{
    collections::{HashMap, HashSet},
    ops::RangeInclusive,
    str::FromStr,
};
use tracing::{debug, info, instrument};

#[instrument]
pub fn run() -> Result<()> {
    let input = include_str!("../data/day16.txt");
    let mut scanner = TicketScanner::parse(input)?;
    let answer: usize = scanner.find_invalid_values().iter().sum();
    info!(?answer);

    let answer_2 = scanner.get_departure_value();
    info!(?answer_2);

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
struct Ticket {
    values: Vec<usize>,
}

impl Ticket {
    fn has_invalid_value(&self, rules: &[Rule]) -> Option<usize> {
        for value in self.values.iter() {
            if !rules.iter().any(|rule| rule.is_valid_value(value)) {
                return Some(*value);
            }
        }
        None
    }
}

impl FromStr for Ticket {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s
            .split(',')
            .map(usize::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { values })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Rule {
    name: String,
    range_a: RangeInclusive<usize>,
    range_b: RangeInclusive<usize>,
}

impl Rule {
    fn is_valid_value(&self, value: &usize) -> bool {
        self.range_a.contains(value) || self.range_b.contains(value)
    }

    fn is_valid_for_all_tickets_at_index(&self, tickets: &[Ticket], index: usize) -> bool {
        tickets
            .iter()
            .all(|ticket| self.is_valid_value(&ticket.values[index]))
    }
}

impl FromStr for Rule {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(": ");
        let name = iter
            .next()
            .ok_or_else(|| eyre!("invalid input, no name: {}", s))?
            .to_string();
        let mut ranges = iter
            .next()
            .ok_or_else(|| eyre!("invalid input, no ranges: {}", s))?
            .split(" or ");

        let mut range_a_values = ranges
            .next()
            .ok_or_else(|| eyre!("invalid input, no first range: {}", s))?
            .split('-')
            .map(usize::from_str);

        let range_a_start = range_a_values
            .next()
            .ok_or_else(|| eyre!("invalid input, no start number in first range, {}", s))??;
        let range_a_end = range_a_values
            .next()
            .ok_or_else(|| eyre!("invalid input, no end number in first range, {}", s))??;

        let mut range_b_values = ranges
            .next()
            .ok_or_else(|| eyre!("invalid input, no first range: {}", s))?
            .split('-')
            .map(usize::from_str);

        let range_b_start = range_b_values
            .next()
            .ok_or_else(|| eyre!("invalid input, no start number in second range, {}", s))??;
        let range_b_end = range_b_values
            .next()
            .ok_or_else(|| eyre!("invalid input, no end number in second range, {}", s))??;

        Ok(Self {
            name,
            range_a: range_a_start..=range_a_end,
            range_b: range_b_start..=range_b_end,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct TicketScanner {
    rules: Vec<Rule>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

impl TicketScanner {
    fn parse(input: &str) -> Result<Self> {
        let mut iter = input.split("\n\n");
        let rules = iter
            .next()
            .ok_or_else(|| eyre!("invalid input, missing rules: {}", input))?
            .lines()
            .map(Rule::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        let my_ticket = iter
            .next()
            .ok_or_else(|| eyre!("invalid ticket, missing my ticket section: {}", input))?
            .lines()
            .skip(1) // skip header
            .map(Ticket::from_str)
            .next()
            .ok_or_else(|| eyre!("invalid ticket, missing my ticket: {}", input))??;

        let nearby_tickets = iter
            .next()
            .ok_or_else(|| eyre!("invalid ticket, missing nearby tickets section: {}", input))?
            .lines()
            .skip(1) // skip header
            .map(Ticket::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            rules,
            my_ticket,
            nearby_tickets,
        })
    }

    fn find_invalid_values(&self) -> Vec<usize> {
        self.nearby_tickets
            .iter()
            .filter_map(|ticket| ticket.has_invalid_value(&self.rules))
            .collect()
    }

    fn filter_valid_tickets(&mut self) {
        let tickets: Vec<Ticket> = self.nearby_tickets.drain(..).collect();
        self.nearby_tickets = tickets
            .into_iter()
            .filter(|ticket| ticket.has_invalid_value(&self.rules).is_none())
            .collect();
    }

    fn sort_rules(&mut self) {
        self.filter_valid_tickets();

        let mut value_indexes: HashSet<usize> = (0..self.rules.len()).into_iter().collect();
        let mut rule_index_to_rule_map: HashMap<_, _> = self.rules.drain(..).enumerate().collect();
        let mut value_index_to_rule_index_map = HashMap::new();

        while !value_indexes.is_empty() {
            'search: for value_index in value_indexes.clone() {
                let valid = rule_index_to_rule_map
                    .iter()
                    .filter_map(|(i, rule)| {
                        if rule.is_valid_for_all_tickets_at_index(&self.nearby_tickets, value_index)
                        {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if let [rule_index] = valid.as_slice() {
                    let rule = rule_index_to_rule_map.remove(rule_index).unwrap();
                    value_index_to_rule_index_map.insert(value_index, rule);
                    value_indexes.remove(&value_index);
                    continue 'search;
                }
            }
        }

        let mut value_index_to_rule_index = value_index_to_rule_index_map
            .into_iter()
            .collect::<Vec<_>>();

        value_index_to_rule_index.sort_by(|a, b| a.0.cmp(&b.0));

        self.rules = value_index_to_rule_index
            .into_iter()
            .map(|(_, rule)| rule)
            .collect();
    }

    fn get_departure_value(&mut self) -> usize {
        self.sort_rules();

        self.rules
            .iter()
            .zip(self.my_ticket.values.iter().cloned())
            .filter_map(|(rule, value)| {
                debug!(?rule, value);
                assert!(rule.is_valid_value(&value));
                if rule.name.starts_with("departure") {
                    Some(value)
                } else {
                    None
                }
            })
            .product()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use eyre::Result;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day16_test.txt");

        let scanner = TicketScanner::parse(input)?;

        debug!(?scanner);

        Ok(())
    }

    #[test]
    fn test_scanner() -> Result<()> {
        let input = include_str!("../data/day16_test.txt");
        let scanner = TicketScanner::parse(input)?;
        let values = scanner.find_invalid_values();
        assert_eq!(values, vec![4, 55, 12,]);

        Ok(())
    }

    #[test]
    fn test_scanner_find_valid_tickets() -> Result<()> {
        let input = include_str!("../data/day16_test_2.txt");
        let mut scanner = TicketScanner::parse(input)?;
        scanner.filter_valid_tickets();
        assert_eq!(
            scanner.nearby_tickets,
            vec![
                Ticket {
                    values: vec![3, 9, 18]
                },
                Ticket {
                    values: vec![15, 1, 5]
                },
                Ticket {
                    values: vec![5, 14, 9]
                }
            ]
        );

        Ok(())
    }

    #[test]
    fn test_scanner_sort_rules() -> Result<()> {
        let input = include_str!("../data/day16_test_2.txt");
        let mut scanner = TicketScanner::parse(input)?;

        assert_eq!(
            scanner.rules,
            vec![
                Rule {
                    name: "class".into(),
                    range_a: 0..=1,
                    range_b: 4..=19,
                },
                Rule {
                    name: "row".into(),
                    range_a: 0..=5,
                    range_b: 8..=19,
                },
                Rule {
                    name: "seat".into(),
                    range_a: 0..=13,
                    range_b: 16..=19,
                },
            ]
        );

        scanner.sort_rules();

        assert_eq!(
            scanner.rules,
            vec![
                Rule {
                    name: "row".into(),
                    range_a: 0..=5,
                    range_b: 8..=19,
                },
                Rule {
                    name: "class".into(),
                    range_a: 0..=1,
                    range_b: 4..=19,
                },
                Rule {
                    name: "seat".into(),
                    range_a: 0..=13,
                    range_b: 16..=19,
                },
            ]
        );

        Ok(())
    }

    #[test]
    fn test_scanner_get_departure_value() -> Result<()> {
        let input = include_str!("../data/day16_test_2.txt");
        let mut scanner = TicketScanner::parse(input)?;

        assert_eq!(scanner.get_departure_value(), 1);

        Ok(())
    }
}
