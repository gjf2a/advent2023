use advent_code_lib::{chooser_main, GridCharWorld, ManhattanDir, Part, Position, DirType};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut rocks = GridCharWorld::from_char_file(filename)?;
        match part {
            Part::One => {
                roll_rocks(&mut rocks, ManhattanDir::N);
                println!("Part {part:?}: {}", calculate_load(&rocks));
            }
            Part::Two => {}
        }

        Ok(())
    })
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
