use std::{fmt::Display, str::FromStr};

use advent_code_lib::{chooser_main, Part, all_lines};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let result = match part {
            Part::One => {
                let prospects = all_lines(filename)?.map(|line| line.parse::<SpringProspect>().unwrap()).collect::<Vec<_>>();
                prospects.iter().map(|p| p.num_unknown()).max().unwrap()
                
            },
            Part::Two => 999_999,
        };
        println!("Part {part:?}: {result}");
        Ok(())
    })
}

struct SpringProspect {
    codes: Vec<Code>,
    nums: Vec<usize>,
}

impl SpringProspect {
    fn num_unknown(&self) -> usize {
        self.codes.iter().filter(|c| **c == Code::Unknown).count()
    }
}

impl FromStr for SpringProspect {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let codes = parts.next().unwrap().chars().map(|c| c.try_into().unwrap()).collect();
        let nums = parts.next().unwrap().split(',').map(|c| c.parse::<usize>().unwrap()).collect();
        Ok(Self {codes, nums})
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Code {
    Operational, Damaged, Unknown
}

impl TryFrom<char> for Code {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Self::Unknown),
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            _ => Err(anyhow::anyhow!("Unrecognized character '{value}'"))
        }
    }
}

impl Display for Code {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Code::Operational => '.',
            Code::Damaged => '#',
            Code::Unknown => '?',
        };
        write!(f, "{c}")
    }
}