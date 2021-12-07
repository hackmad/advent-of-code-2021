use std::collections::HashMap;
use std::fs;

fn read(input_file: &str) -> Vec<isize> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split(",")
        .map(|s| {
            s.parse::<isize>()
                .expect(&format!("invalid non-numeric input {}", s))
        })
        .collect()
}

fn median(v: &Vec<isize>) -> isize {
    let mut sorted = v.clone();
    sorted.sort();
    let mid = sorted.len() / 2;
    sorted[mid]
}

pub fn part1(input_file: &str) {
    let positions = read(input_file);
    let p_median = median(&positions);
    let cost: isize = positions.iter().map(|p| (*p - p_median).abs()).sum();
    println!("day 07: part 1 = cost={} at pos={}", cost, p_median);
}

pub fn part2(input_file: &str) {
    let positions = read(input_file);
    let min = *positions.iter().min().unwrap();
    let max = *positions.iter().max().unwrap();
    let mut best_cost: Option<isize> = None;
    let mut best_pos: Option<isize> = None;
    for i in min..=max {
        let cost: isize = positions.iter().fold(0, |a, p| {
            let n = (p - i).abs();
            let c = n * (n + 1) / 2;
            a + c
        });
        if let Some(bc) = best_cost {
            if cost < bc {
                best_cost = Some(cost);
                best_pos = Some(i as isize);
            }
        } else {
            best_cost = Some(cost);
            best_pos = Some(i as isize);
        };
    }

    println!(
        "day 07: part 2 = cost={} at pos={}",
        best_cost.unwrap(),
        best_pos.unwrap()
    );
}
