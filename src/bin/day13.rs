use std::cmp::min;

use advent_code_lib::{chooser_main, all_lines, GridCharWorld};

// Part 1: 551 is too low
// 21358 is still too low!!

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let blocks = blocks_from(filename)?;
        let horizontal_blocks = blocks.iter().filter_map(|b| num_columns_left(b)).collect::<Vec<_>>();
        let vertical_blocks = blocks.iter().filter_map(|b| num_rows_above(b)).collect::<Vec<_>>();
        let left_sum = horizontal_blocks.iter().sum::<usize>();
        let above_sum = vertical_blocks.iter().sum::<usize>();
        println!("num blocks: {}", blocks.len());
        println!("left_sum: {left_sum} ({} blocks)", horizontal_blocks.len());
        println!("above_sum: {above_sum} ({} blocks)", vertical_blocks.len());
        println!("Part 1: {}", above_sum * 100 + left_sum);
        Ok(())
    })
}

fn num_columns_left(block: &GridCharWorld) -> Option<usize> {
    for col in 0..block.width() {
        if mirror_col(block, col) {
            println!("{block}\ncol: {col}\n");
            return Some(col);
        }        
    }
    println!("{block}\nH failure\n");
    None
}

fn num_rows_above(block: &GridCharWorld) -> Option<usize> {
    for row in 0..block.height() {
        if mirror_row(block, row) {
            println!("{block}\nrow: {row}\n");
            return Some(row);
        }
    }
    println!("{block}\nV failure\n");
    None
}

fn mirror_col(block: &GridCharWorld, col: usize) -> bool {
    let subwidth = min(col, block.width() - col);
    if subwidth >= 2 {
        let substart = col - subwidth;
        let subend = (substart + subwidth) * 2 - 1;
        //println!("col: {col} sw: {subwidth} ss: {substart} se: {subend}");
        for subcol in substart..col {
            let mirror_col = subend - subcol;
            //println!("{subcol} {mirror_col}");
            for row in 0..block.height() {
                if block.get(subcol, row) != block.get(mirror_col, row) {
                    //println!("Fatal row: {row} {:?} {:?}", block.get(subcol, row), block.get(mirror_col, row));
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
    if subheight >= 2 {
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
    }
}