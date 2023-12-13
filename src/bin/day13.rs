use std::cmp::min;

use advent_code_lib::{all_lines, chooser_main, GridCharWorld, Part, RowMajorPositionIterator, Position};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let blocks = blocks_from(filename)?;
        match part {
            Part::One => {
                let left_sum = block_sum(&blocks, num_columns_left);
                let above_sum = block_sum(&blocks, num_rows_above);
                println!("left_sum: {left_sum}");
                println!("above_sum: {above_sum}");
                println!("Part 1: {}", above_sum * 100 + left_sum);
            }
            Part::Two => {
                /*let left_sum = block_sum(&blocks, find_smudge_columns);
                let above_sum = block_sum(&blocks, find_smudge_rows);
                println!("left_sum: {left_sum}");
                println!("above_sum: {above_sum}");
                println!("Part 2: {}", above_sum * 100 + left_sum);*/
            }
        }
        Ok(())
    })
}

fn block_sum<F: Fn(&GridCharWorld) -> Option<usize>>(
    blocks: &Vec<GridCharWorld>,
    analyzer: F,
) -> usize {
    //blocks.iter().filter_map(|b| analyzer(b)).sum()
    let winners = blocks
        .iter()
        .filter_map(|b| analyzer(b))
        .collect::<Vec<_>>();
    println!("winners: {}", winners.len());
    winners.iter().sum()
}

fn iter_smudge(block: &GridCharWorld) -> impl Iterator<Item = (Position, GridCharWorld)> + '_ {
    RowMajorPositionIterator::new(block.width(), block.height()).map(|p| {
        let mut smudged = block.clone();
        smudged.modify(p, |v| *v = if *v == '#' { '.' } else { '#' });
        (p, smudged)
    })
}
/* 
fn find_smudge_columns(block: &GridCharWorld) -> Option<usize> {
    find_smudge(block, num_columns_left)
}

fn find_smudge_rows(block: &GridCharWorld) -> Option<usize> {
    find_smudge(block, num_rows_above)
}
*/
fn num_columns_left(block: &GridCharWorld) -> Option<usize> {
    Mirror::Column.num_preceding(block).map(|m| m.line)
}

fn num_rows_above(block: &GridCharWorld) -> Option<usize> {
    Mirror::Row.num_preceding(block).map(|m| m.line)
}

struct MirrorLine {
    line: usize,
    start: usize,
    end: usize,
}

impl MirrorLine {
    fn contains(&self, coord: usize) -> bool {
        coord >= self.start && coord <= self.end
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Mirror {
    Column,
    Row
}

impl Mirror {
    fn find_smudge<F: Fn(&GridCharWorld) -> Option<usize>>(
        &self,
        block: &GridCharWorld,
        analyzer: F,
    ) -> Option<usize> {
        None
    }
        
    fn num_preceding(&self, block: &GridCharWorld) -> Option<MirrorLine> {
        for major in 0..self.major_dim(block) {
            if let Some(m) = self.mirror(block, major) {
                return Some(m);
            }
        }
        None
    }

    fn mirror(&self, block: &GridCharWorld, test: usize) -> Option<MirrorLine> {
        let sub = min(test, self.major_dim(block) - test);
        if sub >= 1 {
            let substart = test - sub;
            let subend = (substart + sub) * 2 - 1;
            for subtest in substart..test {
                let mirror = subend - subtest;
                for entry in 0..self.minor_dim(block) {
                    if self.get(block, subtest, entry) != self.get(block, mirror, entry) {
                        return None;
                    }
                }
            }
            Some(MirrorLine { line: test, start: substart, end: subend })
        } else {
            None
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Column => Self::Row,
            Self::Row => Self::Column
        }
    }

    fn major_dim(&self, block: &GridCharWorld) -> usize {
        match self {
            Self::Column => block.width(),
            Self::Row => block.height(),
        }
    }

    fn minor_dim(&self, block: &GridCharWorld) -> usize {
        self.opposite().major_dim(block)
    }

    fn major_coord(&self, p: Position) -> usize {
        match self {
            Self::Column => p.col as usize,
            Self::Row => p.row as usize,
        }
    }

    fn minor_coord(&self, p: Position) -> usize {
        self.opposite().major_coord(p)
    }

    fn get(&self, block: &GridCharWorld, major: usize, minor: usize) -> Option<char> {
        match self {
            Self::Column => block.get(major, minor),
            Self::Row => block.get(minor, major),
        }
    }
}

fn blocks_from(filename: &str) -> anyhow::Result<Vec<GridCharWorld>> {
    let input = all_lines(filename)?
        .map(|line| format!("{line}\n"))
        .collect::<String>();
    Ok(input
        .split("\n\n")
        .map(|b| b.parse::<GridCharWorld>().unwrap())
        .collect())
}

#[cfg(test)]
mod tests {
    use crate::{blocks_from, num_columns_left, num_rows_above, Mirror};

    #[test]
    fn test_horizontal() {
        let blocks = blocks_from("ex/day13.txt").unwrap();
        assert!(Mirror::Column.mirror(&blocks[0], 5).is_some());
        assert_eq!(Some(5), num_columns_left(&blocks[0]));
        assert_eq!(None, num_columns_left(&blocks[1]));
    }

    #[test]
    fn test_second_horizontal() {
        let blocks = blocks_from("ex/day13ferrer.txt").unwrap();
        assert!(Mirror::Column.mirror(&blocks[0], 6).is_some());
        assert_eq!(Some(6), num_columns_left(&blocks[0]));
        assert_eq!(None, num_columns_left(&blocks[1]));
    }

    #[test]
    fn test_input_horizontal() {
        let blocks = blocks_from("ex/day13_input_instances.txt").unwrap();
        assert!(Mirror::Column.mirror(&blocks[0], 4).is_some());
        assert_eq!(Some(4), num_columns_left(&blocks[0]));

        assert!(Mirror::Row.mirror(&blocks[1], 14).is_some());
        assert_eq!(Some(14), num_rows_above(&blocks[1]));
    }
}
