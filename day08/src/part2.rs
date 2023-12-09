use std::collections::HashMap;

pub fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> usize {
    let inputs = parseInput(input);
    // println!("{:#?}", inputs);

    inputs.1.keys().filter(|x| x.ends_with('A')).map(|x| *x).map(|x| {
        let mut moves = 0;
        let mut cur = x;

        while !cur.ends_with('Z') {
            if inputs.0[moves % inputs.0.len()] == 'L' {
                cur = inputs.1.get(cur).unwrap().0
            } else {
                cur = inputs.1.get(cur).unwrap().1
            }
            moves += 1;
        }

        moves
    }).reduce(|x,y| num::integer::lcm(x, y)).unwrap()
}

fn parseInput(input: &str) -> (Vec<char>, HashMap<&str, (&str, &str)>, &str) {
    let lines: Vec<&str> = input.lines().collect();

    let instructions: Vec<char> = lines[0].chars().collect();
    let mut map: HashMap<&str, (&str, &str)> = Default::default();

    lines[2..].iter().for_each(|x| {
        let tup = parseLine(x);
        map.insert(tup.0, (tup.1, tup.2));
    });

    (instructions, map, &lines[2][0..3])
}

fn parseLine(line: &str) -> (&str, &str, &str) {
    (&line[0..3], &line[7..10], &line[12..15])
}
