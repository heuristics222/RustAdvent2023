use std::{cmp::{max, min}, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut grid = parseInput(input);
    let mut sum = 0;

    for y in 0..grid.len() {
        for x in 0..grid.get(y).unwrap().len() {
            let ch = grid.get(y).unwrap().get(x).unwrap();
            if *ch == '*' {

                let mut adjacencies: HashMap<(usize, usize), i32> = Default::default();
                for i in max(0,x-1)..=min(grid.get(y).unwrap().len(),x+1) {
                    for j in max(0, y-1)..=min(grid.len(),y+1) {
                        let num = getNum(&mut grid, i, j);
                        if num.is_some() {
                            adjacencies.insert(num.unwrap().0, num.unwrap().1);
                        }
                    }
                }

                if adjacencies.len() == 2 {
                    let values:Vec<&i32> = adjacencies.values().collect();
                    sum += values[0] * values[1];
                }
            }
        }
    }

    sum
}

fn parseInput(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.split("\n").collect();

    lines.iter().map(|x| x.chars().into_iter().collect()).collect()
}

fn getNum(grid: &mut Vec<Vec<char>>, x: usize, y: usize) -> Option<((usize, usize), i32)> {
    let mut x = x as i32;
    let ch = grid.get(y).unwrap().get(x as usize).unwrap();

    if !ch.is_digit(10) {
        return None;
    }

    while x >= 0 && grid.get(y).unwrap().get(x as usize).unwrap().is_digit(10) {
        x = x - 1;
    }

    x = x + 1;
    let posX = x as usize;
    let posY = y;

    let mut sum:i32 = 0;
    while (x as usize) < grid.get(y).unwrap().len() && grid.get(y).unwrap().get(x as usize).unwrap().is_digit(10) {
        sum = sum * 10 + grid.get(y).unwrap().get(x as usize).unwrap().to_digit(10).unwrap() as i32;
        x = x + 1;
    }

    Some(((posX, posY), sum))
}
