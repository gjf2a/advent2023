use std::fmt::Display;

use advent_code_lib::{all_lines, chooser_main, Part, Point};
use nalgebra::Matrix3;
use num_rational::Ratio;

fn main() -> anyhow::Result<()> {
    chooser_main(|filename, part, options| {
        let points = points_from(filename)?;

        match part {
            Part::One => {
                let points = planeify(&points);
                view_one(&points, &options);
                let (min, max) = part_1_bounds(filename);
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
                if options.contains(&"-input".to_owned()) {
                    for point in points.iter() {
                        println!("{} @ {}", point.0, point.1);
                    }
                }
                if options.contains(&"-coplanar".to_owned()) {
                    let (p1, delta1) = points[0];
                    for (point, delta) in points.iter().skip(1) {
                        // Pick t1 and t2 so that the rank is 1.
                        // To do that, each row must be linear combos of the others.
                        // p1 - (p3 + t2 * delta) == c1 * (p3 - (p3 + t2 * delta))
                        // => p1 - p3 - t2 * delta == c1 * p3 - c1 * (p3 + t2 * delta)
                        // => p1 - p3 - t2 * delta == c1 * p3 - c1 * p3 - c1 * t2 * delta
                        // => p1 - p3 - t2 * delta == -c1 * t2 * delta
                        // => p1 - p3 == -c1 * t2 * delta + t2 * delta
                        // => p1 - p3 == t2 * delta * (-c1 + 1)
                        // => t2 == (p1 - p3) / (delta * (1 - c1))
                        //
                        // p1 - (p3 + t2 * delta) == c2 * ((p1 + t1 * delta1) - (p3 + t2 * delta))
                        // => p1 - p3 - t2 * delta == c2 * (p1 + t1 * delta1) - c2 * (p3 + t2 * delta)
                        // => p1 - p3 - t2 * delta == c2 * p1 + c2 * t1 * delta1 - c2 * p3 - c2 * t2 * delta
                        // => -p3 - t2 * delta == (c2 - 1) * p1 + c2 * t1 * delta1 - c2 * p3 - c2 * t2 * delta
                        // => -t2 * delta == (c2 - 1) * p1 + c2 * t1 * delta1 - c2 * p3 - c2 * t2 * delta + p3
                        // => -t2 * delta == (c2 - 1) * p1 + c2 * t1 * delta1 + (1 - c2) * p3 - c2 * t2 * delta
                        // => 
                        let mut t1 = 1;
                        let p2 = p1 + delta1 * t1;
                        let p3 = *point;
                        let mut t2 = 1;
                        let p4 = p3 + *delta * t2;
                        let matrix = Matrix3::new(
                            (p1[0] - p4[0]) as f64, (p1[1] - p4[1]) as f64, (p1[2] - p4[2]) as f64,
                            (p2[0] - p4[0]) as f64, (p2[1] - p4[1]) as f64, (p2[2] - p4[2]) as f64,
                            (p3[0] - p4[0]) as f64, (p3[1] - p4[1]) as f64, (p3[2] - p4[2]) as f64,
                        );
                        println!("Rank of matrix: {}", matrix.rank(0.0));
                    }
                }
            }
        }
        Ok(())
    })
}

type Hailstone2d = (Point<i128, 2>, Point<i128, 2>);
type Hailstone3d = (Point<i128, 3>, Point<i128, 3>);

fn within(intersection: Option<(Ratio<i128>, Ratio<i128>)>, min: i128, max: i128) -> Option<(Ratio<i128>, Ratio<i128>)> {
    let min = Ratio::new(min, 1);
    let max = Ratio::new(max, 1);
    intersection.filter(|(x, y)| min <= *x && *x <= max && min <= *y && *y <= max)
}

fn future_intersection(a: &Hailstone2d, b: &Hailstone2d) -> Option<(Ratio<i128>, Ratio<i128>)> {
    let line1 = Line2D::new(a);
    let line2 = Line2D::new(b);
    line1
        .intersection(&line2)
        .filter(|(_, y)| in_future(a, *y) && in_future(b, *y))
}

fn in_future(stone: &Hailstone2d, y: Ratio<i128>) -> bool {
    let (point, delta) = stone;
    let p_1 = Ratio::new(point[1], 1);
    y > p_1 && delta[1] > 0 || y < p_1 && delta[1] < 0
}

// y = (rise/run)x + b
// -rise/run x + y = b
// -rise x + run y = run b
struct Line2D {
    slope: Ratio<i128>,
    y_intercept: Ratio<i128>,
    a: i128,
    b: i128,
    c: Ratio<i128>,
}

impl Line2D {
    fn new(stone: &Hailstone2d) -> Self {
        let (point, delta) = stone;
        let rise = delta[1];
        let run = delta[0];
        let slope = Ratio::new(rise, run);
        let y_intercept = Ratio::new(point[1], 1) - slope * point[0];
        let a = -rise;
        let b = run;
        let c = y_intercept * run;
        Self {
            slope,
            y_intercept,
            a,
            b,
            c,
        }
    }

    fn intersection(&self, other: &Self) -> Option<(Ratio<i128>, Ratio<i128>)> {
        if self.slope != other.slope {
            let x = (self.c * other.b - other.c * self.b)
                / (self.a * other.b - self.b * other.a);
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

fn points_from(filename: &str) -> anyhow::Result<Vec<Hailstone3d>> {
    Ok(all_lines(filename)?
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
        .collect::<Vec<_>>())
}

fn planeify(points: &Vec<Hailstone3d>) -> Vec<Hailstone2d> {
    points
        .iter()
        .map(|(p, v)| {
            (
                Point::<i128, 2>::from_iter(p.values().take(2)),
                Point::<i128, 2>::from_iter(v.values().take(2)),
            )
        })
        .collect::<Vec<_>>()
}

fn part_1_bounds(filename: &str) -> (i128, i128) {
    if filename.contains("ex") {
        (7, 27)
    } else {
        (200000000000000, 400000000000000)
    }
}
