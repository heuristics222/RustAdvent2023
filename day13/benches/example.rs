use common::Part;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

// Define a `fibonacci` function and register it for benchmarking.
#[divan::bench]
fn part1() {
    day13::execute(Part::Part1);
}
