use std::cmp::max;

use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use bare_metal_modulo::{MNum, ModNumC};
use enum_iterator::all;
use im::OrdSet;
use indexmap::IndexMap;
use num_integer::Integer;

/*
Example alternates between 39 and 42 active starting at step 13.
Input alternates between 7255 and 7262 active starting at 129.
 */

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let (wrap, iterations) = match part {
            Part::One => (false, if filename.contains("ex") { 6 } else { 64 }),
            Part::Two => (
                true,
                if options.len() > 0 {
                    options[0].parse::<usize>().unwrap()
                } else {
                    26501365
                },
            ),
        };

        let mut table = UnboundedTable::new(&garden, wrap);
        for _ in 0..iterations {
            table.expand_once();
        }
        println!("Part {part:?}: {}", table.current_reachable());

        Ok(())
    })
}

fn bounded(p: Position, garden: &GridCharWorld) -> Position {
    Position {
        row: p.row.mod_floor(&(garden.height() as isize)),
        col: p.col.mod_floor(&(garden.width() as isize)),
    }
}

struct UnboundedTable {
    garden: GridCharWorld,
    table: [IndexMap<Position, OrdSet<Position>>; 2],
    current: ModNumC<usize, 2>,
    wrap: bool,
}

impl UnboundedTable {
    fn new(garden: &GridCharWorld, wrap: bool) -> Self {
        let mut table = [IndexMap::new(), IndexMap::new()];
        for i in 0..table.len() {
            for (p, v) in garden.position_value_iter().filter(|(_, v)| **v != '#') {
                table[i].insert(*p, OrdSet::new());
                if *v == 'S' && i == 0 {
                    table[0]
                        .get_mut(p)
                        .unwrap()
                        .insert(Position { row: 0, col: 0 });
                }
            }
        }
        Self {
            garden: garden.clone(),
            table: table,
            current: ModNumC::new(0),
            wrap,
        }
    }

    fn current_reachable(&self) -> u128 {
        self.table[self.current.a()]
            .iter()
            .map(|(_, sources)| sources.len() as u128)
            .sum()
    }

    fn expand_once(&mut self) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = self.table[target]
            .keys()
            .map(|k| (*k, OrdSet::<Position>::new()))
            .collect::<IndexMap<_, _>>();
        for p in self.table[target].keys() {
            for dir in all::<ManhattanDir>() {
                let neighbor = dir.next_position(*p);
                match self.garden.value(neighbor) {
                    Some(v) => {
                        if v != '#' {
                            if let Some(sources) = self.table[source].get(&neighbor) {
                                for src in sources.iter() {
                                    insertions.get_mut(p).unwrap().insert(*src);
                                }
                            }
                        }
                    }
                    None => {
                        let neighbor = bounded(neighbor, &self.garden);
                        let v = self.garden.value(neighbor).unwrap();
                        if self.wrap && v != '#' {
                            if let Some(sources) = self.table[source].get(&neighbor) {
                                for src in sources.iter() {
                                    insertions
                                        .get_mut(p)
                                        .unwrap()
                                        .insert(dir.inverse().next_position(*src));
                                }
                            }
                        }
                    }
                }
            }
        }
        for (p, sources) in insertions {
            for src in sources.iter() {
                self.table[target].get_mut(&p).unwrap().insert(*src);
            }
        }
        self.current += 1;
    }
}
