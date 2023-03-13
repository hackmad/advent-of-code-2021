use itertools::Either;
use itertools::Itertools;
use std::fs;

struct Entry {
    signals: Vec<String>,
    outputs: Vec<String>,
}

impl Entry {
    fn new(s: Vec<&str>, o: Vec<&str>) -> Self {
        assert!(s.len() == 10, "invalid signals format");
        assert!(o.len() == 4, "invalid ouput format");

        let signals = s.iter().map(|v| v.to_string()).collect_vec();
        let output = o.iter().map(|v| v.to_string()).collect_vec();

        Self {
            signals,
            outputs: output,
        }
    }
}

fn read(input_file: &str) -> Vec<Entry> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .lines()
        .map(|entry| {
            let split = entry.split("|").collect_vec();
            assert!(split.len() == 2, "invalid entry format");

            let s = split[0].trim().split(" ").collect_vec();
            let o = split[1].trim().split(" ").collect_vec();
            Entry::new(s, o)
        })
        .collect()
}

fn signals_to_digits(signal: &str) -> Either<usize, [usize; 3]> {
    match signal.len() {
        2 => Either::Left(1),
        3 => Either::Left(7),
        4 => Either::Left(4),
        5 => Either::Right([2, 3, 5]),
        6 => Either::Right([0, 6, 9]),
        7 => Either::Left(8),
        _ => panic!("invalid signal"),
    }
}

// Finds which chars from `in_str` are missing in `check_str`.
fn get_missing(check_str: &str, in_str: &str) -> Vec<char> {
    let mut missing: Vec<char> = vec![];
    for c in in_str.chars() {
        if !check_str.contains(c) {
            missing.push(c);
        }
    }
    missing
}

fn vec_char_to_str(v: Vec<char>) -> String {
    let mut s = String::new();
    for c in v {
        s.push(c);
    }
    s
}

fn sort_str_chars(s: &str) -> String {
    s.chars().sorted().rev().collect::<String>()
}

fn decode_signals(signals: &Vec<String>) -> Vec<String> {
    let signals = signals
        .iter()
        .map(|signal| (signal, signals_to_digits(&signal)));

    let mut signals_by_digit = [""; 10];

    // Figure out the signals that map to exactly 1 digit.
    let easy_digits = signals.clone().filter(|d| d.1.is_left());
    for (signal, digit) in easy_digits {
        let digit = digit.unwrap_left();
        signals_by_digit[digit] = signal;
    }

    // Get the signals for [2, 3, 5] and [0, 6, 9].
    let signal_235 = signals
        .clone()
        .filter(|(_, d)| d.is_right() && d.unwrap_right()[0] == 2)
        .map(|(s, _)| s);

    let signal_069 = signals
        .clone()
        .filter(|(_, d)| d.is_right() && d.unwrap_right()[0] == 0)
        .map(|(s, _)| s);

    // Out of 2, 3 and 5 only 3 has top-right and bottom-right segments.
    // Compare with 1 to partition the signals for 3 vs. 2, 5.
    let signal_3_25: (Vec<&String>, Vec<&String>) =
        signal_235.partition(|s| get_missing(s, &signals_by_digit[1]).len() == 0);

    let (signal_3, signal_25) = signal_3_25;
    signals_by_digit[3] = signal_3[0];

    // Compare 3 with 7 to isolate middle and bottom segment.
    // Then compare with 4 to isolate bottom and then isolate middle.
    let middle_or_bottom = get_missing(&signals_by_digit[7], &signals_by_digit[3]);
    let middle_or_bottom = vec_char_to_str(middle_or_bottom);

    let bottom = get_missing(&signals_by_digit[4], &middle_or_bottom)[0];
    let middle = get_missing(&bottom.to_string(), &middle_or_bottom)[0];

    // Compare 1 and 4 to get top-left + middle. And, since we know middle we get
    // top-left.
    let top_left_and_middle = get_missing(&signals_by_digit[1], &signals_by_digit[4]);
    let top_left_and_middle = vec_char_to_str(top_left_and_middle);
    let top_left = get_missing(&middle.to_string(), &top_left_and_middle)[0];

    // Compare 3 and 8 to get top-left + bottom-left and since we know top-left
    // we get bottom-left.
    let top_left_and_bottom_left = get_missing(&signals_by_digit[3], &signals_by_digit[8]);
    let top_left_and_bottom_left = vec_char_to_str(top_left_and_bottom_left);
    let bottom_left = get_missing(&top_left.to_string(), &top_left_and_bottom_left)[0];

    // Compare 0, 6, 9 with 8 to figure out each digit based on which segment is
    // missing.
    for signal in signal_069 {
        let missing = get_missing(signal, &signals_by_digit[8])[0];
        if missing == middle {
            signals_by_digit[0] = signal;
        } else if missing == bottom_left {
            signals_by_digit[9] = signal;
        } else {
            signals_by_digit[6] = signal;
        }
    }

    // Compare 2, 5 with 8 to figure out each digit based on which segment is
    // missing.
    for signal in signal_25 {
        let missing = get_missing(signal, &signals_by_digit[8]);
        if missing.contains(&top_left) {
            signals_by_digit[2] = signal;
        } else {
            signals_by_digit[5] = signal;
        }
    }

    // Sort chars for each digit's signals to make output matching easier.
    signals_by_digit
        .iter()
        .map(|&s| sort_str_chars(s))
        .collect_vec()
}

fn match_output(output: &str, digits_to_str: &Vec<String>) -> Option<usize> {
    let output = sort_str_chars(output);
    for i in 0..digits_to_str.len() {
        if output == digits_to_str[i] {
            return Some(i);
        }
    }
    None
}

pub fn part1(input_file: &str) {
    let entries = read(input_file);
    let count_1478 = entries.iter().fold(0, |sum, entry| {
        sum + entry
            .outputs
            .iter()
            .filter(|output| signals_to_digits(&output).is_left())
            .count()
    });
    println!("day 08: part 1 = {}", count_1478);
}

pub fn part2(input_file: &str) {
    let entries = read(input_file);

    let sum: usize = entries
        .iter()
        .map(|entry| {
            let digits_to_str = decode_signals(&entry.signals);

            let n = entry.outputs.len();
            let mut number = 0;
            for i in 0..n {
                let digit = match_output(&entry.outputs[i], &digits_to_str).unwrap();
                number += digit * 10_usize.pow((n - i - 1) as u32);
            }
            number
        })
        .sum();

    println!("day 08: part 2 = {}", sum);
}
