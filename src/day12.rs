use std::collections::VecDeque;
use std::fs;

use itertools::Itertools;

struct Cave {
    name: String,
    is_start: bool,
    is_end: bool,
    is_big: bool,
}
impl Cave {
    fn new(name: &str) -> Self {
        assert!(name.len() > 0, "cave names cannot be empty");

        // Assume chars are either all upper or all lower.
        const A: u8 = 'A' as u8;
        const Z: u8 = 'Z' as u8;
        let name_bytes = name.as_bytes();
        let is_big = name_bytes[0] >= A && name_bytes[0] <= Z;

        Self {
            name: name.to_string(),
            is_start: name.eq("start"),
            is_end: name.eq("end"),
            is_big,
        }
    }
}

struct Edge(usize, usize);

struct Graph {
    edges: Vec<Edge>,
    points: Vec<Cave>,
}

impl Graph {
    fn new() -> Self {
        Self {
            edges: vec![],
            points: vec![],
        }
    }

    fn find_or_add_point(&mut self, pt: &str) -> usize {
        for i in 0..self.points.len() {
            if self.points[i].name.eq(pt) {
                return i;
            }
        }
        self.points.push(Cave::new(pt));
        self.points.len() - 1
    }

    fn insert_edge(&mut self, pts: [&str; 2]) {
        let p0 = self.find_or_add_point(pts[0]);
        let p1 = self.find_or_add_point(pts[1]);
        self.edges.push(Edge(p0, p1));
    }

    fn find_paths<P>(&self, can_visit: &P) -> Vec<Vec<String>>
    where
        P: Fn(&VecDeque<usize>, usize) -> bool,
    {
        let mut paths: Vec<Vec<String>> = vec![];

        let starts = self
            .edges
            .iter()
            .filter(|&e| self.points[e.0].is_start || self.points[e.1].is_start)
            .map(|e| {
                if self.points[e.0].is_start {
                    (e.0, e.1)
                } else {
                    (e.1, e.0)
                }
            });

        for (from, to) in starts {
            let mut acc: Vec<Vec<usize>> = vec![];
            let mut path: VecDeque<usize> = VecDeque::new();

            path.push_back(from);
            self.traverse(&mut acc, &mut path, to, &can_visit);
            path.pop_back();

            for path in acc.iter() {
                let path: Vec<String> = path.iter().map(|&i| self.points[i].name.clone()).collect();
                paths.push(path);
            }
        }

        paths
    }

    fn traverse<P>(
        &self,
        paths: &mut Vec<Vec<usize>>,
        current_path: &mut VecDeque<usize>,
        to: usize,
        can_visit: &P,
    ) where
        P: Fn(&VecDeque<usize>, usize) -> bool,
    {
        /*
        print!("day 12:    ");
        for _i in 0..current_path.len() {
            print!("  ");
        }
        print!("{} -> {}\n", self.points[from].name, self.points[to].name);
        */

        if can_visit(current_path, to) {
            current_path.push_back(to);

            if self.points[to].is_end {
                let path: Vec<usize> = current_path.iter().map(|&p| p).collect();
                paths.push(path);
            } else {
                for e in self.edges.iter() {
                    if e.0 == to {
                        self.traverse(paths, current_path, e.1, can_visit);
                    } else if e.1 == to {
                        self.traverse(paths, current_path, e.0, can_visit);
                    }
                }
            }

            current_path.pop_back();
        }
    }
}

fn read(input_file: &str) -> Graph {
    let mut graph = Graph::new();

    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    for row in contents.split("\n") {
        let caves: Vec<_> = row.split("-").collect();
        assert!(caves.len() == 2, "invalid edge");
        graph.insert_edge([caves[0], caves[1]]);
    }

    graph
}

pub fn part1(input_file: &str) {
    let graph = read(input_file);

    let can_visit = |current_path: &VecDeque<usize>, p: usize| {
        !current_path.contains(&p) || graph.points[p].is_big
    };

    let paths = graph.find_paths(&can_visit);

    /*
    for path in paths.iter() {
        print!("day 12: ");
        for (i, cave) in path.iter().enumerate() {
            if i > 0 {
                print!(",");
            }
            print!("{}", cave)
        }
        println!();
    }
    */

    println!("day 12: part 1 = {}", paths.len());
}

pub fn part2(input_file: &str) {
    let graph = read(input_file);

    let can_visit = |current_path: &VecDeque<usize>, p: usize| {
        if graph.points[p].is_start {
            return false;
        } else if graph.points[p].is_end {
            return !current_path.contains(&p);
        } else if graph.points[p].is_big {
            return true;
        }

        let smalls = current_path
            .iter()
            .filter(|&x| {
                !graph.points[*x].is_big && !graph.points[*x].is_start && !graph.points[*x].is_end
            })
            .counts_by(|&x| x);

        if let Some(count) = smalls.get(&p) {
            if *count == 2 {
                return false;
            }

            if *count == 1 {
                return !smalls.values().contains(&2);
            }
        }
        true
    };

    let paths = graph.find_paths(&can_visit);

    /*
    for path in paths.iter() {
        print!("day 12: ");
        for (i, cave) in path.iter().enumerate() {
            if i > 0 {
                print!(",");
            }
            print!("{}", cave)
        }
        println!();
    }
    */

    println!("day 12: part 1 = {}", paths.len());
}
