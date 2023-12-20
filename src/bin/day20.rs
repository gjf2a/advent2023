use std::{collections::VecDeque, fmt::Display};

use advent_code_lib::{chooser_main, all_lines, Part};
use enum_iterator::{Sequence, all};
use hash_histogram::HashHistogram;
use indexmap::{IndexMap, IndexSet};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        let mut circuit = Circuit::from(filename)?;
        match part {
            Part::One => {
                for _ in 0..1000 {
                    circuit.push_button();
                }
                println!("{circuit:?}");
                println!("Part {part:?}: {}", circuit.score());
            }
            Part::Two => {
                circuit.push_button();
                println!("After one push:\n{}", circuit.stats);
                for _ in 1..10000 {
                    circuit.push_button();
                }
                println!("After 10000 pushes:\n{}", circuit.stats);
            }
        }
        Ok(())
    })
}

#[derive(Debug)]
struct Circuit {
    connections: IndexMap<String,(Module,Vec<String>)>,
    pulse_count: HashHistogram<Pulse>,
    stats: LevelStats
}

impl Circuit {
    fn score(&self) -> u64 {
        all::<Pulse>().map(|p| self.pulse_count.count(&p) as u64).product()
    }

    fn push_button(&mut self) {
        let mut pending = VecDeque::new();
        self.pulse_count.bump(&Pulse::Low);
        self.stats.add("broadcaster", 0);
        for starter in self.connections.get("broadcaster").unwrap().1.iter() {
            pending.push_back((starter.clone(), "broadcaster".to_string(), Pulse::Low, 1));
        }
        while let Some((module_name, source, input, level)) = pending.pop_front() {
            self.stats.add(module_name.as_str(), level);
            self.pulse_count.bump(&input);
            let (module, outputs) = self.connections.get_mut(module_name.as_str()).unwrap();
            if let Some(output_pulse) = module.apply_input(source.as_str(), input) {
                for output in outputs.iter() {
                    pending.push_back((output.clone(), module_name.clone(), output_pulse, level + 1));
                }
            }
        }
    }

    fn from(filename: &str) -> anyhow::Result<Self> {
        let mut connections = IndexMap::new();
        let mut incoming_names = IndexMap::new();
        for line in all_lines(filename)? {
            let mut top = line.split(" -> ");
            let (module, module_name) = Module::module_name(top.next().unwrap());
            let destinations = top.next().unwrap();
            let edges: Vec<String> = destinations.split(",").map(|s| s.trim().to_owned()).collect();
            for edge in edges.iter() {
                if !incoming_names.contains_key(edge.as_str()) {
                    incoming_names.insert(edge.clone(), vec![]);
                }
                incoming_names.get_mut(edge.as_str()).unwrap().push(module_name.clone());
            }
            connections.insert(module_name, (module, edges));
        }
        let outputs: Vec<String> = incoming_names.keys().filter(|s| !connections.contains_key(s.as_str())).map(|s| s.to_string()).collect();
        for output in outputs.iter() {
            connections.insert(output.clone(), (Module::Output, vec![]));
        }
        for (name, (m, _)) in connections.iter_mut() {
            match m {
                Module::Conjunction(incoming) => {
                    for name in incoming_names.get(name.as_str()).unwrap() {
                        incoming.insert(name.clone(), Pulse::Low);
                    }
                }
                _ => {}
            }
        }
        Ok(Self {connections, pulse_count: HashHistogram::default(), stats: LevelStats::default()})
    }
}

#[derive(Default, Debug)]
struct LevelStats {
    at_level: Vec<IndexSet<String>>,
    levels_for: IndexMap<String,IndexSet<usize>>,
}

impl LevelStats {
    fn add(&mut self, module_name: &str, level: usize) {
        if self.at_level.len() == 0 || level > self.highest_level() {
            self.at_level.push(IndexSet::new());
        }
        self.at_level[level].insert(module_name.to_owned());
        if !self.levels_for.contains_key(module_name) {
            self.levels_for.insert(module_name.to_owned(), IndexSet::new());
        }
        self.levels_for.get_mut(module_name).unwrap().insert(level);
    }

    fn highest_level(&self) -> usize {
        self.at_level.len() - 1
    }
}

impl Display for LevelStats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for level in 0..self.at_level.len() {
            writeln!(f, "{level}: {:?}", self.at_level[level])?;
        }
        for (key, levels) in self.levels_for.iter() {
            writeln!(f, "{key}: {levels:?}")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
enum Module {
    Broadcaster,
    Conjunction(IndexMap<String,Pulse>),
    FlipFlop(FlipFlopState),
    Output,
}

impl Module {
    fn apply_input(&mut self, source: &str, input: Pulse) -> Option<Pulse> {
        match self {
            Self::Broadcaster => Some(Pulse::Low),
            Self::Output => None,
            Self::Conjunction(incoming) => {
                *(incoming.get_mut(source).unwrap()) = input;
                if incoming.values().all(|p| *p == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Self::FlipFlop(state) => {
                match input {
                    Pulse::Low => {
                        state.flip();
                        match state {
                            FlipFlopState::On => Some(Pulse::High),
                            FlipFlopState::Off => Some(Pulse::Low),
                        }
                    }
                    Pulse::High => None
                }
            }  
        }
    }

    fn module_name(name: &str) -> (Self, String) {
        match name.chars().next().unwrap() {
            'b' => (Self::Broadcaster, name.to_owned()),
            '&' => (Self::Conjunction(IndexMap::new()), name[1..].to_owned()),
            '%' => (Self::FlipFlop(FlipFlopState::Off), name[1..].to_owned()),
            _ => panic!("Unrecognized start character"),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, Sequence)]
enum Pulse {
    #[default]
    Low,
    High
}

#[derive(Debug)]
enum FlipFlopState {
    Off, On
}

impl FlipFlopState {
    fn flip(&mut self) {
        *self = match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        };
    }
}