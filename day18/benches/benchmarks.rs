fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day18::part1::execute(include_str!("../src/input.txt"));
}

#[divan::bench]
fn part2() {
    day18::part2::execute(include_str!("../src/input.txt"));
}
