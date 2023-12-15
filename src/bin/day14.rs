use std::collections::HashMap;

use advent_code_lib::{chooser_main, GridCharWorld, ManhattanDir, Part, Position, DirType};

const TOTAL_CYCLES: usize = 1000000000;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut rocks = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => {
                roll_rocks(&mut rocks, ManhattanDir::N);
            }
            Part::Two => {
                cycle_to_target(&mut rocks);
            }
        }
        
        println!("Part {part:?}: {}", calculate_load(&rocks));
        Ok(())
    })
}

fn cycle_to_target(rocks: &mut GridCharWorld) {
    let mut seen_already = HashMap::new();
    let mut when = 0;
    loop {
        seen_already.insert(rocks.clone(), when);
        cycle_rocks(rocks);
        when += 1;
        if let Some(start) = seen_already.get(&rocks) {
            let period = when - start;
            let num_additional_periods = (TOTAL_CYCLES - when) / period;
            when += num_additional_periods * period;
            while when < TOTAL_CYCLES {
                cycle_rocks(rocks);
                when += 1;
            }
            return;
        }
    }
}

fn cycle_rocks(rocks: &mut GridCharWorld) {
    for dir in [ManhattanDir::N, ManhattanDir::W, ManhattanDir::S, ManhattanDir::E] {
        roll_rocks(rocks, dir)
    }
}

fn rock_positions(rocks: &GridCharWorld) -> impl Iterator<Item=Position> + '_ {
    rocks.position_value_iter().filter(|(_, c)| **c == 'O').map(|(p,_)| *p)
}

fn calculate_load(rocks: &GridCharWorld) -> usize {
    rock_positions(rocks).map(|p| rocks.width() - p.row as usize).sum()
}

fn roll_rocks(rocks: &mut GridCharWorld, dir: ManhattanDir) {
    let sorted_rocks = find_sorted_rocks(rocks, dir);
    for rock in sorted_rocks {
        let destination = roll_rock(&rocks, rock, dir);
        rocks.swap(rock, destination);
    }
}

fn find_sorted_rocks(rocks: &GridCharWorld, dir: ManhattanDir) -> Vec<Position> {
    let mut result = rock_positions(rocks).collect::<Vec<_>>();
    result.sort_by_key(|k| dir_key(k, dir));
    result
}

fn dir_key(p: &Position, dir: ManhattanDir) -> isize {
    match dir {
        ManhattanDir::N => p.row,
        ManhattanDir::E => -p.col,
        ManhattanDir::S => -p.row,
        ManhattanDir::W => p.col,
    }
}

fn roll_rock(rocks: &GridCharWorld, p: Position, dir: ManhattanDir) -> Position {
    let mut prev = p;
    loop {
        let candidate = dir.next_position(prev);
        if !rocks.in_bounds(candidate) || rocks.value(candidate).unwrap() != '.' {
            return prev;
        }
        prev = candidate;
    }
}
