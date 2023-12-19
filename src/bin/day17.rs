use advent_code_lib::{
    chooser_main, DirType, GridCharWorld, GridDigitWorld, ManhattanDir, Position,
};
use bare_metal_modulo::MNum;
use enum_iterator::all;
use indexmap::IndexMap;

const MAX_STRAIGHT: usize = 3;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let heat_loss_map = GridDigitWorld::from_digit_file(filename)?;
        println!(
            "height: {} width: {}",
            heat_loss_map.height(),
            heat_loss_map.width()
        );
        let mut table = CrucibleCostTable::new(
            &heat_loss_map,
            Position { row: 0, col: 0 },
            Position {
                row: heat_loss_map.height() as isize - 1,
                col: heat_loss_map.width() as isize - 1,
            },
        );
        while !table.finished() {
            table.add_level();
        }
        let (best_cost, best_path) = table.best().unwrap();
        for p in best_path.iter() {
            print!("{p} ");
        }
        println!();
        visualize(filename, &best_path)?;
        println!("Heat loss: {best_cost}");
        //table.dump();
        Ok(())
    })
}

fn visualize(filename: &str, path: &Vec<Position>) -> anyhow::Result<()> {
    let mut chars = GridCharWorld::from_char_file(filename)?;
    for i in 1..path.len() {
        let c = match path[i - 1].manhattan_dir_to(path[i]).unwrap() {
            ManhattanDir::N => '^',
            ManhattanDir::E => '>',
            ManhattanDir::S => 'v',
            ManhattanDir::W => '<',
        };
        chars.modify(path[i], |v| *v = c);
    }
    println!("{chars}");
    Ok(())
}

#[derive(Debug)]
struct CrucibleCostTable {
    pending: Vec<IndexMap<(Position, usize, ManhattanDir), (u64, Vec<ManhattanDir>, Vec<Position>)>>,
    goal_at_level: Vec<Option<(u64, Vec<Position>)>>,
    heat_loss_map: GridDigitWorld,
    goal: Position,
}

impl CrucibleCostTable {
    fn dump(&self) {
        for (i, level) in self.pending.iter().enumerate() {
            println!("Level {i}");
            let mut keys = level.keys().collect::<Vec<_>>();
            keys.sort();
            for (p, s, incoming) in keys {
                let (c, ds, v) = level.get(&(*p, *s, *incoming)).unwrap();
                print!("{p} {s} {c}:");
                for prev in v.iter() {
                    print!(" {prev}");
                }
                println!();
            }
            println!();
        }
    }

    fn new(heat_loss_map: &GridDigitWorld, start: Position, goal: Position) -> Self {
        let mut zero = IndexMap::new();
        let mut one = IndexMap::new();
        for dir in all::<ManhattanDir>() {
            let neighbor = dir.next_position(start);
            if let Some(loss) = heat_loss_map.value(neighbor) {
                one.insert((neighbor, 1, dir), (loss.a() as u64, vec![dir], vec![start, neighbor]));
            }
        }
        let table = vec![zero, one];
        Self {
            goal_at_level: vec![None],
            pending: table,
            heat_loss_map: heat_loss_map.clone(),
            goal,
        }
    }

    fn add_level(&mut self) {
        let level_num = self.goal_at_level.len();
        self.goal_at_level.push(None);
        let mut level: IndexMap<(Position, usize, ManhattanDir), (u64, Vec<ManhattanDir>, Vec<Position>)> = IndexMap::new();
        for ((pos, in_a_row, last_dir), (cost, dirs, path)) in self.pending.last().unwrap().iter() {
            let dir_start = if dirs.len() < MAX_STRAIGHT {dirs.len()} else {dirs.len() - MAX_STRAIGHT};
            let prev_dirs = &dirs[dir_start..];
            for dir in [*last_dir, last_dir.clockwise(), last_dir.counterclockwise()] {
                let streak = 1 + prev_dirs.iter().rev().take_while(|d| **d == dir).count();
                if streak <= MAX_STRAIGHT {
                    let neighbor = dir.next_position(*pos);
                    if !path.contains(&neighbor) {
                        if let Some(loss) = self.heat_loss_map.value(neighbor) {
                            let mut new_path = path.clone();
                            new_path.push(neighbor);
                            let mut new_dirs = dirs.clone();
                            new_dirs.push(dir);
                            let neighbor_cost = *cost + loss.a() as u64;
                            if neighbor == self.goal {
                                let goal_cost = self.goal_at_level[level_num]
                                    .as_ref()
                                    .map_or(u64::MAX, |(c, _)| *c);
                                if neighbor_cost < goal_cost {
                                    self.goal_at_level[level_num] = Some((neighbor_cost, new_path));
                                }
                            } else {
                                let better = level
                                    .get(&(neighbor, streak, dir))
                                    .map_or(true, |(other_cost, _, _)| neighbor_cost < *other_cost);
                                if better {
                                    level.insert((neighbor, streak, dir), (neighbor_cost, new_dirs, new_path));
                                }
                            }
                        }
                    }
                }
            }
        }
        self.pending.push(level);
    }

    fn finished(&self) -> bool {
        self.pending.last().unwrap().len() == 0
    }

    fn best(&self) -> Option<(u64, Vec<Position>)> {
        self.goal_at_level
            .iter()
            .filter_map(|x| x.clone())
            .min_by_key(|(cost, _)| *cost)
    }
}
