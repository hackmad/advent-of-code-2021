use std::fs;

fn read(input_file: &str) -> (Vec<u16>, usize) {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    let contents: Vec<&str> = contents.split("\n").collect();
    let n = contents[0].len();

    let contents = contents
        .iter()
        .map(|s| u16::from_str_radix(s, 2).expect(&format!("invalid non-numeric input {}", s)))
        .collect();
    (contents, n)
}

fn calc_sum_all_bits(diagnostics: &Vec<u16>, num_bits: usize) -> Vec<usize> {
    // sum_bits[] is least to most significant bit
    let mut sum_bits = vec![0_usize; num_bits];
    for bits in diagnostics {
        for bit in 0..num_bits {
            let mask = 1 << bit;
            sum_bits[bit] += (*bits as usize & mask) >> bit;
        }
    }
    sum_bits
}

pub fn part1(input_file: &str) {
    let (diagnostics, num_bits) = read(input_file);
    let n = diagnostics.len();

    let sum_bits = calc_sum_all_bits(&diagnostics, num_bits);

    let mut gamma = 0;
    let mut epsilon = 0;
    for bit in 0..num_bits {
        let mask = 1 << bit;
        if sum_bits[bit] >= n - sum_bits[bit] {
            gamma |= mask;
        } else {
            epsilon |= mask;
        }
    }

    println!(
        "day 03: part 1 = {} * {} = {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
}

fn calc_sum_bits(diagnostics: &Vec<u16>, bit: usize) -> usize {
    // bit 0 is least significant
    let mut sum_bits = 0_usize;
    for bits in diagnostics {
        let mask = 1 << bit;
        sum_bits += (*bits as usize & mask) >> bit;
    }
    sum_bits
}

fn is_bit_set(bits: &u16, bit: usize) -> bool {
    (*bits as usize & (1 << bit)) >> bit == 1
}

fn calc_rating<C>(diagnostics: &Vec<u16>, num_bits: usize, bit_criteria: C) -> Vec<u16>
where
    C: Fn(usize, usize) -> bool,
{
    // bit_criteria(s, n) should return true if s out of n values are most commonly 1 OR
    // least commonly 1.

    let mut v: Vec<u16> = diagnostics.clone();
    for bit in (0..num_bits).rev() {
        let n = v.len();
        if n == 1 {
            break;
        }

        let sum_bits = calc_sum_bits(&v, bit);
        if bit_criteria(sum_bits, n) {
            v = v
                .iter()
                .filter(|&bits| is_bit_set(bits, bit))
                .map(|x| *x)
                .collect();
        } else {
            v = v
                .iter()
                .filter(|&bits| !is_bit_set(bits, bit))
                .map(|x| *x)
                .collect();
        }
    }
    v
}

pub fn part2(input_file: &str) {
    let (diagnostics, num_bits) = read(input_file);

    let o2 = calc_rating(&diagnostics, num_bits, |sum_bits, n| {
        // Most common: more than half the values are 1.
        sum_bits >= n - sum_bits
    });

    let co2 = calc_rating(&diagnostics, num_bits, |sum_bits, n| {
        // Least common: less than half the values are 1.
        sum_bits < n - sum_bits
    });

    println!(
        "day 03: part 2 = {} * {} = {}",
        o2[0],
        co2[0],
        o2[0] as u32 * co2[0] as u32
    );
}
