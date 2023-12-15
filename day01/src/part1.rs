pub fn main() -> String {
    let input = include_str!("./input.txt");
    let output = part1(input);
    output.to_string()
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.split("\n") {
        let first = firstDigit(&mut line.chars().into_iter());
        let last = firstDigit(&mut line.chars().rev().into_iter());

        sum += first * 10 + last;
    }

    sum
}

fn firstDigit(input: &mut dyn Iterator<Item=char>) -> u32 {

    for ch in input {
        if ch.is_digit(10) {
            return ch.to_digit(10).unwrap();
        }
    }
    panic!("digit not found");
}
