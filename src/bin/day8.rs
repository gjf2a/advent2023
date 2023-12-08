use advent_code_lib::{chooser_main, Part, all_lines};
use bare_metal_modulo::{ModNum, MNum};
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut lines = all_lines(filename)?;
        let instructions = instructions(lines.next().unwrap());
        let map = graph(lines.skip(1));
        println!("Part one: {}", navigate(&instructions, &map));
        Ok(())
    })
}

fn navigate(instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) -> usize {
    let mut step_count = 0;
    let mut i = ModNum::new(0, instructions.len());
    let mut location = "AAA".to_owned();
    while location.as_str() != "ZZZ" {
        let options = map.get(location.as_str()).unwrap();
        location = if instructions[i.a()] == 'L' {options.0.clone()} else {options.1.clone()};
        step_count += 1;
        i += 1;
    }

    step_count
}

fn instructions(line: String) -> Vec<char> {
    line.chars().collect()
}

fn graph(lines: impl Iterator<Item=String>) -> IndexMap<String,(String,String)> {
    let mut result = IndexMap::new();
    for line in lines {
        let line = line.replace('(', "").replace(')', "").replace(',', "");
        let parts: Vec<&str> = line.split_whitespace().collect();
        result.insert(parts[0].to_owned(), (parts[2].to_owned(), parts[3].to_owned()));
    }
    result
}