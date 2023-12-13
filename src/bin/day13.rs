use std::cmp::min;

use advent_code_lib::{all_lines, chooser_main, GridCharWorld, Part, RowMajorPositionIterator};

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
                let left_sum = block_sum(&blocks, find_smudge_columns);
                let above_sum = block_sum(&blocks, find_smudge_rows);
                println!("left_sum: {left_sum}");
                println!("above_sum: {above_sum}");
                println!("Part 2: {}", above_sum * 100 + left_sum);
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

fn iter_smudge(block: &GridCharWorld) -> impl Iterator<Item = GridCharWorld> + '_ {
    RowMajorPositionIterator::new(block.width(), block.height()).map(|p| {
        let mut smudged = block.clone();
        smudged.modify(p, |v| *v = if *v == '#' { '.' } else { '#' });
        smudged
    })
}

fn find_smudge<F: Fn(&GridCharWorld) -> Option<usize>>(
    block: &GridCharWorld,
    analyzer: F,
) -> Option<usize> {
    iter_smudge(block)
        .filter_map(|smudged| analyzer(&smudged).map(|w| (smudged, w)))
        .inspect(|(smudged, w)| {
            println!("smudged:\n{smudged}");
            println!("line: {}", w);
        })
        .map(|(_, w)| w)
        .next()
}

fn find_smudge_columns(block: &GridCharWorld) -> Option<usize> {
    find_smudge(block, num_columns_left)
}

fn find_smudge_rows(block: &GridCharWorld) -> Option<usize> {
    find_smudge(block, num_rows_above)
}

fn num_columns_left(block: &GridCharWorld) -> Option<usize> {
    Mirror::Column.num_preceding(block)/*
    for col in 0..block.width() {
        if mirror_col(block, col) {
            return Some(col);
        }
    }
    None*/
}

fn num_rows_above(block: &GridCharWorld) -> Option<usize> {
    Mirror::Row.num_preceding(block)/*
    for row in 0..block.height() {
        if mirror_row(block, row) {
            return Some(row);
        }
    }
    None
    */
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Mirror {
    Column,
    Row
}

impl Mirror {
    fn num_preceding(&self, block: &GridCharWorld) -> Option<usize> {
        for major in 0..self.dim(block) {
            if self.mirror(block, major) {
                return Some(major);
            }
        }
        None
    }

    fn mirror(&self, block: &GridCharWorld, test: usize) -> bool {
        let sub = min(test, self.dim(block) - test);
        if sub >= 1 {
            let substart = test - sub;
            let subend = (substart + sub) * 2 - 1;
            for subtest in substart..test {
                let mirror = subend - subtest;
                for entry in 0..self.alt_dim(block) {
                    if self.get(block, subtest, entry) != self.get(block, mirror, entry) {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    fn dim(&self, block: &GridCharWorld) -> usize {
        match self {
            Self::Column => block.width(),
            Self::Row => block.height(),
        }
    }

    fn alt_dim(&self, block: &GridCharWorld) -> usize {
        match self {
            Self::Column => block.height(),
            Self::Row => block.width(),
        }
    }

    fn get(&self, block: &GridCharWorld, major: usize, minor: usize) -> Option<char> {
        match self {
            Self::Column => block.get(major, minor),
            Self::Row => block.get(minor, major),
        }
    }
}

fn mirror_col(block: &GridCharWorld, col: usize) -> bool {
    let subwidth = min(col, block.width() - col);
    if subwidth >= 1 {
        let substart = col - subwidth;
        let subend = (substart + subwidth) * 2 - 1;
        for subcol in substart..col {
            let mirror_col = subend - subcol;
            for row in 0..block.height() {
                if block.get(subcol, row) != block.get(mirror_col, row) {
                    return false;
                }
            }
        }
        true
    } else {
        false
    }
}

fn mirror_row(block: &GridCharWorld, row: usize) -> bool {
    let subheight = min(row, block.height() - row);
    if subheight >= 1 {
        let substart = row - subheight;
        let subend = (substart + subheight) * 2 - 1;
        for subrow in substart..row {
            let mirror_row = subend - subrow;
            for col in 0..block.width() {
                if block.get(col, subrow) != block.get(col, mirror_row) {
                    return false;
                }
            }
        }
        true
    } else {
        false
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
    use crate::{blocks_from, mirror_col, mirror_row, num_columns_left, num_rows_above};

    #[test]
    fn test_horizontal() {
        let blocks = blocks_from("ex/day13.txt").unwrap();
        assert!(mirror_col(&blocks[0], 5));
        assert_eq!(Some(5), num_columns_left(&blocks[0]));
        assert_eq!(None, num_columns_left(&blocks[1]));
    }

    #[test]
    fn test_second_horizontal() {
        let blocks = blocks_from("ex/day13ferrer.txt").unwrap();
        assert!(mirror_col(&blocks[0], 6));
        assert_eq!(Some(6), num_columns_left(&blocks[0]));
        assert_eq!(None, num_columns_left(&blocks[1]));
    }

    #[test]
    fn test_input_horizontal() {
        let blocks = blocks_from("ex/day13_input_instances.txt").unwrap();
        assert!(mirror_col(&blocks[0], 4));
        assert_eq!(Some(4), num_columns_left(&blocks[0]));

        assert!(mirror_row(&blocks[1], 14));
        assert_eq!(Some(14), num_rows_above(&blocks[1]));
    }
}
