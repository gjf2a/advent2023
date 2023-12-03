use std::collections::HashMap;

use advent_code_lib::{GridCharWorld, chooser_main, Part, Position};


fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let grid = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => {
                let parts = part_nums_from(&grid);
                println!("{parts:?}");
                let part1 = parts.iter().sum::<u64>();
                println!("Part 1: {part1}");
            }
            Part::Two => {
                
            }
        }
        Ok(())
    })
}

fn part_nums_from(grid: &GridCharWorld) -> Vec<u64> {
    let mut result = Vec::new();
    let mut pending = Vec::new();
    for (p, c) in grid.position_value_iter() {
        if c.is_ascii_digit() {
            pending.push((p, c));
        } else if pending.len() > 0 {
            process_pending(&mut result, &pending, grid);
            pending = Vec::new();
        }
    }
    process_pending(&mut result, &pending, grid);
    result
}

fn process_pending(result: &mut Vec<u64>, pending: &Vec<(&Position, &char)>, grid: &GridCharWorld) {
    let value = num_from_pending(&pending);
    let symbols = adjacent_symbols(&pending, grid);
    if symbols.len() > 0 {
        result.push(value);
    }
}

fn num_from_pending(pending: &Vec<(&Position, &char)>) -> u64 {
    let mut result = 0;
    for (_,c) in pending.iter() {
        result *= 10;
        result += **c as u64 - '0' as u64;
    }
    result
}

fn adjacent_symbols(pending: &Vec<(&Position, &char)>, grid: &GridCharWorld) -> HashMap<Position, char> {
    let mut result = HashMap::new();
    for (p,_) in pending.iter() {
        for n in p.neighbors() {
            if let Some(v) = grid.value(n) {
                if !v.is_ascii_digit() && v != '.' {
                    result.insert(n, v);
                }
            }
        }
    }
    result
}