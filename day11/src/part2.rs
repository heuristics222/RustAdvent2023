use std::collections::HashSet;

use itertools::Itertools;

pub fn execute(input: &str, expansion: usize) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut cols:Vec<bool> = vec!(true; inputs[0].len());
    let rows:Vec<bool> = inputs.iter().map(|x| x.iter().all(|ch| *ch == '.')).collect();

    inputs.iter().for_each(|x| {
        x.iter().enumerate().for_each(|(i, ch)| if *ch == '#' {cols[i] = false})
    });

    let mut set:HashSet<(usize, usize)> = Default::default();
    inputs.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, ch)| {

            let emptyCols = cols[0..x].iter().filter(|b| **b).count();
            let emptyRows = rows[0..y].iter().filter(|b| **b).count();

            if *ch == '#' {
                set.insert((x + emptyCols * (expansion - 1), y + emptyRows * (expansion - 1)));
            }
        })
    });

    (set.iter().cartesian_product(set.iter()).map(|(p1, p2)| {
        (p1.0 as i64 - p2.0 as i64).abs() + (p1.1 as i64 - p2.1 as i64).abs()
    }).sum::<i64>() / 2).to_string()
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
", 10);
        assert_eq!(result, "1030");
    }

    #[test]
    fn it_works2() {
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
", 100);
        assert_eq!(result, "8410");
    }
}
