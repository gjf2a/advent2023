use std::cmp::min;

use advent_code_lib::{chooser_main, all_lines, GridCharWorld};

// Part 1: 551 is too low
// 769 is also too low - I got there by starting with higher columns and rows and going down.

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let blocks = blocks_from(filename)?;
        let left_sum = blocks.iter().filter_map(|b| num_columns_left(b)).sum::<usize>();
        let above_sum = blocks.iter().filter_map(|b| num_rows_above(b)).sum::<usize>();
        println!("num blocks: {}", blocks.len());
        println!("left_sum: {left_sum}");
        println!("above_sum: {above_sum}");
        println!("Part 1: {}", above_sum * 100 + left_sum);
        Ok(())
    })
}

fn num_columns_left(block: &GridCharWorld) -> Option<usize> {
    for col in (0..block.width()).rev() {
        if mirror_col(block, col) {
            println!("{block}\n");
            return Some(col);
        }        
    }
    None
}

fn num_rows_above(block: &GridCharWorld) -> Option<usize> {
    for row in (0..block.height()).rev() {
        if mirror_row(block, row) {
            println!("{block}\n");
            return Some(row);
        }
    }
    None
}

fn mirror_col(block: &GridCharWorld, col: usize) -> bool {
    let subwidth = min(col, block.width() - col);
    let substart = col - subwidth;
    let subend = substart + subwidth * 2;
    for subcol in substart..col {
        let mirror_col = subend - subcol;
        for row in 0..block.height() {
            if block.get(subcol, row) != block.get(mirror_col, row) {
                return false;
            }
        }
    }
    subwidth > 1
}

fn mirror_row(block: &GridCharWorld, row: usize) -> bool {
    let subheight = min(row, block.height() - row);
    let substart = row - subheight;
    let subend = substart + subheight * 2;
    for subrow in substart..row {
        let mirror_row = subend - subrow;
        for col in 0..block.width() {
            if block.get(col, subrow) != block.get(col, mirror_row) {
                return false;
            }
        }
    }
    subheight > 1
}

fn blocks_from(filename: &str) -> anyhow::Result<Vec<GridCharWorld>> {
    let input = all_lines(filename)?.map(|line| format!("{line}\n")).collect::<String>();
    Ok(input.split("\n\n").map(|b| b.parse::<GridCharWorld>().unwrap()).collect())
}

#[cfg(test)]
mod tests {
    use crate::{blocks_from, mirror_col, num_columns_left};

    #[test]
    fn test_horizontal() {
        let blocks = blocks_from("ex/day13.txt").unwrap();
        assert!(mirror_col(&blocks[0], 5));
        assert_eq!(Some(5), num_columns_left(&blocks[0]));
        assert_eq!(None, num_columns_left(&blocks[1]));
    }
}