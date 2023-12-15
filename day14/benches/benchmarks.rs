use common::Part;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day14::execute(Part::Part1);
}

#[divan::bench]
fn part2() {
    day14::execute(Part::Part2);
}
