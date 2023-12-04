use std::{cmp::min, str::FromStr};

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
            Part::Two => {
                let count_table = CardCountTable::new(&lines);
                println!("Part 2: {}", count_table.part2());
            }
        }
        Ok(())
    })
}

struct CardCountTable {
    card_counts: Vec<u64>,
}

impl CardCountTable {
    fn new(cards: &Vec<ScratchCard>) -> Self {
        let mut card_counts: Vec<u64> = std::iter::repeat(1).take(cards.len()).collect();
        for i in 0..cards.len() {
            let num_matches = cards[i].num_match() as usize;
            let end = min(i + num_matches + 1, card_counts.len());
            for j in (i + 1)..end {
                card_counts[j] += card_counts[i];
            }
        }
        Self { card_counts }
    }

    fn part2(&self) -> u64 {
        self.card_counts.iter().sum()
    }
}

struct ScratchCard {
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
        let colon = s.split(": ");
        let mut bar = colon.skip(1).next().unwrap().split(" | ");
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
            winning_numbers,
            numbers_in_hand,
        })
    }
}
