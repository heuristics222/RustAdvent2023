use common::Part;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    {{project-name}}::execute(Part::Part1);
}

#[divan::bench]
fn part2() {
    {{project-name}}::execute(Part::Part2);
}
