fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    {{project-name}}::part1::execute(include_str!("../src/input.txt"));
}

#[divan::bench]
fn part2() {
    {{project-name}}::part2::execute(include_str!("../src/input.txt"));
}
