fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    day23::part1::execute(include_str!("../src/input.txt"));
}

#[divan::bench]
fn part2() {
    day23::part2::execute(include_str!("../src/input.txt"));
}

#[divan::bench]
fn part2_fast() {
    day23::part2_fast::execute(include_str!("../src/input.txt"));
}
