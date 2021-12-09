use std::fs;

fn read(input_file: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|row| row.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect()
}

fn search_positions(r: usize, c: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    let r = r as isize;
    let c = c as isize;
    let h = h as isize;
    let w = w as isize;

    let adj: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut p = Vec::<(usize, usize)>::with_capacity(4);
    for a in adj {
        let r1 = r + a.0;
        let c1 = c + a.1;
        if r1 >= 0 && r1 < h && c1 >= 0 && c1 < w {
            p.push((r1 as usize, c1 as usize));
        }
    }
    p
}

fn find_low_points(hm: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut lows: Vec<(usize, usize)> = vec![];

    let h = hm.len();
    let w = hm[0].len();

    for r in 0..h {
        for c in 0..w {
            let is_low = search_positions(r, c, h, w)
                .iter()
                .fold(true, |a, &(r2, c2)| a && hm[r][c] < hm[r2][c2]);

            if is_low {
                lows.push((r, c));
            }
        }
    }

    lows
}

fn search_basin(
    hm: &Vec<Vec<u8>>,
    seen: &mut Vec<Vec<bool>>,
    r: usize,
    c: usize,
    h: usize,
    w: usize,
) -> usize {
    if seen[r][c] {
        return 0;
    }
    seen[r][c] = true;

    if hm[r][c] > 8 {
        return 0;
    }

    let positions = search_positions(r, c, h, w);

    let mut s = 1;
    for &(r2, c2) in positions.iter() {
        if hm[r2][c2] > hm[r][c] {
            s += search_basin(hm, seen, r2, c2, h, w);
        }
    }
    s
}

pub fn part1(input_file: &str) {
    let heightmap = read(input_file);
    let lows = find_low_points(&heightmap);
    let sum = lows
        .iter()
        .fold(0_usize, |a, &(r, c)| a + heightmap[r][c] as usize + 1);
    println!("day 09: part 1 = {}", sum);
}

pub fn part2(input_file: &str) {
    let heightmap = read(input_file);
    let lows = find_low_points(&heightmap);

    let h = heightmap.len();
    let w = heightmap[0].len();

    let mut seen = vec![vec![false; w]; h];
    let mut basin_sizes: Vec<usize> = lows
        .iter()
        .map(|&(r, c)| search_basin(&heightmap, &mut seen, r, c, h, w))
        .collect();
    basin_sizes.sort();

    let n = basin_sizes.iter().rev().take(3).fold(1, |a, s| a * s);
    println!("day 09: part 2 = {}", n);
}
