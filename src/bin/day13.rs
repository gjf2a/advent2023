use std::cmp::min;

use advent_code_lib::{
    all_lines, chooser_main, GridCharWorld, Part, Position, RowMajorPositionIterator,
};
use enum_iterator::{all, Sequence};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let blocks = blocks_from(filename)?;
        let reflection_lines = lines_for(&blocks);
        assert_eq!(blocks.len(), reflection_lines.len());
        match part {
            Part::One => {
                println!("Part 1: {}", summary(&reflection_lines));
            }
            Part::Two => {
                let smudge_lines = (0..blocks.len())
                    .map(|i| smudge_for(&blocks[i], reflection_lines[i]))
                    .collect::<Vec<_>>();
                println!("Part 2: {}", summary(&smudge_lines));
            }
        }
        Ok(())
    })
}

fn summary(lines: &Vec<MirrorLine>) -> usize {
    lines.iter().map(|ml| ml.summary()).sum()
}

fn lines_for(blocks: &Vec<GridCharWorld>) -> Vec<MirrorLine> {
    blocks.iter().map(line_for).collect()
}

fn line_for(block: &GridCharWorld) -> MirrorLine {
    all::<Mirror>()
        .filter_map(|m| m.num_preceding(block, None))
        .next()
        .unwrap()
}

fn smudge_for(block: &GridCharWorld, mirror: MirrorLine) -> MirrorLine {
    for (smudge, smudged) in iter_smudge(block) {
        if let Some(line) = all::<Mirror>()
            .filter_map(|m| m.num_preceding(&smudged, Some(mirror)))
            .next()
        {
            if line.contains(line.dir.major_coord(smudge)) {
                return line;
            }
        }
    }
    assert!(false);
    mirror
}

fn iter_smudge(block: &GridCharWorld) -> impl Iterator<Item = (Position, GridCharWorld)> + '_ {
    RowMajorPositionIterator::new(block.width(), block.height()).map(|p| {
        let mut smudged = block.clone();
        smudged.modify(p, |v| *v = if *v == '#' { '.' } else { '#' });
        (p, smudged)
    })
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct MirrorLine {
    dir: Mirror,
    line: usize,
    start: usize,
    end: usize,
}

impl MirrorLine {
    fn contains(&self, coord: usize) -> bool {
        coord >= self.start && coord <= self.end
    }

    fn summary(&self) -> usize {
        self.line
            * match self.dir {
                Mirror::Column => 1,
                Mirror::Row => 100,
            }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Sequence)]
enum Mirror {
    Column,
    Row,
}

impl Mirror {
    fn num_preceding(
        &self,
        block: &GridCharWorld,
        prohibited: Option<MirrorLine>,
    ) -> Option<MirrorLine> {
        for major in 0..self.major_dim(block) {
            let m = self.mirror(block, major);
            if m != prohibited {
                if let Some(m) = m {
                    return Some(m);
                }
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
            Some(MirrorLine {
                dir: *self,
                line: test,
                start: substart,
                end: subend - substart,
            })
        } else {
            None
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Column => Self::Row,
            Self::Row => Self::Column,
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
    use advent_code_lib::GridCharWorld;

    use crate::{blocks_from, Mirror};

    fn num_columns_left(block: &GridCharWorld) -> Option<usize> {
        Mirror::Column.num_preceding(block, None).map(|m| m.line)
    }

    fn num_rows_above(block: &GridCharWorld) -> Option<usize> {
        Mirror::Row.num_preceding(block, None).map(|m| m.line)
    }

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
