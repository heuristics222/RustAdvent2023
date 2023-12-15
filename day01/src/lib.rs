use common::Part;
mod part1;
mod part2;

pub fn execute(part: Part) -> String {
    match part {
        Part::Part1 => part1::main(),
        Part::Part2 => part2::main(),
    }
}
