use common::Part;
pub mod part1;
pub mod part2;

pub fn execute(part: Part) -> String {
    let input = include_str!("./input.txt");
    let solution = match part {
        Part::Part1 => part1::execute(input, 200000000000000., 400000000000000.),
        Part::Part2 => part2::execute(input),
    };
    solution
}
