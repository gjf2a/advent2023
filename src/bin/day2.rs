use std::{collections::HashMap, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let part1constraint = "12 red, 13 blue, 14 green".parse::<ColorCombo>()?;
        let part1: usize = all_lines(filename)?
        .map(|line| {
            let mut game_rest = line.split(": ");
            let game = game_rest.next().unwrap();
            let game_num = game.split_whitespace().skip(1).next().unwrap().parse::<usize>().unwrap();
            let rest = game_rest.next().unwrap();
            let possible = rest.split("; ").map(|c| c.parse::<ColorCombo>().unwrap()).all(|c| c.possible_given(&part1constraint));
            if possible {game_num} else {0}
        }).sum();
        println!("Part 1: {part1}");
        Ok(())
    })
}

struct ColorCombo {
    color2count: HashMap<String, usize>,
}

impl ColorCombo {
    fn possible_given(&self, constraint: &Self) -> bool {
        self.color2count.iter().all(|(k, v)| {
            constraint
                .color2count
                .get(k.as_str())
                .map_or(false, |max| *max >= *v)
        })
    }
}

impl FromStr for ColorCombo {
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
