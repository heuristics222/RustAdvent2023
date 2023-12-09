use std::cmp::min;


static NUMS: &'static [(&str, &str)] = &[
    ("one", "1"),
    ("two", "2"),
    ("three", "3"),
    ("four", "4"),
    ("five", "5"),
    ("six", "6"),
    ("seven", "7"),
    ("eight", "8"),
    ("nine", "9"),
];

pub(crate) fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.split("\n") {
        let newLine = adjustLine(line);
        let first = firstDigit(&mut newLine.chars().into_iter());
        let last = firstDigit(&mut newLine.chars().rev().into_iter());

        sum += first * 10 + last;
    }

    sum
}

// Ugly but it works 🤷‍♂️
fn adjustLine(input: &str) -> String {
    let mut output = "".to_string();

    let mut i = 0;
    while i < input.len() {
        let index = output.len();
        for (k, v) in NUMS {
            let slice = &input[i..min(i + k.len(), input.len())];
            if &slice == k {
                output += v;
                i += slice.len() - 1;
                break;
            }
        }
        if index == output.len() {
            output += &input[i..i+1];
            i += 1;
        }
    }

    return output;
}

fn firstDigit(input: &mut dyn Iterator<Item=char>) -> u32 {

    while let Some(ch) = input.next() {
        if ch.is_digit(10) {
            return ch.to_digit(10).unwrap();
        } else {
        }
    }
    panic!("digit not found");
}
