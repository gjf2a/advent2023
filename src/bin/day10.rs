use advent_code_lib::{chooser_main, Part};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {

        match part {
            Part::One => {
                let total = 0;
                println!("Part one: {}", total);
            }
            Part::Two => {
                let total = 0;
                println!("Part two: {}", total);
            }
        }
        Ok(())
    })
}

