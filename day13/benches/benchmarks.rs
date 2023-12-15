use common::Part;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day13::execute(Part::Part1);
}

#[divan::bench]
fn part2() {
    day13::execute(Part::Part2);
}
