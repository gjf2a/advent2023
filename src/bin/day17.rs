use advent_code_lib::{chooser_main, GridDigitWorld, heuristic_search, Position, ManhattanDir, DirType};
use bare_metal_modulo::MNum;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let heat_loss_map = GridDigitWorld::from_digit_file(filename)?;
        println!("Part {part:?}: {}", min_heat_loss(&heat_loss_map));
        Ok(())
    })
}

fn min_heat_loss(heat_loss_map: &GridDigitWorld) -> u64 {
    let goal = Position {row: heat_loss_map.height() as isize - 1, col: heat_loss_map.width() as isize - 1 };
    let result = heuristic_search(Crucible::default(), |c| c.total_heat_loss, |c| c.location == goal, |c| c.location.manhattan_distance(goal) as u64, |c| c.successors(heat_loss_map));
    println!("enqueued: {}", result.enqueued());
    let at_goal = result.node_at_goal().unwrap();
    println!("{at_goal:?}");
    at_goal.total_heat_loss
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Crucible {
    total_heat_loss: u64,
    location: Position,
    dir: ManhattanDir,
    last_3_moves: [Option<ManhattanDir>; 3],
}

impl Default for Crucible {
    fn default() -> Self {
        Self { total_heat_loss: Default::default(), location: Default::default(), dir: ManhattanDir::E, last_3_moves: Default::default() }
    }
}

impl Crucible {
    fn successors(&self, heat_loss_map: &GridDigitWorld) -> Vec<Self> {
        self.eligible_moves().iter().filter_map(|dir| self.successor(heat_loss_map, *dir)).collect()
    }

    fn successor(&self, heat_loss_map: &GridDigitWorld, dir: ManhattanDir) -> Option<Self> {
        let location = dir.next_position(self.location);
        if let Some(loss) = heat_loss_map.value(location) {
            let total_heat_loss = self.total_heat_loss + loss.a() as u64;
            let mut last_3_moves = [None; 3];
            for i in 0..(last_3_moves.len() - 1) {
                last_3_moves[i] = self.last_3_moves[i + 1];
            }
            last_3_moves[last_3_moves.len() - 1] = Some(dir);
            Some(Self { total_heat_loss, location, dir, last_3_moves })
        } else {
            None
        }
    }

    fn eligible_moves(&self) -> Vec<ManhattanDir> {
        let mut result = vec![self.dir.clockwise(), self.dir.counterclockwise()];
        if !self.last_3_moves.iter().all(|m| m.map_or(false, |d| d == self.dir)) {
            result.push(self.dir);
        }
        result
    }
}