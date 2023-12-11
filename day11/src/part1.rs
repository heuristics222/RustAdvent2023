use std::collections::HashSet;

use itertools::Itertools;

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut cols:Vec<bool> = vec!(true; inputs[0].len());

    inputs.iter().for_each(|x| {
        x.iter().enumerate().for_each(|(i, ch)| if *ch == '#' {cols[i] = false})
    });

    let mut expanded:Vec<Vec<char>> = Default::default();

    inputs.iter().enumerate().for_each(|(_, line)| {
        let mut newLine: Vec<char> = Default::default();
        line.iter().enumerate().for_each(|(x, ch)| {
            newLine.push(*ch);
            if cols[x] {
                newLine.push('.');
            }
        });
        expanded.push(newLine);
        if line.iter().all(|x| *x == '.') {
            expanded.push(expanded.last().unwrap().clone());
        }
    });

    // dbg!(expanded);

    let mut set:HashSet<(usize, usize)> = Default::default();
    expanded.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, ch)| {
            if *ch == '#' {
                set.insert((x, y));
            }
        })
    });

    let product:Vec<(&(usize, usize), &(usize, usize))> = set.iter().cartesian_product(set.iter()).collect();

    let distances:Vec<i32> = product.iter().map(|(p1, p2)| {

        let distance = if p1 == p2 {
            0
        } else {
            (p1.0 as i32 - p2.0 as i32).abs() + (p1.1 as i32 - p2.1 as i32).abs()
        };

        distance
    }).collect();

    (distances.iter().sum::<i32>() / 2).to_string()
}

fn parseInput(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
");
        assert_eq!(result, "374");
    }
}
