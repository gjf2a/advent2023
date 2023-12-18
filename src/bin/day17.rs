use std::{collections::{BinaryHeap, VecDeque}, cmp::{Reverse, min}};

use advent_code_lib::{chooser_main, GridDigitWorld, Position, ManhattanDir, DirType, heuristic_search_path_check};
use bare_metal_modulo::MNum;
use enum_iterator::all;
use indexmap::IndexMap;

// 736 is too high for Part 1. It enqueued 14455006 nodes and ran for 5:36.84 minutes.

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let heat_loss_map = GridDigitWorld::from_digit_file(filename)?;
        println!("height: {} width: {}", heat_loss_map.height(), heat_loss_map.width());
        println!("Part {part:?}: {}", min_heat_loss(&heat_loss_map));
        Ok(())
    })
}

struct CrucibleCostTable {
    costs: IndexMap<Position, CrucibleCosts>, 
    heat_loss_map: GridDigitWorld,
}

impl CrucibleCostTable {
    fn new(heat_loss_map: &GridDigitWorld) -> Self {
        let mut costs = IndexMap::new();
        costs.insert(Position::default(), CrucibleCosts::default());
        Self {heat_loss_map: heat_loss_map.clone(), costs}
    }

    fn costs_at(&mut self, p: Position) -> CrucibleCosts {
        if self.heat_loss_map.in_bounds(p) {
            match self.costs.get(&p) {
                Some(c) => *c,
                None => {
                    let mut costs = CrucibleCosts::maximum();
                    for incoming in all::<ManhattanDir>() {
                        let neighbor = incoming.next_position(p);
                        if self.heat_loss_map.in_bounds(neighbor) {
                            let neighbor_costs = self.costs_at(neighbor);
                        } /*else if neighbor.row < 0 && neighbor.col < self.heat_loss_map.width() as isize || neighbor.col < 0 && neighbor.row < self.heat_loss_map.height() as isize {

                        }*/
                    }
                    self.costs.insert(p, costs);
                    costs
                }
            }
        } else {
            CrucibleCosts::default()
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
struct CrucibleCosts {
    from_turn: u64,
    from_straight: [u64; 3]
}

impl CrucibleCosts {
    fn min_cost(&self) -> u64 {
        min(self.from_turn, self.from_straight.iter().min().copied().unwrap())
    }

    fn maximum() -> Self {
        Self {from_turn: u64::MAX, from_straight: [u64::MAX; 3]}
    }
}

fn min_heat_loss(heat_loss_map: &GridDigitWorld) -> u64 {
    let goal = Position {row: heat_loss_map.height() as isize - 1, col: heat_loss_map.width() as isize - 1 };
    let result = heuristic_search_path_check(Crucible::default(), |c| c.total_heat_loss, |c| c.location == goal, |c| c.estimate_to_goal(goal, &heat_loss_map), |path| within_consecutive_limit(&path, 3), |c| c.successors(heat_loss_map));
    println!("enqueued: {}", result.enqueued());
    for c in result.path().unwrap().iter() {
        print!("{} ", c.location);
    }
    println!("Approve? {}", within_consecutive_limit(&result.path().unwrap(), 3));
    println!();
    let at_goal = result.node_at_goal().unwrap();
    println!("{at_goal:?}");
    at_goal.total_heat_loss
}

fn within_consecutive_limit(path: &VecDeque<Crucible>, limit: usize) -> bool {
    let mut dirs = (0..(path.len() - 1)).map(|i| path[i].location.manhattan_dir_to(path[i + 1].location).unwrap());
    match dirs.next() {
        None => true,
        Some(mut prev) => {
            /*for p in path.iter().map(|c| c.location) {
                print!("{p} ");
            }
            print!("\n{prev:?} ");*/
            let mut consecutive = 1;
            for dir in dirs {
                //print!("{dir:?} ");
                if prev == dir {
                    consecutive += 1;
                    if consecutive > limit {
                        //println!("Exceeded limit {limit} by {consecutive} at {prev:?} to {dir:?}");
                        return false;
                    }
                } else {
                    consecutive = 1;
                }
                prev = dir;
            }
            //println!("ok");
            true
        }
    }
} 

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct Crucible {
    total_heat_loss: u64,
    location: Position,
    dir: ManhattanDir,
}

impl Default for Crucible {
    fn default() -> Self {
        Self { total_heat_loss: Default::default(), location: Default::default(), dir: ManhattanDir::E }
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
            Some(Self { total_heat_loss, location, dir })
        } else {
            None
        }
    }

    fn estimate_to_goal(&self, goal: Position, heat_loss_map: &GridDigitWorld) -> u64 {
        //println!("visiting {} {:?} {}", self.location, self.dir, self.total_heat_loss);
        self.location.manhattan_distance(goal) as u64/* 
        let mut losses = BinaryHeap::new();
        for row in self.location.row..heat_loss_map.height() as isize {
            for col in self.location.col..heat_loss_map.width() as isize {
                let p = Position {row, col};
                if p != self.location {
                    losses.push(Reverse(heat_loss_map.value(p).unwrap()));
                }
            }
        }

        let mut remaining_distance = self.location.manhattan_distance(goal);
        let mut estimate = 0;
        while remaining_distance > 0 {
            estimate += losses.pop().unwrap().0.a() as u64;
            remaining_distance -= 1;
        }
        estimate*/
    }

    fn eligible_moves(&self) -> Vec<ManhattanDir> {
        vec![self.dir.clockwise(), self.dir, self.dir.counterclockwise()]
    }
}