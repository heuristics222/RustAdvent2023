use common::Part;
pub mod part1;
pub mod part2;
pub mod part2_fast;

pub fn execute(part: Part, alt: bool) -> String {
    let input = include_str!("./input.txt");
    let solution = match part {
        Part::Part1 => part1::execute(input),
        Part::Part2 => if alt { part2_fast::execute(input) } else { part2::execute(input) },
    };
    solution
}
