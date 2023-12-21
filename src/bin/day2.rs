use std::{cmp::max, collections::HashMap, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part};

const COLORS: [&'static str; 3] = ["red", "green", "blue"];

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        match part {
            Part::One => {
                let part1constraint = "12 red, 13 green, 14 blue".parse::<BagOfCubes>()?;
                let part1: usize = all_lines(filename)?
                    .map(|line| game_num_rest(line.as_str()))
                    .filter(|(_, rest)| {
                        rest.split("; ")
                            .map(|c| c.parse::<BagOfCubes>().unwrap())
                            .all(|c| c.possible_given(&part1constraint))
                    })
                    .map(|(game_num, _)| game_num)
                    .sum();
                println!("Part 1: {part1}");
            }
            Part::Two => {
                let part2: usize = all_lines(filename)?
                    .map(|line| {
                        let (_, rest) = game_num_rest(line.as_str());
                        rest.split("; ")
                            .map(|c| c.parse::<BagOfCubes>().unwrap())
                            .fold(BagOfCubes::default(), |c1, c2| c1.maxes(&c2))
                            .power()
                    })
                    .sum();
                println!("Part 2: {part2}");
            }
        }
        Ok(())
    })
}

fn game_num_rest(line: &str) -> (usize, String) {
    let mut game_rest = line.split(": ");
    let game = game_rest.next().unwrap();
    let game_num = game
        .split_whitespace()
        .skip(1)
        .next()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    (game_num, game_rest.next().unwrap().to_owned())
}

#[derive(Default)]
struct BagOfCubes {
    color2count: HashMap<String, usize>,
}

impl BagOfCubes {
    fn possible_given(&self, constraint: &Self) -> bool {
        self.color2count.iter().all(|(k, v)| {
            constraint
                .color2count
                .get(k.as_str())
                .map_or(false, |max| *max >= *v)
        })
    }

    fn count(&self, color: &str) -> usize {
        *self.color2count.get(color).unwrap_or(&0)
    }

    fn maxes(&self, other: &BagOfCubes) -> BagOfCubes {
        let mut result = Self::default();
        for color in COLORS.iter() {
            result.color2count.insert(
                color.to_string(),
                max(self.count(color), other.count(color)),
            );
        }
        result
    }

    fn power(&self) -> usize {
        COLORS.iter().map(|color| self.count(color)).product()
    }
}

impl FromStr for BagOfCubes {
    type Err = anyhow::Error;

    /// We expect a comma-separated list of counts and colors.
    /// e.g. `3 blue, 4 green, 5 red`
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self {
            color2count: HashMap::new(),
        };
        for color_pair in s.split(", ") {
            let parts = color_pair.split_whitespace().collect::<Vec<_>>();
            if parts.len() != 2 {
                return Err(anyhow::anyhow!("Bad format of {color_pair}"));
            }
            result
                .color2count
                .insert(parts[1].to_owned(), parts[0].parse().unwrap());
        }
        Ok(result)
    }
}
