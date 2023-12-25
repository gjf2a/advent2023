use std::fmt::Display;
use std::io::Write;

use advent_code_lib::{all_lines, chooser_main};
use hash_histogram::HashHistogram;
use indexmap::{IndexMap, IndexSet};

/*
To solve the problem using graphviz:
1. Install graphviz (https://graphviz.org)
2. Use `cargo run --bin day25 in/day25.txt one -dot` to convert input file to Graphviz DOT format.
3. Use `dot -Tpng -Kneato -O in/day25.dot` to generate a PNG visualization.
4. Examine the PNG and find the three edges to cut.
5. Copy the `dot` file and remove the three edges identified visually.
6. Use `dot -Tpng -Kneato -O in/day25_cut.dot` to visualize the cut graph, to ensure the correct edges were removed.
7. Use `ccomps -x in/day25_cut.dot > in/day25_components.dot` to generate a `dot` file containing the separated connected components.
8. Use `cargo run --bin day25 in/day25_components.dot` to get the sizes of the components and puzzle solution.
*/

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, _, options| {
        if options.contains(&"-show".to_owned()) {
            let graph = Graph::from_file(filename)?;
            println!("{graph}");
        }
        if options.contains(&"-degrees".to_owned()) {
            let graph = Graph::from_file(filename)?;
            println!("{}", graph.degree_histogram());
        }
        if options.contains(&"-dot".to_owned()) {
            graphviz(filename)?;
        }
        if filename.contains("components.dot") {
            let graphs = Graph::from_dot_file(filename)?;
            let total = graphs
                .values()
                .map(|g| g.graph.len())
                .inspect(|s| println!("{s}"))
                .product::<usize>();
            println!("Part One: {total}");
        }

        Ok(())
    })
}

fn graphviz(filename: &str) -> anyhow::Result<()> {
    let prefix = filename.split(".").next().unwrap();
    let output = format!("{prefix}.dot");
    let mut file_out = std::fs::File::create(output)?;
    writeln!(file_out, "graph G {{")?;
    for line in all_lines(filename)? {
        let mut parts = line.split(':');
        let src = parts.next().unwrap();
        for dest in parts.next().unwrap().split_whitespace() {
            writeln!(file_out, "  {src} -- {dest}")?;
        }
    }
    writeln!(file_out, "}}")?;
    Ok(())
}

#[derive(Debug)]
struct Graph {
    graph: IndexMap<String, IndexSet<String>>,
}

impl Graph {
    fn from_file(filename: &str) -> anyhow::Result<Self> {
        let mut result = Self {
            graph: IndexMap::new(),
        };
        for line in all_lines(filename)? {
            let mut parts = line.split(':');
            let src = parts.next().unwrap();
            for dest in parts.next().unwrap().split_whitespace() {
                result.add_edge(src, dest);
            }
        }
        Ok(result)
    }

    fn from_dot_file(filename: &str) -> anyhow::Result<IndexMap<String, Self>> {
        let mut result = IndexMap::new();
        let mut name = String::new();
        for line in all_lines(filename)? {
            if line.starts_with("graph") {
                name = line.split_whitespace().skip(1).next().unwrap().to_owned();
                println!("Starting {name}");
                result.insert(
                    name.clone(),
                    Self {
                        graph: IndexMap::new(),
                    },
                );
            } else if line.starts_with("}") {
                println!("Finished {name}");
            } else {
                let mut line = line.trim_end_matches(";").trim().split(" -- ");
                let src = line.next().unwrap();
                let dest = line.next().unwrap();
                result.get_mut(name.as_str()).unwrap().add_edge(src, dest);
            }
        }
        Ok(result)
    }

    fn degree_histogram(&self) -> HashHistogram<usize> {
        let mut degrees = HashHistogram::new();
        for edges in self.graph.values() {
            degrees.bump(&edges.len());
        }
        degrees
    }

    fn add_edge(&mut self, src: &str, dest: &str) {
        self.add_edge_one_way(src, dest);
        self.add_edge_one_way(dest, src);
    }

    fn add_edge_one_way(&mut self, src: &str, dest: &str) {
        if !self.graph.contains_key(src) {
            self.graph.insert(src.to_owned(), IndexSet::new());
        }
        self.graph.get_mut(src).unwrap().insert(dest.to_owned());
    }
}

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (node, edges) in self.graph.iter() {
            write!(f, "{node}:")?;
            for edge in edges.iter() {
                write!(f, " {edge}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
