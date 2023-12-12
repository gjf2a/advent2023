use std::{fmt::Display, str::FromStr};

use advent_code_lib::{chooser_main, Part, all_lines};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let result = match part {
            Part::One => {
                let prospects = all_lines(filename)?.map(|line| line.parse::<SpringProspect>().unwrap()).collect::<Vec<_>>();
                for p in prospects.iter() {
                    println!("{p}");
                    println!("{:?}", p.all_starts());
                }
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

    fn all_starts(&self) -> Vec<Vec<usize>> {
        let mut result = vec![];
        let mut earliest = 0;
        for num in self.nums.iter() {
            print!("num: {num} earliest: {earliest}");
            let s = self.starts_for(*num, earliest);
            println!(" {s:?}");
            earliest = s[0] + *num + 1;
            result.push(s);
        }
        result
    }

    fn starts_for(&self, length: usize, start: usize) -> Vec<usize> {
        let mut result = vec![];
        if length < self.codes.len() {
            for i in start..=self.codes.len() - length {
                if (i..(i + length)).all(|j| self.codes[j].possible_damage()) {
                    if i + length == self.codes.len() || self.codes[i + length].possible_works() {
                        result.push(i);
                    }
                }
            }
        }
        result
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
enum Code {
    Operational, Damaged, Unknown
}

impl Code {
    fn possible_damage(&self) -> bool {
        *self != Self::Operational
    }

    fn possible_works(&self) -> bool {
        *self != Self::Damaged
    }
}

impl Display for SpringProspect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for code in self.codes.iter() {
            write!(f, "{code}")?;
        }
        write!(f, " {}", self.nums[0])?;
        for num in self.nums.iter().skip(1) {
            write!(f, ",{num}")?;
        }
        Ok(())
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