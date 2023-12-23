use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use enum_iterator::all;
use im::Vector;
use indexmap::IndexSet;

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

fn goal(map: &GridCharWorld) -> Position {
    Position {
        row: map.height() as isize - 1,
        col: map.width() as isize - 2,
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
        checker.find_cycle_members_from(Position {row: 0, col: 1}, None);
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

struct LongPathTable {
    map: GridCharWorld,
    destinations_at_length: Vec<Vector<Position>>,
    paths_at_length: Vec<Vec<Vector<Position>>>,
    expanding: bool,
    goal: Position,
    hike_up_slope: bool,
}

impl LongPathTable {
    fn new(filename: &str, hike_up_slope: bool) -> anyhow::Result<Self> {
        let map = GridCharWorld::from_char_file(filename)?;
        let mut level_0 = Vector::new();
        level_0.push_back(Position {row: 0, col: 1});
        let destinations_at_length = vec![level_0];
        let paths_at_length = vec![vec![Vector::new()]];
        let goal = goal(&map);
        Ok(Self {
            map,
            destinations_at_length,
            paths_at_length,
            expanding: true,
            goal,
            hike_up_slope,
        })
    }

    fn last_level(&self) -> usize {
        assert_eq!(self.destinations_at_length.len(), self.paths_at_length.len());
        self.destinations_at_length.len() - 1
    }

    fn expand(&mut self) {
        let mut expanding = false;
        let mut new_destinations = Vector::new();
        let mut new_paths = vec![];
        for i in 0..self.destinations_at_length[self.last_level()].len() {
            let candidate = self.destinations_at_length[self.last_level()][i];
            if candidate != self.goal {
                let path = &self.paths_at_length[self.last_level()][i];
                for neighbor in self.traversible_neighbors(&candidate, path) {
                    if !new_destinations.contains(&neighbor) {
                        new_destinations.push_back(neighbor);
                        let mut new_path = path.clone();
                        new_path.push_back(candidate);
                        new_paths.push(new_path);
                        expanding = true;
                    }
                }
            }
        }
        self.expanding = expanding;
        self.destinations_at_length.push(new_destinations);
        self.paths_at_length.push(new_paths);
    }

    fn expand_fully(&mut self, show_levels: bool) {
        while self.expanding {
            self.expand();
            if show_levels {
                println!("Finished level {} (/{}) ({} nodes)", self.paths_at_length.len(), self.map.width() * self.map.height(), self.paths_at_length[self.last_level()].len());
            }
        }
    }

    fn max_goal_level(&self) -> usize {
        (0..self.destinations_at_length.len())
            .rev()
            .find(|i| {
                self.destinations_at_length[*i]
                    .iter()
                    .any(|p| *p == self.goal)
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
