use nom::{multi::separated_list1, character::complete::char, IResult};

pub fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let inputs = parseInput(input).unwrap().1;
    // println!("{:#?}", inputs);

    inputs.iter().map(|x| evaluate(x)).sum()
}

fn evaluate(input: &Vec<i32>) -> i32 {

    if input.iter().all(|x| *x == 0) {
        return 0;
    }

    let mut newVec: Vec<i32> = Default::default();

    for i in 1..input.len() {
        newVec.push(input[i] - input[i-1])
    }

    input.first().unwrap() - evaluate(&newVec)
}

fn parseInput(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(
        char('\n'),
        separated_list1(
            char(' '),
            nom::character::complete::i32
        )
    )(input)
}
