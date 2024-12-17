use std::{collections::VecDeque, fmt::Display};

use advent_code_lib::{all_lines, chooser_main, Part};
use enum_iterator::{all, Sequence};
use hash_histogram::HashHistogram;
use indexmap::{IndexMap, IndexSet};

/*
Patterns:
* (zeros to first one, length)
* 0: broadcast
* all 0 con: dh, db, sg, lm
* all 0 ff: tg, qj, jf, hn, tb, vt, lr, dx, cl, fr, gz, pz, xh, gr
* all 1: qq, bx, bc, gj (level 2 con), jm (level 4 con)
* (0, 1): pl, xr, mn, xc (level 1 ff)
* (1, 2): pm, rh, sc, hv (level 2 ff)
* (3, 4): nq, lp, xb, ks
* (7, 8): nd, zl, dg, lz
* (15, 16): cv, tr, zh, ll
* (31, 32): sz, vq, zx, st
* (63, 64): bt, ld, ms, mm
* Inferences:
* (127, 128): gr, cl, xh, gz
* (255, 256): hn, jf, vt, nf
* (511, 512): gv, qc, lr, tg
* (1023, 1024): xf, sf, fr, dx
* (2047, 2048): qj, pz, tb, hk

* Conjunction flip-flop outputs:
* qq: gr, cv, vq, lp, pl, bt
* bx: ll, xc
* gj: ks, ld, xr
* bc: mn, zl, xb, mm, hv, gz

* Need to solve for when qq, bx, gj, and bc simultaneously output zero.
* 1. Figure out which of them reaches zero first.
* 2. Adjust offsets for its outputs.
* 3. See which of the four gets solved next.
* 4. Adjust offsets for its outputs.
* 5. Keep going until they all reach zero at the same time.

qq outputs zero at 3851.
* lp, pl, and bt are all affected, but gr, cv, and vq are not.
* They, in turn, affect other flip-flops, which cause qq to output one again by the end of 3851.

* Conjunction inputs:
* qq: pl, rh, lz, hn, gv, xf, qj
* bx: xc, pm, nq, dg, zx, ms, xh, vt, lr, fr, tb
* gj: xr, sc, nd, tr, sz, cl, jf, qc, sf, pz
* bc: mn, zh, st, nf, tg, dx, hk
*
* dh: bc
* db: bx
* sg: gj
* lm: qq
*
* jm: sg, lm, dh, db


 */

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let mut circuit = Circuit::from(filename)?;
        match part {
            Part::One => {
                let cycles = if options.len() == 0 {1000} else {options[0].parse::<usize>().unwrap()};
                for _ in 0..cycles {
                    circuit.push_button();
                }
                println!("{:?}", circuit.current_pulses());
                println!("Part {part:?}: {}", circuit.score());
            }
            Part::Two => {
                if options.contains(&"-expr".to_string()) {
                    let periods = [
                        ["pl", "xr", "mn", "xc"], 
                        ["pm", "rh", "sc", "hv"],
                        ["nq", "lp", "xb", "ks"],
                        ["nd", "zl", "dg", "lz"],
                        ["cv", "tr", "zh", "ll"],
                        ["sz", "vq", "zx", "st"],
                        ["bt", "ld", "ms", "mm"],
                        ["gr", "cl", "xh", "gz"],
                        ["hn", "jf", "vt", "nf"],
                        ["gv", "qc", "lr", "tg"],
                        ["xf", "sf", "fr", "dx"],
                        ["qj", "pz", "tb", "hk"],
                    ];
                    let mut flip_flops = IndexMap::new();
                    for (i, period) in periods.iter().enumerate() {
                        for name in period.iter() {
                            flip_flops.insert(name.to_string(), FlipFlopTracker::new(i));
                        }
                    }

                    let conjunction_inputs = [
                        ("qq", vec!["pl", "rh", "lz", "hn", "gv", "xf", "qj"]),
                        ("bx", vec!["xc", "pm", "nq", "dg", "zx", "ms", "xh", "vt", "lr", "fr", "tb"]),
                        ("gj", vec!["xr", "sc", "nd", "tr", "sz", "cl", "jf", "qc", "sf", "pz"]),
                        ("bc", vec!["mn", "zh", "st", "nf", "tg", "dx", "hk"])];
                    let mut conjunctions = IndexMap::new();
                    for (name, flips) in conjunction_inputs.iter() {
                        conjunctions.insert(name.to_string(), ConjunctionTracker::new(&flips, &flip_flops));
                    }

                    for (name, conjunction) in conjunctions.iter() {
                        println!("Checking earliest for {name}...");
                        let earliest = conjunction.earliest_all_sources_on(&flip_flops);
                        println!("{earliest}");
                    }

                } else {
                    let pushes = options[0].parse::<usize>().unwrap();
                    illustrate(&mut circuit, pushes);
                }
            }
        }
        Ok(())
    })
}

