use regex::{Captures, Regex};
use std::fs;

#[derive(Copy, Clone, Debug)]
struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}
impl Target {
    fn new(x1: i32, x2: i32, y1: i32, y2: i32) -> Self {
        Self { x1, x2, y1, y2 }
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"target\s+area:\s+x=(-?\d+)\.\.(-?\d+),\s+y=(-?\d+)\.\.(-?\d+)")
            .expect("invalid regex");
}

fn parse_capture(captures: &Captures, i: usize, name: &str) -> i32 {
    captures
        .get(i)
        .expect(format!("{name} not matched").as_str())
        .as_str()
        .parse()
        .expect(format!("invalid {name}. expecting integer").as_str())
}

fn read(input_file: &str) -> Target {
    let content = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let captures = RE
        .captures(content.trim_end())
        .expect("Unable to extract with regex capture groups");

    let x1 = parse_capture(&captures, 1, "x1");
    let x2 = parse_capture(&captures, 2, "x2");
    let y1 = parse_capture(&captures, 3, "y1");
    let y2 = parse_capture(&captures, 4, "y2");

    Target::new(x1, x2, y1, y2)
}

/*
 * (x, y) = (0, 0)
 * (vx, vy)
 *
 * loop {
 *    x += vx
 *    y += vy
 *
 *    if vx > 0 { vx -= 1 } else if vx < 0 { vx += 1 }
 *    vy -= 1
 * }
 *
 * | iteration | p_x                     | v_x   | p_y                     | v_y   |
 * | --------- | ----------------------- | ----- | ----------------------- | ----- |
 * | init      | 0                       | vx    | 0                       | vy    |
 * | --------- | ----------------------- | ----- | ----------------------- | ----- |
 * | 0         | vx                      | vx-1  | vy                      | vy-1  |
 * | 1         | vx + vx-1               | vx-2  | vy + vy-1               | vy-2  |
 * | 2         | vx + vx-1 + vx-2        | vx-3  | vy + vy-1 + vy-2        | vy-3  |
 * | 3         | vx + vx-1 + vx-2 + vx-3 | vx-4  | vy + vy-1 + vy-2 + vy-3 | vy-4  |
 *
 * Changes to p_x and p_y depends on v_x and v_y but are independent of x, y axes.
 *
 * Starting at p_y = 0 with vy > 0
 *
 * We will come back to p_y = 0 after N steps, then v_y will be -vy.
 * The next step, p_y < 0 for the first time after starting.
 * At this point velocity is -vy - 1
 *
 * We want to reach between y1 and y2 of target values inclusive.
 * Lowest of the two is min(y1, y2) = target_min_y
 *
 * We can use -target_min_y so that it reaches lowest target y value after crossing y = 0
 * => -vy - 1
 * => -target_min_y - 1
 *
 * The highest point happens when v_y drops to 0 and using the table above:
 * => p_y = sum(vy) = sum(-target_min_y - 1)
 */
pub fn part1(input_file: &str) {
    println!("day 17: part 1");
    let target = read(input_file);
    let target_min_y = -target.y1.min(target.y2) - 1;
    let p_y = sum(target_min_y);
    println!("Highest possible y = {p_y}");
}

pub fn part2(input_file: &str) {
    println!("day 17: part 2");
    let target = read(input_file);

    let x = target.x1.max(target.x2);
    let y = target.y1.min(target.y2).abs();

    let mut velocities: Vec<(i32, i32)> = vec![];
    for vx in 0..=x {
        for vy in -y..=y {
            if hits(&target, vx, vy) {
                velocities.push((vx, vy));
            }
        }
    }

    println!("Distinct velocities {}", velocities.len());
}

fn hits(target: &Target, mut vx: i32, mut vy: i32) -> bool {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    loop {
        // Check out of bounds
        if x > target.x2 || y < target.y1 {
            return false;
        }
        if vx == 0 && (x < target.x1 || x > target.x2) {
            return false;
        }

        // Check within target
        if x >= target.x1 && x <= target.x2 && y >= target.y1 && y <= target.y2 {
            return true;
        }

        // Continue trajectory
        x += vx;
        y += vy;

        if vx > 0 {
            vx -= 1;
        }

        vy -= 1;
    }
}

fn sum(n: i32) -> i32 {
    n * (n + 1) / 2
}
