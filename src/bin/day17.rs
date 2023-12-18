use advent_code_lib::{chooser_main, DirType, GridDigitWorld, ManhattanDir, Position, GridCharWorld};
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
        let mut best = None;
        loop {
            if let Some((best_cost, best_path)) = &mut best {
                table.add_level();
                if let Some((new_cost, new_path)) = table.at_goal() {
                    if *best_cost <= new_cost {
                        break;
                    } else {
                        *best_cost = new_cost;
                        *best_path = new_path;
                    }
                }
            } else {
                if let Some(result) = table.at_goal() {
                    best = Some(result);
                } else {
                    table.add_level();
                }
            }
        }
        let (best_cost, best_path) = best.unwrap();
        visualize(filename, &best_path)?;
        println!("{best_path:?}");
        println!("best cost: {best_cost} ({})", best_path.len());
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

struct CrucibleCostTable {
    table: Vec<IndexMap<Position, (u64, Vec<Position>)>>,
    heat_loss_map: GridDigitWorld,
    goal: Position,
}

impl CrucibleCostTable {
    fn new(heat_loss_map: &GridDigitWorld, start: Position, goal: Position) -> Self {
        let mut zero = IndexMap::new();
        zero.insert(start, (0, vec![start]));
        let mut one = IndexMap::new();
        for dir in all::<ManhattanDir>() {
            let neighbor = dir.next_position(start);
            if let Some(loss) = heat_loss_map.value(neighbor) {
                one.insert(neighbor, (loss.a() as u64, vec![start, neighbor]));
            }
        }
        let table = vec![zero, one];
        Self {
            table,
            heat_loss_map: heat_loss_map.clone(),
            goal,
        }
    }

    fn add_level(&mut self) {
        let mut level = IndexMap::new();
        for (pos, (cost, path)) in self.table.last().unwrap().iter() {
            let prev_dirs = self.last_n_dirs(MAX_STRAIGHT, *pos);
            for dir in all::<ManhattanDir>() {
                let dir_ok = dir != dir.inverse()
                    && (prev_dirs.len() < MAX_STRAIGHT || !prev_dirs.iter().all(|d| *d == dir));
                if dir_ok {
                    let neighbor = dir.next_position(*pos);
                    if let Some(loss) = self.heat_loss_map.value(neighbor) {
                        let neighbor_cost = *cost + loss.a() as u64;
                        let mut better = true;
                        if let Some((other_cost, _)) = level.get(&neighbor) {
                            better = neighbor_cost < *other_cost;
                        }
                        if better {
                            let mut new_path = path.clone();
                            new_path.push(neighbor);
                            level.insert(neighbor, (neighbor_cost, new_path));
                        }
                    }
                }
            }
        }
        self.table.push(level);
    }

    fn last_n_dirs(&self, n: usize, end: Position) -> Vec<ManhattanDir> {
        let (_, path) = self.table.last().unwrap().get(&end).unwrap();
        let path_start = if path.len() < n + 1 {
            0
        } else {
            path.len() - (n + 1)
        };
        (path_start..path.len() - 1)
            .map(|i| path[i].manhattan_dir_to(path[i + 1]).unwrap())
            .collect()
    }

    fn at_goal(&self) -> Option<(u64, Vec<Position>)> {
        self.table
            .last()
            .unwrap()
            .iter()
            .find(|(p, _)| **p == self.goal)
            .map(|(_, r)| r)
            .cloned()
    }
}
