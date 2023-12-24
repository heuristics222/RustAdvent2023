fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day24::part1::execute(include_str!("../src/input.txt"), 7., 27.);
}

#[divan::bench]
fn part2() {
    day24::part2::execute(include_str!("../src/input.txt"));
}
