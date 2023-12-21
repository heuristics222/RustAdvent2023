use std::collections::HashSet;

use itertools::Itertools;

pub fn execute(input: &str, start: (i64, i64), steps: i64) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut positions:HashSet<(i64, i64)> = Default::default();
    let len = inputs.len() as i64;

    positions.insert(start);

    let mut quad_points:Vec<usize> = Default::default();

    for i in 1..steps {

        let clone = positions.clone();
        let points = clone.iter().collect_vec();
        positions.clear();

        points.iter().for_each(|(x, y)| {
            if inputs[(y.rem_euclid(len)) as usize][((x -1).rem_euclid(len)) as usize] {
                positions.insert((x - 1, *y));
            }
            if inputs[((y-1).rem_euclid(len)) as usize][((x).rem_euclid(len)) as usize] {
                positions.insert((*x, y -1));
            }
            if inputs[((y).rem_euclid(len)) as usize][((x +1).rem_euclid(len)) as usize]{
                positions.insert((x + 1, *y));
            }
            if inputs[((y+1).rem_euclid(len)) as usize][((x).rem_euclid(len)) as usize] {
                positions.insert((*x, y + 1));
            }
        });

        if (i - 65) % 131 == 0 {
            quad_points.push(positions.len());
            if quad_points.len() == 3 {
                break;
            }
        }
    }

    let x1 = 0 as i64;
    let x2 = 1 as i64;
    let x3 = 2 as i64;
    let y1 = quad_points[0] as i64;
    let y2 = quad_points[1] as i64;
    let y3 = quad_points[2] as i64;
    let a = (x1 * (y3 - y2) + x2 * (y1 - y3) + x3 * (y2 - y1)) / ((x1 - x2) * (x1 - x3) * (x2 - x3));
    let b = (y2 - y1) / (x2 - x1) - a * (x1 + x2);
    let c = y1 - a * x1 * x1 - b * x1;

    let a = a as usize;
    let b = b as usize;
    let c = c as usize;

    let x = (steps as usize - 65) / 131;

    (x * x * a + x*b + c).to_string()
}

fn parseInput(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| {
        line.chars().map(|ch| {
            ch == '.'
        }).collect()
    }).collect()
}
