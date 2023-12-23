use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use bare_metal_modulo::{ModNum, MNum};
use enum_iterator::all;
use im::Vector;
use indexmap::{IndexSet, IndexMap};

// Part 2 is probably somewhere slightly higher than 4750, based on a different buggy solution. 
// * 4750 is definitely too low.
// * That took 15 minutes with the buggy solution.

const START: Position = Position {row: 0, col: 1};

fn goal(map: &GridCharWorld) -> Position {
    Position {
        row: map.height() as isize - 1,
        col: map.width() as isize - 2,
    }
}

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let mut table = LongPathTable::new(filename, part == Part::Two)?;
        match part {
            Part::One => {
                table.expand_fully(options.len() > 0);
                println!("Part {part:?}: {}", table.max_goal_level());
            }
            Part::Two => {
                if options.len() > 0 {
                    match options[0].as_str() {
                        "-bad" => {
                            table.expand_fully(true);
                            println!("Part {part:?}: {}", table.max_goal_level());
                        }
                        "-cycle" => {
                            let cycles = CycleChecker::get_cycles(filename)?;
                            println!("{}", cycles.map);
                            print!("Cycle points:");
                            for pt in cycles.cycle_members() {
                                print!(" {pt}");
                            }
                            println!();
                        }
                        "-j" => {
                            let map = GridCharWorld::from_char_file(filename)?;
                            let junctions = JunctionDistances::new(&map);
                            for (node, edge) in junctions.junctions2neighbors.iter() {
                                print!("{node}:");
                                for (target, weight) in edge.iter() {
                                    print!(" {target}:{weight} ");
                                }
                                println!();
                            }
                        }
                        _ => println!("Unrecognized option"),
                    }
                } else {
                    println!("Attempt a real solution here.");
                }
            }
        }

        Ok(())
    })
}

#[derive(Debug)]
struct JunctionDistances {
    junctions2neighbors: IndexMap<Position,IndexMap<Position, usize>>
}

impl JunctionDistances {
    fn new(map: &GridCharWorld) -> Self {
        let goal = goal(map);
        let mut open_list = Vector::new();
        open_list.push_back((ManhattanDir::S.next_position(START), START, START, 1));
        let mut junctions2neighbors = IndexMap::new();
        junctions2neighbors.insert(START, IndexMap::new());
        junctions2neighbors.insert(goal, IndexMap::new());
        let mut visited = IndexSet::new();
        while let Some((node, parent, last_junction, last_junction_distance)) = open_list.pop_front() {
            if !visited.contains(&(node, parent, last_junction, last_junction_distance)) {
                visited.insert((node, parent, last_junction, last_junction_distance));
                let neighbors = node.manhattan_neighbors().filter(|n| *n != parent && map.value(*n).map_or(false, |v| v != '#')).collect::<Vec<_>>();
                if neighbors.contains(&goal) {
                    update_both(last_junction_distance + 1, last_junction, goal, &mut junctions2neighbors);
                } else if neighbors.len() == 1 {
                    open_list.push_back((neighbors[0], node, last_junction, last_junction_distance + 1));
                } else if neighbors.len() > 1 {
                    update_both(last_junction_distance, last_junction, node, &mut junctions2neighbors);
                    for neighbor in neighbors {
                        open_list.push_back((neighbor, node, node, 1));
                    }
                }
            }
        }
        Self {junctions2neighbors}
    }
}

fn update_both(distance: usize, p1: Position, p2: Position, junctions2neighbors: &mut IndexMap<Position,IndexMap<Position,usize>>) {
    let ps = [p1, p2];
    for p in ps.iter() {
        if !junctions2neighbors.contains_key(p) {
            junctions2neighbors.insert(*p, IndexMap::new());
        }
    }

    for i in ModNum::new(0, ps.len()).iter() {
        update_bigger(distance, ps[i.a()], junctions2neighbors.get_mut(&ps[(i + 1).a()]).unwrap());
    }
}