struct FlipFlopTracker {
    start: usize,
    period: usize,
    zeros: Vec<usize>
}

impl FlipFlopTracker {
    fn new(array_index: usize) -> Self {
        let period = 2_usize.pow(array_index as u32);
        let start = period - 1;
        let advances = vec![];
        Self {start, period, zeros: advances}
    }

    fn add_zero_at(&mut self, zero: usize) {
        self.zeros.push(zero);
    }

    fn is_on(&self, time_step: usize) -> bool {
        if time_step >= self.start {
            let period_index = (time_step - self.start) / self.period;
            period_index % 2 == 0
        } else {
            false
        }
    }

    fn next_on_step(&self, current_step: usize) -> usize {
        if self.is_on(current_step) {
            current_step
        } else if current_step < self.start {
            self.start
        } else {
            let period_offset = (current_step - self.start) % self.period;
            current_step + self.period - period_offset
        }
    }
}

struct ConjunctionTracker {
    sources: Vec<String>
}

impl ConjunctionTracker {
    fn new(sources: &[&str], flip_flops: &IndexMap<String, FlipFlopTracker>) -> Self {
        let mut sources = sources.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        sources.sort_by_key(|k| flip_flops.get(k.as_str()).unwrap().period);
        sources.reverse();
        Self {sources}
    }

    fn earliest_all_sources_on(&self, flip_flops: &IndexMap<String, FlipFlopTracker>) -> usize {
        let mut earliest = 0;
        while !self.sources.iter().all(|s| flip_flops.get(s.as_str()).unwrap().is_on(earliest)) {
            for s in self.sources.iter() {
                earliest = flip_flops.get(s.as_str()).unwrap().next_on_step(earliest);
                println!("{earliest}");
            }
        }
        earliest
    }
}

#[derive(Debug)]
struct Circuit {
    connections: IndexMap<String, (Module, Vec<String>)>,
    pulse_count: HashHistogram<Pulse>,
    stats: LevelStats,
    num_button_pushes: u128,
    most_recent_incoming_pulse: IndexMap<String, (Pulse, u128)>,
    most_recent_outgoing_pulse: IndexMap<String, (Pulse, u128)>,
    pulses_at_end: IndexMap<String, Vec<Pulse>>,
}

impl Circuit {
    fn score(&self) -> u64 {
        all::<Pulse>()
            .map(|p| self.pulse_count.count(&p) as u64)
            .product()
    }

    fn current_pulses(&self) -> IndexMap<String,Pulse> {
        self.pulses_at_end.iter().map(|(k, v)| (k.clone(), v.last().copied().unwrap())).collect()
    }

    fn push_button(&mut self) {
        self.num_button_pushes += 1;
        let mut pending = VecDeque::new();
        self.pulse_count.bump(&Pulse::Low);
        self.stats.add("broadcaster", 0, None);
        for starter in self.connections.get("broadcaster").unwrap().1.iter() {
            pending.push_back((starter.clone(), "broadcaster".to_string(), Pulse::Low, 1));
        }
        while let Some((module_name, source, input, level)) = pending.pop_front() {
            self.stats
                .add(module_name.as_str(), level, Some(source.clone()));
            self.most_recent_incoming_pulse
                .insert(module_name.clone(), (input, self.num_button_pushes));
            self.pulse_count.bump(&input);
            let (module, outputs) = self.connections.get_mut(module_name.as_str()).unwrap();
            if let Some(output_pulse) = module.apply_input(source.as_str(), input) {
                self.most_recent_outgoing_pulse
                    .insert(module_name.clone(), (output_pulse, self.num_button_pushes));
                if module_name == "rx" && output_pulse == Pulse::Low {
                    println!("rx low at level {level}");
                }
                for output in outputs.iter() {
                    pending.push_back((
                        output.clone(),
                        module_name.clone(),
                        output_pulse,
                        level + 1,
                    ));
                }
            }
        }
        for (name, outcomes) in self.pulses_at_end.iter_mut() {
            outcomes.push(
                self.most_recent_outgoing_pulse
                    .get(name.as_str())
                    .map(|(p, _)| *p)
                    .unwrap_or(Pulse::Low),
            );
        }
    }

    fn print_outcomes(&self) {
        for (name, outcomes) in self.pulses_at_end.iter() {
            println!(
                "{name} {}",
                outcomes.iter().map(|p| p.digit()).collect::<String>()
            );
        }
    }

