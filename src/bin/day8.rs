use advent_code_lib::{chooser_main, Part, all_lines};
use bare_metal_modulo::{ModNum, MNum};
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut lines = all_lines(filename)?;
        let instructions = instructions(lines.next().unwrap());
        let map = graph(lines.skip(1));
        match part {
            Part::One => println!("Part one: {}", navigate(&instructions, &map)),
            Part::Two => println!("Part two: {}", ghost_navigate(&instructions, &map)),
        }
        Ok(())
    })
}

fn navigate(instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) -> usize {
    let mut step_count = 0;
    let mut i = ModNum::new(0, instructions.len());
    let mut location = "AAA".to_owned();
    while location.as_str() != "ZZZ" {
        navigate_once(&mut i, &mut step_count, &mut location, instructions, map);
    }

    step_count
}

fn navigate_once(i: &mut ModNum<usize>, step_count: &mut usize, location: &mut String, instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) {
    let options = map.get(location.as_str()).unwrap();
    *location = if instructions[i.a()] == 'L' {options.0.clone()} else {options.1.clone()};
    *step_count += 1;
    *i += 1;
}

fn ghost_navigate(instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) -> usize {
    let mut step_count = 0;
    let mut i = ModNum::new(0, instructions.len());
    let mut locations = all_starts(map);
    while !all_end(&locations) {
        for location in locations.iter_mut() {
            navigate_once(&mut i, &mut step_count, location, instructions, map);
        }
    }
    step_count
}

fn all_starts(map: &IndexMap<String,(String,String)>) -> Vec<String> {
    map.keys().filter(|k| k.ends_with("A")).map(|k| k.to_string()).collect()
}

fn all_end(locations: &Vec<String>) -> bool {
    locations.iter().all(|loc| loc.ends_with("Z"))
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