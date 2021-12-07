extern crate itertools;
extern crate regex;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;

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
}
