use std::{collections::VecDeque, str::FromStr};

use advent_code_lib::{all_lines, chooser_main, Part, Point};
use indexmap::IndexSet;

// 56341 is too high for Part 2

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let view = options.len() > 0;
        let compacted = compacted_bricks(filename, view)?;
        let supporters = supporters(&compacted);
        let necessary = (0..compacted.len())
            .filter(|i| supporters.iter().any(|s| s.len() == 1 && s[0] == *i))
            .collect::<IndexSet<_>>();
        match part {
            Part::One => {
                println!("Part {part:?}: {}", compacted.len() - necessary.len());
            }
            Part::Two => {
                let on_ground = compacted
                    .iter()
                    .enumerate()
                    .filter(|(_,brick)| brick.on_ground())
                    .map(|(i,_)| i)
                    .collect::<IndexSet<_>>();
                println!("on ground: {on_ground:?}");
                let total = necessary
                    .iter()
                    .map(|n| falling_after_disintegrating(&on_ground, *n, &supporters))
                    .sum::<usize>();
                println!("Part {part:?}: {total}");
            }
        }
        Ok(())
    })
}

fn falling_after_disintegrating(
    on_ground: &IndexSet<usize>,
    disintegrated: usize,
    supporters: &Vec<IndexSet<usize>>,
) -> usize {
    let supporting = supporting(&supporters);
    let mut supporters = supporters.clone();
    let mut disintegrating = VecDeque::new();
    disintegrating.push_back(disintegrated);
    while let Some(disintegrator) = disintegrating.pop_front() {
        for target in supporting[disintegrator].iter() {
            supporters[*target].remove(&disintegrator);
            if supporters[*target].len() == 0 {
                disintegrating.push_back(*target);
            }
        }
    }
    supporters
        .iter()
        .enumerate()
        .filter(|(i, support)| !on_ground.contains(i) && support.len() == 0)
        .count()
}

fn supporting(supporters: &Vec<IndexSet<usize>>) -> Vec<Vec<usize>> {
    let mut result = (0..supporters.len()).map(|_| vec![]).collect::<Vec<_>>();
    for (i, i_support) in supporters.iter().enumerate() {
        for supporter in i_support.iter() {
            result[*supporter].push(i);
        }
    }
    result
}

fn compacted_bricks(filename: &str, view: bool) -> anyhow::Result<Vec<Brick>> {
    let mut bricks = all_lines(filename)?
        .map(|line| line.parse::<Brick>().unwrap())
        .collect::<Vec<_>>();
    bricks.sort_by_key(|k| k.bottom());
    let result = compacted(&bricks);
    if view {
        for brick in result.iter() {
            println!("{brick:?}");
        }
    }
    Ok(result)
}

fn supporters(compacted: &Vec<Brick>) -> Vec<IndexSet<usize>> {
    let mut supporters = vec![];
    for (i, brick) in compacted.iter().enumerate() {
        supporters.push(IndexSet::new());
        for j in 0..i {
            if brick.overlaps(&compacted[j]) && compacted[j].top() + 1 == brick.bottom() {
                supporters[i].insert(j);
            }
        }
    }
    supporters
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
    fn on_ground(&self) -> bool {
        self.bottom() == 1
    }

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
