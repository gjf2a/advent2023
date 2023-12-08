use advent_code_lib::{chooser_main, Part, all_lines};
use bare_metal_modulo::{ModNum, MNum};
use indexmap::IndexMap;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut lines = all_lines(filename)?;
        let instructions = instructions(lines.next().unwrap());
        let map = graph(lines.skip(1));
        match part {
            Part::One => println!("Part one: {}", navigate("AAA", &instructions, &map)),
            Part::Two => println!("Part two: {}", ghost_navigate(&instructions, &map)),
        }
        Ok(())
    })
}

fn navigate(start: &str, instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) -> u64 {
    let mut step_count = 0;
    let mut i = ModNum::new(0, instructions.len());
    let mut location = start.to_owned();
    while !location.as_str().ends_with("Z") {
        let options = map.get(location.as_str()).unwrap();
        location = if instructions[i.a()] == 'L' {options.0.clone()} else {options.1.clone()};
        i += 1;
        step_count += 1;
    }
    step_count
}

fn ghost_navigate(instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) -> u64 {
    let locations = all_starts(map);
    let distances = all_distances(&locations, instructions, map);
    distances.iter().product::<u64>() / big_gcd(&distances)
}

fn big_gcd(ns: &Vec<u64>) -> u64 {
    ns.iter().copied().reduce(gcd).unwrap()
}

fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn all_starts(map: &IndexMap<String,(String,String)>) -> Vec<String> {
    map.keys().filter(|k| k.ends_with("A")).map(|k| k.to_string()).collect()
}

fn all_distances(locations: &Vec<String>, instructions: &Vec<char>, map: &IndexMap<String,(String,String)>) -> Vec<u64> {
    locations.iter().map(|start| navigate(start.as_str(), instructions, map)).collect()
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