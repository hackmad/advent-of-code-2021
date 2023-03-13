use itertools::{iproduct, Itertools};
use std::collections::VecDeque;
use std::fs;
use std::hash::{Hash, Hasher};
use std::{cmp::Ordering, collections::BinaryHeap, collections::HashMap};

type Grid = Vec<Vec<usize>>;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
// The priority queue depends on `Ord`. Explicitly implement the trait so the
// queue becomes a min-heap instead of a max-heap.
impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.x.cmp(&self.x).then_with(|| other.y.cmp(&self.y))
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read(input_file: &str) -> Grid {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .lines()
        .map(|row| {
            row.chars()
                .map(|c| (c as u8 - '0' as u8) as usize)
                .collect()
        })
        .collect()
}

fn get_neighbours(p: &Point, w: usize, h: usize) -> Vec<Point> {
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(x, y)| (p.x as isize + x, p.y as isize + y))
        .filter(|&(x, y)| x >= 0 && x < w as isize && y >= 0 && y < h as isize)
        .map(|(x, y)| Point::new(x as usize, y as usize))
        .collect()
}

fn manhattan_distance(p0: &Point, p1: &Point) -> usize {
    let dx = p0.x.max(p1.x) - p0.x.min(p1.x);
    let dy = p0.y.max(p1.y) - p0.y.min(p1.y);
    dx + dy
}

fn reconstruct_path(came_from: &HashMap<Point, Point>, current: &Point) -> Vec<Point> {
    let mut total_path: VecDeque<Point> = VecDeque::new();
    total_path.push_front(*current);

    let mut curr = current;
    while let Some(c) = came_from.get(curr) {
        total_path.push_front(*c);
        curr = c;
    }

    total_path.iter().map(|&v| v).collect()
}

// https://en.wikipedia.org/wiki/A*_search_algorithm
// A* finds a path from start to goal.
// - `h` is the heuristic function. h(n) estimates the cost to reach goal from
//   node n.
fn a_star<H>(grid: &Grid, start: &Point, goal: &Point, h: &H) -> Vec<Point>
where
    H: Fn(&Point, &Point) -> usize, // Heuristic function
{
    let height = grid.len();
    let width = grid[0].len();

    // The set of discovered nodes that may need to be (re-)expanded.
    // Initially, only the start node is known.
    let mut open_set = BinaryHeap::new();
    open_set.push(*start);

    // For node n, came_from[n] is the node immediately preceding it on the
    // cheapest path from start to n currently known.
    let mut came_from: HashMap<Point, Point> = HashMap::new();

    // For node n, g_score[n] is the cost of the cheapest path from start to n
    // currently known.
    let mut g_score: HashMap<Point, usize> = HashMap::new();
    for (x, y) in iproduct!(0..width, 0..height) {
        g_score.insert(Point::new(x, y), usize::MAX);
    }
    g_score.insert(*start, 0);

    // For node n, f_score[n] := g_score[n] + h(n). f_score[n] represents our
    // current best guess as to how short a path from start to finish can be if
    // it goes through n.
    let mut f_score: HashMap<Point, usize> = HashMap::new();
    for (x, y) in iproduct!(0..width, 0..height) {
        f_score.insert(Point::new(x, y), usize::MAX);
    }
    f_score.insert(*start, h(start, goal));

    while !open_set.is_empty() {
        // Get the node in open_set having the lowest f_score[] value. This
        // operation can occur in O(1) time if open_set is a min-heap or a
        // priority queue.
        let current = open_set.pop().unwrap();

        if current.eq(goal) {
            return reconstruct_path(&came_from, &current);
        }

        let neighbours = get_neighbours(&current, grid.len(), grid[0].len());
        for neighbour in neighbours.iter() {
            // d(current, neighbour) is the weight of the edge from current to
            // neighbour tentative_g_score is the distance from start to the
            // neighbour through current
            let d = grid[neighbour.y][neighbour.x];
            let tentative_g_score = *g_score.get(&current).unwrap() + d;
            if tentative_g_score < *g_score.get(neighbour).unwrap() {
                // This path to neighbour is better than any previous one.
                // Record it!
                came_from.insert(*neighbour, current);
                g_score.insert(*neighbour, tentative_g_score);
                f_score.insert(*neighbour, tentative_g_score + h(&current, neighbour));

                // This check is expensive.
                if !open_set.iter().contains(neighbour) {
                    open_set.push(*neighbour);
                }
            }
        }
    }

    // Open set is empty but goal was never reached.
    vec![]
}

pub fn part1(input_file: &str) {
    let grid = read(input_file);
    let h = grid.len();
    let w = grid[0].len();

    /*
    for y in 0..h {
        for x in 0..w {
            print!("{}", grid2[y][x]);
        }
        println!();
    }
    */

    let best = a_star(
        &grid,
        &Point::new(0, 0),
        &Point::new(w - 1, h - 1),
        &manhattan_distance,
    );
    //println!("{:?}", best);

    let mut risk: usize = best.iter().map(|p| grid[p.y][p.x]).sum();
    risk -= grid[0][0]; // Start is not entered. Skip risk.
    println!("day 15: part 1 = {}", risk);
}

pub fn part2(input_file: &str) {
    let grid = read(input_file);
    let mut grid2 = grid.clone();

    // Expand 4 times to the right
    for i in 1..5 {
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                let mut v = grid[y][x] + i;
                if v > 9 {
                    v -= 9;
                }
                grid2[y].push(v);
            }
        }
    }

    // Expand 4 times down
    for i in 1..5 {
        for y in 0..grid.len() {
            let mut row = vec![];
            for x in 0..grid2[0].len() {
                let mut v = grid2[y][x] + i;
                if v > 9 {
                    v -= 9;
                }
                row.push(v);
            }
            grid2.push(row);
        }
    }

    let h = grid2.len();
    let w = grid2[0].len();

    /*
    for y in 0..h {
        for x in 0..w {
            print!("{}", grid2[y][x]);
        }
        println!();
    }
    */

    let best = a_star(
        &grid2,
        &Point::new(0, 0),
        &Point::new(w - 1, h - 1),
        &manhattan_distance,
    );
    //println!("{:?}", best);

    let mut risk: usize = best.iter().map(|p| grid2[p.y][p.x]).sum();
    risk -= grid2[0][0]; // Start is not entered. Skip risk.
    println!("day 15: part 2 = {}", risk);
}
