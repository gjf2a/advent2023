use std::fmt::Display;

use advent_code_lib::{all_lines, chooser_main, Part, Point};

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let points = all_lines(filename)?
            .map(|line| {
                let mut parts = line.split('@');
                let pos = parts
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<Point<i128, 3>>()
                    .unwrap();
                let vec = parts
                    .next()
                    .unwrap()
                    .trim()
                    .parse::<Point<i128, 3>>()
                    .unwrap();
                (pos, vec)
            })
            .collect::<Vec<_>>();

        match part {
            Part::One => {
                let points = points
                    .iter()
                    .map(|(p, v)| {
                        (
                            Point::<i128, 2>::from_iter(p.values().take(2)),
                            Point::<i128, 2>::from_iter(v.values().take(2)),
                        )
                    })
                    .collect::<Vec<_>>();
                view_one(&points, &options);
                let (min, max) = if filename.contains("ex") {(7, 27)} else {(200000000000000, 400000000000000)};
                let mut num_intersected = 0;
                for i in 0..points.len() {
                    for j in (i + 1)..points.len() {
                        if within(future_intersection(&points[i], &points[j]), min, max).is_some() {
                            num_intersected += 1;
                        }
                    }
                }
                println!("Part {part:?}: {num_intersected}");
            }
            Part::Two => {
                if options.len() > 0 && options.contains(&"-input".to_owned()) {
                    for point in points.iter() {
                        println!("{} @ {}", point.0, point.1);
                    }
                }
            }
        }
        Ok(())
    })
}

type Hailstone2d = (Point<i128, 2>, Point<i128, 2>);

fn within(intersection: Option<(f64, f64)>, min: i128, max: i128) -> Option<(f64, f64)> {
    let min = min as f64;
    let max = max as f64;
    intersection.filter(|(x, y)| min <= *x && *x <= max && min <= *y && *y <= max)
}

fn future_intersection(a: &Hailstone2d, b: &Hailstone2d) -> Option<(f64, f64)> {
    let line1 = Line2D::new(a);
    let line2 = Line2D::new(b);
    line1
        .intersection(&line2)
        .filter(|(_, y)| in_future(a, *y) && in_future(b, *y))
}

fn in_future(stone: &Hailstone2d, y: f64) -> bool {
    let (point, delta) = stone;
    y > point[1] as f64 && delta[1] > 0 || y < point[1] as f64 && delta[1] < 0
}

// y = (rise/run)x + b
// -rise/run x + y = b
// -rise x + run y = run b
struct Line2D {
    slope: f64,
    y_intercept: f64,
    a: i128,
    b: i128,
    c: f64,
}

impl Line2D {
    fn new(stone: &Hailstone2d) -> Self {
        let (point, delta) = stone;
        let rise = delta[1];
        let run = delta[0];
        let slope = rise as f64 / run as f64;
        let y_intercept = point[1] as f64 - slope * point[0] as f64;
        let a = -rise;
        let b = run;
        let c = y_intercept * run as f64;
        Self {
            slope,
            y_intercept,
            a,
            b,
            c,
        }
    }

    fn intersection(&self, other: &Self) -> Option<(f64, f64)> {
        if self.slope != other.slope {
            let x = (self.c * other.b as f64 - self.b as f64 * other.c)
                / (self.a * other.b - self.b * other.a) as f64;
            let y = self.slope * x + self.y_intercept;
            Some((x, y))
        } else {
            None
        }
    }
}

impl Display for Line2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} x + {} y = {}", self.a, self.b, self.c)
    }
}

fn view_one(points: &Vec<Hailstone2d>, options: &[String]) {
    if options.len() > 0 {
        if options.contains(&"-input".to_owned()) {
            for point in points.iter() {
                println!("{} @ {}", point.0, point.1);
            }
        }
        if options.contains(&"-intersections".to_owned()) {
            for (i, (p1, d1)) in points.iter().enumerate() {
                let line1 = Line2D::new(&points[i]);
                for j in (i + 1)..points.len() {
                    let line2 = Line2D::new(&points[j]);
                    println!("A: {p1} @ {d1} ({line1})");
                    println!("B: {} @ {} ({line2})", points[j].0, points[j].1);
                    let intersection = line1.intersection(&line2);
                    println!("{intersection:?}\n");
                }
            }
        }
        if options.contains(&"-future".to_owned()) {
            for (i, (p1, d1)) in points.iter().enumerate() {
                for j in (i + 1)..points.len() {
                    println!("A: {p1} @ {d1}");
                    println!("B: {} @ {}", points[j].0, points[j].1);
                    let intersection = future_intersection(&(*p1, *d1), &points[j]);
                    println!("{intersection:?}\n");
                }
            }
        }
    }
}