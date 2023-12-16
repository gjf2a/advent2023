use advent_code_lib::{chooser_main, GridCharWorld, ManhattanDir, Position, DirType};
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mirrors = GridCharWorld::from_char_file(filename)?;
        let activated = activate_tiles(&mirrors);
        //print_activated(&activated, &mirrors);
        println!("Part {part:?}: {}", activated.len());
        Ok(())
    })
}

fn print_activated(activated: &IndexSet<Position>, mirrors: &GridCharWorld) {
    mirrors.position_iter().for_each(|p| {
        if p.col == 0 {
            println!();
        }
        print!("{}", if activated.contains(&p) {'#'} else {'.'});
    })
}

fn activate_tiles(mirrors: &GridCharWorld) -> IndexSet<Position> {
    let mut visited = IndexSet::new();
    let mut current = IndexSet::new();
    current.insert(LightBeam::default());
    visited.insert(LightBeam::default());
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
        '/' =>  vec![beam.next_beam(match beam.dir {
            ManhattanDir::N => ManhattanDir::E,
            ManhattanDir::E => ManhattanDir::N,
            ManhattanDir::S => ManhattanDir::W,
            ManhattanDir::W => ManhattanDir::S,
        })],
        '-' => match beam.dir {
            ManhattanDir::E | ManhattanDir::W => pass,
            _ => beam.split_beam(vec![ManhattanDir::E, ManhattanDir::W]),
        }
        '|' => match beam.dir {
            ManhattanDir::N | ManhattanDir::S => pass,
            _ => beam.split_beam(vec![ManhattanDir::N, ManhattanDir::S]),
        }
        _ => panic!("Should not happen")
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct LightBeam {
    p: Position,
    dir: ManhattanDir,
}

impl Default for LightBeam {
    fn default() -> Self {
        Self {p: Position { row: 0, col: 0}, dir: ManhattanDir::E}   
    }
}

impl LightBeam {
    fn next_beam(&self, dir: ManhattanDir) -> Self {
        LightBeam {
            dir,
            p: dir.next_position(self.p)
        }
    }

    fn split_beam(&self, dirs: Vec<ManhattanDir>) -> Vec<LightBeam> {
        dirs.iter().map(|dir| self.next_beam(*dir)).collect()
    }
}