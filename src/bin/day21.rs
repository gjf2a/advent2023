use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use bare_metal_modulo::{ModNumC, MNum};
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
                let mut table = AlternationTable::new(start);
                let iterations = if filename.contains("ex") {6} else {64};
                for _ in 0..iterations {
                    table.expand_once(&garden);
                }
                println!("Part {part:?}: {}", table.current_reachable());
            }
            Part::Two => {
                if options[0] == "saturate" {
                    let mut table = AlternationTable::new(start);
                    let num_open = garden.position_value_iter().filter(|(_,v)| **v != '#').count();
                    let mut count = 0;
                    let mut prev_open = 0;
                    let mut prev_sum = 0;
                    while prev_sum < prev_open + table.current_reachable() {
                        prev_sum = prev_open + table.current_reachable();
                        prev_open = table.current_reachable();
                        count += 1;
                        table.expand_once(&garden);
                    }
                    println!("After {count} iterations, we alternate between {prev_open} and {}", table.current_reachable());
                    println!("Total open squares: {num_open}; sum of alternation: {}", prev_open + table.current_reachable());
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

struct AlternationTable {
    table: [IndexSet<Position>; 2],
    current: ModNumC<usize,2>,
    last_iteration_unchanged: bool
}

impl AlternationTable {
    fn new(start: Position) -> Self {
        let mut odd = IndexSet::new();
        odd.insert(start);
        Self {table: [odd, IndexSet::new()], current: ModNumC::new(0), last_iteration_unchanged: false}
    }

    fn current_reachable(&self) -> usize {
        self.table[self.current.a()].len()
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = vec![];
        for p in self.table[source].iter() {
            for neighbor in p.manhattan_neighbors() {
                if let Some(content) = garden.value(neighbor) {
                    if content != '#' {
                        insertions.push(neighbor);
                    }
                }
            }
        }
        let start_len = self.table[target].len();
        for p in insertions {
            self.table[target].insert(p);
        }
        self.last_iteration_unchanged = start_len == self.table[target].len();
        self.current += 1;
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
