use advent_code_lib::{chooser_main, Point};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {

        Ok(())
    })
}

struct Brick {
    start: Point<isize,3>,
    end: Point<isize,3>,
}