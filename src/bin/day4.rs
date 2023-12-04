use std::str::FromStr;

use advent_code_lib::{all_lines, chooser_main, Part};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let lines: Vec<ScratchCard> = all_lines(filename)?
            .map(|line| line.parse::<ScratchCard>().unwrap())
            .collect();
        match part {
            Part::One => {
                let part1 = lines.iter().map(|line| line.part1()).sum::<u64>();
                println!("Part 1: {part1}");
            }
            Part::Two => {}
        }
        Ok(())
    })
}

struct CardCountTable {}

struct ScratchCard {
    card_num: usize,
    winning_numbers: IndexSet<u64>,
    numbers_in_hand: IndexSet<u64>,
}

impl ScratchCard {
    fn num_match(&self) -> u32 {
        self.numbers_in_hand
            .intersection(&self.winning_numbers)
            .count() as u32
    }

    fn part1(&self) -> u64 {
        let num_match = self.num_match();
        if num_match >= 1 {
            2_u64.pow(num_match - 1)
        } else {
            0
        }
    }
}

impl FromStr for ScratchCard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colon = s.split(": ");
        let card_num = colon
            .next()
            .unwrap()
            .split_whitespace()
            .skip(1)
            .next()
            .unwrap()
            .parse::<usize>()?;
        let mut bar = colon.next().unwrap().split(" | ");
        let winning_numbers = bar
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        let numbers_in_hand = bar
            .next()
            .unwrap()
            .split_whitespace()
            .map(|s| s.parse::<u64>().unwrap())
            .collect();
        Ok(Self {
            card_num,
            winning_numbers,
            numbers_in_hand,
        })
    }
}
