use std::collections::HashSet;

use itertools::Itertools;

pub fn execute(input: &str, start: (usize, usize), steps: usize) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    let mut positions:HashSet<(usize, usize)> = Default::default();

    positions.insert(start);

    for i in 0..steps {
        let clone = positions.clone();
        let points = clone.iter().collect_vec();
        positions.clear();

        points.iter().for_each(|point| {
            if point.0 > 0 && inputs[point.1][point.0 -1] {
                positions.insert((point.0 - 1, point.1));
            }
            if point.1 > 0 && inputs[point.1-1][point.0]{
                positions.insert((point.0, point.1 -1));
            }
            if point.0 < inputs[0].len() -1 && inputs[point.1][point.0 +1]{
                positions.insert((point.0 + 1, point.1));
            }
            if point.1 < inputs.len() -1 && inputs[point.1+1][point.0]{
                positions.insert((point.0, point.1 + 1));
            }
        });

        println!("{} {}", i, positions.len());
    }

    positions.len().to_string()
}

fn parseInput(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| {
        line.chars().map(|ch| {
            ch == '.'
        }).collect()
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = execute("
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##...####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
", (5,5), 1000);
        assert_eq!(result, "16");
    }
}
