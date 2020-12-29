use eyre::{bail, eyre, Context, Result};
use std::convert::TryFrom;
use std::{cell::RefCell, collections::HashMap, fmt::Debug, str::FromStr, time::Instant};
use tracing::{debug, info, instrument, trace};

#[instrument]
pub fn run() -> Result<()> {
    let tileset = TileSet::parse(include_str!("../data/day20.txt"))?;
    info!(t = ?tileset.tiles.len());
    let corner_product = tileset.find_corner_product();
    info!(corner_product);

    Ok(())
}

#[derive(Debug, Clone)]
enum Rule {
    Char(char),
    Other(Vec<usize>),
    Either(Vec<usize>, Vec<usize>),
}

trait Day19Str {
    fn parse_rule_id(&'static self) -> Result<usize>;
    fn parse_rule_list(&'static self) -> Result<Vec<usize>>;
    fn parse_rule(&'static self) -> Result<(usize, Rule)>;
    fn parse_input(&'static self) -> Result<(HashMap<usize, Rule>, Vec<&'static str>)>;
}

impl Day19Str for str {
    fn parse_rule_id(&'static self) -> Result<usize> {
        self.parse().context(self)
    }

    fn parse_rule_list(&'static self) -> Result<Vec<usize>> {
        self.split(' ')
            .map(|s| s.parse().map_err(eyre::Report::new).context(s))
            .collect()
    }

    fn parse_rule(&'static self) -> Result<(usize, Rule)> {
        let mut iter = self.split(": ");
        let id_str = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))
            .context(self)?;

        let id = id_str.parse_rule_id()?;

        let rule_str = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))
            .context(self)?;

        if rule_str.starts_with('"') {
            return Ok((id, Rule::Char(rule_str.chars().nth(1).unwrap())));
        }

        let mut iter = rule_str.split(" | ");

        let rule_ids = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))
            .context(self)?
            .parse_rule_list()
            .unwrap();

        if let Some(other_rule_ids) = iter.next() {
            let other_rule_ids = other_rule_ids
                .split(' ')
                .map(usize::from_str)
                .collect::<Result<Vec<_>, _>>()
                .unwrap();
            Ok((id, Rule::Either(rule_ids, other_rule_ids)))
        } else {
            Ok((id, Rule::Other(rule_ids)))
        }
    }

    fn parse_input(&'static self) -> Result<(HashMap<usize, Rule>, Vec<&'static str>)> {
        let mut iter = self.split("\n\n");
        let rules = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))
            .context(self)?
            .lines()
            .map(str::parse_rule)
            .collect::<Result<_>>()?;

        let messages = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))
            .context(self)?
            .lines()
            .collect();

        Ok((rules, messages))
    }
}

struct RuleMatcher {
    rules: HashMap<usize, Rule>,
    max_recurse: RefCell<HashMap<usize, usize>>,
    current_recurse: RefCell<HashMap<usize, usize>>,
    recurse_limit: usize,
}

#[derive(Debug, Clone)]
pub enum Match {
    Finished,
    Next(usize),
}

impl RuleMatcher {
    fn new(rules: HashMap<usize, Rule>) -> Self {
        Self {
            rules,
            max_recurse: Default::default(),
            current_recurse: Default::default(),
            recurse_limit: 1,
        }
    }

    fn clear(&self) {
        self.max_recurse.borrow_mut().clear();
        self.current_recurse.borrow_mut().clear();
    }

