use advent_code_lib::{
    chooser_main, GridCharWorld, Position,
};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut rocks = GridCharWorld::from_char_file(filename)?;
        roll_rocks(&mut rocks);
        println!("{rocks}");
        println!("Part {part:?}: {}", calculate_load(&rocks));
        Ok(())
    })
}

fn calculate_load(rocks: &GridCharWorld) -> usize {
    rocks.position_value_iter()
    .filter(|(_, c)| **c == 'O')
    .map(|(p, _)| rocks.width() - p.row as usize)
    .sum()
}

fn roll_rocks(rocks: &mut GridCharWorld) {
    for p in rocks.position_iter() {
        if rocks.value(p).unwrap() == 'O' {
            let destination = roll_north(&rocks, p);
            rocks.swap(p, destination);
        }
    }
}

fn roll_north(rocks: &GridCharWorld, p: Position) -> Position {
    let mut prev = p;
    for above in (0..p.row).rev() {
        let candidate = Position {
            col: p.col,
            row: above,
        };
        if rocks.value(candidate).unwrap() != '.' {
            return prev;
        }
        prev = candidate;
    }
    Position { col: p.col, row: 0 }
}
