use std::collections::HashMap;

use advent_code_lib::{chooser_main, GridCharWorld, ManhattanDir, Part, Position, DirType};

const TOTAL_CYCLES: usize = 1000000000;

// My answer of 106858 for Part 2 is too high.

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut rocks = GridCharWorld::from_char_file(filename)?;
        if part == Part::Two {
            let mut test = rocks.clone();
            let (start, length) = find_seq_start_length(&mut test);
            println!("Sequence starts at {start} and is of length {length}");
            let num_loops = (TOTAL_CYCLES - start) / length;
            let leftover = TOTAL_CYCLES - (start + num_loops * length);
            println!("This implies that the sequence loops {num_loops} times, and needs to loop {leftover} more times.");
            let emulation_loops = start + leftover;
            println!("To emulate this, we should loop {emulation_loops} times from the start state.");
            assert_eq!(TOTAL_CYCLES, emulation_loops + num_loops * length);
            let mut test2 = rocks.clone();
            for _ in 0..emulation_loops + length * 3 {
                cycle_rocks(&mut test2);
            }
            for _ in 0..emulation_loops {
                cycle_rocks(&mut rocks);
            }
            assert_eq!(test2, rocks);
        }
        
        roll_rocks(&mut rocks, ManhattanDir::N);
        println!("Part {part:?}: {}", calculate_load(&rocks));

        Ok(())
    })
}

fn find_seq_start_length(rocks: &mut GridCharWorld) -> (usize, usize) {
    let mut seen_already = HashMap::new();
    let mut when = 0;
    loop {
        seen_already.insert(rocks.clone(), when);
        cycle_rocks(rocks);
        when += 1;
        if let Some(start) = seen_already.get(&rocks) {
            return (*start, when - start);
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
