use std::{cmp::max, collections::VecDeque};

use advent_code_lib::{all_lines, chooser_main, DirType, ManhattanDir, Part, Position};
use enum_iterator::all;
use indexmap::{IndexMap, IndexSet};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let pipes = PipeMaze::from_file(filename)?;
        match part {
            Part::One => {
                let distances = pipes.distance_map(pipes.start);
                println!("Part one: {}", distances.values().max().unwrap());
            }
            Part::Two => {
                println!("Part two: {}", pipes.num_enclosed_tiles());
            }
        }
        Ok(())
    })
}

#[derive(Debug, Clone)]
struct PipeMaze {
    pipes: IndexMap<Position, [ManhattanDir; 2]>,
    spaces: IndexSet<Position>,
    start: Position,
    width: usize,
    height: usize,
}

impl PipeMaze {
    fn distance_map(&self, start: Position) -> IndexMap<Position, u64> {
        let mut result = IndexMap::new();
        let mut queue = VecDeque::new();
        queue.push_front((start, 0));
        while let Some((p, d)) = queue.pop_back() {
            if !result.contains_key(&p) {
                result.insert(p, d);
                for n in self.outgoing(&p) {
                    queue.push_front((n, d + 1));
                }
            }
        }
        result
    }

    fn num_enclosed_tiles(&self) -> usize {
        let mut loop_pipes_only = self.clone();
        loop_pipes_only.clear_non_loop_pipes();
        let doubled = loop_pipes_only.doubled();
        let mut spaces = loop_pipes_only.spaces.clone();
        let mut visited = IndexSet::new();
        for start in doubled.edge_spaces() {
            if !visited.contains(&start) {
                let outside = doubled.distance_map(start);
                for (out, _) in outside.iter() {
                    visited.insert(*out);
                    if out.row % 2 == 0 && out.col % 2 == 0 {
                        spaces.remove(&(*out / 2));
                    }
                }
            }
        }
        spaces.len()
    }

    fn outgoing(&self, p: &Position) -> Vec<Position> {
        if let Some(ds) = self.pipes.get(p) {
            ds.iter().map(|d| d.next_position(*p)).collect()
        } else if self.spaces.contains(p) {
            all::<ManhattanDir>()
                .map(|d| d.next_position(*p))
                .filter(|n| self.spaces.contains(n))
                .collect()
        } else {
            vec![]
        }
    }

    fn incoming(&self, p: &Position) -> Vec<(ManhattanDir, Position)> {
        all::<ManhattanDir>()
            .map(|d| (d, d.next_position(*p)))
            .filter(|(_, n)| self.outgoing(n).contains(p))
            .collect()
    }

    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut result = Self {
            pipes: IndexMap::new(),
            start: Position::new(),
            spaces: IndexSet::new(),
            width: 0,
            height: 0,
        };
        for (row, row_text) in all_lines(filename)?.enumerate() {
            for (col, pipe) in row_text.char_indices() {
                result.add_pipe(row, col, pipe)?;
                result.width = max(result.width, col + 1);
            }
            result.height = max(result.height, row + 1);
        }
        let start_incoming = result.incoming(&result.start);
        assert_eq!(2, start_incoming.len());
        result
            .pipes
            .insert(result.start, [start_incoming[0].0, start_incoming[1].0]);
        Ok(result)
    }

    fn edge_spaces(&self) -> IndexSet<Position> {
        self.spaces
            .iter()
            .filter(|s| {
                s.row == 0
                    || s.col == 0
                    || s.row == self.height as isize - 1
                    || s.col == self.width as isize - 1
            })
            .copied()
            .collect()
    }

    fn doubled(&self) -> Self {
        let mut result = Self {
            pipes: IndexMap::new(),
            start: self.start * 2,
            spaces: IndexSet::new(),
            width: self.width * 2,
            height: self.height * 2,
        };
        for space in self.spaces.iter() {
            let mapped = *space * 2;
            result.spaces.insert(mapped);
            let below = ManhattanDir::S.next_position(mapped);
            result.spaces.insert(below);
            result.spaces.insert(ManhattanDir::E.next_position(mapped));
            result.spaces.insert(ManhattanDir::E.next_position(below));
        }
        for (pipe, dirs) in self.pipes.iter() {
            let mapped = *pipe * 2;
            let south = ManhattanDir::S.next_position(mapped);
            let east = ManhattanDir::E.next_position(mapped);
            let southeast = ManhattanDir::E.next_position(south);
            if dirs.contains(&ManhattanDir::S) {
                result
                    .pipes
                    .insert(south, [ManhattanDir::N, ManhattanDir::S]);
            } else {
                result.spaces.insert(south);
            }
            if dirs.contains(&ManhattanDir::E) {
                result
                    .pipes
                    .insert(east, [ManhattanDir::W, ManhattanDir::E]);
            } else {
                result.spaces.insert(east);
            }
            result.spaces.insert(southeast);
        }
        result
    }

    fn clear_non_loop_pipes(&mut self) {
        let loop_pipes = self.distance_map(self.start);
        let non_loop_pipes = self
            .pipes
            .iter()
            .map(|(p, _)| *p)
            .filter(|p| !loop_pipes.contains_key(p))
            .collect::<Vec<_>>();
        for p in non_loop_pipes {
            self.pipes.remove(&p);
            self.spaces.insert(p);
        }
    }

    fn add_pipe(&mut self, row: usize, col: usize, pipe: char) -> anyhow::Result<()> {
        let p = Position {
            row: row as isize,
            col: col as isize,
        };
        match pipe {
            'S' => {
                self.start = p;
            }
            '|' => {
                self.pipes.insert(p, [ManhattanDir::N, ManhattanDir::S]);
            }
            '-' => {
                self.pipes.insert(p, [ManhattanDir::E, ManhattanDir::W]);
            }
            '7' => {
                self.pipes.insert(p, [ManhattanDir::W, ManhattanDir::S]);
            }
            'L' => {
                self.pipes.insert(p, [ManhattanDir::N, ManhattanDir::E]);
            }
            'F' => {
                self.pipes.insert(p, [ManhattanDir::S, ManhattanDir::E]);
            }
            'J' => {
                self.pipes.insert(p, [ManhattanDir::N, ManhattanDir::W]);
            }
            '.' => {
                self.spaces.insert(p);
            }
            _ => {
                return Err(anyhow::anyhow!("Unrecognized character '{pipe}'"));
            }
        }
        Ok(())
    }
}
