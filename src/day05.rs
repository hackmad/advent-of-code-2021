use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}
impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{},{}", self.x, self.y))
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
struct Line {
    p1: Point,
    p2: Point,
}
impl Line {
    fn new(coords: [i32; 4]) -> Self {
        Self {
            p1: Point::new(coords[0], coords[1]),
            p2: Point::new(coords[2], coords[3]),
        }
    }
}
impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{} -> {}", self.p1, self.p2))
    }
}

fn read(input_file: &str) -> Vec<Line> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|s| s.replace(" -> ", ","))
        .map(|s| {
            let coords: Vec<i32> = s
                .split(",")
                .map(|c| {
                    c.parse::<i32>()
                        .expect(&format!("invalid non-numeric input {}", s))
                })
                .collect();
            Line::new(coords.try_into().unwrap())
        })
        .collect()
}

fn update_point_count(pc: &mut HashMap<Point, usize>, x: i32, y: i32) {
    let p = Point::new(x, y);
    if let Some(count) = pc.get_mut(&p) {
        *count += 1;
    } else {
        pc.insert(p, 1);
    }
}

pub fn part1(input_file: &str) {
    let lines = read(input_file);

    let mut point_count: HashMap<Point, usize> = HashMap::new();
    for line in lines {
        if line.p1.x == line.p2.x {
            let range = if line.p1.y < line.p2.y {
                line.p1.y..line.p2.y + 1
            } else {
                line.p2.y..line.p1.y + 1
            };
            for y in range {
                update_point_count(&mut point_count, line.p1.x, y);
            }
        } else if line.p1.y == line.p2.y {
            let range = if line.p1.x < line.p2.x {
                line.p1.x..line.p2.x + 1
            } else {
                line.p2.x..line.p1.x + 1
            };
            for x in range {
                update_point_count(&mut point_count, x, line.p1.y);
            }
        }
    }

    let overlaps = point_count.values().filter(|&c| *c >= 2).count();
    println!("day 05: part 1 = {}", overlaps);
}

pub fn part2(input_file: &str) {
    let lines = read(input_file);

    let mut point_count: HashMap<Point, usize> = HashMap::new();
    for line in lines {
        if line.p1.x == line.p2.x {
            let range = if line.p1.y < line.p2.y {
                line.p1.y..line.p2.y + 1
            } else {
                line.p2.y..line.p1.y + 1
            };
            for y in range {
                update_point_count(&mut point_count, line.p1.x, y);
            }
        } else if line.p1.y == line.p2.y {
            let range = if line.p1.x < line.p2.x {
                line.p1.x..line.p2.x + 1
            } else {
                line.p2.x..line.p1.x + 1
            };
            for x in range {
                update_point_count(&mut point_count, x, line.p1.y);
            }
        } else {
            let x_step: i32 = if line.p1.x <= line.p2.x { 1 } else { -1 };
            let y_step: i32 = if line.p1.y <= line.p2.y { 1 } else { -1 };
            let mut x = line.p1.x;
            let mut y = line.p1.y;
            if line.p1.x <= line.p2.x {
                while x <= line.p2.x {
                    update_point_count(&mut point_count, x, y);
                    x += x_step;
                    y += y_step;
                }
            } else {
                while x >= line.p2.x {
                    update_point_count(&mut point_count, x, y);
                    x += x_step;
                    y += y_step;
                }
            }
        }
    }

    let overlaps = point_count.values().filter(|&c| *c >= 2).count();
    println!("day 05: part 2 = {}", overlaps);
}
