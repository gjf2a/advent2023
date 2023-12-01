use advent_code_lib::{all_lines, chooser_main, Part};
use map_macro::hash_map;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        match part {
            Part::One => {
                let part1 = all_lines(filename)?
                    .map(|line| calibration_num(line.as_str()))
                    .sum::<u64>();
                println!("Part 1: {part1}");
            }
            Part::Two => {
                let part2 = all_lines(filename)?
                    .map(|line| calibration_num(words2digits(line.as_str()).as_str()))
                    .sum::<u64>();
                println!("Part 2: {part2}");
            }
        }

        Ok(())
    })
}

fn first_digit(line: &str) -> u64 {
    line.chars().find(|c| c.is_digit(10)).unwrap() as u64 - '0' as u64
}

fn last_digit(line: &str) -> u64 {
    line.chars().rev().find(|c| c.is_digit(10)).unwrap() as u64 - '0' as u64
}

fn calibration_num(line: &str) -> u64 {
    let one = first_digit(line);
    let two = last_digit(line);
    format!("{one}{two}").parse().unwrap()
}

fn words2digits(line: &str) -> String {
    let mut result = String::new();
    let mut i = 0;
    while i < line.len() {
        let (c, n) = eat_next(line, i);
        result.push(c);
        i = n;
    }
    result
}

fn eat_next(s: &str, pos: usize) -> (char, usize) {
    let nums = hash_map! {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
    };

    for (num, c) in nums {
        if pos + num.len() <= s.len() && &s[pos..pos + num.len()] == num {
            return (c, pos + num.len());
        }
    }

    (s[pos..pos + 1].chars().next().unwrap(), pos + 1)
}
