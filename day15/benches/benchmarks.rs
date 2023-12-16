use common::Part;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day15::execute(Part::Part1);
}

#[divan::bench]
fn part1_fast() {
    day15::part1::execute_fast(include_str!("../src/input.txt"));
}

#[divan::bench]
fn part2() {
    day15::execute(Part::Part2);
}
