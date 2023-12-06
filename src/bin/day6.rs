use advent_code_lib::{all_lines, chooser_main, Part};
use anyhow::Result;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let races = Race::races(filename)?;
        match part {
            Part::One => {
                let part1 = races
                    .iter()
                    .map(|r| r.ways_to_beat_record())
                    //.inspect(|n| {println!("ways to win: {n}")})
                    .product::<usize>();
                println!("Part 1: {part1}");
            }
            Part::Two => {}
        }
        Ok(())
    })
}

#[derive(Copy, Clone, Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn races(filename: &str) -> Result<Vec<Self>> {
        let mut result = vec![];
        let mut lines = all_lines(filename)?;
        let times = nums_from(lines.next().unwrap());
        let distances = nums_from(lines.next().unwrap());
        assert_eq!(times.len(), distances.len());
        for i in 0..times.len() {
            result.push(Self {
                time: times[i],
                distance: distances[i],
            });
        }
        Ok(result)
    }

    fn distance_traveled(&self, hold_time: u64) -> u64 {
        let race_time = self.time - hold_time;
        race_time * hold_time
    }

    fn ways_to_beat_record(&self) -> usize {
        //println!("Race: {self:?}");
        (0..=self.time)
            .map(|t| self.distance_traveled(t))
            //.inspect(|t| {println!("time: {t}");})
            .filter(|d| *d > self.distance)
            .count()
    }
}

fn nums_from(line: String) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}
