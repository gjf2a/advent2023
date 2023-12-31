use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use bare_metal_modulo::{MNum, ModNumC};
use enum_iterator::all;
use im::OrdSet;
use indexmap::{IndexMap, IndexSet};
use num_integer::Integer;
use point_set::PointSet;

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

        let mut table = CountingTable::new(&garden, wrap);
        for i in 0..iterations {
            table.expand_once();
            if i % 100 == 0 {
                println!("{i}: {}", table.current_reachable());
            }
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

struct CountingTable {
    garden: GridCharWorld,
    table: [IndexMap<Position, CountingRecord>; 2],
    current: usize,
    wrap: bool,
}

impl CountingTable {
    fn new(garden: &GridCharWorld, wrap: bool) -> Self {
        let mut result = Self {garden: garden.clone(), table: [IndexMap::new(), IndexMap::new()], current: 0, wrap};
        for i in 0..result.table.len() {
            for (p, v) in garden.position_value_iter().filter(|(_,v)| **v != '#') {
                result.table[i].insert(*p, if *v == 'S' && i == 0 {CountingRecord::start()} else {CountingRecord::new()});
            }
        }
        result
    }

    fn current_reachable(&self) -> u128 {
        self.table[self.current % 2]
            .iter()
            .map(|(_, sources)| sources.total_visits)
            .sum()
    }

    fn expand_once(&mut self) {
        let source = self.current % 2;
        let target = (self.current + 1) % 2;
        let target_positions = self.table[target].keys().copied().collect::<Vec<_>>();
        for target_pos in target_positions {
            let mut neighbor_visits = IndexMap::new();
            for dir in all::<ManhattanDir>() {
                if let Some(neighbor) = self.get_neighbor_for(dir, target_pos) {
                    let neighbor_visit = self.table[source].get(&neighbor).unwrap().total_visits;
                    if neighbor_visit > 0 {
                        let target_counter = self.table[target].get_mut(&target_pos).unwrap();
                        if !target_counter.earliest_signal_from.contains_key(&dir) {
                            target_counter.earliest_signal_from.insert(dir, self.current);
                        }
                        if let Some(earliest) = target_counter.earliest_signal_from.get(&dir).copied() {
                            if self.current % earliest == 0 {
                                neighbor_visits.insert(dir, neighbor_visit);
                            }
                        }
                    }
                }
            }        
            if let Some(new_visits) = neighbor_visits.values().max() {
                self.table[target].get_mut(&target_pos).unwrap().total_visits += new_visits;
            }
        }
    }

    fn get_neighbor_for(&self, dir: ManhattanDir, target_pos: Position) -> Option<Position> {
        let mut neighbor = dir.next_position(target_pos);
        if !self.garden.in_bounds(neighbor) {
            if self.wrap {
                neighbor = Position {row: neighbor.row % self.garden.height() as isize, col: neighbor.col & self.garden.width() as isize};
            } else {
                return None;
            }
        }
        if self.garden.value(neighbor).unwrap() == '#' {
            None
        } else {
            Some(neighbor)
        }
    }
}

struct CountingRecord {
    earliest_signal_from: IndexMap<ManhattanDir, usize>,
    num_visits_from: IndexMap<ManhattanDir, u128>,
    total_visits: u128,
}

impl CountingRecord {
    fn new() -> Self {
        Self {earliest_signal_from: IndexMap::new(), num_visits_from: all::<ManhattanDir>().map(|d| (d, 0)).collect(), total_visits: 0}
    }

    fn start() -> Self {
        let mut start = Self::new();
        start.total_visits = 1;
        start
    }
}

struct RegionVisitRecord {
    received: PointSet,
    pending: IndexMap<ManhattanDir, IndexSet<Position>>,
}

impl RegionVisitRecord {
    fn new() -> Self {
        let pending = all::<ManhattanDir>()
            .map(|d| (d, IndexSet::new()))
            .collect();
        Self {
            pending,
            received: PointSet::default(),
        }
    }

    fn regions_visited(&self) -> u128 {
        self.received.len() as u128
    }

    fn receive_visit_from(&mut self, region: Position) {
        if !self.received.contains(region.col as i64, region.row as i64) {
            self.received.insert(region.col as i64, region.row as i64);
            for v in self.pending.values_mut() {
                v.insert(region);
            }
        }
    }

    fn wipe_pending(&mut self, dir: ManhattanDir) {
        self.pending.insert(dir, IndexSet::new());
    }
}

struct UnboundedTable {
    garden: GridCharWorld,
    table: [IndexMap<Position, RegionVisitRecord>; 2],
    current: ModNumC<usize, 2>,
    wrap: bool,
}

impl UnboundedTable {
    fn new(garden: &GridCharWorld, wrap: bool) -> Self {
        let mut table = [IndexMap::new(), IndexMap::new()];
        for i in 0..table.len() {
            for (p, v) in garden.position_value_iter().filter(|(_, v)| **v != '#') {
                table[i].insert(*p, RegionVisitRecord::new());
                if *v == 'S' && i == 0 {
                    table[0]
                        .get_mut(p)
                        .unwrap()
                        .receive_visit_from(Position { row: 0, col: 0 });
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
            .map(|(_, sources)| sources.regions_visited() as u128)
            .sum()
    }

    fn expand_once(&mut self) {
        let source = self.current.a();
        let target = (self.current + 1).a();
        let mut insertions = self.table[target]
            .keys()
            .map(|k| (*k, OrdSet::<Position>::new()))
            .collect::<IndexMap<_, _>>();
        let mut removals = vec![];
        for p in self.table[target].keys() {
            for dir in all::<ManhattanDir>() {
                let neighbor = dir.next_position(*p);
                match self.garden.value(neighbor) {
                    Some(v) => {
                        if v != '#' {
                            if let Some(sources) = self.table[source].get(&neighbor) {
                                for src in sources.pending.get(&dir.inverse()).unwrap().iter() {
                                    insertions.get_mut(p).unwrap().insert(*src);
                                    removals.push((neighbor, dir.inverse()));
                                }
                            }
                        }
                    }
                    None => {
                        let neighbor = bounded(neighbor, &self.garden);
                        let v = self.garden.value(neighbor).unwrap();
                        if self.wrap && v != '#' {
                            if let Some(sources) = self.table[source].get(&neighbor) {
                                for src in sources.pending.get(&dir.inverse()).unwrap().iter() {
                                    insertions
                                        .get_mut(p)
                                        .unwrap()
                                        .insert(dir.inverse().next_position(*src));
                                    removals.push((neighbor, dir.inverse()));
                                }
                            }
                        }
                    }
                }
            }
        }
        for (p, sources) in insertions {
            for src in sources.iter() {
                self.table[target]
                    .get_mut(&p)
                    .unwrap()
                    .receive_visit_from(*src);
            }
        }
        for (p, dir) in removals {
            self.table[source].get_mut(&p).unwrap().wipe_pending(dir);
        }
        self.current += 1;
    }
}
