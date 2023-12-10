use std::collections::VecDeque;

use advent_code_lib::{chooser_main, Part, Position, ManhattanDir, all_lines, DirType};
use enum_iterator::all;
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let pipes = PipeMaze::from_file(filename)?;
        match part {
            Part::One => {
                let distances = pipes.distance_map();
                println!("Part one: {}", distances.values().max().unwrap());
            }
            Part::Two => {
                let total = 0;
                println!("Part two: {}", total);
            }
        }
        Ok(())
    })
}

#[derive(Debug)]
struct PipeMaze {
    pipes: IndexMap<Position,[ManhattanDir; 2]>,
    start: Position,
}

impl PipeMaze {
    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut pipes = IndexMap::new();
        let mut start = Position::default();
        for (row, row_text) in all_lines(filename)?.enumerate() {
            for (col, col_char) in row_text.char_indices() {
                let p = Position {row: row as isize, col: col as isize};
                match col_char {
                    'S' => {start = p;}
                    '|' => {pipes.insert(p, [ManhattanDir::N, ManhattanDir::S]);}
                    '-' => {pipes.insert(p, [ManhattanDir::E, ManhattanDir::W]);}
                    '7' => {pipes.insert(p, [ManhattanDir::W, ManhattanDir::S]);}
                    'L' => {pipes.insert(p, [ManhattanDir::N, ManhattanDir::E]);}
                    'F' => {pipes.insert(p, [ManhattanDir::S, ManhattanDir::E]);}
                    'J' => {pipes.insert(p, [ManhattanDir::N, ManhattanDir::W]);}
                    _ => {}
                }
            }
        }
        let mut result = Self {pipes, start};
        let start_incoming = result.incoming(&result.start);
        assert_eq!(2, start_incoming.len());
        result.pipes.insert(result.start, [start_incoming[0].0, start_incoming[1].0]);
        Ok(result)
    }

    fn distance_map(&self) -> IndexMap<Position, u64> {
        let mut result = IndexMap::new();
        let mut queue = VecDeque::new();
        queue.push_front((self.start, 0));
        while let Some((p, d)) = queue.pop_back() {
            if !result.contains_key(&p) {
                result.insert(p, d);
                for n in self.outgoing(&p).unwrap() {
                    queue.push_front((n, d + 1));
                }
            }
        }
        result
    }

    fn outgoing(&self, p: &Position) -> Option<[Position; 2]> {
        self.pipes.get(p).map(|ds| ds.map(|d| d.next_position(*p)))
    }

    fn incoming(&self, p: &Position) -> Vec<(ManhattanDir, Position)> {
        all::<ManhattanDir>()
        .map(|d| (d, d.next_position(*p)))
        .filter(|(_,n)| self.outgoing(n).map_or(false, |out| out.contains(p)))
        .collect()
    }
}