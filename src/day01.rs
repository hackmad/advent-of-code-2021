use itertools::multizip;
use std::fs;

fn read(input_file: &str) -> Vec<i64> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|s| s.parse::<i64>().expect("invalid non-numeric input"))
        .collect()
}

fn calc_increases(depths: Vec<i64>) -> i64 {
    match depths.len() {
        0 => 0,
        _ => depths
            .iter()
            .zip(depths.iter().skip(1))
            .fold(0, |a, (d0, d1)| if d1 > d0 { a + 1 } else { a }),
    }
}

pub fn part1(input_file: &str) {
    let depths = read(input_file);
    let n = calc_increases(depths);
    println!("day 01: part 1 = {}", n);
}

pub fn part2(input_file: &str) {
    let depths = read(input_file);
    let sliding_depths: Vec<_> =
        multizip((depths.iter(), depths.iter().skip(1), depths.iter().skip(2)))
            .map(|(d0, d1, d2)| d0 + d1 + d2)
            .collect();
    let n = calc_increases(sliding_depths);
    println!("day 01: part 2 = {}", n);
}
