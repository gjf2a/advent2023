use advent_code_lib::{
    chooser_main, DirType, GridCharWorld, GridDigitWorld, ManhattanDir, Position, Part,
};
use bare_metal_modulo::MNum;
use enum_iterator::all;
use indexmap::IndexMap;

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
        let mut i = 1;
        while !table.finished() {
            match part {
                Part::One => table.next_level(1, 3),
                Part::Two => table.next_level(4, 10),
            };
            println!("Level: {} ({})", i, table.pending.len());
            if let Some((cost, path)) = &table.solution {
                println!("Cost: {cost}");
                //visualize(filename, path)?;
            }
            i += 1;
        }
        let (best_cost, best_path) = table.solution.unwrap();
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
    pending: IndexMap<(Position, usize, ManhattanDir), (u64, Vec<ManhattanDir>, Vec<Position>)>,
    solution: Option<(u64, Vec<Position>)>,
    heat_loss_map: GridDigitWorld,
    goal: Position,
}

impl CrucibleCostTable {
    fn new(heat_loss_map: &GridDigitWorld, start: Position, goal: Position) -> Self {
        let mut one = IndexMap::new();
        for dir in all::<ManhattanDir>() {
            let neighbor = dir.next_position(start);
            if let Some(loss) = heat_loss_map.value(neighbor) {
                one.insert((neighbor, 1, dir), (loss.a() as u64, vec![dir], vec![start, neighbor]));
            }
        }
        Self {
            solution: None,
            pending: one,
            heat_loss_map: heat_loss_map.clone(),
            goal,
        }
    }

    fn next_level(&mut self, streak_min: usize, streak_max: usize) {
        let mut level: IndexMap<(Position, usize, ManhattanDir), (u64, Vec<ManhattanDir>, Vec<Position>)> = IndexMap::new();
        for ((pos, in_a_row, last_dir), (cost, dirs, path)) in self.pending.iter() {
            for dir in [*last_dir, last_dir.clockwise(), last_dir.counterclockwise()] {
                let streak = 1 + dirs.iter().rev().take_while(|d| **d == dir).count();
                let prev_streak = dirs.iter().rev().take_while(|d| *d == last_dir).count();
                if (dir == *last_dir || prev_streak >= streak_min) && streak <= streak_max {
                    let neighbor = dir.next_position(*pos);
                    if !path.contains(&neighbor) {
                        if let Some(loss) = self.heat_loss_map.value(neighbor) {
                            let mut new_path = path.clone();
                            new_path.push(neighbor);
                            let mut new_dirs = dirs.clone();
                            new_dirs.push(dir);
                            let neighbor_cost = *cost + loss.a() as u64;
                            if neighbor == self.goal {
                                let goal_cost = self.solution
                                    .as_ref()
                                    .map_or(u64::MAX, |(c, _)| *c);
                                if neighbor_cost < goal_cost {
                                    self.solution = Some((neighbor_cost, new_path));
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
        std::mem::swap(&mut self.pending, &mut level);
    }

    fn finished(&self) -> bool {
        self.pending.len() == 0
    }
}
