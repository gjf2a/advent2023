use std::{cmp::Reverse, collections::BinaryHeap};

use advent_code_lib::{
    chooser_main, heuristic_search, DirType, GridCharWorld, GridDigitWorld, ManhattanDir, Part,
    Position,
};
use bare_metal_modulo::MNum;
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        let heat_loss_map = GridDigitWorld::from_digit_file(filename)?;
        println!(
            "height: {} width: {}",
            heat_loss_map.height(),
            heat_loss_map.width()
        );
        let (streak_min, streak_max) = match part {
            Part::One => (1, 3),
            Part::Two => (4, 10),
        };
        let goal = Position {
            row: heat_loss_map.height() as isize - 1,
            col: heat_loss_map.width() as isize - 1,
        };
        let preloaded = heat_loss_map
            .position_iter()
            .map(|p| (p, heuristic(&heat_loss_map, goal, p)))
            .collect::<IndexMap<_, _>>();
        let result = heuristic_search(
            CrucibleStatus::default(),
            |c| c.p == goal && c.streak >= streak_min,
            |c| preloaded.get(&c.p).copied().unwrap(),
            |c, p| {
                let mut result = vec![];
                let path_back = p.path_back_from(c);
                for dir in [
                    c.incoming,
                    c.incoming.clockwise(),
                    c.incoming.counterclockwise(),
                ] {
                    if (dir != c.incoming || c.streak < streak_max)
                        && (dir == c.incoming || c.streak >= streak_min)
                    {
                        let neighbor = dir.next_position(c.p);
                        if path_back
                            .as_ref()
                            .map_or(true, |path| path.iter().all(|pc| pc.p != neighbor))
                        {
                            if let Some(loss) = heat_loss_map.value(neighbor) {
                                let streak = if dir == c.incoming { c.streak + 1 } else { 1 };
                                result.push((
                                    CrucibleStatus {
                                        p: neighbor,
                                        streak,
                                        incoming: dir,
                                    },
                                    loss.a() as u64,
                                ));
                            }
                        }
                    }
                }
                result
            },
        );
        let total_heat_loss = result.cost().unwrap();
        let path_back = result.path().unwrap();
        visualize(filename, path_back.iter().map(|c| c.p))?;
        println!("enqueued: {}", result.enqueued());
        println!("Part {part:?}: {total_heat_loss}");

        Ok(())
    })
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
struct CrucibleStatus {
    p: Position,
    streak: usize,
    incoming: ManhattanDir,
}

impl Default for CrucibleStatus {
    fn default() -> Self {
        Self {
            p: Default::default(),
            streak: 0,
            incoming: ManhattanDir::E,
        }
    }
}

fn heuristic(heat_loss_map: &GridDigitWorld, goal: Position, location: Position) -> u64 {
    let mut losses = BinaryHeap::new();
    for row in location.row..heat_loss_map.height() as isize {
        for col in location.col..heat_loss_map.width() as isize {
            let p = Position { row, col };
            if p != location {
                losses.push(Reverse(heat_loss_map.value(p).unwrap()));
            }
        }
    }

    let mut remaining_distance = location.manhattan_distance(goal);
    let mut estimate = 0;
    while remaining_distance > 0 {
        estimate += losses.pop().unwrap().0.a() as u64;
        remaining_distance -= 1;
    }
    estimate
}

fn visualize(filename: &str, mut path: impl Iterator<Item = Position>) -> anyhow::Result<()> {
    let mut chars = GridCharWorld::from_char_file(filename)?;
    let mut prev = path.next().unwrap();
    for next in path {
        let c = match prev.manhattan_dir_to(next).unwrap() {
            ManhattanDir::N => '^',
            ManhattanDir::E => '>',
            ManhattanDir::S => 'v',
            ManhattanDir::W => '<',
        };
        chars.modify(next, |v| *v = c);
        prev = next;
    }
    println!("{chars}");
    Ok(())
}
