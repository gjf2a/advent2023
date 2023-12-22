use advent_code_lib::{chooser_main, GridCharWorld, Part, Position};
use bare_metal_modulo::{MNum, ModNumC};
use indexmap::IndexSet;
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
                let iterations = if filename.contains("ex") { 6 } else { 64 };
                for _ in 0..iterations {
                    table.expand_once(&garden);
                }
                println!("Part {part:?}: {}", table.current_reachable());
            }
            Part::Two => {
                if options.len() > 0 && options[0] == "saturate" {
                    let mut table = AlternationTable::new(start);
                    let num_open = garden
                        .position_value_iter()
                        .filter(|(_, v)| **v != '#')
                        .count();
                    let mut count = 0;
                    let mut prev_open = 0;
                    let mut prev_sum = 0;
                    while prev_sum < prev_open + table.current_reachable() {
                        prev_sum = prev_open + table.current_reachable();
                        prev_open = table.current_reachable();
                        count += 1;
                        table.expand_once(&garden);
                    }
                    println!(
                        "After {count} iterations, we alternate between {prev_open} and {}",
                        table.current_reachable()
                    );
                    println!(
                        "Total open squares: {num_open}; sum of alternation: {}",
                        prev_open + table.current_reachable()
                    );
                } else {
                    let iterations = if options.len() > 0 {
                        options[0].parse::<usize>().unwrap()
                    } else {
                        26501365
                    };
                    let mut table = InfiniteTable::new(start);
                    for _ in 0..iterations {
                        table.expand_once(&garden);
                        if options.len() > 1 {
                            print!("current:");
                            for p in table.table[table.current.a()].iter() {
                                print!(" {p}");
                            }
                            println!();
                        }
                    }
                    println!("Part {part:?}: {}", table.current_reachable());
                }
            }
        }

        Ok(())
    })
}

struct AlternationTable {
    table: [IndexSet<Position>; 2],
    current: ModNumC<usize, 2>,
    last_iteration_unchanged: bool,
}

impl AlternationTable {
    fn new(start: Position) -> Self {
        let mut odd = IndexSet::new();
        odd.insert(start);
        Self {
            table: [odd, IndexSet::new()],
            current: ModNumC::new(0),
            last_iteration_unchanged: false,
        }
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

struct InfiniteTable {
    table: [IndexSet<Position>; 2],
    current: ModNumC<usize, 2>,
}

impl InfiniteTable {
    fn new(start: Position) -> Self {
        let mut odd = IndexSet::new();
        odd.insert(start);
        Self {
            table: [odd, IndexSet::new()],
            current: ModNumC::new(0),
        }
    }

    fn current_reachable(&self) -> u128 {
        self.table[self.current.a()].len() as u128
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = IndexSet::new();
        for p in self.table[source].iter() {
            for neighbor in p.manhattan_neighbors() {
                if let Some(content) = garden.value(bounded(neighbor, garden)) {
                    if content != '#' {
                        insertions.insert(neighbor);
                    }
                }
            }
        }
        for p in insertions {
            self.table[target].insert(p);
        }
        self.current += 1;
    }
}

fn bounded(p: Position, garden: &GridCharWorld) -> Position {
    Position {
        row: p.row.mod_floor(&(garden.height() as isize)),
        col: p.col.mod_floor(&(garden.width() as isize)),
    }
}
