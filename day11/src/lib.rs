use common::Part;
mod part1;
mod part2;

pub fn execute(part: Part) -> String {
    let input = include_str!("./input.txt");
    let solution = match part {
        Part::Part1 => part1::execute(input),
        Part::Part2 => part2::execute(input, 1000000),
    };
    solution
}
