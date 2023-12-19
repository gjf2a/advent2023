use std::{str::FromStr, fmt::Display};

use advent_code_lib::{chooser_main, all_lines};
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, problem| {
        let (graph, parts) = input(filename)?;
        println!("{graph}");
        for part in parts.iter() {
            println!("{part}");
        }
        let total = parts.iter().filter_map(|part| graph.accepts(part)).sum::<u64>();
        println!("Part {problem:?}: {total}");
        Ok(())
    })
}

#[derive(Clone, Default)]
struct RuleGraph {
    rules: IndexMap<String,Vec<Rule>>
}

#[derive(Default)]
struct Part {
    ratings: IndexMap<char,u64>
}

impl Part {
    fn rating(&self) -> u64 {
        self.ratings.values().sum()
    }
}

#[derive(Clone)]
enum Rule {
    Condition {rating: char, op: char, value: u64, outcome: String},
    Uncondition(String)
}

impl Rule {
    fn match_for(&self, part: &Part) -> Option<String> {
        match self {
            Self::Uncondition(s) => Some(s.clone()),
            Self::Condition { rating, op, value, outcome } => {
                match op {
                    '>' => if part.ratings.get(rating).unwrap() > value {
                        Some(outcome.clone())
                    } else {
                        None
                    },
                    '<' => if part.ratings.get(rating).unwrap() < value {
                        Some(outcome.clone())
                    } else {
                        None
                    },
                    _ => panic!("Unrecognized op {op}")
                }
            }
        }
    }
}

impl RuleGraph {
    fn accepts(&self, part: &Part) -> Option<u64> {
        let mut current = "in".to_owned();
        loop {
            for rule in self.rules.get(current.as_str()).unwrap().iter() {
                if let Some(destination) = rule.match_for(part) {
                    match destination.as_str() {
                        "A" => return Some(part.rating()),
                        "R" => return None,
                        _ => current = destination.clone()
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

fn input(filename: &str) -> anyhow::Result<(RuleGraph,Vec<Part>)> {
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
            let value = chars.collect::<String>().parse::<u64>()?;
            Ok(Self::Condition { rating, op, value, outcome: outcome.to_owned() })
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
            let value = assign.next().unwrap().parse::<u64>()?;
            result.ratings.insert(key.chars().next().unwrap(), value);
        }
        Ok(result)
    }
}

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::Condition { rating, op, value, outcome } => write!(f, "{rating}{op}{value}:{outcome},"),
            Rule::Uncondition(n) => write!(f, "{n}")
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
            if !first {write!(f, ",")?;}
            first = false;
            write!(f, "{k}={v}")?;
        }
        write!(f, "}}")
    }
}