use std::fs;

fn read(input_file: &str) -> Vec<(String, i64)> {
    let contents = fs::read_to_string(input_file).expect("Something went wrong reading the file");
    contents
        .split("\n")
        .map(|s| {
            let c: Vec<&str> = s.split(" ").collect();
            let command = c[0].to_string();
            let value = c[1].parse::<i64>().expect("Invalid command value");
            (command, value)
        })
        .collect()
}

pub fn part1(input_file: &str) {
    let commands = read(input_file);

    let mut x: i64 = 0;
    let mut y: i64 = 0;

    for (command, value) in commands {
        match &command[..] {
            "forward" => x += value,
            "down" => y += value,
            "up" => y -= value,
            c => panic!("Invalid command {}", c),
        }
    }
    println!("day 02: part 1 = {}", x * y);
}

pub fn part2(input_file: &str) {
    let commands = read(input_file);

    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut aim: i64 = 0;

    for (command, value) in commands {
        match &command[..] {
            "forward" => {
                x += value;
                y += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            c => panic!("Invalid command {}", c),
        }
    }
    println!("day 02: part 2 = {}", x * y);
}
