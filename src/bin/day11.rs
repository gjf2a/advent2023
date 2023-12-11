use advent_code_lib::{chooser_main, Part, GridCharWorld, Position};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let grid = galaxy_grid(filename)?;
        println!("{grid}");
        match part {
            Part::One => {
                println!("Part one: {}", 0);
            }
            Part::Two => {
                println!("Part two: {}", 0);
            }
        }
        Ok(())
    })
}

fn galaxy_grid(filename: &str) -> anyhow::Result<GridCharWorld> {
    let mut result = GridCharWorld::from_char_file(filename)?;
    let mut row_offset = 0;
    for row in 0..result.height() {
        let row = (row + row_offset) as isize;
        if (0..result.width()).all(|col| result.value(Position {row, col: col as isize}).unwrap() == '.') {
            result = result.with_new_row(row, |_| '.');
            row_offset += 1;
        }
    }

    let mut col_offset = 0;
    for col in 0..result.width() {
        let col = (col + col_offset) as isize;
        if (0..result.height()).all(|row| result.value(Position {row: row as isize, col}).unwrap() == '.') {
            result = result.with_new_column(col, |_| '.');
            col_offset += 1;
        }
    }

    Ok(result)
}

fn all_galaxies(galaxy_grid: &GridCharWorld) -> Vec<Position> {
    galaxy_grid.position_value_iter().filter(|(p, c)| **c == '#').map(|(p, c)| *p).collect()
}