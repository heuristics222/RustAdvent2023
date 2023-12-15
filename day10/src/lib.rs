use common::Part;
mod part1;
mod part2;
mod part1_clean;
mod part2_clean;

pub fn execute(part: Part, alt: bool) -> String {
    let input = include_str!("./input.txt");
    let input_clean = include_str!("./input_clean.txt");
    let start = (62, 63);
    let solution = match part {
        Part::Part1 => if alt { part1_clean::execute(input_clean, start) } else  {part1::execute(input) },
        Part::Part2 => if alt { part2_clean::execute(input_clean, start) } else  {part2::execute(input) },
    };
    solution
}
