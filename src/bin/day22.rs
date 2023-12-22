use std::str::FromStr;

use advent_code_lib::{all_lines, chooser_main, Point};
use indexmap::IndexSet;

// 881 is too high for Part 1

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let view = options.len() > 0;
        let mut bricks = all_lines(filename)?
            .map(|line| line.parse::<Brick>().unwrap())
            .collect::<Vec<_>>();
        bricks.sort_by_key(|k| k.bottom());
        if view {
            for brick in bricks.iter() {
                println!("{brick:?}");
            }
            println!("longest: {}", bricks.iter().map(|b| b.len()).max().unwrap());
        }
        let compacted = compacted(&bricks);
        if view {
            for brick in compacted.iter() {
                println!("{brick:?}");
            }
        }
        let mut supporters = vec![];
        for (i, brick) in compacted.iter().enumerate() {
            supporters.push(vec![]);
            for j in (0..i).skip_while(|j| compacted[*j].top() + 1 < brick.bottom()) {
                if brick.overlaps(&compacted[j]) {
                    supporters[i].push(j);
                }
            }
        }
        if view {
            println!("{supporters:?}");
        }
        let necessary = (0..compacted.len())
            .filter(|i| supporters.iter().any(|s| s.len() == 1 && s[0] == *i))
            .collect::<IndexSet<_>>();
        let disintegrate = compacted.len() - necessary.len();
        println!("Part {part:?}: {disintegrate}");
        Ok(())
    })
}

fn compacted(bricks: &Vec<Brick>) -> Vec<Brick> {
    let floor = &bricks[0];
    let mut result = vec![floor.drop_to(1)];
    for brick in bricks.iter().skip(1) {
        let target_z = 1 + result
            .iter()
            .filter(|below| below.overlaps(brick))
            .map(|below| below.top())
            .max()
            .unwrap_or(0);
        result.push(brick.drop_to(target_z));
    }
    result
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Brick {
    cubes: Vec<Point<isize, 3>>,
}

impl Brick {
    fn bottom(&self) -> isize {
        self.zs().min().unwrap()
    }

    fn overlaps(&self, other: &Brick) -> bool {
        self.cubes.iter().any(|cube| {
            other
                .cubes
                .iter()
                .any(|other| (0..2).all(|i| cube[i] == other[i]))
        })
    }

    fn drop_to(&self, target_z: isize) -> Brick {
        let distance = self.bottom() - target_z;
        Self {
            cubes: self
                .cubes
                .iter()
                .map(|cube| {
                    let mut drop = *cube;
                    drop[2] -= distance;
                    drop
                })
                .collect(),
        }
    }

    fn len(&self) -> usize {
        self.cubes.len()
    }

    fn top(&self) -> isize {
        self.zs().max().unwrap()
    }

    fn zs(&self) -> impl Iterator<Item = isize> + '_ {
        self.cubes.iter().map(|cube| cube[2])
    }
}

impl FromStr for Brick {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut points = s.split("~");
        let start = points.next().unwrap().parse::<Point<isize, 3>>()?;
        let end = points.next().unwrap().parse::<Point<isize, 3>>()?;
        let axis = (0..=2).find(|i| start[*i] != end[*i]).unwrap_or(0);
        let cubes = (start[axis]..=end[axis])
            .map(|n| {
                let mut cube = start;
                cube[axis] = n;
                cube
            })
            .collect();
        Ok(Self { cubes })
    }
}
