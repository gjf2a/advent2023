use advent_code_lib::{chooser_main, Position, all_lines, ManhattanDir, DirType};
use enum_iterator::all;
use indexmap::{IndexSet, IndexMap};

// My answer for Part 1, 19675, is too low.
// My next answer, 42442, is too high.

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let outline = TrenchOutline::new(filename)?;
        println!("{}", outline.capacity());
        println!("total cells: {}", outline.total_cells());
        Ok(())
    })
}

struct TrenchOutline {
    colors: IndexMap<Position, String>,
    min_col: isize,
    max_col: isize,
    min_row: isize,
    max_row: isize,
}

impl TrenchOutline {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let mut colors = IndexMap::new();
        let mut p = Position::default();
        for line in all_lines(filename)? {
            let (dir, distance, code) = parse_line(line.as_str())?;
            for _ in 0..distance {
                p = dir.next_position(p);
                colors.insert(p, code.clone());
            }
        }
        let min_col = colors.keys().map(|p| p.col).min().unwrap();
        let min_row = colors.keys().map(|p| p.row).min().unwrap();
        let max_col = colors.keys().map(|p| p.col).max().unwrap();
        let max_row = colors.keys().map(|p| p.row).max().unwrap();
        Ok(Self { colors, min_col, max_col, min_row, max_row })
    }

    fn in_bounds(&self, p: Position) -> bool {
        self.min_col <= p.col && p.col <= self.max_col && self.min_row <= p.row && p.row <= self.max_row
    }

    fn capacity(&self) -> usize {
        let mut count = 0;
        for row in self.min_row..=self.max_row {
            for col in self.min_col..=self.max_col {
                let p = Position {row, col};
                if self.trench_hit_count(p) == all::<ManhattanDir>().count() {
                    count += 1;
                }
            }
        }
        count
    }

    fn trench_hit_count(&self, start: Position) -> usize {
        all::<ManhattanDir>().filter(|dir| self.hits_trench_in(start, *dir)).count()
    }

    fn hits_trench_in(&self, start: Position, dir: ManhattanDir) -> bool {
        let mut p = start;
        while self.in_bounds(p) {
            if self.colors.contains_key(&p) {
                return true;
            }
            p = dir.next_position(p);
        }
        false
    }

    fn total_cells(&self) -> isize {
        (self.max_row - self.min_row + 1) * (self.max_col - self.min_col + 1)
    }
}

fn parse_line(line: &str) -> anyhow::Result<(ManhattanDir, usize, String)> {
    let mut parts = line.split_whitespace();
    let dir = match parts.next().unwrap() {
        "R" => ManhattanDir::E,
        "D" => ManhattanDir::S,
        "L" => ManhattanDir::W,
        "U" => ManhattanDir::N,
        _ => Err(anyhow::anyhow!("Unrecognized movement"))?
    };
    let distance = parts.next().unwrap().parse::<usize>()?;
    let code = parts.next().unwrap().to_owned();
    Ok((dir, distance, code))
}