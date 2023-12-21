use advent_code_lib::{chooser_main, GridCharWorld, Position, ManhattanDir, DirType, Part};
use enum_iterator::all;
use indexmap::{IndexSet, IndexMap};
use num_integer::Integer;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let start = garden.position_value_iter().find(|(_, c)| **c == 'S').map(|(p, _)| *p).unwrap();
        
        match part {
            Part::One => {
                let mut table = ReachableTable::new(start);
                for _ in 0..64 {
                    table.expand_once(&garden);
                }
                println!("Part {part:?}: {}", table.last_row().len());
            }
            Part::Two => {
                let mut table = InfiniteTable::new(start);
                for i in 0..10 {
                    println!("round {}", i + 1);
                    table.expand_once(&garden);
                }
                println!("Part {part:?}: {}", table.score());
            }
        }
        
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

#[derive(Debug)]
struct InfiniteTable {
    counts: IndexMap<Position,u128>
}

impl InfiniteTable {
    fn new(start: Position) -> Self {
        let mut counts = IndexMap::new();
        counts.insert(start, 1);
        Self {counts}
    }

    fn score(&self) -> u128 {
        self.counts.values().sum()
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let mut candidates = IndexMap::new();
        for (p, count) in self.counts.iter() {
            for dir in all::<ManhattanDir>() {
                let neighbor = dir.next_position(*p);
                candidates.insert(neighbor, count);
                print!("{neighbor}({count}) ");
            }
        }
        println!();

        let mut new_level = IndexMap::new();
        for (mut candidate, count) in candidates {
            wrap_in_bounds(&mut candidate, garden);
            if garden.value(candidate).unwrap() != '#' {
                match new_level.get_mut(&candidate) {
                    None => {new_level.insert(candidate, *count);}
                    Some(new_count) => {
                        *new_count += *count;
                    }
                }
            }
        }
        std::mem::swap(&mut self.counts, &mut new_level);
    }    
}

fn wrap_in_bounds(p: &mut Position, garden: &GridCharWorld) {
    p.col = p.col.mod_floor(&(garden.width() as isize));
    p.row = p.row.mod_floor(&(garden.height() as isize));
}