use advent_code_lib::{chooser_main, all_lines};
use bare_metal_modulo::{ModNumC, MNum};

const PERIOD: usize = 256;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let line = all_lines(filename)?.next().unwrap();
        println!("Part {part:?}: {}", initialization_hash_sum(line.as_str()));
        Ok(())
    })
}

fn initialization_hash_sum(s: &str) -> u64 {
    s.split(",").map(|sub| modular_hash(sub).a() as u64).sum()
}

fn modular_hash(s: &str) -> ModNumC<u16, PERIOD> {
    let mut current = ModNumC::new(0);
    for byte in s.as_bytes().iter() {
        current += *byte as u16;
        current *= 17;
    }
    current
}

#[cfg(test)]
mod tests {
    use advent_code_lib::all_lines;
    use bare_metal_modulo::MNum;

    use crate::modular_hash;

    #[test]
    fn test_hash_1() {
        assert_eq!(52_u16, modular_hash("HASH").a());
    }

    #[test]
    fn test_hash_2() {
        let line = all_lines("ex/day15.txt").unwrap().next().unwrap();
        let expected = vec![30, 253, 97, 47, 14, 180, 9, 197, 48, 214, 231];
        for (s, h) in line.split(",").zip(expected.iter()) {
            assert_eq!(*h, modular_hash(s).a());
        }
    }
}