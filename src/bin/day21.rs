use advent_code_lib::{chooser_main, GridCharWorld, Position, ManhattanDir, DirType};
use enum_iterator::all;
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let start = garden.position_value_iter().find(|(_, c)| **c == 'S').map(|(p, _)| *p).unwrap();
        let mut table = ReachableTable::new(start);
        for _ in 0..64 {
            table.expand_once(&garden);
        }
        println!("{}", table.last_row().len());
        Ok(())
    })
}

#[derive(Debug)]
struct ReachableTable {
    reachable: Vec<IndexSet<Position>>
}

impl ReachableTable {
    fn new(start: Position) -> Self {
        let mut reachable = vec![IndexSet::new()];
        reachable[0].insert(start);
        Self {reachable}
    }

    fn last_row(&self) -> &IndexSet<Position> {
        &self.reachable[self.reachable.len() - 1]
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let mut next_level = IndexSet::new();
        for p in self.reachable[self.reachable.len() - 1].iter() {
            for dir in all::<ManhattanDir>() {
                let neighbor = dir.next_position(*p);
                if let Some(content) = garden.value(neighbor) {
                    if content != '#' {
                        next_level.insert(neighbor);
                    }
                }
            }
        }
        self.reachable.push(next_level);
    }
}