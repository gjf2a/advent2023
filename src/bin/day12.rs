use std::{collections::VecDeque, fmt::Display, iter::repeat, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part};

// Part 1 does not work: 9263 is too high.
// This solution: 10613!!!

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let result = match part {
            Part::One => {
                let prospects = all_lines(filename)?
                    .map(|line| line.parse::<SpringProspect>().unwrap())
                    .collect::<Vec<_>>();
                let mut total = 0;
                for p in prospects.iter() {
                    println!("{p}");
                    let starts = p.all_starts();
                    let usable = p.num_can_use(&starts);
                    println!("usable: {usable}");
                    total += usable;
                }
                total
            }
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
    fn num_can_use(&self, starts: &Vec<Vec<usize>>) -> usize {
        all_combos_from(starts)
            .iter()
            .filter_map(|combo| self.solution(combo))
            .count()
    }

    fn solution(&self, starts: &VecDeque<usize>) -> Option<Self> {
        assert_eq!(starts.len(), self.nums.len());
        let codes = repeat(Code::Operational).take(self.codes.len()).collect();
        let mut solution = Self {
            codes,
            nums: self.nums.clone(),
        };
        for (i, start) in starts.iter().enumerate() {
            for j in *start..(*start + self.nums[i]) {
                solution.codes[j] = Code::Damaged;
            }
        }

        let retained_known = (0..self.codes.len())
            .all(|i| self.codes[i] == Code::Unknown || self.codes[i] == solution.codes[i]);
        if retained_known && solution.is_valid_solution() {
            Some(solution)
        } else {
            None
        }
    }

    fn is_valid_solution(&self) -> bool {
        let num_damaged = self.codes.iter().filter(|c| **c == Code::Damaged).count();
        if num_damaged != self.nums.iter().sum() {
            return false;
        }

        let mut sequences_left = self.nums.iter().collect::<VecDeque<_>>();
        let mut i = 0;
        while let Some(mut seq) = sequences_left.pop_front().copied() {
            while self.codes[i] == Code::Operational {
                i += 1;
                if i == self.codes.len() {
                    return false;
                }
            }
            while seq > 0 {
                if self.codes[i] != Code::Damaged {
                    return false;
                }
                seq -= 1;
                i += 1;
                if i == self.codes.len() && seq > 0 {
                    return false;
                }
            }
            if i < self.codes.len() && self.codes[i] != Code::Operational {
                return false;
            }
        }
        while i < self.codes.len() {
            if self.codes[i] != Code::Operational {
                return false;
            }
            i += 1;
        }

        true
    }

    fn all_starts(&self) -> Vec<Vec<usize>> {
        let mut result = vec![];
        let mut earliest = 0;
        for num in self.nums.iter() {
            let s = self.starts_for(*num, earliest);
            earliest = s[0] + *num + 1;
            result.push(s);
        }
        result
    }

    fn starts_for(&self, length: usize, start: usize) -> Vec<usize> {
        let mut result = vec![];
        if length < self.codes.len() {
            for i in start..=self.codes.len() - length {
                if (i..(i + length)).all(|j| self.codes[j].possible_damage())
                    && (i == 0 || self.codes[i - 1] != Code::Damaged)
                {
                    if i + length == self.codes.len() || self.codes[i + length].possible_works() {
                        result.push(i);
                    }
                }
            }
        }
        result
    }
}

fn all_combos_from(starts: &Vec<Vec<usize>>) -> Vec<VecDeque<usize>> {
    all_combo_help(starts, 0)
}

fn all_combo_help(starts: &Vec<Vec<usize>>, i: usize) -> Vec<VecDeque<usize>> {
    if i == starts.len() - 1 {
        starts[i].iter().map(|n| VecDeque::from([*n])).collect()
    } else {
        let mut result = vec![];
        let options = all_combo_help(starts, i + 1);
        for start in starts[i].iter() {
            for option in options.iter() {
                let mut version = option.clone();
                version.push_front(*start);
                result.push(version);
            }
        }
        result
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug)]
enum Code {
    #[default]
    Operational,
    Damaged,
    Unknown,
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
        let codes = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| c.try_into().unwrap())
            .collect();
        let nums = parts
            .next()
            .unwrap()
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        Ok(Self { codes, nums })
    }
}

impl TryFrom<char> for Code {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '?' => Ok(Self::Unknown),
            '.' => Ok(Self::Operational),
            '#' => Ok(Self::Damaged),
            _ => Err(anyhow::anyhow!("Unrecognized character '{value}'")),
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
