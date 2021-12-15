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

fn main() {
    day01::part1("./inputs/day01/1.txt");
    day01::part2("./inputs/day01/1.txt");

    day02::part1("./inputs/day02/1.txt");
    day02::part2("./inputs/day02/1.txt");

    day03::part1("./inputs/day03/1.txt");
    day03::part2("./inputs/day03/1.txt");

    day04::part1("./inputs/day04/1.txt");
    day04::part2("./inputs/day04/1.txt");

    day05::part1("./inputs/day05/1.txt");
    day05::part2("./inputs/day05/1.txt");

    day06::part1("./inputs/day06/1.txt");
    day06::part2("./inputs/day06/1.txt");

    day07::part1("./inputs/day07/1.txt");
    day07::part2("./inputs/day07/1.txt");

    day08::part1("./inputs/day08/1.txt");
    day08::part2("./inputs/day08/1.txt");

    day09::part1("./inputs/day09/1.txt");
    day09::part2("./inputs/day09/1.txt");

    day10::part_1_and_2("./inputs/day10/1.txt");

    day11::part1("./inputs/day11/1.txt");
    day11::part2("./inputs/day11/1.txt");

    day12::part1("./inputs/day12/3.txt");
    day12::part2("./inputs/day12/3.txt");

    day13::part_1_and_2("./inputs/day13/1.txt");

    day14::part1("./inputs/day14/1.txt");
    day14::part2("./inputs/day14/1.txt");
}
