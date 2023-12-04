use std::str::FromStr;

use advent_code_lib::{chooser_main, Part, all_lines};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let lines: Vec<Line> = all_lines(filename)?.map(|line| line.parse::<Line>().unwrap()).collect();
        match part {
            Part::One => {
                let part1 = lines.iter().map(|line| line.part1()).sum::<u64>();
                println!("Part 1: {part1}");
            }
            Part::Two => {
                
            }
        }
        Ok(())
    })
}

struct Line {
    card_num: u64,
    winning_numbers: IndexSet<u64>,
    numbers_in_hand: IndexSet<u64>,
}

impl Line {
    fn part1(&self) -> u64 {
        let num_match = self.numbers_in_hand.intersection(&self.winning_numbers).count() as u32;
        if num_match >= 1 {2_u64.pow(num_match - 1)} else {0}
    }
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colon = s.split(": ");
        let card_num = colon.next().unwrap().split_whitespace().skip(1).next().unwrap().parse::<u64>()?;
        let mut bar = colon.next().unwrap().split(" | ");
        let winning_numbers = bar.next().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        let numbers_in_hand = bar.next().unwrap().split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect();
        Ok(Self {card_num, winning_numbers, numbers_in_hand})
    }
}