use advent_code_lib::{chooser_main, GridCharWorld, Part, Position};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let expansion_factor = match part {
            Part::One => 1,
            Part::Two => 999_999,
        };
        let galaxy_grid = GridCharWorld::from_char_file(filename)?;
        let galaxies = expanded_galaxies(&galaxy_grid, expansion_factor);
        let distances = galaxy_distances(&galaxies);
        let total = distances.iter().sum::<usize>();
        println!("Part {part:?}: {total}");
        Ok(())
    })
}

fn all_galaxies(galaxy_grid: &GridCharWorld) -> Vec<Position> {
    galaxy_grid
        .position_value_iter()
        .filter(|(_, c)| **c == '#')
        .map(|(p, _)| *p)
        .collect()
}

fn expanded_galaxies(galaxy_grid: &GridCharWorld, expansion_factor: isize) -> Vec<Position> {
    let empty_rows = empty_rows(galaxy_grid);
    let empty_cols = empty_columns(galaxy_grid);
    all_galaxies(galaxy_grid)
        .iter()
        .map(|galaxy| {
            let row_expansions = empty_rows
                .iter()
                .take_while(|row| **row < galaxy.row)
                .count() as isize;
            let col_expansions = empty_cols
                .iter()
                .take_while(|col| **col < galaxy.col)
                .count() as isize;
            Position {
                row: galaxy.row + row_expansions * expansion_factor,
                col: galaxy.col + col_expansions * expansion_factor,
            }
        })
        .collect()
}

fn empty_rows(galaxy_grid: &GridCharWorld) -> Vec<isize> {
    (0..galaxy_grid.height())
        .filter(|row| (0..galaxy_grid.width()).all(|col| is_empty(galaxy_grid, *row, col)))
        .map(|row| row as isize)
        .collect()
}

fn is_empty(galaxy_grid: &GridCharWorld, row: usize, col: usize) -> bool {
    let p = Position {
        row: row as isize,
        col: col as isize,
    };
    galaxy_grid.value(p).unwrap() == '.'
}

fn empty_columns(galaxy_grid: &GridCharWorld) -> Vec<isize> {
    (0..galaxy_grid.width())
        .filter(|col| (0..galaxy_grid.height()).all(|row| is_empty(galaxy_grid, row, *col)))
        .map(|col| col as isize)
        .collect()
}

fn galaxy_distances(galaxies: &Vec<Position>) -> Vec<usize> {
    let mut result = vec![];
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            result.push(galaxies[i].manhattan_distance(galaxies[j]));
        }
    }
    result
}
