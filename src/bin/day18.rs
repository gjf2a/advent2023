use advent_code_lib::{
    all_lines, breadth_first_search, chooser_main, ContinueSearch, DirType, ManhattanDir, Part,
    Position, SearchQueue,
};
use enum_iterator::all;
use indexmap::{IndexMap, IndexSet};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part| {
        match part {
            Part::One => {
                let outline = TrenchOutline::new(filename)?;
                println!("Part {part:?}: {}", outline.capacity());
            }
            Part::Two => {
                let points = points(filename)?;
                println!("Double Area: {}", shoelace(&points));
                println!("Perimeter: {}", perimeter(filename)?);
                println!("Part {part:?}: {}", trench_area(filename)?);
            }
        }
        Ok(())
    })
}

fn trench_area(filename: &str) -> anyhow::Result<i128> {
    let area = shoelace(&points(filename)?);
    let perimeter = perimeter(filename)?;
    Ok((area + perimeter) / 2 + 1)
}

fn hex_distance(line: &str) -> anyhow::Result<i128> {
    let hashtag = line.find('#').unwrap();
    Ok(i128::from_str_radix(&line[hashtag + 1..hashtag + 6], 16)?)
}

fn perimeter(filename: &str) -> anyhow::Result<i128> {
    Ok(all_lines(filename)?
        .map(|line| hex_distance(line.as_str()).unwrap())
        .sum())
}

fn shoelace(points: &Vec<(i128, i128)>) -> i128 {
    (0..points.len())
        .map(|i| (i, (i + 1) % points.len()))
        .map(|(i, j)| determinant(points[i], points[j]))
        .sum()
}

fn points(filename: &str) -> anyhow::Result<Vec<(i128, i128)>> {
    let mut at = (0, 0);
    let mut ps = vec![at];
    for line in all_lines(filename)? {
        let distance = hex_distance(line.as_str())?;
        match &line[line.len() - 2..line.len() - 1] {
            "0" => {
                at.0 += distance;
            }
            "1" => {
                at.1 += distance;
            }
            "2" => {
                at.0 -= distance;
            }
            "3" => {
                at.1 -= distance;
            }
            _ => panic!("Unrecognized digit"),
        };
        ps.push(at);
    }
    Ok(ps)
}

fn determinant(p1: (i128, i128), p2: (i128, i128)) -> i128 {
    p1.0 * p2.1 - p1.1 * p2.0
}

struct TrenchOutline {
    outline: IndexMap<Position, String>,
    min_col: isize,
    max_col: isize,
    min_row: isize,
    max_row: isize,
}

impl TrenchOutline {
    fn new(filename: &str) -> anyhow::Result<Self> {
        let mut colors = IndexMap::new();
        let mut p = Position::default();
        for line in all_lines(filename)? {
            let (dir, distance, code) = parse_line(line.as_str())?;
            for _ in 0..distance {
                p = dir.next_position(p);
                colors.insert(p, code.clone());
            }
        }
        let min_col = colors.keys().map(|p| p.col).min().unwrap();
        let min_row = colors.keys().map(|p| p.row).min().unwrap();
        let max_col = colors.keys().map(|p| p.col).max().unwrap();
        let max_row = colors.keys().map(|p| p.row).max().unwrap();
        Ok(Self {
            outline: colors,
            min_col,
            max_col,
            min_row,
            max_row,
        })
    }

    fn in_bounds(&self, p: Position) -> bool {
        self.min_col <= p.col
            && p.col <= self.max_col
            && self.min_row <= p.row
            && p.row <= self.max_row
    }

    fn capacity(&self) -> usize {
        let mut visited_in = IndexSet::new();
        let mut visited_out = IndexSet::new();
        for row in self.min_row..=self.max_row {
            for col in self.min_col..=self.max_col {
                let p = Position { row, col };
                if self.outline.contains_key(&p) {
                    visited_in.insert(p);
                } else if !visited_in.contains(&p) && !visited_out.contains(&p) {
                    let search_outcome = breadth_first_search(&p, |n, q| {
                        if self.in_bounds(*n) {
                            for dir in all::<ManhattanDir>() {
                                let neighbor = dir.next_position(*n);
                                if !self.outline.contains_key(&neighbor) {
                                    q.enqueue(&neighbor);
                                }
                            }
                            ContinueSearch::Yes
                        } else {
                            ContinueSearch::No
                        }
                    });
                    let visited = search_outcome
                        .keys()
                        .copied()
                        .collect::<IndexSet<Position>>();
                    if visited.iter().any(|k| !self.in_bounds(*k)) {
                        visited_out = visited_out.union(&visited).copied().collect();
                    } else {
                        visited_in = visited_in.union(&visited).copied().collect();
                    }
                }
            }
        }
        visited_in.len()
    }

    fn total_cells(&self) -> isize {
        (self.max_row - self.min_row + 1) * (self.max_col - self.min_col + 1)
    }
}

fn parse_line(line: &str) -> anyhow::Result<(ManhattanDir, usize, String)> {
    let mut parts = line.split_whitespace();
    let dir = match parts.next().unwrap() {
        "R" => ManhattanDir::E,
        "D" => ManhattanDir::S,
        "L" => ManhattanDir::W,
        "U" => ManhattanDir::N,
        _ => Err(anyhow::anyhow!("Unrecognized movement"))?,
    };
    let distance = parts.next().unwrap().parse::<usize>()?;
    let code = parts.next().unwrap().to_owned();
    Ok((dir, distance, code))
}
