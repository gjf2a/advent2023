use advent_code_lib::{all_lines, chooser_main, Part};
use bare_metal_modulo::{MNum, ModNumC};
use gapbuf::GapBuffer;

const PERIOD: usize = 256;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let line = all_lines(filename)?.next().unwrap();
        let value = match part {
            Part::One => initialization_hash_sum(line.as_str()),
            Part::Two => {
                let mut boxes = Boxes::new();
                for command in line.split(",") {
                    boxes.command(command);
                }
                boxes.calculation()
            }
        };
        println!("Part {part:?}: {value}");
        Ok(())
    })
}

#[derive(Debug)]
struct Boxes {
    boxes: [GapBuffer<(String, u64)>; PERIOD],
}

impl Boxes {
    fn new() -> Self {
        Self {
            boxes: [0; PERIOD].map(|_| GapBuffer::new()),
        }
    }

    fn command(&mut self, s: &str) {
        let code_at = s.find(|c| c == '-' || c == '=').unwrap();
        let key = &s[..code_at];
        let box_num = modular_hash(key).a() as usize;
        let within_box = (0..self.boxes[box_num].len()).find(|i| self.boxes[box_num][*i].0 == key);
        if &s[code_at..code_at + 1] == "-" {
            if let Some(i) = within_box {
                self.boxes[box_num].remove(i);
            }
        } else {
            let digit = s[(code_at + 1)..].parse::<u64>().unwrap();
            match within_box {
                None => {
                    self.boxes[box_num].push_back((key.to_owned(), digit));
                }
                Some(i) => {
                    self.boxes[box_num][i].1 = digit;
                }
            }
        }
    }

    fn calculation(&self) -> u64 {
        self.boxes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let box_index = i as u64 + 1;
                b.iter()
                    .enumerate()
                    .map(|(j, (_, focal_length))| (j as u64 + 1) * box_index * focal_length)
                    .sum::<u64>()
            })
            .sum()
    }
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
