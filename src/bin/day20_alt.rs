use std::collections::HashMap;

use advent_code_lib::{all_lines, chooser_main, AdjacencySets};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, _| {
        println!("{filename} {part:?}");
        let mut types = HashMap::new();
        let mut forward = AdjacencySets::new();
        let mut backward = AdjacencySets::new();
        let mut counts = HashMap::new();
        for line in all_lines(filename)? {
            let parts = line.split(" -> ").collect::<Vec<_>>();
            if parts[0] == "broadcaster" {
                for target in parts[1].split(",") {
                    counts.insert(target.to_owned(), 1);
                }    
            } else {
                let name = &parts[0][1..];
                let mod_char = parts[0].chars().next().unwrap();
                let mod_type = if mod_char == '&' {Module::Nand} else {Module::FlipFlop};
                types.insert(name.to_owned(), mod_type);
                for target in parts[1].split(",") {
                    forward.connect(name, target);
                    backward.connect(target, name);
                }
            }
        }

        while !counts.contains_key("rx") {
            for node in backward.keys().filter(|k| !counts.contains_key(*k)) {
                match types.get(node).unwrap() {
                    Module::FlipFlop => todo!(),
                    Module::Nand => todo!(),
                }
            }
        }
        Ok(())
    })
}

enum Module {
    FlipFlop,
    Nand
}