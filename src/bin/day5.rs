use std::{
    cmp::{max, min},
    str::FromStr,
};

use advent_code_lib::{all_lines, chooser_main, Part};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        match part {
            Part::One => {
                let mut lines = all_lines(filename)?;
                let seeds = get_seeds(lines.next().unwrap());
                lines.next().unwrap();
                println!("Part 1: {}", seed_locator(seeds, lines)?);
            }
            Part::Two => {
                let mut lines = all_lines(filename)?;
                let seeds = get_many_seeds(lines.next().unwrap());
                lines.next().unwrap();
                println!("Part 2: {}", seed_locator(seeds, lines)?);
            }
        }
        Ok(())
    })
}

fn seed_locator(
    mut seeds: IndexSet<Interval>,
    mut lines: impl Iterator<Item = String>,
) -> anyhow::Result<u64> {
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
    println!("{seeds:?}");
    Ok(seeds.iter().map(|s| s.start).min().unwrap())
}

fn get_seeds(line: String) -> IndexSet<Interval> {
    line.split_whitespace()
        .skip(1)
        .map(|n| Interval::singleton(n.parse::<u64>().unwrap()))
        .collect()
}

fn get_many_seeds(line: String) -> IndexSet<Interval> {
    let mut result = IndexSet::new();
    let seed_nums = get_seeds(line).iter().map(|n| *n).collect::<Vec<_>>();
    for i in (0..seed_nums.len()).step_by(2) {
        result.insert(Interval {
            start: seed_nums[i].start,
            length: seed_nums[i + 1].start,
        });
    }
    result
}

fn finish_mapping(seeds: &mut IndexSet<Interval>, mapped_seeds: &mut IndexSet<Interval>) {
    for seed in seeds.drain(..) {
        mapped_seeds.insert(seed);
    }
    std::mem::swap(seeds, mapped_seeds);
    *mapped_seeds = IndexSet::new();
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Interval {
    start: u64,
    length: u64,
}

impl Interval {
    fn singleton(value: u64) -> Self {
        Self {
            start: value,
            length: 1,
        }
    }

    fn remap(&mut self, new_start: u64) {
        self.start = new_start;
    }

    fn within(&self, value: u64) -> bool {
        (self.start..self.start + self.length).contains(&value)
    }

    fn end(&self) -> u64 {
        self.start + self.length - 1
    }

    fn intersection(&self, other: &Self) -> Option<Self> {
        if other.within(self.end()) || self.within(other.end()) {
            let start = max(self.start, other.start);
            let end = min(self.end(), other.end());
            Some(Self {
                start,
                length: end - start + 1,
            })
        } else {
            None
        }
    }
}

struct Mapping {
    source: Interval,
    destination: Interval,
}

impl Mapping {
    fn remap(&self, prev: &mut IndexSet<Interval>, next: &mut IndexSet<Interval>) {
        let start_count = prev.len() + next.len();
        let mappings = prev
            .iter()
            .filter_map(|n| self.mapping(*n).map(|m| (*n, m)))
            .collect::<Vec<_>>();
        for (prev_num, next_num) in mappings {
            prev.remove(&prev_num);
            if next_num.unmoved.length > 0 {
                prev.insert(next_num.unmoved);
            }
            next.insert(next_num.moved);
        }
    }

    fn mapping(&self, value: Interval) -> Option<Remapping> {
        self.source.intersection(&value).map(|intersection| {
            let length = value.length - intersection.length;
            let unmoved = if intersection.start == value.start {
                Interval {
                    start: intersection.end() + 1,
                    length,
                }
            } else {
                Interval {
                    start: value.start,
                    length,
                }
            };
            Remapping {
                moved: Interval {start: intersection.start + self.destination.start - self.source.start, length: intersection.length},
                unmoved,
            }
        })
    }
}

#[derive(Copy, Clone)]
struct Remapping {
    moved: Interval,
    unmoved: Interval,
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nums = s.split_whitespace();
        let destination = nums.next().unwrap().parse::<u64>()?;
        let source = nums.next().unwrap().parse::<u64>()?;
        let length = nums.next().unwrap().parse::<u64>()?;
        Ok(Self {
            destination: Interval {
                start: destination,
                length,
            },
            source: Interval {
                start: source,
                length,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Interval;

    #[test]
    fn interval_test() {
        let in1 = Interval {
            start: 10,
            length: 5,
        };
        let in2 = Interval {
            start: 12,
            length: 10,
        };
    }
}
