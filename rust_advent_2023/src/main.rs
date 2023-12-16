use clap::{Parser, ValueEnum};
use common::Part;

#[derive(Parser, Debug)]
struct Args {
    day: Day,
    part: Part,
    #[clap(short)]
    alt: bool,
}

#[derive(Debug, Clone, ValueEnum)]
enum Day {
    Day01,
    Day02,
    Day03,
    Day04,
    Day05,
    Day06,
    Day07,
    Day08,
    Day09,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
}

fn main() {
    let args = Args::parse();

    let solution = match args.day {
        Day::Day01 => day01::execute(args.part),
        Day::Day02 => day02::execute(args.part),
        Day::Day03 => day03::execute(args.part),
        Day::Day04 => day04::execute(args.part),
        Day::Day05 => day05::execute(args.part),
        Day::Day06 => day06::execute(args.part),
        Day::Day07 => day07::execute(args.part),
        Day::Day08 => day08::execute(args.part),
        Day::Day09 => day09::execute(args.part),
        Day::Day10 => day10::execute(args.part, args.alt),
        Day::Day11 => day11::execute(args.part),
        Day::Day12 => day12::execute(args.part),
        Day::Day13 => day13::execute(args.part),
        Day::Day14 => day14::execute(args.part),
        Day::Day15 => day15::execute(args.part),
        Day::Day16 => day16::execute(args.part),
    };

    println!("{solution}");
}
