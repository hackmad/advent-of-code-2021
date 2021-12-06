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

#[allow(dead_code)]
fn simulate_growth_naive(input_file: &str, days: usize) -> usize {
    let mut fish_timers = read(input_file);
    for _i in 0..days {
        let mut num_new_fishes = 0;
        for ft in fish_timers.iter_mut() {
            if *ft == 0 {
                *ft = 6;
                num_new_fishes += 1;
            } else {
                *ft -= 1;
            }
        }
        fish_timers.append(&mut vec![8; num_new_fishes]);
    }
    fish_timers.len()
}

fn simulate_growth_fast(input_file: &str, days: usize) -> usize {
    let fish_timers = read(input_file);

    // Keep track of fish count by timer value.
    const N: usize = 9;
    let mut current = [0_usize; N];
    for t in fish_timers {
        current[t] += 1;
    }

    // Keep track of next fish count by timer value.
    let mut next = [0_usize; N];

    for _i in 0..days {
        next.copy_from_slice(&current);
        for t in 0..N {
            if current[t] > 0 {
                // There are n fishes with timer=t.
                let n = current[t];
                if t == 0 {
                    next[0] -= n; // timer reset from 0
                    next[6] += n; // to 6.
                    next[8] += n; // new fish with timer 8.
                } else {
                    next[t] -= n; // timer decreased from t
                    next[t - 1] += n; // to t-1.
                }
            }
        }
        current.copy_from_slice(&next);
    }

    current.iter().sum()
}

pub fn part1(input_file: &str) {
    let n = simulate_growth_fast(input_file, 80);
    println!("day 06: part 1 = {}", n);
}

pub fn part2(input_file: &str) {
    let n = simulate_growth_fast(input_file, 256);
    println!("day 06: part 2 = {}", n);
}
