use advent_code_lib::{all_lines, chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let num_nums = num_nums(filename)?;

        match part {
            Part::One => {
                let total = num_nums
                    .iter()
                    .map(|nums| find_bonus_number(nums))
                    .sum::<i64>();
                println!("Part one: {}", total);
            }
            Part::Two => println!("Part two: {}", 0),
        }
        Ok(())
    })
}

fn num_nums(filename: &str) -> anyhow::Result<Vec<Vec<i64>>> {
    Ok(all_lines(filename)?
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect())
}

fn find_bonus_number(nums: &Vec<i64>) -> i64 {
    let mut sequences = reduce_all(nums);
    augment_once(&mut sequences);
    *sequences[0].last().unwrap()
}

fn augment_once(sequences: &mut Vec<Vec<i64>>) {
    sequences.last_mut().unwrap().push(0);
    for i in (0..sequences.len() - 1).rev() {
        let my_last = *sequences[i].last().unwrap();
        let prev_last = *sequences[i + 1].last().unwrap();
        sequences[i].push(my_last + prev_last);
    }
}

fn reduce_all(nums: &Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences = vec![nums.clone()];
    let mut current = 0;
    while !all_zero(&sequences[current]) {
        assert!(sequences[current].len() > 1);
        sequences.push(reduce_once(&sequences[current]));
        current += 1;
    }
    sequences
}

fn all_zero(nums: &Vec<i64>) -> bool {
    nums.iter().all(|n| *n == 0)
}

fn reduce_once(nums: &Vec<i64>) -> Vec<i64> {
    (0..nums.len() - 1).map(|i| nums[i + 1] - nums[i]).collect()
}
