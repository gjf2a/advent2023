use std::{collections::VecDeque, fmt::Display, ops::RangeInclusive, str::FromStr};

use advent_code_lib::{all_lines, chooser_main};
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, problem| {
        let (graph, parts) = input(filename)?;
        println!("{graph}");
        for part in parts.iter() {
            println!("{part}");
        }
        match problem {
            advent_code_lib::Part::One => {
                let total = parts
                    .iter()
                    .filter_map(|part| graph.accepts(part))
                    .sum::<u128>();
                println!("Part {problem:?}: {total}");
            }
            advent_code_lib::Part::Two => {
                println!("Part {problem:?}: {}", Searcher::search(&graph).score());
            }
        }

        Ok(())
    })
}

#[derive(Clone)]
struct EligibleParts {
    ranges: IndexMap<char, RangeInclusive<u128>>,
}

impl EligibleParts {
    fn new() -> Self {
        let mut ranges = IndexMap::new();
        for c in "xmas".chars() {
            ranges.insert(c, 1..=4000);
        }
        Self { ranges }
    }

    fn replaced(&self, rating: char, new_range: RangeInclusive<u128>) -> Self {
        let mut result = self.clone();
        result.ranges.insert(rating, new_range);
        result
    }

    fn score(&self) -> u128 {
        self.ranges.values().map(|r| r.end() - r.start() + 1).sum()
    }
}

struct Searcher {
    graph: RuleGraph,
    accept: Vec<EligibleParts>,
}

impl Searcher {
    fn score(&self) -> u128 {
        self.accept.iter().map(|ep| ep.score()).product()
    }

    fn search(graph: &RuleGraph) -> Self {
        let mut result = Self {
            graph: graph.clone(),
            accept: vec![],
        };
        result.search_help("in", EligibleParts::new());
        result
    }

    fn search_help(&mut self, current: &str, eligible: EligibleParts) {
        let mut eligible = eligible;
        let rules = self.graph.rules.get(current).unwrap().clone();
        for rule in rules.iter() {
            let (yes, no) = rule.split(&eligible);
            if let Some((parts, name)) = yes {
                match name.as_str() {
                    "A" => self.accept.push(parts),
                    "R" => {}
                    _ => self.search_help(name.as_str(), parts),
                };
            }
            if let Some(parts) = no {
                eligible = parts;
            } else {
                break;
            }
        }
    }
}

#[derive(Clone, Default)]
struct RuleGraph {
    rules: IndexMap<String, Vec<Rule>>,
}

#[derive(Default)]
struct Part {
    ratings: IndexMap<char, u128>,
}

impl Part {
    fn rating(&self) -> u128 {
        self.ratings.values().sum()
    }
}

#[derive(Clone)]
enum Rule {
    Condition {
        rating: char,
        cond: RuleCond,
        value: u128,
        outcome: String,
    },
    Uncondition(String),
}

#[derive(Copy, Clone)]
enum RuleCond {
    Less,
    Greater,
}

impl RuleCond {
    fn check(&self, left: u128, right: u128) -> bool {
        match self {
            Self::Less => left < right,
            Self::Greater => left > right,
        }
    }

    fn range_yes_no(
        &self,
        range: &RangeInclusive<u128>,
        cutoff: u128,
    ) -> (RangeInclusive<u128>, RangeInclusive<u128>) {
        match self {
            Self::Less => (*range.start()..=(cutoff - 1), cutoff..=*range.end()),
            Self::Greater => ((cutoff + 1)..=*range.end(), *range.start()..=cutoff),
        }
    }
}

impl Rule {
    fn split(
        &self,
        eligible: &EligibleParts,
    ) -> (Option<(EligibleParts, String)>, Option<EligibleParts>) {
        match self {
            Self::Uncondition(s) => (Some((eligible.clone(), s.clone())), None),
            Self::Condition {
                rating,
                cond,
                value,
                outcome,
            } => {
                let (yes, no) = cond.range_yes_no(eligible.ranges.get(rating).unwrap(), *value);
                (
                    Some((eligible.replaced(*rating, yes), outcome.clone())),
                    Some(eligible.replaced(*rating, no)),
                )
            }
        }
    }

    fn match_for(&self, part: &Part) -> Option<String> {
        match self {
            Self::Uncondition(s) => Some(s.clone()),
            Self::Condition {
                rating,
                cond,
                value,
                outcome,
            } => part
                .ratings
                .get(rating)
                .filter(|v| cond.check(**v, *value))
                .map(|_| outcome.clone()),
        }
    }
}

impl RuleGraph {
    fn accepts(&self, part: &Part) -> Option<u128> {
        let mut current = "in".to_owned();
        loop {
            for rule in self.rules.get(current.as_str()).unwrap().iter() {
                if let Some(destination) = rule.match_for(part) {
                    match destination.as_str() {
                        "A" => return Some(part.rating()),
                        "R" => return None,
                        _ => {
                            current = destination.clone();
                            break;
                        }
                    }
                }
            }
        }
    }

    fn add_line(&mut self, line: String) -> anyhow::Result<()> {
        let line = line.replace(&['{', '}'], " ");
        let mut parts = line.split_whitespace();
        let name = parts.next().unwrap();
        let mut rules = vec![];
        for r in parts.next().unwrap().split(',') {
            rules.push(r.parse::<Rule>()?);
        }
        self.rules.insert(name.to_owned(), rules);
        Ok(())
    }
}

fn input(filename: &str) -> anyhow::Result<(RuleGraph, Vec<Part>)> {
    let mut first = true;
    let mut graph = RuleGraph::default();
    let mut parts = vec![];
    for line in all_lines(filename)? {
        if line.len() == 0 {
            first = false;
        } else if first {
            graph.add_line(line)?;
        } else {
            parts.push(line.parse::<Part>()?);
        }
    }
    Ok((graph, parts))
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(':') {
            let mut parts = s.split(":");
            let cond = parts.next().unwrap();
            let outcome = parts.next().unwrap();
            let mut chars = cond.chars();
            let rating = chars.next().unwrap();
            let op = chars.next().unwrap();
            let cond = if op == '<' {
                RuleCond::Less
            } else {
                RuleCond::Greater
            };
            let value = chars.collect::<String>().parse::<u128>()?;
            Ok(Self::Condition {
                rating,
                cond,
                value,
                outcome: outcome.to_owned(),
            })
        } else {
            Ok(Self::Uncondition(s.to_owned()))
        }
    }
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self::default();
        for rating in s.replace(&['{', '}'], " ").trim().split(',') {
            let mut assign = rating.split('=');
            let key = assign.next().unwrap();
            let value = assign.next().unwrap().parse::<u128>()?;
            result.ratings.insert(key.chars().next().unwrap(), value);
        }
        Ok(result)
    }
}

impl Display for RuleCond {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Less => write!(f, "<"),
            Self::Greater => write!(f, ">"),
        }
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Condition {
                rating,
                cond,
                value,
                outcome,
            } => write!(f, "{rating}{cond}{value}:{outcome},"),
            Rule::Uncondition(n) => write!(f, "{n}"),
        }
    }
}

impl Display for RuleGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.rules.iter() {
            write!(f, "{key}{{")?;
            for rule in value.iter() {
                write!(f, "{rule}")?;
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}

impl Display for Part {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut first = true;
        for (k, v) in self.ratings.iter() {
            if !first {
                write!(f, ",")?;
            }
            first = false;
            write!(f, "{k}={v}")?;
        }
        write!(f, "}}")
    }
}
