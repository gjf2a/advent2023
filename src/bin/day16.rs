use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use enum_iterator::{all, All};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mirrors = GridCharWorld::from_char_file(filename)?;
        let activated = match part {
            Part::One => activate_tiles(&mirrors, LightBeam::default()),
            Part::Two => {
                for start in EdgeIterator::new(&mirrors) {
                    println!("{start:?}");
                }
                activate_tiles(&mirrors, LightBeam::default()) // just to compile
            }
        };
        println!("Part {part:?}: {}", activated.len());
        Ok(())
    })
}

struct EdgeIterator {
    mirrors: GridCharWorld,
    current_beam_dir: ManhattanDir,
    current_pos_advance_dir: ManhattanDir,
    remaining_dirs: All<ManhattanDir>,
    current_pos: Position,
    done: bool,
}

impl EdgeIterator {
    fn new(mirrors: &GridCharWorld) -> Self {
        let mut remaining_dirs = all::<ManhattanDir>();
        let current_beam_dir = remaining_dirs.next().unwrap();
        let current_pos_advance_dir = current_beam_dir.clockwise();
        let current_pos = start_for(mirrors, current_beam_dir);
        Self {
            mirrors: mirrors.clone(),
            remaining_dirs: all::<ManhattanDir>(),
            current_beam_dir,
            current_pos_advance_dir,
            current_pos,
            done: false,
        }
    }
}

fn start_for(mirrors: &GridCharWorld, dir: ManhattanDir) -> Position {
    match dir {
        ManhattanDir::N => Position {row: mirrors.height() as isize - 1, col: 0},
        ManhattanDir::E => Position {row: 0, col: 0},
        ManhattanDir::S => Position {row: 0, col: mirrors.width() as isize - 1},
        ManhattanDir::W => Position {row: mirrors.height() as isize - 1, col: mirrors.width() as isize - 1}
    }
}

impl Iterator for EdgeIterator {
    type Item = LightBeam;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let next_pos = self.current_beam_dir.next_position(self.current_pos);
            if self.mirrors.in_bounds(next_pos) {
                let result = LightBeam {p: next_pos, dir: self.current_beam_dir};
                self.current_pos = next_pos;
                Some(result)
            } else {
                match self.remaining_dirs.next() {
                    None => {
                        self.done = true;
                        None
                    }
                    Some(dir) => {
                        self.current_beam_dir = dir;
                        self.current_pos_advance_dir = dir.clockwise();
                        self.current_pos = start_for(&self.mirrors, dir);
                        Some(LightBeam {p: self.current_pos, dir})
                    }
                }
            }
        }
    }
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
