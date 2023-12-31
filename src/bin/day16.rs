use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use enum_iterator::{all, All};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let mirrors = GridCharWorld::from_char_file(filename)?;
        let num_activated = match part {
            Part::One => activate_tiles(&mirrors, LightBeam::default()).len(),
            Part::Two => EdgeIterator::new(&mirrors)
                .map(|start: LightBeam| activate_tiles(&mirrors, start).len())
                .max()
                .unwrap(),
        };
        println!("Part {part:?}: {}", num_activated);
        Ok(())
    })
}

fn activate_tiles(mirrors: &GridCharWorld, start: LightBeam) -> IndexSet<Position> {
    let mut visited = IndexSet::new();
    let mut current = IndexSet::new();
    current.insert(start);
    visited.insert(start);
    while current.len() > 0 {
        let mut updated = IndexSet::new();
        for beam in current.iter() {
            for output in propagate_beam(mirrors, beam) {
                if mirrors.in_bounds(output.p) && !visited.contains(&output) {
                    visited.insert(output);
                    updated.insert(output);
                }
            }
        }
        std::mem::swap(&mut updated, &mut current);
    }
    visited.iter().map(|beam| beam.p).collect()
}

fn propagate_beam(mirrors: &GridCharWorld, beam: &LightBeam) -> Vec<LightBeam> {
    let pass = vec![beam.next_beam(beam.dir)];
    match mirrors.value(beam.p).unwrap() {
        '.' => pass,
        '\\' => vec![beam.next_beam(match beam.dir {
            ManhattanDir::N => ManhattanDir::W,
            ManhattanDir::E => ManhattanDir::S,
            ManhattanDir::S => ManhattanDir::E,
            ManhattanDir::W => ManhattanDir::N,
        })],
        '/' => vec![beam.next_beam(match beam.dir {
            ManhattanDir::N => ManhattanDir::E,
            ManhattanDir::E => ManhattanDir::N,
            ManhattanDir::S => ManhattanDir::W,
            ManhattanDir::W => ManhattanDir::S,
        })],
        '-' => match beam.dir {
            ManhattanDir::E | ManhattanDir::W => pass,
            _ => beam.split_beam(vec![ManhattanDir::E, ManhattanDir::W]),
        },
        '|' => match beam.dir {
            ManhattanDir::N | ManhattanDir::S => pass,
            _ => beam.split_beam(vec![ManhattanDir::N, ManhattanDir::S]),
        },
        _ => panic!("Should not happen"),
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct LightBeam {
    p: Position,
    dir: ManhattanDir,
}

impl Default for LightBeam {
    fn default() -> Self {
        Self {
            p: Position { row: 0, col: 0 },
            dir: ManhattanDir::E,
        }
    }
}

impl LightBeam {
    fn next_beam(&self, dir: ManhattanDir) -> Self {
        LightBeam {
            dir,
            p: dir.next_position(self.p),
        }
    }

    fn split_beam(&self, dirs: Vec<ManhattanDir>) -> Vec<LightBeam> {
        dirs.iter().map(|dir| self.next_beam(*dir)).collect()
    }
}

struct EdgeIterator {
    mirrors: GridCharWorld,
    beam_dir: ManhattanDir,
    remaining_dirs: All<ManhattanDir>,
    start_pos: Position,
    done: bool,
}

impl EdgeIterator {
    fn new(mirrors: &GridCharWorld) -> Self {
        let mut remaining_dirs = all::<ManhattanDir>();
        let beam_dir = remaining_dirs.next().unwrap();
        let start_pos = start_for(mirrors, beam_dir);
        Self {
            mirrors: mirrors.clone(),
            remaining_dirs,
            beam_dir,
            start_pos,
            done: false,
        }
    }
}

fn start_for(mirrors: &GridCharWorld, dir: ManhattanDir) -> Position {
    match dir {
        ManhattanDir::N => Position {
            row: mirrors.height() as isize - 1,
            col: 0,
        },
        ManhattanDir::E => Position { row: 0, col: 0 },
        ManhattanDir::S => Position {
            row: 0,
            col: mirrors.width() as isize - 1,
        },
        ManhattanDir::W => Position {
            row: mirrors.height() as isize - 1,
            col: mirrors.width() as isize - 1,
        },
    }
}

impl Iterator for EdgeIterator {
    type Item = LightBeam;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let result = LightBeam {
                p: self.start_pos,
                dir: self.beam_dir,
            };
            let next_pos = self.beam_dir.clockwise().next_position(self.start_pos);
            if self.mirrors.in_bounds(next_pos) {
                self.start_pos = next_pos;
            } else {
                match self.remaining_dirs.next() {
                    None => {
                        self.done = true;
                    }
                    Some(dir) => {
                        self.beam_dir = dir;
                        self.start_pos = start_for(&self.mirrors, dir);
                    }
                };
            }
            Some(result)
        }
    }
}