fn update_bigger(distance: usize, source: Position, target_table: &mut IndexMap<Position, usize>) {
    match target_table.get_mut(&source) {
        None => {
            target_table.insert(source, distance);
        }
        Some(old_distance) => {
            if distance > *old_distance {
                *old_distance = distance;
            }
        }
    }
}

struct LongPathTable {
    map: GridCharWorld,
    paths_of_length: Vec<IndexSet<(Position, Vector<Position>)>>,
    expanding: bool,
    goal: Position,
    hike_up_slope: bool,
}

impl LongPathTable {
    fn new(filename: &str, hike_up_slope: bool) -> anyhow::Result<Self> {
        let map = GridCharWorld::from_char_file(filename)?;
        let mut start_set = IndexSet::new();
        start_set.insert((START, Vector::new()));
        let paths_of_length = vec![start_set];
        let goal = goal(&map);
        Ok(Self {
            map,
            paths_of_length,
            expanding: true,
            goal,
            hike_up_slope,
        })
    }

    fn expand(&mut self) {
        let mut expanding = false;
        let mut new_level = IndexSet::new();
        for (candidate, path) in self.paths_of_length[self.paths_of_length.len() - 1].iter() {
            if *candidate != self.goal {
                for neighbor in self.traversible_neighbors(candidate, path) {
                    let mut new_path = path.clone();
                    new_path.push_back(*candidate);
                    new_level.insert((neighbor, new_path));
                    expanding = true;
                }
            }
        }
        self.expanding = expanding;
        self.paths_of_length.push(new_level);
    }

    fn expand_fully(&mut self, show_levels: bool) {
        while self.expanding {
            self.expand();
            if show_levels {
                println!("Finished level {} (/{}) ({} nodes)", self.paths_of_length.len(), self.map.width() * self.map.height(), self.paths_of_length.last().unwrap().len());
            }
        }
    }

    fn max_goal_level(&self) -> usize {
        (0..self.paths_of_length.len())
            .rev()
            .find(|i| {
                self.paths_of_length[*i]
                    .iter()
                    .any(|(p, _)| *p == self.goal)
            })
            .unwrap()
    }

    fn traversible_neighbors<'a>(
        &'a self,
        p: &'a Position,
        path: &'a Vector<Position>,
    ) -> impl Iterator<Item = Position> + 'a {
        all::<ManhattanDir>()
            .filter(|d| {
                self.hike_up_slope
                    || match self.map.value(*p).unwrap() {
                        '>' => *d == ManhattanDir::E,
                        '<' => *d == ManhattanDir::W,
                        '^' => *d == ManhattanDir::N,
                        'v' => *d == ManhattanDir::S,
                        '.' => true,
                        _ => panic!("this shouldn't happen"),
                    }
            })
            .map(|d| d.next_position(*p))
            .filter(|n| !path.contains(n) && self.map.value(*n).map_or(false, |v| v != '#'))
    }
}

const CYCLE_CHAR: char = '@';

struct CycleChecker {
    visited: IndexSet<Position>,
    map: GridCharWorld,
}

impl CycleChecker {
    fn get_cycles(filename: &str) -> anyhow::Result<Self> {
        let map = GridCharWorld::from_char_file(filename)?;
        let mut checker = Self {map, visited: IndexSet::new()};
        checker.find_cycle_members_from(START, None);
        Ok(checker)
    }

    fn cycle_members(&self) -> impl Iterator<Item=Position> + '_ {
        self.map.position_value_iter().filter(|(_,v)| **v == CYCLE_CHAR).map(|(p,_)| *p)
    }

    fn find_cycle_members_from(&mut self, p: Position, parent: Option<Position>) {
        self.visited.insert(p);
        for neighbor in p.manhattan_neighbors().filter(|n| parent.map_or(true, |pt| pt != *n)) {
            if self.map.value(neighbor).map_or(false, |v| v != '#') {
                if self.visited.contains(&neighbor) {
                    self.map.modify(neighbor, |v| *v = CYCLE_CHAR);
                } else {
                    self.find_cycle_members_from(neighbor, Some(p));
                }
            }
        }
    }
}