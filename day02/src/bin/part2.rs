use std::cmp::max;

#[derive(Debug)]
struct Game {
    number: i32,
    reveals: Vec<(i32, i32, i32)>,
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let games = parseInput(input);
    let mut sum = 0;

    for game in games {
        let mut minCubes = (0, 0, 0);
        for reveal in game.reveals {
            minCubes.0 = max(minCubes.0, reveal.0);
            minCubes.1 = max(minCubes.1, reveal.1);
            minCubes.2 = max(minCubes.2, reveal.2);
        }
        sum += minCubes.0 * minCubes.1 * minCubes.2;

        println!("{}, {}, {}", minCubes.0, minCubes.1, minCubes.2);
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
