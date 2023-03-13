use std::fs;

use itertools::iproduct;

fn read(input_file: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .lines()
        .map(|row| row.chars().map(|c| c as u8 - '0' as u8).collect())
        .collect()
}

fn neighbours(r: usize, c: usize, h: usize, w: usize) -> Vec<(usize, usize)> {
    // Convert to isize for easier computation.
    let (r, c, h, w) = (r as isize, c as isize, h as isize, w as isize);

    // Left, Right, Bottom, Top
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .map(|&(r1, c1)| (r + r1, c + c1)) // Neighbour
        .filter(|&(r1, c1)| r1 >= 0 && r1 < h && c1 >= 0 && c1 < w) // Bounds check
        .map(|(r1, c1)| (r1 as usize, c1 as usize)) // Convert back to usize
        .collect()
}

fn find_low_points(hm: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let h = hm.len();
    let w = hm[0].len();

    iproduct!(0..h, 0..w)
        .filter(|&(r, c)| {
            neighbours(r, c, h, w)
                .iter()
                .fold(true, |a, &(r2, c2)| a && hm[r][c] < hm[r2][c2])
        })
        .collect()
}

fn bfs_basin(
    hm: &Vec<Vec<u8>>,
    seen: &mut Vec<Vec<bool>>,
    r: usize,
    c: usize,
    h: usize,
    w: usize,
) -> usize {
    // Ignore already seen point.
    if seen[r][c] {
        return 0;
    }
    seen[r][c] = true;

    // Don't include 9.
    if hm[r][c] > 8 {
        return 0;
    }

    // Add 1 for current point.
    neighbours(r, c, h, w).iter().fold(1, |sum, &(r2, c2)| {
        // Include next higher neighbour.
        if hm[r2][c2] > hm[r][c] {
            sum + bfs_basin(hm, seen, r2, c2, h, w)
        } else {
            sum
        }
    })
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

    // We could use separate `seen` for each bfs_basin() call and then
    // count all true values to get size of basin without counting the
    // visited points in the recursive function.
    let mut seen = vec![vec![false; w]; h];
    let mut basin_sizes: Vec<usize> = lows
        .iter()
        .map(|&(r, c)| bfs_basin(&heightmap, &mut seen, r, c, h, w))
        .collect();
    basin_sizes.sort();

    // Multiply top 3 sizes.
    let n = basin_sizes.iter().rev().take(3).fold(1, |a, s| a * s);
    println!("day 09: part 2 = {}", n);
}
