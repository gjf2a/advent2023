use std::{collections::VecDeque, fmt::Display, iter::repeat, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part};
use indexmap::IndexSet;

/*
Interesting case:
?.?#????.? 3,1

Five solutions:
..###.#... 3,1
..###..#.. 3,1
..###....# 3,1
...###.#.. 3,1
...###...# 3,1

I calculate 6:
[[(2, 3), (3, 2), (5, 1)], [(6, 1), (7, 1), (9, 1)]]
 */

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let result = match part {
            Part::One => {
                let prospects = all_lines(filename)?
                    .map(|line| line.parse::<SpringProspect>().unwrap())
                    .collect::<Vec<_>>();
                let mut total = 0;
                let mut total2 = 0;
                for p in prospects.iter() {
                    println!("{p}");
                    let starts = p.all_starts();
                    println!("{starts:?}");
                    let combos = p.start_combo_counts();
                    println!("{combos:?}");
                    let usable1 = p.num_can_use(&starts);
                    total += usable1;
                    let usable2 = combos[0].iter().map(|(_, c)| *c).sum::<usize>();
                    println!("usable: {usable1} ({usable2})");
                    total2 += usable2;
                }
                println!("total: {total} ({total2})");
                total
            }
            Part::Two => 999_999,
        };
        println!("Part {part:?}: {result}");
        Ok(())
    })
}

#[derive(Hash, Eq, PartialEq)]
struct SpringProspect {
    codes: Vec<Code>,
    nums: Vec<usize>,
}

impl SpringProspect {
    fn num_can_use(&self, starts: &Vec<Vec<usize>>) -> usize {
        self.all_solutions(starts).len()
    }

    fn all_solutions(&self, starts: &Vec<Vec<usize>>) -> IndexSet<Self> {
        all_combos_from(starts)
            .iter()
            .filter_map(|combo| self.solution(combo))
            .inspect(|s| println!("{s}"))
            .collect()
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
        if self.codes.iter().any(|c| *c == Code::Unknown) {
            return false;
        }

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
                if (potential_start..(potential_start + length)).all(|j| self.codes[j].possible_damage())
                    && (potential_start == 0 || self.codes[potential_start - 1] != Code::Damaged)
                {
                    let next_code = potential_start + length;
                    if next_code == self.codes.len() || self.codes[next_code].possible_works() {
                        let definite_damage_after = self.codes[next_code..].iter().filter(|c| **c == Code::Damaged).count();
                        let remaining_lengths = self.nums[(length_index + 1)..].iter().sum::<usize>();
                        //println!("ps: {potential_start} li: {length_index} nc: {next_code} dda: {definite_damage_after} rl: {remaining_lengths}");
                        if definite_damage_after <= remaining_lengths {
                            result.push(potential_start);
                        }
                    }
                }
            }
        }
        result
    }
}

fn all_combos_from(starts: &Vec<Vec<usize>>) -> Vec<VecDeque<usize>> {
    let result = all_combo_help(starts, 0);
    println!("num combos: {}", result.len());
    result
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
