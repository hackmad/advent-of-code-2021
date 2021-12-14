use itertools::Itertools;
use std::collections::HashSet;
use std::fs;
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone)]
enum Axis {
    X,
    Y,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Coord {
    x: usize,
    y: usize,
}
impl Coord {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl Hash for Coord {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

#[derive(Clone)]
struct Fold {
    axis: Axis,
    value: usize,
}
impl Fold {
    fn new(axis: String, value: usize) -> Self {
        let axis = match &axis[..] {
            "x" => Axis::X,
            "y" => Axis::Y,
            _ => panic!("invalid axis '{}'", axis),
        };
        Self { axis, value }
    }
}

#[derive(Clone)]
struct Origami {
    dots: HashSet<Coord>,
}
impl Origami {
    fn new() -> Self {
        Self {
            dots: HashSet::new(),
        }
    }

    fn get_dimensions(&self) -> Coord {
        // Assume at least one dot.
        // We don't encode the entire paper. So this won't include:
        // - Rows with y > of bottom-most dot.y that are all empty.
        // - Columns with x > right-most dot.x that are all empty.
        let mut m = Coord::new(usize::MIN, usize::MIN);
        for dot in self.dots.iter() {
            m.x = m.x.max(dot.x);
            m.y = m.y.max(dot.y);
        }
        m
    }

    fn visible_count(&self) -> usize {
        self.dots.len()
    }

    fn fold_left(&mut self, value: usize) -> HashSet<Coord> {
        let mut new_dots = HashSet::new();
        for &dot in self.dots.iter() {
            // Dots never on fold line.
            let c = if dot.x > value {
                let d = dot.x - value;
                let x = value - d;
                Coord::new(x, dot.y)
            } else {
                dot
            };
            new_dots.insert(c);
        }
        new_dots
    }

    fn fold_up(&mut self, value: usize) -> HashSet<Coord> {
        let mut new_dots = HashSet::new();
        for &dot in self.dots.iter() {
            // Dots never on fold line.
            let c = if dot.y > value {
                let d = dot.y - value;
                let y = value - d;
                Coord::new(dot.x, y)
            } else {
                dot
            };
            new_dots.insert(c);
        }
        new_dots
    }

    fn fold(&mut self, fold: &Fold) {
        self.dots = match fold.axis {
            Axis::X => self.fold_left(fold.value),
            Axis::Y => self.fold_up(fold.value),
        };
    }
}

fn read(input_file: &str) -> (Origami, Vec<Fold>) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let mut origami = Origami::new();
    let mut folds: Vec<Fold> = vec![];
    let mut dots = true;

    for row in contents.split("\n") {
        if row.len() == 0 {
            dots = false;
            continue;
        }

        if dots {
            let c: (usize, usize) = row
                .split(",")
                .map(|v| v.parse::<usize>().unwrap())
                .collect_tuple()
                .unwrap();
            origami.dots.insert(Coord::new(c.0, c.1));
        } else {
            let r = row.replace("fold along ", "");
            let c: (String, usize) = r
                .split("=")
                .collect_tuple()
                .map(|(axis, value)| (axis.to_string(), value.parse::<usize>().unwrap()))
                .unwrap();
            folds.push(Fold::new(c.0, c.1));
        }
    }

    (origami, folds)
}

pub fn part_1_and_2(input_file: &str) {
    let (mut origami, folds) = read(input_file);
    origami.fold(&folds[0]);

    let count = origami.visible_count();
    println!("day 13: part 1 = {}", count);

    for fold in folds.iter().skip(1) {
        origami.fold(fold);
    }

    let dim = origami.get_dimensions();
    for y in 0..=dim.y {
        if y == 0 {
            print!("day 13: part 2 = ");
        } else {
            print!("                 ");
        }

        for x in 0..=dim.x {
            if origami.dots.contains(&Coord::new(x, y)) {
                print!("#");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}
