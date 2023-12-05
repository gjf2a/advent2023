use std::str::FromStr;

use advent_code_lib::{all_lines, chooser_main, Part};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        match part {
            Part::One => {
                let mut lines = all_lines(filename)?;
                let mut seeds = get_seeds(lines.next().unwrap());
                lines.next().unwrap();
                let mut mapped_seeds = IndexSet::new();
                while let Some(line) = lines.next() {
                    match line.chars().next() {
                        None => finish_mapping(&mut seeds, &mut mapped_seeds),
                        Some(c) => match c {
                            '0'..='9' => {
                                line.parse::<Mapping>()?
                                    .remap(&mut seeds, &mut mapped_seeds);
                            }
                            'a'..='z' => {}
                            _ => return Err(anyhow::anyhow!("Illegal line start character {c}")),
                        },
                    }
                }
                finish_mapping(&mut seeds, &mut mapped_seeds);
                let part1 = seeds.iter().min().copied().unwrap();
                println!("Part 1: {part1}");
            }
            Part::Two => {}
        }
        Ok(())
    })
}

fn get_seeds(line: String) -> IndexSet<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|n| n.parse::<u64>().unwrap())
        .collect()
}

fn finish_mapping(seeds: &mut IndexSet<u64>, mapped_seeds: &mut IndexSet<u64>) {
    for seed in seeds.drain(..) {
        mapped_seeds.insert(seed);
    }
    std::mem::swap(seeds, mapped_seeds);
    *mapped_seeds = IndexSet::new();
}

struct Mapping {
    destination: u64,
    source: u64,
    range: u64,
}

impl Mapping {
    fn remap(&self, prev: &mut IndexSet<u64>, next: &mut IndexSet<u64>) {
        let start_count = prev.len() + next.len();
        let mappings = prev
            .iter()
            .filter_map(|n| self.mapping(*n).map(|m| (*n, m)))
            .collect::<Vec<_>>();
        for (prev_num, next_num) in mappings {
            prev.remove(&prev_num);
            next.insert(next_num);
        }
        assert_eq!(start_count, prev.len() + next.len());
    }

    fn mapping(&self, value: u64) -> Option<u64> {
        if (self.source..(self.source + self.range)).contains(&value) {
            Some(value + self.destination - self.source)
        } else {
            None
        }
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        let destination = nums.next().unwrap().parse()?;
        let source = nums.next().unwrap().parse()?;
        let range = nums.next().unwrap().parse()?;
        Ok(Self {
            destination,
            source,
            range,
        })
    }
}