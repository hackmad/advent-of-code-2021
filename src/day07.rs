use std::fs;

fn read(input_file: &str) -> Vec<usize> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split(",")
        .map(|s| {
            s.parse::<usize>()
                .expect(&format!("invalid non-numeric input {}", s))
        })
        .collect()
}

fn median(v: &Vec<usize>) -> usize {
    let mut sorted = v.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    sorted[mid]
}

pub fn part1(input_file: &str) {
    let positions = read(input_file);
    let p_median = median(&positions);
    let cost: usize = positions
        .iter()
        .map(|p| {
            if *p > p_median {
                *p - p_median
            } else {
                p_median - *p
            }
        })
        .sum();
    println!("day 07: part 1 = {}", cost);
}

pub fn part2(input_file: &str) {
    let _positions = read(input_file);
    let cost = 0; // todo!
    println!("day 07: part 2 = {}", 0);
}