    fn from(filename: &str) -> anyhow::Result<Self> {
        let mut connections = IndexMap::new();
        let mut pulses_at_end = IndexMap::new();
        let mut incoming_names = IndexMap::new();
        for line in all_lines(filename)? {
            let mut top = line.split(" -> ");
            let (module, module_name) = Module::module_name(top.next().unwrap());
            let destinations = top.next().unwrap();
            let edges: Vec<String> = destinations
                .split(",")
                .map(|s| s.trim().to_owned())
                .collect();
            for edge in edges.iter() {
                if !incoming_names.contains_key(edge.as_str()) {
                    incoming_names.insert(edge.clone(), vec![]);
                }
                incoming_names
                    .get_mut(edge.as_str())
                    .unwrap()
                    .push(module_name.clone());
            }
            pulses_at_end.insert(module_name.clone(), vec![]);
            connections.insert(module_name, (module, edges));
        }
        let outputs: Vec<String> = incoming_names
            .keys()
            .filter(|s| !connections.contains_key(s.as_str()))
            .map(|s| s.to_string())
            .collect();
        for output in outputs.iter() {
            connections.insert(output.clone(), (Module::Output, vec![]));
            pulses_at_end.insert(output.clone(), vec![]);
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
        Ok(Self {
            connections,
            pulse_count: HashHistogram::default(),
            stats: LevelStats::default(),
            num_button_pushes: 0,
            most_recent_incoming_pulse: IndexMap::new(),
            most_recent_outgoing_pulse: IndexMap::new(),
            pulses_at_end,
        })
    }
}

#[derive(Default, Debug)]
struct LevelStats {
    at_level: Vec<IndexSet<String>>,
    levels_for: IndexMap<String, IndexSet<usize>>,
    ancestors: IndexMap<(String, usize), IndexSet<(String, usize)>>,
}

impl LevelStats {
    fn add(&mut self, module_name: &str, level: usize, parent: Option<String>) {
        if self.at_level.len() == 0 || level > self.highest_level() {
            self.at_level.push(IndexSet::new());
        }
        if let Some(parent) = parent {
            let key = (module_name.to_owned(), level);
            if !self.ancestors.contains_key(&key) {
                self.ancestors.insert(key.clone(), IndexSet::new());
            }
            self.ancestors
                .get_mut(&key)
                .unwrap()
                .insert((parent.clone(), level - 1));
        }
        self.at_level[level].insert(module_name.to_owned());
        if !self.levels_for.contains_key(module_name) {
            self.levels_for
                .insert(module_name.to_owned(), IndexSet::new());
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
    Conjunction(IndexMap<String, Pulse>),
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
                    if incoming.len() >= 5 {println!("All inputs High!");}
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
            Self::FlipFlop(state) => match input {
                Pulse::Low => {
                    state.flip();
                    match state {
                        FlipFlopState::On => Some(Pulse::High),
                        FlipFlopState::Off => Some(Pulse::Low),
                    }
                }
                Pulse::High => None,
            },
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
    High,
}

impl Pulse {
    fn digit(&self) -> char {
        match self {
            Self::Low => '0',
            Self::High => '1',
        }
    }
}

#[derive(Debug)]
enum FlipFlopState {
    Off,
    On,
}

impl FlipFlopState {
    fn flip(&mut self) {
        *self = match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        };
    }
}

fn illustrate(circuit: &mut Circuit, pushes: usize) {
    circuit.push_button();
    println!("After one push:\n{}", circuit.stats);
    for _ in 1..pushes {
        circuit.push_button();
    }
    circuit.print_outcomes();
    let ff = circuit
        .connections
        .iter()
        .filter(|(_, (m, _))| match m {
            Module::FlipFlop(_) => true,
            _ => false,
        })
        .map(|(n, _)| n)
        .collect::<Vec<_>>();
    println!("flip-flops: {ff:?}");
    let con = circuit
        .connections
        .iter()
        .filter(|(_, (m, _))| match m {
            Module::Conjunction(_) => true,
            _ => false,
        })
        .map(|(n, _)| n)
        .collect::<Vec<_>>();
    println!("conjuncts:  {con:?}");
    //println!("After {pushes} pushes:\n{}", circuit.stats);
}

#[cfg(test)]
mod tests {
    use std::cmp::max;

    use crate::FlipFlopTracker;

    #[test]
    fn test_ff_cycler() {
        for i in 0..=11 {
            test_ff_cycler_generic(i, 10000);
        }
    }

    fn test_ff_cycler_generic(array_index: usize, num_tests: usize) {
        let t = FlipFlopTracker::new(array_index);
        let num_tests = max(num_tests, t.start * 2);
        for i in 0..t.start {
            assert!(!t.is_on(i));
        }
        for i in t.start..num_tests {
            assert_eq!((i - t.start) % (t.period * 2) < t.period, t.is_on(i));
        }
    }
}