use std::collections::HashMap;
use std::fs;

use itertools::Itertools;

fn read(input_file: &str) -> (Vec<char>, HashMap<String, char>) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");

    let mut template: Vec<char> = vec![];
    let mut insertion_rules = HashMap::new();

    for (i, row) in contents.split("\n").enumerate() {
        if i == 0 {
            template = row.chars().collect();
            continue;
        }
        if i == 1 {
            continue;
        }

        let (pair, element): (String, char) = row
            .split(" -> ")
            .collect_tuple()
            .map(|(a, b)| (a.to_string(), b.as_bytes()[0] as char))
            .unwrap();
        insertion_rules.insert(pair, element);
    }

    (template, insertion_rules)
}

#[allow(unused)]
fn apply_insertion_rules_naive(
    polymer: &mut Vec<char>,
    insertion_rules: &HashMap<String, char>,
    n: usize,
) {
    // You can see why this will not work after a couple iterations.
    //
    //  n | 1    | 2    | 3    | 4      | 5      | ...
    // ---+------+------+------+--------+--------+-----
    //  x | 2x-1 | 4x-3 | 8x-7 | 16x-15 | 32x-31 | ...
    let pow = 2_usize.pow(n as u32);

    let mut len = polymer.len();
    let max_len = pow * (len - 1) + 1;

    let mut p1 = vec![' '; max_len];
    let mut p2 = vec![' '; max_len];

    p1[0..len].copy_from_slice(&polymer);

    for _ in 0..n {
        for i in 0..len - 1 {
            let a = p1[i];
            let b = p1[i + 1];
            let pair = format!("{}{}", a, b);

            let elem = insertion_rules
                .get(&pair)
                .expect(format!("rule for '{}' not found", pair).as_str());
            if i == 0 {
                p2[0] = a;
            }
            p2[i * 2 + 1] = *elem;
            p2[i * 2 + 2] = b;
        }
        std::mem::swap(&mut p1, &mut p2);
        len = 2 * len - 1;
    }

    if n % 2 == 1 {
        *polymer = p2;
    } else {
        *polymer = p1;
    }
}

fn apply_insertion_rules(
    polymer: &Vec<char>,
    insertion_rules: &HashMap<String, char>,
    iterations: usize,
) -> (usize, usize) {
    // Tracks the count of elements.
    let mut elem_counts: HashMap<char, usize> = HashMap::new();

    // Track the count of pairs in current iteration.
    let mut pair_counts: HashMap<String, usize> = HashMap::new();

    // Initialize counts to 0 using pair -> elemeent insertion rules.
    for (pair, elem) in insertion_rules.iter() {
        pair_counts.insert(pair.clone(), 0);
        elem_counts.insert(*elem, 0);
    }

    // Add pair and element counts in polymer (elems).
    for i in 0..polymer.len() {
        *elem_counts.get_mut(&polymer[i]).unwrap() += 1;

        if i < polymer.len() - 1 {
            let pair = format!("{}{}", polymer[i], polymer[i + 1]);
            *pair_counts.get_mut(&pair).unwrap() += 1;
        }
    }

    for _k in 0..iterations {
        let mut new_pair_counts = pair_counts.clone();

        for (pair, &pcount) in pair_counts.iter() {
            // Current iteration's pairs have count > 0. Don't apply rules.
            if pcount == 0 {
                continue;
            }

            // Apply rule to see what element is inserted.
            let inserted_elem = insertion_rules.get(pair).unwrap();

            // Increase each element's count by `pcount`.
            *elem_counts.get_mut(inserted_elem).unwrap() += pcount;

            // Figure out the 2 new pairs based on insertion rule.
            let pair_elems: Vec<char> = pair.chars().collect();
            let pair1 = format!("{}{}", pair_elems[0], inserted_elem);
            let pair2 = format!("{}{}", inserted_elem, pair_elems[1]);

            // Current pair will be broken. Decrease count by `pcount`.
            *new_pair_counts.get_mut(pair).unwrap() -= pcount;

            // Increase count of new pairs by `pcount`.
            *new_pair_counts.get_mut(&pair1).unwrap() += pcount;
            *new_pair_counts.get_mut(&pair2).unwrap() += pcount;
        }

        pair_counts = new_pair_counts;
    }

    let counts: Vec<(char, usize)> = elem_counts
        .iter()
        .sorted_by_key(|(&_elem, &count)| count)
        .map(|(&elem, &count)| (elem, count))
        .collect();

    let least_common = counts[0].1;
    let most_common = counts[counts.len() - 1].1;

    (least_common, most_common)
}

pub fn part1(input_file: &str) {
    let (template, insertion_rules) = read(input_file);
    let (least_common, most_common) = apply_insertion_rules(&template, &insertion_rules, 10);

    println!(
        "day 13: part 1 = {} - {} = {}",
        most_common,
        least_common,
        most_common - least_common
    );
}

pub fn part2(input_file: &str) {
    let (template, insertion_rules) = read(input_file);
    let (least_common, most_common) = apply_insertion_rules(&template, &insertion_rules, 40);

    println!(
        "day 13: part 2 = {} - {} = {}",
        most_common,
        least_common,
        most_common - least_common
    );
}
