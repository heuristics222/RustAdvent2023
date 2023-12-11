use itertools::Itertools;
use nom::{multi::separated_list1, character::complete::char, IResult};

pub fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let inputs = parseInput(input.trim()).unwrap().1;
    // println!("{:#?}", inputs);

    inputs.iter().map(|x| evaluate(x)).sum()
}

fn evaluate(input: &Vec<i32>) -> i32 {

    if input.iter().all(|x| *x == 0) {
        return 0;
    }

    evaluate(&input.iter().tuple_windows().map(|(x, y)| y - x).collect()) + input.last().unwrap()
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

#[cfg(test)]
mod tests {
    // use test::Bencher;

    use super::*;

    #[test]
    fn it_works() {
        let result = part1("
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
");
        assert_eq!(result, 114);
    }

    // #[bench]
    // fn bench(b: &mut Bencher) {
    //     let input = include_str!("./input.txt");
    //     b.iter(|| {
    //         part1(input);
    //     })
    // }
}
