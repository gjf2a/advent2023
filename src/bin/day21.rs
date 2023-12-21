use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use enum_iterator::all;
use indexmap::{IndexMap, IndexSet};
use num_integer::Integer;


/*
Example alternates between 39 and 42 active starting at step 13.
Input alternates between 7255 and 7262 active starting at 129.
 */

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let garden = GridCharWorld::from_char_file(filename)?;
        let start = garden
            .position_value_iter()
            .find(|(_, c)| **c == 'S')
            .map(|(p, _)| *p)
            .unwrap();

        match part {
            Part::One => {
                let mut table = ReachableTable::new(start);
                let iterations = if filename.contains("ex") {6} else {64};
                for _ in 0..iterations {
                    table.expand_once(&garden);
                }
                println!("Part {part:?}: {}", table.last_row().len());
            }
            Part::Two => {
                if options[0] == "saturate" {
                    let mut table = ReachableTable::new(start);
                    let num_open = garden.position_value_iter().filter(|(_,v)| **v != '#').count();
                    let mut count = 0;
                    while table.last_row().len() < num_open {
                        count += 1;
                        table.expand_once(&garden);
                        println!("{count}: {}/{num_open}", table.last_row().len())
                    }
                    println!("Table saturated after {count} iterations.");
                } else {
                    let iterations = options[0].parse::<usize>().unwrap();
                    let mut table = InfiniteTable::new(start);
                    for i in 0..iterations {
                        println!("round {}", i + 1);
                        table.expand_once(&garden);
                    }
                    println!("Part {part:?}: {}", table.score());
                }
            }
        }

        Ok(())
    })
}

#[derive(Debug)]
struct ReachableTable {
    reachable: Vec<IndexSet<Position>>,
}

impl ReachableTable {
    fn new(start: Position) -> Self {
        let mut reachable = vec![IndexSet::new()];
        reachable[0].insert(start);
        Self { reachable }
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
    counts: IndexMap<(Position, Option<ManhattanDir>), u128>,
}

impl InfiniteTable {
    fn new(start: Position) -> Self {
        let mut counts = IndexMap::new();
        counts.insert((start, None), 1);
        Self { counts }
    }

    fn score(&self) -> u128 {
        self.counts.values().sum()
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let mut candidates = IndexMap::new();
        for ((p, _), count) in self.counts.iter() {
            for dir in all::<ManhattanDir>() {
                let neighbor = dir.next_position(*p);
                let key = (neighbor, Some(dir));
                match candidates.get_mut(&key) {
                    None => {candidates.insert(key, *count);}
                    Some(v) => *v += *count,
                }
            }
        }

        let mut new_level = IndexMap::new();
        for ((mut candidate, incoming), count) in candidates {
            wrap_in_bounds(&mut candidate, garden);
            if garden.value(candidate).unwrap() != '#' {
                if incoming.map_or(true, |incoming| {
                    !garden.at_edge(candidate) || !new_level.contains_key(&(candidate, Some(incoming.inverse())))
                }) {
                    match new_level.get_mut(&(candidate, incoming)) {
                        None => {
                            new_level.insert((candidate, incoming), count);
                        }
                        Some(new_count) => {
                            *new_count += count;
                        }
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
