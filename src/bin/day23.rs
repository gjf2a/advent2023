use advent_code_lib::{chooser_main, GridCharWorld, Position, ManhattanDir, DirType};
use enum_iterator::all;
use indexmap::IndexSet;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let mut table = LongPathTable::new(filename)?;
        table.expand_fully();
        println!("Part {part:?}: {}", table.max_goal_level());
        Ok(())
    })
}

struct LongPathTable {
    map: GridCharWorld,
    paths_of_length: Vec<Vec<(Position, IndexSet<Position>)>>,
    expanding: bool,
    goal: Position,
}


impl LongPathTable {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let map = GridCharWorld::from_char_file(filename)?;
        let paths_of_length = vec![vec![(Position {row: 0, col: 1}, IndexSet::new())]];
        let goal = Position {row: map.height() as isize - 1, col: map.width() as isize - 2};
        Ok(Self {map, paths_of_length, expanding: true, goal})
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
        (0..self.paths_of_length.len()).rev().find(|i| self.paths_of_length[*i].iter().any(|(p,_)| *p == self.goal)).unwrap()
    }

    fn traversible_neighbors<'a>(&'a self, p: &'a Position, path: &'a IndexSet<Position>) -> impl Iterator<Item=Position> + 'a {
        all::<ManhattanDir>()
        .filter(|d| {
            match self.map.value(*p).unwrap() {
                '>' => *d == ManhattanDir::E,
                '<' => *d == ManhattanDir::W,
                '^' => *d == ManhattanDir::N,
                'v' => *d == ManhattanDir::S,
                '.' => true,
                _ => panic!("this shouldn't happen")
            }
        })
        .map(|d| d.next_position(*p))
        .filter(|n| !path.contains(n) && self.map.value(*n).unwrap() != '#')
    }
}