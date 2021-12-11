use std::fs;

fn read(input_file: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|row| row.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect()
}

fn get_neighbours(i: usize, j: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    [
        (-1, 0),
        (-1, -1),
        (0, -1),
        (1, -1),
        (1, 0),
        (1, 1),
        (0, 1),
        (-1, 1),
    ]
    .iter()
    .map(|&(m, n)| (i as isize + m, j as isize + n))
    .filter(|&(m, n)| m >= 0 && m < h as isize && n >= 0 && n < w as isize)
    .map(|(m, n)| (m as usize, n as usize))
    .collect()
}

pub fn part1(input_file: &str) {
    let mut current = read(input_file);

    let h = current.len();
    assert!(h > 0);

    let w = current[0].len();
    assert!(w > 0);

    let mut flashed = vec![vec![false; h]; w];
    let mut next = vec![vec![0_u8; h]; w];
    let mut count = 0;

    for _step in 0..100 {
        for i in 0..next.len() {
            for j in 0..next[i].len() {
                next[i][j] = current[i][j] + 1;
            }
        }

        loop {
            let mut more_flashes = false;
            for i in 0..next.len() {
                for j in 0..next[i].len() {
                    if !flashed[i][j] && next[i][j] > 9 {
                        flashed[i][j] = true;
                        count += 1;

                        for (m, n) in get_neighbours(i, j, h, w) {
                            next[m][n] += 1;

                            if next[m][n] > 9 {
                                more_flashes = true;
                            }
                        }
                    }
                }
            }
            if !more_flashes {
                break;
            }
        }

        for i in 0..next.len() {
            for j in 0..next[i].len() {
                if flashed[i][j] {
                    current[i][j] = 0;
                    flashed[i][j] = false;
                } else {
                    current[i][j] = next[i][j];
                }
            }
        }
    }

    println!("day 11: part 1 = {}", count);
}

pub fn part2(input_file: &str) {
    let mut current = read(input_file);

    let h = current.len();
    assert!(h > 0);

    let w = current[0].len();
    assert!(w > 0);

    let mut flashed = vec![vec![false; h]; w];
    let mut next = vec![vec![0_u8; h]; w];
    let mut step = 0;

    loop {
        for i in 0..next.len() {
            for j in 0..next[i].len() {
                next[i][j] = current[i][j] + 1;
            }
        }

        loop {
            let mut more_flashes = false;
            for i in 0..next.len() {
                for j in 0..next[i].len() {
                    if !flashed[i][j] && next[i][j] > 9 {
                        flashed[i][j] = true;

                        for (m, n) in get_neighbours(i, j, h, w) {
                            next[m][n] += 1;

                            if next[m][n] > 9 {
                                more_flashes = true;
                            }
                        }
                    }
                }
            }
            if !more_flashes {
                break;
            }
        }

        step += 1;

        let mut num_flashed = 0;
        for i in 0..next.len() {
            for j in 0..next[i].len() {
                if flashed[i][j] {
                    current[i][j] = 0;
                    flashed[i][j] = false;
                    num_flashed += 1;
                } else {
                    current[i][j] = next[i][j];
                }
            }
        }
        if num_flashed == w * h {
            break;
        }
    }

    println!("day 11: part 2 = {}", step);
}