    #[instrument(level = "trace", name = "m", skip(self, msg))]
    fn matches_rule(&self, msg: &'static str, i: usize, id: usize) -> Result<Option<Match>> {
        let max = self.max_recurse.borrow().get(&id).cloned();
        if let Some(max) = max {
            let current = self
                .current_recurse
                .borrow()
                .get(&id)
                .cloned()
                .unwrap_or_default();

            if current > max {
                trace!("recursion limit reached");
                return Ok(Some(Match::Next(i)));
            }

            self.current_recurse.borrow_mut().insert(id, current + 1);
        }

        let rule = self
            .rules
            .get(&id)
            .ok_or_else(|| eyre!("invalid rule id: {}", id))?;

        let result = match rule {
            Rule::Char(want) => {
                let got = msg.chars().nth(i);
                trace!(?want, ?got);

                match got {
                    Some(got) if *want == got => {
                        if msg.len() == i + 1 {
                            Some(Match::Finished)
                        } else {
                            Some(Match::Next(i + 1))
                        }
                    }
                    _ => None,
                }
            }
            Rule::Other(rule_list) => self.matches_rule_list(msg, i, rule_list)?,
            Rule::Either(rule_list_a, rule_list_b) => {
                let shared = rule_list_a
                    .iter()
                    .zip(rule_list_b.iter())
                    .take_while(|(a, b)| a == b)
                    .count();

                let shared_list = &rule_list_a[0..shared];
                let a_list = &rule_list_a[shared..];
                let b_list = &rule_list_b[shared..];

                match self.matches_rule_list(msg, i, shared_list)? {
                    Some(Match::Next(i)) => {
                        if let Some(n) = self.matches_rule_list(msg, i, a_list)? {
                            Some(n)
                        } else if let Some(n) = self.matches_rule_list(msg, i, b_list)? {
                            Some(n)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            }
        };

        if result.is_none() {
            trace!("fail");
        } else {
            trace!("match: {:?}", result);
        }

        Ok(result)
    }

    #[instrument(level = "trace", name = "l", skip(self, msg, ids))]
    fn matches_rule_list(
        &self,
        msg: &'static str,
        mut i: usize,
        ids: &[usize],
    ) -> Result<Option<Match>> {
        let mut iter = ids.iter().peekable();
        while let Some(id) = iter.next() {
            let res = self.matches_rule(msg, i, *id)?;

            match res {
                Some(Match::Next(next)) => {
                    i = next;
                }
                Some(Match::Finished) => {
                    if iter.peek().is_none() {
                        trace!("complete match detected!");
                        return Ok(Some(Match::Finished));
                    } else {
                        trace!("non-complete match detected! {:?}", iter.peek());
                        return Ok(None);
                    }
                }
                None => return Ok(None),
            }
        }

        trace!(?ids);

        if i == msg.len() {
            Ok(Some(Match::Finished))
        } else {
            Ok(Some(Match::Next(i)))
        }
    }

    #[instrument(level = "trace", name = "t" skip(self), fields(len = msg.len()))]
    fn test_msg(&self, msg: &'static str) -> Result<bool> {
        self.clear();

        for i in 0..self.recurse_limit {
            for j in 0..self.recurse_limit {
                trace!(i, j);
                self.max_recurse.borrow_mut().insert(8, i);
                self.max_recurse.borrow_mut().insert(11, j);
                self.current_recurse.borrow_mut().insert(8, 0);
                self.current_recurse.borrow_mut().insert(11, 0);
                match self.matches_rule(msg, 0, 0)? {
                    Some(Match::Finished) => return Ok(true),
                    Some(Match::Next(next)) => {
                        trace!(len = ?msg.len(), next);
                        if msg.len() == next {
                            return Ok(true);
                        }
                    }
                    None => (),
                }
            }
        }

        Ok(false)
    }

    fn test_msgs(&self, msgs: &[&'static str]) -> Result<usize> {
        let mut valid = 0;
        let now = Instant::now();

        for &msg in msgs {
            if self.test_msg(msg)? {
                valid += 1;
            }
        }

        let elapsed_ms = now.elapsed().as_millis();
        info!(?elapsed_ms);

        Ok(valid)
    }

    fn enable_part_2(&mut self) {
        // these rules are changed from the instructions to match how the matcher works
        self.rules.insert(8, Rule::Other(vec![42, 8]));
        self.rules.insert(11, Rule::Other(vec![42, 11, 31]));
        self.recurse_limit = 5;
    }

    fn from_str(input: &'static str) -> Result<(Self, Vec<&'static str>)> {
        let (rules, msgs) = input.parse_input()?;

        trace!(?rules);
        trace!(?msgs);

        Ok((RuleMatcher::new(rules), msgs))
    }
}

#[derive(Clone)]
struct Tile {
    data: [bool; 100],
    id: u64,
    edge_ids: [u16; 4],         // up, right, down, left
    flipped_edge_ids: [u16; 4], // up, right, down, left
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\nid={}", self.id)?;

        for (i, v) in self.data.iter().enumerate() {
            if i % 10 == 0 {
                writeln!(f)?;
            }
            if *v {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        writeln!(f)?;

        Ok(())
    }
}

#[instrument(level = "debug", skip(data))]
fn calc_edge_id(data: &[bool], start: isize, step: isize) -> Result<u16> {
    let mut id = 0;

    for n in 0..10 {
        let i = start + (n * step);
        id <<= 1;
        if *data
            .get(usize::try_from(i)?)
            .ok_or_else(|| eyre!("invalid index: {}", i))?
        {
            id += 1;
        }
    }

    Ok(id)
}

impl Tile {
    fn parse(input: &str) -> Result<Tile> {
        let mut iter = input.lines();

        let id = iter
            .next()
            .ok_or_else(|| eyre!("unexpected end of input"))?
            .trim_start_matches("Tile ")
            .trim_end_matches(':')
            .parse::<u64>()?;

        debug!(id);

        let mut data = [false; 100];

        for (i, row) in iter.enumerate() {
            for (j, c) in row.chars().enumerate() {
                match c {
                    '#' => data[j + (i * 10)] = true,
                    '.' => (),
                    c => bail!("invalid char: {}", c),
                }
            }
        }

        assert_eq!(data.len(), 100);

        let edge_ids = [
            calc_edge_id(&data, 0, 1)?,  //  up
            calc_edge_id(&data, 9, 10)?, //  right
            calc_edge_id(&data, 90, 1)?, //  down
            calc_edge_id(&data, 0, 10)?, //  left
        ];

        let flipped_edge_ids = [
            calc_edge_id(&data, 9, -1)?,   //  up
            calc_edge_id(&data, 99, -10)?, //  right
            calc_edge_id(&data, 99, -1)?,  //  down
            calc_edge_id(&data, 90, -10)?, //  left
        ];

        Ok(Self {
            id,
            data,
            edge_ids,
            flipped_edge_ids,
        })
    }
}

#[derive(Debug)]
struct TileSet {
    tiles: Vec<Tile>,
}

impl TileSet {
    fn parse(input: &str) -> Result<Self> {
        let tiles = input
            .split("\n\n")
            .map(Tile::parse)
            .collect::<Result<Vec<_>>>()?;
        Ok(Self { tiles })
    }

    fn find_match(&self, source_id: u64, edge_id: u16) -> impl Iterator<Item = &Tile> {
        self.tiles.iter().filter(move |&t| {
            t.id != source_id
                && (t.edge_ids.contains(&edge_id) || t.flipped_edge_ids.contains(&edge_id))
        })
    }

    fn is_corner(&self, tile: &Tile) -> bool {
        let mut n = 0;

        if self.find_match(tile.id, tile.edge_ids[0]).next().is_some() {
            n += 1;
        }

        if self.find_match(tile.id, tile.edge_ids[1]).next().is_some() {
            n += 1;
        }

        if self.find_match(tile.id, tile.edge_ids[2]).next().is_some() {
            n += 1;
        }

        if self.find_match(tile.id, tile.edge_ids[3]).next().is_some() {
            n += 1;
        }

        n == 2
    }

    // nw, ne,se,sw
    fn find_corner_product(&self) -> u64 {
        self.tiles
            .iter()
            .filter_map(|t| if self.is_corner(t) { Some(t.id) } else { None })
            .take(4)
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::debug;

    #[test]
    fn test_parse() -> Result<()> {
        let input = include_str!("../data/day20_test.txt");

        let result = TileSet::parse(input)?;

        debug!(?result);

        Ok(())
    }

    #[test]
    fn test_find_corner_product() -> Result<()> {
        let input = include_str!("../data/day20_test.txt");

        let tiles = TileSet::parse(input)?;

        let corner_product = tiles.find_corner_product();

        assert_eq!(corner_product, 20899048083289);

        Ok(())
    }

    #[test]
    fn test_msgs() -> Result<()> {
        let (matcher, msgs) = RuleMatcher::from_str(include_str!("../data/day19_test.txt"))?;

        let result = matcher.test_msgs(&msgs)?;

        assert_eq!(result, 2);

        Ok(())
    }

    #[test]
    fn test_msgs_2() -> Result<()> {
        let (mut matcher, _msgs) = RuleMatcher::from_str(include_str!("../data/day19_test_2.txt"))?;

        matcher.enable_part_2();

        assert_eq!(
            matcher.test_msg("abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa")?,
            false
        );
        assert_eq!(matcher.test_msg("bbabbbbaabaabba")?, true);
        assert_eq!(matcher.test_msg("babbbbaabbbbbabbbbbbaabaaabaaa")?, true);
        assert_eq!(
            matcher.test_msg("aaabbbbbbaaaabaababaabababbabaaabbababababaaa")?,
            true
        );
        assert_eq!(matcher.test_msg("bbbbbbbaaaabbbbaaabbabaaa")?, true);
        assert_eq!(
            matcher.test_msg("bbbababbbbaaaaaaaabbababaaababaabab")?,
            true
        );
        assert_eq!(matcher.test_msg("ababaaaaaabaaab")?, true);
        assert_eq!(matcher.test_msg("ababaaaaabbbaba")?, true);
        assert_eq!(matcher.test_msg("baabbaaaabbaaaababbaababb")?, true);
        assert_eq!(matcher.test_msg("abbbbabbbbaaaababbbbbbaaaababb")?, true);
        assert_eq!(matcher.test_msg("aaaaabbaabaaaaababaa")?, true);
        assert_eq!(matcher.test_msg("aaaabbaaaabbaaa")?, false);
        assert_eq!(
            matcher.test_msg("aaaabbaabbaaaaaaabbbabbbaaabbaabaaa")?,
            true
        );
        assert_eq!(matcher.test_msg("babaaabbbaaabaababbaabababaaab")?, false);
        assert_eq!(
            matcher.test_msg("aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba")?,
            true
        );

        Ok(())
    }

    #[test]
    fn test_msgs_custom() -> Result<()> {
        let (mut matcher, _msgs) =
            RuleMatcher::from_str(include_str!("../data/day19_test_custom.txt"))?;

        matcher.enable_part_2();

        assert_eq!(matcher.test_msg("a")?, false);
        assert_eq!(matcher.test_msg("ab")?, false);
        assert_eq!(matcher.test_msg("b")?, false);

        assert_eq!(matcher.test_msg("aaaabb")?, true, "2 x 8, 2 x 42");
        assert_eq!(matcher.test_msg("aaabb")?, true, "1 x 8, 2 x 42");
        assert_eq!(matcher.test_msg("aaab")?, true, "2 x 8, 1 x 42");
        assert_eq!(matcher.test_msg("aab")?, true, "1 x 8, 1 x 42");

        Ok(())
    }
}
