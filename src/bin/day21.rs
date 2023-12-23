use advent_code_lib::{chooser_main, GridCharWorld, Part, Position, ManhattanDir, DirType, RingIterator};
use bare_metal_modulo::{MNum, ModNumC};
use enum_iterator::all;
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
                    //println!("{table:?} {}", table.current_reachable());
                }
                println!("Part {part:?}: {}", table.current_reachable());
            }
            Part::Two => {
                if options.len() > 0 && options[0] == "saturate" {
                    println!("ignored interior: {:?}", find_ignored_interior(start, &garden));
                } else {
                    let ignored_interior = find_ignored_interior(start, &garden);
                    let iterations = if options.len() > 0 {
                        options[0].parse::<usize>().unwrap()
                    } else {
                        26501365
                    };
                    let mut table = ExpandingDonut::new(start, &garden, ignored_interior);
                    println!("{table:?}");
                    while table.expansions < iterations {
                        table.expand_once(&garden);
                        //println!("{table:?} {}", table.current_reachable());
                    }
                    println!("{:?}", table.donut_holes);
                    println!("Part {part:?}: {}", table.current_reachable());
                }
            }
        }

        Ok(())
    })
}

fn find_ignored_interior(start: Position, garden: &GridCharWorld) -> IndexSet<Position> {
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
    let ignored = table.ignored_points(&garden);
    println!("Ignored: {ignored:?}");
    let ignored_interior = ignored.iter().filter(|p| garden.ring_iter().all(|r| r != **p)).copied().collect::<IndexSet<_>>();
    println!("Ignored interior: {ignored_interior:?}");
    ignored_interior
}

fn is_blocked(garden: &GridCharWorld, p: Position) -> bool {
    garden.value(p).map_or(true, |v| v == '#')
}

#[derive(Debug)]
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

    fn ignored_points(&self, garden: &GridCharWorld) -> IndexSet<Position> {
        garden.position_value_iter().filter(|(p, v)| **v != '#' && (0..2).all(|s| !self.table[s].contains(*p))).map(|(p,_)| *p).collect()
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = vec![];
        for p in self.table[source].iter() {
            for neighbor in p.manhattan_neighbors() {
                if !is_blocked(garden, neighbor) {
                    insertions.push(neighbor);
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
struct ExpandingDonut {
    donut_holes: [DonutHole; 2],
    frontier: [IndexSet<Position>; 2],
    current: ModNumC<usize, 2>,
    expansions: usize,
    ignored_interior: IndexSet<Position>,
}

#[derive(Debug)]
struct DonutHole {
    min_x: isize, max_x: isize, min_y: isize, max_y: isize, count: u128
}

impl ExpandingDonut {
    fn new(start: Position, garden: &GridCharWorld, ignored_interior: IndexSet<Position>) -> Self {
        let mut donut_holes = [DonutHole::new(start), DonutHole::new(start)];
        donut_holes[0].count = 1;
        let frontier = start.manhattan_neighbors().filter(|n| !is_blocked(garden, *n)).collect();
        Self {donut_holes, expansions: 1, current: ModNumC::new(1), frontier: [IndexSet::new(), frontier], ignored_interior}
    }
    
    fn current_reachable(&self) -> u128 {
        self.frontier[self.current.a()].len() as u128 + self.donut_holes[self.current.a()].count
    }

    fn expand_once(&mut self, garden: &GridCharWorld) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = vec![];
        for p in self.frontier[source].iter() {
            for neighbor in p.manhattan_neighbors() {
                if !is_blocked(garden, bounded(neighbor, garden)) {
                    insertions.push(neighbor);
                }
            }
        }
        for p in insertions {
            if !self.donut_holes[target].contains(p) {
                self.frontier[target].insert(p);
            }
        }
        if self.ring_complete(garden, self.ring(1)) && self.ring_complete(garden, self.ring(2)) {
            for i in 0..self.donut_holes.len() {
                let mut counts = 0;
                for p in self.ring(1) {
                    if self.frontier[i].contains(&p) {
                        counts += 1;
                        self.frontier[i].remove(&p);
                    }
                }
                self.donut_holes[i].expand(counts);
            }
        }
        self.current += 1;
        self.expansions += 1;
    }

    fn ring(&self, offset: isize) -> RingIterator {
        let start = Position {col: self.donut_holes[0].min_x - offset, row: self.donut_holes[0].min_y - offset};
        let width = self.donut_holes[0].width() + 2 * offset;
        let height = self.donut_holes[0].height() + 2 * offset;
        RingIterator::new(start, width, height)
    }

    fn ring_complete(&self, garden: &GridCharWorld, mut ring: RingIterator) -> bool {
        ring.all(|r| self.ignored_interior.contains(&bounded(r, garden)) || is_blocked(garden, r) || self.frontier.iter().any(|f| f.contains(&r)))
    }
}

impl DonutHole {
    fn new(start: Position) -> Self {
        Self {
            min_x: start.col,
            min_y: start.row,
            max_x: start.col,
            max_y: start.row,
            count: 0
        }
    }

    fn width(&self) -> isize {
        self.max_x + 1 - self.min_x
    }

    fn height(&self) -> isize {
        self.max_y + 1 - self.min_y
    }

    fn expand(&mut self, counts: u128) {
        self.min_y -= 1;
        self.max_y += 1;
        self.min_x -= 1;
        self.max_x += 1;
        self.count += counts;
    }

    fn contains(&self, p: Position) -> bool {
        self.min_x <= p.col && p.col <= self.max_x && self.min_y <= p.row && p.row <= self.max_y
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
