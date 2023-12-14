use std::{collections::VecDeque, fmt::Display, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut lines = all_lines(filename)?
            .map(|line| line.parse::<SpringProspect>().unwrap())
            .collect::<Vec<_>>();
        if part == Part::Two {}
        let total = lines
            .iter()
            .map(|line| line.start_combo_counts())
            .map(|combos| combos[0].iter().map(|(_, c)| *c).sum::<usize>())
            .sum::<usize>();

        println!("Part {part:?}: {total}");
        Ok(())
    })
}

#[derive(Hash, Eq, PartialEq)]
struct SpringProspect {
    codes: Vec<Code>,
    nums: Vec<usize>,
}

impl SpringProspect {
    fn all_starts(&self) -> Vec<Vec<usize>> {
        let mut result: Vec<Vec<usize>> = vec![];
        let mut earliest = 0;
        for (i, num) in self.nums.iter().enumerate() {
            let s = self.starts_for(*num, earliest, i);
            earliest = s[0] + *num + 1;
            if i > 0 {
                let latest = s[s.len() - 1] - 2;
                for j in 0..i {
                    while result[j][result[j].len() - 1] > latest {
                        result[j].pop();
                    }
                }
            }
            result.push(s);
        }
        result
    }

    fn start_combo_counts(&self) -> VecDeque<Vec<(usize, usize)>> {
        let starts = self.all_starts();
        let base_case = starts[starts.len() - 1].iter().map(|s| (*s, 1)).collect();
        let mut result: VecDeque<Vec<(usize, usize)>> = VecDeque::new();
        result.push_front(base_case);
        for row in (0..starts.len() - 1).rev() {
            let mut row_values = vec![];
            for start in starts[row].iter() {
                let mut total = 0;
                for (next, count) in result[0].iter() {
                    let end = *start + self.nums[row];
                    if end < *next {
                        if (end..*next).all(|i| self.codes[i] != Code::Damaged) {
                            total += *count;
                        }
                    }
                }
                if total > 0 {
                    row_values.push((*start, total));
                }
            }
            result.push_front(row_values);
        }
        result
    }

    fn starts_for(&self, length: usize, start: usize, length_index: usize) -> Vec<usize> {
        let mut result = vec![];
        if length < self.codes.len() {
            for potential_start in start..=self.codes.len() - length {
                if (potential_start..(potential_start + length))
                    .all(|j| self.codes[j].possible_damage())
                    && (potential_start == 0 || self.codes[potential_start - 1] != Code::Damaged)
                {
                    let next_code = potential_start + length;
                    if next_code == self.codes.len() || self.codes[next_code].possible_works() {
                        let definite_damage_after = self.codes[next_code..]
                            .iter()
                            .filter(|c| **c == Code::Damaged)
                            .count();
                        let remaining_lengths =
                            self.nums[(length_index + 1)..].iter().sum::<usize>();
                        if definite_damage_after <= remaining_lengths {
                            let definite_damage_before = self.codes[0..potential_start]
                                .iter()
                                .filter(|c| **c == Code::Damaged)
                                .count();
                            let lengths_before = self.nums[0..length_index].iter().sum::<usize>();
                            if definite_damage_before <= lengths_before {
                                result.push(potential_start);
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

#[derive(Default, Clone, Copy, Eq, PartialEq, Debug, Hash)]
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

#[cfg(test)]
mod tests {
    use crate::SpringProspect;

    #[test]
    fn test1() {
        let s = "?.?#????.? 3,1";
        println!("{s}");
        let p = s.parse::<SpringProspect>().unwrap();
        let combos = p.start_combo_counts();
        let combo_str = format!("{combos:?}");
        assert_eq!("[[(2, 3), (3, 2)], [(6, 1), (7, 1), (9, 1)]]", combo_str);
    }
}
