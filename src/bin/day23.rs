use advent_code_lib::{chooser_main, DirType, GridCharWorld, ManhattanDir, Part, Position};
use enum_iterator::all;
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let mut table = LongPathTable::new(filename, part == Part::Two)?;
        match part {
            Part::One => {
                table.expand_fully();
                println!("Part {part:?}: {}", table.max_goal_level());
            }
            Part::Two => {
                if options.len() > 0 {
                    match options[0].as_str() {
                        "-bad" => {
                            table.expand_fully();
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
    paths_of_length: Vec<Vec<(Position, IndexSet<Position>)>>,
    expanding: bool,
    goal: Position,
    hike_up_slope: bool,
}

impl LongPathTable {
    fn new(filename: &str, hike_up_slope: bool) -> anyhow::Result<Self> {
        let map = GridCharWorld::from_char_file(filename)?;
        let paths_of_length = vec![vec![(Position { row: 0, col: 1 }, IndexSet::new())]];
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
        let mut new_level = vec![];
        for (candidate, path) in self.paths_of_length[self.paths_of_length.len() - 1].iter() {
            if *candidate != self.goal {
                for neighbor in self.traversible_neighbors(candidate, path) {
                    let mut new_path = path.clone();
                    new_path.insert(*candidate);
                    new_level.push((neighbor, new_path));
                    expanding = true;
                }
            }
        }
        self.expanding = expanding;
        self.paths_of_length.push(new_level);
    }

    fn expand_fully(&mut self) {
        while self.expanding {
            self.expand()
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
        path: &'a IndexSet<Position>,
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
