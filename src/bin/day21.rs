use advent_code_lib::{
    chooser_main, GridCharWorld, Part, Position, RingIterator,
};
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
                    //println!("{table:?} {}", table.current_reachable());
                }
                println!("Part {part:?}: {}", table.current_reachable());
            }
            Part::Two => {
                if options.len() > 0 && options[0] == "saturate" {
                    println!(
                        "ignored interior: {:?}",
                        find_ignored_interior(start, &garden)
                    );
                } else {
                    let ignored_interior = find_ignored_interior(start, &garden);
                    let mut garden = garden.clone();
                    for ignored in ignored_interior {
                        garden.modify(ignored, |v| *v = '#');
                    }
                    let iterations = if options.len() > 0 {
                        options[0].parse::<usize>().unwrap()
                    } else {
                        26501365
                    };
                    let mut table = ExpandingDonut::new(start, &garden);
                    while table.expansions < iterations {
                        table.expand_once();
                        println!("{} reachable: {} (frontier: {} + donut: {})", table.expansions, table.current_reachable(), table.frontier[0].len() + table.frontier[1].len(), table.donut_holes[0].width() * table.donut_holes[0].height());
                        //table.show_garden_view();
                    }
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
    let ignored_interior = ignored
        .iter()
        .filter(|p| garden.ring_iter().all(|r| r != **p))
        .copied()
        .collect::<IndexSet<_>>();
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
        garden
            .position_value_iter()
            .filter(|(p, v)| **v != '#' && (0..2).all(|s| !self.table[s].contains(*p)))
            .map(|(p, _)| *p)
            .collect()
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
    garden: GridCharWorld,
    donut_holes: [DonutHole; 2],
    frontier: [IndexSet<Position>; 2],
    current: ModNumC<usize, 2>,
    expansions: usize,
}

#[derive(Debug)]
struct DonutHole {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
    count: u128,
    generators: Vec<Position>,
}

impl ExpandingDonut {
    fn new(start: Position, garden: &GridCharWorld) -> Self {
        let mut donut_holes = [DonutHole::new(start), DonutHole::new(start)];
        donut_holes[0].count = 1;
        donut_holes[0].generators.push(start);
        let frontier = start
            .manhattan_neighbors()
            .filter(|n| !is_blocked(garden, *n))
            .collect();
        Self {
            garden: garden.clone(),
            donut_holes,
            expansions: 1,
            current: ModNumC::new(1),
            frontier: [IndexSet::new(), frontier],
        }
    }

    fn current_reachable(&self) -> u128 {
        self.frontier[self.current.a()].len() as u128 + self.donut_holes[self.current.a()].count
    }

    fn expand_once(&mut self) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = vec![];
        for p in self.frontier[source].iter().chain(self.donut_holes[source].generators.iter()) {
            for neighbor in p.manhattan_neighbors() {
                if !is_blocked(&self.garden, bounded(neighbor, &self.garden)) {
                    insertions.push(neighbor);
                }
            }
        }
        for p in insertions {
            if !self.donut_holes[target].contains(p) {
                self.frontier[target].insert(p);
            }
        }
        if self.ring_complete(self.ring(0, 1)) {
            for i in 0..self.donut_holes.len() {
                self.donut_holes[i].generators = vec![];
                let mut counts = 0;
                for p in self.ring(i, 1) {
                    if self.frontier[i].contains(&p) {
                        counts += 1;
                        self.frontier[i].remove(&p);
                        self.donut_holes[i].generators.push(p);
                    }
                }
                self.donut_holes[i].expand(counts);
            }
        }
        self.current += 1;
        self.expansions += 1;
    }

    fn ring(&self, donut: usize, offset: isize) -> RingIterator {
        let start = Position {
            col: self.donut_holes[donut].min_x - offset,
            row: self.donut_holes[donut].min_y - offset,
        };
        let width = self.donut_holes[donut].width() + 2 * offset;
        let height = self.donut_holes[donut].height() + 2 * offset;
        RingIterator::new(start, width, height)
    }

    fn ring_complete(&self, mut ring: RingIterator) -> bool {
        /*for r in ring {
            if !(is_blocked(&self.garden, r) || self.frontier[0].contains(&r) || self.frontier[1].contains(&r)) {
                return false;
            }
            print!("r: {r} ");
        }
        println!("complete!");
        true*/
        ring.all(|r| is_blocked(&self.garden, bounded(r, &self.garden)) || self.frontier.iter().any(|f| f.contains(&r)))
    }

    fn show_garden_view(&self) {
        let mut garden_view = self.garden.clone();
        for i in 0..2 {
            println!("donut {i}: {:?}",self.donut_holes[i]);
            for row in self.donut_holes[i].min_y..=self.donut_holes[i].max_y {
                for col in self.donut_holes[i].min_x..=self.donut_holes[i].max_x {
                    let p = Position {row, col};
                    if !is_blocked(&self.garden, bounded(p, &self.garden)) {
                        garden_view.modify(p, |v| *v = 'H');
                    }
                }
            }
            for p in self.frontier[i].iter() {
                garden_view.modify(*p, |v| *v = (i as u8 + '0' as u8) as char);
            }
        }
        println!("{garden_view}");
    }
}

impl DonutHole {
    fn new(start: Position) -> Self {
        Self {
            min_x: start.col,
            min_y: start.row,
            max_x: start.col,
            max_y: start.row,
            count: 0,
            generators: vec![]
        }
    }

    fn perimeter(&self) -> isize {
        2 * (self.width() + self.height() - 2)
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

fn bounded(p: Position, garden: &GridCharWorld) -> Position {
    Position {
        row: p.row.mod_floor(&(garden.height() as isize)),
        col: p.col.mod_floor(&(garden.width() as isize)),
    }
}
