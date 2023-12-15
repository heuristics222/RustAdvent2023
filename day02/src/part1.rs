#[derive(Debug)]
struct Game {
    number: i32,
    reveals: Vec<(i32, i32, i32)>,
}

pub fn main() -> String {
    let input = include_str!("./input.txt");
    let output = part1(input);
    output.to_string()
}

fn part1(input: &str) -> i32 {
    let games = parseInput(input);
    let mut sum = 0;

    for game in games {
        let mut valid = true;
        for reveal in game.reveals {
            valid &= reveal.0 <= 12 && reveal.1 <= 13 && reveal.2 <= 14;
        }
        sum += match valid { true => game.number, false => 0 };
    }

    sum
}

fn parseInput(input: &str) -> Vec<Game> {
    input.split("\n").map(|x| parseGame(x)).collect()
}

fn parseGame(line: &str) -> Game {
    let s: Vec<&str> = line.split(":").collect();
    let gameStringSplit: Vec<&str> = s.get(0).unwrap().split_whitespace().collect();
    let reveals: Vec<&str> = s.get(1).unwrap().split(";").collect();
    let reveals: Vec<(i32, i32, i32)> = reveals.iter().map(|x| toRGBTuple(x)).collect();

    Game {number: gameStringSplit.get(1).unwrap().parse().unwrap(), reveals}
}

fn toRGBTuple(input: &str) -> (i32, i32, i32) {
    let mut tuple = (0, 0, 0);
    let input: Vec<&str> = input.split(",").collect();

    for x in input {
        let split: Vec<&str> = x.split_whitespace().collect();
        match split.get(1).unwrap() {
            &"red" => tuple.0 = split.get(0).unwrap().parse().unwrap(),
            &"green" => tuple.1 = split.get(0).unwrap().parse().unwrap(),
            &"blue" => tuple.2 = split.get(0).unwrap().parse().unwrap(),
            _ => panic!("???"),
        }
    }

    tuple
}
