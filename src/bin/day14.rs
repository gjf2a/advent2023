use advent_code_lib::{
    chooser_main, GridCharWorld, ManhattanDir, Position, RowMajorPositionIterator,
};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let rocks = GridCharWorld::from_char_file(filename)?;

        Ok(())
    })
}

fn roll_rocks(rocks: &mut GridCharWorld) {
    for p in rocks.position_iter() {
        if rocks.value(p).unwrap() == 'O' {
            for above in (0..p.row).rev() {}
        }
    }
}

fn roll_north(rocks: &GridCharWorld, p: Position) -> Position {
    for above in (0..p.row).rev() {
        let candidate = Position {
            col: p.col,
            row: above,
        };
        if rocks.value(candidate).unwrap() != '.' {}
    }
    Position { col: p.col, row: 0 }
}
