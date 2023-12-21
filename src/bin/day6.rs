use advent_code_lib::{all_lines, chooser_main, Part};
use anyhow::Result;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        match part {
            Part::One => {
                let races = Race::races(filename)?;
                println!("Part 1: {}", Race::score(&races));
            }
            Part::Two => {
                let race = Race::race(filename)?;
                println!("Part 2: {}", race.ways_to_beat_record());
            }
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

    fn race(filename: &str) -> Result<Self> {
        let mut lines = all_lines(filename)?;
        let time = kerning_fixed_num_from(lines.next().unwrap());
        let distance = kerning_fixed_num_from(lines.next().unwrap());
        Ok(Self { time, distance })
    }

    fn distance_traveled(&self, hold_time: u64) -> u64 {
        let race_time = self.time - hold_time;
        race_time * hold_time
    }

    fn ways_to_beat_record(&self) -> usize {
        (0..=self.time)
            .map(|t| self.distance_traveled(t))
            .filter(|d| *d > self.distance)
            .count()
    }

    fn score(races: &Vec<Self>) -> usize {
        races.iter().map(|r| r.ways_to_beat_record()).product()
    }
}

fn nums_from(line: String) -> Vec<u64> {
    line.split_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect()
}

fn kerning_fixed_num_from(line: String) -> u64 {
    line.split_whitespace()
        .skip(1)
        .collect::<String>()
        .parse()
        .unwrap()
}
