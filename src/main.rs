use std::env;

extern crate itertools;
extern crate regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Please specify day [1-31]");
        return;
    }

    let day = args[1].parse::<u8>();
    if day.is_err() {
        eprintln!(
            "Invalid day '{}'. Please specify a value between 1 and 31.",
            args[1]
        );
        return;
    }
    let day = day.ok().unwrap();

    match day {
        1 => {
            day01::part1("./inputs/day01/1.txt");
            day01::part2("./inputs/day01/1.txt");
        }
        2 => {
            day02::part1("./inputs/day02/1.txt");
            day02::part2("./inputs/day02/1.txt");
        }
        3 => {
            day03::part1("./inputs/day03/1.txt");
            day03::part2("./inputs/day03/1.txt");
        }
        4 => {
            day04::part1("./inputs/day04/1.txt");
            day04::part2("./inputs/day04/1.txt");
        }
        5 => {
            day05::part1("./inputs/day05/1.txt");
            day05::part2("./inputs/day05/1.txt");
        }
        6 => {
            day06::part1("./inputs/day06/1.txt");
            day06::part2("./inputs/day06/1.txt");
        }
        7 => {
            day07::part1("./inputs/day07/1.txt");
            day07::part2("./inputs/day07/1.txt");
        }
        8 => {
            day08::part1("./inputs/day08/1.txt");
            day08::part2("./inputs/day08/1.txt");
        }
        9 => {
            day09::part1("./inputs/day09/1.txt");
            day09::part2("./inputs/day09/1.txt");
        }
        10 => {
            day10::part_1_and_2("./inputs/day10/1.txt");
        }
        11 => {
            day11::part1("./inputs/day11/1.txt");
            day11::part2("./inputs/day11/1.txt");
        }
        12 => {
            day12::part1("./inputs/day12/3.txt");
            day12::part2("./inputs/day12/3.txt");
        }
        13 => {
            day13::part_1_and_2("./inputs/day13/1.txt");
        }
        14 => {
            day14::part1("./inputs/day14/1.txt");
            day14::part2("./inputs/day14/1.txt");
        }
        15 => {
            day15::part1("./inputs/day15/1.txt");
            day15::part2("./inputs/day15/1.txt");
        }
        16 => {
            day16::part1("./inputs/day16/1.txt");
            day16::part2("./inputs/day16/0.txt");
        }
        d if d <= 31 => {
            println!("Day {} not done yet", day);
        }
        d => {
            eprintln!(
                "Invalid day '{}'. Please specify a value between 1 and 31.",
                d
            );
        }
    }
}
