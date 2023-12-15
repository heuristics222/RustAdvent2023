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
fn part2() {
    day15::execute(Part::Part2);
}
