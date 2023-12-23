use std::collections::{VecDeque, HashSet};

use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub fn execute(input: &str) -> String {
    let inputs = parseInput(input.trim());
    // println!("{:#?}", inputs);

    // let valid:HashMap<(isize, isize), Space> = inputs.iter().enumerate().for_each(|(y, row)| row.iter().enumerate().for_each(|(x, ch| {
    //     match ch {
    //         '.' => Space::Valid,
    //         '>' => Space::Right,
    //     }
    // })
    let start_x = inputs[0].iter().enumerate().find_or_first(|(i, x)| **x == '.').unwrap().0;

    let mut queue: VecDeque<(isize, isize, usize, HashSet<(isize, isize)>)> = Default::default();

    let dirs:Vec<(isize, isize, Direction)> = vec![(-1, 0, Direction::Left), (1, 0, Direction::Right), (0, -1, Direction::Up), (0, 1, Direction::Down)];
    let visited:HashSet<(isize, isize)> = Default::default();

    let mut paths:Vec<usize> = Default::default();

    queue.push_back((start_x as isize, 0, 0, visited));

    while let Some((x, y, len, mut visited)) = queue.pop_front() {
        visited.insert((x, y));

        let new_positions:Vec<(isize, isize, usize)> = dirs.iter().filter_map(|dir| {
            let z = Some((x + dir.0, y + dir.1, len + 1));
            if visited.contains(&(x + dir.0, y + dir.1)) {
                None
            } else if z.unwrap().0 < 0 || z.unwrap().0 >= inputs.len() as isize || z.unwrap().1 < 0 || z.unwrap().1 >= inputs.len() as isize {
                if z.unwrap().1 >= inputs.len() as isize {
                    paths.push(len);
                }
                None
            } else {
                match inputs[(y + dir.1) as usize][(x + dir.0) as usize] {
                    '.' => z,
                    '>' => if dir.2 == Direction::Right {z} else {None},
                    '<' => if dir.2 == Direction::Left {z} else {None},
                    '^' => if dir.2 == Direction::Up {z} else {None},
                    'v' => if dir.2 == Direction::Down {z} else {None},
                    '#' => None,
                    _ => panic!(),
                }
            }
        }).collect();

        if new_positions.len() > 1 {
            new_positions.iter().for_each(|z| {
                queue.push_back((z.0, z.1, z.2, visited.clone()));
            })
        } else if new_positions.len() == 1 {
            queue.push_back((new_positions[0].0, new_positions[0].1, new_positions[0].2, visited));
        }
    }

    paths.iter().max().unwrap().to_string()
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
#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
");
        assert_eq!(result, "94");
    }
}
